//! Secret scanning for the `provenance/secrets/data` gate (D-56, ADR 0001).
//!
//! Secret formats are not maintained here (ADV23R12-01/02): they come from
//! the gitleaks default ruleset, vendored unchanged and pinned in trusted
//! policy ([`LOCAL_RULES`] names the copy, its version and its blob id),
//! plus a few local rules in the same schema. This module evaluates that
//! schema in Rust: `regex`, `secretGroup`, `entropy`, `keywords`, `path` and
//! per-rule `allowlists` (`regexes` with `regexTarget`, `stopwords`, `paths`,
//! `condition`). The global allowlist's `regexes` and `stopwords` apply; its
//! `paths` never match, so every tracked file is scanned. A key this module
//! does not understand, or a regex that does not compile, is an error: the
//! check fails closed rather than silently skipping a rule.
//!
//! Every file is scanned in every plausible text view (R12-01): its raw
//! bytes, UTF-8 with invalid sequences replaced, and UTF-16 and UTF-32 in
//! both byte orders at every alignment, ignoring any byte-order mark. A
//! finding in any view counts, so choosing an encoding can never hide
//! content that another reading shows.

use std::collections::{BTreeMap, BTreeSet};

use regex::bytes::{Regex, RegexBuilder};
use toml_edit::{DocumentMut, Item, TableLike};

use crate::util::Ctx;

/// Trusted local rules and the pin of the vendored gitleaks ruleset.
pub const LOCAL_RULES: &str = "xtask/policy/secret-rules.toml";

/// What an allowlist regex is matched against.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    Secret,
    Match,
    Line,
}

#[derive(Debug)]
struct Allowlist {
    and: bool,
    target: Target,
    regexes: Vec<Regex>,
    stopwords: Vec<String>,
    paths: Vec<Regex>,
    /// Global allowlist: its paths never match (every file is scanned).
    ignore_paths: bool,
    /// Rule ids a global allowlist is limited to (`targetRules`); empty
    /// means every rule.
    target_rules: Vec<String>,
}

impl Allowlist {
    fn applies_to(&self, id: &str) -> bool {
        self.target_rules.is_empty() || self.target_rules.iter().any(|rule| rule == id)
    }

    fn allows(&self, path: &str, secret: &[u8], matched: &[u8], line: &[u8]) -> bool {
        let target = match self.target {
            Target::Secret => secret,
            Target::Match => matched,
            Target::Line => line,
        };
        let lower = String::from_utf8_lossy(secret).to_lowercase();
        let mut checks = Vec::new();
        if !self.paths.is_empty() {
            checks
                .push(!self.ignore_paths && self.paths.iter().any(|r| r.is_match(path.as_bytes())));
        }
        if !self.regexes.is_empty() {
            checks.push(self.regexes.iter().any(|r| r.is_match(target)));
        }
        if !self.stopwords.is_empty() {
            checks.push(self.stopwords.iter().any(|w| lower.contains(w.as_str())));
        }
        if self.and {
            !checks.is_empty() && checks.iter().all(|c| *c)
        } else {
            checks.iter().any(|c| *c)
        }
    }
}

#[derive(Debug)]
struct Rule {
    id: String,
    regex: Option<Regex>,
    path: Option<Regex>,
    secret_group: usize,
    entropy: f64,
    keywords: Vec<Vec<u8>>,
    allowlists: Vec<Allowlist>,
}

/// A compiled secret ruleset.
#[derive(Debug)]
pub struct Ruleset {
    rules: Vec<Rule>,
    global: Vec<Allowlist>,
    /// The vendored ruleset's path and pinned blob id, when loaded from policy.
    pinned: Option<(String, String)>,
}

/// One secret finding: the line of the first view that showed it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hit {
    /// One-based line in that view, or `None` for a path rule.
    pub line: Option<usize>,
    /// That line's text (for exact-line exemptions).
    pub line_text: String,
    /// Description naming the rules and views.
    pub message: String,
}

