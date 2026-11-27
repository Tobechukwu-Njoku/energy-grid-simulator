//! Classify pull-request changes and check GitHub reviews for the current head.

use std::process::Command;

use serde_json::Value;
use toml_edit::Item;

use crate::util::{Ctx, Finding, git, is_sha};
use crate::workflows::{Policy, spoofed_gate_names};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Tier {
    Docs,
    Product,
    Policy,
}

fn tier(path: &str) -> Tier {
    let components: Vec<_> = path.split('/').collect();
    let file = components.last().copied().unwrap_or_default();
    if [
        ".github/",
        ".githooks/",
        ".claude/",
        "xtask/",
        "docs/process/",
        "docs/security/",
    ]
    .iter()
    .any(|prefix| path.starts_with(prefix))
        || matches!(
            path,
            "AGENTS.md"
                | "CLAUDE.md"
                | "CONTRIBUTING.md"
                | "SECURITY.md"
                | "deny.toml"
                | "docs/architecture/adr/0001-repo-layout-and-ci-gates.md"
        )
        || matches!(
            file,
            "Cargo.toml"
                | "Cargo.lock"
                | "manifest.toml"
                | "sources.tsv"
                | "clippy.toml"
                | ".clippy.toml"
                | "rustfmt.toml"
                | ".rustfmt.toml"
                | "rust-toolchain"
                | "rust-toolchain.toml"
                | "build.rs"
        )
        || components.iter().any(|part| {
            matches!(
                *part,
                "security"
                    | "security.rs"
                    | "provenance"
                    | "provenance.rs"
                    | "data-rules"
                    | "data-rules.md"
                    | "data_rules.rs"
                    | "migrations"
                    | "schema-migrations"
                    | ".cargo"
                    | ".config"
            )
        })
    {
        Tier::Policy
    } else if path.starts_with("crates/") || path.starts_with("spikes/") {
        Tier::Product
    } else if path.starts_with("docs/")
        || path.starts_with("notes/")
        || file == "STATUS"
        || file.ends_with(".md")
    {
        Tier::Docs
    } else {
        Tier::Product
    }
}

fn classify<'a>(paths: impl IntoIterator<Item = &'a str>) -> Tier {
    paths.into_iter().map(tier).max().unwrap_or(Tier::Docs)
}

/// A source file (.rs .py .ts .js .go) under a `tests`, `test` or
/// `__tests__` directory, or named `*_test.*` or `test_*.*`.
fn is_test_file(path: &str) -> bool {
    let (directory, file) = path.rsplit_once('/').unwrap_or(("", path));
    let Some((stem, extension)) = file.rsplit_once('.') else {
        return false;
    };
    matches!(extension, "rs" | "py" | "ts" | "js" | "go")
        && (directory
            .split('/')
            .any(|part| matches!(part, "tests" | "test" | "__tests__"))
            || stem.ends_with("_test")
            || stem.starts_with("test_"))
}

/// A conventional `feat` or `fix` title, with an optional scope and `!`,
/// compared after trimming and lowercasing.
fn is_feat_or_fix(title: &str) -> bool {
    let title = title.trim().to_ascii_lowercase();
    let Some((prefix, _)) = title.split_once(':') else {
        return false;
    };
    let prefix = prefix.trim_end();
    let prefix = prefix.strip_suffix('!').unwrap_or(prefix);
    let kind = match prefix.split_once('(') {
        Some((kind, scope)) if scope.ends_with(')') && scope.len() > 1 => kind,
        _ => prefix,
    };
    matches!(kind, "feat" | "fix")
}

/// Pull-request fields shared with the attribution check.
pub struct PullRequest {
    /// GitHub repository in owner/name form.
    pub repository: String,
    /// Pull request number within the repository.
    pub number: u64,
    head: String,
    author: String,
}

