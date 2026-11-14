//! Repository content policy.
//!
//! - `python` (D-73): no Python outside `spikes/` and the allow-listed offline
//!   reference-validator directories; no tracked bytecode (ADV23-11).
//! - `provenance` (D-56, ADR 0001 `provenance/secrets/data`, ADV23-04): every
//!   dataset in a provenance manifest is tagged `open-commercial` or
//!   `research-only`; share-alike data is never used; research-only files are
//!   never committed; data files exist only under allow-listed sample paths
//!   AND each one resolves to an explicit open-commercial provenance record
//!   (F9); no secrets, key files or `raw/` directories are tracked.
//! - `codeowners`: under GitHub's CODEOWNERS rules (last matching rule wins,
//!   invalid lines are skipped), every protected policy path is owned by the
//!   policy owner (F8).

use std::collections::{BTreeMap, BTreeSet};

use toml_edit::{DocumentMut, Item};

use crate::util::{Ctx, Finding, git_command, glob_match, is_sha, string_array, tracked};

/// Trusted repository policy.
pub const POLICY: &str = "xtask/policy/repository.toml";

fn policy(ctx: &Ctx) -> Result<DocumentMut, String> {
    ctx.policy_toml(POLICY)
}

/// Whether any component of a repository path equals `name`, ignoring letter
/// case: Git stores `RAW/` and `raw/` as different paths, but case-insensitive
/// file systems and people read them as the same directory (ADV23R6-04).
#[must_use]
pub fn has_component(path: &str, name: &str) -> bool {
    path.split('/')
        .any(|part| part.to_lowercase() == name.to_lowercase())
}

/// Python and bytecode check over a list of tracked paths.
#[must_use]
pub fn check_python(
    paths: &[String],
    allowed: &[String],
    heads: &dyn Fn(&str) -> Option<String>,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    for path in paths {
        let name = path.rsplit('/').next().unwrap_or(path);
        let lower = name.to_lowercase();
        if lower.ends_with(".pyc") || lower.ends_with(".pyo") || has_component(path, "__pycache__")
        {
            findings.push(Finding::at(
                path,
                None,
                "compiled Python bytecode must not be tracked",
            ));
            continue;
        }
        if allowed.iter().any(|dir| glob_match(dir, path)) {
            continue;
        }
        let python_name = [".py", ".pyw", ".pyi", ".ipynb"]
            .iter()
            .any(|ext| lower.ends_with(ext));
        let python_shebang = !name.contains('.')
            && heads(path).is_some_and(|first| first.starts_with("#!") && first.contains("python"));
        if python_name || python_shebang {
            findings.push(Finding::at(
                path,
                None,
                "Python is allowed only in spikes/ and allow-listed reference validators (D-73)",
            ));
        }
    }
    findings
}

/// Runs the Python check.
///
/// # Errors
///
/// Fails if Git or the trusted policy fail.
pub fn python(ctx: &Ctx) -> Result<Vec<Finding>, String> {
    let document = policy(ctx)?;
    let allowed = string_array(&document["python_allowed"]);
    let paths = tracked(&ctx.repo)?;
    let heads = |path: &str| {
        std::fs::read(ctx.repo.join(path)).ok().map(|bytes| {
            let end = bytes
                .iter()
                .position(|b| *b == b'\n')
                .unwrap_or(bytes.len())
                .min(200);
            String::from_utf8_lossy(&bytes[..end]).into_owned()
        })
    };
    Ok(check_python(&paths, &allowed, &heads))
}

const STATUSES: &[&str] = &["open-commercial", "research-only"];
const SHARE_ALIKE: &[&str] = &[
    "odbl",
    "openstreetmap",
    "cc-by-sa",
    "cc by-sa",
    "share-alike",
    "sharealike",
    "share alike",
];

fn mentions_share_alike(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    SHARE_ALIKE.iter().any(|needle| lower.contains(needle))
        || lower
            .split(|c: char| !c.is_ascii_alphanumeric())
            .any(|word| word == "osm")
}

/// Final words of a manifest key that names files (ADV23R-09, ADV23R3-06).
///
/// Manifest schema rule: a key names one or more files when its last
/// `_`/`-`-separated word is one of these (`file`, `sha256_index_file`,
/// `relative_path`, `local_name`, `data_files`, ...), at any depth inside a
/// dataset table. The dataset's own `name` is its title, not a file. In
/// addition, any string value shaped like a file name (no white space, no URL
/// scheme, a final segment with an extension) is registered whatever its key,
/// so a key outside the schema cannot hide a file.
const FILE_KEY_WORDS: &[&str] = &[
    "file",
    "files",
    "filename",
    "filenames",
    "path",
    "paths",
    "name",
    "names",
];

fn is_file_key(key: &str) -> bool {
    let lower = key.to_ascii_lowercase();
    lower != "name"
        && lower
            .rsplit(['_', '-', '.'])
            .next()
            .is_some_and(|word| FILE_KEY_WORDS.contains(&word))
}

fn looks_like_file(value: &str) -> bool {
    if value.is_empty() || value.contains(char::is_whitespace) || value.contains("://") {
        return false;
    }
    let last = value.rsplit('/').next().unwrap_or(value);
    last.rsplit_once('.').is_some_and(|(stem, extension)| {
        !stem.is_empty()
            && !extension.is_empty()
            && extension.len() <= 10
            && extension.chars().all(|c| c.is_ascii_alphanumeric())
            && extension.chars().any(|c| c.is_ascii_alphabetic())
    })
}

/// Collects file references from one dataset table, recursively. Every
/// value is walked whatever its container shape (arrays of arrays, inline
/// tables in arrays, nested tables), keeping the enclosing key so the schema
/// rule still applies to nested values; nothing is silently dropped (R4-03).
fn collect_files(key: &str, item: &Item, out: &mut Vec<String>) {
    match item {
        Item::Value(value) => collect_value(key, value, out),
        Item::Table(table) => {
            for (inner, value) in table {
                collect_files(inner, value, out);
            }
        }
        Item::ArrayOfTables(tables) => {
            for table in tables {
                for (inner, value) in table {
                    collect_files(inner, value, out);
                }
            }
        }
        Item::None => {}
    }
}

fn collect_value(key: &str, value: &toml_edit::Value, out: &mut Vec<String>) {
    match value {
        toml_edit::Value::String(text) => {
            let text = text.value();
            if is_file_key(key) || looks_like_file(text) {
                out.push(text.clone());
            }
        }
        toml_edit::Value::Array(array) => {
            for element in array {
                collect_value(key, element, out);
            }
        }
        toml_edit::Value::InlineTable(table) => {
            for (inner, element) in table {
                collect_value(inner, element, out);
            }
        }
        _ => {}
    }
}

/// Datasets declared by one manifest: (name, status, file names, licence text).
fn manifest_datasets(document: &DocumentMut) -> Vec<(String, Option<String>, Vec<String>, String)> {
    let mut found = Vec::new();
    let describe = |name: String, table: &dyn toml_edit::TableLike| {
        let field = |key: &str| table.get(key).and_then(Item::as_str).map(str::to_owned);
        let mut files = Vec::new();
        for (key, item) in table.iter() {
            if key != "name" {
                collect_files(key, item, &mut files);
            }
        }
        let licence = ["licence", "licence_name", "license", "license_name"]
            .iter()
            .filter_map(|key| field(key))
            .collect::<Vec<_>>()
            .join(" ");
        let status = field("licence_status").or_else(|| field("license_status"));
        (name, status, files, licence)
    };
    match document.get("datasets") {
        Some(Item::ArrayOfTables(tables)) => {
            for (index, table) in tables.iter().enumerate() {
                let name = table
                    .get("name")
                    .and_then(Item::as_str)
                    .map_or_else(|| format!("#{index}"), str::to_owned);
                found.push(describe(name, table));
            }
        }
        Some(item) => {
            if let Some(table) = item.as_table_like() {
                for (key, value) in table.iter() {
                    if let Some(inner) = value.as_table_like() {
                        found.push(describe(key.to_owned(), inner));
                    }
                }
            }
        }
        None => {}
    }
    found
}

/// Checks one provenance manifest; returns findings and research-only file names.
#[must_use]
pub fn check_manifest(path: &str, text: &str) -> (Vec<Finding>, Vec<String>) {
    let (findings, records) = manifest_records(path, text);
    let restricted = records
        .into_iter()
        .filter(|(status, _)| status == "research-only")
        .map(|(_, file)| file)
        .collect();
    (findings, restricted)
}

/// Checks one provenance manifest; returns findings and every
/// (licence status, file name) record it declares.
#[must_use]
pub fn manifest_records(path: &str, text: &str) -> (Vec<Finding>, Vec<(String, String)>) {
    let mut findings = Vec::new();
    let mut restricted = Vec::new();
    let document: DocumentMut = match text.parse() {
        Ok(document) => document,
        Err(error) => {
            return (
                vec![Finding::at(path, None, format!("invalid TOML: {error}"))],
                restricted,
            );
        }
    };
    let datasets = manifest_datasets(&document);
    if datasets.is_empty() {
        findings.push(Finding::at(
            path,
            None,
            "provenance manifest declares no datasets",
        ));
    }
    for (name, status, files, licence) in datasets {
        match status.as_deref() {
            Some(status) if STATUSES.contains(&status) => {
                restricted.extend(files.into_iter().map(|file| (status.to_owned(), file)));
            }
            Some(other) => findings.push(Finding::at(path, None, format!("dataset {name}: licence_status `{other}` must be open-commercial or research-only (D-56)"))),
            None => findings.push(Finding::at(path, None, format!("dataset {name}: missing licence_status (D-56)"))),
        }
        if mentions_share_alike(&licence) {
            findings.push(Finding::at(
                path,
                None,
                format!(
                    "dataset {name}: share-alike (ODbL/OSM/CC-BY-SA) data is never used (D-56)"
                ),
            ));
        }
    }
    (findings, restricted)
}

