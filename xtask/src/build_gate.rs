//! Compiler-enforced unsafe boundary and native-runtime ban (ADR 0001, NFR-10).
//!
//! `cargo xtask build-gate` builds the whole workspace under test (every
//! target, every feature, build scripts and proc macros included) with the
//! trusted xtask binary as `RUSTC_WRAPPER`. The wrapper and the flags it adds
//! come from the trusted tree; the tree under test cannot weaken them:
//!
//! - Every rustc invocation for a crate whose package lives in the repository
//!   gets `-F unsafe_code` appended, except the library/test targets of a
//!   `crates/solver-*` adapter. A command-line `forbid` cannot be lowered by
//!   any attribute, `[lints]` entry or macro expansion; only `--cap-lints`
//!   could, so the wrapper removes it for repository crates and refuses
//!   argument files. Macro-generated `allow`s, `include!`d files (renamed
//!   macro or not, `OUT_DIR` or not), `#[path]` modules, glob-selected
//!   members and patched build-dependencies are all just code the compiler
//!   sees (R5-01..04, ADV23R5-02/03/05).
//! - Every rustc invocation gets `-C linker=<xtask>`: the linker guard refuses
//!   any linker input naming a Python or Julia library and otherwise runs the
//!   platform `cc`. Link requests from `#[link]` attributes, however they were
//!   generated (ADV23R5-04), and from build scripts reach the linker.
//! - Build-script output is read from Cargo's JSON messages
//!   (`build-script-executed`): a `rustc-link-lib` or a `rustc-link-search`
//!   directory holding a Python or Julia library is a finding even when no
//!   executable links it.
//!
//! Cargo runs from the trusted tree (no repository `.cargo/config` is
//! loaded) with every Cargo/rustc flag, wrapper and lint-cap variable removed
//! from its environment. This runs the pull request's build scripts and
//! proc macros, so it belongs in an unprivileged CI job, never in the trusted
//! gates. Code that runs at build time shares the job's user and could tamper
//! with the job itself; that residual risk is recorded in ADR 0001.

use std::ffi::OsString;
use std::io::{BufRead as _, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

use crate::rust_policy::{cargo, is_adapter_dir};
use crate::util::{Ctx, Finding};

/// Set in the wrapper's environment: the canonical repository root.
pub const WRAP_ENV: &str = "XTASK_BUILD_GATE_REPO";
/// Set by the wrapper for rustc, which passes it on to the linker.
pub const LINK_ENV: &str = "XTASK_BUILD_GATE_LINKER";

/// Whether a library name or file names a Python or Julia runtime.
#[must_use]
pub fn names_runtime(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("python") || lower.contains("julia")
}

/// Entry point used by `main` before argument parsing: runs the rustc
/// wrapper or the linker guard when the build gate invoked this binary.
#[must_use]
pub fn dispatch() -> Option<ExitCode> {
    let args: Vec<OsString> = std::env::args_os().skip(1).collect();
    if std::env::var_os(LINK_ENV).is_some() {
        return Some(link(&args));
    }
    let repo = std::env::var_os(WRAP_ENV)?;
    Some(wrap(Path::new(&repo), &args))
}

fn exit(status: std::io::Result<std::process::ExitStatus>, what: &str) -> ExitCode {
    match status {
        Ok(status) => ExitCode::from(u8::try_from(status.code().unwrap_or(1)).unwrap_or(1)),
        Err(error) => {
            eprintln!("error: build gate could not run {what}: {error}");
            ExitCode::from(1)
        }
    }
}

/// How the wrapper changes one rustc command line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Wrapped {
    /// Run with these arguments.
    Run(Vec<String>),
    /// Refuse, with this reason.
    Refuse(String),
}

/// Rewrites one rustc argument list (without the rustc path).
/// `manifest_dir` is the crate's package directory relative to the
/// repository, `None` if the package is outside it (a registry crate).
#[must_use]
pub fn rewrite(args: &[String], manifest_dir: Option<&str>, linker: &str) -> Wrapped {
    let crate_name = args
        .iter()
        .position(|arg| arg == "--crate-name")
        .and_then(|index| args.get(index + 1));
    let Some(crate_name) = crate_name.filter(|name| *name != "___") else {
        return Wrapped::Run(args.to_vec()); // version and target-info probes
    };
    let mut out = Vec::with_capacity(args.len() + 4);
    if let Some(dir) = manifest_dir {
        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            if arg.starts_with('@') {
                return Wrapped::Refuse(format!(
                    "argument file `{arg}` for repository crate `{crate_name}`"
                ));
            }
            if arg == "--cap-lints" {
                iter.next();
                continue;
            }
            if arg.starts_with("--cap-lints=") {
                continue;
            }
            out.push(arg.clone());
        }
        let build_script = crate_name.starts_with("build_script_");
        if build_script || !is_adapter_dir(dir) {
            out.extend(["-F".to_owned(), "unsafe_code".to_owned()]);
        }
    } else {
        out.extend(args.iter().cloned());
    }
    out.extend(["-C".to_owned(), format!("linker={linker}")]);
    Wrapped::Run(out)
}

