//! End-to-end attribution scans over scratch repositories (R2, R3, R4, R1).

#![forbid(unsafe_code)]

mod common;

use common::{Repo, TempDir, footer, j, root, trailer};
use xtask::attribution::{Range, scan_range};
use xtask::util::Ctx;

/// A trusted policy tree holding the real pattern file and exemptions.
fn trusted() -> TempDir {
    let dir = TempDir::new("policy");
    for path in [
        ".githooks/attribution-patterns.regex",
        "xtask/policy/attribution-exemptions.toml",
    ] {
        let target = dir.path().join(path);
        std::fs::create_dir_all(target.parent().expect("parent")).expect("mkdir");
        std::fs::copy(root().join(path), target).expect("copy policy");
    }
    dir
}

fn scan(repo: &Repo, policy: &TempDir, base: &str, title: Option<&str>) -> Vec<String> {
    let ctx = Ctx {
        repo: repo.path().to_path_buf(),
        policy: policy.path().to_path_buf(),
    };
    let range = Range {
        base: Some(base.to_owned()),
        head: repo.git(&["rev-parse", "HEAD"]),
        title: title.map(str::to_owned),
        body: None,
        body_history: None,
    };
    let (findings, _) = scan_range(&ctx, &range).expect("scan");
    findings
        .into_iter()
        .map(|f| {
            format!(
                "{}:{}: {}",
                f.file.unwrap_or_default(),
                f.line.unwrap_or(0),
                f.message
            )
        })
        .collect()
}

fn setup() -> (Repo, TempDir, String) {
    let repo = Repo::new();
    repo.write("README.md", "base\n");
    let base = repo.commit("chore: base");
    (repo, trusted(), base)
}

#[test]
fn clean_range_passes() {
    let (repo, policy, base) = setup();
    repo.write("src/lib.rs", "// ordinary comment\n");
    repo.commit("feat: add library");
    assert!(scan(&repo, &policy, &base, Some("feat: add library")).is_empty());
}

// R2 / ADV23-01: a source comment carrying the footer is rejected.
#[test]
fn r2_source_comment_is_rejected() {
    let (repo, policy, base) = setup();
    repo.write("src/main.rs", &format!("fn main() {{}}\n// {}\n", footer()));
    repo.commit("feat: main");
    let found = scan(&repo, &policy, &base, None);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].starts_with("src/main.rs:2:"), "{found:?}");
}

// R3: head-controlled .gitattributes cannot hide added lines.
#[test]
fn r3_gitattributes_cannot_hide_added_lines() {
    for attributes in ["*.rs -diff\n", "*.rs binary\n", "*.rs diff=hide\n"] {
        let (repo, policy, base) = setup();
        repo.git(&["config", "diff.hide.textconv", "true"]);
        repo.write(".gitattributes", attributes);
        repo.write("src/lib.rs", &format!("/*\n * {}\n */\n", footer()));
        repo.commit("feat: lib");
        let found = scan(&repo, &policy, &base, None);
        assert_eq!(found.len(), 1, "{attributes}: {found:?}");
        assert!(found[0].starts_with("src/lib.rs:2:"), "{found:?}");
    }
}

// R4 / ADV23-08: a stored message keeps text after a scissors line.
#[test]
fn r4_stored_scissors_message_is_scanned() {
    let (repo, policy, base) = setup();
    repo.commit(&format!(
        "fix: clean subject\n\n# ------------------------ >8 ------------------------\n{}\n",
        trailer()
    ));
    let found = scan(&repo, &policy, &base, None);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("message line 4"), "{found:?}");
}

#[test]
fn merge_message_and_title_are_scanned() {
    let (repo, policy, base) = setup();
    repo.git(&["checkout", "-q", "-b", "side"]);
    repo.commit("feat: side");
    repo.git(&["checkout", "-q", "main"]);
    repo.commit("feat: main");
    let message = format!("merge side\n\n{}", footer());
    repo.git(&["merge", "-q", "--no-ff", "side", "-m", &message]);
    assert_eq!(scan(&repo, &policy, &base, None).len(), 1);
    assert_eq!(scan(&repo, &policy, &base, Some(&footer())).len(), 2);
}

// ADV23-09: reported line numbers match the new file.
#[test]
fn adv23_09_line_numbers_match_new_file() {
    let (repo, policy, base) = setup();
    repo.write(
        "NOTES.md",
        &(1..=30).map(|n| format!("line {n}\n")).collect::<String>(),
    );
    let base2 = repo.commit("docs: notes");
    let mut text: String = (1..=30).map(|n| format!("line {n}\n")).collect();
    text = text.replace("line 20\n", &format!("line 20\n{}\n", footer()));
    repo.write("NOTES.md", &text);
    repo.commit("docs: more");
    let found = scan(&repo, &policy, &base2, None);
    assert_eq!(found.len(), 1);
    assert!(found[0].starts_with("NOTES.md:21:"), "{found:?}");
    let _ = base;
}