/// Per line: the first view's line text, the rule ids and the view names.
type LineHits = (String, BTreeSet<String>, BTreeSet<&'static str>);

/// Translates Go RE2 syntax the Rust `regex` crate reads differently: a `{`
/// that does not start a counted repetition is a literal in Go.
#[must_use]
pub fn translate(pattern: &str) -> String {
    let chars: Vec<char> = pattern.chars().collect();
    let mut out = String::with_capacity(pattern.len());
    let mut in_class = false;
    let mut index = 0;
    while index < chars.len() {
        let c = chars[index];
        if c == '\\' {
            out.push(c);
            if let Some(next) = chars.get(index + 1) {
                out.push(*next);
            }
            index += 2;
            continue;
        }
        if in_class {
            if c == ']' {
                in_class = false;
            }
            out.push(c);
        } else if c == '[' {
            in_class = true;
            out.push(c);
            // A `]` (after an optional `^`) first in a class is a literal.
            for _ in 0..2 {
                match chars.get(index + 1) {
                    Some('^') if out.ends_with('[') => {
                        out.push('^');
                        index += 1;
                    }
                    Some(']') => {
                        out.push(']');
                        index += 1;
                        break;
                    }
                    _ => break,
                }
            }
        } else if c == '{' && !counted(&chars[index..]) {
            out.push_str("\\{");
        } else if c == '}' && !closes_count(&out) {
            out.push_str("\\}");
        } else {
            out.push(c);
        }
        index += 1;
    }
    out
}

/// Whether `rest` (starting at `{`) is `{n}`, `{n,}` or `{n,m}`.
fn counted(rest: &[char]) -> bool {
    let Some(end) = rest.iter().position(|c| *c == '}') else {
        return false;
    };
    let body: String = rest[1..end].iter().collect();
    let mut parts = body.splitn(2, ',');
    let first = parts.next().unwrap_or_default();
    let digits = |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_digit());
    digits(first)
        && parts
            .next()
            .is_none_or(|second| second.is_empty() || digits(second))
}

/// Whether the translated text so far ends inside an open counted repetition.
fn closes_count(out: &str) -> bool {
    let Some(open) = out.rfind('{') else {
        return false;
    };
    if out[..open].ends_with('\\') {
        return false;
    }
    let body = &out[open + 1..];
    !body.is_empty() && body.chars().all(|c| c.is_ascii_digit() || c == ',')
}

fn compile(pattern: &str, place: &str) -> Result<Regex, String> {
    RegexBuilder::new(&translate(pattern))
        .unicode(false)
        .size_limit(64 << 20)
        .build()
        .map_err(|error| format!("{place}: regex does not compile, failing closed: {error}"))
}

fn check_keys(table: &dyn TableLike, allowed: &[&str], place: &str) -> Result<(), String> {
    match table.iter().find(|(key, _)| !allowed.contains(key)) {
        Some((key, _)) => Err(format!(
            "{place}: unsupported key `{key}`; xtask cannot evaluate it, failing closed"
        )),
        None => Ok(()),
    }
}

fn strings(table: &dyn TableLike, key: &str, place: &str) -> Result<Vec<String>, String> {
    let Some(item) = table.get(key) else {
        return Ok(Vec::new());
    };
    item.as_array()
        .and_then(|array| {
            array
                .iter()
                .map(|value| value.as_str().map(str::to_owned))
                .collect::<Option<Vec<_>>>()
        })
        .ok_or_else(|| format!("{place}: `{key}` must be an array of strings"))
}

fn text<'a>(table: &'a dyn TableLike, key: &str, place: &str) -> Result<Option<&'a str>, String> {
    table.get(key).map_or(Ok(None), |item| {
        item.as_str()
            .map(Some)
            .ok_or_else(|| format!("{place}: `{key}` must be a string"))
    })
}