fn wrap(repo: &Path, args: &[OsString]) -> ExitCode {
    let Some((rustc, rest)) = args.split_first() else {
        eprintln!("error: build gate wrapper called without rustc");
        return ExitCode::from(1);
    };
    let Some(rest) = rest
        .iter()
        .map(|arg| arg.to_str().map(str::to_owned))
        .collect::<Option<Vec<_>>>()
    else {
        eprintln!("error: build gate wrapper: non-UTF-8 rustc argument");
        return ExitCode::from(1);
    };
    // Cargo sets the package directory for every unit; without it the unit
    // is treated as repository code (fail closed).
    let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR").map_or_else(
        || Some(String::new()),
        |dir| {
            let dir = PathBuf::from(dir);
            let dir = dir.canonicalize().unwrap_or(dir);
            dir.strip_prefix(repo)
                .ok()
                .map(|rest| rest.to_string_lossy().replace('\\', "/"))
        },
    );
    let exe = match std::env::current_exe() {
        Ok(exe) => exe.to_string_lossy().into_owned(),
        Err(error) => {
            eprintln!("error: build gate wrapper: {error}");
            return ExitCode::from(1);
        }
    };
    match rewrite(&rest, manifest_dir.as_deref(), &exe) {
        Wrapped::Run(args) => exit(
            Command::new(rustc)
                .args(&args)
                .env(LINK_ENV, "1")
                .env_remove(WRAP_ENV)
                .status(),
            "rustc",
        ),
        Wrapped::Refuse(reason) => {
            eprintln!("error: build gate refuses {reason}");
            ExitCode::from(1)
        }
    }
}

/// Library names and files a linker command line asks for. `@file`
/// arguments are expanded by `read`; `-Wl,` lists are split.
#[must_use]
pub fn linked(args: &[String], read: &dyn Fn(&str) -> Option<String>) -> Vec<String> {
    let mut expanded = Vec::new();
    for arg in args {
        match arg.strip_prefix('@').and_then(read) {
            Some(text) => expanded.extend(
                text.split(['\n', '\r', ' ', '\t'])
                    .filter(|part| !part.is_empty())
                    .map(|part| part.trim_matches('"').to_owned()),
            ),
            None => expanded.push(arg.clone()),
        }
    }
    let mut pieces = Vec::new();
    for arg in expanded {
        match arg.strip_prefix("-Wl,") {
            Some(list) => pieces.extend(list.split(',').map(str::to_owned)),
            None => pieces.push(arg),
        }
    }
    let mut out = Vec::new();
    let mut iter = pieces.iter();
    while let Some(piece) = iter.next() {
        if piece == "-l" || piece == "--library" {
            if let Some(name) = iter.next() {
                out.push(name.clone());
            }
        } else if let Some(name) = piece
            .strip_prefix("--library=")
            .or_else(|| piece.strip_prefix("-l"))
        {
            out.push(name.to_owned());
        } else if !piece.starts_with('-') {
            let file = piece.rsplit(['/', '\\']).next().unwrap_or(piece);
            let library = file.starts_with("lib")
                || file.ends_with(".so")
                || file.contains(".so.")
                || file.ends_with(".a")
                || file.ends_with(".dylib")
                || file.ends_with(".tbd");
            if library {
                out.push(file.to_owned());
            }
        }
    }
    out
}

fn link(args: &[OsString]) -> ExitCode {
    let text: Vec<String> = args
        .iter()
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect();
    let read = |path: &str| std::fs::read_to_string(path).ok();
    let refused: Vec<String> = linked(&text, &read)
        .into_iter()
        .filter(|name| names_runtime(name))
        .collect();
    if !refused.is_empty() {
        eprintln!(
            "error: NFR-10: linking a Python or Julia runtime is forbidden; the linker was asked for {}",
            refused.join(", ")
        );
        return ExitCode::from(1);
    }
    exit(
        Command::new("cc").args(args).env_remove(LINK_ENV).status(),
        "the linker (cc)",
    )
}