// R1 / ADV23-17: the head cannot weaken the trusted patterns or add exemptions.
#[test]
fn r1_head_cannot_weaken_trusted_policy() {
    let (repo, policy, base) = setup();
    repo.write(
        ".githooks/attribution-patterns.regex",
        "^never-matches-anything$\n",
    );
    repo.write(
        "xtask/policy/attribution-exemptions.toml",
        "[[exemption]]\npath = \"NOTES.md\"\nline = \"x\"\nreason = \"x\"\n",
    );
    repo.write("NOTES.md", &format!("{}\n", footer()));
    repo.write("docs/process/team.md", &format!("{}\n", footer()));
    repo.commit("chore: weaken policy");
    assert_eq!(scan(&repo, &policy, &base, None).len(), 2);
}

// ADV23-17: an exemption covers one exact line in one file only.
#[test]
fn adv23_17_exemptions_are_line_scoped() {
    let (repo, policy, base) = setup();
    let exempt = format!("- {}", footer());
    std::fs::write(
        policy
            .path()
            .join("xtask/policy/attribution-exemptions.toml"),
        format!(
            "[[exemption]]\npath = \"docs/a.md\"\nline = {exempt:?}\nreason = \"policy example\"\n"
        ),
    )
    .expect("write exemptions");
    repo.write("docs/a.md", &format!("{exempt}\n{}\n", footer()));
    repo.write("docs/b.md", &format!("{exempt}\n"));
    repo.commit("docs: examples");
    let found = scan(&repo, &policy, &base, None);
    assert_eq!(found.len(), 2, "{found:?}");
    std::fs::write(
        policy
            .path()
            .join("xtask/policy/attribution-exemptions.toml"),
        "[[exemption]]\npath = \"docs/*\"\nline = \"x\"\nreason = \"wide\"\n",
    )
    .expect("write exemptions");
    let ctx = Ctx {
        repo: repo.path().to_path_buf(),
        policy: policy.path().to_path_buf(),
    };
    let range = Range {
        base: Some(base),
        head: repo.git(&["rev-parse", "HEAD"]),
        title: None,
        body: None,
        body_history: None,
    };
    assert!(
        scan_range(&ctx, &range).is_err(),
        "wildcard exemptions are rejected"
    );
}

// F3: attribution in a trailing comment after code is the only violation.
#[test]
fn f3_trailing_comment_is_rejected_end_to_end() {
    for line in [
        format!("pub const N: u8 = 0; // {}", footer()),
        format!("pub const M: u8 = 0; /* {} */", footer()),
    ] {
        let (repo, policy, base) = setup();
        repo.write("src/lib.rs", &format!("{line}\n"));
        repo.commit("feat: constant");
        let found = scan(&repo, &policy, &base, None);
        assert_eq!(found.len(), 1, "{line}: {found:?}");
        assert!(found[0].starts_with("src/lib.rs:1:"), "{found:?}");
    }
    let (repo, policy, base) = setup();
    repo.write("run.sh", &format!("echo ok  # {}\n", footer()));
    repo.commit("feat: script");
    assert_eq!(scan(&repo, &policy, &base, None).len(), 1);
}

// ADV23R-05: an AI author or committer identity is rejected even when the
// message and the diff are clean.
#[test]
fn adv23r_05_ai_commit_identity_is_rejected() {
    let ai = j(&["Cla", "ude Code <cla", "ude@anthropic.com>"]);
    let (repo, policy, base) = setup();
    repo.write("src/lib.rs", "pub fn solve() {}\n");
    repo.git(&["add", "-A"]);
    repo.git(&["commit", "-q", "--author", &ai, "-m", "feat: solver stub"]);
    let found = scan(&repo, &policy, &base, None);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("author is an AI identity"), "{found:?}");

    let (repo, policy, base) = setup();
    repo.write("src/lib.rs", "pub fn solve() {}\n");
    repo.git(&["add", "-A"]);
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(repo.path())
        .args(["commit", "-q", "-m", "feat: solver stub"])
        .env("GIT_COMMITTER_NAME", j(&["Gem", "ini"]))
        .env("GIT_COMMITTER_EMAIL", "bot@example.org")
        .output()
        .expect("git");
    assert!(output.status.success());
    let found = scan(&repo, &policy, &base, None);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0].contains("committer is an AI identity"),
        "{found:?}"
    );

    // The D-74 machine account is not an AI identity.
    let (repo, policy, base) = setup();
    repo.write("src/lib.rs", "pub fn solve() {}\n");
    repo.git(&["add", "-A"]);
    repo.git(&[
        "commit",
        "-q",
        "--author",
        "egs-bot <egs-bot@users.noreply.github.com>",
        "-m",
        "feat: solver stub",
    ]);
    assert!(scan(&repo, &policy, &base, None).is_empty());
}