fn allowlist(table: &dyn TableLike, global: bool, place: &str) -> Result<Allowlist, String> {
    check_keys(
        table,
        &[
            "description",
            "condition",
            "regexTarget",
            "regexes",
            "stopwords",
            "paths",
            "commits",
            "targetRules",
        ],
        place,
    )?;
    let target_rules = strings(table, "targetRules", place)?;
    if !global && !target_rules.is_empty() {
        return Err(format!(
            "{place}: targetRules is only valid in a global allowlist"
        ));
    }
    if !strings(table, "commits", place)?.is_empty() {
        return Err(format!(
            "{place}: commit allowlists are not supported, failing closed"
        ));
    }
    let and = match text(table, "condition", place)?.map(str::to_ascii_uppercase) {
        None => false,
        Some(condition) if condition == "OR" => false,
        Some(condition) if condition == "AND" => true,
        Some(other) => return Err(format!("{place}: unknown condition `{other}`")),
    };
    let target = match text(table, "regexTarget", place)? {
        None | Some("secret") => Target::Secret,
        Some("match") => Target::Match,
        Some("line") => Target::Line,
        Some(other) => return Err(format!("{place}: unknown regexTarget `{other}`")),
    };
    let compile_all = |key: &str| -> Result<Vec<Regex>, String> {
        strings(table, key, place)?
            .iter()
            .map(|pattern| compile(pattern, &format!("{place}.{key}")))
            .collect()
    };
    Ok(Allowlist {
        and,
        target,
        regexes: compile_all("regexes")?,
        stopwords: strings(table, "stopwords", place)?
            .into_iter()
            .map(|word| word.to_lowercase())
            .collect(),
        paths: compile_all("paths")?,
        ignore_paths: global,
        target_rules,
    })
}

fn tables<'a>(item: Option<&'a Item>, place: &str) -> Result<Vec<&'a dyn TableLike>, String> {
    match item {
        None => Ok(Vec::new()),
        Some(Item::ArrayOfTables(array)) => Ok(array.iter().map(|t| t as &dyn TableLike).collect()),
        Some(item) => item
            .as_table_like()
            .map(|table| vec![table])
            .ok_or_else(|| format!("{place} must be a table or array of tables")),
    }
}

fn rule(table: &dyn TableLike, place: &str) -> Result<Rule, String> {
    check_keys(
        table,
        &[
            "id",
            "description",
            "regex",
            "secretGroup",
            "entropy",
            "keywords",
            "path",
            "tags",
            "allowlists",
            "allowlist",
        ],
        place,
    )?;
    let id = text(table, "id", place)?
        .filter(|id| !id.trim().is_empty())
        .ok_or_else(|| format!("{place}: rule has no id"))?
        .to_owned();
    let place = format!("{place} `{id}`");
    let regex = text(table, "regex", &place)?
        .map(|pattern| compile(pattern, &place))
        .transpose()?;
    let path = text(table, "path", &place)?
        .map(|pattern| compile(pattern, &format!("{place}.path")))
        .transpose()?;
    if regex.is_none() && path.is_none() {
        return Err(format!("{place}: rule has neither regex nor path"));
    }
    let secret_group = match table.get("secretGroup") {
        None => 0,
        Some(item) => item
            .as_integer()
            .and_then(|n| usize::try_from(n).ok())
            .ok_or_else(|| format!("{place}: secretGroup must be a non-negative integer"))?,
    };
    if let Some(regex) = &regex
        && secret_group >= regex.captures_len()
    {
        return Err(format!(
            "{place}: secretGroup {secret_group} does not exist"
        ));
    }
    let entropy = match table.get("entropy") {
        None => 0.0,
        Some(item) => item
            .as_float()
            .or_else(|| item.as_integer().map(|n| n as f64))
            .ok_or_else(|| format!("{place}: entropy must be a number"))?,
    };
    let mut allowlists = Vec::new();
    for key in ["allowlists", "allowlist"] {
        for table in tables(table.get(key), &format!("{place}.{key}"))? {
            allowlists.push(allowlist(table, false, &format!("{place}.{key}"))?);
        }
    }
    Ok(Rule {
        id,
        regex,
        path,
        secret_group,
        entropy,
        keywords: strings(table, "keywords", &place)?
            .into_iter()
            .map(|word| word.to_ascii_lowercase().into_bytes())
            .collect(),
        allowlists,
    })
}