impl PullRequest {
    /// Read GitHub's event metadata.
    ///
    /// # Errors
    ///
    /// Fails when a required field is missing or malformed.
    pub fn from_event(event: &Value) -> Result<Self, String> {
        let pull = &event["pull_request"];
        let head = pull["head"]["sha"].as_str().unwrap_or_default();
        let number = pull["number"].as_u64().ok_or("event: no PR number")?;
        let author = pull["user"]["login"].as_str().unwrap_or_default();
        let repository = event["repository"]["full_name"]
            .as_str()
            .unwrap_or_default();
        let name_ok = |part: &str| {
            !part.is_empty()
                && part
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'-' | b'_' | b'.'))
        };
        if !is_sha(head)
            || !name_ok(author)
            || !repository
                .split_once('/')
                .is_some_and(|(owner, name)| name_ok(owner) && name_ok(name))
        {
            return Err("invalid pull request event fields".to_owned());
        }
        Ok(Self {
            repository: repository.to_owned(),
            number,
            head: head.to_owned(),
            author: author.to_owned(),
        })
    }

    fn endpoint(&self) -> String {
        format!("repos/{}/pulls/{}", self.repository, self.number)
    }
}

fn gh(args: &[&str]) -> Result<Value, String> {
    let output = Command::new("gh")
        .arg("api")
        .args(args)
        .output()
        .map_err(|error| format!("gh: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "gh api failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|error| format!("gh api response: {error}"))
}

/// Whether `approver` (from trusted policy) approved the current head and
/// has not since requested changes or had the approval dismissed. Only that
/// one account counts: other people, bots and the PR author never do.
fn approval_findings(pull: &PullRequest, approver: &str, reviews: &[&Value]) -> Vec<Finding> {
    if pull.author.eq_ignore_ascii_case(approver) {
        return vec![Finding::general(format!(
            "{approver} authored this PR and cannot approve it; open it from another account"
        ))];
    }
    // GitHub lists reviews in chronological order. A comment or pending draft
    // does not supersede a decision; a later decision on any SHA does.
    let mut approved = false;
    for review in reviews {
        let (Some(login), Some(state)) =
            (review["user"]["login"].as_str(), review["state"].as_str())
        else {
            continue;
        };
        if login.eq_ignore_ascii_case(approver) && !matches!(state, "COMMENTED" | "PENDING") {
            approved =
                state == "APPROVED" && review["commit_id"].as_str() == Some(pull.head.as_str());
        }
    }
    if approved {
        Vec::new()
    } else {
        vec![Finding::general(format!(
            "needs an APPROVED review from {approver} on the current head"
        ))]
    }
}

fn paths(listing: &str) -> impl Iterator<Item = &str> {
    listing.split_terminator('\0')
}

/// `changes` are the added or modified paths with their added-line counts.
fn missing_test<'a>(title: &str, changes: impl IntoIterator<Item = (&'a str, u64)>) -> bool {
    is_feat_or_fix(title)
        && !changes
            .into_iter()
            .any(|(path, added)| added > 0 && is_test_file(path))
}

/// Parse `git diff --numstat -z --no-renames` output; binary files count 0.
fn numstat(listing: &str) -> Vec<(&str, u64)> {
    listing
        .split_terminator('\0')
        .filter_map(|entry| {
            let mut fields = entry.splitn(3, '\t');
            let added = fields.next()?.parse().unwrap_or(0);
            let _deleted = fields.next()?;
            Some((fields.next()?, added))
        })
        .collect()
}

