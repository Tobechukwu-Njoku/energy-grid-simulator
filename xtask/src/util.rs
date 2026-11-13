//! Shared helpers: repository context, Git access, findings and safe output.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Where the checks read the tree under test and the trusted policy.
///
/// `repo` is the checkout being judged (the PR head in CI). `policy` is the
/// trusted tree that supplies policy files and configuration: the base branch
/// in CI, so a pull request cannot weaken the rules applied to itself (R1).
#[derive(Debug, Clone)]
pub struct Ctx {
    /// Tree under test.
    pub repo: PathBuf,
    /// Trusted policy tree.
    pub policy: PathBuf,
}

impl Ctx {
    /// Context whose policy is the tree itself (local runs and tests).
    #[must_use]
    pub fn local(repo: &Path) -> Self {
        Self {
            repo: repo.to_path_buf(),
            policy: repo.to_path_buf(),
        }
    }

    /// Reads a trusted policy file as UTF-8.
    ///
    /// # Errors
    ///
    /// Fails if the file is missing or not UTF-8.
    pub fn policy_text(&self, relative: &str) -> Result<String, String> {
        std::fs::read_to_string(self.policy.join(relative))
            .map_err(|error| format!("trusted policy {relative}: {error}"))
    }

    /// Parses a trusted TOML policy file.
    ///
    /// # Errors
    ///
    /// Fails if the file is missing or is not valid TOML.
    pub fn policy_toml(&self, relative: &str) -> Result<toml_edit::DocumentMut, String> {
        self.policy_text(relative)?
            .parse()
            .map_err(|error| format!("trusted policy {relative}: {error}"))
    }
}

/// One policy violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// Repository-relative file, if any.
    pub file: Option<String>,
    /// One-based line, if known.
    pub line: Option<usize>,
    /// What is wrong. May contain untrusted text; it is escaped on output.
    pub message: String,
}

impl Finding {
    /// A finding not tied to a file.
    #[must_use]
    pub fn general(message: impl Into<String>) -> Self {
        Self {
            file: None,
            line: None,
            message: message.into(),
        }
    }

    /// A finding at a file and optional line.
    #[must_use]
    pub fn at(file: impl Into<String>, line: Option<usize>, message: impl Into<String>) -> Self {
        Self {
            file: Some(file.into()),
            line,
            message: message.into(),
        }
    }
}

/// Escapes text for the data part of a GitHub workflow command (ADV23-14).
#[must_use]
pub fn escape_data(text: &str) -> String {
    text.replace('%', "%25")
        .replace('\r', "%0D")
        .replace('\n', "%0A")
}

/// Escapes text for a workflow command property such as `file=` (ADV23-14).
#[must_use]
pub fn escape_property(text: &str) -> String {
    escape_data(text).replace(':', "%3A").replace(',', "%2C")
}

/// Makes untrusted text safe on a plain log line: no line breaks, no control
/// characters and no `::` or `##[` sequence the runner could read as a command.
#[must_use]
pub fn plain(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for character in text.chars() {
        if character.is_control() {
            let _ = write!(out, "\\u{{{:x}}}", u32::from(character));
        } else {
            out.push(character);
        }
    }
    out.replace("::", ":\\:").replace("##[", "#\\#[")
}

/// Formats one finding: a workflow `::error` command under GitHub Actions,
/// otherwise an indented plain line.
#[must_use]
pub fn format_finding(finding: &Finding, annotate: bool) -> String {
    let location = match (&finding.file, finding.line) {
        (Some(file), Some(line)) => format!("{file}:{line}: "),
        (Some(file), None) => format!("{file}: "),
        _ => String::new(),
    };
    if annotate {
        let mut properties = Vec::new();
        if let Some(file) = &finding.file {
            properties.push(format!("file={}", escape_property(file)));
        }
        if let Some(line) = finding.line {
            properties.push(format!("line={line}"));
        }
        let joined = if properties.is_empty() {
            String::new()
        } else {
            format!(" {}", properties.join(","))
        };
        format!(
            "::error{joined}::{}",
            escape_data(&plain(&format!("{location}{}", finding.message)))
        )
    } else {
        format!("  {}", plain(&format!("{location}{}", finding.message)))
    }
}

/// Prints a named check's result and returns whether it passed.
pub fn report(name: &str, findings: &[Finding]) -> bool {
    let annotate = std::env::var_os("GITHUB_ACTIONS").is_some();
    println!(
        "{}: {name}",
        if findings.is_empty() { "PASS" } else { "FAIL" }
    );
    for finding in findings {
        println!("{}", format_finding(finding, annotate));
    }
    findings.is_empty()
}

