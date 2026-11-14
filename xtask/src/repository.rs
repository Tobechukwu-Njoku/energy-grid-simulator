//! Repository content policy.
//!
//! - `python` (D-73): no Python, Julia or other non-allow-listed script or
//!   source language (by extension or shebang) outside `spikes/` and the
//!   allow-listed offline reference-validator directories; no tracked
//!   bytecode (ADV23-11, ADV23R10-02).
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

/// Programming and scripting language source extensions (D-20, D-73,
/// ADV23R10-02). Outside the allowed directories a file with one of these
/// extensions is rejected unless the trusted policy's
/// `source_extensions_allowed` lists it; the product language is Rust.
pub const LANGUAGE_EXTENSIONS: &[&str] = &[
    ".py", ".pyw", ".pyi", ".pyx", ".pxd", ".ipynb", ".jl", ".r", ".rmd", ".qmd", ".m", ".mlx",
    ".js", ".mjs", ".cjs", ".jsx", ".ts", ".mts", ".cts", ".tsx", ".rb", ".pl", ".pm", ".lua",
    ".php", ".ps1", ".psm1", ".bat", ".cmd", ".vbs", ".vb", ".tcl", ".awk", ".groovy", ".gradle",
    ".kt", ".kts", ".scala", ".sc", ".java", ".clj", ".cljs", ".cljc", ".cs", ".fs", ".fsx", ".go",
    ".swift", ".dart", ".ex", ".exs", ".erl", ".hrl", ".hs", ".lhs", ".ml", ".mli", ".nim", ".zig",
    ".cr", ".c", ".h", ".cc", ".cpp", ".cxx", ".hpp", ".hxx", ".f", ".for", ".f90", ".f95", ".f03",
    ".f08", ".coffee", ".elm", ".rkt", ".scm", ".ss", ".lisp", ".lsp", ".el", ".vim", ".fish",
    ".zsh", ".ksh", ".csh", ".tcsh", ".do", ".ado", ".sas", ".stan", ".wl", ".nb", ".mo", ".d",
    ".v", ".sv", ".vhd", ".asm", ".s", ".wasm", ".wat", ".sql", ".cu", ".cuh", ".mex", ".octave",
    ".sage", ".hy", ".raku", ".rakumod", ".p6", ".pl6",
];

/// Script and source language policy over a list of tracked paths (D-73,
/// ADV23R10-02). Outside the `allowed` directories, a file is rejected if
/// its extension is a [`LANGUAGE_EXTENSIONS`] entry not in `extensions`, or
/// if its first line is a shebang (`#!`, but not a Rust `#![` attribute)
/// whose interpreter is not in `interpreters`. Python bytecode is rejected
/// everywhere.
#[must_use]
pub fn check_scripts(
    paths: &[String],
    allowed: &[String],
    extensions: &[String],
    interpreters: &[String],
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
        let permitted = |ext: &str| extensions.iter().any(|e| e.eq_ignore_ascii_case(ext));
        if let Some(ext) = LANGUAGE_EXTENSIONS
            .iter()
            .find(|ext| lower.ends_with(*ext) && !permitted(ext))
        {
            findings.push(Finding::at(
                path,
                None,
                format!(
                    "{ext} source is allowed only in spikes/ and allow-listed reference validators; product code, tooling and tests are Rust (D-73)"
                ),
            ));
            continue;
        }
        if let Some(interpreter) = heads(path).as_deref().and_then(shebang_interpreter)
            && !interpreters
                .iter()
                .any(|allowed| allowed.eq_ignore_ascii_case(&interpreter))
        {
            findings.push(Finding::at(
                path,
                None,
                format!(
                    "`{interpreter}` script is allowed only in spikes/ and allow-listed reference validators (D-73)"
                ),
            ));
        }
    }
    findings
}

/// The interpreter a shebang line names: the program's file name, or for
/// `env` its first non-option, non-assignment argument. `None` when the line
/// is not a shebang (a Rust `#![...]` inner attribute is not one). Leading
/// byte-order marks are ignored (ADV23R11-01).
#[must_use]
pub fn shebang_interpreter(first: &str) -> Option<String> {
    let rest = first.trim_start_matches('\u{feff}').strip_prefix("#!")?;
    if rest.trim_start().starts_with('[') {
        return None;
    }
    let mut words = rest.split_whitespace();
    let base = |word: &str| word.rsplit('/').next().unwrap_or(word).to_lowercase();
    let program = base(words.next().unwrap_or_default());
    if program != "env" {
        return Some(program);
    }
    Some(
        words
            .find(|word| !word.starts_with('-') && !word.contains('='))
            .map_or(program, base),
    )
}

/// Runs the script language check (D-73).
///
/// # Errors
///
/// Fails if Git or the trusted policy fail.
pub fn python(ctx: &Ctx) -> Result<Vec<Finding>, String> {
    use std::io::Read as _;
    let document = policy(ctx)?;
    let allowed = string_array(&document["python_allowed"]);
    let extensions = string_array(&document["source_extensions_allowed"]);
    let interpreters = string_array(&document["shebang_interpreters_allowed"]);
    let paths = tracked(&ctx.repo)?;
    let heads = |path: &str| {
        let mut bytes = Vec::new();
        std::fs::File::open(ctx.repo.join(path))
            .and_then(|file| file.take(1024).read_to_end(&mut bytes))
            .ok()?;
        // The same decoder as the secrets gate: BOM, UTF-16/32 (ADV23R11-01).
        decoded_head(&bytes).lines().next().map(str::to_owned)
    };
    Ok(check_scripts(
        &paths,
        &allowed,
        &extensions,
        &interpreters,
        &heads,
    ))
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
    if let Some(format) = solver_model(bytes) {
        return Some(format);
    }
    declares_dtd(bytes).then_some(XML_DTD)
}

