//! `cargo xtask <check>`: repository policy checks and CI gates (D-73).

#![forbid(unsafe_code)]

use std::path::PathBuf;
use std::process::ExitCode;

use xtask::util::{Ctx, Finding, report};
use xtask::{attribution, docs, repository, review_gates, rust_policy, workflows};

const HELP: &str = "\
usage: cargo xtask [--repo DIR] [--policy DIR] <command> [options]

  --repo DIR     tree under test (default: current directory)
  --policy DIR   trusted policy tree (default: --repo). CI passes the base branch.

commands:
  policy         unsafe scope, native runtimes, manifests, workflows, Python, CODEOWNERS
  provenance     dataset manifests, raw/research-only data, secrets (D-56)
  docs           Markdown links and requirement traceability
  attribution    AI-attribution scan (D-52) of a range:
                   --base REV --head REV [--title T] [--body-file F]
                 or, in GitHub Actions, the range and metadata of the event
  review         independent-review evidence gate: --evidence REV (from the event)
  approval       protected-path owner approval gate (from the event)
  all            policy, provenance and docs
";

struct Args {
    ctx: Ctx,
    command: String,
    options: Vec<(String, String)>,
}

fn parse() -> Result<Args, String> {
    let mut repo = None;
    let mut policy = None;
    let mut command = None;
    let mut options = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let mut value = |name: &str| args.next().ok_or(format!("{name} needs a value"));
        match arg.as_str() {
            "--repo" => repo = Some(PathBuf::from(value("--repo")?)),
            "--policy" => policy = Some(PathBuf::from(value("--policy")?)),
            "-h" | "--help" | "help" => return Err(String::new()),
            flag if flag.starts_with("--") && command.is_some() => {
                options.push((flag.to_owned(), value(flag)?));
            }
            name if command.is_none() => command = Some(name.to_owned()),
            other => return Err(format!("unexpected argument {other}")),
        }
    }
    let repo = match repo {
        Some(path) => path,
        None => std::env::current_dir().map_err(|e| e.to_string())?,
    };
    let policy = policy.unwrap_or_else(|| repo.clone());
    Ok(Args {
        ctx: Ctx { repo, policy },
        command: command.ok_or_else(String::new)?,
        options,
    })
}

fn option<'a>(args: &'a Args, name: &str) -> Option<&'a str> {
    args.options
        .iter()
        .find(|(key, _)| key == name)
        .map(|(_, value)| value.as_str())
}

fn event() -> Result<Option<(String, serde_json::Value)>, String> {
    let (Some(name), Some(path)) = (
        std::env::var_os("GITHUB_EVENT_NAME"),
        std::env::var_os("GITHUB_EVENT_PATH"),
    ) else {
        return Ok(None);
    };
    let text = std::fs::read_to_string(&path).map_err(|e| format!("event file: {e}"))?;
    let value = serde_json::from_str(&text).map_err(|e| format!("event file: {e}"))?;
    Ok(Some((name.to_string_lossy().into_owned(), value)))
}

fn run_policy(ctx: &Ctx) -> Result<bool, String> {
    let mut ok = true;
    ok &= report(
        "unsafe-scope and native-runtimes",
        &rust_policy::check_sources(ctx)?,
    );
    ok &= report("manifests", &rust_policy::check_manifests(ctx)?);
    ok &= report("workflows", &workflows::check(ctx)?);
    ok &= report("python", &repository::python(ctx)?);
    ok &= report("codeowners", &repository::codeowners(ctx)?);
    Ok(ok)
}

fn run_docs(ctx: &Ctx) -> Result<bool, String> {
    let mut ok = true;
    for (name, findings) in docs::check(&ctx.repo)? {
        ok &= report(name, &findings);
    }
    Ok(ok)
}

fn run_attribution(args: &Args) -> Result<bool, String> {
    let range = if let Some(head) = option(args, "--head") {
        let body = option(args, "--body-file")
            .map(std::fs::read_to_string)
            .transpose()
            .map_err(|e| format!("--body-file: {e}"))?;
        attribution::Range {
            base: option(args, "--base").map(str::to_owned),
            head: head.to_owned(),
            title: option(args, "--title").map(str::to_owned),
            body,
        }
    } else if let Some((name, value)) = event()? {
        attribution::Range::from_event(&name, &value)?
    } else {
        return Err("attribution: pass --head (and --base) outside GitHub Actions".to_owned());
    };
    let (findings, summary) = attribution::scan_range(&args.ctx, &range)?;
    println!(
        "scanned {} commit(s), {} added line(s), title={}, body={}",
        summary.commits,
        summary.added_lines,
        if range.title.is_some() { "yes" } else { "no" },
        if range.body.is_some() { "yes" } else { "no" },
    );
    let ok = report("attribution policy (D-52)", &findings);
    if !ok {
        println!("Reword the commits, title or body; CI never rewrites them.");
    }
    Ok(ok)
}

fn pull_request_event() -> Result<serde_json::Value, String> {
    match event()? {
        Some((name, value)) if name == "pull_request" || name == "pull_request_review" => Ok(value),
        Some((name, _)) => Err(format!("unsupported event {name}")),
        None => Err("needs a GitHub pull_request event".to_owned()),
    }
}

fn run(args: &Args) -> Result<bool, String> {
    match args.command.as_str() {
        "policy" => run_policy(&args.ctx),
        "provenance" => Ok(report(
            "provenance/secrets/data (D-56)",
            &repository::provenance(&args.ctx)?,
        )),
        "docs" => run_docs(&args.ctx),
        "attribution" => run_attribution(args),
        "review" => {
            let evidence = option(args, "--evidence").ok_or("review: --evidence REV required")?;
            let value = pull_request_event()?;
            let pull = &value["pull_request"];
            let findings = review_gates::independent_review(
                &args.ctx.repo,
                evidence,
                pull["base"]["sha"].as_str().unwrap_or_default(),
                pull["head"]["sha"].as_str().unwrap_or_default(),
            )?;
            Ok(report("independent-review", &findings))
        }
        "approval" => {
            let value = pull_request_event()?;
            Ok(report(
                "policy-approval",
                &review_gates::policy_approval(&args.ctx, &value)?,
            ))
        }
        "all" => {
            let policy = run_policy(&args.ctx)?;
            let provenance = report(
                "provenance/secrets/data (D-56)",
                &repository::provenance(&args.ctx)?,
            );
            let docs = run_docs(&args.ctx)?;
            Ok(policy && provenance && docs)
        }
        other => Err(format!("unknown command {other}")),
    }
}

fn main() -> ExitCode {
    let args = match parse() {
        Ok(args) => args,
        Err(message) => {
            if !message.is_empty() {
                eprintln!("error: {message}");
            }
            eprint!("{HELP}");
            return ExitCode::from(2);
        }
    };
    match run(&args) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(message) => {
            report("configuration", &[Finding::general(message)]);
            ExitCode::from(2)
        }
    }
}
