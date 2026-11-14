//! Rust source and manifest policy (ADR 0001, NFR-10): the cheap static rules.
//!
//! What Cargo compiles is never inferred here from manifests, globs or source
//! text. It is read from `cargo metadata` (resolved packages, targets, path
//! dependencies, patches and build-dependencies), and the compiler itself
//! enforces the unsafe boundary and the native-runtime ban over the real build
//! (`build_gate`, run in the unprivileged CI job). This module keeps:
//!
//! - `unsafe-scope`: every target (library, binaries, tests, examples, benches,
//!   build script) of every local package that is not a reviewed adapter, as
//!   `cargo metadata` lists it, starts with `#![forbid(unsafe_code)]` (F4,
//!   R3-01), so ordinary builds reject an opt-out too; every library root also
//!   carries `#![doc(test(attr(forbid(unsafe_code))))]`, because rustdoc builds
//!   each doctest as a separate crate that does not inherit the root lint
//!   (R6-04). Reviewed adapters are an explicit list of package names and
//!   directories in the trusted `repository.toml` (ADV23R6-01), never a
//!   directory prefix; their build scripts are never exempt. Every local package and
//!   target file is a tracked file inside the repository; dependencies come
//!   from crates.io or the repository only; a tracked package outside
//!   `spikes/` that the workspace does not build is reported.
//! - `manifests`: the workspace keeps `unsafe_code` at deny/forbid; every
//!   package inherits workspace lints and nothing else (ADV23-06); every package
//!   inherits the workspace licence (ADV23-13); a Cargo configuration file
//!   (`config` or `config.toml` in any `.cargo/` directory) may hold only
//!   `[alias]` entries (ADV23R-06).

use syn::ext::IdentExt as _;
use toml_edit::{DocumentMut, Item};

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::util::{Ctx, Finding, tracked};

const ENFORCING: &[&str] = &["deny", "forbid"];

/// The registry sources a dependency may come from.
const REGISTRIES: &[&str] = &[
    "registry+https://github.com/rust-lang/crates.io-index",
    "sparse+https://index.crates.io/",
];

fn in_scope(path: &str) -> bool {
    !path.starts_with("spikes/")
}

/// A reviewed native solver adapter from the trusted policy: an exact Cargo
/// package name at an exact repository directory (ADV23R6-01).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Adapter {
    /// Cargo package name.
    pub package: String,
    /// Repository-relative package directory.
    pub path: String,
}

/// Trusted policy file that lists the reviewed adapters.
pub const REPOSITORY_POLICY: &str = "xtask/policy/repository.toml";

/// Loads the reviewed adapters (`[[adapter]]` in the trusted
/// `repository.toml`). The list is the only way a package may allow unsafe
/// code; changing it changes a protected path, so it needs the policy owner's
/// exact-head approval (D-74).
///
/// # Errors
///
/// Fails if the policy is missing or an entry is malformed.
pub fn adapters(ctx: &Ctx) -> Result<Vec<Adapter>, String> {
    parse_adapters(&ctx.policy_toml(REPOSITORY_POLICY)?)
}

/// Parses `[[adapter]]` tables.
///
/// # Errors
///
/// Fails on an entry without an exact package name and plain relative path.
pub fn parse_adapters(document: &DocumentMut) -> Result<Vec<Adapter>, String> {
    let mut out = Vec::new();
    let Some(item) = document.get("adapter") else {
        return Ok(out);
    };
    let tables = item
        .as_array_of_tables()
        .ok_or("trusted policy: `adapter` must be an array of tables")?;
    for table in tables {
        let field = |key: &str| table.get(key).and_then(Item::as_str).map(str::to_owned);
        match (field("package"), field("path")) {
            (Some(package), Some(path))
                if !package.is_empty()
                    && !path.is_empty()
                    && !path.contains(['*', '?', '[', '\\'])
                    && !path.starts_with('/')
                    && path
                        .split('/')
                        .all(|part| !part.is_empty() && part != "." && part != "..") =>
            {
                out.push(Adapter { package, path });
            }
            _ => {
                return Err(
                    "trusted policy: every [[adapter]] needs an exact package name and a plain relative path"
                        .to_owned(),
                );
            }
        }
    }
    Ok(out)
}