/// A text encoding, from a byte-order mark or the zero-byte pattern of the
/// first code unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Encoding {
    Utf8,
    Utf16 { big: bool },
    Utf32 { big: bool },
}

/// The encoding of a byte string and the length of its BOM (R11-01,
/// ADV23R11-01, R9-03). Without a BOM, an ASCII-range first character
/// written as UTF-16 or UTF-32 shows as zero bytes beside it; anything else
/// is read as UTF-8.
fn detect_encoding(bytes: &[u8]) -> (Encoding, usize) {
    match bytes {
        [0xef, 0xbb, 0xbf, ..] => (Encoding::Utf8, 3),
        [0xff, 0xfe, 0, 0, ..] => (Encoding::Utf32 { big: false }, 4),
        [0, 0, 0xfe, 0xff, ..] => (Encoding::Utf32 { big: true }, 4),
        [0xff, 0xfe, ..] => (Encoding::Utf16 { big: false }, 2),
        [0xfe, 0xff, ..] => (Encoding::Utf16 { big: true }, 2),
        [a, 0, 0, 0, ..] if *a != 0 => (Encoding::Utf32 { big: false }, 0),
        [0, 0, 0, b, ..] if *b != 0 => (Encoding::Utf32 { big: true }, 0),
        [a, 0, ..] if *a != 0 => (Encoding::Utf16 { big: false }, 0),
        [0, b, ..] if *b != 0 => (Encoding::Utf16 { big: true }, 0),
        _ => (Encoding::Utf8, 0),
    }
}

/// Code units of `width` bytes; a trailing partial unit is dropped.
fn code_units(bytes: &[u8], width: usize, big: bool) -> impl Iterator<Item = u32> + '_ {
    bytes.chunks_exact(width).map(move |chunk| {
        let fold = |acc: u32, b: &u8| (acc << 8) | u32::from(*b);
        if big {
            chunk.iter().fold(0, fold)
        } else {
            chunk.iter().rev().fold(0, fold)
        }
    })
}

/// Decodes text after its BOM, replacing invalid sequences.
fn decode_lossy(bytes: &[u8]) -> String {
    let (encoding, skip) = detect_encoding(bytes);
    let body = &bytes[skip..];
    match encoding {
        Encoding::Utf8 => String::from_utf8_lossy(body).into_owned(),
        Encoding::Utf16 { big } => char::decode_utf16(
            code_units(body, 2, big).map(|unit| u16::try_from(unit).unwrap_or(0xfffd)),
        )
        .map(|c| c.unwrap_or('\u{fffd}'))
        .collect(),
        Encoding::Utf32 { big } => code_units(body, 4, big)
            .map(|unit| char::from_u32(unit).unwrap_or('\u{fffd}'))
            .collect(),
    }
}

/// A whole file decoded as text: UTF-8, UTF-16 or UTF-32, either byte order,
/// with or without BOM (R11-01). `None` unless the file decodes strictly
/// (complete code units, no invalid sequences or unpaired surrogates) to text
/// without control characters other than tab, line feed, carriage return,
/// form feed and escape. Callers treat `None` as data (fail closed).
#[must_use]
pub fn decode_text(bytes: &[u8]) -> Option<String> {
    let (encoding, skip) = detect_encoding(bytes);
    let body = &bytes[skip..];
    let text = match encoding {
        Encoding::Utf8 => std::str::from_utf8(body).ok()?.to_owned(),
        Encoding::Utf16 { big } => {
            if !body.len().is_multiple_of(2) {
                return None;
            }
            char::decode_utf16(
                code_units(body, 2, big).map(|unit| u16::try_from(unit).unwrap_or(0xfffd)),
            )
            .collect::<Result<String, _>>()
            .ok()?
        }
        Encoding::Utf32 { big } => {
            if !body.len().is_multiple_of(4) {
                return None;
            }
            code_units(body, 4, big)
                .map(char::from_u32)
                .collect::<Option<String>>()?
        }
    };
    text.chars()
        .all(|c| !c.is_control() || matches!(c, '\t' | '\n' | '\r' | '\x0c' | '\x1b'))
        .then_some(text)
}

/// The first 64 KiB of a file as text, decoded like [`decode_text`] but
/// replacing invalid sequences, with any BOM removed (ADV23R9-05, R9-03).
fn decoded_head(bytes: &[u8]) -> String {
    decode_lossy(&bytes[..bytes.len().min(64 * 1024)])
}

