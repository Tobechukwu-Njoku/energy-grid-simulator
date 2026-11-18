//! Documentation gates (ADR 0001 profile B0): links and traceability.
//!
//! - `links`: every relative inline link, image and reference-style link
//!   definition points at a tracked path, and every `[text][label]` reference
//!   has a definition (C1, ADV23-18).
//! - `decisions`: decision IDs in the log are unique and contiguous.
//! - `references`: every `D-<n>` cited in any tracked Markdown file outside
//!   review reports exists (ADV23-18).
//! - `requirements`: every FR-/NFR- section is unique and cites a decision.
//! - `allocation`: every requirement is allocated to a milestone, and every
//!   allocated ID exists (ADV23-18).
//! - `adrs`: ADR files are `NNNN-title.md` with a matching title and status.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use fancy_regex::Regex;

use crate::util::{Finding, tracked};

const REQUIREMENTS: &str = "docs/requirements/04-requirements.md";
const DECISIONS: &str = "docs/requirements/03-decisions-log.md";
const ADR_DIR: &str = "docs/architecture/adr/";
const REVIEWS: &str = "docs/requirements/reviews/";

fn regex(pattern: &str) -> Regex {
    Regex::new(pattern).unwrap_or_else(|error| panic!("built-in regex {pattern}: {error}"))
}

/// Yields (line number, text) outside fenced code, with code spans removed.
/// A code span may continue onto following lines of the same paragraph.
#[must_use]
pub fn prose_lines(text: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut fence: Option<&str> = None;
    let mut in_span = false;
    for (index, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        let marker = if trimmed.starts_with("```") {
            Some("```")
        } else if trimmed.starts_with("~~~") {
            Some("~~~")
        } else {
            None
        };
        if let Some(marker) = marker {
            match fence {
                None => fence = Some(marker),
                Some(open) if open == marker => fence = None,
                Some(_) => {}
            }
            continue;
        }
        if fence.is_some() {
            continue;
        }
        if line.trim().is_empty() {
            in_span = false;
        }
        let mut kept = String::new();
        for (part_index, part) in line.split('`').enumerate() {
            if part_index > 0 {
                in_span = !in_span;
            }
            if !in_span {
                kept.push_str(part);
            }
        }
        out.push((index + 1, kept));
    }
    out
}

fn normalise_label(label: &str) -> String {
    label
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn is_external(target: &str) -> bool {
    target.starts_with('#')
        || target.split_once(':').is_some_and(|(scheme, _)| {
            !scheme.is_empty()
                && scheme
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '.' | '-'))
        })
}

fn resolves(root: &Path, file: &str, target: &str, known: &BTreeSet<String>) -> bool {
    let relative = target.split('#').next().unwrap_or_default();
    let relative = relative.split('?').next().unwrap_or_default();
    if relative.is_empty() {
        return true;
    }
    let decoded = relative.replace("%20", " ");
    let joined = if let Some(absolute) = decoded.strip_prefix('/') {
        absolute.to_owned()
    } else {
        let parent = Path::new(file).parent().unwrap_or(Path::new(""));
        parent.join(&decoded).to_string_lossy().into_owned()
    };
    let mut parts: Vec<&str> = Vec::new();
    for part in joined.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                if parts.pop().is_none() {
                    return root.join(&joined).exists();
                }
            }
            other => parts.push(other),
        }
    }
    let path = parts.join("/");
    known.contains(&path)
        || known
            .iter()
            .any(|tracked| tracked.starts_with(&format!("{path}/")))
}