impl Ruleset {
    /// Compiles rulesets in the gitleaks schema, given as `(name, text)`.
    ///
    /// # Errors
    ///
    /// Fails closed on invalid TOML, an unsupported key, a duplicate rule id
    /// or a regex that does not compile.
    pub fn parse(sources: &[(&str, &str)]) -> Result<Self, String> {
        let mut rules = Vec::new();
        let mut global = Vec::new();
        let mut ids = BTreeSet::new();
        for (name, source) in sources {
            let document: DocumentMut =
                source.parse().map_err(|error| format!("{name}: {error}"))?;
            check_keys(
                document.as_table(),
                &[
                    "title",
                    "description",
                    "minVersion",
                    "vendor",
                    "allowlist",
                    "allowlists",
                    "rules",
                ],
                name,
            )?;
            for key in ["allowlist", "allowlists"] {
                for table in tables(document.get(key), &format!("{name}: {key}"))? {
                    global.push(allowlist(table, true, &format!("{name}: {key}"))?);
                }
            }
            for table in tables(document.get("rules"), &format!("{name}: rules"))? {
                let rule = rule(table, &format!("{name}: rule"))?;
                if !ids.insert(rule.id.clone()) {
                    return Err(format!("{name}: duplicate rule id `{}`", rule.id));
                }
                rules.push(rule);
            }
        }
        if rules.is_empty() {
            return Err("secret ruleset has no rules; the gate would be vacuous".to_owned());
        }
        for id in global.iter().flat_map(|list| &list.target_rules) {
            if !ids.contains(id) {
                return Err(format!("allowlist targetRules names unknown rule `{id}`"));
            }
        }
        Ok(Self {
            rules,
            global,
            pinned: None,
        })
    }

    /// Loads the trusted local rules and the pinned vendored gitleaks ruleset.
    ///
    /// # Errors
    ///
    /// Fails closed if a file is missing, the vendored copy's blob id differs
    /// from the pin, or [`Ruleset::parse`] fails.
    pub fn load(ctx: &Ctx) -> Result<Self, String> {
        let local = ctx.policy_text(LOCAL_RULES)?;
        let document: DocumentMut = local
            .parse()
            .map_err(|error| format!("{LOCAL_RULES}: {error}"))?;
        let vendor = document
            .get("vendor")
            .and_then(Item::as_table_like)
            .ok_or_else(|| format!("{LOCAL_RULES}: [vendor] is missing"))?;
        let field = |key: &str| {
            vendor
                .get(key)
                .and_then(Item::as_str)
                .filter(|value| !value.trim().is_empty())
                .ok_or_else(|| format!("{LOCAL_RULES}: vendor.{key} is missing"))
        };
        for key in ["licence", "version", "commit", "source"] {
            field(key)?;
        }
        ctx.policy_text(field("licence_file")?)?;
        let file = field("file")?;
        let vendored = ctx.policy_text(file)?;
        let blob = crate::repository::line_blob(&ctx.policy, &vendored);
        if blob != field("blob")? {
            return Err(format!(
                "{file}: blob {blob} differs from the pinned vendor.blob in {LOCAL_RULES}; follow the CONTRIBUTING update procedure"
            ));
        }
        let mut rules = Self::parse(&[(file, &vendored), (LOCAL_RULES, &local)])?;
        rules.pinned = Some((file.to_owned(), blob));
        Ok(rules)
    }

    /// Whether a tracked file is the pinned vendored ruleset, byte for byte:
    /// it quotes its own patterns and upstream's allowlisted example keys,
    /// so only that exact copy is not scanned. Any edit makes it scanned.
    #[must_use]
    pub fn is_pinned_copy(&self, path: &str, blob: impl FnOnce() -> String) -> bool {
        self.pinned
            .as_ref()
            .is_some_and(|(file, pinned)| file == path && blob() == *pinned)
    }

    /// Secret findings in one file, over every text view of its bytes,
    /// one per line (the first view that showed it names the line).
    #[must_use]
    pub fn scan(&self, path: &str, bytes: &[u8]) -> Vec<Hit> {
        let mut found: BTreeMap<Option<usize>, LineHits> = BTreeMap::new();
        for rule in &self.rules {
            if rule.regex.is_none()
                && rule
                    .path
                    .as_ref()
                    .is_some_and(|r| r.is_match(path.as_bytes()))
            {
                found.entry(None).or_default().1.insert(rule.id.clone());
            }
        }
        for (view, text) in views(bytes) {
            for (line, line_text, id) in self.scan_view(path, &text) {
                let entry = found
                    .entry(Some(line))
                    .or_insert_with(|| (line_text, BTreeSet::new(), BTreeSet::new()));
                entry.1.insert(id);
                entry.2.insert(view);
            }
        }
        found
            .into_iter()
            .map(|(line, (line_text, ids, views))| {
                let ids: Vec<String> = ids.into_iter().collect();
                let views: Vec<&str> = views.into_iter().collect();
                let message = if line.is_none() {
                    format!(
                        "private key or credential file must not be committed ({})",
                        ids.join(", ")
                    )
                } else {
                    format!(
                        "possible secret committed: {} ({} view)",
                        ids.join(", "),
                        views.join(", ")
                    )
                };
                Hit {
                    line,
                    line_text,
                    message,
                }
            })
            .collect()
    }