/// Whether a local package (name, repository-relative directory) is a
/// reviewed adapter. Both the name and the directory must match an entry.
#[must_use]
pub fn is_adapter(adapters: &[Adapter], name: &str, dir: &str) -> bool {
    adapters
        .iter()
        .any(|adapter| adapter.package == name && adapter.path == dir)
}

/// Environment variables a policy-run Cargo never inherits: anything that can
/// change the compiler, its flags, wrappers, linkers, lint caps or the target
/// directory (ADV23R5-01). `CARGO_HOME` and `CARGO_NET_OFFLINE` are kept.
#[must_use]
pub fn is_sanitised(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    if upper == "CARGO_HOME" || upper == "CARGO_NET_OFFLINE" {
        return false;
    }
    upper == "CARGO"
        || upper.starts_with("CARGO_")
        || upper.starts_with("__CARGO")
        || upper.starts_with("RUSTC")
        || upper.starts_with("RUSTDOC")
        || upper.contains("RUSTFLAGS")
        || upper.starts_with("XTASK_")
}

/// A Cargo command from the trusted toolchain with a sanitised environment,
/// run from the trusted tree so no repository `.cargo/config` is loaded.
#[must_use]
pub fn cargo(ctx: &Ctx) -> Command {
    let cargo = std::env::var_os("CARGO").map_or_else(|| PathBuf::from("cargo"), PathBuf::from);
    let rustc = cargo
        .parent()
        .map(|dir| dir.join(format!("rustc{}", std::env::consts::EXE_SUFFIX)))
        .filter(|path| path.is_file());
    let mut command = Command::new(&cargo);
    for (name, _) in std::env::vars_os() {
        if is_sanitised(&name.to_string_lossy()) {
            command.env_remove(&name);
        }
    }
    if let Some(rustc) = rustc {
        command.env("RUSTC", rustc);
    }
    command.current_dir(&ctx.policy);
    command
}

/// One target of a resolved package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target {
    /// Target name.
    pub name: String,
    /// Target kinds (`lib`, `bin`, `test`, `custom-build`, ...).
    pub kinds: Vec<String>,
    /// Source path: repository-relative if inside the repository, else absolute.
    pub src: String,
    /// Whether `src` lies inside the repository.
    pub inside: bool,
    /// Canonical absolute source path.
    pub src_abs: String,
}

/// One resolved package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    /// Package name.
    pub name: String,
    /// Manifest: repository-relative if inside the repository, else absolute.
    pub manifest: String,
    /// Whether the manifest lies inside the repository.
    pub inside: bool,
    /// Source (`None` for a local path package).
    pub source: Option<String>,
    /// Whether the package is a reviewed adapter of the trusted policy.
    pub adapter: bool,
    /// Canonical absolute manifest path, as Cargo reports it.
    pub manifest_abs: String,
    /// Feature names the package declares (`cargo metadata`).
    pub features: Vec<String>,
    /// Targets.
    pub targets: Vec<Target>,
}

/// The resolved package graph, as `cargo metadata` reports it.
#[derive(Debug, Clone, Default)]
pub struct Metadata {
    /// Every package in the resolve, local and registry.
    pub packages: Vec<Package>,
    /// Package ids of the workspace members.
    pub members: Vec<String>,
    /// Package ids, parallel to `packages`.
    pub ids: Vec<String>,
}

impl Metadata {
    /// Local (path) packages: workspace members, path dependencies of any
    /// kind and local patches.
    pub fn local(&self) -> impl Iterator<Item = &Package> {
        self.packages.iter().filter(|p| p.source.is_none())
    }
}