/// Checks links in one Markdown file against the tracked path set.
#[must_use]
pub fn check_links_in(
    root: &Path,
    file: &str,
    text: &str,
    known: &BTreeSet<String>,
) -> Vec<Finding> {
    let inline = regex(r#"\]\(\s*<?([^)\s>]+)>?(?:\s+"[^"]*")?\s*\)"#);
    let definition = regex(r"^\s{0,3}\[([^\]]+)\]:\s*<?([^\s>]+)>?");
    let reference = regex(r"(?<!\\)\[([^\]]+)\]\[([^\]]*)\]");
    let mut findings = Vec::new();
    let lines = prose_lines(text);
    let mut defined = BTreeMap::new();
    for (number, line) in &lines {
        if let Ok(Some(captures)) = definition.captures(line.as_str()) {
            let label = normalise_label(&captures[1]);
            let target = captures[2].to_owned();
            if !is_external(&target) && !resolves(root, file, &target, known) {
                findings.push(Finding::at(
                    file,
                    Some(*number),
                    format!("broken link definition [{label}]: {target}"),
                ));
            }
            defined.insert(label, target);
        }
    }
    for (number, line) in &lines {
        for captures in inline.captures_iter(line.as_str()).flatten() {
            let target = &captures[1];
            if !is_external(target) && !resolves(root, file, target, known) {
                findings.push(Finding::at(
                    file,
                    Some(*number),
                    format!("broken link to {target}"),
                ));
            }
        }
        for captures in reference.captures_iter(line.as_str()).flatten() {
            let label = if captures[2].is_empty() {
                &captures[1]
            } else {
                &captures[2]
            };
            if !defined.contains_key(&normalise_label(label)) {
                findings.push(Finding::at(
                    file,
                    Some(*number),
                    format!("undefined link reference [{label}]"),
                ));
            }
        }
    }
    findings
}

fn decision_ids(text: &str) -> Vec<u32> {
    let row = regex(r"^\|\s*D-(\d+)\s*\|");
    text.lines()
        .filter_map(|line| row.captures(line).ok().flatten())
        .filter_map(|captures| captures[1].parse().ok())
        .collect()
}

fn check_decisions(ids: &[u32]) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut seen = BTreeSet::new();
    for id in ids {
        if !seen.insert(*id) {
            findings.push(Finding::at(
                DECISIONS,
                None,
                format!("duplicate decision D-{id}"),
            ));
        }
    }
    if let Some(max) = seen.iter().max() {
        let missing: Vec<u32> = (1..=*max).filter(|id| !seen.contains(id)).collect();
        if !missing.is_empty() {
            findings.push(Finding::at(
                DECISIONS,
                None,
                format!("missing decision IDs {missing:?}"),
            ));
        }
    }
    findings
}

/// Requirement sections: ID to body lines; a duplicate is a finding.
fn requirement_sections(text: &str) -> (BTreeMap<String, Vec<String>>, Vec<Finding>) {
    let heading = regex(r"^###\s+((?:N?FR)-(\d+))\b");
    let mut sections: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut findings = Vec::new();
    let mut current: Option<String> = None;
    for line in text.lines() {
        if let Ok(Some(captures)) = heading.captures(line) {
            let id = captures[1].to_owned();
            if sections.contains_key(&id) {
                findings.push(Finding::at(
                    REQUIREMENTS,
                    None,
                    format!("duplicate requirement {id}"),
                ));
            }
            sections.insert(id.clone(), Vec::new());
            current = Some(id);
        } else if line.starts_with("## ") || line.starts_with("### ") {
            current = None;
        } else if let Some(id) = &current {
            sections
                .entry(id.clone())
                .or_default()
                .push(line.to_owned());
        }
    }
    (sections, findings)
}

fn allocation(text: &str) -> BTreeSet<String> {
    let range = regex(r"\b(N?FR)-(\d+)\s*[–—-]\s*(?:N?FR-)?(\d+)\b");
    let single = regex(r"\b(N?FR)-(\d+)\b");
    let mut allocated = BTreeSet::new();
    for line in text.lines() {
        if !line.trim_start().starts_with("| M") {
            continue;
        }
        let cells: Vec<&str> = line.split('|').collect();
        let Some(cell) = cells.get(2) else { continue };
        if !cells.get(1).is_some_and(|c| {
            c.trim_start().starts_with('M')
                && c.trim()[1..].starts_with(|ch: char| ch.is_ascii_digit())
        }) {
            continue;
        }
        for captures in range.captures_iter(*cell).flatten() {
            let (Ok(start), Ok(end)) = (captures[2].parse::<u32>(), captures[3].parse::<u32>())
            else {
                continue;
            };
            for n in start..=end {
                allocated.insert(format!("{}-{n}", &captures[1]));
            }
        }
        for captures in single.captures_iter(*cell).flatten() {
            allocated.insert(format!("{}-{}", &captures[1], &captures[2]));
        }
    }
    allocated
}