/// Check the current PR's net changes and reviews using policy from main.
///
/// # Errors
///
/// Fails if Git or GitHub cannot provide the current PR data or the trusted
/// policy cannot be loaded.
pub fn check(ctx: &Ctx, main: &str, pull: &PullRequest) -> Result<Vec<Finding>, String> {
    let current = gh(&[&pull.endpoint()])?;
    if current["head"]["sha"].as_str() != Some(&pull.head)
        || current["base"]["ref"].as_str() != Some("main")
        || current["state"].as_str() != Some("open")
    {
        return Err("PR is no longer open on this head against main".to_owned());
    }
    let title = current["title"]
        .as_str()
        .ok_or("GitHub response: no PR title")?;
    let merged = git(&ctx.repo, &["merge-tree", "--write-tree", main, &pull.head])?;
    let merged = String::from_utf8_lossy(&merged);
    let merged = merged.trim();
    if !is_sha(merged) {
        return Err("git merge-tree did not return a merge result tree".to_owned());
    }
    let changes = git(
        &ctx.repo,
        &[
            "diff",
            "--name-only",
            "-z",
            "--no-renames",
            "--no-ext-diff",
            main,
            merged,
            "--",
        ],
    )?;
    let changes = String::from_utf8_lossy(&changes);
    let tier = classify(paths(&changes));
    println!("Review tier {}", tier as u8);

    let policy = Policy::load(ctx)?;
    let listing = git(
        &ctx.repo,
        &[
            "ls-tree",
            "-r",
            "-z",
            "--name-only",
            merged,
            "--",
            ".github/workflows/",
        ],
    )?;
    let mut workflows = Vec::new();
    for name in paths(&String::from_utf8_lossy(&listing)) {
        if name.ends_with(".yml") || name.ends_with(".yaml") {
            let body = git(&ctx.repo, &["show", &format!("{merged}:{name}")])?;
            workflows.push((name.to_owned(), String::from_utf8_lossy(&body).into_owned()));
        }
    }
    let mut findings = spoofed_gate_names(&workflows, &policy.protected_gates());

    if is_feat_or_fix(title) {
        let test_candidates = git(
            &ctx.repo,
            &[
                "diff",
                "--numstat",
                "-z",
                "--diff-filter=AM",
                "--no-renames",
                "--no-ext-diff",
                main,
                merged,
                "--",
            ],
        )?;
        if missing_test(title, numstat(&String::from_utf8_lossy(&test_candidates))) {
            findings.push(Finding::general(
                "feat/fix PRs must add or modify a test file",
            ));
        }
    }
    if tier != Tier::Docs {
        let document = ctx.policy_toml("xtask/policy/repository.toml")?;
        let approver = document
            .get("review_approver")
            .and_then(Item::as_str)
            .ok_or("trusted policy: review_approver missing")?;
        let pages = gh(&[
            "--paginate",
            "--slurp",
            &format!("{}/reviews", pull.endpoint()),
        ])?;
        let mut reviews = Vec::new();
        for page in pages.as_array().ok_or("GitHub response: no pages")? {
            reviews.extend(page.as_array().ok_or("GitHub response: no reviews")?.iter());
        }
        findings.extend(approval_findings(pull, approver, &reviews));
    }
    Ok(findings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn highest_changed_tier_wins() {
        assert_eq!(classify([]), Tier::Docs);
        for path in [
            "docs/requirements/STATUS.md",
            "notes/draft.txt",
            "STATUS",
            "README.md",
            "archive/reviews/old-review.md",
        ] {
            assert_eq!(classify([path]), Tier::Docs, "{path}");
        }
        for path in [
            "crates/network/src/lib.rs",
            "crates/domain/README.md",
            "spikes/data/REPORT.md",
            "spikes/example/model.py",
            "tests/acceptance/case.toml",
            "apps/web/src/main.ts",
        ] {
            assert_eq!(classify(["docs/roadmap.md", path]), Tier::Product, "{path}");
        }
        for path in [
            ".github/workflows/ci.yml",
            "xtask/src/main.rs",
            "xtask/README.md",
            ".githooks/commit-msg",
            "deny.toml",
            "crates/domain/Cargo.toml",
            "Cargo.lock",
            "crates/api/src/security.rs",
            "spikes/data/sources.tsv",
            "spikes/data/manifest.toml",
            "docs/data-rules.md",
            "crates/api/migrations/0001.sql",
            "docs/process/team.md",
            "docs/security/ci-runner-policy.md",
            "AGENTS.md",
            ".config/nextest.toml",
            "crates/domain/.config/nextest.toml",
            "clippy.toml",
            "crates/domain/.clippy.toml",
            "rustfmt.toml",
            "crates/domain/.rustfmt.toml",
            "rust-toolchain",
            "crates/domain/rust-toolchain.toml",
            "crates/domain/build.rs",
            ".cargo/config.toml",
            "crates/domain/.cargo/config.toml",
        ] {
            assert_eq!(
                classify(["docs/roadmap.md", path, "crates/network/src/lib.rs"]),
                Tier::Policy,
                "{path}"
            );
        }
    }

    #[test]
    fn conventional_feat_and_fix_need_an_added_test_line() {
        let none: [(&str, u64); 0] = [];
        for title in [
            "feat: network",
            "fix(api): dispatch",
            "feat!: breaking",
            "fix(io)!: breaking",
            "  Feat: padded and capitalised",
            "FIX(Grid)!: upper case",
        ] {
            assert!(missing_test(title, none), "{title}");
            for not_test in [
                "crates/domain/src/lib.rs",
                "tests/README.md",
                "tests/cases/grid.toml",
                "crates/domain/src/tests.rs",
                "apps/web/src/grid.spec.ts",
                "crates/domain/src/contest_x.rs",
            ] {
                assert!(missing_test(title, [(not_test, 5)]), "{title} {not_test}");
            }
            for test in [
                "crates/domain/tests/grid.rs",
                "spikes/example/test/grid.py",
                "apps/web/__tests__/grid.ts",
                "apps/web/src/grid_test.js",
                "tools/check/grid_test.go",
                "spikes/example/test_grid.py",
            ] {
                assert!(!missing_test(title, [(test, 1)]), "{title} {test}");
                // Only deletions, or a binary change, do not count.
                assert!(missing_test(title, [(test, 0)]), "{title} {test}");
            }
        }
        for title in [
            "docs: fix links",
            "feature: text",
            "fixing: text",
            "fix the grid",
            "feat(): text",
        ] {
            assert!(!missing_test(title, none), "{title}");
        }
    }

    #[test]
    fn numstat_reads_added_lines() {
        assert_eq!(
            numstat("3\t1\tcrates/a/tests/x.rs\0-\t-\tlogo.png\0"),
            [("crates/a/tests/x.rs", 3), ("logo.png", 0)]
        );
    }

    #[test]
    fn only_the_policy_approver_on_the_current_head_counts() {
        let pull = PullRequest {
            repository: "example/project".to_owned(),
            number: 1,
            head: "current".to_owned(),
            author: "automated-machines".to_owned(),
        };
        let approver = "Tobechukwu-Njoku";
        let review = |login: &str, state: &str, commit: &str| json!({"user": {"login": login}, "state": state, "commit_id": commit});
        let passes = |reviews: &[&Value]| approval_findings(&pull, approver, reviews).is_empty();
        let owner = review("tobechukwu-njoku", "APPROVED", "current");

        assert!(!passes(&[]));
        assert!(!passes(&[&review("outsider", "APPROVED", "current")]));
        assert!(!passes(&[&review(
            "dependabot[bot]",
            "APPROVED",
            "current"
        )]));
        assert!(!passes(&[&review(
            "automated-machines",
            "APPROVED",
            "current"
        )]));
        assert!(!passes(&[&review("Tobechukwu-Njoku", "APPROVED", "old")]));
        assert!(passes(&[&owner]));
        assert!(passes(&[
            &review("Tobechukwu-Njoku", "APPROVED", "old"),
            &owner,
            &review("Tobechukwu-Njoku", "COMMENTED", "current"),
            &review("outsider", "CHANGES_REQUESTED", "current"),
        ]));
        for state in ["CHANGES_REQUESTED", "DISMISSED"] {
            assert!(!passes(&[
                &owner,
                &review("Tobechukwu-Njoku", state, "current")
            ]));
        }
        // GitHub marks a dismissed approval itself as DISMISSED.
        assert!(!passes(&[&review(
            "Tobechukwu-Njoku",
            "DISMISSED",
            "current"
        )]));
        // The approver cannot approve their own PR.
        let own = PullRequest {
            author: "Tobechukwu-Njoku".to_owned(),
            ..pull
        };
        assert!(!approval_findings(&own, approver, &[&owner]).is_empty());
    }
}
