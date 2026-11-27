//! Trusted-policy integration tests over scratch repositories (F1,
//! ADV23R-08).

#![forbid(unsafe_code)]

mod common;

use std::process::Command;

use common::{Repo, TempDir, root};

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