fn sort_key(id: &str) -> (String, u32) {
    let (kind, number) = id.split_once('-').unwrap_or((id, "0"));
    (kind.to_owned(), number.parse().unwrap_or(0))
}

/// Checks requirements and their milestone allocation.
#[must_use]
pub fn check_requirements(text: &str) -> (Vec<Finding>, Vec<Finding>) {
    let decision = regex(r"\bD-\d+\b");
    let (sections, mut requirement_findings) = requirement_sections(text);
    if sections.is_empty() {
        requirement_findings.push(Finding::at(
            REQUIREMENTS,
            None,
            "no FR-/NFR- sections found",
        ));
    }
    for (id, body) in &sections {
        let source: Vec<&String> = body
            .iter()
            .filter(|line| line.contains("Rationale / source:"))
            .collect();
        let cited = source
            .iter()
            .any(|line| decision.is_match(line).unwrap_or(false));
        if !cited {
            requirement_findings.push(Finding::at(
                REQUIREMENTS,
                None,
                format!("{id} cites no decision in 'Rationale / source:'"),
            ));
        }
    }
    let allocated = allocation(text);
    let mut allocation_findings = Vec::new();
    let mut missing: Vec<&String> = sections
        .keys()
        .filter(|id| !allocated.contains(*id))
        .collect();
    missing.sort_by_key(|id| sort_key(id));
    for id in missing {
        allocation_findings.push(Finding::at(
            REQUIREMENTS,
            None,
            format!("{id} is not allocated to a milestone"),
        ));
    }
    let mut phantom: Vec<&String> = allocated
        .iter()
        .filter(|id| !sections.contains_key(*id))
        .collect();
    phantom.sort_by_key(|id| sort_key(id));
    for id in phantom {
        allocation_findings.push(Finding::at(
            REQUIREMENTS,
            None,
            format!("milestone table allocates unknown requirement {id}"),
        ));
    }
    (requirement_findings, allocation_findings)
}

fn check_adr(path: &str, text: &str) -> Vec<Finding> {
    let name = regex(r"^(\d{4})-[a-z0-9]+(?:-[a-z0-9]+)*\.md$");
    let file = path.rsplit('/').next().unwrap_or(path);
    let Ok(Some(captures)) = name.captures(file) else {
        return vec![Finding::at(
            path,
            None,
            "name must be NNNN-lower-case-title.md",
        )];
    };
    let number = &captures[1];
    let mut findings = Vec::new();
    if !text
        .lines()
        .next()
        .is_some_and(|line| line.starts_with(&format!("# ADR {number} ")))
    {
        findings.push(Finding::at(
            path,
            Some(1),
            format!("first line must be '# ADR {number} — title'"),
        ));
    }
    let status = regex(r"^- \*\*Status:\*\* \S");
    if !text
        .lines()
        .take(10)
        .any(|line| status.is_match(line).unwrap_or(false))
    {
        findings.push(Finding::at(
            path,
            None,
            "missing '- **Status:**' line near the top",
        ));
    }
    findings
}