/// Checks a `sources.tsv` provenance table: (clearance, local-name, url).
/// Returns findings and research-only local names.
#[must_use]
pub fn check_sources_table(path: &str, text: &str) -> (Vec<Finding>, Vec<String>) {
    let (findings, records) = sources_records(path, text);
    let restricted = records
        .into_iter()
        .filter(|(status, _)| status == "research-only")
        .map(|(_, file)| file)
        .collect();
    (findings, restricted)
}

/// Checks a `sources.tsv` table; returns findings and (clearance, local-name)
/// records.
#[must_use]
pub fn sources_records(path: &str, text: &str) -> (Vec<Finding>, Vec<(String, String)>) {
    let mut findings = Vec::new();
    let mut restricted = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let cells: Vec<&str> = line.split('\t').collect();
        let clearance = cells.first().copied().unwrap_or_default();
        if !(STATUSES.contains(&clearance) || clearance == "metadata") {
            findings.push(Finding::at(
                path,
                Some(index + 1),
                format!(
                    "clearance `{clearance}` must be open-commercial, research-only or metadata"
                ),
            ));
        }
        if let Some(name) = cells.get(1) {
            restricted.push((clearance.to_owned(), (*name).to_owned()));
        }
        if mentions_share_alike(line) {
            findings.push(Finding::at(
                path,
                Some(index + 1),
                "share-alike (ODbL/OSM/CC-BY-SA) data is never used (D-56)",
            ));
        }
    }
    (findings, restricted)
}

struct SecretRule {
    name: &'static str,
    prefix: &'static str,
    min_tail: usize,
    tail: fn(char) -> bool,
    /// The tail must mix letters and digits (generic prefixes such as `sk-`).
    mixed: bool,
}

fn token_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-'
}

fn upper_digit(c: char) -> bool {
    c.is_ascii_uppercase() || c.is_ascii_digit()
}

const SECRET_RULES: &[SecretRule] = &[
    SecretRule {
        name: "AWS access key",
        prefix: "AKIA",
        min_tail: 16,
        tail: upper_digit,
        mixed: false,
    },
    SecretRule {
        name: "AWS access key",
        prefix: "ASIA",
        min_tail: 16,
        tail: upper_digit,
        mixed: false,
    },
    SecretRule {
        name: "GitHub token",
        prefix: "ghp_",
        min_tail: 36,
        tail: token_char,
        mixed: false,
    },
    SecretRule {
        name: "GitHub token",
        prefix: "gho_",
        min_tail: 36,
        tail: token_char,
        mixed: false,
    },
    SecretRule {
        name: "GitHub token",
        prefix: "ghu_",
        min_tail: 36,
        tail: token_char,
        mixed: false,
    },
    SecretRule {
        name: "GitHub token",
        prefix: "ghs_",
        min_tail: 36,
        tail: token_char,
        mixed: false,
    },
    SecretRule {
        name: "GitHub token",
        prefix: "ghr_",
        min_tail: 36,
        tail: token_char,
        mixed: false,
    },
    SecretRule {
        name: "GitHub token",
        prefix: "github_pat_",
        min_tail: 40,
        tail: token_char,
        mixed: false,
    },
    SecretRule {
        name: "Slack token",
        prefix: "xoxb-",
        min_tail: 20,
        tail: token_char,
        mixed: false,
    },
    SecretRule {
        name: "Slack token",
        prefix: "xoxp-",
        min_tail: 20,
        tail: token_char,
        mixed: false,
    },
    SecretRule {
        name: "Google API key",
        prefix: "AIza",
        min_tail: 35,
        tail: token_char,
        mixed: false,
    },
    SecretRule {
        name: "OpenAI-style key",
        prefix: "sk-proj-",
        min_tail: 32,
        tail: token_char,
        mixed: false,
    },
    SecretRule {
        name: "Anthropic-style key",
        prefix: "sk-ant-",
        min_tail: 32,
        tail: token_char,
        mixed: false,
    },
    SecretRule {
        name: "Stripe key",
        prefix: "sk_live_",
        min_tail: 20,
        tail: token_char,
        mixed: false,
    },
    SecretRule {
        name: "OpenAI-style key",
        prefix: "sk-",
        min_tail: 32,
        tail: token_char,
        mixed: true,
    },
    SecretRule {
        name: "Slack token",
        prefix: "xoxa-",
        min_tail: 20,
        tail: token_char,
        mixed: false,
    },
    SecretRule {
        name: "Slack token",
        prefix: "xoxr-",
        min_tail: 20,
        tail: token_char,
        mixed: false,
    },
    SecretRule {
        name: "Slack token",
        prefix: "xapp-",
        min_tail: 20,
        tail: token_char,
        mixed: false,
    },
    SecretRule {
        name: "Slack webhook",
        prefix: "hooks.slack.com/services/T",
        min_tail: 20,
        tail: |c| token_char(c) || c == '/',
        mixed: false,
    },
    SecretRule {
        name: "GitLab token",
        prefix: "glpat-",
        min_tail: 20,
        tail: token_char,
        mixed: false,
    },
    SecretRule {
        name: "Hugging Face token",
        prefix: "hf_",
        min_tail: 30,
        tail: token_char,
        mixed: true,
    },
    SecretRule {
        name: "npm token",
        prefix: "npm_",
        min_tail: 36,
        tail: token_char,
        mixed: true,
    },
    SecretRule {
        name: "PyPI token",
        prefix: "pypi-AgE",
        min_tail: 50,
        tail: token_char,
        mixed: false,
    },
    SecretRule {
        name: "Stripe key",
        prefix: "rk_live_",
        min_tail: 20,
        tail: token_char,
        mixed: false,
    },
    SecretRule {
        name: "Google OAuth token",
        prefix: "ya29.",
        min_tail: 30,
        tail: |c| token_char(c) || c == '.',
        mixed: true,
    },
    SecretRule {
        name: "SendGrid key",
        prefix: "SG.",
        min_tail: 60,
        tail: |c| token_char(c) || c == '.',
        mixed: true,
    },
    SecretRule {
        name: "Groq key",
        prefix: "gsk_",
        min_tail: 40,
        tail: token_char,
        mixed: true,
    },
    SecretRule {
        name: "xAI key",
        prefix: "xai-",
        min_tail: 40,
        tail: token_char,
        mixed: true,
    },
    SecretRule {
        name: "DigitalOcean token",
        prefix: "dop_v1_",
        min_tail: 64,
        tail: token_char,
        mixed: false,
    },
    SecretRule {
        name: "Azure storage key",
        prefix: "AccountKey=",
        min_tail: 40,
        tail: |c| c.is_ascii_alphanumeric() || matches!(c, '+' | '/' | '='),
        mixed: true,
    },
];

// Split so this source file does not match its own rule.
const PEM_HEADER: &str = concat!("-----BEG", "IN");
const PEM_KEY: &str = concat!("PRIVATE", " KEY");

/// Secret findings in one text.
#[must_use]
pub fn scan_secrets(path: &str, text: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.contains(PEM_HEADER) && line.contains(PEM_KEY) {
            findings.push(Finding::at(
                path,
                Some(index + 1),
                "private key material must not be committed",
            ));
        }
        let mut reported = false;
        for rule in SECRET_RULES {
            if reported {
                break;
            }
            let mut rest = line;
            while let Some(position) = rest.find(rule.prefix) {
                let before_ok = position == 0
                    || !rest[..position].ends_with(|c: char| c.is_ascii_alphanumeric());
                let tail: String = rest[position + rule.prefix.len()..]
                    .chars()
                    .take_while(|c| (rule.tail)(*c))
                    .collect();
                let mixed_ok = !rule.mixed
                    || (tail.chars().any(|c| c.is_ascii_digit())
                        && tail.chars().any(|c| c.is_ascii_alphabetic()));
                if before_ok && tail.len() >= rule.min_tail && mixed_ok {
                    findings.push(Finding::at(
                        path,
                        Some(index + 1),
                        format!("possible {} committed", rule.name),
                    ));
                    reported = true;
                    break;
                }
                rest = &rest[position + rule.prefix.len()..];
            }
        }
    }
    findings
}

const KEY_FILES: &[&str] = &[
    ".env",
    "id_rsa",
    "id_ed25519",
    "id_ecdsa",
    ".netrc",
    ".pgpass",
    "credentials.json",
    "gurobi.lic",
    "xpauth.xpr",
];
const KEY_EXTENSIONS: &[&str] = &[
    ".pem",
    ".key",
    ".p12",
    ".pfx",
    ".jks",
    ".keystore",
    ".lic",
    ".ovpn",
];