fn package_name(id: &str) -> String {
    // `path+file:///r/crates/engine#egs-engine@0.0.0`, or `...#0.0.0` when
    // the name is the directory name, or `registry+...#name@1.0.0`.
    let (url, tail) = id.rsplit_once('#').unwrap_or((id, ""));
    match tail.split_once('@') {
        Some((name, _)) => name.to_owned(),
        None => url.rsplit('/').next().unwrap_or(id).to_owned(),
    }
}

/// Findings from Cargo's JSON message stream.
#[must_use]
pub fn analyse(messages: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    for line in messages.lines() {
        let Ok(message) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        let package = package_name(message["package_id"].as_str().unwrap_or_default());
        match message["reason"].as_str() {
            Some("build-script-executed") => {
                for library in message["linked_libs"].as_array().into_iter().flatten() {
                    let library = library.as_str().unwrap_or_default();
                    if names_runtime(library) {
                        findings.push(Finding::general(format!(
                            "build script of `{package}` asks Cargo to link `{library}`: linking a Python or Julia runtime is forbidden (NFR-10)"
                        )));
                    }
                }
                for search in message["linked_paths"].as_array().into_iter().flatten() {
                    let search = search.as_str().unwrap_or_default();
                    let dir = search.split_once('=').map_or(search, |(_, dir)| dir);
                    let Ok(entries) = std::fs::read_dir(dir) else {
                        continue;
                    };
                    for entry in entries.flatten() {
                        let name = entry.file_name().to_string_lossy().into_owned();
                        if names_runtime(&name)
                            && !linked(std::slice::from_ref(&name), &|_| None).is_empty()
                        {
                            findings.push(Finding::general(format!(
                                "build script of `{package}` adds link search path `{dir}` holding `{name}`: linking a Python or Julia runtime is forbidden (NFR-10)"
                            )));
                        }
                    }
                }
            }
            Some("compiler-message") if message["message"]["level"] == "error" => {
                let inner = &message["message"];
                let span = inner["spans"]
                    .as_array()
                    .and_then(|spans| spans.iter().find(|s| s["is_primary"] == true));
                let mut text = format!(
                    "`{package}`: {}",
                    inner["message"].as_str().unwrap_or("compiler error")
                );
                let rendered = inner["rendered"].as_str().unwrap_or_default();
                for line in rendered.lines().filter(|line| line.contains("NFR-10")) {
                    text.push_str(" | ");
                    text.push_str(line.trim());
                }
                findings.push(match span {
                    Some(span) => Finding::at(
                        span["file_name"].as_str().unwrap_or_default(),
                        span["line_start"]
                            .as_u64()
                            .and_then(|line| usize::try_from(line).ok()),
                        text,
                    ),
                    None => Finding::general(text),
                });
            }
            _ => {}
        }
    }
    findings
}