/// Expands numeric character references. General entities are never
/// expanded: any document with a DTD or entity declaration is data by
/// itself (R10-01, [`declares_dtd`]).
fn expand_references(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(index) = rest.find('&') {
        out.push_str(&rest[..index]);
        rest = &rest[index..];
        let end = rest.find(';').filter(|end| *end <= 40);
        let decoded = end.and_then(|end| {
            let number = rest[..=end].strip_prefix("&#").map(|n| &n[..n.len() - 1])?;
            number
                .strip_prefix(['x', 'X'])
                .map_or_else(
                    || number.parse::<u32>().ok(),
                    |hex| u32::from_str_radix(hex, 16).ok(),
                )
                .and_then(char::from_u32)
                .map(|value| (value, end + 1))
        });
        match decoded {
            Some((value, length)) => {
                out.push(value);
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

/// Data format name for markup with a DTD or entity declaration (R10-01).
pub const XML_DTD: &str = "XML with DTD or entity declarations";

/// Whether a file is markup (its text starts with `<`, in any encoding the
/// sniffer reads) that holds a `<!DOCTYPE` or `<!ENTITY` declaration
/// anywhere in its first 64 KiB (R10-01). Entities can rename the namespaces
/// the RDF check looks for, so such files are data unless the trusted policy
/// lists them in `xml_dtd_allowed`; the sniffer never expands entities.
#[must_use]
pub fn declares_dtd(bytes: &[u8]) -> bool {
    let text = decoded_head(bytes);
    if !text.trim_start().starts_with('<') {
        return false;
    }
    let lower = text.to_lowercase();
    lower.contains("<!doctype") || lower.contains("<!entity")
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
/// after expanding character references, name the RDF syntax
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

/// A file name's extension, lower-cased with its dot (`.rs`); `None` for a
/// name without one or a dotfile such as `.env`.
#[must_use]
pub fn extension(name: &str) -> Option<String> {
    let index = name.rfind('.').filter(|index| *index > 0)?;
    Some(name[index..].to_ascii_lowercase())
}

/// Leading bytes each allowed binary (image) type must start with; an
/// allowed binary type without an entry here never passes.
const BINARY_SIGNATURES: &[(&str, &[u8])] = &[
    (".png", b"\x89PNG\r\n\x1a\n"),
    (".jpg", b"\xff\xd8\xff"),
    (".jpeg", b"\xff\xd8\xff"),
    (".gif", b"GIF8"),
    (".webp", b"RIFF"),
];

/// Data format name for a file that is neither text nor an allowed binary.
pub const UNDECODABLE: &str = "binary or undecodable text";
/// Data format name for an allowed binary type whose content does not match.
pub const BINARY_MISMATCH: &str = "content that does not match its binary type";
/// Data format name for a delimited table in a plain-text file.
pub const DELIMITED: &str = "delimited table";

/// Content-based data classification beyond [`data_signature`] (R11-01,
/// ADV23R11-02): a file of an allowed binary type must carry that type's
/// signature; any other file must decode as text ([`decode_text`]); and a
/// plain-text file (`.txt`, or no extension and no shebang) must not hold a
/// delimited table.
#[must_use]
pub fn content_class(
    name: &str,
    bytes: &[u8],
    decoded: Option<&str>,
    binary_types: &[String],
) -> Option<&'static str> {
    let ext = extension(name);
    if let Some(ext) = ext
        .as_deref()
        .filter(|ext| binary_types.iter().any(|t| t.eq_ignore_ascii_case(ext)))
    {
        let matches = BINARY_SIGNATURES
            .iter()
            .any(|(kind, magic)| *kind == ext && bytes.starts_with(magic))
            && (ext != ".webp" || bytes.get(8..12) == Some(b"WEBP"));
        return (!matches).then_some(BINARY_MISMATCH);
    }
    let Some(text) = decoded else {
        return Some(UNDECODABLE);
    };
    let plain = match ext.as_deref() {
        Some(".txt") => true,
        None => !text.starts_with("#!"),
        Some(_) => false,
    };
    (plain && delimited_table(text)).then_some(DELIMITED)
}

/// Whether text holds a delimited table: five consecutive lines with the
/// same non-zero number of commas, tabs, semicolons or pipes.
#[must_use]
pub fn delimited_table(text: &str) -> bool {
    [',', '\t', ';', '|'].iter().any(|delimiter| {
        let (mut run, mut width) = (0_usize, 0_usize);
        text.lines().any(|line| {
            let count = line.matches(*delimiter).count();
            if count > 0 && count == width {
                run += 1;
            } else {
                width = count;
                run = usize::from(count > 0);
            }
            run >= 5
        })
    })
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
    check_paths_inner(paths, document, records, sniffed, true)
}

/// [`check_paths_with_content`]; without `type_allowlist`, a file is data
/// only when its content is (history scan, R13-03).
fn check_paths_inner(
    paths: &[String],
    document: &DocumentMut,
    records: &[(String, String)],
    sniffed: &BTreeMap<String, &str>,
    type_allowlist: bool,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    // A missing list allows nothing (fail closed).
    let list = |key: &str| document.get(key).map(string_array).unwrap_or_default();
    let text_types = list("tracked_extensions_allowed");
    let binary_types = list("binary_extensions_allowed");
    let names = list("tracked_names_allowed");
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
        let in_samples = sample_paths.iter().any(|glob| glob_match(glob, path));
        // ADV23R11-02: an allowlist of file types, not a list of data
        // extensions. Every other type is data that needs a provenance
        // record or a metadata listing.
        let type_allowed = names.iter().any(|allowed| allowed == name)
            || extension(name).is_some_and(|ext| {
                text_types
                    .iter()
                    .chain(&binary_types)
                    .any(|allowed| allowed.eq_ignore_ascii_case(&ext))
            });
        let is_data = (type_allowlist && !type_allowed) || content.is_some();
        if !is_data || listed_metadata {
            continue;
        }
        if !in_samples {
            findings.push(Finding::at(
                path,
                None,
                match content {
                    Some(format) => format!(
                        "data file ({format} content) outside the allow-listed open-commercial sample paths (D-56)"
                    ),
                    None => "data file outside the allow-listed open-commercial sample paths (D-56): its type is not in the trusted tracked_extensions_allowed; register it as data or ask the policy owner to allow the type"
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
pub(crate) fn line_blob(repo: &std::path::Path, text: &str) -> String {
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

/// Whether a detected block of `path` is a reviewed exemption.
fn exempt_block(
    repo: &std::path::Path,
    exemptions: &[(String, String)],
    path: &str,
    block: &str,
) -> bool {
    let mut blob = None;
    exemptions.iter().any(|(file, wanted)| {
        file == path && blob.get_or_insert_with(|| line_blob(repo, block)) == wanted
    })
}

/// Synthetic example keys quoted in review reports: each names one exact
/// detected block of one file by the Git blob hash of its text, with a
/// reason (trusted policy; no wildcards). A block is the whole lines of a
/// finding and, for a private key header, everything through the end of
/// that key, so a reviewed header never admits a different key body (R13-01).
fn load_secret_exemptions(document: &DocumentMut) -> Result<Vec<(String, String)>, String> {
    let mut found = Vec::new();
    if let Some(tables) = document
        .get("secret_exemption")
        .and_then(Item::as_array_of_tables)
    {
        for table in tables {
            let field = |key: &str| table.get(key).and_then(Item::as_str).map(str::to_owned);
            if table.contains_key("line_blob") {
                return Err(format!(
                    "{POLICY}: secret_exemption.line_blob is no longer accepted; name the whole detected block with block_blob (R13-01)"
                ));
            }
            match (field("path"), field("block_blob"), field("reason")) {
                (Some(path), Some(blob), Some(reason))
                    if !path.contains(['*', '?', '['])
                        && is_sha(&blob)
                        && !reason.trim().is_empty() =>
                {
                    found.push((path, blob));
                }
                _ => {
                    return Err(format!(
                        "{POLICY}: every secret_exemption needs one exact path, block_blob and reason"
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

/// Runs the provenance, secrets and raw-data gate on the checkout.
///
/// # Errors
///
/// Fails if Git, file reads or the trusted policy fail.
pub fn provenance(ctx: &Ctx) -> Result<Vec<Finding>, String> {
    provenance_range(ctx, None)
}

/// A commit range for [`provenance_range`]: `base..head`, or every commit
/// reachable from `head` when there is no base.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct History {
    /// Commits reachable from here are not scanned.
    pub base: Option<String>,
    /// The newest commit of the range.
    pub head: String,
}

/// Runs the provenance, secrets and raw-data gate on the checkout and, with
/// a range, on every file version any commit of the range introduced
/// (R13-03): a secret or data file added and later deleted or replaced
/// within the range still fails.
///
/// # Errors
///
/// Fails if Git, file reads or the trusted policy fail.
pub fn provenance_range(ctx: &Ctx, range: Option<&History>) -> Result<Vec<Finding>, String> {
    let document = policy(ctx)?;
    let paths = tracked(&ctx.repo)?;
    let mut findings = Vec::new();
    let mut records = Vec::new();
    let mut manifests = 0;
    let secret_exemptions = load_secret_exemptions(&document)?;
    let secrets = crate::secrets::Ruleset::load(ctx)?;
    let metadata = string_array(&document["data_metadata_files"]);
    let dtd_allowed = string_array(&document["xml_dtd_allowed"]);
    if document
        .get("tracked_extensions_allowed")
        .and_then(Item::as_array)
        .is_none()
    {
        return Err(format!(
            "{POLICY}: tracked_extensions_allowed is missing; the data gate needs the file-type allowlist (ADV23R11-02)"
        ));
    }
    let binary_types = document
        .get("binary_extensions_allowed")
        .map(string_array)
        .unwrap_or_default();
    let mut sniffed = BTreeMap::new();
    for path in &paths {
        let name = path.rsplit('/').next().unwrap_or(path);
        let bytes =
            std::fs::read(ctx.repo.join(path)).map_err(|error| format!("{path}: {error}"))?;
        if let Some(format) = data_signature(&bytes)
            && !(format == XML_DTD && dtd_allowed.iter().any(|file| file == path))
        {
            sniffed.insert(path.clone(), format);
        }
        // R11-01: secrets, manifests and records are read from the decoded
        // text; a file that is not text and not an allowed binary is data.
        let decoded = decode_text(&bytes);
        if !sniffed.contains_key(path)
            && let Some(format) = content_class(name, &bytes, decoded.as_deref(), &binary_types)
        {
            sniffed.insert(path.clone(), format);
        }
        let text = decoded.unwrap_or_else(|| decode_lossy(&bytes));
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
        // R12-01: secrets are searched in every text view of the raw bytes,
        // not only the decoded text, so no encoding choice hides one.
        let pinned = secrets.is_pinned_copy(path, || {
            std::str::from_utf8(&bytes)
                .map_or_else(|_| String::new(), |text| line_blob(&ctx.repo, text))
        });
        if !pinned {
            let exempt = |block: &str| exempt_block(&ctx.repo, &secret_exemptions, path, block);
            for hit in secrets.scan_with(path, &bytes, &exempt) {
                findings.push(Finding::at(path, hit.line, hit.message));
            }
        }
    }
    if manifests == 0 {
        findings.push(Finding::general(
            "no provenance manifest found; the gate would be vacuous",
        ));
    }
    findings.extend(check_paths_with_content(
        &paths, &document, &records, &sniffed,
    ));
    if let Some(range) = range {
        let scan = HistoryScan {
            document: &document,
            records: &records,
            secrets: &secrets,
            exemptions: &secret_exemptions,
            dtd_allowed: &dtd_allowed,
        };
        findings.extend(scan.run(ctx, range)?);
    }
    Ok(findings)
}

/// The trusted inputs a historical scan shares with the checkout scan.
struct HistoryScan<'a> {
    document: &'a DocumentMut,
    records: &'a [(String, String)],
    secrets: &'a crate::secrets::Ruleset,
    exemptions: &'a [(String, String)],
    dtd_allowed: &'a [String],
}

impl HistoryScan<'_> {
    fn run(&self, ctx: &Ctx, range: &History) -> Result<Vec<Finding>, String> {
        let repo = ctx.repo.as_path();
        let commit = |revision: &str| -> Result<String, String> {
            let spec = format!("{revision}^{{commit}}");
            let out = crate::util::git(
                repo,
                &[
                    "rev-parse",
                    "--verify",
                    "--quiet",
                    "--end-of-options",
                    &spec,
                ],
            )
            .map_err(|_| format!("history scan: {revision} is not a commit"))?;
            Ok(String::from_utf8_lossy(&out).trim().to_owned())
        };
        let head = commit(&range.head)?;
        let selector = match &range.base {
            Some(base) => format!("{}..{head}", commit(base)?),
            None => head.clone(),
        };
        // Objects the range introduced: reachable from head, not from base.
        let objects = crate::util::git(
            repo,
            &["rev-list", "--objects", "--no-object-names", &selector],
        )?;
        let new_objects: BTreeSet<String> = String::from_utf8_lossy(&objects)
            .split_whitespace()
            .map(str::to_owned)
            .collect();
        // File versions at head are the checkout, scanned already.
        let mut at_head = BTreeSet::new();
        let listing = crate::util::git(repo, &["ls-tree", "-r", "-z", "--full-tree", &head])?;
        for entry in listing.split(|b| *b == 0).filter(|e| !e.is_empty()) {
            let entry = String::from_utf8_lossy(entry);
            if let Some((meta, path)) = entry.split_once('\t')
                && let Some(blob) = meta.split(' ').nth(2)
            {
                at_head.insert((path.to_owned(), blob.to_owned()));
            }
        }
        // Every (path, blob) any commit added or changed, against each parent.
        let commits = crate::util::git(repo, &["rev-list", &selector])?;
        let mut versions: BTreeMap<(String, String), String> = BTreeMap::new();
        for sha in String::from_utf8_lossy(&commits).split_whitespace() {
            let raw = crate::util::git(
                repo,
                &[
                    "diff-tree",
                    "-r",
                    "-z",
                    "-m",
                    "--root",
                    "--no-renames",
                    "--no-commit-id",
                    "--raw",
                    "--no-abbrev",
                    sha,
                ],
            )?;
            let mut fields = raw.split(|b| *b == 0);
            while let Some(meta) = fields.next() {
                let meta = String::from_utf8_lossy(meta);
                let Some(meta) = meta.strip_prefix(':') else {
                    continue;
                };
                let Some(path) = fields.next() else { break };
                let parts: Vec<&str> = meta.split(' ').collect();
                let (Some(mode), Some(blob), Some(status)) =
                    (parts.get(1), parts.get(3), parts.get(4))
                else {
                    continue;
                };
                if status.starts_with('D') || *mode == "160000" {
                    continue;
                }
                let key = (
                    String::from_utf8_lossy(path).into_owned(),
                    (*blob).to_owned(),
                );
                versions.entry(key).or_insert_with(|| sha[..12].to_owned());
            }
        }
        let mut findings = Vec::new();
        let mut paths = Vec::new();
        let mut sniffed = BTreeMap::new();
        let mut origin: BTreeMap<String, String> = BTreeMap::new();
        for ((path, blob), sha) in &versions {
            if at_head.contains(&(path.clone(), blob.clone())) {
                continue;
            }
            paths.push(path.clone());
            origin.entry(path.clone()).or_insert_with(|| sha.clone());
            if !new_objects.contains(blob) {
                continue;
            }
            let bytes = crate::util::git(repo, &["cat-file", "blob", blob])?;
            // Recognised data formats only: the file-type allowlist and the
            // binary/undecodable class apply to the checkout, not history.
            let format = data_signature(&bytes).filter(|format| {
                !(*format == XML_DTD && self.dtd_allowed.iter().any(|f| f == path))
            });
            if let Some(format) = format {
                sniffed.insert(path.clone(), format);
            }
            if self.secrets.is_pinned_copy(path, || blob.clone()) {
                continue;
            }
            let exempt = |block: &str| exempt_block(repo, self.exemptions, path, block);
            for hit in self.secrets.scan_with(path, &bytes, &exempt) {
                findings.push(Finding::at(
                    path.clone(),
                    hit.line,
                    format!("in commit {sha} (blob {}): {}", &blob[..12], hit.message),
                ));
            }
        }
        paths.sort();
        paths.dedup();
        for finding in check_paths_inner(&paths, self.document, self.records, &sniffed, false) {
            let sha = finding
                .file
                .as_ref()
                .and_then(|file| origin.get(file))
                .cloned()
                .unwrap_or_default();
            findings.push(Finding {
                message: format!("in commit {sha}: {}", finding.message),
                ..finding
            });
        }
        Ok(findings)
    }
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
        let found = check_scripts(
            &paths(&[
                "ci/check.py",
                "spikes/x/y.py",
                "tools/reference-validators/v.py",
                "xtask/src/main.rs",
                "a/notebook.ipynb",
            ]),
            &allowed,
            &[],
            &[],
            &none,
        );
        assert_eq!(found.len(), 2);
        let shebang = |_: &str| Some("#!/usr/bin/env python3".to_owned());
        assert_eq!(
            check_scripts(&paths(&["ci/check"]), &allowed, &[], &[], &shebang).len(),
            1
        );
    }

    // ADV23-11 / C2: tracked bytecode is rejected anywhere.
    #[test]
    fn adv23_11_bytecode_is_rejected() {
        let allowed = paths(&["spikes/"]);
        let none = |_: &str| None;
        let found = check_scripts(
            &paths(&[
                "ci/__pycache__/x.cpython-313.pyc",
                "spikes/a/__pycache__/b.pyc",
            ]),
            &allowed,
            &[],
            &[],
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
        let document: DocumentMut = "tracked_extensions_allowed = [\".txt\", \".md\"]\nopen_commercial_data = [\"spikes/data/samples/*.csv\"]\ndata_metadata_files = [\"spikes/data/sources.tsv\"]\n".parse().expect("toml");
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
                ".env",
                "k/server.pem",
                "k/server.pem",
                ".env.example"
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
        let document: DocumentMut = "tracked_extensions_allowed = [\".txt\", \".md\"]\nopen_commercial_data = [\"spikes/data/samples/*\"]\ndata_metadata_files = []\n".parse().expect("toml");
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
            assert_eq!(
                check_scripts(&paths(&[path]), &[], &[], &[], &none).len(),
                1,
                "{path}"
            );
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
        // Entities are not expanded (R10-01): this one is still data, by the
        // character reference in its literal text, and by its DTD alone.
        assert!(data_signature(entity.as_bytes()).is_some());
        assert!(declares_dtd(entity.as_bytes()));
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

    fn secret_rules() -> &'static crate::secrets::Ruleset {
        static RULES: std::sync::OnceLock<crate::secrets::Ruleset> = std::sync::OnceLock::new();
        RULES.get_or_init(|| {
            crate::secrets::Ruleset::parse(&[
                ("gitleaks.toml", include_str!("../policy/gitleaks.toml")),
                (
                    "secret-rules.toml",
                    include_str!("../policy/secret-rules.toml"),
                ),
            ])
            .expect("ruleset")
        })
    }

    fn scan_bytes(bytes: &[u8]) -> Vec<crate::secrets::Hit> {
        secret_rules().scan("f", bytes)
    }

    fn scan_secrets(path: &str, text: &str) -> Vec<crate::secrets::Hit> {
        secret_rules().scan(path, text.as_bytes())
    }

    fn trusted_policy() -> DocumentMut {
        std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("policy/repository.toml"),
        )
        .expect("policy")
        .parse()
        .expect("toml")
    }

    // R10-01: the GPT node-root fixture hides the RDF namespace behind a
    // nested entity, which the once-only expansion at 88148f0 missed. Any
    // markup with a DTD or entity declaration is now data, unexpanded.
    #[test]
    fn r10_01_dtd_and_entity_declarations_are_data() {
        let gpt = "<!DOCTYPE r:Description [\n<!ENTITY tail \"rdf-&#x73;yntax-ns#\">\n<!ENTITY ns \"http://www.w3.org/1999/02/22-&tail;\">\n]>\n<r:Description xmlns:r=\"&ns;\" r:about=\"urn:grid:bus:1\">\n  <bus:nominalVoltage xmlns:bus=\"urn:grid:terms:\">400</bus:nominalVoltage>\n</r:Description>\n";
        let literal = gpt.replace(
            "xmlns:r=\"&ns;\"",
            "xmlns:r=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"",
        );
        let cycle = "<!DOCTYPE a [<!ENTITY a \"&b;\"><!ENTITY b \"&a;\">]>\n<a>&a;</a>\n";
        let lol = format!(
            "<?xml version=\"1.0\"?>\n<!DOCTYPE x [\n<!ENTITY l0 \"l\">\n{}]>\n<x>&l9;</x>\n",
            (1..10)
                .map(|n| format!(
                    "<!ENTITY l{n} \"{}\">\n",
                    format!("&l{};", n - 1).repeat(10)
                ))
                .collect::<String>()
        );
        let external = "<!DOCTYPE x SYSTEM \"http://example.org/x.dtd\">\n<x/>\n";
        let html = "<!doctype html>\n<html><body>grid</body></html>\n";
        let entity_only = "<x>\n<!ENTITY ns \"urn:x\">\n</x>\n";
        let utf16: Vec<u8> = [0xff, 0xfe]
            .into_iter()
            .chain(gpt.encode_utf16().flat_map(u16::to_le_bytes))
            .collect();
        for bytes in [
            gpt.as_bytes(),
            cycle.as_bytes(),
            lol.as_bytes(),
            external.as_bytes(),
            html.as_bytes(),
            entity_only.as_bytes(),
            &utf16,
        ] {
            assert_eq!(data_signature(bytes), Some(XML_DTD), "{bytes:?}");
        }
        assert_eq!(data_signature(literal.as_bytes()), Some("CIM/RDF XML"));
        let document = trusted_policy();
        let path = "spikes/network-notes.txt".to_owned();
        let sniffed = BTreeMap::from([(path.clone(), XML_DTD)]);
        let found = check_paths_with_content(&[path], &document, &[], &sniffed);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].message.contains("D-56"));
        // Controls: Markdown and code that quote a DTD, and markup without one.
        for text in [
            format!("# Review\n\n```xml\n{gpt}```\n"),
            "let dtd = \"<!DOCTYPE x [<!ENTITY a \\\"b\\\">]>\";\n".to_owned(),
            "<?xml version=\"1.0\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\"/>\n".to_owned(),
            "<html><body>no doctype</body></html>".to_owned(),
        ] {
            assert_eq!(data_signature(text.as_bytes()), None, "{text}");
        }
    }

    // ADV23R10-01: solver solution, MIP start, basis, infeasibility and
    // pseudo-Boolean files passed the data gate at 88148f0.
    #[test]
    fn adv23r10_01_solver_solutions_are_data() {
        let document = trusted_policy();
        for path in [
            "spikes/dispatch.sol",
            "spikes/unit_commitment.mst",
            "spikes/warm.bas",
            "spikes/conflict.ilp",
            "spikes/model.sav",
            "spikes/instance.opb",
            "spikes/instance.wbo",
            "spikes/uc.dat",
            "spikes/uc.mod",
            "spikes/DISPATCH.SOL",
        ] {
            assert_eq!(
                check_paths(&paths(&[path]), &document, &[]).len(),
                1,
                "{path}"
            );
        }
        // Control: ordinary source and documentation are not data.
        for path in ["crates/solver/src/solution.rs", "docs/solutions.md"] {
            assert!(check_paths(&paths(&[path]), &document, &[]).is_empty());
        }
    }

    // ADV23R10-02 (D-73): Julia and every other non-Rust script or source
    // language passed outside spikes/ at 88148f0; only Python was checked.
    #[test]
    fn adv23r10_02_script_languages_are_rejected() {
        let document = trusted_policy();
        let allowed = string_array(&document["python_allowed"]);
        let extensions = string_array(&document["source_extensions_allowed"]);
        let interpreters = string_array(&document["shebang_interpreters_allowed"]);
        let check = |path: &str, head: Option<&str>| {
            let head = head.map(str::to_owned);
            check_scripts(
                &paths(&[path]),
                &allowed,
                &extensions,
                &interpreters,
                &|_| head.clone(),
            )
        };
        for path in [
            "crates/validator.jl",
            "crates/grid/check.R",
            "tools/case.m",
            "ci/run.js",
            "ci/run.mjs",
            "ci/run.ts",
            "ci/run.rb",
            "ci/run.pl",
            "ci/run.lua",
            "ci/run.ps1",
            "ci/run.php",
            "ci/run.go",
            "ci/native.c",
            "ci/native.cpp",
            "ci/run.zsh",
            "ci/check.py",
        ] {
            assert_eq!(check(path, None).len(), 1, "{path}");
        }
        for (path, head) in [
            ("ci/run", "#!/usr/bin/env julia"),
            ("ci/run.txt", "#!/usr/bin/env -S julia --startup-file=no"),
            ("ci/run", "#!/usr/bin/Rscript"),
            ("ci/run", "#!/usr/bin/env node"),
            ("ci/run", "#!/usr/bin/env FOO=1 perl"),
            ("ci/run.sh", "#! /usr/bin/python3"),
        ] {
            assert_eq!(check(path, Some(head)).len(), 1, "{path}: {head}");
        }
        // Controls: Rust (including an inner attribute on the first line),
        // policy files, documentation, sh/bash hooks, and any language in the
        // allowed directories.
        for (path, head) in [
            ("crates/grid/src/lib.rs", Some("#![forbid(unsafe_code)]")),
            ("Cargo.toml", None),
            (".github/workflows/ci.yml", None),
            ("docs/a.md", None),
            (".githooks/commit-msg", Some("#!/bin/sh")),
            ("ci/load.sh", Some("#!/usr/bin/env bash")),
            ("spikes/x/validate.jl", Some("#!/usr/bin/env julia")),
            ("tools/reference-validators/v.R", None),
            ("docs/notes.txt", Some("# heading")),
        ] {
            assert!(check(path, head).is_empty(), "{path}");
        }
        assert_eq!(shebang_interpreter("#![deny(warnings)]"), None);
        assert_eq!(
            shebang_interpreter("#!/bin/bash -e").as_deref(),
            Some("bash")
        );
    }

    fn encodings(text: &str) -> Vec<(&'static str, Vec<u8>)> {
        let utf16 = |big: bool| -> Vec<u8> {
            text.encode_utf16()
                .flat_map(|u| {
                    if big {
                        u.to_be_bytes()
                    } else {
                        u.to_le_bytes()
                    }
                })
                .collect()
        };
        let utf32 = |big: bool| -> Vec<u8> {
            text.chars()
                .flat_map(|c| {
                    let u = u32::from(c);
                    if big {
                        u.to_be_bytes()
                    } else {
                        u.to_le_bytes()
                    }
                })
                .collect()
        };
        let bom = |b: &[u8], body: Vec<u8>| [b.to_vec(), body].concat();
        vec![
            ("utf-8 bom", bom(b"\xef\xbb\xbf", text.as_bytes().to_vec())),
            ("utf-16le bom", bom(b"\xff\xfe", utf16(false))),
            ("utf-16be bom", bom(b"\xfe\xff", utf16(true))),
            ("utf-16le", utf16(false)),
            ("utf-16be", utf16(true)),
            ("utf-32le bom", bom(b"\xff\xfe\x00\x00", utf32(false))),
            ("utf-32be bom", bom(b"\x00\x00\xfe\xff", utf32(true))),
            ("utf-32le", utf32(false)),
            ("utf-32be", utf32(true)),
        ]
    }

    // R11-01: a recognised secret written as UTF-16/32 text passed the
    // secrets gate at d71cf40; files are now decoded before scanning, and a
    // file that is not decodable text is data (fail closed).
    #[test]
    fn r11_01_encoded_text_is_decoded_before_secret_scan() {
        let key = ["gh", "p_", &"0".repeat(36)].concat();
        let line = format!("service_token={key}\n");
        assert_eq!(decode_text(line.as_bytes()).as_deref(), Some(line.as_str()));
        for (name, bytes) in encodings(&line) {
            let text = decode_text(&bytes).unwrap_or_else(|| panic!("{name}"));
            assert_eq!(text, line, "{name}");
            assert_eq!(scan_bytes(&bytes).len(), 1, "{name}");
        }
        // A signature after 64 KiB is still found.
        let long = format!("{}{line}", "a\n".repeat(40 * 1024));
        let bytes: Vec<u8> = long.encode_utf16().flat_map(u16::to_le_bytes).collect();
        assert!(bytes.len() > 64 * 1024);
        assert_eq!(scan_bytes(&bytes).len(), 1);
        // Not confidently text: invalid UTF-8, odd UTF-16, unpaired
        // surrogates, NUL and other control bytes.
        for bytes in [
            &b"ab\x80\x81\xfe"[..],
            b"a\x00b",
            b"\xff\xfea\x00b",
            b"\xff\xfe\x00\xd8a\x00",
            b"text\x00\x00\x00\x00more",
            b"bell\x07",
        ] {
            assert_eq!(decode_text(bytes), None, "{bytes:?}");
            assert_eq!(
                content_class("docs/a.txt", bytes, None, &[]),
                Some(UNDECODABLE)
            );
        }
        let document = trusted_policy();
        let path = "docs/notes.txt".to_owned();
        let sniffed = BTreeMap::from([(path.clone(), UNDECODABLE)]);
        assert_eq!(
            check_paths_with_content(&[path], &document, &[], &sniffed).len(),
            1
        );
        // Controls: ordinary text in every encoding decodes and has no secret.
        for (name, bytes) in encodings("# Notes\n\tgrid é 電\r\n") {
            let text = decode_text(&bytes).unwrap_or_else(|| panic!("{name}"));
            assert!(scan_bytes(&bytes).is_empty(), "{name}");
            assert_eq!(content_class("docs/a.md", &bytes, Some(&text), &[]), None);
        }
    }

    // ADV23R11-01: a UTF-8 BOM before a shebang hid the interpreter at d71cf40.
    #[test]
    fn adv23r11_01_bom_before_shebang_is_read() {
        let document = trusted_policy();
        let allowed = string_array(&document["python_allowed"]);
        let extensions = string_array(&document["source_extensions_allowed"]);
        let interpreters = string_array(&document["shebang_interpreters_allowed"]);
        for interpreter in ["python3", "julia"] {
            let script = format!("#!/usr/bin/env {interpreter}\nprint(1)\n");
            for (name, bytes) in encodings(&script) {
                let head = decoded_head(&bytes).lines().next().map(str::to_owned);
                assert_eq!(
                    head.as_deref().and_then(shebang_interpreter).as_deref(),
                    Some(interpreter),
                    "{name}"
                );
                let found = check_scripts(
                    &paths(&["crates/run_tool"]),
                    &allowed,
                    &extensions,
                    &interpreters,
                    &|_| head.clone(),
                );
                assert_eq!(found.len(), 1, "{name}");
            }
        }
        assert_eq!(
            shebang_interpreter("\u{feff}\u{feff}#!/bin/sh").as_deref(),
            Some("sh")
        );
        // Controls: a BOM before a Rust attribute or ordinary text.
        assert_eq!(shebang_interpreter("\u{feff}#![forbid(unsafe_code)]"), None);
        assert_eq!(shebang_interpreter("\u{feff}# heading"), None);
    }

    // ADV23R11-02: data types missing from the old data-extension list
    // passed at d71cf40. Every type outside the trusted allowlist is data.
    #[test]
    fn adv23r11_02_file_types_are_an_allowlist() {
        let document = trusted_policy();
        for path in [
            "spikes/conflict.iis",
            "spikes/model.svf",
            "spikes/problem.cnf",
            "spikes/problem.smt2",
            "spikes/problem.smt",
            "crates/solve.run",
            "crates/run_tool",
            "docs/table.unknownext",
            "docs/.hidden",
            "docs/a.json",
            "docs/a.csv",
        ] {
            let found = check_paths(&paths(&[path]), &document, &[]);
            assert_eq!(found.len(), 1, "{path}: {found:?}");
            assert!(found[0].message.contains("D-56"), "{found:?}");
        }
        // A missing allowlist allows nothing.
        let empty: DocumentMut = "open_commercial_data = []\ndata_metadata_files = []\n"
            .parse()
            .expect("toml");
        assert_eq!(
            check_paths(&paths(&["crates/a/src/lib.rs"]), &empty, &[]).len(),
            1
        );
        // Content still decides under allowed types: a table in a .txt or an
        // extensionless file, and an image whose bytes are not that image.
        let binary = string_array(&document["binary_extensions_allowed"]);
        let table = "a,b,c\n1,2,3\n4,5,6\n7,8,9\n10,11,12\n";
        assert_eq!(
            content_class("docs/t.txt", table.as_bytes(), Some(table), &binary),
            Some(DELIMITED)
        );
        assert_eq!(
            content_class("NOTICE", table.as_bytes(), Some(table), &binary),
            Some(DELIMITED)
        );
        let tsv = table.replace(',', "\t");
        assert_eq!(
            content_class("docs/t.txt", tsv.as_bytes(), Some(&tsv), &binary),
            Some(DELIMITED)
        );
        assert_eq!(
            content_class("docs/fig.png", table.as_bytes(), None, &binary),
            Some(BINARY_MISMATCH)
        );
        assert_eq!(
            content_class("docs/fig.webp", b"RIFF\0\0\0\0WAVE", None, &binary),
            Some(BINARY_MISMATCH)
        );
        // Controls: allowed source, docs, lock files, hooks, licences, real
        // images, shell scripts with pipes, and short prose lists.
        for path in [
            "crates/a/src/lib.rs",
            "Cargo.lock",
            "docs/a.md",
            ".githooks/commit-msg",
            ".githooks/attribution-patterns.regex",
            ".github/CODEOWNERS",
            ".gitignore",
            "LICENSE",
            "docs/fig.png",
            "docs/FIG.PNG",
            "docs/diagram.svg",
            "spikes/x/requirements.txt",
            "spikes/x/fetch.py",
        ] {
            assert!(
                check_paths(&paths(&[path]), &document, &[]).is_empty(),
                "{path}"
            );
        }
        assert_eq!(
            content_class("docs/fig.png", b"\x89PNG\r\n\x1a\n\0\0", None, &binary),
            None
        );
        assert_eq!(
            content_class("docs/fig.webp", b"RIFF\0\0\0\0WEBPVP8 ", None, &binary),
            None
        );
        let hook = "#!/bin/sh\na | b\nc | d\ne | f\ng | h\ni | j\n";
        assert_eq!(
            content_class("commit-msg", hook.as_bytes(), Some(hook), &binary),
            None
        );
        let prose = "One, two.\nThree, four.\nFive.\n";
        assert_eq!(
            content_class("docs/n.txt", prose.as_bytes(), Some(prose), &binary),
            None
        );
        assert_eq!(
            content_class("docs/t.md", table.as_bytes(), Some(table), &binary),
            None
        );
        assert_eq!(extension(".env"), None);
        assert_eq!(extension("a.TAR.GZ").as_deref(), Some(".gz"));
    }
}