/// Runs every documentation gate; returns (name, findings) pairs.
///
/// # Errors
///
/// Fails if Git or file reads fail.
pub fn check(root: &Path) -> Result<Vec<(&'static str, Vec<Finding>)>, String> {
    let files = tracked(root)?;
    let known: BTreeSet<String> = files.iter().cloned().collect();
    let read = |path: &str| {
        std::fs::read_to_string(root.join(path)).map_err(|error| format!("{path}: {error}"))
    };
    let markdown: Vec<&String> = files.iter().filter(|path| path.ends_with(".md")).collect();

    let mut links = Vec::new();
    for path in &markdown {
        links.extend(check_links_in(root, path, &read(path)?, &known));
    }

    let ids = decision_ids(&read(DECISIONS)?);
    let decisions = check_decisions(&ids);
    let known_ids: BTreeSet<u32> = ids.into_iter().collect();
    let reference = regex(r"\bD-(\d+)\b");
    let mut references = Vec::new();
    for path in &markdown {
        if path.starts_with(REVIEWS) || path.starts_with("spikes/") {
            continue;
        }
        for (number, line) in prose_lines(&read(path)?) {
            for captures in reference.captures_iter(line.as_str()).flatten() {
                if let Ok(id) = captures[1].parse::<u32>()
                    && !known_ids.contains(&id)
                {
                    references.push(Finding::at(
                        path.as_str(),
                        Some(number),
                        format!("cites unknown decision D-{id}"),
                    ));
                }
            }
        }
    }

    let (requirements, allocation) = check_requirements(&read(REQUIREMENTS)?);
    let mut adrs = Vec::new();
    for path in &markdown {
        if path.starts_with(ADR_DIR) && !path[ADR_DIR.len()..].contains('/') {
            adrs.extend(check_adr(path, &read(path)?));
        }
    }
    println!("checked {} Markdown files", markdown.len());
    Ok(vec![
        ("links", links),
        ("decisions", decisions),
        ("references", references),
        ("requirements", requirements),
        ("allocation", allocation),
        ("adrs", adrs),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn known() -> BTreeSet<String> {
        ["docs/a.md", "docs/sub/b.md", "README.md"]
            .into_iter()
            .map(str::to_owned)
            .collect()
    }

    fn links(text: &str) -> Vec<Finding> {
        check_links_in(Path::new("/nonexistent"), "docs/a.md", text, &known())
    }

    #[test]
    fn inline_links_resolve() {
        assert!(
            links("[b](sub/b.md) [r](../README.md#x) [w](https://x.org) [s](#top) [d](sub)")
                .is_empty()
        );
        assert_eq!(links("[x](missing.md)").len(), 1);
        assert_eq!(links("![img](missing.png)").len(), 1);
        assert!(links("`[x](missing.md)`").is_empty());
        assert!(links("```\n[x](missing.md)\n```").is_empty());
    }

    // C1 / ADV23-18: reference-style links are checked.
    #[test]
    fn c1_adv23_18_reference_links_are_checked() {
        assert!(links("[b][target]\n\n[target]: sub/b.md").is_empty());
        assert_eq!(links("[broken][target]\n\n[target]: missing.md").len(), 1);
        assert_eq!(links("[broken][nowhere]").len(), 1);
        assert!(links("[Target][]\n\n[target]: sub/b.md").is_empty());
    }

    // ADV23-18: phantom milestone allocations are rejected.
    #[test]
    fn adv23_18_phantom_allocation_is_rejected() {
        let text = "### FR-1 — x\nRationale / source: D-1\n\n## Milestones\n| Milestone | Requirements |\n|---|---|\n| M0 | FR-1, FR-999 |\n";
        let (requirements, allocation) = check_requirements(text);
        assert!(requirements.is_empty());
        assert_eq!(allocation.len(), 1);
        assert!(allocation[0].message.contains("FR-999"));
    }

    #[test]
    fn unallocated_and_uncited_requirements_fail() {
        let text =
            "### FR-1 — x\nno source\n### NFR-2 — y\nRationale / source: D-2\n| M0 | NFR-2 |\n";
        let (requirements, allocation) = check_requirements(text);
        assert_eq!(requirements.len(), 1);
        assert_eq!(allocation.len(), 1);
    }

    #[test]
    fn decision_log_gaps_and_duplicates() {
        assert_eq!(check_decisions(&[1, 2, 2, 4]).len(), 2);
        assert!(check_decisions(&[1, 2, 3]).is_empty());
    }
}
