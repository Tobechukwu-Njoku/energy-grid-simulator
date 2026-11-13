//! The shell+Perl commit-msg hook, tested from Rust with the shared fixtures
//! (`.githooks/tests/fixtures.json`), plus hook/CI parity.

mod common;

use std::path::Path;
use std::process::Command;

use common::{Repo, TempDir, root};
use xtask::attribution::{PATTERNS, Patterns};

fn fixtures() -> Vec<serde_json::Value> {
    let text =
        std::fs::read_to_string(root().join(".githooks/tests/fixtures.json")).expect("fixtures");
    serde_json::from_str(&text).expect("fixtures JSON")
}

fn bytes(fixture: &serde_json::Value, key: &str) -> Vec<u8> {
    let hex_key = format!("{key}_hex");
    if let Some(hex) = fixture[&hex_key].as_str() {
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("hex"))
            .collect()
    } else {
        fixture[key]
            .as_str()
            .expect("fixture text")
            .as_bytes()
            .to_vec()
    }
}

fn copy_hooks(target: &Path) {
    for path in ["commit-msg", "attribution-patterns.regex"] {
        std::fs::create_dir_all(target).expect("mkdir");
        std::fs::copy(root().join(".githooks").join(path), target.join(path)).expect("copy hook");
    }
}

fn run_hook(hooks: &Path, message: &Path) -> std::process::Output {
    let output = Command::new(hooks.join("commit-msg"))
        .arg(message)
        .output()
        .expect("run hook");
    let leaked = std::fs::read_dir(message.parent().expect("parent"))
        .expect("list")
        .filter_map(Result::ok)
        .any(|entry| entry.file_name().to_string_lossy().contains(".strip."));
    assert!(!leaked, "temporary file leaked");
    output
}

#[test]
fn hook_fixtures() {
    let dir = TempDir::new("hook");
    let hooks = dir.path().join("hooks with spaces");
    copy_hooks(&hooks);
    let message = dir.path().join("message with spaces");
    let all = fixtures();
    assert!(all.len() >= 100, "fixture file shrank");
    for fixture in &all {
        let name = fixture["name"].as_str().unwrap_or("?");
        std::fs::write(&message, bytes(fixture, "input")).expect("write");
        let output = run_hook(&hooks, &message);
        let expect_fail = fixture["exit"].as_i64() != Some(0);
        assert_eq!(
            !output.status.success(),
            expect_fail,
            "{name}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            std::fs::read(&message).expect("read"),
            bytes(fixture, "expected"),
            "{name}"
        );
        if let Some(stderr) = fixture["stderr"].as_str() {
            assert!(
                String::from_utf8_lossy(&output.stderr).contains(stderr),
                "{name}"
            );
        }
    }
}

#[test]
fn hook_pattern_file_robustness() {
    let dir = TempDir::new("hook patterns");
    let hooks = dir.path().join("hooks");
    copy_hooks(&hooks);
    let patterns = hooks.join("attribution-patterns.regex");
    let original = std::fs::read(&patterns).expect("patterns");
    let message = dir.path().join("message");
    let text = format!(
        "fix: pattern-file robustness\n\n{}\n",
        ["Gener", "ated with GPT6"].concat()
    );
    let crlf = String::from_utf8(original.clone())
        .expect("utf8")
        .replace('\n', "\r\n")
        .into_bytes();
    let mut padded = original.clone();
    padded.extend_from_slice(b"\n  \n# comment\n");
    for variant in [padded, crlf] {
        std::fs::write(&patterns, &variant).expect("write");
        std::fs::write(&message, &text).expect("write");
        assert!(run_hook(&hooks, &message).status.success());
        assert_eq!(
            std::fs::read(&message).expect("read"),
            b"fix: pattern-file robustness\n\n"
        );
    }
    for suffix in [&b".*\n"[..], b"[\n", b"\xff\n"] {
        let mut variant = original.clone();
        variant.extend_from_slice(suffix);
        std::fs::write(&patterns, &variant).expect("write");
        std::fs::write(&message, &text).expect("write");
        assert!(!run_hook(&hooks, &message).status.success(), "{suffix:?}");
        assert_eq!(
            std::fs::read(&message).expect("read"),
            text.as_bytes(),
            "failed load rewrote message"
        );
    }
    std::fs::write(&patterns, b"# no rules\n\n").expect("write");
    std::fs::write(&message, &text).expect("write");
    assert!(!run_hook(&hooks, &message).status.success());
}