    fn scan_view(&self, path: &str, text: &[u8]) -> Vec<(usize, String, String)> {
        let lower = text.to_ascii_lowercase();
        let newlines: Vec<usize> = memchr::memchr_iter(b'\n', text).collect();
        let mut hits = Vec::new();
        for rule in &self.rules {
            let Some(regex) = &rule.regex else { continue };
            if rule
                .path
                .as_ref()
                .is_some_and(|r| !r.is_match(path.as_bytes()))
            {
                continue;
            }
            if !rule.keywords.is_empty()
                && !rule
                    .keywords
                    .iter()
                    .any(|word| memchr::memmem::find(&lower, word).is_some())
            {
                continue;
            }
            for captures in regex.captures_iter(text) {
                let Some(whole) = captures.get(0) else {
                    continue;
                };
                let secret = if rule.secret_group > 0 {
                    captures
                        .get(rule.secret_group)
                        .map_or(&[][..], |m| m.as_bytes())
                } else {
                    captures
                        .iter()
                        .skip(1)
                        .flatten()
                        .map(|m| m.as_bytes())
                        .find(|s| !s.is_empty())
                        .unwrap_or(whole.as_bytes())
                };
                if rule.entropy > 0.0 && entropy(secret) <= rule.entropy {
                    continue;
                }
                let line = newlines.partition_point(|n| *n < whole.start());
                let start = if line == 0 { 0 } else { newlines[line - 1] + 1 };
                let end_line = newlines.partition_point(|n| *n < whole.end());
                let end = newlines.get(end_line).copied().unwrap_or(text.len());
                let lines = &text[start..end.max(start)];
                if self
                    .global
                    .iter()
                    .filter(|list| list.applies_to(&rule.id))
                    .chain(&rule.allowlists)
                    .any(|list| list.allows(path, secret, whole.as_bytes(), lines))
                {
                    continue;
                }
                let first_end = newlines.get(line).copied().unwrap_or(text.len());
                let first = String::from_utf8_lossy(&text[start..first_end]);
                let first = first.strip_suffix('\r').unwrap_or(&first).to_owned();
                hits.push((line + 1, first, rule.id.clone()));
            }
        }
        hits
    }
}

/// Shannon entropy as gitleaks computes it: over characters, divided by the
/// byte length.
#[must_use]
pub fn entropy(secret: &[u8]) -> f64 {
    if secret.is_empty() {
        return 0.0;
    }
    let text = String::from_utf8_lossy(secret);
    let mut counts: BTreeMap<char, usize> = BTreeMap::new();
    for c in text.chars() {
        *counts.entry(c).or_default() += 1;
    }
    #[allow(clippy::cast_precision_loss)]
    let length = secret.len() as f64;
    counts
        .values()
        .map(|count| {
            #[allow(clippy::cast_precision_loss)]
            let frequency = *count as f64 / length;
            -frequency * frequency.log2()
        })
        .sum()
}