fn canonical(path: &str) -> PathBuf {
    let path = Path::new(path);
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn relative(root: &Path, path: &str) -> (String, bool) {
    let canonical = canonical(path);
    match canonical.strip_prefix(root) {
        Ok(rest) => (rest.to_string_lossy().replace('\\', "/"), true),
        Err(_) => (canonical.to_string_lossy().into_owned(), false),
    }
}

/// Parses `cargo metadata --format-version 1` output.
///
/// # Errors
///
/// Fails on malformed JSON.
pub fn parse_metadata(repo: &Path, json: &[u8], adapters: &[Adapter]) -> Result<Metadata, String> {
    let root = repo.canonicalize().unwrap_or_else(|_| repo.to_path_buf());
    let value: serde_json::Value =
        serde_json::from_slice(json).map_err(|error| format!("cargo metadata: {error}"))?;
    let packages = value["packages"]
        .as_array()
        .ok_or("cargo metadata: no packages")?;
    let mut out = Vec::new();
    let mut ids = Vec::new();
    for package in packages {
        ids.push(package["id"].as_str().unwrap_or_default().to_owned());
        let manifest_path = package["manifest_path"].as_str().unwrap_or("");
        let (manifest, inside) = relative(&root, manifest_path);
        let dir = manifest.rsplit_once('/').map_or("", |(dir, _)| dir);
        let source = package["source"].as_str().map(str::to_owned);
        let targets = package["targets"]
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or_default()
            .iter()
            .map(|target| {
                let src_path = target["src_path"].as_str().unwrap_or("");
                let (src, inside) = relative(&root, src_path);
                Target {
                    src_abs: canonical(src_path).to_string_lossy().into_owned(),
                    name: target["name"].as_str().unwrap_or_default().to_owned(),
                    kinds: target["kind"]
                        .as_array()
                        .map(Vec::as_slice)
                        .unwrap_or_default()
                        .iter()
                        .filter_map(|kind| kind.as_str().map(str::to_owned))
                        .collect(),
                    src,
                    inside,
                }
            })
            .collect();
        let name = package["name"].as_str().unwrap_or_default().to_owned();
        out.push(Package {
            adapter: source.is_none() && inside && is_adapter(adapters, &name, dir),
            name,
            manifest_abs: canonical(manifest_path).to_string_lossy().into_owned(),
            features: package["features"]
                .as_object()
                .map(|features| features.keys().cloned().collect())
                .unwrap_or_default(),
            manifest,
            inside,
            source,
            targets,
        });
    }
    let members = value["workspace_members"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
        .iter()
        .filter_map(|id| id.as_str().map(str::to_owned))
        .collect();
    Ok(Metadata {
        packages: out,
        members,
        ids,
    })
}

/// Runs `cargo metadata` on the tree under test: every feature, every
/// platform, the committed lock file. Cargo resolves members, globs, path
/// dependencies, patches and build-dependencies; nothing is re-derived here.
///
/// # Errors
///
/// Returns Cargo's message if it cannot resolve the workspace; callers report
/// it as a finding, so the check fails closed.
pub fn metadata(ctx: &Ctx) -> Result<Metadata, String> {
    let output = cargo(ctx)
        .args([
            "metadata",
            "--format-version",
            "1",
            "--locked",
            "--all-features",
            "--manifest-path",
        ])
        .arg(ctx.repo.join("Cargo.toml"))
        .output()
        .map_err(|error| format!("cargo metadata: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "cargo metadata failed, so what Cargo builds cannot be checked: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    parse_metadata(&ctx.repo, &output.stdout, &adapters(ctx)?)
}

/// Whether a parsed file starts with a top-level `#![forbid(unsafe_code)]`
/// (not inside `cfg_attr`, which a configuration could switch off).
///
/// The lint list is parsed structurally: one of its entries must be the bare,
/// unqualified Rust lint path `unsafe_code`. A tool-qualified lint such as
/// `clippy::unsafe_code` or a nested group does not forbid Rust's lint and does
/// not qualify (R3-01). A trailing `reason = "..."` is allowed.
#[must_use]
pub fn forbids_unsafe(file: &syn::File) -> bool {
    use syn::punctuated::Punctuated;
    file.attrs.iter().any(|attribute| {
        if !matches!(attribute.style, syn::AttrStyle::Inner(_))
            || !attribute.path().is_ident("forbid")
        {
            return false;
        }
        let syn::Meta::List(list) = &attribute.meta else {
            return false;
        };
        list.parse_args_with(Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
            .is_ok_and(|entries| {
                entries.iter().any(|entry| {
                    matches!(entry, syn::Meta::Path(path)
                        if path.leading_colon.is_none()
                            && path.segments.len() == 1
                            && path.segments[0].arguments.is_none()
                            && path.segments[0].ident.unraw() == "unsafe_code")
                })
            })
    })
}

fn lists_forbid_unsafe(meta: &syn::Meta, path: &[&str]) -> bool {
    use syn::punctuated::Punctuated;
    let syn::Meta::List(list) = meta else {
        return false;
    };
    let Some((head, rest)) = path.split_first() else {
        return false;
    };
    if !list.path.is_ident(head) {
        return false;
    }
    list.parse_args_with(Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
        .is_ok_and(|entries| {
            entries.iter().any(|entry| match rest {
                [] => matches!(entry, syn::Meta::Path(path)
                    if path.leading_colon.is_none()
                        && path.segments.len() == 1
                        && path.segments[0].arguments.is_none()
                        && path.segments[0].ident.unraw() == "unsafe_code"),
                _ => lists_forbid_unsafe(entry, rest),
            })
        })
}

/// Whether a parsed file carries a top-level, unconditional
/// `#![doc(test(attr(forbid(unsafe_code))))]`. rustdoc compiles every
/// doctest as its own crate; this attribute is how the crate's forbid reaches
/// those crates (R6-04).
#[must_use]
pub fn forbids_unsafe_in_doctests(file: &syn::File) -> bool {
    file.attrs.iter().any(|attribute| {
        matches!(attribute.style, syn::AttrStyle::Inner(_))
            && lists_forbid_unsafe(&attribute.meta, &["doc", "test", "attr", "forbid"])
    })
}

/// Checks that a crate root forbids unsafe code (F4) and, for a library
/// root, in its doctests too (R6-04).
#[must_use]
pub fn check_crate_root(path: &str, source: &str, library: bool) -> Vec<Finding> {
    match syn::parse_file(source) {
        Ok(file) => {
            let mut findings = Vec::new();
            if !forbids_unsafe(&file) {
                findings.push(Finding::at(
                    path,
                    Some(1),
                    "crate root of a package that is not a reviewed adapter must start with #![forbid(unsafe_code)] (ADR 0001)",
                ));
            }
            if library && !forbids_unsafe_in_doctests(&file) {
                findings.push(Finding::at(
                    path,
                    Some(1),
                    "library root of a package that is not a reviewed adapter must carry #![doc(test(attr(forbid(unsafe_code))))]: doctests are separate crates (R6-04)",
                ));
            }
            findings
        }
        Err(error) => vec![Finding::at(
            path,
            Some(error.span().start().line),
            format!("cannot parse Rust ({error}); the policy check fails closed"),
        )],
    }
}

/// Checks the resolved package graph: local packages and their targets are
/// tracked files inside the repository, every target of a non-adapter local
/// package forbids unsafe code at its root, dependencies come only from
/// crates.io or the repository, and no tracked package outside `spikes/` is
/// left out of the workspace build.
///
/// # Errors
///
/// Fails if Git or file reads fail. A Cargo resolution failure is a finding.
pub fn check_graph(ctx: &Ctx, metadata: &Metadata) -> Result<Vec<Finding>, String> {
    let files: BTreeSet<String> = tracked(&ctx.repo)?.into_iter().collect();
    let mut findings = Vec::new();
    // root -> (needs forbid, is a library root)
    let mut roots: BTreeMap<String, (bool, bool)> = BTreeMap::new();
    let mut built = BTreeSet::new();
    for package in &metadata.packages {
        match &package.source {
            Some(source) if REGISTRIES.contains(&source.as_str()) => continue,
            Some(source) => {
                findings.push(Finding::general(format!(
                    "dependency `{}` comes from `{source}`; only crates.io or this repository are allowed",
                    package.name
                )));
                continue;
            }
            None => {}
        }
        if !package.inside || !files.contains(&package.manifest) {
            findings.push(Finding::at(
                &package.manifest,
                None,
                format!(
                    "local package `{}` is not a tracked file in this repository, so it cannot be checked",
                    package.name
                ),
            ));
            continue;
        }
        built.insert(package.manifest.clone());
        for target in &package.targets {
            if !target.inside || !files.contains(&target.src) {
                findings.push(Finding::at(
                    &target.src,
                    None,
                    format!(
                        "target `{}` of `{}` is not a tracked file in this repository",
                        target.name, package.name
                    ),
                ));
                continue;
            }
            let library = target
                .kinds
                .iter()
                .any(|kind| matches!(kind.as_str(), "lib" | "rlib" | "dylib" | "proc-macro"));
            let entry = roots.entry(target.src.clone()).or_default();
            // A build script is never exempt (ADV23R6-01).
            let build_script = target.kinds.iter().any(|kind| kind == "custom-build");
            if !package.adapter || build_script {
                entry.0 = true;
                entry.1 |= library;
            }
        }
    }
    for (path, (needs_forbid, library)) in roots {
        if !needs_forbid {
            continue;
        }
        let source = std::fs::read_to_string(ctx.repo.join(&path))
            .map_err(|error| format!("{path}: {error}"))?;
        findings.extend(check_crate_root(&path, &source, library));
    }
    for manifest in &files {
        let is_manifest = manifest == "Cargo.toml" || manifest.ends_with("/Cargo.toml");
        if !is_manifest || !in_scope(manifest) || built.contains(manifest) {
            continue;
        }
        let text = std::fs::read_to_string(ctx.repo.join(manifest))
            .map_err(|error| format!("{manifest}: {error}"))?;
        let has_package = text
            .parse::<DocumentMut>()
            .map_or(true, |document| document.get("package").is_some());
        if has_package {
            findings.push(Finding::at(
                manifest,
                None,
                "package outside spikes/ is not part of the workspace build; add it to members or move it to spikes/",
            ));
        }
    }
    Ok(findings)
}

/// Runs `unsafe-scope` over the resolved workspace (`cargo metadata`).
///
/// # Errors
///
/// Fails if Git or file reads fail.
pub fn check_sources(ctx: &Ctx) -> Result<Vec<Finding>, String> {
    match metadata(ctx) {
        Ok(metadata) => check_graph(ctx, &metadata),
        Err(message) => Ok(vec![Finding::at("Cargo.toml", None, message)]),
    }
}

fn level(item: &Item) -> Option<&str> {
    item.as_str()
        .or_else(|| item.get("level").and_then(Item::as_str))
}

/// Checks the workspace manifest.
#[must_use]
pub fn check_workspace_manifest(path: &str, document: &DocumentMut, licence: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    match document
        .get("workspace")
        .and_then(|w| w.get("lints"))
        .and_then(|l| l.get("rust"))
        .and_then(|r| r.get("unsafe_code"))
    {
        Some(item) if level(item).is_some_and(|l| ENFORCING.contains(&l)) => {}
        Some(_) => findings.push(Finding::at(
            path,
            None,
            "workspace.lints.rust.unsafe_code must be deny or forbid",
        )),
        None => findings.push(Finding::at(
            path,
            None,
            "workspace.lints.rust.unsafe_code is required (deny or forbid)",
        )),
    }
    let declared = document
        .get("workspace")
        .and_then(|w| w.get("package"))
        .and_then(|p| p.get("license"))
        .and_then(Item::as_str);
    if declared != Some(licence) {
        findings.push(Finding::at(
            path,
            None,
            format!("workspace.package.license must be {licence:?} (trusted policy)"),
        ));
    }
    findings
}

/// Checks one package manifest.
#[must_use]
pub fn check_package_manifest(path: &str, document: &DocumentMut) -> Vec<Finding> {
    let mut findings = Vec::new();
    let Some(package) = document.get("package") else {
        return findings;
    };
    let lints = document.get("lints");
    let inherits = lints
        .and_then(|l| l.get("workspace"))
        .and_then(Item::as_bool)
        == Some(true);
    let only_workspace = lints
        .and_then(Item::as_table_like)
        .is_some_and(|table| table.iter().all(|(key, _)| key == "workspace"));
    if !(inherits && only_workspace) {
        findings.push(Finding::at(
            path,
            None,
            "[lints] must be exactly `workspace = true`; per-crate lint overrides are not allowed",
        ));
    }
    let licence_inherited = package
        .get("license")
        .and_then(|l| l.get("workspace"))
        .and_then(Item::as_bool)
        == Some(true);
    if !licence_inherited || package.get("license-file").is_some() {
        findings.push(Finding::at(
            path,
            None,
            "package licence must be `license.workspace = true` with no license-file",
        ));
    }
    findings
}

/// Cargo configuration tables a repository file may set. Everything else
/// (`build`, `target`, `env`, `unstable`, `source`, `net`, `http`, ...) can
/// change flags, wrappers, linkers or sources and is rejected (ADV23R-06).
const CARGO_CONFIG_ALLOWED: &[&str] = &["alias"];

/// Checks a Cargo configuration file: only `[alias]` entries are allowed.
#[must_use]
pub fn check_cargo_config(path: &str, text: &str) -> Vec<Finding> {
    let document: DocumentMut = match text.parse() {
        Ok(document) => document,
        Err(error) => {
            return vec![Finding::at(
                path,
                None,
                format!("invalid Cargo config TOML: {error}"),
            )];
        }
    };
    document
        .iter()
        .filter(|(key, _)| !CARGO_CONFIG_ALLOWED.contains(key))
        .map(|(key, _)| {
            Finding::at(
                path,
                None,
                format!(
                    "Cargo config may only define [alias]; `{key}` can change rustflags, wrappers, linkers or lint levels"
                ),
            )
        })
        .collect()
}

/// Whether `path` is a Cargo configuration file Cargo would load:
/// `config` or `config.toml` directly inside a `.cargo` directory.
#[must_use]
pub fn is_cargo_config(path: &str) -> bool {
    let mut parts = path.rsplit('/');
    let name = parts.next().unwrap_or_default();
    (name == "config" || name == "config.toml") && parts.next() == Some(".cargo")
}

/// Runs the manifest checks.
///
/// # Errors
///
/// Fails if Git, file reads or the trusted policy fail.
pub fn check_manifests(ctx: &Ctx) -> Result<Vec<Finding>, String> {
    let policy = ctx.policy_toml("xtask/policy/repository.toml")?;
    let licence = policy
        .get("workspace_licence")
        .and_then(Item::as_str)
        .ok_or("trusted policy: workspace_licence missing")?
        .to_owned();
    let files: BTreeSet<String> = tracked(&ctx.repo)?.into_iter().collect();
    let mut findings = Vec::new();
    let packages: BTreeSet<String> = match metadata(ctx) {
        Ok(metadata) => metadata
            .local()
            .filter(|package| package.inside)
            .map(|package| package.manifest.clone())
            .collect(),
        Err(message) => {
            findings.push(Finding::at("Cargo.toml", None, message));
            BTreeSet::new()
        }
    };
    let mut saw_root = false;
    for path in files {
        let file_name = path.rsplit('/').next().unwrap_or(&path);
        let is_cargo_config = is_cargo_config(&path);
        if file_name != "Cargo.toml" && !is_cargo_config {
            continue;
        }
        // Production packages are checked wherever they live (R4-01).
        if !in_scope(&path) && !packages.contains(&path) {
            continue;
        }
        let text = std::fs::read_to_string(ctx.repo.join(&path))
            .map_err(|error| format!("{path}: {error}"))?;
        if is_cargo_config {
            findings.extend(check_cargo_config(&path, &text));
            continue;
        }
        let document: DocumentMut = match text.parse() {
            Ok(document) => document,
            Err(error) => {
                findings.push(Finding::at(&path, None, format!("invalid TOML: {error}")));
                continue;
            }
        };
        if path == "Cargo.toml" {
            saw_root = true;
            findings.extend(check_workspace_manifest(&path, &document, &licence));
        }
        findings.extend(check_package_manifest(&path, &document));
    }
    if !saw_root {
        findings.push(Finding::general("Cargo.toml: workspace manifest missing"));
    }
    Ok(findings)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc(text: &str) -> DocumentMut {
        text.parse().expect("toml")
    }

    const LICENCE: &str = "LicenseRef-Test";

    // R5: the workspace lint must be present and enforcing.
    #[test]
    fn r5_workspace_unsafe_lint_is_mandatory() {
        let good = doc(
            "[workspace.package]\nlicense = \"LicenseRef-Test\"\n[workspace.lints.rust]\nunsafe_code = \"deny\"\n",
        );
        assert!(check_workspace_manifest("Cargo.toml", &good, LICENCE).is_empty());
        let table = doc(
            "[workspace.package]\nlicense = \"LicenseRef-Test\"\n[workspace.lints.rust]\nunsafe_code = { level = \"forbid\" }\n",
        );
        assert!(check_workspace_manifest("Cargo.toml", &table, LICENCE).is_empty());
        let missing = doc(
            "[workspace.package]\nlicense = \"LicenseRef-Test\"\n[workspace.lints.rust]\nmissing_docs = \"warn\"\n",
        );
        assert_eq!(
            check_workspace_manifest("Cargo.toml", &missing, LICENCE).len(),
            1
        );
        let lowered = doc(
            "[workspace.package]\nlicense = \"LicenseRef-Test\"\n[workspace.lints.rust.unsafe_code]\nlevel = \"allow\"\n",
        );
        assert_eq!(
            check_workspace_manifest("Cargo.toml", &lowered, LICENCE).len(),
            1
        );
    }

    // ADV23-06: sub-table overrides in a crate manifest.
    #[test]
    fn adv23_06_crate_lint_overrides_are_rejected() {
        let base = "[package]\nname = \"x\"\nlicense.workspace = true\n";
        assert!(
            check_package_manifest(
                "crates/x/Cargo.toml",
                &doc(&format!("{base}[lints]\nworkspace = true\n"))
            )
            .is_empty()
        );
        for extra in [
            "[lints]\nworkspace = true\n[lints.rust.unsafe_code]\nlevel = \"allow\"\n",
            "[lints.rust]\nunsafe_code = \"allow\"\n",
            "",
            "[lints]\nworkspace = false\n",
        ] {
            let found =
                check_package_manifest("crates/x/Cargo.toml", &doc(&format!("{base}{extra}")));
            assert_eq!(found.len(), 1, "{extra}");
        }
    }

    // ADV23-13: crate licences come from the workspace and are checked.
    #[test]
    fn adv23_13_crate_licence_must_inherit() {
        let lints = "[lints]\nworkspace = true\n";
        for package in [
            "[package]\nname = \"x\"\n",
            "[package]\nname = \"x\"\nlicense = \"GPL-3.0-only\"\n",
            "[package]\nname = \"x\"\nlicense.workspace = true\nlicense-file = \"COPYING\"\n",
        ] {
            let found =
                check_package_manifest("crates/x/Cargo.toml", &doc(&format!("{package}{lints}")));
            assert_eq!(found.len(), 1, "{package}");
        }
        let wrong = doc(
            "[workspace.package]\nlicense = \"MIT\"\n[workspace.lints.rust]\nunsafe_code = \"deny\"\n",
        );
        assert_eq!(
            check_workspace_manifest("Cargo.toml", &wrong, LICENCE).len(),
            1
        );
    }

    #[test]
    fn cargo_config_cannot_change_lints() {
        assert!(
            check_cargo_config(
                ".cargo/config.toml",
                "[alias]\nxtask = \"run -p xtask --\"\n"
            )
            .is_empty()
        );
        assert!(
            !check_cargo_config(
                ".cargo/config.toml",
                "[build]\nrustflags = [\"-A\", \"unsafe_code\"]\n"
            )
            .is_empty()
        );
        assert!(
            !check_cargo_config(
                ".cargo/config.toml",
                "[target.x86_64-unknown-linux-gnu]\nrustflags = [\"--cap-lints=allow\"]\n"
            )
            .is_empty()
        );
    }
}
