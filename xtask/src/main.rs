//! `cargo xtask <check>`: repository policy checks and CI gates (D-73).

#![forbid(unsafe_code)]

use std::path::PathBuf;
use std::process::ExitCode;

use xtask::util::{Ctx, Finding, report};
use xtask::{attribution, build_gate, docs, repository, review_gates, rust_policy, workflows};

const HELP: &str = "\
usage: cargo xtask [--repo DIR] [--policy DIR] <command> [options]

  --repo DIR     tree under test (default: current directory)
  --policy DIR   trusted policy tree (default: --repo). CI passes the base branch.

commands:
  policy         crate-root forbid over `cargo metadata`, manifests, workflows, Python,
                 CODEOWNERS
  build-gate     build the workspace (default, all and no default features) and each
                 member alone (default, no default and each single feature), as
                 production targets, all targets and doctests, with the compiler
                 forbidding unsafe code outside the reviewed adapters of the trusted
                 policy and a linker guard accepting only allow-listed libraries
                 resolved from trusted directories
                 [--target-dir DIR] [--jobs N]. Runs the tree's build scripts and
                 doctests: CI runs it only as the last step of an unprivileged job,
                 never in the trusted gates
  provenance     dataset manifests, raw/research-only data, secrets (D-56)
  docs           Markdown links and requirement traceability
  attribution    AI-attribution scan (D-52) of a range:
                   --base REV --head REV [--title T] [--body-file F]
                 or, in GitHub Actions, the range and metadata of the event;
                 --base-ref REV replaces the event base (CI: current main)
  review         independent-review evidence gate: --main REV (current main,
                 also the evidence tree); head from the pull-request event
  approval       protected-path owner approval gate: --main REV; event fields
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
    ok &= report("unsafe-scope", &rust_policy::check_sources(ctx)?);
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
        let mut range = attribution::Range::from_event(&name, &value)?;
        if let Some(base) = option(args, "--base-ref") {
            range.base = Some(base.to_owned());
        }
        range
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
        Some((name, value))
            if matches!(
                name.as_str(),
                "pull_request" | "pull_request_target" | "pull_request_review"
            ) =>
        {
            Ok(value)
        }
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
        "build-gate" => {
            let wrapper = std::env::current_exe().map_err(|e| format!("current_exe: {e}"))?;
            let target = option(args, "--target-dir").map_or_else(
                || args.ctx.repo.join("target").join("build-gate"),
                PathBuf::from,
            );
            let jobs = option(args, "--jobs")
                .map(|jobs| {
                    jobs.parse()
                        .map_err(|_| format!("--jobs {jobs}: not a number"))
                })
                .transpose()?;
            Ok(report(
                "build-gate: compiler-enforced unsafe scope and native runtimes",
                &build_gate::check_with(
                    &args.ctx,
                    &wrapper,
                    &target,
                    &build_gate::Options { jobs },
                )?,
            ))
        }
        "attribution" => run_attribution(args),
        "review" => {
            let main = option(args, "--main").ok_or("review: --main REV required")?;
            let value = pull_request_event()?;
            let head = value["pull_request"]["head"]["sha"]
                .as_str()
                .unwrap_or_default();
            let findings = review_gates::independent_review(&args.ctx, main, head)?;
            Ok(report("independent-review", &findings))
        }
        "approval" => {
            let main = option(args, "--main").ok_or("approval: --main REV required")?;
            let value = pull_request_event()?;
            let pull = review_gates::PullRequest::from_event(&value)?;
            let (files, jobs) = workflows::Policy::load(&args.ctx)?.trusted_gates();
            Ok(report(
                "policy-approval",
                &review_gates::policy_approval(&args.ctx, main, &pull, &files, &jobs)?,
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
    if let Some(code) = build_gate::dispatch() {
        return code;
    }
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
