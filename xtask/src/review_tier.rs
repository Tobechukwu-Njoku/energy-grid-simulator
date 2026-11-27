//! Classify pull-request changes and check GitHub reviews for the current head.

use std::collections::BTreeMap;
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
        ".cargo/",
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
                | "rust-toolchain.toml"
                | "docs/architecture/adr/0001-repo-layout-and-ci-gates.md"
        )
        || matches!(
            file,
            "Cargo.toml" | "Cargo.lock" | "manifest.toml" | "sources.tsv"
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
            )
        })
    {
        Tier::Policy
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

fn is_test_file(path: &str) -> bool {
    let (directory, file) = path.rsplit_once('/').unwrap_or(("", path));
    if file.ends_with(".md") {
        return false;
    }
    directory
        .split('/')
        .any(|part| matches!(part, "tests" | "test" | "__tests__"))
        || matches!(file, "tests.rs" | "test.rs")
        || file
            .strip_suffix(".rs")
            .is_some_and(|stem| stem.ends_with("_test") || stem.ends_with("_tests"))
        || file
            .rsplit_once('.')
            .is_some_and(|(stem, _)| stem.ends_with(".test") || stem.ends_with(".spec"))
}

fn is_feat_or_fix(title: &str) -> bool {
    let Some((prefix, _)) = title.split_once(':') else {
        return false;
    };
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

fn approval_findings(
    tier: Tier,
    pull: &PullRequest,
    owner: &str,
    reviews: &[&Value],
) -> Vec<Finding> {
    // GitHub lists reviews in chronological order. A comment or draft does
    // not supersede a decision. A later decision on another SHA does.
    let mut latest = BTreeMap::new();
    for review in reviews {
        let (Some(login), Some(state)) =
            (review["user"]["login"].as_str(), review["state"].as_str())
        else {
            continue;
        };
        if !login.eq_ignore_ascii_case(&pull.author) && !matches!(state, "COMMENTED" | "PENDING") {
            latest.insert(
                login.to_ascii_lowercase(),
                state == "APPROVED" && review["commit_id"].as_str() == Some(pull.head.as_str()),
            );
        }
    }
    let owner_key = owner.to_ascii_lowercase();
    let mut findings = Vec::new();
    if !latest
        .iter()
        .any(|(login, &approved)| approved && (tier != Tier::Policy || *login != owner_key))
    {
        findings.push(Finding::general(
            "needs a non-author reviewer approval on the current head",
        ));
    }
    if tier == Tier::Policy && latest.get(&owner_key) != Some(&true) {
        findings.push(Finding::general(format!(
            "needs approval from {owner} on the current head"
        )));
    }
    findings
}

fn paths(listing: &str) -> impl Iterator<Item = &str> {
    listing.split_terminator('\0')
}

fn missing_test(title: &str, changed_or_added: impl IntoIterator<Item = impl AsRef<str>>) -> bool {
    is_feat_or_fix(title)
        && !changed_or_added
            .into_iter()
            .any(|path| is_test_file(path.as_ref()))
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
                "--diff-filter=AMT",
                "--no-renames",
                "--no-ext-diff",
                main,
                merged,
                "--",
            ],
        )?;
        if missing_test(title, paths(&String::from_utf8_lossy(&test_candidates))) {
            findings.push(Finding::general(
                "feat/fix PRs must add or modify a test file",
            ));
        }
    }
    if tier != Tier::Docs {
        let document = ctx.policy_toml("xtask/policy/repository.toml")?;
        let owner = document
            .get("policy_owner")
            .and_then(Item::as_str)
            .ok_or("trusted policy: policy_owner missing")?;
        let pages = gh(&[
            "--paginate",
            "--slurp",
            &format!("{}/reviews", pull.endpoint()),
        ])?;
        let mut reviews = Vec::new();
        for page in pages.as_array().ok_or("GitHub response: no pages")? {
            reviews.extend(page.as_array().ok_or("GitHub response: no reviews")?.iter());
        }
        findings.extend(approval_findings(tier, pull, owner, &reviews));
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
            "crates/domain/README.md",
            "archive/reviews/old-review.md",
        ] {
            assert_eq!(classify([path]), Tier::Docs, "{path}");
        }
        for path in [
            "crates/network/src/lib.rs",
            "spikes/example/model.py",
            "tests/acceptance/case.toml",
            "apps/web/src/main.ts",
        ] {
            assert_eq!(classify(["docs/roadmap.md", path]), Tier::Product, "{path}");
        }
        for path in [
            ".github/workflows/ci.yml",
            "xtask/src/main.rs",
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
        ] {
            assert_eq!(
                classify(["docs/roadmap.md", path, "crates/network/src/lib.rs"]),
                Tier::Policy,
                "{path}"
            );
        }
    }

    #[test]
    fn conventional_feat_and_fix_need_a_test_file() {
        let no_paths: [&str; 0] = [];
        for title in [
            "feat: network",
            "fix(api): dispatch",
            "feat!: breaking",
            "fix(io)!: breaking",
        ] {
            assert!(missing_test(title, no_paths));
            assert!(missing_test(
                title,
                ["crates/domain/src/lib.rs", "tests/README.md"]
            ));
            for test in [
                "crates/domain/tests/grid.rs",
                "crates/domain/src/tests.rs",
                "tests/cases/grid.toml",
                "apps/web/src/grid.spec.ts",
                "spikes/example/test/test_grid.py",
            ] {
                assert!(!missing_test(title, [test]), "{title} {test}");
            }
        }
        for title in [
            "docs: fix links",
            "feature: text",
            "fixing: text",
            "fix the grid",
            "feat(): text",
        ] {
            assert!(!missing_test(title, no_paths));
        }
    }

    #[test]
    fn approval_must_be_current_non_author_and_separate_from_owner_on_tier_two() {
        let pull = PullRequest {
            repository: "example/project".to_owned(),
            number: 1,
            head: "current".to_owned(),
            author: "automated-machines".to_owned(),
        };
        let owner = "Tobechukwu-Njoku";
        let review = |login: &str, state: &str, commit: &str| json!({"user": {"login": login}, "state": state, "commit_id": commit});
        let reviewer = |state, commit| review("reviewer", state, commit);
        let approved = reviewer("APPROVED", "current");
        let owner_approved = review("tobechukwu-njoku", "APPROVED", "current");

        assert_eq!(
            approval_findings(
                Tier::Product,
                &pull,
                owner,
                &[
                    &review("AUTOMATED-MACHINES", "APPROVED", "current"),
                    &reviewer("APPROVED", "old"),
                ]
            )
            .len(),
            1
        );
        assert!(
            approval_findings(
                Tier::Product,
                &pull,
                owner,
                &[&approved, &reviewer("COMMENTED", "current")]
            )
            .is_empty()
        );
        for state in ["CHANGES_REQUESTED", "DISMISSED"] {
            assert_eq!(
                approval_findings(
                    Tier::Product,
                    &pull,
                    owner,
                    &[&approved, &reviewer(state, "current")]
                )
                .len(),
                1
            );
        }
        assert_eq!(
            approval_findings(Tier::Policy, &pull, owner, &[&owner_approved]).len(),
            1
        );
        assert_eq!(
            approval_findings(Tier::Policy, &pull, owner, &[&approved]).len(),
            1
        );
        assert!(
            approval_findings(Tier::Policy, &pull, owner, &[&approved, &owner_approved]).is_empty()
        );
    }
}