/// Builds the workspace under test with the compiler enforcing the unsafe
/// boundary and the linker guard enforcing the native-runtime ban.
/// `wrapper` is the trusted xtask binary; `target_dir` receives the build.
///
/// # Errors
///
/// Fails if Cargo cannot be started.
pub fn check(ctx: &Ctx, wrapper: &Path, target_dir: &Path) -> Result<Vec<Finding>, String> {
    let repo = ctx
        .repo
        .canonicalize()
        .map_err(|error| format!("{}: {error}", ctx.repo.display()))?;
    let mut child = cargo(ctx)
        .args([
            "build",
            "--workspace",
            "--all-targets",
            "--all-features",
            "--locked",
            "--keep-going",
            "--message-format=json",
            "--manifest-path",
        ])
        .arg(repo.join("Cargo.toml"))
        .arg("--target-dir")
        .arg(target_dir)
        .env("RUSTC_WRAPPER", wrapper)
        .env(WRAP_ENV, &repo)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("cargo build: {error}"))?;
    let stderr = child.stderr.take().map(|stderr| {
        std::thread::spawn(move || {
            let mut text = String::new();
            for line in BufReader::new(stderr).lines().map_while(Result::ok) {
                text.push_str(&line);
                text.push('\n');
            }
            text
        })
    });
    let mut messages = String::new();
    if let Some(stdout) = child.stdout.take() {
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            messages.push_str(&line);
            messages.push('\n');
        }
    }
    let status = child
        .wait()
        .map_err(|error| format!("cargo build: {error}"))?;
    let stderr = stderr
        .and_then(|handle| handle.join().ok())
        .unwrap_or_default();
    let mut findings = analyse(&messages);
    if !status.success() && findings.is_empty() {
        let tail: Vec<&str> = stderr.lines().rev().take(20).collect();
        let tail: Vec<&str> = tail.into_iter().rev().collect();
        findings.push(Finding::general(format!(
            "cargo build failed under the build gate: {}",
            tail.join("\n")
        )));
    }
    for finding in &mut findings {
        if let Some(file) = &finding.file
            && let Ok(rest) = Path::new(file).strip_prefix(&repo)
        {
            finding.file = Some(rest.to_string_lossy().into_owned());
        }
    }
    Ok(findings)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(args: &[&str]) -> Vec<String> {
        args.iter().map(|arg| (*arg).to_owned()).collect()
    }

    #[test]
    fn wrapper_forbids_unsafe_and_strips_lint_caps_for_repository_crates() {
        let args = strings(&[
            "--crate-name",
            "egs_engine",
            "--cap-lints",
            "allow",
            "--cap-lints=warn",
            "-A",
            "unsafe_code",
        ]);
        let Wrapped::Run(out) = rewrite(&args, Some("crates/engine"), "/x") else {
            panic!("refused");
        };
        assert!(!out.iter().any(|arg| arg.contains("cap-lints")), "{out:?}");
        assert_eq!(
            out[out.len() - 4..],
            strings(&["-F", "unsafe_code", "-C", "linker=/x"])
        );
        // Adapters are exempt, their build scripts are not.
        let Wrapped::Run(out) = rewrite(&args, Some("crates/solver-highs"), "/x") else {
            panic!("refused");
        };
        assert!(!out.contains(&"-F".to_owned()));
        let script = strings(&["--crate-name", "build_script_build"]);
        let Wrapped::Run(out) = rewrite(&script, Some("crates/solver-highs"), "/x") else {
            panic!("refused");
        };
        assert!(out.contains(&"-F".to_owned()));
        // Registry crates keep Cargo's cap and only get the linker guard.
        let Wrapped::Run(out) = rewrite(&args, None, "/x") else {
            panic!("refused");
        };
        assert!(out.contains(&"--cap-lints".to_owned()));
        assert!(!out.contains(&"-F".to_owned()));
        // Probes pass through; argument files are refused.
        let probe = strings(&["-", "--crate-name", "___", "--print=file-names"]);
        assert_eq!(rewrite(&probe, Some(""), "/x"), Wrapped::Run(probe.clone()));
        assert!(matches!(
            rewrite(
                &strings(&["--crate-name", "a", "@flags"]),
                Some("crates/a"),
                "/x"
            ),
            Wrapped::Refuse(_)
        ));
        assert!(is_adapter_dir("crates/solver-highs"));
        assert!(!is_adapter_dir("crates/solver-"));
        assert!(!is_adapter_dir("crates/solver-x/sub"));
        assert!(!is_adapter_dir("spikes/solver-x"));
    }

    #[test]
    fn linker_inputs_are_found_in_every_spelling() {
        let files = |path: &str| (path == "rsp").then(|| "\"-lpython3.12\"\n-lm\n".to_owned());
        let args = strings(&[
            "-lfoo",
            "-l",
            "julia",
            "-Wl,-Bdynamic,-lpython3",
            "-Wl,--library=libjulia",
            "/usr/lib/libpython3.12.so.1.0",
            "x.o",
            "@rsp",
        ]);
        let names = linked(&args, &files);
        let runtime: Vec<&String> = names.iter().filter(|n| names_runtime(n)).collect();
        assert_eq!(runtime.len(), 5, "{names:?}");
        assert!(names.contains(&"foo".to_owned()));
        assert!(!names.contains(&"x.o".to_owned()));
    }

    #[test]
    fn cargo_messages_are_analysed() {
        let messages = [
            r#"{"reason":"build-script-executed","package_id":"path+file:///r/crates/engine#egs-engine@0.0.0","linked_libs":["dylib=python3.12","m"],"linked_paths":[]}"#,
            r#"{"reason":"compiler-message","package_id":"path+file:///r/crates/x#x@0.0.0","message":{"level":"error","message":"usage of an `unsafe` block","rendered":"error: usage of an `unsafe` block","spans":[{"is_primary":true,"file_name":"src/lib.rs","line_start":3}]}}"#,
            r#"{"reason":"compiler-message","package_id":"x","message":{"level":"warning","rendered":"w","spans":[]}}"#,
            "not json",
        ]
        .join("\n");
        let found = analyse(&messages);
        assert_eq!(found.len(), 2, "{found:?}");
        assert!(found[0].message.contains("egs-engine"));
        assert_eq!(found[1].line, Some(3));
        assert_eq!(found[1].message, "`x`: usage of an `unsafe` block");
        assert_eq!(package_name("path+file:///r/xtask#0.0.0"), "xtask");
    }
}
