//! Trusted-gate integration tests over scratch repositories (F1, F2, F6,
//! ADV23R-07, ADV23R-08).

#![forbid(unsafe_code)]

mod common;

use std::process::Command;

use common::{Repo, TempDir, root};
use xtask::review_gates::{self, PullRequest};
use xtask::util::Ctx;

fn bare_origin(dir: &TempDir) -> String {
    let origin = dir.path().join("origin.git");
    let status = Command::new("git")
        .args(["init", "-q", "--bare", "-b", "main"])
        .arg(&origin)
        .status()
        .expect("git init");
    assert!(status.success());
    origin.to_string_lossy().into_owned()
}

struct Loaded {
    ok: bool,
    output: String,
    log: String,
}

fn run_loader(repo: &Repo, scratch: &TempDir, base: &str, head: &str) -> Loaded {
    let event = scratch.path().join("event.json");
    std::fs::write(
        &event,
        serde_json::json!({"pull_request": {"base": {"sha": base}, "head": {"sha": head}}})
            .to_string(),
    )
    .expect("event");
    let output_file = scratch.path().join("output");
    let _ = std::fs::remove_file(&output_file);
    let runner_temp = scratch.path().join("runner");
    std::fs::create_dir_all(&runner_temp).expect("runner temp");
    let result = Command::new("bash")
        .arg(root().join(".github/actions/trusted-policy/load.sh"))
        .current_dir(repo.path())
        .env("GITHUB_EVENT_PATH", &event)
        .env("GITHUB_OUTPUT", &output_file)
        .env("RUNNER_TEMP", &runner_temp)
        .env("TRUSTED_POLICY_SKIP_TOOLCHAIN", "1")
        .output()
        .expect("run loader");
    Loaded {
        ok: result.status.success(),
        output: std::fs::read_to_string(&output_file).unwrap_or_default(),
        log: format!(
            "{}{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        ),
    }
}

fn trusted_head(output: &str) -> String {
    let dir = output
        .lines()
        .find_map(|line| line.strip_prefix("dir="))
        .expect("dir output");
    let out = Command::new("git")
        .args(["-C", dir, "rev-parse", "HEAD"])
        .output()
        .expect("rev-parse");
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

// F1 / ADV23R-08: the loader reads policy from the CURRENT main, never the
// event's stale base, and fails closed when main has no policy unless the
// exact head is the coordinator-attested bootstrap seed.
#[test]
fn f1_adv23r_08_loader_uses_current_main_and_fails_closed() {
    let scratch = TempDir::new("loader");
    let origin = bare_origin(&scratch);
    let repo = Repo::new();
    repo.git(&["remote", "add", "origin", &origin]);
    repo.write("README.md", "main\n");
    let stale_base = repo.commit("chore: main without policy");
    repo.git(&["push", "-q", "origin", "main"]);
    repo.git(&["checkout", "-q", "-b", "pr"]);
    repo.write("xtask/Cargo.toml", "[package]\nname = \"xtask\"\n");
    let head = repo.commit("feat: seed policy");

    // No policy on main and no attestation: fail closed, no fallback.
    let loaded = run_loader(&repo, &scratch, &stale_base, &head);
    assert!(!loaded.ok, "{}", loaded.log);
    assert!(loaded.log.contains("failing closed"), "{}", loaded.log);
    assert!(!loaded.output.contains("dir="));

    // An attestation for another SHA does not help.
    repo.git(&["checkout", "-q", "main"]);
    repo.write(
        "docs/process/ci-bootstrap.toml",
        &format!("seed_sha = \"{}\"\n", "b".repeat(40)),
    );
    repo.commit("docs: attest another seed");
    repo.git(&["push", "-q", "origin", "main"]);
    assert!(!run_loader(&repo, &scratch, &stale_base, &head).ok);

    // A PR cannot attest itself: the file is read from main only.
    repo.git(&["checkout", "-q", "pr"]);
    repo.write(
        "docs/process/ci-bootstrap.toml",
        &format!("seed_sha = \"{head}\"\n"),
    );
    let self_attested = repo.commit("docs: self attestation");
    assert!(!run_loader(&repo, &scratch, &stale_base, &self_attested).ok);

    // The exact attested head is the only bootstrap exception.
    repo.git(&["checkout", "-q", "main"]);
    repo.write(
        "docs/process/ci-bootstrap.toml",
        &format!("seed_sha = \"{head}\"\n"),
    );
    repo.commit("docs: attest seed");
    repo.git(&["push", "-q", "origin", "main"]);
    let loaded = run_loader(&repo, &scratch, &stale_base, &head);
    assert!(loaded.ok, "{}", loaded.log);
    assert_eq!(trusted_head(&loaded.output), head);

    // Once main has policy, it is used: the current tip, not the event base.
    repo.write("xtask/Cargo.toml", "[package]\nname = \"xtask\"\n");
    let current = repo.commit("feat: policy on main");
    repo.git(&["push", "-q", "origin", "main"]);
    let loaded = run_loader(&repo, &scratch, &stale_base, &head);
    assert!(loaded.ok, "{}", loaded.log);
    assert_eq!(trusted_head(&loaded.output), current);
    assert!(loaded.output.contains(&format!("main={current}")));

    // Policy later deleted from main: the old attestation no longer matches
    // a new head, so the loader fails closed again.
    repo.git(&["rm", "-q", "-r", "xtask"]);
    repo.commit("chore: remove policy");
    repo.git(&["push", "-q", "origin", "main"]);
    assert!(!run_loader(&repo, &scratch, &stale_base, &self_attested).ok);
}

const REVIEW_DIR: &str = "docs/requirements/reviews";

fn evidence(reviewed: &str) -> Vec<(String, String)> {
    let header = |role: &str| {
        format!(
            "- Author role/provider: code generation / Anthropic\n- Reviewer role/provider: {role}\n- Reviewed head: `{reviewed}`\n"
        )
    };
    vec![
        (
            format!("{REVIEW_DIR}/gpt-review.md"),
            format!(
                "# R\n{}- Verdict: **approve**\n",
                header("independent code review / OpenAI")
            ),
        ),
        (
            format!("{REVIEW_DIR}/gemini-adv.md"),
            format!(
                "# A\n{}- Verdict: must-fix\n\n### ADV1-01: x\n",
                header("adversarial review / Google")
            ),
        ),
        (
            format!("{REVIEW_DIR}/triage.md"),
            format!(
                "# T\n- Triager role/provider: triage / OpenAI\n- Triaged report: `{REVIEW_DIR}/gemini-adv.md`\n- Reviewed head: `{reviewed}`\n\n| ID | Disposition |\n|---|---|\n| ADV1-01 | fixed |\n"
            ),
        ),
    ]
}

fn review_ctx(repo: &Repo) -> Ctx {
    Ctx {
        repo: repo.path().to_path_buf(),
        policy: root(),
    }
}

// F6: a mode-only change after the reviewed commit invalidates the review;
// evidence-only changes and a merge of main do not.
#[test]
fn f6_mode_change_invalidates_review() {
    let repo = Repo::new();
    repo.write("README.md", "main\n");
    repo.commit("chore: main");
    repo.git(&["checkout", "-q", "-b", "pr"]);
    repo.write("run.sh", "echo ok\n");
    let reviewed = repo.commit("feat: script");
    repo.git(&["update-index", "--chmod=+x", "run.sh"]);
    repo.git(&["commit", "-q", "-m", "chore: executable"]);
    repo.git(&["reset", "-q", "--hard"]);
    let mode_changed = repo.git(&["rev-parse", "HEAD"]);
    repo.git(&["checkout", "-q", "main"]);
    for (path, text) in evidence(&reviewed) {
        repo.write(&path, &text);
    }
    let main = repo.commit("docs: review evidence");
    let ctx = review_ctx(&repo);

    // The reviewed commit itself as head: complete evidence passes.
    repo.git(&["branch", "-q", "reviewed", &reviewed]);
    let found = review_gates::independent_review(&ctx, &main, &reviewed).expect("gate");
    assert!(found.is_empty(), "{found:?}");

    // Mode change: same blob, different tree entry. Rejected.
    let found = review_gates::independent_review(&ctx, &main, &mode_changed).expect("gate");
    assert!(!found.is_empty(), "mode change accepted");
    assert!(
        !review_gates::still_current(repo.path(), &main, &reviewed, &mode_changed).expect("git")
    );

    // A merge of main (which carries the evidence) keeps it current.
    repo.git(&["checkout", "-q", "-b", "pr2", &reviewed]);
    repo.git(&["merge", "-q", "--no-ff", "main", "-m", "merge main"]);
    let merged = repo.git(&["rev-parse", "HEAD"]);
    assert!(review_gates::still_current(repo.path(), &main, &reviewed, &merged).expect("git"));
    let found = review_gates::independent_review(&ctx, &main, &merged).expect("gate");
    assert!(found.is_empty(), "{found:?}");
}

// ADV23R-07: same-vendor models do not satisfy the gate end to end.
#[test]
fn adv23r_07_same_vendor_review_is_rejected_end_to_end() {
    let repo = Repo::new();
    repo.write("README.md", "main\n");
    repo.commit("chore: main");
    repo.git(&["checkout", "-q", "-b", "pr"]);
    repo.write("src/lib.rs", "pub fn f() {}\n");
    let head = repo.commit("feat: f");
    repo.git(&["checkout", "-q", "main"]);
    let mut reports = evidence(&head);
    reports[0].1 = format!(
        "# R\n- Author role/provider: Anthropic / Claude-Sonnet\n- Reviewer role/provider: Anthropic / Claude-Opus\n- Reviewed head: {head}\n- Verdict: approved\n"
    );
    for (path, text) in reports {
        repo.write(&path, &text);
    }
    let main = repo.commit("docs: evidence");
    let found = review_gates::independent_review(&review_ctx(&repo), &main, &head).expect("gate");
    assert!(!found.is_empty());
}

// F2 / D-74: a policy change in a PR opened by the owner fails with the reason,
// without needing the GitHub API.
#[test]
fn f2_owner_authored_policy_pr_explains_why_it_cannot_pass() {
    let repo = Repo::new();
    repo.write("README.md", "main\n");
    let main = repo.commit("chore: main");
    repo.git(&["checkout", "-q", "-b", "pr"]);
    repo.write("xtask/src/main.rs", "fn main() {}\n");
    repo.write(
        ".github/workflows/spoof.yml",
        "on: pull_request\njobs:\n  policy-approval:\n    runs-on: ubuntu-24.04\n    steps: [{run: echo ok}]\n",
    );
    let head = repo.commit("feat: policy");
    let pull = PullRequest::from_event(&serde_json::json!({
        "pull_request": {"number": 1, "head": {"sha": head}, "user": {"login": "Tobechukwu-Njoku"}},
        "repository": {"full_name": "o/r"}
    }))
    .expect("event");
    let gates = xtask::workflows::Policy::load(&review_ctx(&repo))
        .expect("gates")
        .protected_gates();
    let found =
        review_gates::policy_approval(&review_ctx(&repo), &main, &pull, &gates).expect("gate");
    let text: Vec<&str> = found.iter().map(|f| f.message.as_str()).collect();
    assert!(
        text.iter()
            .any(|m| m.contains("cannot pass") && m.contains("egs-bot")),
        "{text:?}"
    );
    assert!(
        text.iter().any(|m| m.contains("reuses the check name")),
        "{text:?}"
    );
}

// R3-03: a PR-branch change under the reviews directory is not exempt. Here
// the "evidence" file is compiled production source, included by the crate.
#[test]
fn r3_03_review_directory_change_invalidates_review() {
    let repo = Repo::new();
    repo.write("README.md", "main\n");
    repo.commit("chore: main");
    repo.git(&["checkout", "-q", "-b", "pr"]);
    repo.write(
        "crates/engine/src/lib.rs",
        "#![forbid(unsafe_code)]\ninclude!(\"../../../docs/requirements/reviews/payload.rs\");\n",
    );
    repo.write(
        &format!("{REVIEW_DIR}/payload.rs"),
        "pub fn price() -> u8 { 1 }\n",
    );
    let reviewed = repo.commit("feat: engine");
    repo.write(
        &format!("{REVIEW_DIR}/payload.rs"),
        "pub fn price() -> u8 { 99 }\n",
    );
    let changed = repo.commit("docs: notes");
    repo.git(&["checkout", "-q", "main"]);
    for (path, text) in evidence(&reviewed) {
        repo.write(&path, &text);
    }
    let main = repo.commit("docs: review evidence");
    assert!(!review_gates::still_current(repo.path(), &main, &reviewed, &changed).expect("git"));
    let found =
        review_gates::independent_review(&review_ctx(&repo), &main, &changed).expect("gate");
    assert!(!found.is_empty(), "changed payload accepted");
    // Control: the reviewed commit itself passes.
    let found =
        review_gates::independent_review(&review_ctx(&repo), &main, &reviewed).expect("gate");
    assert!(found.is_empty(), "{found:?}");
}