/// Runs Git in `repo` with repository attributes neutralised where Git allows.
///
/// # Errors
///
/// Fails if Git cannot run or exits non-zero.
pub fn git(repo: &Path, args: &[&str]) -> Result<Vec<u8>, String> {
    let output = git_command(repo)
        .args(args)
        .output()
        .map_err(|error| format!("git: {error}"))?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(format!(
            "git {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

/// A Git command with deterministic, attribute-independent settings.
#[must_use]
pub fn git_command(repo: &Path) -> Command {
    let mut command = Command::new("git");
    command
        .arg("-C")
        .arg(repo)
        .args([
            "-c",
            "core.quotePath=false",
            "-c",
            "core.attributesFile=/dev/null",
            "-c",
            "diff.external=",
        ])
        .env("GIT_ATTR_NOSYSTEM", "1")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env_remove("GIT_EXTERNAL_DIFF");
    command
}

/// Files tracked by Git, repository-relative.
///
/// # Errors
///
/// Fails if Git fails.
pub fn tracked(repo: &Path) -> Result<Vec<String>, String> {
    let output = git(repo, &["ls-files", "-z"])?;
    Ok(output
        .split(|byte| *byte == 0)
        .filter(|name| !name.is_empty())
        .map(|name| String::from_utf8_lossy(name).into_owned())
        .collect())
}

/// Whether `text` is a full lower-case hexadecimal commit SHA.
#[must_use]
pub fn is_sha(text: &str) -> bool {
    text.len() == 40
        && text
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

/// Simple glob: `*` matches within a path segment, `**` across segments, and a
/// trailing `/` matches everything below a directory.
#[must_use]
pub fn glob_match(pattern: &str, path: &str) -> bool {
    if let Some(prefix) = pattern.strip_suffix('/') {
        return path.starts_with(&format!("{prefix}/"));
    }
    glob_bytes(pattern.as_bytes(), path.as_bytes())
}

fn glob_bytes(pattern: &[u8], path: &[u8]) -> bool {
    match pattern.split_first() {
        None => path.is_empty(),
        Some((b'*', rest)) if rest.first() == Some(&b'*') => {
            let rest = &rest[1..];
            let rest = rest.strip_prefix(b"/").unwrap_or(rest);
            (0..=path.len()).any(|index| glob_bytes(rest, &path[index..]))
        }
        Some((b'*', rest)) => {
            for index in 0..=path.len() {
                if glob_bytes(rest, &path[index..]) {
                    return true;
                }
                if path.get(index) == Some(&b'/') {
                    break;
                }
            }
            false
        }
        Some((head, rest)) => path.first() == Some(head) && glob_bytes(rest, &path[1..]),
    }
}

/// Reads a string array from a TOML item.
#[must_use]
pub fn string_array(item: &toml_edit::Item) -> Vec<String> {
    item.as_array()
        .map(|array| {
            array
                .iter()
                .filter_map(|value| value.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ADV23-14: untrusted text cannot start or inject a workflow command.
    #[test]
    fn adv23_14_annotations_escape_untrusted_text() {
        let finding = Finding::at("a,b:c.rs", Some(3), "bad%0A\n::warning::spoof\r##[group]x");
        let line = format_finding(&finding, true);
        assert!(line.starts_with("::error file=a%2Cb%3Ac.rs,line=3::"));
        assert_eq!(line.lines().count(), 1);
        assert!(!line[2..].contains("::warning"));
        assert!(!line.contains("##["));
        assert!(line.contains("%250A"));
        let plain_line = format_finding(&finding, false);
        assert_eq!(plain_line.lines().count(), 1);
        assert!(!plain_line.contains("::"));
    }

    #[test]
    fn globs() {
        assert!(glob_match("xtask/", "xtask/src/main.rs"));
        assert!(!glob_match("xtask/", "xtaskx/a"));
        assert!(glob_match(
            ".github/workflows/*.yml",
            ".github/workflows/ci.yml"
        ));
        assert!(!glob_match("docs/*", "docs/a/b.md"));
        assert!(glob_match("docs/**", "docs/a/b.md"));
        assert!(glob_match("**/*.py", "a/b/c.py"));
        assert!(glob_match("**/*.py", "c.py"));
    }

    #[test]
    fn sha_validation() {
        assert!(is_sha(&"a".repeat(40)));
        assert!(!is_sha(&"A".repeat(40)));
        assert!(!is_sha("$(touch x)"));
    }
}