#[test]
fn hook_in_git_commits_merges_and_encodings() {
    let dir = TempDir::new("hook git");
    let hooks = dir.path().join("hooks");
    copy_hooks(&hooks);
    let repo = Repo::new();
    repo.git(&[
        "config",
        "core.hooksPath",
        hooks.to_str().expect("utf8 path"),
    ]);
    let message = dir.path().join("message");
    let footer = ["Gener", "ated with GPT6"].concat();
    for merge in [false, true] {
        if merge {
            let base = repo.git(&["rev-parse", "HEAD"]);
            repo.git(&["checkout", "-q", "-b", "merge-source"]);
            repo.git(&["commit", "-q", "--allow-empty", "-m", "test: source side"]);
            repo.git(&["checkout", "-q", "-b", "merge-destination", &base]);
            repo.git(&[
                "commit",
                "-q",
                "--allow-empty",
                "-m",
                "test: destination side",
            ]);
            repo.git(&["merge", "-q", "--no-ff", "--no-commit", "merge-source"]);
        }
        std::fs::write(
            &message,
            format!("fix: preserve human trailers\n\n{footer}\nCo-authored-by: Jane Doe <jane@google.com>\n"),
        )
        .expect("write");
        repo.git(&[
            "-c",
            "commit.cleanup=verbatim",
            "commit",
            "-q",
            "--allow-empty",
            "-F",
            message.to_str().expect("utf8"),
        ]);
        let committed = repo.git(&["log", "-1", "--format=%B"]);
        assert!(!committed.contains("ated with") && committed.contains("Jane Doe"));
        if merge {
            assert_eq!(
                repo.git(&["rev-list", "--parents", "-n", "1", "HEAD"])
                    .split_whitespace()
                    .count(),
                3
            );
        }
    }
    std::fs::write(&message, format!("{footer}\n\nBody\n")).expect("write");
    assert!(
        !repo
            .run(&[
                "commit",
                "--allow-empty",
                "-F",
                message.to_str().expect("utf8")
            ])
            .status
            .success()
    );
    repo.git(&["config", "i18n.commitEncoding", "ISO-8859-1"]);
    std::fs::write(&message, b"fix: caf\xe9 encoding\n").expect("write");
    assert!(
        !repo
            .run(&[
                "commit",
                "--allow-empty",
                "-F",
                message.to_str().expect("utf8")
            ])
            .status
            .success()
    );
    assert_eq!(
        std::fs::read(&message).expect("read"),
        b"fix: caf\xe9 encoding\n"
    );
}

/// CI rejects every message the hook would change or refuse. CI is stricter
/// by design (Unicode folding, comment and Markdown prefixes, no scissors
/// truncation for stored messages); every extra rejection is listed here.
#[test]
fn hook_and_ci_parity() {
    let patterns =
        Patterns::load(&std::fs::read(root().join(PATTERNS)).expect("patterns")).expect("load");
    let dir = TempDir::new("parity");
    let hooks = dir.path().join("hooks");
    copy_hooks(&hooks);
    let message = dir.path().join("message");
    let mut stricter = Vec::new();
    let (mut flagged, mut kept) = (0, 0);
    for fixture in fixtures() {
        let name = fixture["name"].as_str().unwrap_or("?").to_owned();
        let raw = bytes(&fixture, "input");
        std::fs::write(&message, &raw).expect("write");
        let output = run_hook(&hooks, &message);
        let hook_flags = !output.status.success() || std::fs::read(&message).expect("read") != raw;
        let ci_flags = match std::str::from_utf8(&raw) {
            Ok(text) => !patterns.scan_text(text).is_empty(),
            Err(_) => true,
        };
        if hook_flags {
            flagged += 1;
            assert!(ci_flags, "{name}: hook flags it but CI does not");
        } else {
            kept += 1;
            if ci_flags {
                stricter.push(name);
            }
        }
    }
    assert!(flagged > 0 && kept > 0);
    assert_eq!(
        stricter, EXPECTED_STRICTER,
        "update the list only for intended CI-only rejections"
    );
}

// "scissors": stored messages are scanned past a scissors line (R4).
const EXPECTED_STRICTER: &[&str] = &["scissors"];