/// Every text view of a file (R12-01): raw bytes, lossy UTF-8 when the bytes
/// are not valid UTF-8, and lossy UTF-16/UTF-32 in both byte orders from
/// every alignment. BOMs are not interpreted: each view decodes all bytes.
#[must_use]
pub fn views(bytes: &[u8]) -> Vec<(&'static str, Vec<u8>)> {
    let mut out = vec![("raw", bytes.to_vec())];
    if std::str::from_utf8(bytes).is_err() {
        out.push((
            "utf-8",
            String::from_utf8_lossy(bytes).into_owned().into_bytes(),
        ));
    }
    let names = [
        ("utf-16le", 2, false, 0),
        ("utf-16le+1", 2, false, 1),
        ("utf-16be", 2, true, 0),
        ("utf-16be+1", 2, true, 1),
        ("utf-32le", 4, false, 0),
        ("utf-32le+1", 4, false, 1),
        ("utf-32le+2", 4, false, 2),
        ("utf-32le+3", 4, false, 3),
        ("utf-32be", 4, true, 0),
        ("utf-32be+1", 4, true, 1),
        ("utf-32be+2", 4, true, 2),
        ("utf-32be+3", 4, true, 3),
    ];
    for (name, width, big, offset) in names {
        let Some(body) = bytes.get(offset..) else {
            continue;
        };
        if body.len() < width {
            continue;
        }
        let units = body.chunks_exact(width).map(|chunk| {
            let fold = |acc: u32, b: &u8| (acc << 8) | u32::from(*b);
            if big {
                chunk.iter().fold(0, fold)
            } else {
                chunk.iter().rev().fold(0, fold)
            }
        });
        let text: String = if width == 2 {
            char::decode_utf16(units.map(|u| u16::try_from(u).unwrap_or(0xfffd)))
                .map(|c| c.unwrap_or('\u{fffd}'))
                .collect()
        } else {
            units
                .map(|u| char::from_u32(u).unwrap_or('\u{fffd}'))
                .collect()
        };
        out.push((name, text.into_bytes()));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rules() -> &'static Ruleset {
        static RULES: std::sync::OnceLock<Ruleset> = std::sync::OnceLock::new();
        RULES.get_or_init(|| {
            Ruleset::parse(&[
                ("gitleaks.toml", include_str!("../policy/gitleaks.toml")),
                (
                    "secret-rules.toml",
                    include_str!("../policy/secret-rules.toml"),
                ),
            ])
            .expect("ruleset")
        })
    }

    fn ids(path: &str, bytes: &[u8]) -> Vec<String> {
        rules()
            .scan(path, bytes)
            .into_iter()
            .map(|hit| hit.message)
            .collect()
    }

    // R12-01: at d996aac a two-byte prefix `41 00` made the decoder pick
    // UTF-16LE for the whole file, and only that view was scanned, so a raw
    // ASCII token passed. Every view is now scanned and any finding counts.
    #[test]
    fn r12_01_raw_view_is_scanned_whatever_the_encoding_guess() {
        let token = ["gh", "p_", &"0".repeat(36)].concat();
        let line = format!("service_token={token}\n");
        for prefix in [
            &b"A\x00"[..],
            b"\x00A",
            b"A\x00\x00\x00",
            b"\x00\x00\x00A",
            b"\xff\xfe",
            b"\xfe\xff",
            b"\xff\xfe\x00\x00",
            b"\x00\x00\xfe\xff",
            b"\xef\xbb\xbf",
            b"\x80\x81",
            b"",
        ] {
            let bytes = [prefix, line.as_bytes(), b" "].concat();
            let found = ids("docs/connection-notes.txt", &bytes);
            assert_eq!(found.len(), 1, "{prefix:?}: {found:?}");
            assert!(found[0].contains("local-github-token"), "{found:?}");
        }
        // Encoded text in every encoding, odd alignment and a mixed file.
        let utf16: Vec<u8> = line.encode_utf16().flat_map(u16::to_le_bytes).collect();
        let utf32: Vec<u8> = line
            .chars()
            .flat_map(|c| u32::from(c).to_be_bytes())
            .collect();
        for bytes in [
            [&b"x"[..], &utf16].concat(),
            [&b"abc"[..], &utf32].concat(),
            [&b"plain ascii\n"[..], &utf16].concat(),
        ] {
            assert_eq!(ids("f", &bytes).len(), 1, "{bytes:?}");
        }
        // Harmless controls in every view.
        for bytes in [
            &b"A\x00harmless notes\n "[..],
            b"\xff\xfeplain",
            b"grid \xc3\xa9\n",
        ] {
            assert!(ids("f", bytes).is_empty(), "{bytes:?}");
        }
    }

    // ADV23R12-01: a PuTTY private key in an allowed text file passed at
    // d996aac; .ppk files are key files by name.
    #[test]
    fn adv23r12_01_putty_keys_are_detected() {
        let key = concat!(
            "PuTTY-User-Key-File",
            "-3: ssh-ed25519\nEncryption: none\nComment: test-key\n",
            "Public-Lines: 2\nAAAAC3NzaC1lZDI1NTE5AAAAIGx5\nPrivate-Lines: 1\nAAAAYCe\n"
        );
        let found = rules().scan("crates/putty_key.txt", key.as_bytes());
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].line, Some(1));
        assert!(found[0].message.contains("local-putty-private-key"));
        let found = rules().scan("keys/server.PPK", b"anything");
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].line, None);
        assert!(ids("docs/putty.md", b"PuTTY user key files are not allowed.\n").is_empty());
    }

    // ADV23R12-02: GitLab runner and Perplexity tokens passed at d996aac;
    // the vendored gitleaks ruleset covers them.
    #[test]
    fn adv23r12_02_vendored_ruleset_covers_more_formats() {
        for (text, id) in [
            (
                [
                    "gitlab_runner_token=\"gl",
                    "rt-1234567890abcdef1234567890\"\n",
                ]
                .concat(),
                "gitlab-runner-authentication-token",
            ),
            (
                [
                    "perplexity_key=\"pp",
                    "lx-abcdef1234567890abcdef1234567890abcdef1234567890\"\n",
                ]
                .concat(),
                "perplexity-api-key",
            ),
        ] {
            let found = ids("crates/token.rs", text.as_bytes());
            assert_eq!(found.len(), 1, "{found:?}");
            assert!(found[0].contains(id), "{found:?}");
        }
        // Every vendored rule compiled (none skipped) plus the local rules.
        let vendored = include_str!("../policy/gitleaks.toml")
            .lines()
            .filter(|line| *line == "[[rules]]")
            .count();
        let local = include_str!("../policy/secret-rules.toml")
            .lines()
            .filter(|line| *line == "[[rules]]")
            .count();
        assert!(vendored > 200);
        assert_eq!(rules().rules.len(), vendored + local);
        // Git commit ids beside the word Sourcegraph are not tokens; its
        // prefixed form still is.
        let sha = "0123456789abcdef0123456789abcdef01234567";
        assert!(ids("f", format!("Sourcegraph review of {sha}\n").as_bytes()).is_empty());
        let token = ["sg", "p_", "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"].concat();
        assert_eq!(
            ids("f", format!("sourcegraph token {token}\n").as_bytes()).len(),
            1
        );
    }

    #[test]
    fn go_braces_are_translated() {
        assert_eq!(translate(r"^\$(?:\d+|{\d+})$"), r"^\$(?:\d+|\{\d+\})$");
        assert_eq!(translate(r"a{2,3}b{4}c{5,}"), r"a{2,3}b{4}c{5,}");
        assert_eq!(translate(r"[{}]x{1}"), r"[{}]x{1}");
        assert_eq!(translate(r"[]{]"), r"[]{]");
        assert_eq!(translate(r"\{\{x}}"), r"\{\{x\}\}");
    }

    #[test]
    fn entropy_matches_gitleaks() {
        assert!(entropy(b"aaaa").abs() < f64::EPSILON);
        assert!((entropy(b"ab") - 1.0).abs() < 1e-9);
    }

    #[test]
    fn unsupported_keys_and_bad_regexes_fail_closed() {
        let ok = "[[rules]]\nid = \"x\"\nregex = '''x'''\n";
        assert!(Ruleset::parse(&[("t", ok)]).is_ok());
        for bad in [
            "[[rules]]\nid = \"x\"\nregex = '''(x'''\n",
            "[[rules]]\nid = \"x\"\nregex = '''x'''\nnewThing = 1\n",
            "[[rules]]\nid = \"x\"\n",
            "[[rules]]\nid = \"x\"\nregex = '''x'''\nsecretGroup = 2\n",
            "[[rules]]\nid = \"x\"\nregex = '''x'''\n[[rules.required]]\nid = \"y\"\n",
            "[extend]\nuseDefault = true\n[[rules]]\nid = \"x\"\nregex = '''x'''\n",
            "[[rules]]\nid = \"x\"\nregex = '''x'''\n[[rules.allowlists]]\ncommits = [\"abc\"]\n",
            "",
        ] {
            assert!(Ruleset::parse(&[("t", bad)]).is_err(), "{bad}");
        }
        assert!(
            Ruleset::parse(&[("a", ok), ("b", ok)]).is_err(),
            "duplicate id"
        );
    }
}
