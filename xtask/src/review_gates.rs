//! Human-evidence gates (NFR-3.AC3/AC5, ADR 0001), checked automatically.
//!
//! `independent-review` (ADV23-10) passes only when the trusted evidence tree
//! (the fetched `main`, never the PR head) holds:
//! - a review report whose `Reviewed head:` is a commit of this PR, whose
//!   `Verdict:` is approve/approved/merge-ready, and whose author and reviewer
//!   providers differ; and no PR-changed path (other than review reports)
//!   differs between that reviewed commit and the current head; and
//! - an adversarial report (file name contains `adversarial`) that cites a
//!   full commit SHA of this PR.
//!
//! Triage dispositions of adversarial findings remain a coordinator duty; the
//! gate reports that explicitly instead of passing silently.
//!
//! `policy-approval` (R1): when a PR changes a protected policy path, the
//! policy owner's latest review state must be APPROVED on the exact head SHA.

use std::collections::BTreeSet;
use std::path::Path;

use crate::util::{Ctx, Finding, git, glob_match, is_sha, string_array};

const REVIEWS: &str = "docs/requirements/reviews/";

/// Parsed machine-checked review header.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReviewHeader {
    /// Reviewed commit.
    pub head: Option<String>,
    /// Verdict word, lower case.
    pub verdict: Option<String>,
    /// Author provider, lower case.
    pub author: Option<String>,
    /// Reviewer provider, lower case.
    pub reviewer: Option<String>,
}

fn field<'a>(line: &'a str, name: &str) -> Option<&'a str> {
    let cleaned = line.trim_start_matches(['-', '*', ' ', '_']);
    let rest = cleaned.strip_prefix(name)?;
    let rest = rest.trim_start_matches(['*', '_']).strip_prefix(':')?;
    Some(rest.trim().trim_start_matches(['*', '_']).trim())
}

fn provider(value: &str) -> Option<String> {
    let last = value.rsplit('/').next()?.trim();
    let word: String = last
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '-')
        .collect();
    (!word.is_empty()).then(|| word.to_lowercase())
}

/// Parses the header fields of a review report.
#[must_use]
pub fn parse_header(text: &str) -> ReviewHeader {
    let mut header = ReviewHeader::default();
    for line in text.lines().take(40) {
        if let Some(value) = field(line, "Reviewed head") {
            let sha: String = value
                .trim_matches(['`', ' ', '.'])
                .chars()
                .take(40)
                .collect();
            if is_sha(&sha) {
                header.head = Some(sha);
            }
        } else if let Some(value) = field(line, "Verdict") {
            let word: String = value
                .trim_start_matches(['*', '_', '`'])
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '-')
                .collect();
            header.verdict = Some(word.to_lowercase());
        } else if let Some(value) = field(line, "Author role/provider") {
            header.author = provider(value);
        } else if let Some(value) = field(line, "Reviewer role/provider") {
            header.reviewer = provider(value);
        }
    }
    header
}

const APPROVING: &[&str] = &["approve", "approved", "merge-ready"];

/// Evaluates review evidence.
///
/// `reports` are (path, text) from the trusted tree; `commits` are the PR's
/// commits; `unchanged_since(sha)` says whether every PR-changed path is the
/// same at `sha` and at the head.
#[must_use]
pub fn evaluate(
    reports: &[(String, String)],
    commits: &BTreeSet<String>,
    unchanged_since: &dyn Fn(&str) -> bool,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut approved = false;
    let mut notes = Vec::new();
    for (path, text) in reports {
        let header = parse_header(text);
        let Some(head) = &header.head else { continue };
        if !commits.contains(head) {
            continue;
        }
        let verdict_ok = header
            .verdict
            .as_deref()
            .is_some_and(|v| APPROVING.contains(&v));
        let providers_ok = match (&header.author, &header.reviewer) {
            (Some(author), Some(reviewer)) => author != reviewer,
            _ => false,
        };
        let current = unchanged_since(head);
        if verdict_ok && providers_ok && current {
            approved = true;
        } else {
            notes.push(format!(
                "{path}: reviews {} but verdict={:?}, providers differ={providers_ok}, still current={current}",
                &head[..12],
                header.verdict.as_deref().unwrap_or("none")
            ));
        }
    }
    if !approved {
        findings.push(Finding::general(
            "no approving independent review (different provider) covers the current head",
        ));
        findings.extend(notes.into_iter().map(Finding::general));
    }
    let adversarial = reports.iter().any(|(path, text)| {
        path.contains("adversarial")
            && text
                .split(|c: char| !c.is_ascii_hexdigit())
                .any(|word| commits.contains(word))
    });
    if !adversarial {
        findings.push(Finding::general(
            "no adversarial review report cites a commit of this PR",
        ));
    }
    findings
}