/// Runs the CLI attribution scan on a `pull_request` event, with an optional
/// saved GraphQL edit history; `gh` gets no credentials.
fn run_event(
    repo: &Repo,
    policy: &TempDir,
    body: &str,
    history: Option<&serde_json::Value>,
    actions: bool,
) -> (i32, String) {
    let scratch = TempDir::new("event");
    let head = repo.git(&["rev-parse", "HEAD"]);
    let event = serde_json::json!({
        "action": "edited",
        "pull_request": {
            "number": 7, "title": "feat: checks", "body": body,
            "base": {"sha": head}, "head": {"sha": head},
            "user": {"login": "owner"}
        },
        "repository": {"full_name": "o/r"},
        "sender": {"login": j(&["code", "rabbitai[bot]"]), "type": "Bot"}
    });
    let event_path = scratch.path().join("event.json");
    std::fs::write(&event_path, event.to_string()).expect("event");
    let gh_config = scratch.path().join("gh");
    std::fs::create_dir_all(&gh_config).expect("gh config");
    let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_xtask"));
    command
        .arg("--repo")
        .arg(repo.path())
        .arg("--policy")
        .arg(policy.path())
        .arg("attribution")
        .env("GITHUB_EVENT_NAME", "pull_request")
        .env("GITHUB_EVENT_PATH", &event_path)
        .env("GH_CONFIG_DIR", &gh_config)
        .env_remove("GH_TOKEN")
        .env_remove("GITHUB_TOKEN")
        .env_remove("GH_ENTERPRISE_TOKEN")
        .env_remove("GITHUB_ACTIONS");
    if actions {
        command.env("GITHUB_ACTIONS", "true");
    }
    if let Some(history) = history {
        let path = scratch.path().join("history.json");
        std::fs::write(&path, history.to_string()).expect("history");
        command.arg("--body-history").arg(path);
    }
    let output = command.output().expect("run xtask");
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    )
}

/// A GraphQL response page; `edits` are (login, is bot, body), oldest first.
fn history(edits: &[(&str, bool, &str)]) -> serde_json::Value {
    let nodes: Vec<serde_json::Value> = edits
        .iter()
        .enumerate()
        .rev()
        .map(|(index, (login, bot, text))| {
            serde_json::json!({
                "editedAt": format!("2026-11-14T00:00:{index:02}Z"),
                "deletedAt": null,
                "diff": text,
                "editor": {"__typename": if *bot { "Bot" } else { "User" }, "login": login}
            })
        })
        .collect();
    serde_json::json!({"data": {"repository": {"pullRequest": {
        "body": edits.last().map(|edit| edit.2),
        "author": {"__typename": "User", "login": "owner"},
        "userContentEdits": {"pageInfo": {"hasNextPage": false}, "nodes": nodes}
    }}}})
}

// R8-01 (D-61): through the CLI event path, a review bot's PR-description
// summary block passes only when its edit history shows the bot added it.
// The reviewer's replay (same body, bot or user sender) exited 1 at bcd4b2c.
#[test]
fn r8_01_bot_summary_block_needs_verified_provenance() {
    let (repo, policy, _) = setup();
    let bot = j(&["code", "rabbitai"]);
    let block = [
        j(&[
            "<!-- This is an auto-generated comment: release notes by code",
            "rabbit.ai -->",
        ]),
        j(&["## Summary by Code", "Rabbit"]),
        String::new(),
        "* Add checks.".to_owned(),
        j(&[
            "<!-- end of auto-generated comment: release notes by code",
            "rabbit.ai -->",
        ]),
    ]
    .join("\n");
    let body = format!("Implement M0 checks.\n\n{block}\n");

    // Genuine bot block: passes.
    let genuine = history(&[
        ("owner", false, "Implement M0 checks."),
        (&bot, true, &body),
    ]);
    let (code, out) = run_event(&repo, &policy, &body, Some(&genuine), false);
    assert_eq!(code, 0, "{out}");

    // The same block written by a user: fails.
    let spoofed = history(&[
        ("owner", false, "Implement M0 checks."),
        ("owner", false, &body),
    ]);
    let (code, out) = run_event(&repo, &policy, &body, Some(&spoofed), false);
    assert_eq!(code, 1, "{out}");
    assert!(out.contains("not exempt (D-61)"), "{out}");

    // Extra credit outside a genuine block: fails on that line only.
    let extra = format!("Implement M0 checks.\n{}\n\n{block}\n", footer());
    let added = history(&[
        ("owner", false, "Implement M0 checks."),
        (&bot, true, &body),
        ("owner", false, &extra),
    ]);
    let (code, out) = run_event(&repo, &policy, &extra, Some(&added), false);
    assert_eq!(code, 1, "{out}");
    assert!(
        out.contains("pull request body line 2: attribution"),
        "{out}"
    );
    assert!(!out.contains("D-61"), "{out}");

    // No usable edit history (gh has no credentials here): fails closed.
    let (code, out) = run_event(&repo, &policy, &body, None, false);
    assert_eq!(code, 1, "{out}");
    assert!(out.contains("could not be verified"), "{out}");

    // A saved history is refused inside GitHub Actions.
    let (code, out) = run_event(&repo, &policy, &body, Some(&genuine), true);
    assert_eq!(code, 2, "{out}");

    // A body without a bot block needs no history at all.
    let (code, out) = run_event(&repo, &policy, "Implement M0 checks.\n", None, false);
    assert_eq!(code, 0, "{out}");
}
