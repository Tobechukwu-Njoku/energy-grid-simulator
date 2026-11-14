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
//! Each line is also tested after Unicode NFKD decomposition, removal of
//! zero-width and other format characters, removal of every combining mark
//! (so precomposed accents fall away, ADV23R3-01) and confusable folding
//! (ADV23-07), and after stripping comment, quote and Markdown prefixes, link
//! syntax (inline and reference style) and string literal quoting. Comment
//! delimiters are also recognised generically, as any run of characters that
//! are neither letters, digits nor white space, so comments in any language
//! are covered without a per-language list (R2/ADV23-01, ADV23R3-03).
//! Exemptions name one exact line in one file (ADV23-17).

use std::collections::BTreeSet;
use std::path::Path;
use std::sync::OnceLock;

use fancy_regex::{Regex, RegexBuilder};
use unicode_normalization::UnicodeNormalization as _;
use unicode_normalization::char::is_combining_mark;
use unicode_security::confusable_detection::skeleton;

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
        let mut fragments: Vec<(String, String)> = Vec::new();
        for raw in text.split('\n') {
            let rule = raw.strip_suffix('\r').unwrap_or(raw);
            let trimmed = rule.trim_start();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if let Some(definition) = trimmed.strip_prefix("define ") {
                let (name, body) = definition
                    .split_once(" = ")
                    .ok_or_else(|| format!("invalid fragment definition: {definition}"))?;
                let name = name.trim();
                if name.is_empty() || !name.chars().all(|c| c.is_ascii_uppercase() || c == '_') {
                    return Err(format!("invalid fragment name: {name}"));
                }
                let body = expand(body, &fragments)?;
                fragments.push((name.to_owned(), body));
                continue;
            }
            let rule = expand(rule, &fragments)?;
            let regex = RegexBuilder::new(&rule)
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

    /// Whether a commit identity (`Name <email>`) is an AI tool or model:
    /// it is tested exactly as a co-author trailer naming it would be.
    #[must_use]
    pub fn matches_identity(&self, identity: &str) -> bool {
        self.matches_line(&format!("Co-authored-by: {identity}"))
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

/// Replaces every `{{NAME}}` with its earlier `define NAME = body`; an
/// unknown name is an error, so a typo cannot silently drop a rule.
fn expand(rule: &str, fragments: &[(String, String)]) -> Result<String, String> {
    let mut out = String::with_capacity(rule.len());
    let mut rest = rule;
    while let Some(open) = rest.find("{{") {
        let close = rest[open..]
            .find("}}")
            .map(|end| open + end)
            .ok_or_else(|| format!("unterminated fragment reference in: {rule}"))?;
        let name = &rest[open + 2..close];
        let body = fragments
            .iter()
            .rev()
            .find(|(known, _)| known == name)
            .map(|(_, body)| body)
            .ok_or_else(|| format!("undefined fragment {{{{{name}}}}}"))?;
        out.push_str(&rest[..open]);
        out.push_str(body);
        rest = &rest[close + 2..];
    }
    out.push_str(rest);
    Ok(out)
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

/// Folds one non-ASCII character to its ASCII look-alike using the Unicode
/// confusables table (UTS #39 skeletons, ADV23-07, ADV23R-01/02). When the
/// character itself has no ASCII prototype, its upper- and lower-case forms
/// are tried, so case pairs such as Cyrillic `м`/`М` fold alike. `l_as` is
/// the letter used for the shared `I`/`l`/`1` prototype.
fn fold_char(character: char, l_as: char, out: &mut String) {
    if character.is_ascii() {
        out.push(character);
        return;
    }
    let ascii = |text: &str| -> Option<String> {
        let skeleton: String = skeleton(text).collect();
        (!skeleton.is_empty() && skeleton.is_ascii()).then_some(skeleton)
    };
    let mut buffer = [0; 4];
    let folded = ascii(character.encode_utf8(&mut buffer))
        .or_else(|| ascii(&character.to_uppercase().collect::<String>()))
        .or_else(|| ascii(&character.to_lowercase().collect::<String>()));
    match folded {
        Some(text) => out.extend(text.chars().map(|c| if c == 'l' { l_as } else { c })),
        None => out.push(character),
    }
}

/// Pictographs, emoji modifiers and joiners (`\p{So}`, `\p{Sk}`), which an
/// attribution line may be decorated with (ADV23R-03).
fn pictographs() -> &'static Regex {
    static PICTOGRAPHS: OnceLock<Regex> = OnceLock::new();
    PICTOGRAPHS.get_or_init(|| {
        Regex::new(r"[\p{So}\p{Sk}\x{FE0F}\x{FE0E}\x{200D}\x{20E3}]").expect("pictograph class")
    })
}

/// Compatibility decomposition (NFKD), removal of invisible characters and
/// of every combining mark (Unicode category M), then confusable folding.
/// Decomposing first splits precomposed letters such as `á` into a base
/// letter and a mark, so accents of any kind fall away (ADV23R3-01).
#[must_use]
pub fn normalise(line: &str) -> String {
    normalise_with(line, 'l')
}

fn normalise_with(line: &str, l_as: char) -> String {
    let mut out = String::with_capacity(line.len());
    for character in line
        .nfkd()
        .filter(|character| !is_invisible(*character) && !is_combining_mark(*character))
    {
        fold_char(character, l_as, &mut out);
    }
    out
}

/// Forms of a line with pictographs removed.
fn depictograph(line: &str) -> String {
    pictographs().replace_all(line, "").into_owned()
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

/// Comment and cell markers after which a payload may start anywhere in a
/// line, not only at its beginning (F3): `code(); // footer`.
const TRAILING_MARKERS: &[&str] = &[
    "//", "/*", "<!--", "#", "--", ";", "%", "{-", "(*", "\"\"\"", "'''", "|", "REM ", "rem ",
];

/// Every suffix of `line` that starts at a comment or cell marker after
/// column one (column one is already covered by `strip_markup`).
fn trailing_payloads(line: &str) -> BTreeSet<&str> {
    let mut found = BTreeSet::new();
    for marker in TRAILING_MARKERS {
        for (start, _) in line.match_indices(marker) {
            if start > 0 {
                found.insert(&line[start..]);
            }
        }
    }
    found
}

/// A comment or markup delimiter: any character that is neither a letter,
/// a digit nor white space. Comment syntax differs between languages
/// (`<# #>`, `--[[ ]]`, `@`, `REM`, `!`, `%`, ...), so delimiters are
/// recognised generically rather than from a list (ADV23R3-03).
fn is_delimiter(character: char) -> bool {
    !character.is_alphanumeric() && !character.is_whitespace()
}

/// Generic delimiter stripping. Returns the forms of `line` that start at the
/// line start or at the start of any run of delimiters, whether or not white
/// space precedes it (`$x = 1<# ... #>`, `mov r0, r1@...`, ADV23R4-04), with
/// those leading delimiters removed; each form is returned
/// as is, without trailing delimiter-only tokens, and without any trailing
/// delimiters. A payload hidden behind any comment syntax, at the start of a
/// line or after code, is thereby exposed without a per-language list.
fn delimiter_forms(line: &str) -> BTreeSet<String> {
    let mut starts = vec![0];
    let mut previous_delimiter = true;
    for (index, character) in line.char_indices() {
        let delimiter = is_delimiter(character);
        if delimiter && !previous_delimiter && index > 0 {
            starts.push(index);
        }
        previous_delimiter = delimiter;
    }
    let mut found = BTreeSet::new();
    for start in starts {
        let body = line[start..].trim_start_matches(|c: char| is_delimiter(c) || c.is_whitespace());
        if body.is_empty() {
            continue;
        }
        found.insert(body.to_owned());
        let mut words: Vec<&str> = body.split_whitespace().collect();
        while words
            .last()
            .is_some_and(|word| word.chars().all(is_delimiter))
        {
            words.pop();
        }
        found.insert(words.join(" "));
        found.insert(
            body.trim_end_matches(|c: char| is_delimiter(c) || c.is_whitespace())
                .to_owned(),
        );
    }
    found
}

/// Every form of a line that is tested against the patterns.
#[must_use]
pub fn candidates(line: &str) -> Vec<String> {
    let mut set = BTreeSet::new();
    set.insert(line.to_owned());
    let mut bases = BTreeSet::new();
    for l_as in ['l', 'i'] {
        let base = normalise_with(line, l_as);
        bases.insert(depictograph(&base));
        bases.insert(base);
    }
    for base in bases {
        let unescaped = base.replace("\\n", " ").replace("\\t", " ");
        for form in [base, unescaped] {
            let delinked = delink(&form);
            let untagged = untag(&delinked);
            let plain: String = untagged
                .chars()
                .filter(|c| !matches!(c, '*' | '_' | '~' | '`'))
                .collect();
            for variant in [&form, &delinked, &untagged, &plain] {
                set.insert(variant.clone());
                set.insert(strip_markup(variant));
                for payload in trailing_payloads(variant) {
                    set.insert(strip_markup(payload));
                }
                set.extend(delimiter_forms(variant));
            }
            for segment in quoted(&form) {
                set.insert(strip_markup(&segment));
                set.insert(strip_markup(&untag(&delink(&segment))));
            }
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
        let header = &raw[..raw.len() - message.len()];
        for (role, identity) in identities(header) {
            if patterns.matches_identity(&identity) {
                findings.push(Finding::general(format!(
                    "commit {short} {role} is an AI identity: {}",
                    clip(&identity)
                )));
            }
        }
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

/// `author` and `committer` identities (`Name <email>`) of a raw commit
/// object header (ADV23R-05). Timestamps are dropped.
#[must_use]
pub fn identities(header: &[u8]) -> Vec<(&'static str, String)> {
    let text = String::from_utf8_lossy(header);
    let mut found = Vec::new();
    for line in text.lines() {
        for role in ["author", "committer"] {
            if let Some(rest) = line.strip_prefix(role).and_then(|r| r.strip_prefix(' ')) {
                let identity = rest.rfind('>').map_or(rest, |end| &rest[..=end]).to_owned();
                found.push((role, identity));
            }
        }
    }
    found
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

    // F3: a comment or cell after code on the same line.
    #[test]
    fn f3_trailing_comments_are_detected() {
        let p = patterns();
        let footer = footer();
        let trailer = trailer();
        for line in [
            format!("pub const N: u8 = 0; // {footer}"),
            format!("pub const N: u8 = 0;// {footer}"),
            format!("let x = 1; /* {footer} */"),
            format!("x = 1  # {footer}"),
            format!("SELECT 1; -- {footer}"),
            format!("<p>text</p> <!-- {footer} -->"),
            format!("(setq x 1) ; {footer}"),
            format!("| cell | {footer} |"),
            format!("fn f() {{}} // {trailer}"),
        ] {
            assert!(p.matches_line(&line), "missed: {line}");
        }
        for line in [
            "let url = \"https://example.com/generated/with\"; // ordinary",
            "x = 1  # generated with care",
            "| Generated with | the build script |",
        ] {
            assert!(!p.matches_line(line), "false positive: {line}");
        }
    }

    // ADV23R-01: Unicode minus and dash variants in trailer hyphens.
    #[test]
    fn adv23r_01_unicode_dashes_are_detected() {
        let p = patterns();
        for dash in [
            '\u{2212}', '\u{2010}', '\u{2011}', '\u{2013}', '\u{2014}', '\u{2043}', '\u{fe58}',
            '\u{fe63}', '\u{ff0d}', '\u{2796}', '\u{02d7}',
        ] {
            let line = j(&[
                "Co",
                "-Auth",
                "ored",
                "-By: Cla",
                "ude <noreply@",
                "anthropic.com>",
            ])
            .replace('-', &dash.to_string());
            assert!(p.matches_line(&line), "missed U+{:04X}", u32::from(dash));
        }
    }

    // ADV23R-02: confusables come from the Unicode (UTS #39) table, in both cases.
    #[test]
    fn adv23r_02_confusable_table_is_complete() {
        let p = patterns();
        for line in [
            j(&["Gener", "ated with Ge\u{043c}ini"]),
            j(&["Gener", "ated with GE\u{041c}INI"]),
            j(&["Gener", "ated with Gem\u{0456}ni"]),
            j(&["GENER", "ATED WITH GEM\u{0406}NI"]),
            j(&["Gener", "ated with C\u{13de}aude"]),
            j(&["Gener", "ated with \u{a4d6}emini"]),
            j(&["Gener", "ated with Cla\u{057d}de"]),
            j(&["Gener", "ated with Clau\u{0501}e"]),
            j(&["Gener", "ated with Co\u{0440}ilot"]),
        ] {
            assert!(p.matches_line(&line), "missed: {line:?}");
        }
        assert!(normalise("\u{043c}").eq_ignore_ascii_case("m"));
        assert!(!p.matches_line("Привет, мир: generated with care"));
    }

    // ADV23R-03: leading pictographs before footers and trailers.
    #[test]
    fn adv23r_03_pictograph_prefixes_are_detected() {
        let p = patterns();
        for prefix in [
            "\u{2728}",
            "\u{1f9e0}",
            "\u{1f4bb}",
            "\u{26a1}\u{fe0f}",
            "\u{1f916}",
        ] {
            for body in [footer(), trailer()] {
                let line = format!("{prefix} {body}");
                assert!(p.matches_line(&line), "missed: {line}");
            }
        }
        assert!(p.matches_line(&j(&["Gener", "ated with \u{2728} Cla", "ude Code"])));
        assert!(!p.matches_line("\u{2728} New feature: faster parsing"));
    }

    // ADV23R-04: Copilot without an e-mail and spaced trailer keys.
    #[test]
    fn adv23r_04_copilot_and_spaced_trailers_are_detected() {
        let p = patterns();
        for line in [
            j(&["Co-Auth", "ored-By: GitHub Cop", "ilot"]),
            j(&["Co-auth", "ored-by: github-cop", "ilot"]),
            j(&["Co-auth", "ored by: Cla", "ude <noreply@", "anthropic.com>"]),
            j(&["Co auth", "ored by: Cla", "ude"]),
            j(&["Auth", "ored by : Gem", "ini"]),
            j(&["Signed off by: Cla", "ude"]),
        ] {
            assert!(p.matches_line(&line), "missed: {line}");
        }
        for line in [
            "Co-authored by: Jane Doe <jane@example.org>",
            "Photo by: Claude Monet",
            "Reviewed by: Ada Lovelace",
        ] {
            assert!(!p.matches_line(line), "false positive: {line}");
        }
    }

    // ADV23R-05: author and committer identities of a commit object.
    #[test]
    fn adv23r_05_commit_identities_are_checked() {
        let p = patterns();
        let raw = j(&[
            "tree 4b825dc642cb6eb9a060e54bf8d69288fbee4904\n",
            "author Cla",
            "ude Code <cla",
            "ude@anthropic.com> 1700000000 +0000\n",
            "committer Jane Doe <jane@example.org> 1700000000 +0000\n",
        ]);
        let found = identities(raw.as_bytes());
        assert_eq!(found.len(), 2);
        assert!(p.matches_identity(&found[0].1), "{found:?}");
        assert!(!p.matches_identity(&found[1].1), "{found:?}");
        for identity in [
            j(&[
                "Cop",
                "ilot <198982749+Cop",
                "ilot@users.noreply.github.com>",
            ]),
            j(&[
                "gem",
                "ini-code-assist[bot] <1+gem",
                "ini[bot]@users.noreply.github.com>",
            ]),
            j(&["Agent <x@", "openai.com>"]),
        ] {
            assert!(p.matches_identity(&identity), "missed: {identity}");
        }
        for identity in [
            "egs-bot <egs-bot@users.noreply.github.com>",
            "Claude Shannon <cshannon@example.com>",
        ] {
            assert!(!p.matches_identity(identity), "false positive: {identity}");
        }
    }

    // ADV23R3-01: precomposed accents decompose and their marks are removed.
    #[test]
    fn adv23r3_01_precomposed_diacritics_are_detected() {
        let p = patterns();
        for line in [
            j(&["Gener", "ated with Cl\u{00e1}", "ude Code"]),
            j(&["G\u{00e9}n\u{00e9}r", "ated with Cl\u{00e0}", "ude"]),
            j(&["Co-Auth", "ored-By: Cl\u{00e2}", "ude"]),
            j(&["Co-auth", "ored-by: G\u{00e9}m\u{00ed}n\u{00ed}"]),
            j(&["Gener", "ated with C\u{014d}p\u{012b}l\u{014d}t"]),
            j(&["Gener", "ated with \u{0108}laude"]),
            j(&["Gener", "ated with Cla\u{1ee5}de"]),
        ] {
            assert!(p.matches_line(&line), "missed: {line:?}");
        }
        assert_eq!(normalise("Cl\u{00e1}ude"), "Claude");
        assert!(!p.matches_line("R\u{00e9}sum\u{00e9} generated with care"));
    }

    // ADV23R3-02: a vendor name before the model name.
    #[test]
    fn adv23r3_02_vendor_prefixed_models_are_detected() {
        let p = patterns();
        for line in [
            j(&["Gener", "ated with Google Gem", "ini"]),
            j(&["Gener", "ated with OpenAI Chat", "GPT"]),
            j(&["Gener", "ated with Meta Lla", "ma 3"]),
            j(&["Gener", "ated with Google's Gem", "ini 2.5 Pro"]),
            j(&["Gener", "ated with Microsoft GitHub Cop", "ilot"]),
            j(&[
                "Gener",
                "ated with [Google Gem",
                "ini](https://example.com)",
            ]),
            j(&["Co-Auth", "ored-", "By: Anthropic Cla", "ude"]),
            j(&["Co-Auth", "ored-By: Google Gem", "ini <x@example.org>"]),
            j(&["Co-Auth", "ored-", "By: Mistral AI Mis", "tral Large"]),
        ] {
            assert!(p.matches_line(&line), "missed: {line}");
        }
        for line in [
            "Generated with Google Sheets",
            "Co-authored-by: Meta Hansen <meta@example.org>",
        ] {
            assert!(!p.matches_line(line), "false positive: {line}");
        }
    }

    // ADV23R3-03: comment syntax of any language, recognised generically.
    #[test]
    fn adv23r3_03_any_comment_syntax_is_detected() {
        let p = patterns();
        let footer = footer();
        let trailer = trailer();
        for line in [
            format!("<# {footer} #>"),
            format!("--[[ {footer} ]]"),
            format!("@ {footer}"),
            format!("mov r0, r1 @ {footer}"),
            format!("$x = 1 <# {footer} #>"),
            format!("x = 1 --[[{footer}]]"),
            format!("{{# {footer} #}}"),
            format!("<%-- {footer} --%>"),
            format!("{{{{!-- {footer} --}}}}"),
            format!("!! {footer}"),
            format!("\\ {footer}"),
            format!("(: {footer} :)"),
            format!("<# {trailer} #>"),
            format!("@rem {footer}"),
        ] {
            assert!(p.matches_line(&line), "missed: {line}");
        }
        for line in [
            "@ generated with care",
            "<# Generated with the build script #>",
            "x = 1 -- written by hand",
        ] {
            assert!(!p.matches_line(line), "false positive: {line}");
        }
    }

    // ADV23R3-04: vendor domains, including Google and subdomains.
    #[test]
    fn adv23r3_04_vendor_domains_are_detected() {
        let p = patterns();
        for identity in [
            j(&["Assistant <gem", "ini@google.com>"]),
            j(&["Assistant <assistant@", "google.com>"]),
            j(&["Helper <cop", "ilot@microsoft.com>"]),
            j(&["Assistant <agent@", "deepmind.com>"]),
            j(&["Helper <x@mail.", "anthropic.com>"]),
            j(&["Helper <x@", "x.ai>"]),
            j(&["Helper <x@", "cognition.ai>"]),
        ] {
            assert!(p.matches_identity(&identity), "missed: {identity}");
            assert!(
                p.matches_line(&format!("{}{identity}", j(&["Co-Auth", "ored-By: "]))),
                "missed trailer: {identity}"
            );
        }
        // Large employers' domains need a model or agent in the local part,
        // so their human staff can still be credited.
        for identity in [
            "Jane Doe <jane@example.org>",
            "Jane Doe <jane@google.com>",
            "Jane Doe <jane@notgoogle.com.example.org>",
        ] {
            assert!(!p.matches_identity(identity), "false positive: {identity}");
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

    // Footer and email-less trailer forms of a name, assembled at run time.
    fn forms(name: &str) -> Vec<String> {
        vec![
            j(&["Gener", "ated with ", name]),
            j(&["Co-Auth", "ored-", "By: ", name]),
            j(&["Gener", "ated with [", name, "](https://example.com)"]),
        ]
    }

    fn all_match(p: &Patterns, names: &[&[&str]]) {
        for parts in names {
            let name = parts.concat();
            for line in forms(&name) {
                assert!(p.matches_line(&line), "missed: {line}");
            }
        }
    }

    // ADV23R4-04: a comment delimiter glued to code, with no white space.
    #[test]
    fn adv23r4_04_glued_delimiters_are_detected() {
        let p = patterns();
        let footer = footer();
        for line in [
            format!("$x = 1<# {footer} #>"),
            format!("mov r0, r1@{footer}"),
            format!("x = 1--[[{footer}]]"),
            format!("call();//{footer}"),
            format!("value=1;{footer}"),
        ] {
            assert!(p.matches_line(&line), "missed: {line}");
        }
        for line in [
            "$x = 1<# Generated with the build script #>",
            "mov r0, r1@ generated with care",
        ] {
            assert!(!p.matches_line(line), "false positive: {line}");
        }
    }

    // ADV23R4-05: an `AI` suffix after a model or tool name.
    #[test]
    fn adv23r4_05_ai_suffix_is_detected() {
        all_match(
            &patterns(),
            &[
                &["Cla", "ude AI"],
                &["Gem", "ini AI"],
                &["Deep", "Seek AI"],
                &["Ai", "der AI"],
            ],
        );
    }

    // ADV23R4-06: `R`-prefixed and other letter-prefixed versions.
    #[test]
    fn adv23r4_06_reasoning_model_versions_are_detected() {
        all_match(
            &patterns(),
            &[
                &["Deep", "Seek-R1"],
                &["Deep", "Seek R1 Distill"],
                &["Qw", "Q-32B"],
                &["o", "3-mini-high"],
            ],
        );
    }

    // ADV23R4-07: agent-mode qualifiers.
    #[test]
    fn adv23r4_07_agent_qualifier_is_detected() {
        all_match(
            &patterns(),
            &[
                &["Cur", "sor Agent"],
                &["Cur", "sor Background Agent"],
                &["Cla", "ude Agent SDK"],
            ],
        );
    }

    // ADV23R4-08: Antigravity by name and by Google address.
    #[test]
    fn adv23r4_08_antigravity_is_detected() {
        let p = patterns();
        all_match(&p, &[&["Anti", "gravity"], &["Google Anti", "gravity"]]);
        let identity = j(&["Anti", "gravity <anti", "gravity@google.com>"]);
        assert!(p.matches_identity(&identity), "missed: {identity}");
        assert!(p.matches_line(&j(&["Co-Auth", "ored-", "By: ", &identity])));
    }

    // ADV23R4-09: product tiers after a name, any number of words.
    #[test]
    fn adv23r4_09_tiers_are_detected() {
        all_match(
            &patterns(),
            &[
                &["Gem", "ini Advanced"],
                &["Gem", "ini 2.0 Flash Thinking"],
                &["Gem", "ini Experimental 1206"],
                &["Gem", "ini 2.5 Pro Deep Think"],
            ],
        );
    }

    // ADV23R4-10: Copilot surfaces.
    #[test]
    fn adv23r4_10_copilot_surfaces_are_detected() {
        all_match(
            &patterns(),
            &[
                &["Cop", "ilot Workspace"],
                &["Cop", "ilot Chat"],
                &["GitHub Cop", "ilot Coding Agent"],
            ],
        );
    }

    // ADV23R4-05..10 general mechanism: names live once in the policy file as
    // fragments; a human co-author with a personal address still passes,
    // while a model qualifier with any address is caught.
    #[test]
    fn adv23r4_names_are_defined_once_and_humans_pass() {
        let text = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join(PATTERNS),
        )
        .expect("pattern file");
        let model_lists = text
            .lines()
            .filter(|line| line.contains(&j(&["anti", "gravity|"])))
            .count();
        assert_eq!(
            model_lists, 1,
            "the model list must be defined exactly once"
        );
        let p = patterns();
        for identity in [
            j(&["Cla", "ude AI <x@example.org>"]),
            j(&["Cur", "sor Agent <bot@example.org>"]),
            j(&["Deep", "Seek-R1 <x@example.org>"]),
        ] {
            assert!(p.matches_identity(&identity), "missed: {identity}");
        }
        for identity in [
            "Claude Shannon <cshannon@example.com>",
            "Claude Monet <monet@example.fr>",
            "Devin Smith <devin@example.org>",
            "Jules Verne <jules@example.org>",
        ] {
            assert!(!p.matches_identity(identity), "false positive: {identity}");
        }
    }

    // ADV23R5-06/07: any attribution verb and joining word, with or without
    // a "the help/assistance of" phrase, before a known name.
    #[test]
    fn adv23r5_06_07_attribution_phrases_are_detected() {
        let p = patterns();
        let name = j(&["Cla", "ude"]);
        for verb in [
            "Generated",
            "Produced",
            "Written",
            "Created",
            "Made",
            "Built",
            "Developed",
        ] {
            for joiner in [
                "with",
                "by",
                "via",
                "using",
                "with help from",
                "with the help of",
                "with the assistance of",
                "with the aid of",
                "using the support of",
            ] {
                let line = format!("{verb} {joiner} {name} Code");
                assert!(p.matches_line(&line), "missed: {line}");
            }
        }
        for line in [
            "Generated via the build script",
            "Produced by the release pipeline",
            "Written with help from the team",
            "Made with love",
        ] {
            assert!(!p.matches_line(line), "false positive: {line}");
        }
    }

    // ADV23R5-08/09: newly listed tools and models, in every form.
    #[test]
    fn adv23r5_08_09_listed_tools_are_detected() {
        let p = patterns();
        all_match(
            &p,
            &[
                &["Sourcegraph Co", "dy"],
                &["Co", "dy"],
                &["Amazon Code", "Whisperer"],
                &["Amazon ", "Q Developer"],
                &["GL", "M-4"],
                &["Chat", "GLM"],
                &["Zhipu GL", "M-4"],
                &["Command ", "R+"],
                &["Cohere Command ", "R"],
            ],
        );
        let identity = j(&["Co", "dy <co", "dy@sourcegraph.com>"]);
        assert!(p.matches_identity(&identity), "missed: {identity}");
        assert!(p.matches_line(&j(&["Co-Auth", "ored-", "By: ", &identity])));
        for line in [
            "Generated with command run.sh",
            "Co-authored-by: Cody Smith <cody@example.org>",
        ] {
            assert!(!p.matches_line(line), "false positive: {line}");
        }
    }

    // ADV23R5-10: any `<Words> by:` or `<Words> with:` trailer naming a known
    // name, not only listed verbs.
    #[test]
    fn adv23r5_10_any_words_by_trailer_is_detected() {
        let p = patterns();
        let name = j(&["Cla", "ude"]);
        for words in [
            "Prompted by",
            "Implemented by",
            "Prepared by",
            "Pair programmed with",
            "Reviewed and tested by",
        ] {
            let line = format!("{words}: {name}");
            assert!(p.matches_line(&line), "missed: {line}");
        }
        for line in [
            "Prompted by: the outage report",
            "Reviewed by: Claude Monet <monet@example.fr>",
            "Fixes issue raised by: Jane Doe",
        ] {
            assert!(!p.matches_line(line), "false positive: {line}");
        }
    }

    // ADV23R6-05..10: generic assistants, vendor-only names, collaboration
    // phrasing, possessives, `from:` trailers and newly listed tools.
    #[test]
    fn adv23r6_05_10_generic_attribution_is_detected() {
        let p = patterns();
        let claude = j(&["Cla", "ude"]);
        let trailer = j(&["Co-Auth", "ored-By: "]);
        let mut missed = Vec::new();
        let lines = [
            // ADV23R6-05: generic identities in trailers.
            format!("{trailer}AI"),
            format!("{trailer}LLM"),
            format!("{trailer}AI Assistant"),
            format!("{trailer}LLM Bot"),
            format!("{trailer}an AI coding assistant"),
            j(&["Generated by an ", "LLM"]),
            j(&["Written with the help of a ", "language model"]),
            // ADV23R6-06: vendor-only AI products.
            j(&["Generated with Goo", "gle AI"]),
            j(&["Generated with Me", "ta AI"]),
            j(&["Generated with Apple ", "Intelligence"]),
            j(&["Built with Microsoft ", "AI"]),
            format!("{trailer}{}", j(&["Goo", "gle AI"])),
            // ADV23R6-07: collaboration verbs and joiners.
            format!("Pair-programmed with {claude}"),
            format!("Pair programming with {claude}"),
            format!("Written in collaboration with {claude}"),
            format!("Developed in partnership with {claude}"),
            format!("Generated thanks to {claude}"),
            format!("Co-written with {claude}"),
            format!("Collaboration with {claude}"),
            format!("Built together with {claude}"),
            // ADV23R6-08: possessive help.
            format!("Written with {claude}'s help"),
            format!("Built with {claude}\u{2019}s assistance."),
            j(&["Generated with Chat", "GPT's help"]),
            j(&["Made with the ", "AI's help"]),
            // ADV23R6-09: `from:` trailers.
            format!("Help from: {claude}"),
            format!("Assistance from: {claude}"),
            format!("Code from: {claude}"),
            j(&["Help from: ", "AI"]),
            // ADV23R6-10: Roo Code, Continue, Supermaven.
            j(&["Generated with Roo ", "Code"]),
            j(&["Generated with Roo-", "Cline"]),
            j(&["Generated with Con", "tinue"]),
            j(&["Generated with Con", "tinue.dev"]),
            j(&["Generated with Super", "maven"]),
            format!("{trailer}{}", j(&["Super", "maven"])),
        ];
        for line in &lines {
            if !p.matches_line(line) {
                missed.push(line.clone());
            }
        }
        assert!(missed.is_empty(), "missed: {missed:#?}");
        // Human credit and ordinary prose stay allowed.
        for line in [
            "Co-authored-by: Ai Nakamura <ai@example.com>",
            "Co-authored-by: Claude Monet <monet@example.fr>",
            "Reviewed by: Claude Monet <monet@example.fr>",
            "Help from: Jane Doe",
            "Ported from: the upstream parser",
            "Built with Google Cloud Build",
            "Generated with intelligence and care",
            "Written in collaboration with the grid operator",
            "Made with the team's help",
            "Generated with continued fractions",
            "Built with Rooms service",
            "Made with maven",
            "The AI module was rewritten.",
            "Written with care",
        ] {
            assert!(!p.matches_line(line), "false positive: {line}");
        }
    }

    // Fragment references fail closed: an undefined or unterminated name is
    // a load error, not a silently empty rule.
    #[test]
    fn fragment_references_fail_closed() {
        assert!(Patterns::load(b"define A = x\n^{{A}}y$\n").is_ok());
        assert!(Patterns::load(b"^{{MISSING}}y$\n").is_err());
        assert!(Patterns::load(b"^{{A y$\n").is_err());
        assert!(Patterns::load(b"define lower = x\n^x$\n").is_err());
        assert!(Patterns::load(b"define A x\n^x$\n").is_err());
    }
}