/// Runs the independent-review gate against the trusted evidence revision.
///
/// # Errors
///
/// Fails if Git fails or a revision is invalid.
pub fn independent_review(
    repo: &Path,
    evidence: &str,
    base: &str,
    head: &str,
) -> Result<Vec<Finding>, String> {
    for sha in [base, head] {
        if !is_sha(sha) {
            return Err("independent-review: base and head must be full SHAs".to_owned());
        }
    }
    let commits: BTreeSet<String> =
        String::from_utf8_lossy(&git(repo, &["rev-list", &format!("{base}..{head}")])?)
            .split_whitespace()
            .map(str::to_owned)
            .collect();
    let from = String::from_utf8_lossy(&git(repo, &["merge-base", base, head])?)
        .trim()
        .to_owned();
    let changed: Vec<String> = String::from_utf8_lossy(&git(
        repo,
        &["diff", "--name-only", "-z", "--no-renames", &from, head],
    )?)
    .split('\0')
    .filter(|name| !name.is_empty() && !name.starts_with(REVIEWS))
    .map(str::to_owned)
    .collect();
    let listing = git(
        repo,
        &[
            "ls-tree",
            "-r",
            "-z",
            "--name-only",
            evidence,
            "--",
            REVIEWS,
        ],
    )?;
    let mut reports = Vec::new();
    for name in String::from_utf8_lossy(&listing).split('\0') {
        if name.ends_with(".md") {
            let text = git(repo, &["show", &format!("{evidence}:{name}")])?;
            reports.push((name.to_owned(), String::from_utf8_lossy(&text).into_owned()));
        }
    }
    let blob = |rev: &str, path: &str| {
        git(
            repo,
            &["rev-parse", "--verify", "--quiet", &format!("{rev}:{path}")],
        )
        .ok()
        .map(|out| String::from_utf8_lossy(&out).trim().to_owned())
    };
    let unchanged = |sha: &str| {
        changed
            .iter()
            .all(|path| blob(sha, path) == blob(head, path))
    };
    println!(
        "evidence {evidence}: {} report(s); PR has {} commit(s), {} changed path(s)",
        reports.len(),
        commits.len(),
        changed.len()
    );
    let findings = evaluate(&reports, &commits, &unchanged);
    println!(
        "NOTE: adversarial-finding triage dispositions are confirmed by the coordinator (NFR-3.AC5)"
    );
    Ok(findings)
}

/// One pull-request review: (login, state, commit SHA), in submission order.
pub type Review = (String, String, String);

/// Evaluates owner approval for protected-path changes.
#[must_use]
pub fn evaluate_approval(
    changed: &[String],
    protected: &[String],
    owner: &str,
    head: &str,
    reviews: &[Review],
) -> Vec<Finding> {
    let touched: Vec<&String> = changed
        .iter()
        .filter(|path| {
            protected
                .iter()
                .any(|pattern| glob_match(pattern.trim_start_matches('/'), path))
        })
        .collect();
    if touched.is_empty() {
        println!("no protected policy path changed");
        return Vec::new();
    }
    let latest = reviews.iter().rfind(|(login, state, _)| {
        login.eq_ignore_ascii_case(owner) && state != "COMMENTED" && state != "PENDING"
    });
    match latest {
        Some((_, state, commit)) if state == "APPROVED" && commit == head => Vec::new(),
        _ => {
            let mut findings = vec![Finding::general(format!(
                "protected policy paths changed; needs the policy owner's approving review on head {}",
                &head[..head.len().min(12)]
            ))];
            findings.extend(
                touched
                    .into_iter()
                    .map(|path| Finding::at(path.as_str(), None, "protected policy path")),
            );
            findings
        }
    }
}

