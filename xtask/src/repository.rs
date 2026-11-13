//! Repository content policy.
//!
//! - `python` (D-73): no Python outside `spikes/` and the allow-listed offline
//!   reference-validator directories; no tracked bytecode (ADV23-11).
//! - `provenance` (D-56, ADR 0001 `provenance/secrets/data`, ADV23-04): every
//!   dataset in a provenance manifest is tagged `open-commercial` or
//!   `research-only`; share-alike data is never used; research-only files are
//!   never committed; data files exist only under allow-listed open-commercial
//!   sample paths; no secrets, key files or `raw/` directories are tracked.
//! - `codeowners`: every protected policy path is owned by the policy owner.

use toml_edit::{DocumentMut, Item};

use crate::util::{Ctx, Finding, glob_match, string_array, tracked};

/// Trusted repository policy.
pub const POLICY: &str = "xtask/policy/repository.toml";

fn policy(ctx: &Ctx) -> Result<DocumentMut, String> {
    ctx.policy_toml(POLICY)
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
        if name.ends_with(".pyc")
            || name.ends_with(".pyo")
            || path.split('/').any(|part| part == "__pycache__")
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
            .any(|ext| name.ends_with(ext));
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

/// Datasets declared by one manifest: (name, status, file names, licence text).
fn manifest_datasets(document: &DocumentMut) -> Vec<(String, Option<String>, Vec<String>, String)> {
    let mut found = Vec::new();
    let describe = |name: String, table: &dyn toml_edit::TableLike| {
        let field = |key: &str| table.get(key).and_then(Item::as_str).map(str::to_owned);
        let files = ["filename", "relative_path", "local_name"]
            .iter()
            .filter_map(|key| field(key))
            .collect();
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
                if status == "research-only" {
                    restricted.extend(files);
                }
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
#[must_use]
pub fn check_sources_table(path: &str, text: &str) -> (Vec<Finding>, Vec<String>) {
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
        if clearance == "research-only"
            && let Some(name) = cells.get(1)
        {
            restricted.push((*name).to_owned());
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
    },
    SecretRule {
        name: "AWS access key",
        prefix: "ASIA",
        min_tail: 16,
        tail: upper_digit,
    },
    SecretRule {
        name: "GitHub token",
        prefix: "ghp_",
        min_tail: 36,
        tail: token_char,
    },
    SecretRule {
        name: "GitHub token",
        prefix: "gho_",
        min_tail: 36,
        tail: token_char,
    },
    SecretRule {
        name: "GitHub token",
        prefix: "ghu_",
        min_tail: 36,
        tail: token_char,
    },
    SecretRule {
        name: "GitHub token",
        prefix: "ghs_",
        min_tail: 36,
        tail: token_char,
    },
    SecretRule {
        name: "GitHub token",
        prefix: "ghr_",
        min_tail: 36,
        tail: token_char,
    },
    SecretRule {
        name: "GitHub token",
        prefix: "github_pat_",
        min_tail: 40,
        tail: token_char,
    },
    SecretRule {
        name: "Slack token",
        prefix: "xoxb-",
        min_tail: 20,
        tail: token_char,
    },
    SecretRule {
        name: "Slack token",
        prefix: "xoxp-",
        min_tail: 20,
        tail: token_char,
    },
    SecretRule {
        name: "Google API key",
        prefix: "AIza",
        min_tail: 35,
        tail: token_char,
    },
    SecretRule {
        name: "OpenAI-style key",
        prefix: "sk-proj-",
        min_tail: 32,
        tail: token_char,
    },
    SecretRule {
        name: "Anthropic-style key",
        prefix: "sk-ant-",
        min_tail: 32,
        tail: token_char,
    },
    SecretRule {
        name: "Stripe key",
        prefix: "sk_live_",
        min_tail: 20,
        tail: token_char,
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
        for rule in SECRET_RULES {
            let mut rest = line;
            while let Some(position) = rest.find(rule.prefix) {
                let before_ok = position == 0
                    || !rest[..position].ends_with(|c: char| c.is_ascii_alphanumeric());
                let tail: String = rest[position + rule.prefix.len()..]
                    .chars()
                    .take_while(|c| (rule.tail)(*c))
                    .collect();
                if before_ok && tail.len() >= rule.min_tail {
                    findings.push(Finding::at(
                        path,
                        Some(index + 1),
                        format!("possible {} committed", rule.name),
                    ));
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

/// Checks tracked paths for data and key files.
#[must_use]
pub fn check_paths(
    paths: &[String],
    document: &DocumentMut,
    restricted: &[String],
) -> Vec<Finding> {
    let mut findings = Vec::new();
    let data_extensions = string_array(&document["data_extensions"]);
    let sample_paths = string_array(&document["open_commercial_data"]);
    let metadata = string_array(&document["data_metadata_files"]);
    for path in paths {
        let name = path.rsplit('/').next().unwrap_or(path);
        let lower = name.to_ascii_lowercase();
        if path.split('/').any(|part| part == "raw") {
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
        if restricted.iter().any(|file| {
            let base = file.rsplit('/').next().unwrap_or(file);
            !base.is_empty() && base == name
        }) {
            findings.push(Finding::at(
                path,
                None,
                "research-only dataset file is committed (D-56)",
            ));
        }
        let is_data = data_extensions
            .iter()
            .any(|ext| lower.ends_with(ext.as_str()));
        let allowed = sample_paths.iter().any(|glob| glob_match(glob, path))
            || metadata.iter().any(|file| file == path);
        if is_data && !allowed {
            findings.push(Finding::at(
                path,
                None,
                "data file outside the allow-listed open-commercial sample paths (D-56)",
            ));
        }
    }
    findings
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
    let mut restricted = Vec::new();
    let mut manifests = 0;
    for path in &paths {
        let name = path.rsplit('/').next().unwrap_or(path);
        let bytes =
            std::fs::read(ctx.repo.join(path)).map_err(|error| format!("{path}: {error}"))?;
        let text = String::from_utf8_lossy(&bytes);
        if name == "manifest.toml" {
            manifests += 1;
            let (found, files) = check_manifest(path, &text);
            findings.extend(found);
            restricted.extend(files);
        }
        if name == "sources.tsv" {
            let (found, files) = check_sources_table(path, &text);
            findings.extend(found);
            restricted.extend(files);
        }
        findings.extend(scan_secrets(path, &text));
    }
    if manifests == 0 {
        findings.push(Finding::general(
            "no provenance manifest found; the gate would be vacuous",
        ));
    }
    findings.extend(check_paths(&paths, &document, &restricted));
    Ok(findings)
}

/// Checks that CODEOWNERS assigns every protected path to the owner.
///
/// # Errors
///
/// Fails if the trusted policy cannot be read.
pub fn codeowners(ctx: &Ctx) -> Result<Vec<Finding>, String> {
    let document = policy(ctx)?;
    let owner = document
        .get("policy_owner")
        .and_then(Item::as_str)
        .ok_or("trusted policy: policy_owner missing")?;
    let protected = string_array(&document["protected_paths"]);
    let text = std::fs::read_to_string(ctx.repo.join(".github/CODEOWNERS")).unwrap_or_default();
    Ok(check_codeowners(&text, owner, &protected))
}

/// Pure CODEOWNERS comparison.
#[must_use]
pub fn check_codeowners(text: &str, owner: &str, protected: &[String]) -> Vec<Finding> {
    let handle = format!("@{owner}");
    let rules: Vec<(String, bool)> = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let pattern = parts.next()?.trim_start_matches('/').to_owned();
            Some((pattern, parts.any(|o| o.eq_ignore_ascii_case(&handle))))
        })
        .collect();
    let mut findings = Vec::new();
    for path in protected {
        let wanted = path.trim_start_matches('/');
        let last = rules
            .iter()
            .rev()
            .find(|(pattern, _)| pattern == wanted || pattern == "*");
        if !matches!(last, Some((_, true))) {
            findings.push(Finding::at(
                ".github/CODEOWNERS",
                None,
                format!("{wanted} must be owned by {handle}"),
            ));
        }
    }
    findings
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
        let restricted = paths(&["etys.xlsx"]);
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
            &restricted,
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
}
