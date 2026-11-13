//! Attribution policy (D-52, D-61, ADR 0001, NFR-3.AC4).
//!
//! Uses the same pattern file as the local `commit-msg` hook, read from the
//! trusted policy tree, and REJECTS matches; CI never rewrites anything.
//!
//! Scanned: every stored commit message in the range (all lines, including any
//! text after a scissors line, R4/ADV23-08), the PR title and body, and every
//! added line of the range. Added lines come from a forced-text diff that
//! ignores `.gitattributes`, text conversion and external diff drivers (R3).
//!
//! Each line is also tested after Unicode NFKC normalisation, removal of
//! zero-width and other format characters, combining-mark removal and
//! confusable folding (ADV23-07), and after stripping comment, quote and
//! Markdown prefixes, link syntax (inline and reference style) and string
//! literal quoting, so comments and headers in any language are covered
//! (R2/ADV23-01). Exemptions name one exact line in one file (ADV23-17).

use std::collections::BTreeSet;
use std::path::Path;

use fancy_regex::{Regex, RegexBuilder};
use unicode_normalization::UnicodeNormalization as _;

use crate::util::{Ctx, Finding, git, git_command, is_sha};

/// Pattern file shared with the commit-msg hook.
pub const PATTERNS: &str = ".githooks/attribution-patterns.regex";
/// Line-scoped exemptions (trusted policy).
pub const EXEMPTIONS: &str = "xtask/policy/attribution-exemptions.toml";

/// Compiled attribution patterns.
#[derive(Debug)]
pub struct Patterns(Vec<Regex>);

impl Patterns {
    /// Loads patterns exactly as the hook does: strict UTF-8, blank and `#`
    /// lines ignored, CRLF accepted, case-insensitive. An invalid, empty or
    /// empty-string-matching rule set is an error.
    ///
    /// # Errors
    ///
    /// Describes the first problem found.
    pub fn load(bytes: &[u8]) -> Result<Self, String> {
        let text = std::str::from_utf8(bytes).map_err(|_| "patterns: invalid UTF-8".to_owned())?;
        let mut rules = Vec::new();
        for raw in text.split('\n') {
            let rule = raw.strip_suffix('\r').unwrap_or(raw);
            let trimmed = rule.trim_start();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let regex = RegexBuilder::new(rule)
                .case_insensitive(true)
                .build()
                .map_err(|error| format!("invalid pattern: {error}"))?;
            if regex.is_match("").unwrap_or(true) {
                return Err("unsafe empty-string attribution pattern".to_owned());
            }
            rules.push(regex);
        }
        if rules.is_empty() {
            return Err("no attribution patterns loaded".to_owned());
        }
        Ok(Self(rules))
    }

    /// Whether one candidate string matches. A matcher error counts as a
    /// match: the check fails closed.
    fn matches_candidate(&self, candidate: &str) -> bool {
        self.0
            .iter()
            .any(|regex| regex.is_match(candidate).unwrap_or(true))
    }

    /// Whether a single line is attribution in any of its normalised forms.
    #[must_use]
    pub fn matches_line(&self, line: &str) -> bool {
        candidates(line)
            .iter()
            .any(|candidate| self.matches_candidate(candidate))
    }

    /// One-based numbers and text of the matching lines of `text`.
    #[must_use]
    pub fn scan_text(&self, text: &str) -> Vec<(usize, String)> {
        split_lines(text)
            .enumerate()
            .filter(|(_, line)| self.matches_line(line))
            .map(|(index, line)| (index + 1, line.to_owned()))
            .collect()
    }
}

/// Splits on every line terminator a renderer might honour, not just `\n`.
fn split_lines(text: &str) -> impl Iterator<Item = &str> {
    text.split([
        '\n', '\r', '\u{0b}', '\u{0c}', '\u{85}', '\u{2028}', '\u{2029}',
    ])
}

/// Unicode format characters (category Cf) and other invisible fillers.
const fn is_invisible(character: char) -> bool {
    matches!(
        character,
        '\u{00ad}'
            | '\u{034f}'
            | '\u{061c}'
            | '\u{115f}'
            | '\u{1160}'
            | '\u{17b4}'
            | '\u{17b5}'
            | '\u{180b}'..='\u{180f}'
            | '\u{200b}'..='\u{200f}'
            | '\u{202a}'..='\u{202e}'
            | '\u{2060}'..='\u{206f}'
            | '\u{3164}'
            | '\u{feff}'
            | '\u{ffa0}'
            | '\u{fff9}'..='\u{fffb}'
            | '\u{0600}'..='\u{0605}'
            | '\u{06dd}'
            | '\u{070f}'
            | '\u{08e2}'
            | '\u{110bd}'
            | '\u{110cd}'
            | '\u{13430}'..='\u{1343f}'
            | '\u{1bca0}'..='\u{1bca3}'
            | '\u{1d173}'..='\u{1d17a}'
            | '\u{e0000}'..='\u{e007f}'
    )
}