/// Leading bytes of data, archive and compressed formats: a file with one of
/// these signatures is data whatever its name (ADV23R7-02).
const DATA_SIGNATURES: &[(&[u8], &str)] = &[
    (b"\x1f\x8b", "gzip"),
    (b"\x28\xb5\x2f\xfd", "zstd"),
    (b"\x04\x22\x4d\x18", "lz4"),
    (b"\x02\x21\x4c\x18", "lz4 legacy"),
    (b"\xfd7zXZ\x00", "xz"),
    (b"\x5d\x00\x00", "lzma"),
    (b"7z\xbc\xaf\x27\x1c", "7z"),
    (b"PK\x03\x04", "zip"),
    (b"PK\x05\x06", "zip"),
    (b"PK\x07\x08", "zip"),
    (b"Rar!\x1a\x07", "rar"),
    (b"\x1f\x9d", "compress"),
    (b"\xff\x06\x00\x00sNaPpY", "snappy"),
    (b"LZIP", "lzip"),
    (b"\x89LZO", "lzo"),
    (b"MSCF", "cab"),
    (b"PAR1", "parquet"),
    (b"ARROW1", "arrow"),
    (b"Obj\x01", "avro"),
    (b"\x89HDF\r\n\x1a\n", "hdf5"),
    (b"CDF\x01", "netcdf"),
    (b"CDF\x02", "netcdf"),
    (b"GRIB", "grib"),
    (b"SQLite format 3\x00", "sqlite"),
    (b"\x93NUMPY", "numpy"),
    (b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1", "ole2 (xls)"),
    (b"%PDF-", "pdf"),
    (b"MATLAB 5.0 MAT-file", "matlab"),
    (b"\x00\x00\x27\x0a", "shapefile"),
];

/// The data format a file's content shows, if any: a known signature, a
/// tar header, or a Python pickle stream (ADV23R7-02).
#[must_use]
pub fn data_signature(bytes: &[u8]) -> Option<&'static str> {
    if let Some((_, name)) = DATA_SIGNATURES
        .iter()
        .find(|(magic, _)| bytes.starts_with(magic))
    {
        return Some(name);
    }
    if bytes.starts_with(b"BZh") && bytes.get(3).is_some_and(|b| (b'1'..=b'9').contains(b)) {
        return Some("bzip2");
    }
    if bytes.get(257..262) == Some(b"ustar") {
        return Some("tar");
    }
    if bytes.len() > 2 && bytes[0] == 0x80 && (2..=5).contains(&bytes[1]) {
        return Some("pickle");
    }
    // DuckDB keeps its magic after an 8-byte checksum.
    if bytes.get(8..12) == Some(b"DUCK") {
        return Some("duckdb");
    }
    if is_cim_rdf(bytes) {
        return Some("CIM/RDF XML");
    }
    solver_model(bytes)
}

/// The first 64 KiB of a file as text, decoded from UTF-8, UTF-16 or UTF-32
/// by byte-order mark or by the zero-byte pattern around a leading `<`
/// (ADV23R9-05, R9-03), with any BOM removed.
fn decoded_head(bytes: &[u8]) -> String {
    let head = &bytes[..bytes.len().min(64 * 1024)];
    let units = |width: usize, big: bool, skip: usize| -> String {
        head[skip..]
            .chunks_exact(width)
            .map(|chunk| {
                let value = if big {
                    chunk
                        .iter()
                        .fold(0_u32, |acc, b| (acc << 8) | u32::from(*b))
                } else {
                    chunk
                        .iter()
                        .rev()
                        .fold(0_u32, |acc, b| (acc << 8) | u32::from(*b))
                };
                char::from_u32(value).unwrap_or('\u{fffd}')
            })
            .collect()
    };
    match head {
        [0xef, 0xbb, 0xbf, ..] => String::from_utf8_lossy(&head[3..]).into_owned(),
        [0xff, 0xfe, 0, 0, ..] => units(4, false, 4),
        [0, 0, 0xfe, 0xff, ..] => units(4, true, 4),
        [0xff, 0xfe, ..] => units(2, false, 2),
        [0xfe, 0xff, ..] => units(2, true, 2),
        [a, 0, 0, 0, ..] if *a != 0 => units(4, false, 0),
        [0, 0, 0, b, ..] if *b != 0 => units(4, true, 0),
        [a, 0, ..] if *a != 0 => units(2, false, 0),
        [0, b, ..] if *b != 0 => units(2, true, 0),
        _ => String::from_utf8_lossy(head).into_owned(),
    }
}

