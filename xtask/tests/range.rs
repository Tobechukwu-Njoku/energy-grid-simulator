//! End-to-end attribution scans over scratch repositories (R2, R3, R4, R1).

mod common;

use common::{Repo, TempDir, footer, root, trailer};
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
    };
    assert!(
        scan_range(&ctx, &range).is_err(),
        "wildcard exemptions are rejected"
    );
}