/// Combining marks that only decorate a base letter.
const fn is_combining(character: char) -> bool {
    matches!(
        character,
        '\u{0300}'..='\u{036f}'
            | '\u{1ab0}'..='\u{1aff}'
            | '\u{1dc0}'..='\u{1dff}'
            | '\u{20d0}'..='\u{20ff}'
            | '\u{fe20}'..='\u{fe2f}'
    )
}

/// Folds common Cyrillic, Greek and other Latin look-alikes to ASCII.
const fn fold_confusable(character: char) -> char {
    match character {
        'а' | 'α' | 'ɑ' => 'a',
        'А' | 'Α' | 'Ꭺ' => 'A',
        'В' | 'Β' | 'Ᏼ' => 'B',
        'ь' | 'Ь' => 'b',
        'с' | 'ϲ' | 'ᴄ' => 'c',
        'С' | 'Ϲ' | 'Ꮯ' => 'C',
        'ԁ' => 'd',
        'Ꭰ' => 'D',
        'е' | 'ε' | 'ҽ' => 'e',
        'Е' | 'Ε' | 'Ꭼ' => 'E',
        'ɡ' => 'g',
        'һ' => 'h',
        'Н' | 'Η' | 'Ꮋ' => 'H',
        'і' | 'ι' | 'ı' | 'ɩ' => 'i',
        'І' | 'Ι' | 'Ӏ' => 'I',
        'ј' | 'ϳ' => 'j',
        'Ј' | 'Ꭻ' => 'J',
        'κ' | 'к' => 'k',
        'К' | 'Κ' | 'Ꮶ' => 'K',
        'ӏ' | 'ⅼ' => 'l',
        'Ꮮ' => 'L',
        'М' | 'Μ' | 'Ꮇ' => 'M',
        'ո' => 'n',
        'Ν' => 'N',
        'о' | 'ο' | 'օ' | 'σ' => 'o',
        'О' | 'Ο' | 'Օ' => 'O',
        'р' | 'ρ' => 'p',
        'Р' | 'Ρ' | 'Ꮲ' => 'P',
        'ԛ' => 'q',
        'г' => 'r',
        'ѕ' => 's',
        'Ѕ' | 'Ꮪ' => 'S',
        'т' | 'τ' => 't',
        'Т' | 'Τ' | 'Ꭲ' => 'T',
        'υ' | 'ս' => 'u',
        'ν' | 'ѵ' => 'v',
        'Ꮩ' => 'V',
        'ԝ' | 'ѡ' => 'w',
        'Ꮃ' => 'W',
        'х' | 'χ' => 'x',
        'Х' | 'Χ' => 'X',
        'у' | 'γ' => 'y',
        'Ү' | 'Υ' => 'Y',
        'Ζ' | 'Ꮓ' => 'Z',
        other => other,
    }
}

/// NFKC, invisible and combining-mark removal, confusable folding.
#[must_use]
pub fn normalise(line: &str) -> String {
    line.nfkc()
        .filter(|character| !is_invisible(*character) && !is_combining(*character))
        .map(fold_confusable)
        .collect()
}

const PREFIXES: &[&str] = &[
    "<!--", "///", "//!", "//", "/**", "/*!", "/*", "{-", "(*", "\"\"\"", "'''", "#!", "#'", "#",
    "--", ";;", ";", "%", "*", "+", "-", ">", "|", "!", "REM ", "rem ", "::",
];
const SUFFIXES: &[&str] = &["-->", "*/", "-}", "*)", "\"\"\"", "'''", "|", "\\"];

/// Strips comment, quote, list and diff prefixes and comment closers.
fn strip_markup(line: &str) -> String {
    let mut text = line.trim();
    loop {
        let before = text;
        for prefix in PREFIXES {
            if let Some(rest) = text.strip_prefix(prefix) {
                text = rest.trim_start();
                break;
            }
        }
        if let Some(rest) = strip_list_number(text) {
            text = rest.trim_start();
        }
        for suffix in SUFFIXES {
            if let Some(rest) = text.strip_suffix(suffix) {
                text = rest.trim_end();
                break;
            }
        }
        if text == before {
            return text.to_owned();
        }
    }
}