/// Runs the policy-approval gate for a pull-request event.
///
/// # Errors
///
/// Fails on an invalid event, Git failure or GitHub API failure.
pub fn policy_approval(ctx: &Ctx, event: &serde_json::Value) -> Result<Vec<Finding>, String> {
    let document = ctx.policy_toml("xtask/policy/repository.toml")?;
    let owner = document
        .get("policy_owner")
        .and_then(toml_edit::Item::as_str)
        .ok_or("trusted policy: policy_owner missing")?
        .to_owned();
    let protected = string_array(&document["protected_paths"]);
    let pull = &event["pull_request"];
    let base = pull["base"]["sha"].as_str().unwrap_or_default();
    let head = pull["head"]["sha"].as_str().unwrap_or_default();
    let number = pull["number"].as_u64().ok_or("event: no PR number")?;
    let repository = event["repository"]["full_name"]
        .as_str()
        .unwrap_or_default();
    let valid_repo = repository.split_once('/').is_some_and(|(a, b)| {
        [a, b].iter().all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        })
    });
    if !(is_sha(base) && is_sha(head) && valid_repo) {
        return Err("policy-approval: invalid event fields".to_owned());
    }
    let from = String::from_utf8_lossy(&git(&ctx.repo, &["merge-base", base, head])?)
        .trim()
        .to_owned();
    let changed: Vec<String> = String::from_utf8_lossy(&git(
        &ctx.repo,
        &["diff", "--name-only", "-z", "--no-renames", &from, head],
    )?)
    .split('\0')
    .filter(|name| !name.is_empty())
    .map(str::to_owned)
    .collect();
    let output = std::process::Command::new("gh")
        .args([
            "api",
            "--paginate",
            &format!("repos/{repository}/pulls/{number}/reviews"),
            "--jq",
            ".[] | [.user.login, .state, .commit_id] | @tsv",
        ])
        .output()
        .map_err(|error| format!("gh: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "gh api failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let reviews: Vec<Review> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('\t');
            Some((
                parts.next()?.to_owned(),
                parts.next()?.to_owned(),
                parts.next()?.to_owned(),
            ))
        })
        .collect();
    Ok(evaluate_approval(
        &changed, &protected, &owner, head, &reviews,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sha(c: char) -> String {
        std::iter::repeat_n(c, 40).collect()
    }

    fn report(head: &str, verdict: &str, author: &str, reviewer: &str) -> String {
        format!(
            "# Review\n\n- Author role/provider: code generation / {author}.\n- Reviewer role/provider: independent code review / {reviewer}.\n- Reviewed head: `{head}`.\n- Verdict: **{verdict}**. Notes.\n"
        )
    }

    fn adversarial(head: &str) -> (String, String) {
        (
            "docs/requirements/reviews/gemini-adversarial-x.md".into(),
            format!("Target: head `{head}`"),
        )
    }

    // ADV23-10: the gate is not vacuous.
    #[test]
    fn adv23_10_review_gate_requires_evidence() {
        let commits: BTreeSet<String> = [sha('a'), sha('b')].into_iter().collect();
        let yes = |_: &str| true;
        assert_eq!(evaluate(&[], &commits, &yes).len(), 2);
        let good = vec![
            (
                "docs/requirements/reviews/r.md".into(),
                report(&sha('a'), "approve", "Anthropic", "OpenAI"),
            ),
            adversarial(&sha('b')),
        ];
        assert!(evaluate(&good, &commits, &yes).is_empty());
        let must_fix = vec![
            (
                "r.md".into(),
                report(&sha('a'), "must-fix", "Anthropic", "OpenAI"),
            ),
            adversarial(&sha('a')),
        ];
        assert!(!evaluate(&must_fix, &commits, &yes).is_empty());
        let same_provider = vec![
            (
                "r.md".into(),
                report(&sha('a'), "approve", "OpenAI", "OpenAI"),
            ),
            adversarial(&sha('a')),
        ];
        assert!(!evaluate(&same_provider, &commits, &yes).is_empty());
        let other_pr = vec![
            (
                "r.md".into(),
                report(&sha('c'), "approve", "Anthropic", "OpenAI"),
            ),
            adversarial(&sha('c')),
        ];
        assert_eq!(evaluate(&other_pr, &commits, &yes).len(), 2);
        let stale = |_: &str| false;
        assert!(!evaluate(&good, &commits, &stale).is_empty());
    }

    #[test]
    fn header_parsing_matches_existing_reports() {
        let text = "- Author role/provider: code generation / Anthropic.\n- Reviewer role/provider: independent code review / OpenAI.\n- Reviewed head: `bebd708a574940805854886130cbf8f15907b29b`.\n- Verdict: **must-fix**. R1–R8 below.\n";
        let header = parse_header(text);
        assert_eq!(
            header.head.as_deref(),
            Some("bebd708a574940805854886130cbf8f15907b29b")
        );
        assert_eq!(header.verdict.as_deref(), Some("must-fix"));
        assert_eq!(header.author.as_deref(), Some("anthropic"));
        assert_eq!(header.reviewer.as_deref(), Some("openai"));
    }

    // R1: protected policy changes need the owner's approval on the head.
    #[test]
    fn r1_protected_changes_need_owner_approval() {
        let protected = vec!["deny.toml".to_owned(), "xtask/".to_owned()];
        let head = sha('h');
        let changed = vec!["deny.toml".to_owned()];
        let review = |login: &str, state: &str, commit: &str| {
            (login.to_owned(), state.to_owned(), commit.to_owned())
        };
        assert!(
            evaluate_approval(&["src/x.rs".into()], &protected, "Owner", &head, &[]).is_empty()
        );
        assert!(!evaluate_approval(&changed, &protected, "Owner", &head, &[]).is_empty());
        assert!(
            evaluate_approval(
                &changed,
                &protected,
                "Owner",
                &head,
                &[review("owner", "APPROVED", &head)]
            )
            .is_empty()
        );
        assert!(
            !evaluate_approval(
                &changed,
                &protected,
                "Owner",
                &head,
                &[review("owner", "APPROVED", &sha('o'))]
            )
            .is_empty()
        );
        assert!(
            !evaluate_approval(
                &changed,
                &protected,
                "Owner",
                &head,
                &[review("someone", "APPROVED", &head)]
            )
            .is_empty()
        );
        let dismissed = [
            review("owner", "APPROVED", &head),
            review("owner", "CHANGES_REQUESTED", &head),
            review("owner", "COMMENTED", &head),
        ];
        assert!(!evaluate_approval(&changed, &protected, "Owner", &head, &dismissed).is_empty());
        assert!(
            !evaluate_approval(
                &["xtask/src/main.rs".into()],
                &protected,
                "Owner",
                &head,
                &[]
            )
            .is_empty()
        );
    }
}
