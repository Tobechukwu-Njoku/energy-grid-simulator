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

pub(crate) const POLICY_PATTERNS: &[&str] = &[
    "/.github/",
    "/.githooks/",
    "/.claude/",
    "/xtask/",
    "/docs/process/",
    "/docs/security/",
    "/AGENTS.md",
    "/CLAUDE.md",
    "/CONTRIBUTING.md",
    "/SECURITY.md",
    "/deny.toml",
    "/docs/architecture/adr/0001-repo-layout-and-ci-gates.md",
    "Cargo.toml",
    "Cargo.lock",
    "manifest.toml",
    "sources.tsv",
    "clippy.toml",
    ".clippy.toml",
    "rustfmt.toml",
    ".rustfmt.toml",
    "rust-toolchain",
    "rust-toolchain.toml",
    "build.rs",
    "security/",
    "security.rs",
    "provenance/",
    "provenance.rs",
    "data-rules/",
    "data-rules.md",
    "data_rules.rs",
    "migrations/",
    "schema-migrations/",
    ".cargo/",
    ".config/",
];

pub(crate) fn is_policy_path(path: &str) -> bool {
    POLICY_PATTERNS.iter().any(|pattern| {
        if let Some(anchored) = pattern.strip_prefix('/') {
            if anchored.ends_with('/') {
                path.starts_with(anchored)
            } else {
                path == anchored
            }
        } else if let Some(directory) = pattern.strip_suffix('/') {
            path.rsplit_once('/')
                .is_some_and(|(parent, _)| parent.split('/').any(|part| part == directory))
        } else {
            path.rsplit('/').next() == Some(*pattern)
        }
    })
}

fn tier(path: &str) -> Tier {
    if is_policy_path(path) {
        Tier::Policy
    } else if (!path.contains('/') && path.ends_with(".md"))
        || (["docs/", "notes/"]
            .iter()
            .any(|prefix| path.starts_with(prefix))
            && [".md", ".txt"].iter().any(|suffix| path.ends_with(suffix)))
    {
        Tier::Docs
    } else {
        Tier::Product
    }
}

fn classify<'a>(paths: impl IntoIterator<Item = &'a str>) -> Tier {
    paths.into_iter().map(tier).max().unwrap_or(Tier::Docs)
}

pub(crate) fn in_rust_tree(path: &str) -> bool {
    ["crates/", "spikes/", "xtask/"]
        .iter()
        .any(|prefix| path.starts_with(prefix))
}

/// Whether the file is a test target, reading Rust contents for inline tests.
fn is_test_file(path: &str, read: impl FnOnce() -> Result<String, String>) -> Result<bool, String> {
    if !in_rust_tree(path) && !path.starts_with("tests/") {
        return Ok(false);
    }
    let (directory, file) = path.rsplit_once('/').unwrap_or(("", path));
    if file.ends_with(".py") {
        return Ok(file.starts_with("test_"));
    }
    if !file.ends_with(".rs") {
        return Ok(false);
    }
    if directory.split('/').any(|part| part == "tests") {
        return Ok(true);
    }

    use syn::visit::Visit as _;
    #[derive(Default)]
    struct TestAttribute(bool);
    impl<'ast> syn::visit::Visit<'ast> for TestAttribute {
        fn visit_attribute(&mut self, attr: &'ast syn::Attribute) {
            self.0 |= attr.path().is_ident("test")
                || (attr.path().is_ident("cfg")
                    && attr
                        .parse_args::<syn::Ident>()
                        .is_ok_and(|arg| arg == "test"));
        }
    }
    let mut found = TestAttribute::default();
    if let Ok(source) = syn::parse_file(&read()?) {
        found.visit_file(&source);
    }
    Ok(found.0)
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

fn has_test_change(
    listing: &str,
    mut read: impl FnMut(&str) -> Result<String, String>,
) -> Result<bool, String> {
    for path in paths(listing) {
        if is_test_file(path, || read(path))? {
            return Ok(true);
        }
    }
    Ok(false)
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
                "--name-only",
                "-z",
                "--diff-filter=AM",
                "--no-renames",
                "--no-ext-diff",
                main,
                merged,
                "--",
            ],
        )?;
        if !has_test_change(&String::from_utf8_lossy(&test_candidates), |path| {
            let body = git(&ctx.repo, &["show", &format!("{merged}:{path}")])?;
            Ok(String::from_utf8_lossy(&body).into_owned())
        })? {
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
            "README.md",
        ] {
            assert_eq!(classify([path]), Tier::Docs, "{path}");
        }
        for path in [
            "crates/network/src/lib.rs",
            "crates/domain/README.md",
            "spikes/data/REPORT.md",
            "spikes/example/model.py",
            "tests/acceptance/case.toml",
            "tests/README.md",
            "docs/model.json",
            "notes/sample.rs",
            "STATUS",
            "archive/reviews/old-review.md",
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
    fn conventional_feat_and_fix_titles() {
        for title in [
            "feat: network",
            "fix(api): dispatch",
            "feat!: breaking",
            "fix(io)!: breaking",
            "  Feat: padded and capitalised",
            "FIX(Grid)!: upper case",
        ] {
            assert!(is_feat_or_fix(title), "{title}");
        }
        for title in [
            "docs: fix links",
            "feature: text",
            "fixing: text",
            "fix the grid",
            "feat(): text",
        ] {
            assert!(!is_feat_or_fix(title), "{title}");
        }
    }

    #[test]
    fn only_real_test_targets_in_supported_directories_count() {
        for (path, source) in [
            ("crates/domain/tests/grid.rs", ""),
            ("tests/grid.rs", ""),
            ("spikes/example/test_grid.py", ""),
            ("xtask/src/lib.rs", "#[cfg(test)] mod tests {}"),
            ("crates/domain/src/grid.rs", "#[test] fn grid() {}"),
        ] {
            assert!(has_test_change(path, |_| Ok(source.to_owned())).expect("test target"));
        }
        for (path, source) in [
            ("docs/tests/grid.rs", "#[test] fn grid() {}"),
            ("tools/test_grid.py", ""),
            ("crates/domain/src/tests.rs", "// #[test]"),
            (
                "crates/domain/src/grid.rs",
                r##"const TEXT: &str = "#[cfg(test)]";"##,
            ),
            ("spikes/example/tests/grid.py", ""),
            ("tests/README.md", ""),
            ("tests/grid.js", ""),
        ] {
            assert!(!has_test_change(path, |_| Ok(source.to_owned())).expect("non-test target"));
        }
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