fn strip_list_number(text: &str) -> Option<&str> {
    let digits = text.bytes().take_while(u8::is_ascii_digit).count();
    if digits == 0 {
        return None;
    }
    text[digits..]
        .strip_prefix('.')
        .or_else(|| text[digits..].strip_prefix(')'))
}

/// Replaces `[text](target)`, `[text][ref]` and `[text]` with `text`.
fn delink(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(open) = rest.find('[') {
        let Some(close) = rest[open..].find(']').map(|index| open + index) else {
            break;
        };
        out.push_str(&rest[..open]);
        out.push_str(&rest[open + 1..close]);
        let after = &rest[close + 1..];
        rest = match after.chars().next() {
            Some('(') => after.find(')').map_or(after, |end| &after[end + 1..]),
            Some('[') => after.find(']').map_or(after, |end| &after[end + 1..]),
            _ => after,
        };
    }
    out.push_str(rest);
    out
}

/// Removes HTML/XML tags that are not e-mail addresses.
fn untag(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(open) = rest.find('<') {
        let tail = &rest[open..];
        let close = tail.find('>');
        let is_tag = close.is_some_and(|end| {
            let inner = &tail[1..end];
            !inner.contains('@')
                && inner
                    .trim_start_matches('/')
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_ascii_alphabetic() || c == '!')
        });
        out.push_str(&rest[..open]);
        match (is_tag, close) {
            (true, Some(end)) => rest = &tail[end + 1..],
            _ => {
                out.push('<');
                rest = &tail[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

/// Contents of every quoted segment (string literals, code spans).
fn quoted(line: &str) -> Vec<String> {
    let mut found = Vec::new();
    for quote in ['"', '\'', '`'] {
        let parts: Vec<&str> = line.split(quote).collect();
        for (index, part) in parts.iter().enumerate() {
            if index % 2 == 1 && index + 1 < parts.len() {
                found.push((*part).to_owned());
            }
        }
    }
    found
}

/// Every form of a line that is tested against the patterns.
#[must_use]
pub fn candidates(line: &str) -> Vec<String> {
    let mut set = BTreeSet::new();
    set.insert(line.to_owned());
    let base = normalise(line);
    let unescaped = base.replace("\\n", " ").replace("\\t", " ");
    for form in [base.clone(), unescaped] {
        let delinked = delink(&form);
        let untagged = untag(&delinked);
        let plain: String = untagged
            .chars()
            .filter(|c| !matches!(c, '*' | '_' | '~' | '`'))
            .collect();
        for variant in [&form, &delinked, &untagged, &plain] {
            set.insert(variant.clone());
            set.insert(strip_markup(variant));
        }
        for segment in quoted(&form) {
            set.insert(strip_markup(&segment));
            set.insert(strip_markup(&untag(&delink(&segment))));
        }
    }
    set.into_iter().collect()
}

/// One added line with its new-file line number.
pub type AddedLine = (usize, Vec<u8>);

/// Added lines of one file's unified diff, numbered in the new file. Context
/// lines advance the counter, so any `--unified` setting works (ADV23-09).
#[must_use]
pub fn parse_hunks(diff: &[u8]) -> Vec<AddedLine> {
    let mut found = Vec::new();
    let mut number: Option<usize> = None;
    for line in diff.split(|byte| *byte == b'\n') {
        if line.starts_with(b"@@ ") {
            number = hunk_start(line);
            continue;
        }
        let Some(current) = number else {
            continue;
        };
        match line.first() {
            Some(b'+') => {
                found.push((current, line[1..].to_vec()));
                number = Some(current + 1);
            }
            Some(b' ') | None => number = Some(current + 1),
            _ => {}
        }
    }
    found
}

fn hunk_start(line: &[u8]) -> Option<usize> {
    let text = std::str::from_utf8(line).ok()?;
    let plus = text.split(' ').find(|part| part.starts_with('+'))?;
    plus[1..].split(',').next()?.parse().ok()
}

/// What to scan.
#[derive(Debug, Clone, Default)]
pub struct Range {
    /// Exclusive base revision; `None` scans the whole history of `head`.
    pub base: Option<String>,
    /// Head revision.
    pub head: String,
    /// Pull request title.
    pub title: Option<String>,
    /// Pull request body.
    pub body: Option<String>,
}

impl Range {
    /// Reads the range and metadata from the GitHub event file.
    ///
    /// # Errors
    ///
    /// Fails on an unsupported event or an invalid SHA.
    pub fn from_event(name: &str, event: &serde_json::Value) -> Result<Self, String> {
        let sha = |value: &serde_json::Value| -> Result<Option<String>, String> {
            match value.as_str() {
                Some(text) if text.bytes().all(|byte| byte == b'0') => Ok(None),
                Some(text) if is_sha(text) => Ok(Some(text.to_owned())),
                _ => Err("event: missing or invalid commit SHA".to_owned()),
            }
        };
        match name {
            "pull_request" | "pull_request_review" => {
                let pull = &event["pull_request"];
                Ok(Self {
                    base: sha(&pull["base"]["sha"])?,
                    head: sha(&pull["head"]["sha"])?.ok_or("event: no head SHA")?,
                    title: Some(pull["title"].as_str().unwrap_or_default().to_owned()),
                    body: Some(pull["body"].as_str().unwrap_or_default().to_owned()),
                })
            }
            "push" => Ok(Self {
                base: sha(&event["before"])?,
                head: sha(&event["after"])?.ok_or("event: no head SHA")?,
                title: None,
                body: None,
            }),
            other => Err(format!("unsupported event {other}")),
        }
    }
}

/// Line-scoped exemption: one exact line in one file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exemption {
    /// Repository-relative path.
    pub path: String,
    /// The exact added line.
    pub line: String,
}

/// Loads exemptions from the trusted policy; each needs a path, a line and a
/// reason. Path wildcards are rejected (ADV23-17).
///
/// # Errors
///
/// Fails on a malformed entry.
pub fn load_exemptions(ctx: &Ctx) -> Result<Vec<Exemption>, String> {
    let document = ctx.policy_toml(EXEMPTIONS)?;
    let mut found = Vec::new();
    if let Some(tables) = document
        .get("exemption")
        .and_then(toml_edit::Item::as_array_of_tables)
    {
        for table in tables {
            let field = |key: &str| {
                table
                    .get(key)
                    .and_then(toml_edit::Item::as_str)
                    .map(str::to_owned)
            };
            let (Some(path), Some(line), Some(reason)) =
                (field("path"), field("line"), field("reason"))
            else {
                return Err(format!(
                    "{EXEMPTIONS}: every exemption needs path, line and reason"
                ));
            };
            if path.contains(['*', '?', '[']) || reason.trim().is_empty() || line.trim().is_empty()
            {
                return Err(format!(
                    "{EXEMPTIONS}: exemption for {path} must name one file and one line"
                ));
            }
            found.push(Exemption { path, line });
        }
    }
    Ok(found)
}

/// Counts reported after a scan.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Summary {
    /// Commit messages scanned.
    pub commits: usize,
    /// Added lines scanned.
    pub added_lines: usize,
}

/// Scans a range and its metadata.
///
/// # Errors
///
/// Fails if the policy cannot be loaded or Git fails.
pub fn scan_range(ctx: &Ctx, range: &Range) -> Result<(Vec<Finding>, Summary), String> {
    let pattern_bytes = std::fs::read(ctx.policy.join(PATTERNS))
        .map_err(|error| format!("trusted policy {PATTERNS}: {error}"))?;
    let patterns = Patterns::load(&pattern_bytes)?;
    let exemptions = load_exemptions(ctx)?;
    let repo = ctx.repo.as_path();
    let head = verify_commit(repo, &range.head)?;
    let base = range
        .base
        .as_deref()
        .map(|base| verify_commit(repo, base))
        .transpose()?;

    let mut findings = Vec::new();
    let mut summary = Summary::default();

    let selector = base
        .as_ref()
        .map_or_else(|| head.clone(), |base| format!("{base}..{head}"));
    let listed = git(repo, &["rev-list", &selector])?;
    for sha in String::from_utf8_lossy(&listed).split_whitespace() {
        summary.commits += 1;
        let raw = git(repo, &["cat-file", "commit", sha])?;
        let message = raw
            .windows(2)
            .position(|pair| pair == b"\n\n")
            .map_or(&raw[..0], |index| &raw[index + 2..]);
        let short = &sha[..12];
        match std::str::from_utf8(message) {
            Ok(text) => {
                for (line, content) in patterns.scan_text(text) {
                    findings.push(Finding::general(format!(
                        "commit {short} message line {line}: attribution: {}",
                        clip(&content)
                    )));
                }
            }
            Err(_) => findings.push(Finding::general(format!(
                "commit {short}: message is not valid UTF-8"
            ))),
        }
    }
    for (label, text) in [
        ("pull request title", &range.title),
        ("pull request body", &range.body),
    ] {
        if let Some(text) = text {
            for (line, content) in patterns.scan_text(text) {
                findings.push(Finding::general(format!(
                    "{label} line {line}: attribution: {}",
                    clip(&content)
                )));
            }
        }
    }

    let from = match &base {
        Some(base) => String::from_utf8_lossy(&git(repo, &["merge-base", base, &head])?)
            .trim()
            .to_owned(),
        None => empty_tree(repo)?,
    };
    let names = git(
        repo,
        &[
            "diff",
            "--name-only",
            "-z",
            "--no-renames",
            "--no-ext-diff",
            "--diff-filter=d",
            &from,
            &head,
        ],
    )?;
    for name in names
        .split(|byte| *byte == 0)
        .filter(|name| !name.is_empty())
    {
        let path = String::from_utf8_lossy(name).into_owned();
        let pathspec = format!(":(literal){path}");
        let diff = git(
            repo,
            &[
                "diff",
                "--text",
                "--no-textconv",
                "--no-ext-diff",
                "--no-color",
                "--no-renames",
                "--unified=0",
                &from,
                &head,
                "--",
                &pathspec,
            ],
        )?;
        for (number, bytes) in parse_hunks(&diff) {
            summary.added_lines += 1;
            let text = String::from_utf8_lossy(&bytes);
            let text = text.strip_suffix('\r').unwrap_or(&text);
            if !patterns.matches_line(text) {
                continue;
            }
            if exemptions.iter().any(|e| e.path == path && e.line == text) {
                continue;
            }
            findings.push(Finding::at(
                path.clone(),
                Some(number),
                format!("attribution in added line: {}", clip(text)),
            ));
        }
    }
    Ok((findings, summary))
}

fn clip(text: &str) -> String {
    let mut clipped: String = text.chars().take(160).collect();
    if clipped.len() < text.len() {
        clipped.push('…');
    }
    clipped
}

fn verify_commit(repo: &Path, revision: &str) -> Result<String, String> {
    let spec = format!("{revision}^{{commit}}");
    let output = git(
        repo,
        &[
            "rev-parse",
            "--verify",
            "--quiet",
            "--end-of-options",
            &spec,
        ],
    )?;
    Ok(String::from_utf8_lossy(&output).trim().to_owned())
}

fn empty_tree(repo: &Path) -> Result<String, String> {
    let output = git_command(repo)
        .args(["hash-object", "-t", "tree", "--stdin"])
        .stdin(std::process::Stdio::null())
        .output()
        .map_err(|error| format!("git: {error}"))?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn patterns() -> Patterns {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join(PATTERNS);
        Patterns::load(&std::fs::read(path).expect("pattern file")).expect("patterns load")
    }

    /// Builds attribution text from pieces so this file holds no literal.
    fn j(parts: &[&str]) -> String {
        parts.concat()
    }

    fn footer() -> String {
        j(&["Gener", "ated with Cla", "ude Code"])
    }

    fn trailer() -> String {
        j(&[
            "Co-Auth",
            "ored-By: Cla",
            "ude Opus 5.5 <noreply@",
            "anthropic.com>",
        ])
    }

    // R2 / ADV23-01: comments and headers in any language.
    #[test]
    fn r2_adv23_01_comment_prefixes_are_detected() {
        let p = patterns();
        let footer = footer();
        for line in [
            format!("// {footer}"),
            format!("/// {footer}"),
            format!("//! {footer}"),
            format!("/* {footer} */"),
            format!(" * {footer}"),
            format!("# {footer}"),
            format!("-- {footer}"),
            format!("<!-- {footer} -->"),
            format!("; {footer}"),
            format!("+{footer}"),
            format!("- {footer}"),
            format!("1. {footer}"),
            format!("> **{footer}**"),
            format!("#[doc = \"{footer}\"]"),
            format!("println!(\"{footer}\");"),
            format!("<sub>{footer}</sub>"),
            format!("// {}", trailer()),
        ] {
            assert!(p.matches_line(&line), "missed: {line}");
        }
    }

    // ADV23-07: invisible characters, compatibility forms and homoglyphs.
    #[test]
    fn adv23_07_unicode_evasions_are_detected() {
        let p = patterns();
        for line in [
            j(&["Gener", "ated with Cl\u{200b}aude Code"]),
            j(&["Gener", "ated with Cl\u{0430}ude Code"]),
            j(&[
                "Co\u{00ad}-Auth",
                "ored-By: Cla",
                "ude <noreply@",
                "anthropic.com>",
            ]),
            j(&["\u{ff27}ener", "ated with \u{ff23}laude"]),
            j(&["Gener", "ated with C\u{0332}laude"]),
            j(&["Gener", "ated with Cl\u{0430}\u{200d}ude Code"]),
        ] {
            assert!(p.matches_line(&line), "missed: {line:?}");
        }
    }

    // Reference-style Markdown links do not hide a footer.
    #[test]
    fn reference_style_links_are_detected() {
        let p = patterns();
        assert!(p.matches_line(&j(&["Gener", "ated with [Cla", "ude Code][1]"])));
        assert!(p.matches_line(&j(&["Gener", "ated with [Cla", "ude Code]"])));
        assert!(p.matches_line(&j(&[
            "Gener",
            "ated with [Cla",
            "ude Code](https://example.com)"
        ])));
    }

    #[test]
    fn ordinary_prose_and_human_trailers_pass() {
        let p = patterns();
        for line in [
            "Co-authored-by: Jane Doe <jane@example.org>",
            "// Generated with care by the build script",
            "let generated = with_claude_monet();",
            "No AI attribution in commits (D-52).",
            "- **No AI attribution** in commits, PR titles and bodies",
            "Model names may appear only in process docs",
            "Reviewed-by: Ada Lovelace <ada@example.org>",
            "привет мир",
        ] {
            assert!(!p.matches_line(line), "false positive: {line}");
        }
    }

    // R4 / ADV23-08: text after a scissors line is still scanned.
    #[test]
    fn r4_adv23_08_scissors_does_not_truncate() {
        let p = patterns();
        let text = format!(
            "fix: thing\n# ------------------------ >8 ------------------------\n{}\n",
            trailer()
        );
        assert_eq!(p.scan_text(&text).len(), 1);
        assert_eq!(p.scan_text(&text)[0].0, 3);
    }

    #[test]
    fn hidden_line_terminators_split_lines() {
        let p = patterns();
        let text = format!("fix: thing\u{2028}{}", footer());
        assert_eq!(p.scan_text(&text).len(), 1);
    }

    #[test]
    fn bad_pattern_files_fail_closed() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join(PATTERNS);
        let original = std::fs::read(path).expect("pattern file");
        for suffix in [&b".*\n"[..], b"[\n", b"\xff\n"] {
            let mut variant = original.clone();
            variant.extend_from_slice(suffix);
            assert!(Patterns::load(&variant).is_err(), "{suffix:?}");
        }
        assert!(Patterns::load(b"# none\n\n").is_err());
        let crlf: Vec<u8> = String::from_utf8(original)
            .expect("utf8")
            .replace('\n', "\r\n")
            .into_bytes();
        assert!(Patterns::load(&crlf).is_ok());
    }

    // ADV23-09: context lines advance the new-file line number.
    #[test]
    fn adv23_09_context_lines_are_counted() {
        let diff = b"diff --git a/x b/x\n--- a/x\n+++ b/x\n@@ -1,3 +10,5 @@ fn\n ctx\n-old\n+new1\n ctx2\n\n+new2\n\\ No newline at end of file\n@@ -40 +42,0 @@\n-gone\n@@ -50,0 +51 @@\n+last\n";
        let added = parse_hunks(diff);
        let numbers: Vec<usize> = added.iter().map(|(number, _)| *number).collect();
        assert_eq!(numbers, [11, 14, 51]);
        assert_eq!(added[0].1, b"new1");
        // A `+++` header is not an added line.
        assert!(parse_hunks(b"+++ b/x\n").is_empty());
    }

    #[test]
    fn event_parsing_rejects_bad_shas() {
        let good = "a".repeat(40);
        let event = serde_json::json!({"pull_request": {"base": {"sha": good}, "head": {"sha": "$(id)"}, "title": "t", "body": null}});
        assert!(Range::from_event("pull_request", &event).is_err());
        let event = serde_json::json!({"before": "0".repeat(40), "after": good});
        let range = Range::from_event("push", &event).expect("push");
        assert!(range.base.is_none());
        assert!(Range::from_event("workflow_run", &event).is_err());
    }
}