/// Expands character references and the internal DTD's general entities
/// (`<!ENTITY rdf "http://...">`, common in RDF/XML and OWL), once.
fn expand_references(text: &str) -> String {
    let mut entities: Vec<(String, String)> = Vec::new();
    let mut rest = text;
    while let Some(index) = rest.find("<!ENTITY") {
        rest = &rest[index + 8..];
        let body = rest.trim_start();
        let name: String = body
            .chars()
            .take_while(|c| !c.is_whitespace() && *c != '"' && *c != '\'')
            .collect();
        let after = body[name.len()..].trim_start();
        if let Some(quote) = after.chars().next().filter(|c| *c == '"' || *c == '\'')
            && let Some(close) = after[1..].find(quote)
            && name != "%"
        {
            entities.push((format!("&{name};"), after[1..=close].to_owned()));
        }
    }
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(index) = rest.find('&') {
        out.push_str(&rest[..index]);
        rest = &rest[index..];
        let end = rest.find(';').filter(|end| *end <= 40);
        let decoded = end.and_then(|end| {
            let reference = &rest[..=end];
            let number = reference.strip_prefix("&#").map(|n| &n[..n.len() - 1]);
            number
                .and_then(|n| {
                    n.strip_prefix(['x', 'X']).map_or_else(
                        || n.parse::<u32>().ok(),
                        |hex| u32::from_str_radix(hex, 16).ok(),
                    )
                })
                .and_then(char::from_u32)
                .map(String::from)
                .or_else(|| {
                    entities
                        .iter()
                        .find(|(name, _)| name == reference)
                        .map(|(_, value)| value.clone())
                })
                .map(|value| (value, end + 1))
        });
        match decoded {
            Some((value, length)) => {
                out.push_str(&value);
                rest = &rest[length..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

/// The qualified name of an XML document's root element, after the prolog
/// (declaration, processing instructions, comments, doctype with any
/// internal subset); `None` when the text does not start as XML.
fn xml_root(text: &str) -> Option<String> {
    let mut rest = text.trim_start();
    loop {
        if !rest.starts_with('<') {
            return None;
        }
        if let Some(after) = rest.strip_prefix("<?") {
            rest = after.get(after.find("?>")? + 2..)?.trim_start();
        } else if let Some(after) = rest.strip_prefix("<!--") {
            rest = after.get(after.find("-->")? + 3..)?.trim_start();
        } else if rest.len() >= 9 && rest[..9].eq_ignore_ascii_case("<!doctype") {
            let (mut depth, mut quote, mut end) = (0_i32, None, None);
            for (index, c) in rest.char_indices().skip(9) {
                match (quote, c) {
                    (Some(q), c) if c == q => quote = None,
                    (Some(_), _) => {}
                    (None, '"' | '\'') => quote = Some(c),
                    (None, '[') => depth += 1,
                    (None, ']') => depth -= 1,
                    (None, '>') if depth <= 0 => {
                        end = Some(index);
                        break;
                    }
                    _ => {}
                }
            }
            rest = rest.get(end? + 1..)?.trim_start();
        } else {
            let name: String = rest[1..]
                .chars()
                .take_while(|c| !c.is_whitespace() && !matches!(c, '/' | '>'))
                .collect();
            let first = name.chars().next()?;
            return (first.is_alphabetic() || first == '_' || first == ':').then_some(name);
        }
    }
}

/// Whether a file is an RDF/XML document, such as a CGMES / IEC 61970 CIM
/// network model, whatever its name (ADV23R8-04, R9-03): it is XML in UTF-8,
/// UTF-16 or UTF-32 (with or without BOM or XML declaration) and either its
/// root element is `RDF` under any prefix or none, or its first 64 KiB,
/// after expanding character and entity references, name the RDF syntax
/// namespace or the IEC TC57 CIM or ENTSO-E CGMES namespaces.
#[must_use]
pub fn is_cim_rdf(bytes: &[u8]) -> bool {
    let text = decoded_head(bytes);
    let Some(root) = xml_root(&text) else {
        return false;
    };
    let local = root.rsplit(':').next().unwrap_or(&root);
    if local.eq_ignore_ascii_case("RDF") {
        return true;
    }
    let lower = expand_references(&text).to_lowercase();
    [
        "22-rdf-syntax-ns",
        "iec.ch/tc57/",
        "entsoe.eu/cim",
        "entsoe.eu/ns/cim",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

/// The solver model format a text file's content shows (ADV23R9-05): fixed
/// or free MPS (and QPS), CPLEX LP, or a GAMS model. Comment lines (`*` in
/// MPS, `\` in LP) are skipped.
#[must_use]
pub fn solver_model(bytes: &[u8]) -> Option<&'static str> {
    let text = decoded_head(bytes);
    let lines: Vec<&str> = text.lines().map(str::trim_end).collect();
    let first_token = |skip: char| {
        lines
            .iter()
            .map(|line| line.trim_start())
            .find(|line| !line.is_empty() && !line.starts_with(skip))
            .and_then(|line| line.split(|c: char| c.is_whitespace() || c == ':').next())
            .map(str::to_ascii_lowercase)
    };
    // MPS sections start in column 1.
    let section = |name: &str| {
        lines
            .iter()
            .any(|line| line.split_whitespace().next() == Some(name) && !line.starts_with(' '))
    };
    if matches!(first_token('*').as_deref(), Some("name" | "rows"))
        && section("ROWS")
        && section("COLUMNS")
    {
        return Some("MPS solver model");
    }
    let lower: Vec<String> = lines
        .iter()
        .map(|l| l.trim().to_ascii_lowercase())
        .collect();
    if matches!(
        first_token('\\').as_deref(),
        Some(
            "minimize"
                | "maximize"
                | "minimise"
                | "maximise"
                | "minimum"
                | "maximum"
                | "min"
                | "max"
        )
    ) && lower.iter().any(|line| {
        ["subject to", "such that", "st", "s.t."]
            .iter()
            .any(|key| line == key || line.starts_with(&format!("{key} ")))
    }) && lower.iter().any(|line| line == "end")
    {
        return Some("LP solver model");
    }
    let declares = lower.iter().any(|line| {
        line.split_whitespace()
            .next()
            .is_some_and(|word| ["variables", "variable", "equations", "equation"].contains(&word))
    });
    let solves = lower.iter().any(|line| {
        let words: Vec<&str> = line
            .split(|c: char| c.is_whitespace() || c == ';')
            .filter(|w| !w.is_empty())
            .collect();
        words.iter().position(|w| *w == "solve").is_some_and(|at| {
            words[at..].contains(&"using")
                && words[at..]
                    .iter()
                    .any(|w| matches!(*w, "minimizing" | "maximizing" | "min" | "max"))
        })
    });
    (declares && solves).then_some("GAMS solver model")
}

/// Checks tracked paths for data and key files. `records` are the
/// (licence status, file) entries of every provenance manifest and sources
/// table. A data file is allowed only as an exact listed metadata/fixture
/// path, or under a sample path AND registered as open-commercial (F9).
#[must_use]
pub fn check_paths(
    paths: &[String],
    document: &DocumentMut,
    records: &[(String, String)],
) -> Vec<Finding> {
    check_paths_with_content(paths, document, records, &BTreeMap::new())
}

/// [`check_paths`] with the data formats detected from file content
/// (`sniffed`: path to format). A file whose content is data is data even
/// when its extension is not listed (ADV23R7-02).
#[must_use]
pub fn check_paths_with_content(
    paths: &[String],
    document: &DocumentMut,
    records: &[(String, String)],
    sniffed: &BTreeMap<String, &str>,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    let data_extensions = string_array(&document["data_extensions"]);
    let sample_paths = string_array(&document["open_commercial_data"]);
    let metadata = string_array(&document["data_metadata_files"]);
    let aliases: BTreeMap<String, String> = document
        .get("sample_sources")
        .and_then(Item::as_table_like)
        .map(|table| {
            table
                .iter()
                .filter_map(|(key, value)| Some((key.to_owned(), value.as_str()?.to_owned())))
                .collect()
        })
        .unwrap_or_default();
    let mut registry: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (status, file) in records {
        let base = file.rsplit('/').next().unwrap_or(file);
        if !base.is_empty() {
            registry
                .entry(base.to_lowercase())
                .or_default()
                .insert(status.clone());
        }
    }
    for path in paths {
        let name = path.rsplit('/').next().unwrap_or(path);
        let lower = name.to_lowercase();
        if has_component(path, "raw") {
            findings.push(Finding::at(
                path,
                None,
                "files under a raw/ directory must not be committed (D-56)",
            ));
        }
        if KEY_FILES.contains(&lower.as_str())
            || (lower.starts_with(".env.") && lower != ".env.example")
            || KEY_EXTENSIONS.iter().any(|ext| lower.ends_with(ext))
        {
            findings.push(Finding::at(
                path,
                None,
                "key, licence or credential file must not be committed",
            ));
        }
        let listed_metadata = metadata.iter().any(|file| file == path);
        let source = aliases.get(path).map_or(name, String::as_str);
        let statuses = registry.get(&source.to_lowercase());
        if !listed_metadata && statuses.is_some_and(|s| s.contains("research-only")) {
            findings.push(Finding::at(
                path,
                None,
                format!(
                    "research-only dataset file is committed (D-56; provenance record {source})"
                ),
            ));
        }
        let content = sniffed.get(path);
        let is_data = data_extensions
            .iter()
            .any(|ext| lower.ends_with(ext.as_str()))
            || content.is_some();
        if !is_data || listed_metadata {
            continue;
        }
        if !sample_paths.iter().any(|glob| glob_match(glob, path)) {
            findings.push(Finding::at(
                path,
                None,
                match content {
                    Some(format) => format!(
                        "data file ({format} content) outside the allow-listed open-commercial sample paths (D-56)"
                    ),
                    None => "data file outside the allow-listed open-commercial sample paths (D-56)"
                        .to_owned(),
                },
            ));
        } else if !statuses.is_some_and(|s| s.contains("open-commercial")) {
            findings.push(Finding::at(
                path,
                None,
                format!(
                    "data sample has no open-commercial provenance record for {source} (D-56); register it in a manifest or sources.tsv"
                ),
            ));
        }
    }
    findings
}

/// Git blob hash of one line's text, used to name an exempt line without
/// repeating its content.
fn line_blob(repo: &std::path::Path, text: &str) -> String {
    use std::io::Write as _;
    let child = git_command(repo)
        .args(["hash-object", "--stdin"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn();
    let Ok(mut child) = child else {
        return String::new();
    };
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(text.as_bytes());
    }
    child
        .wait_with_output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .unwrap_or_default()
}

/// Synthetic example keys quoted in review reports: each names one exact
/// line of one file, with a reason (trusted policy; no wildcards).
fn load_secret_exemptions(document: &DocumentMut) -> Result<Vec<(String, String)>, String> {
    let mut found = Vec::new();
    if let Some(tables) = document
        .get("secret_exemption")
        .and_then(Item::as_array_of_tables)
    {
        for table in tables {
            let field = |key: &str| table.get(key).and_then(Item::as_str).map(str::to_owned);
            match (field("path"), field("line_blob"), field("reason")) {
                (Some(path), Some(blob), Some(reason))
                    if !path.contains(['*', '?', '['])
                        && is_sha(&blob)
                        && !reason.trim().is_empty() =>
                {
                    found.push((path, blob));
                }
                _ => {
                    return Err(format!(
                        "{POLICY}: every secret_exemption needs one exact path, line_blob and reason"
                    ));
                }
            }
        }
    }
    Ok(found)
}

/// The one canonical provenance manifest name (ADV23R3-07). Records may also
/// live in a `sources.tsv` table.
pub const MANIFEST_NAME: &str = "manifest.toml";
/// The canonical provenance table name.
pub const SOURCES_NAME: &str = "sources.tsv";

/// Suffixes of prose and source documents that may *name* manifests,
/// datasets or sources without being one (reviews, research notes, code).
/// They are still rejected when their content parses as structured
/// provenance records.
const PROSE_SUFFIXES: &[&str] = &[".md", ".rs", ".py", ".sh", ".html", ".css", ".js"];

fn has_status_key(key: &str) -> bool {
    let lower = key.to_ascii_lowercase();
    lower == "licence_status" || lower == "license_status"
}

fn toml_declares_status(table: &dyn toml_edit::TableLike) -> bool {
    table.iter().any(|(key, item)| {
        has_status_key(key)
            || match item {
                Item::Table(inner) => toml_declares_status(inner),
                Item::ArrayOfTables(tables) => tables.iter().any(|t| toml_declares_status(t)),
                Item::Value(value) => toml_value_declares_status(value),
                Item::None => false,
            }
    })
}

fn toml_value_declares_status(value: &toml_edit::Value) -> bool {
    match value {
        toml_edit::Value::InlineTable(table) => toml_declares_status(table),
        toml_edit::Value::Array(array) => array.iter().any(toml_value_declares_status),
        _ => false,
    }
}

fn json_declares_status(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(map) => map
            .iter()
            .any(|(key, inner)| has_status_key(key) || json_declares_status(inner)),
        serde_json::Value::Array(items) => items.iter().any(json_declares_status),
        _ => false,
    }
}

fn yaml_declares_status(value: &yaml_rust2::Yaml) -> bool {
    match value {
        yaml_rust2::Yaml::Hash(map) => map.iter().any(|(key, inner)| {
            key.as_str().is_some_and(has_status_key) || yaml_declares_status(inner)
        }),
        yaml_rust2::Yaml::Array(items) => items.iter().any(yaml_declares_status),
        _ => false,
    }
}

/// Whether the content, parsed structurally (TOML, JSON, YAML, or a
/// delimited table header), declares a licence status. Prose that merely
/// mentions the key does not parse as such a record (R4-04).
fn declares_status(text: &str) -> bool {
    if let Ok(document) = text.parse::<DocumentMut>()
        && toml_declares_status(document.as_table())
    {
        return true;
    }
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(text)
        && json_declares_status(&value)
    {
        return true;
    }
    if let Ok(documents) = yaml_rust2::YamlLoader::load_from_str(text)
        && documents.iter().any(yaml_declares_status)
    {
        return true;
    }
    text.lines()
        .find(|line| !line.trim().is_empty())
        .is_some_and(|header| {
            header
                .split(['\t', ',', ';'])
                .any(|field| has_status_key(field.trim().trim_matches('"')))
        })
}

/// Whether a tracked file other than the canonical `manifest.toml` and
/// `sources.tsv` looks like a provenance manifest. Such a file would not be
/// read, so its records would be ignored; it is rejected instead
/// (ADV23R3-07). The rule does not depend on the file's suffix (R4-04):
///
/// - any file whose content parses as structured records declaring a
///   licence status is a manifest, whatever its name;
/// - any file whose name mentions a manifest, provenance, dataset or sources
///   is a manifest unless it is a prose or source document
///   ([`PROSE_SUFFIXES`]); an extensionless or unknown suffix fails closed.
///
/// Reviewed metadata files listed in the trusted policy are exempt.
#[must_use]
pub fn misnamed_manifest(path: &str, text: &str, metadata: &[String]) -> Option<Finding> {
    let name = path.rsplit('/').next().unwrap_or(path).to_ascii_lowercase();
    if name == MANIFEST_NAME || name == SOURCES_NAME || metadata.iter().any(|m| m == path) {
        return None;
    }
    let prose = PROSE_SUFFIXES.iter().any(|suffix| name.ends_with(suffix));
    let named = !prose
        && ["manifest", "provenance", "dataset", "sources"]
            .iter()
            .any(|word| name.contains(word));
    (named || declares_status(text)).then(|| {
        Finding::at(
            path,
            None,
            format!(
                "provenance records must be in a file named {MANIFEST_NAME} (or {SOURCES_NAME}); this file would not be read (D-56)"
            ),
        )
    })
}

/// Runs the provenance, secrets and raw-data gate.
///
/// # Errors
///
/// Fails if Git, file reads or the trusted policy fail.
pub fn provenance(ctx: &Ctx) -> Result<Vec<Finding>, String> {
    let document = policy(ctx)?;
    let paths = tracked(&ctx.repo)?;
    let mut findings = Vec::new();
    let mut records = Vec::new();
    let mut manifests = 0;
    let secret_exemptions = load_secret_exemptions(&document)?;
    let metadata = string_array(&document["data_metadata_files"]);
    let mut sniffed = BTreeMap::new();
    for path in &paths {
        let name = path.rsplit('/').next().unwrap_or(path);
        let bytes =
            std::fs::read(ctx.repo.join(path)).map_err(|error| format!("{path}: {error}"))?;
        if let Some(format) = data_signature(&bytes) {
            sniffed.insert(path.clone(), format);
        }
        let text = String::from_utf8_lossy(&bytes);
        findings.extend(misnamed_manifest(path, &text, &metadata));
        if name == MANIFEST_NAME {
            manifests += 1;
            let (found, entries) = manifest_records(path, &text);
            findings.extend(found);
            records.extend(entries);
        }
        if name == SOURCES_NAME {
            let (found, entries) = sources_records(path, &text);
            findings.extend(found);
            records.extend(entries);
        }
        let lines: Vec<&str> = text.lines().collect();
        findings.extend(scan_secrets(path, &text).into_iter().filter(|finding| {
            let content = finding
                .line
                .and_then(|line| lines.get(line - 1))
                .copied()
                .unwrap_or_default();
            !secret_exemptions
                .iter()
                .any(|(file, blob)| file == path && line_blob(&ctx.repo, content) == *blob)
        }));
    }
    if manifests == 0 {
        findings.push(Finding::general(
            "no provenance manifest found; the gate would be vacuous",
        ));
    }
    findings.extend(check_paths_with_content(
        &paths, &document, &records, &sniffed,
    ));
    Ok(findings)
}

/// Checks that CODEOWNERS assigns every protected path to the owner.
///
/// # Errors
///
/// Fails if Git or the trusted policy cannot be read.
pub fn codeowners(ctx: &Ctx) -> Result<Vec<Finding>, String> {
    let document = policy(ctx)?;
    let owner = document
        .get("policy_owner")
        .and_then(Item::as_str)
        .ok_or("trusted policy: policy_owner missing")?;
    let protected = string_array(&document["protected_paths"]);
    let text = std::fs::read_to_string(ctx.repo.join(".github/CODEOWNERS")).unwrap_or_default();
    let paths = tracked(&ctx.repo)?;
    Ok(check_codeowners_paths(&text, owner, &protected, &paths))
}

/// One parsed CODEOWNERS rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerRule {
    /// The path pattern as written.
    pub pattern: String,
    /// Owner handles or e-mail addresses (empty clears ownership).
    pub owners: Vec<String>,
}

fn valid_owner(token: &str) -> bool {
    let name_ok = |name: &str| {
        !name.is_empty()
            && name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
    };
    if let Some(handle) = token.strip_prefix('@') {
        return match handle.split_once('/') {
            Some((org, team)) => name_ok(org) && name_ok(team),
            None => name_ok(handle),
        };
    }
    token
        .split_once('@')
        .is_some_and(|(local, domain)| !local.is_empty() && domain.contains('.'))
}

/// Parses CODEOWNERS like GitHub: `#` starts a comment (also after a rule),
/// a line with unsupported syntax or an invalid owner is skipped and
/// reported (F8).
#[must_use]
pub fn parse_codeowners(text: &str) -> (Vec<OwnerRule>, Vec<Finding>) {
    let mut rules = Vec::new();
    let mut findings = Vec::new();
    for (index, raw) in text.lines().enumerate() {
        let line = raw.split_once('#').map_or(raw, |(code, _)| code).trim();
        if line.is_empty() {
            continue;
        }
        let mut tokens = line.split_whitespace();
        let pattern = tokens.next().unwrap_or_default().to_owned();
        let owners: Vec<String> = tokens.map(str::to_owned).collect();
        let bad_pattern = pattern.starts_with('!')
            || pattern.contains(['[', ']', '\\'])
            || crate::util::partial_globstar(&pattern);
        let bad_owner = owners.iter().find(|owner| !valid_owner(owner));
        if bad_pattern || bad_owner.is_some() {
            findings.push(Finding::at(
                ".github/CODEOWNERS",
                Some(index + 1),
                format!(
                    "invalid CODEOWNERS line is ignored by GitHub: {}",
                    bad_owner.map_or("unsupported pattern syntax", String::as_str)
                ),
            ));
            continue;
        }
        rules.push(OwnerRule { pattern, owners });
    }
    (rules, findings)
}

/// Whether a CODEOWNERS pattern matches a repository path (gitignore-style:
/// a leading or inner `/` anchors to the root, otherwise any component
/// matches; a trailing `/` matches directories only; a match on a
/// directory covers everything below it, except after a final `/*`).
#[must_use]
pub fn codeowners_match(pattern: &str, path: &str) -> bool {
    if pattern == "*" || pattern == "**" {
        return true;
    }
    let (anchored, body) = match pattern.strip_prefix('/') {
        Some(rest) => (true, rest),
        None => (pattern.trim_end_matches('/').contains('/'), pattern),
    };
    let directory_only = body.ends_with('/');
    let body = body.trim_end_matches('/');
    if body.is_empty() {
        return false;
    }
    let final_star = body.rsplit('/').next() == Some("*");
    let components: Vec<&str> = path.split('/').collect();
    for end in 1..=components.len() {
        let is_path = end == components.len();
        if (is_path && directory_only) || (!is_path && final_star) {
            continue;
        }
        let matched = if anchored {
            glob_match(body, &components[..end].join("/"))
        } else {
            glob_match(body, components[end - 1])
        };
        if matched {
            return true;
        }
    }
    false
}

/// CODEOWNERS check over the protected policy paths and every tracked file
/// under them: the last matching valid rule must name the owner (F8).
#[must_use]
pub fn check_codeowners_paths(
    text: &str,
    owner: &str,
    protected: &[String],
    tracked: &[String],
) -> Vec<Finding> {
    let handle = format!("@{owner}");
    let (rules, mut findings) = parse_codeowners(text);
    let mut probes = BTreeSet::new();
    for pattern in protected {
        let bare = pattern.trim_start_matches('/');
        if bare.ends_with('/') {
            probes.insert(format!("{bare}CODEOWNERS-probe"));
        } else {
            probes.insert(bare.to_owned());
        }
        for path in tracked {
            if glob_match(bare, path) {
                probes.insert(path.clone());
            }
        }
    }
    for path in probes {
        let last = rules
            .iter()
            .rev()
            .find(|rule| codeowners_match(&rule.pattern, &path));
        let owned = last.is_some_and(|rule| {
            rule.owners
                .iter()
                .any(|candidate| candidate.eq_ignore_ascii_case(&handle))
        });
        if !owned {
            let shown = path.trim_end_matches("CODEOWNERS-probe");
            findings.push(Finding::at(
                ".github/CODEOWNERS",
                None,
                format!(
                    "{shown} must be owned by {handle} (effective rule: {})",
                    last.map_or("none", |rule| rule.pattern.as_str())
                ),
            ));
        }
    }
    findings
}

/// Pure CODEOWNERS comparison for the protected patterns alone.
#[must_use]
pub fn check_codeowners(text: &str, owner: &str, protected: &[String]) -> Vec<Finding> {
    check_codeowners_paths(text, owner, protected, &[])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paths(items: &[&str]) -> Vec<String> {
        items.iter().map(|item| (*item).to_owned()).collect()
    }

    // D-73: Python outside spikes/ and allow-listed validators is rejected.
    #[test]
    fn d73_python_outside_allowed_dirs_is_rejected() {
        let allowed = paths(&["spikes/", "tools/reference-validators/"]);
        let none = |_: &str| None;
        let found = check_python(
            &paths(&[
                "ci/check.py",
                "spikes/x/y.py",
                "tools/reference-validators/v.py",
                "xtask/src/main.rs",
                "a/notebook.ipynb",
            ]),
            &allowed,
            &none,
        );
        assert_eq!(found.len(), 2);
        let shebang = |_: &str| Some("#!/usr/bin/env python3".to_owned());
        assert_eq!(
            check_python(&paths(&["ci/check"]), &allowed, &shebang).len(),
            1
        );
    }

    // ADV23-11 / C2: tracked bytecode is rejected anywhere.
    #[test]
    fn adv23_11_bytecode_is_rejected() {
        let allowed = paths(&["spikes/"]);
        let none = |_: &str| None;
        let found = check_python(
            &paths(&[
                "ci/__pycache__/x.cpython-313.pyc",
                "spikes/a/__pycache__/b.pyc",
            ]),
            &allowed,
            &none,
        );
        assert_eq!(found.len(), 2);
    }

    // ADV23-04 / D-56: manifests, share-alike and research-only files.
    #[test]
    fn adv23_04_manifest_tags_are_required() {
        let good = "[[datasets]]\nname = \"a\"\nfilename = \"a.xlsx\"\nlicence_status = \"research-only\"\n[[datasets]]\nname = \"b\"\nlicence_status = \"open-commercial\"\n";
        let (found, restricted) = check_manifest("m.toml", good);
        assert!(found.is_empty());
        assert_eq!(restricted, ["a.xlsx"]);
        let tables = "[datasets.x]\nlicence_status = \"open-commercial\"\nlicence = \"BMRS\"\n";
        assert!(check_manifest("m.toml", tables).0.is_empty());
        let missing = "[[datasets]]\nname = \"a\"\n";
        assert_eq!(check_manifest("m.toml", missing).0.len(), 1);
        let wrong = "[[datasets]]\nname = \"a\"\nlicence_status = \"public\"\n";
        assert_eq!(check_manifest("m.toml", wrong).0.len(), 1);
        let odbl = "[[datasets]]\nname = \"a\"\nlicence_status = \"open-commercial\"\nlicence_name = \"ODbL 1.0\"\n";
        assert_eq!(check_manifest("m.toml", odbl).0.len(), 1);
        assert_eq!(check_manifest("m.toml", "").0.len(), 1);
    }

    #[test]
    fn adv23_04_data_files_and_secrets() {
        let document: DocumentMut = "data_extensions = [\".csv\", \".xlsx\", \".zip\"]\nopen_commercial_data = [\"spikes/data/samples/*.csv\"]\ndata_metadata_files = [\"spikes/data/sources.tsv\"]\n".parse().expect("toml");
        let records = vec![
            ("research-only".to_owned(), "etys.xlsx".to_owned()),
            ("open-commercial".to_owned(), "a.csv".to_owned()),
        ];
        let found = check_paths(
            &paths(&[
                "spikes/data/samples/a.csv",
                "data/b.csv",
                "x/etys.xlsx",
                "spikes/data/raw/r.txt",
                ".env",
                "k/server.pem",
                ".env.example",
            ]),
            &document,
            &records,
        );
        let messages: Vec<&str> = found
            .iter()
            .map(|f| f.file.as_deref().unwrap_or_default())
            .collect();
        assert_eq!(
            messages,
            [
                "data/b.csv",
                "x/etys.xlsx",
                "x/etys.xlsx",
                "spikes/data/raw/r.txt",
                ".env",
                "k/server.pem"
            ]
        );
        let key = ["gh", "p_", &"a".repeat(36)].concat();
        assert_eq!(scan_secrets("f", &format!("token = \"{key}\"")).len(), 1);
        let pem = ["-----BEGIN RSA PRIV", "ATE KEY-----"].concat();
        assert_eq!(scan_secrets("f", &pem).len(), 1);
        assert!(scan_secrets("f", "AKIA is a prefix; ghp_short").is_empty());
    }

    // ADV23R6-04: every path-component and file-name rule ignores letter case.
    #[test]
    fn adv23r6_04_path_rules_ignore_case() {
        let document: DocumentMut = "data_extensions = [\".csv\", \".xlsx\"]\nopen_commercial_data = [\"spikes/data/samples/*\"]\ndata_metadata_files = []\n".parse().expect("toml");
        let records = vec![
            ("research-only".to_owned(), "etys.xlsx".to_owned()),
            ("research-only".to_owned(), "grid.bin".to_owned()),
        ];
        for path in [
            "x/GRID.BIN",
            "spikes/RAW/notes.txt",
            "spikes/Raw/notes.txt",
            "data/rAw/x.md",
            "x/ETYS.XLSX",
        ] {
            let found = check_paths(&paths(&[path]), &document, &records);
            assert!(!found.is_empty(), "{path}");
        }
        // Control: an ordinary file name that merely contains the letters.
        assert!(check_paths(&paths(&["spikes/drawing/notes.txt"]), &document, &records).is_empty());
        let none = |_: &str| None;
        for path in ["tools/Gen.PY", "a/__PYCACHE__/m.txt", "b/m.PYC"] {
            assert_eq!(check_python(&paths(&[path]), &[], &none).len(), 1, "{path}");
        }
        assert!(has_component("a/RAW/b", "raw"));
        assert!(!has_component("a/rawdata/b", "raw"));
    }

    #[test]
    fn sources_table_clearance() {
        let text = "# clearance\tlocal-name\turl\nmetadata\ta.html\thttps://x\nresearch-only\tb.zip\thttps://y\nfree\tc\thttps://z\n";
        let (found, restricted) = check_sources_table("s.tsv", text);
        assert_eq!(found.len(), 1);
        assert_eq!(restricted, ["b.zip"]);
    }

    // R1: protected policy paths need the owner in CODEOWNERS.
    #[test]
    fn r1_codeowners_cover_protected_paths() {
        let protected = paths(&["/deny.toml", "/xtask/"]);
        assert!(
            check_codeowners("/deny.toml @Owner\n/xtask/ @owner\n", "Owner", &protected).is_empty()
        );
        assert_eq!(
            check_codeowners("/deny.toml @Owner\n", "Owner", &protected).len(),
            1
        );
        assert_eq!(
            check_codeowners(
                "/deny.toml @Owner\n/xtask/ @Owner\n/xtask/ @someone\n",
                "Owner",
                &protected
            )
            .len(),
            1
        );
    }
    // ADV23R-09: generic OpenAI-style and other key formats.
    #[test]
    fn adv23r_09_more_key_formats_are_detected() {
        let tail = "abcdef1234567890abcdef1234567890abcdef12345678";
        for key in [
            ["s", "k-", tail].concat(),
            ["s", "k-svcacct-", tail].concat(),
            ["gl", "pat-", &tail[..24]].concat(),
            ["h", "f_", &tail[..34]].concat(),
            ["np", "m_", &tail[..36]].concat(),
            ["xo", "xa-", &tail[..24]].concat(),
            ["gs", "k_", tail].concat(),
            ["Account", "Key=", tail].concat(),
        ] {
            let found = scan_secrets("f", &format!("key = \"{key}\""));
            assert_eq!(found.len(), 1, "missed: {key}");
        }
        for text in [
            "risk-assessment-of-the-new-transmission-boundary-model",
            "task-abcdefghijklmnopqrstuvwxyzabcdefghij",
            "--sk-some-long-kebab-case-identifier-without-digits",
        ] {
            assert!(scan_secrets("f", text).is_empty(), "false positive: {text}");
        }
    }

    // ADV23R-09: array-valued and alternative manifest file keys.
    #[test]
    fn adv23r_09_manifest_file_arrays_are_read() {
        for files in [
            "files = [\"restricted_grid.json\", \"other.csv\"]",
            "file = \"restricted_grid.json\"",
            "paths = [\"sub/restricted_grid.json\"]",
            "path = \"restricted_grid.json\"",
        ] {
            let text = format!(
                "[[datasets]]\nname = \"r\"\nlicence_status = \"research-only\"\n{files}\n"
            );
            let (found, restricted) = check_manifest("m.toml", &text);
            assert!(found.is_empty(), "{found:?}");
            assert!(
                restricted
                    .iter()
                    .any(|f| f.ends_with("restricted_grid.json")),
                "{files}: {restricted:?}"
            );
        }
    }

    // ADV23R3-06: file references are found by the schema rule (any key whose
    // last word is file/path/name...) and by value shape, at any depth.
    #[test]
    fn adv23r3_06_file_references_follow_the_schema_rule() {
        for files in [
            "sha256_index_file = \"restricted_grid.json\"",
            "index_file = \"restricted_grid.json\"",
            "data_files = [\"restricted_grid.json\"]",
            "source-path = \"a/restricted_grid.json\"",
            "checksums = \"restricted_grid.json\"",
            "parts = [{ uri = \"restricted_grid.json\" }]",
            "[datasets.r.storage]\nlocation = \"restricted_grid.json\"",
        ] {
            let text = format!(
                "[datasets.r]\nname = \"Restricted grid\"\nlicence_status = \"research-only\"\n{files}\n"
            );
            let (found, restricted) = check_manifest("m.toml", &text);
            assert!(found.is_empty(), "{found:?}");
            assert!(
                restricted
                    .iter()
                    .any(|f| f.ends_with("restricted_grid.json")),
                "{files}: {restricted:?}"
            );
        }
        // The title, URLs, versions and prose are not file references.
        let text = "[datasets.r]\nname = \"Grid.v2\"\nlicence_status = \"research-only\"\nversion = \"1.0.0\"\nsource_url = \"https://example.org/a.csv\"\nnote = \"see the README.md file\"\n";
        let (_, restricted) = check_manifest("m.toml", text);
        assert!(restricted.is_empty(), "{restricted:?}");
    }

    // ADV23R3-07: provenance records under any other file name are rejected.
    #[test]
    fn adv23r3_07_misnamed_manifests_are_rejected() {
        let record = "[[datasets]]\nname = \"r\"\nlicence_status = \"research-only\"\n";
        for path in [
            "spikes/data/provenance.toml",
            "spikes/data/data-manifest.toml",
            "spikes/data/dataset.toml",
            "spikes/data/manifest.json",
            "spikes/data/manifest.yaml",
            "spikes/data/anything.toml",
        ] {
            assert!(
                misnamed_manifest(path, record, &[]).is_some(),
                "{path} accepted"
            );
        }
        assert!(misnamed_manifest("spikes/data/manifest.toml", record, &[]).is_none());
        assert!(misnamed_manifest("spikes/data/sources.tsv", "", &[]).is_none());
        assert!(misnamed_manifest("Cargo.toml", "[package]\n", &[]).is_none());
        assert!(misnamed_manifest("docs/notes.md", "See `licence_status`.\n", &[]).is_none());
        assert!(misnamed_manifest("docs/data-sources-gb.md", "# Sources\n", &[]).is_none());
        let listed = vec!["spikes/data/samples/download-manifest.json".to_owned()];
        assert!(
            misnamed_manifest(
                "spikes/data/samples/download-manifest.json",
                record,
                &listed
            )
            .is_none()
        );
    }

    // R4-03: arrays nested in arrays (and any other container shape) are walked.
    #[test]
    fn r4_03_nested_arrays_are_walked() {
        for files in [
            "files = [[\"restricted.txt\"]]",
            "files = [[[\"restricted.txt\"]]]",
            "files = [[{ path = \"restricted.txt\" }]]",
            "parts = [{ inner = [\"restricted.txt\"] }]",
            "files = [\"other.csv\", [\"restricted.txt\"]]",
        ] {
            let text = format!(
                "[[datasets]]\nname = \"r\"\nlicence_status = \"research-only\"\n{files}\n"
            );
            let (found, restricted) = check_manifest("m.toml", &text);
            assert!(found.is_empty(), "{found:?}");
            assert!(
                restricted.iter().any(|f| f == "restricted.txt"),
                "{files}: {restricted:?}"
            );
        }
    }

    // R4-04: the manifest rule does not depend on the file name's suffix.
    #[test]
    fn r4_04_suffix_does_not_exempt_manifests() {
        let record = "[[datasets]]\nname = \"r\"\nlicence_status = \"research-only\"\nfile = \"restricted.txt\"\n";
        for path in [
            "data/provenance",
            "data/records",
            "data/records.txt",
            "data/records.dat",
            "data/manifest",
            "data/MANIFEST.lock",
            "data/notes.md",
        ] {
            assert!(
                misnamed_manifest(path, record, &[]).is_some(),
                "{path} accepted"
            );
        }
        // Named like a manifest with an unknown suffix: fails closed.
        assert!(misnamed_manifest("data/provenance", "anything\n", &[]).is_some());
        // JSON, YAML and delimited headers declaring a status, any name.
        for text in [
            "{\"datasets\": [{\"licence_status\": \"research-only\"}]}",
            "datasets:\n  - licence_status: research-only\n",
            "name\tlicence_status\tfile\nr\tresearch-only\tx.txt\n",
        ] {
            assert!(misnamed_manifest("data/x", text, &[]).is_some(), "{text}");
        }
        // Controls: prose and code may mention the key or the words.
        assert!(
            misnamed_manifest("xtask/src/repository.rs", "let licence_status = 1;\n", &[])
                .is_none()
        );
        assert!(
            misnamed_manifest(
                "docs/review.md",
                "Uses `licence_status = \"x\"` keys.\n",
                &[]
            )
            .is_none()
        );
        assert!(misnamed_manifest("data/manifest.toml", record, &[]).is_none());
    }

    // ADV23R7-02: compressed and archive formats count as data by name, and
    // data is detected by content whatever its name.
    #[test]
    fn adv23r7_02_compressed_and_sniffed_data_is_gated() {
        let document: DocumentMut = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("policy/repository.toml"),
        )
        .expect("policy")
        .parse()
        .expect("toml");
        for path in [
            "spikes/grid_metrics.zst",
            "spikes/transmission_data.lz4",
            "data/flows.csv.zstandard",
            "data/a.xz",
            "data/a.bz2",
            "data/a.7z",
            "data/a.br",
            "data/a.tgz",
            "data/a.rar",
            "data/a.avro",
        ] {
            let found = check_paths(&paths(&[path]), &document, &[]);
            assert_eq!(found.len(), 1, "{path}: {found:?}");
        }
        let mut tar = vec![0_u8; 512];
        tar[257..262].copy_from_slice(b"ustar");
        let samples: Vec<(&str, Vec<u8>)> = vec![
            ("zstd", vec![0x28, 0xb5, 0x2f, 0xfd, 0, 0]),
            ("lz4", vec![0x04, 0x22, 0x4d, 0x18, 0]),
            ("xz", b"\xfd7zXZ\x00\x00".to_vec()),
            ("bzip2", b"BZh91AY&SY".to_vec()),
            ("7z", b"7z\xbc\xaf\x27\x1c\x00".to_vec()),
            ("gzip", vec![0x1f, 0x8b, 8, 0]),
            ("zip", b"PK\x03\x04rest".to_vec()),
            ("parquet", b"PAR1....".to_vec()),
            ("sqlite", b"SQLite format 3\x00".to_vec()),
            ("tar", tar),
        ];
        for (format, bytes) in &samples {
            assert_eq!(data_signature(bytes), Some(*format), "{format}");
            // Renamed to an unlisted extension, the content still decides.
            let path = "spikes/notes.bin".to_owned();
            let sniffed = BTreeMap::from([(path.clone(), *format)]);
            let found = check_paths_with_content(&[path], &document, &[], &sniffed);
            assert_eq!(found.len(), 1, "{format}: {found:?}");
            assert!(found[0].message.contains(format), "{found:?}");
        }
        // Controls: text and source files are not data.
        for text in [
            &b"# Notes\nORC and DUCK are words.\n"[..],
            b"BZh is not a header here",
            b"fn main() {}\n",
            b"\x89PNG\r\n\x1a\n",
        ] {
            assert_eq!(data_signature(text), None, "{text:?}");
        }
        assert!(
            check_paths(
                &paths(&["crates/x/src/lib.rs", "docs/a.md"]),
                &document,
                &[]
            )
            .is_empty()
        );
    }

    // ADV23R8-04: CGMES/CIM RDF-XML, PowerFactory and other network-model
    // formats are data by name, and CIM/RDF XML is data by content.
    #[test]
    fn adv23r8_04_network_models_are_gated() {
        let document: DocumentMut = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("policy/repository.toml"),
        )
        .expect("policy")
        .parse()
        .expect("toml");
        for path in [
            "spikes/europe_grid_model.rdf",
            "spikes/network_case.pfd",
            "spikes/case.dz",
            "spikes/EQ.xml",
            "data/model.ttl",
            "data/model.jsonld",
            "data/grid.dgs",
            "data/case.pwb",
            "data/net.uct",
            "data/net.xiidm",
            "data/feeder.dss",
            "data/feeder.glm",
        ] {
            let found = check_paths(&paths(&[path]), &document, &[]);
            assert_eq!(found.len(), 1, "{path}: {found:?}");
        }
        let cgmes = b"\xef\xbb\xbf<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<rdf:RDF xmlns:cim=\"http://iec.ch/TC57/2013/CIM-schema-cim16#\" xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">\n";
        let bare = b"\n  <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"/>";
        let entsoe = b"<?xml version=\"1.0\"?>\n<!-- model -->\n<md:FullModel xmlns:entsoe=\"http://entsoe.eu/CIM/SchemaExtension/3/1#\"/>";
        for bytes in [&cgmes[..], bare, entsoe] {
            assert_eq!(data_signature(bytes), Some("CIM/RDF XML"));
            let path = "spikes/notes.txt".to_owned();
            let sniffed = BTreeMap::from([(path.clone(), "CIM/RDF XML")]);
            let found = check_paths_with_content(&[path], &document, &[], &sniffed);
            assert_eq!(found.len(), 1, "{found:?}");
        }
        // Controls: Markdown and code that mention RDF are not RDF documents.
        for text in [
            &b"# Notes\nCGMES uses <rdf:RDF> with http://iec.ch/TC57/ namespaces.\n"[..],
            b"let ns = \"http://www.w3.org/1999/02/22-rdf-syntax-ns#\";\n",
            b"<?xml version=\"1.0\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\"/>\n",
        ] {
            assert_eq!(data_signature(text), None, "{text:?}");
        }
    }

    // R9-03 / ADV23R9-05: RDF/XML is recognised by its root element or RDF
    // namespace whatever the prefix, declaration, encoding or BOM; solver
    // model formats are gated by extension and (MPS, LP, GAMS) by content.
    // The GPT fixture (default namespace, no declaration) passed at e64acc5.
    #[test]
    fn r9_03_adv23r9_05_rdf_and_solver_models_are_gated() {
        let document: DocumentMut = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("policy/repository.toml"),
        )
        .expect("policy")
        .parse()
        .expect("toml");
        let gpt = "<RDF xmlns=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"\n     xmlns:cim=\"http://iec.ch/TC57/2013/CIM-schema-cim16#\">\n  <cim:BaseVoltage>\n    <cim:IdentifiedObject.name>400kV</cim:IdentifiedObject.name>\n  </cim:BaseVoltage>\n</RDF>\n";
        let prefixed = "<r:RDF xmlns:r=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"/>\n";
        let bare_root = "<RDF><Description about=\"x\"/></RDF>";
        let node_root = "<ex:Node xmlns:ex=\"urn:x\" xmlns:q=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\" q:about=\"x\"/>";
        let entity = "<?xml version=\"1.0\"?>\n<!DOCTYPE r [\n  <!ENTITY ns \"http://www.w3.org/1999/02/&#50;2-rdf-syntax-ns#\">\n]>\n<x:Thing xmlns:x=\"&ns;\"/>";
        let charref = "<x:Thing xmlns:x=\"http://www.w3.org/1999/02/&#x32;2-rdf-syntax-ns#\"/>";
        let utf16 = |text: &str, big: bool, bom: bool| -> Vec<u8> {
            let mut out = Vec::new();
            if bom {
                out.extend(if big { [0xfe, 0xff] } else { [0xff, 0xfe] });
            }
            for unit in text.encode_utf16() {
                out.extend(if big {
                    unit.to_be_bytes()
                } else {
                    unit.to_le_bytes()
                });
            }
            out
        };
        let utf32: Vec<u8> = prefixed
            .chars()
            .flat_map(|c| (c as u32).to_le_bytes())
            .collect();
        let samples: Vec<Vec<u8>> = vec![
            gpt.as_bytes().to_vec(),
            prefixed.as_bytes().to_vec(),
            bare_root.as_bytes().to_vec(),
            node_root.as_bytes().to_vec(),
            entity.as_bytes().to_vec(),
            charref.as_bytes().to_vec(),
            [b"\xef\xbb\xbf".as_slice(), gpt.as_bytes()].concat(),
            utf16(gpt, false, true),
            utf16(gpt, true, true),
            utf16(gpt, false, false),
            utf16(prefixed, true, false),
            utf32,
            format!("<!-- c -->\n<!DOCTYPE a [<!ELEMENT a ANY>]>\n{prefixed}").into_bytes(),
        ];
        for bytes in &samples {
            assert_eq!(data_signature(bytes), Some("CIM/RDF XML"), "{bytes:?}");
        }
        let mps = "* unit commitment\nNAME          UC\nROWS\n N  COST\n L  LIM1\nCOLUMNS\n    X1        COST         1.0   LIM1         1.0\nRHS\n    RHS       LIM1         4.0\nENDATA\n";
        let free_mps = "ROWS\n N obj\nCOLUMNS\n x obj 1\nENDATA\n";
        let lp = "\\ Problem name: opf\nMinimize\n obj: x + y\nSubject To\n c1: x + y >= 1\nEnd\n";
        let lp_short = "max: 3x + 2y\nst\n c1: x <= 4\nend\n";
        let gams = "Sets i / a, b /;\nVariables z;\nEquations cost;\ncost.. z =e= 1;\nModel m / all /;\nSolve m using lp minimizing z;\n";
        for (text, format) in [
            (mps, "MPS solver model"),
            (free_mps, "MPS solver model"),
            (lp, "LP solver model"),
            (lp_short, "LP solver model"),
            (gams, "GAMS solver model"),
        ] {
            assert_eq!(data_signature(text.as_bytes()), Some(format), "{text}");
            let path = "spikes/notes.txt".to_owned();
            let sniffed = BTreeMap::from([(path.clone(), format)]);
            assert_eq!(
                check_paths_with_content(&[path], &document, &[], &sniffed).len(),
                1
            );
        }
        for path in [
            "spikes/unit_commitment.mps",
            "spikes/europe_opf.lp",
            "spikes/model.qps",
            "spikes/model.nl",
            "spikes/model.gms",
            "spikes/model.gdx",
        ] {
            assert_eq!(
                check_paths(&paths(&[path]), &document, &[]).len(),
                1,
                "{path}"
            );
        }
        // Controls: documentation, code and ordinary XML are not models.
        for text in [
            &b"# Notes\nCGMES uses <rdf:RDF> with http://iec.ch/TC57/ namespaces.\n"[..],
            b"let ns = \"http://www.w3.org/1999/02/22-rdf-syntax-ns#\";\n",
            b"<?xml version=\"1.0\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\"/>\n",
            b"<html><body>RDF</body></html>",
            b"# MPS\nThe NAME, ROWS and COLUMNS sections hold the model.\n",
            b"Minimize the cost subject to limits.\nThe end.\n",
            b"fn solve() { /* solve m using lp minimizing z */ }\n",
            b"NAME\n  ROWS are indented here\n",
        ] {
            assert_eq!(data_signature(text), None, "{text:?}");
        }
    }

    // ADV23R-09: grid and market exchange formats count as data.
    #[test]
    fn adv23r_09_more_data_extensions_are_gated() {
        let document: DocumentMut = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("policy/repository.toml"),
        )
        .expect("policy")
        .parse()
        .expect("toml");
        for path in [
            "data/case.mat",
            "data/ieee14.cdf",
            "data/case9.m",
            "data/api-dump.json",
            "data/flows.xml",
            "data/grid.npz",
        ] {
            let found = check_paths(&paths(&[path]), &document, &[]);
            assert_eq!(found.len(), 1, "{path}: {found:?}");
        }
    }

    // F8: effective (last matching) CODEOWNERS rule decides ownership.
    #[test]
    fn f8_codeowners_overrides_are_rejected() {
        let protected = paths(&["/xtask/", "/deny.toml"]);
        let tracked = paths(&["xtask/src/lib.rs", "xtask/Cargo.toml", "deny.toml"]);
        let check = |text: &str| check_codeowners_paths(text, "Owner", &protected, &tracked);
        assert!(check("/xtask/ @Owner\n/deny.toml @Owner\n").is_empty());
        assert!(check("* @Owner\n").is_empty());
        for text in [
            "/xtask/ @Owner\n/deny.toml @Owner\n/xtask/src/lib.rs @someone\n",
            "/xtask/ @Owner\n/deny.toml @Owner\n*.rs @someone\n",
            "/xtask/ @Owner\n/deny.toml @Owner\n/xtask/src/ @someone\n",
            "/xtask/ @Owner\n/deny.toml @Owner\nsrc/ @someone\n",
            "/xtask/ @Owner\n/deny.toml @Owner\n/xtask/src/*\n",
            "/xtask/ @Owner\n/deny.toml @Owner\nCargo.toml @someone\n",
            "/xtask/ @Owner\n/deny.toml # @Owner\n",
            "/xtask/ @Owner\n/deny.toml @Owner\n* @someone\n",
        ] {
            assert!(!check(text).is_empty(), "accepted: {text}");
        }
        // An invalid line is skipped by GitHub, so it cannot grant ownership.
        let invalid = check("/xtask/ @Owner\n/deny.toml Owner\n");
        assert!(invalid.len() >= 2, "{invalid:?}");
        // `docs/*` does not reach nested files; a later narrower owner rule restores.
        let star = check("/xtask/ @Owner\n/deny.toml @Owner\n/xtask/* @someone\n");
        assert_eq!(star.len(), 2, "direct children only: {star:?}");
        assert!(star.iter().all(|f| !f.message.contains("src/lib.rs")));
        assert!(check("* @someone\n/xtask/ @Owner\n/deny.toml @Owner\n").is_empty());
    }

    // R3-04: `?` is a gitignore wildcard, so a later `?` rule can override.
    #[test]
    fn r3_04_question_mark_override_is_rejected() {
        let protected = paths(&["/xtask/"]);
        let tracked = paths(&["xtask/src/lib.rs"]);
        let check = |text: &str| check_codeowners_paths(text, "Owner", &protected, &tracked);
        assert!(!check("/xtask/ @Owner\n/xtask/sr?/lib.rs @someone\n").is_empty());
        assert!(!check("/xtask/ @Owner\n/x?ask/ @someone\n").is_empty());
        // Controls: `?` needs exactly one character and never matches `/`.
        assert!(check("/xtask/ @Owner\n/xtask/sr?x/lib.rs @someone\n").is_empty());
        assert!(check("/xtask/ @Owner\n/xtask?src/lib.rs @someone\n").is_empty());
    }

    // R4-02: `**/` matches whole directories, so it cannot reach a file
    // whose name merely ends with the literal and manufacture coverage.
    #[test]
    fn r4_02_globstar_cannot_manufacture_coverage() {
        let protected = paths(&["/xtask/"]);
        let tracked = paths(&["xtask/evillib.rs", "xtask/lib.rs", "xtask/src/lib.rs"]);
        let check = |text: &str| check_codeowners_paths(text, "Owner", &protected, &tracked);
        let found = check("/xtask/ @Owner\n/xtask/evillib.rs @someone\n/xtask/**/lib.rs @Owner\n");
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].message.contains("xtask/evillib.rs"), "{found:?}");
        assert!(!codeowners_match("/xtask/**/lib.rs", "xtask/evillib.rs"));
        assert!(codeowners_match("/xtask/**/lib.rs", "xtask/lib.rs"));
        assert!(codeowners_match("/xtask/**/lib.rs", "xtask/src/lib.rs"));
        // `**` that does not fill a component is version-dependent in Git:
        // the line is rejected and grants nothing.
        let partial = check("/xtask/ @Owner\n/xtask/**lib.rs @Owner\n/xtask/evillib.rs @someone\n");
        assert!(partial.len() >= 2, "{partial:?}");
        // Control: a genuine override by `**/` still restores ownership.
        assert!(
            check("/xtask/ @Owner\n/xtask/src/lib.rs @someone\n/xtask/**/lib.rs @Owner\n")
                .is_empty()
        );
    }

    #[test]
    fn codeowners_matching_follows_github_rules() {
        for (pattern, path, expected) in [
            ("/xtask/", "xtask/src/lib.rs", true),
            ("/xtask/", "a/xtask/src/lib.rs", false),
            ("xtask/", "a/xtask/src/lib.rs", true),
            ("/xtask", "xtask/src/lib.rs", true),
            ("*.rs", "a/b/c.rs", true),
            ("/docs/*", "docs/a.md", true),
            ("/docs/*", "docs/a/b.md", false),
            ("docs/**/x.md", "docs/a/b/x.md", true),
            ("/deny.toml", "deny.toml", true),
            ("/deny.toml", "x/deny.toml", false),
            ("deny.toml", "x/deny.toml", true),
            ("/xtask/", "xtask", false),
        ] {
            assert_eq!(
                codeowners_match(pattern, path),
                expected,
                "{pattern} {path}"
            );
        }
    }
}
