//! Rust source and manifest policy (ADR 0001, NFR-10).
//!
//! - `unsafe-scope`: outside `crates/solver-*`, every crate root (library,
//!   binaries, tests, examples, benches and build scripts, as Cargo selects
//!   them) starts with `#![forbid(unsafe_code)]`. A `forbid` cannot be lowered
//!   later, so the compiler rejects any opt-out, including one assembled by a
//!   macro from a parameter (F4). As defence in depth, no attribute anywhere
//!   lowers the lint by name; files are parsed with `syn`, so multiline,
//!   `cfg_attr`, nested and macro-body attributes are seen (R5, ADV23-03,
//!   ADV23-05). Unparseable Rust fails the check.
//! - `manifests`: the workspace keeps `unsafe_code` at deny/forbid; every
//!   package inherits workspace lints and nothing else (ADV23-06); every package
//!   inherits the workspace licence (ADV23-13); a Cargo configuration file
//!   (`config` or `config.toml` in any `.cargo/` directory) may hold only
//!   `[alias]` entries (ADV23R-06).
//! - `native-runtimes`: no build script (the one Cargo selects, including a
//!   custom `build = "..."` path, F7, and every module it includes,
//!   ADV23R4-02) and no `link` attribute, conditional or not, names a Python
//!   or Julia library (ADV23-12). String literals are decoded before matching,
//!   so escapes cannot hide a name (ADV23R4-01).
//!
//! Scope is decided by what Cargo compiles, not by directory (R4-01): every
//! workspace member, every package outside `spikes/`, and every package they
//! reach through `path` dependencies is a production package. Its targets and
//! every module or included file reachable from them (`mod`, `#[path]`,
//! `include!`) are checked wherever they live. The `spikes/` exemption covers
//! only files no production package reaches.

use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::ToTokens as _;
use syn::ext::IdentExt as _;
use syn::spanned::Spanned as _;
use syn::visit::Visit;
use toml_edit::{DocumentMut, Item};

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::util::{Ctx, Finding, glob_match, tracked};

const LOWERING: &[&str] = &["allow", "expect", "warn"];
const ENFORCING: &[&str] = &["deny", "forbid"];

fn in_solver_adapter(path: &str) -> bool {
    path.strip_prefix("crates/")
        .is_some_and(|rest| rest.starts_with("solver-"))
}

fn in_scope(path: &str) -> bool {
    !path.starts_with("spikes/")
}

/// Whether a token stream lowers `unsafe_code`: an `allow`, `expect` or
/// `warn` group that names it, at any nesting depth.
#[must_use]
pub fn lowers_unsafe(tokens: TokenStream) -> bool {
    let trees: Vec<TokenTree> = tokens.into_iter().collect();
    for (index, tree) in trees.iter().enumerate() {
        match tree {
            TokenTree::Ident(ident) => {
                let name = ident.unraw().to_string();
                if LOWERING.contains(&name.as_str())
                    && let Some(TokenTree::Group(group)) = trees.get(index + 1)
                    && group.delimiter() == Delimiter::Parenthesis
                    && names_unsafe(group.stream())
                {
                    return true;
                }
            }
            TokenTree::Group(group) if lowers_unsafe(group.stream()) => return true,
            _ => {}
        }
    }
    false
}

/// Whether a token stream holds a `link(...)` group naming a Python or Julia
/// library at any depth, e.g. inside `cfg_attr` or a macro body (F7).
#[must_use]
pub fn links_runtime(tokens: TokenStream) -> bool {
    let trees: Vec<TokenTree> = tokens.into_iter().collect();
    for (index, tree) in trees.iter().enumerate() {
        match tree {
            TokenTree::Ident(ident) => {
                if ident.unraw() == "link"
                    && let Some(TokenTree::Group(group)) = trees.get(index + 1)
                    && group.delimiter() == Delimiter::Parenthesis
                    && names_runtime(&decoded_text(group.stream()))
                {
                    return true;
                }
            }
            TokenTree::Group(group) if links_runtime(group.stream()) => return true,
            _ => {}
        }
    }
    false
}

fn names_unsafe(tokens: TokenStream) -> bool {
    tokens.into_iter().any(|tree| match tree {
        TokenTree::Ident(ident) => ident.unraw() == "unsafe_code",
        TokenTree::Group(group) => names_unsafe(group.stream()),
        _ => false,
    })
}

struct Scan<'a> {
    path: &'a str,
    build_script: bool,
    check_unsafe: bool,
    findings: Vec<Finding>,
}

impl Scan<'_> {
    fn flag(&mut self, line: usize, message: &str) {
        self.findings
            .push(Finding::at(self.path, Some(line), message.to_owned()));
    }
}

/// The text of a token stream with every string literal decoded as Rust
/// would decode it (`\x70`, `\u{74}`, raw strings, ...), so an escape cannot
/// hide a name (ADV23R4-01). Returns the raw token text, then each decoded
/// literal on its own line, then all decoded literals concatenated, so a
/// name split across `concat!` pieces is also seen.
#[must_use]
pub fn decoded_text(tokens: TokenStream) -> String {
    fn literals(tokens: TokenStream, out: &mut Vec<String>) {
        for tree in tokens {
            match tree {
                TokenTree::Literal(literal) => {
                    let stream = TokenTree::Literal(literal).into_token_stream();
                    if let Ok(text) = syn::parse2::<syn::LitStr>(stream.clone()) {
                        out.push(text.value());
                    } else if let Ok(bytes) = syn::parse2::<syn::LitByteStr>(stream.clone()) {
                        out.push(String::from_utf8_lossy(&bytes.value()).into_owned());
                    } else if let Ok(text) = syn::parse2::<syn::LitCStr>(stream.clone()) {
                        out.push(text.value().to_string_lossy().into_owned());
                    } else if let Ok(character) = syn::parse2::<syn::LitChar>(stream) {
                        out.push(character.value().to_string());
                    }
                }
                TokenTree::Group(group) => literals(group.stream(), out),
                _ => {}
            }
        }
    }
    let mut values = Vec::new();
    let raw = tokens.to_string();
    literals(tokens, &mut values);
    format!("{raw}\n{}\n{}", values.join("\n"), values.concat())
}

fn names_runtime(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    ["python", "julia", "libpython", "libjulia"]
        .iter()
        .any(|name| lower.contains(name))
}

impl<'ast> Visit<'ast> for Scan<'_> {
    fn visit_attribute(&mut self, attribute: &'ast syn::Attribute) {
        let tokens = quote_attr(attribute);
        let line = attribute.span().start().line;
        if self.check_unsafe && lowers_unsafe(tokens.clone()) {
            self.flag(
                line,
                "unsafe_code may only be lowered in crates/solver-* adapters",
            );
        }
        if (attribute.path().is_ident("link") && names_runtime(&decoded_text(tokens.clone())))
            || links_runtime(tokens)
        {
            self.flag(
                line,
                "linking a Python or Julia runtime is forbidden (NFR-10)",
            );
        }
        syn::visit::visit_attribute(self, attribute);
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if self.check_unsafe && lowers_unsafe(mac.tokens.clone()) {
            self.flag(
                mac.span().start().line,
                "macro body lowers unsafe_code outside crates/solver-*",
            );
        }
        if links_runtime(mac.tokens.clone()) {
            self.flag(
                mac.span().start().line,
                "macro body links a Python or Julia runtime (NFR-10)",
            );
        }
        if self.build_script && names_runtime(&decoded_text(mac.tokens.clone())) {
            self.flag(
                mac.span().start().line,
                "build script names a Python or Julia library (NFR-10)",
            );
        }
        syn::visit::visit_macro(self, mac);
    }

    fn visit_lit_str(&mut self, literal: &'ast syn::LitStr) {
        if self.build_script && names_runtime(&literal.value()) {
            self.flag(
                literal.span().start().line,
                "build script names a Python or Julia library (NFR-10)",
            );
        }
    }

    fn visit_lit_byte_str(&mut self, literal: &'ast syn::LitByteStr) {
        if self.build_script && names_runtime(&String::from_utf8_lossy(&literal.value())) {
            self.flag(
                literal.span().start().line,
                "build script names a Python or Julia library (NFR-10)",
            );
        }
    }
}

fn quote_attr(attribute: &syn::Attribute) -> TokenStream {
    attribute.meta.to_token_stream()
}

/// Scans one Rust file; a file named `build.rs` is treated as a build script.
#[must_use]
pub fn scan_rust(path: &str, source: &str) -> Vec<Finding> {
    scan_rust_as(
        path,
        source,
        path == "build.rs" || path.ends_with("/build.rs"),
    )
}

/// Scans one Rust file, stating whether Cargo uses it as a build script (F7).
#[must_use]
pub fn scan_rust_as(path: &str, source: &str, build_script: bool) -> Vec<Finding> {
    scan_rust_with(path, source, build_script, !in_solver_adapter(path))
}

/// Scans one Rust file, stating whether it is build-script code and whether
/// the unsafe-scope rule applies (it does unless only solver adapters use it).
#[must_use]
pub fn scan_rust_with(
    path: &str,
    source: &str,
    build_script: bool,
    check_unsafe: bool,
) -> Vec<Finding> {
    let file = match syn::parse_file(source) {
        Ok(file) => file,
        Err(error) => {
            return vec![Finding::at(
                path,
                Some(error.span().start().line),
                format!("cannot parse Rust ({error}); the policy check fails closed"),
            )];
        }
    };
    let mut scan = Scan {
        path,
        build_script,
        check_unsafe,
        findings: Vec::new(),
    };
    scan.visit_file(&file);
    scan.findings
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

/// Checks that a crate root forbids unsafe code (F4).
#[must_use]
pub fn check_crate_root(path: &str, source: &str) -> Vec<Finding> {
    match syn::parse_file(source) {
        Ok(file) if forbids_unsafe(&file) => Vec::new(),
        Ok(_) => vec![Finding::at(
            path,
            Some(1),
            "crate root outside crates/solver-* must start with #![forbid(unsafe_code)] (ADR 0001)",
        )],
        Err(_) => Vec::new(), // scan_rust reports the parse failure
    }
}

/// Rust targets of one package, as Cargo selects them: explicit target
/// paths plus the conventional auto-discovered files. Returns (crate roots,
/// build scripts), repository-relative.
#[must_use]
pub fn package_targets(
    manifest_path: &str,
    document: &DocumentMut,
    tracked: &BTreeSet<String>,
) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut roots = BTreeSet::new();
    let mut build = BTreeSet::new();
    let Some(package) = document.get("package") else {
        return (roots, build);
    };
    let dir = manifest_path
        .rsplit_once('/')
        .map_or(String::new(), |(dir, _)| format!("{dir}/"));
    let join = |relative: &str| normalise_path(&format!("{dir}{relative}"));
    match package.get("build") {
        Some(item) if item.as_bool() == Some(false) => {}
        Some(item) if item.as_str().is_some() => {
            build.insert(join(item.as_str().unwrap_or_default()));
        }
        _ => {
            let default = join("build.rs");
            if tracked.contains(&default) || package.get("build").is_some() {
                build.insert(default);
            }
        }
    }
    for key in ["lib", "bin", "test", "example", "bench"] {
        let tables: Vec<&dyn toml_edit::TableLike> = match document.get(key) {
            Some(Item::ArrayOfTables(array)) => array
                .iter()
                .map(|table| table as &dyn toml_edit::TableLike)
                .collect(),
            Some(item) => item.as_table_like().into_iter().collect(),
            None => Vec::new(),
        };
        for table in tables {
            if let Some(path) = table.get("path").and_then(Item::as_str) {
                roots.insert(join(path));
            }
        }
    }
    for file in tracked {
        let Some(relative) = file.strip_prefix(&dir) else {
            continue;
        };
        let parts: Vec<&str> = relative.split('/').collect();
        let root = match parts.as_slice() {
            ["src", "lib.rs" | "main.rs"] => true,
            ["src", "bin", name] | ["tests" | "examples" | "benches", name] => {
                name.ends_with(".rs")
            }
            ["src", "bin", _, "main.rs"] | ["tests" | "examples" | "benches", _, "main.rs"] => true,
            _ => false,
        };
        if root {
            roots.insert(file.clone());
        }
    }
    for script in &build {
        roots.insert(script.clone());
    }
    (roots, build)
}

fn normalise_path(path: &str) -> String {
    normalise_within(path).unwrap_or_else(|| path.to_owned())
}

/// Normalises a repository-relative path; `None` if it leaves the repository.
fn normalise_within(path: &str) -> Option<String> {
    let mut parts: Vec<&str> = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            other => parts.push(other),
        }
    }
    Some(parts.join("/"))
}

fn parent_dir(path: &str) -> &str {
    path.rsplit_once('/').map_or("", |(dir, _)| dir)
}

fn join_dir(dir: &str, relative: &str) -> Option<String> {
    if relative.starts_with('/') {
        return None;
    }
    normalise_within(&if dir.is_empty() {
        relative.to_owned()
    } else {
        format!("{dir}/{relative}")
    })
}

/// A package Cargo compiles in the workspace build.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    /// Repository-relative `Cargo.toml` path.
    pub manifest: String,
    /// Whether the package is a reviewed `crates/solver-*` adapter.
    pub adapter: bool,
    /// Whether the package is compiled into a build script (a path
    /// build-dependency, directly or transitively), so its code may emit
    /// Cargo link instructions.
    pub build_context: bool,
}

const DEPENDENCY_TABLES: &[&str] = &[
    "dependencies",
    "dev-dependencies",
    "dev_dependencies",
    "build-dependencies",
    "build_dependencies",
];

/// `path` dependencies of one manifest: (dependency directory as written,
/// whether it is a build dependency).
fn path_dependencies(
    document: &DocumentMut,
    workspace_paths: &BTreeMap<String, String>,
) -> Vec<(String, bool)> {
    let mut found = Vec::new();
    let mut table = |deps: &dyn toml_edit::TableLike, build: bool| {
        for (name, entry) in deps.iter() {
            let Some(entry) = entry.as_table_like() else {
                continue;
            };
            if let Some(path) = entry.get("path").and_then(Item::as_str) {
                found.push((path.to_owned(), build));
            } else if entry.get("workspace").and_then(Item::as_bool) == Some(true) {
                let key = entry.get("package").and_then(Item::as_str).unwrap_or(name);
                if let Some(path) = workspace_paths
                    .get(key)
                    .or_else(|| workspace_paths.get(name))
                {
                    found.push((format!("/{path}"), build));
                }
            }
        }
    };
    let mut scopes: Vec<&dyn toml_edit::TableLike> = vec![document.as_table()];
    if let Some(targets) = document.get("target").and_then(Item::as_table_like) {
        scopes.extend(targets.iter().filter_map(|(_, item)| item.as_table_like()));
    }
    for scope in scopes {
        for key in DEPENDENCY_TABLES {
            if let Some(deps) = scope.get(key).and_then(Item::as_table_like) {
                table(deps, key.starts_with("build"));
            }
        }
    }
    if let Some(deps) = document
        .get("workspace")
        .and_then(|w| w.get("dependencies"))
        .and_then(Item::as_table_like)
    {
        table(deps, false);
    }
    if let Some(outer) = document.get("patch").and_then(Item::as_table_like) {
        for (_, inner) in outer.iter() {
            if let Some(inner) = inner.as_table_like() {
                table(inner, false);
            }
        }
    }
    // `[replace]` entries map a package spec to a table with a `path`.
    if let Some(replace) = document.get("replace").and_then(Item::as_table_like) {
        table(replace, false);
    }
    found
}

/// Every package the workspace build compiles (R4-01): workspace members
/// (by the root manifest's `members`/`exclude` globs), every package outside
/// `spikes/`, and the closure of their `path` dependencies wherever those
/// live. Returns packages by manifest path and findings for dependencies
/// that cannot be checked.
///
/// # Errors
///
/// Fails if a manifest cannot be read.
pub fn production_packages(
    repo: &Path,
    files: &BTreeSet<String>,
) -> Result<(BTreeMap<String, Package>, Vec<Finding>), String> {
    let read = |path: &str| -> Result<Option<DocumentMut>, String> {
        let text =
            std::fs::read_to_string(repo.join(path)).map_err(|error| format!("{path}: {error}"))?;
        Ok(text.parse::<DocumentMut>().ok())
    };
    let manifests: Vec<&String> = files
        .iter()
        .filter(|path| *path == "Cargo.toml" || path.ends_with("/Cargo.toml"))
        .collect();
    let mut queue: Vec<(String, bool)> = manifests
        .iter()
        .filter(|path| in_scope(path))
        .map(|path| ((*path).clone(), false))
        .collect();
    let mut workspace_paths = BTreeMap::new();
    if files.contains("Cargo.toml")
        && let Some(root) = read("Cargo.toml")?
    {
        let workspace = root.get("workspace");
        let globs = |key: &str| {
            workspace
                .and_then(|w| w.get(key))
                .map(crate::util::string_array)
                .unwrap_or_default()
        };
        let members = globs("members");
        let exclude = globs("exclude");
        for manifest in &manifests {
            let dir = parent_dir(manifest);
            let matches = |list: &[String]| {
                list.iter()
                    .any(|glob| normalise_within(glob).is_some_and(|g| glob_match(&g, dir)))
            };
            if matches(&members) && !matches(&exclude) {
                queue.push(((*manifest).clone(), false));
            }
        }
        if let Some(deps) = workspace
            .and_then(|w| w.get("dependencies"))
            .and_then(Item::as_table_like)
        {
            for (name, entry) in deps.iter() {
                if let Some(path) = entry
                    .as_table_like()
                    .and_then(|t| t.get("path"))
                    .and_then(Item::as_str)
                    .and_then(normalise_within)
                {
                    workspace_paths.insert(name.to_owned(), path);
                }
            }
        }
    }
    let mut packages: BTreeMap<String, Package> = BTreeMap::new();
    let mut findings = Vec::new();
    while let Some((manifest, build_context)) = queue.pop() {
        if let Some(known) = packages.get_mut(&manifest) {
            if build_context && !known.build_context {
                known.build_context = true;
            } else {
                continue;
            }
        } else {
            packages.insert(
                manifest.clone(),
                Package {
                    manifest: manifest.clone(),
                    adapter: in_solver_adapter(&manifest),
                    build_context,
                },
            );
        }
        let Some(document) = read(&manifest)? else {
            continue; // `manifests` reports invalid TOML
        };
        let dir = parent_dir(&manifest);
        for (path, build) in path_dependencies(&document, &workspace_paths) {
            let target = match path.strip_prefix('/') {
                Some(rooted) => Some(rooted.to_owned()),
                None => join_dir(dir, &path),
            };
            let dependency = target.map(|t| {
                if t.is_empty() {
                    "Cargo.toml".to_owned()
                } else {
                    format!("{t}/Cargo.toml")
                }
            });
            match dependency {
                Some(dependency) if files.contains(&dependency) => {
                    queue.push((dependency, build_context || build));
                }
                _ => findings.push(Finding::at(
                    &manifest,
                    None,
                    format!(
                        "path dependency `{path}` is not a tracked package in this repository, so it cannot be checked"
                    ),
                )),
            }
        }
    }
    Ok((packages, findings))
}

/// How one reachable source file is compiled.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Use {
    build_script: bool,
    check_unsafe: bool,
}

/// Collects the files one Rust source pulls in: out-of-line `mod`
/// declarations (with `#[path]`, including under `cfg_attr`), `mod`
/// declarations written inside macro bodies, and `include!` of a literal
/// path. Candidate locations are generous: every place rustc could look is
/// considered, so over-inclusion only widens the scan.
struct Modules<'a> {
    files: &'a BTreeSet<String>,
    file_dir: &'a str,
    bases: Vec<String>,
    found: Vec<(String, bool)>, // (candidate path, file is mod-rs-like)
    unresolved: Vec<(usize, String)>,
}

fn path_attributes(attrs: &[syn::Attribute]) -> Vec<String> {
    fn literal_paths(tokens: TokenStream, out: &mut Vec<String>) {
        let trees: Vec<TokenTree> = tokens.into_iter().collect();
        for (index, tree) in trees.iter().enumerate() {
            match tree {
                TokenTree::Ident(ident) if ident.unraw() == "path" => {
                    if let (Some(TokenTree::Punct(eq)), Some(TokenTree::Literal(lit))) =
                        (trees.get(index + 1), trees.get(index + 2))
                        && eq.as_char() == '='
                        && let Ok(text) =
                            syn::parse2::<syn::LitStr>(TokenTree::Literal(lit.clone()).into())
                    {
                        out.push(text.value());
                    }
                }
                TokenTree::Group(group) => literal_paths(group.stream(), out),
                _ => {}
            }
        }
    }
    let mut out = Vec::new();
    for attribute in attrs {
        if attribute.path().is_ident("path") || attribute.path().is_ident("cfg_attr") {
            literal_paths(attribute.meta.to_token_stream(), &mut out);
        }
    }
    out
}

impl Modules<'_> {
    fn declare(&mut self, name: &str, paths: &[String], line: usize) {
        let mut candidates = Vec::new();
        for path in paths {
            for dir in std::iter::once(self.file_dir.to_owned()).chain(self.bases.clone()) {
                if let Some(candidate) = join_dir(&dir, path) {
                    candidates.push((candidate, true));
                }
            }
        }
        for base in self.bases.clone() {
            for (candidate, mod_rs) in [
                (format!("{name}.rs"), false),
                (format!("{name}/mod.rs"), true),
            ] {
                if let Some(path) = join_dir(&base, &candidate) {
                    candidates.push((path, mod_rs));
                }
            }
        }
        candidates.retain(|(path, _)| self.files.contains(path));
        if candidates.is_empty() {
            self.unresolved.push((line, name.to_owned()));
        }
        self.found.extend(candidates);
    }

    fn scan_tokens(&mut self, tokens: TokenStream, line: usize) {
        let trees: Vec<TokenTree> = tokens.into_iter().collect();
        for (index, tree) in trees.iter().enumerate() {
            match tree {
                TokenTree::Ident(ident) if ident.unraw() == "mod" => {
                    if let (Some(TokenTree::Ident(name)), Some(TokenTree::Punct(semi))) =
                        (trees.get(index + 1), trees.get(index + 2))
                        && semi.as_char() == ';'
                    {
                        self.declare(&name.unraw().to_string(), &[], line);
                    }
                }
                TokenTree::Ident(ident) if ident.unraw() == "include" => {
                    if let (Some(TokenTree::Punct(bang)), Some(TokenTree::Group(group))) =
                        (trees.get(index + 1), trees.get(index + 2))
                        && bang.as_char() == '!'
                    {
                        self.include(group.stream(), line);
                    }
                }
                TokenTree::Group(group) => self.scan_tokens(group.stream(), line),
                _ => {}
            }
        }
    }

    fn include(&mut self, tokens: TokenStream, line: usize) {
        if let Ok(path) = syn::parse2::<syn::LitStr>(tokens.clone()) {
            match join_dir(self.file_dir, &path.value()) {
                Some(candidate) if self.files.contains(&candidate) => {
                    self.found.push((candidate, true));
                }
                _ => self.unresolved.push((line, path.value())),
            }
        } else if !tokens.to_string().contains("OUT_DIR") {
            // Generated code under OUT_DIR is produced by the (scanned) build
            // script; any other computed path cannot be followed.
            self.unresolved.push((line, tokens.to_string()));
        }
    }
}

impl<'ast> Visit<'ast> for Modules<'_> {
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        let name = item.ident.unraw().to_string();
        let paths = path_attributes(&item.attrs);
        let line = item.span().start().line;
        if let Some((_, items)) = &item.content {
            let saved = self.bases.clone();
            let mut next: Vec<String> = Vec::new();
            for base in &saved {
                for segment in paths.iter().chain(std::iter::once(&name)) {
                    if let Some(dir) = join_dir(base, segment) {
                        next.push(dir);
                    }
                }
            }
            self.bases = next;
            for inner in items {
                self.visit_item(inner);
            }
            self.bases = saved;
        } else {
            self.declare(&name, &paths, line);
        }
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        let line = mac.span().start().line;
        if mac
            .path
            .segments
            .last()
            .is_some_and(|s| s.ident == "include")
        {
            self.include(mac.tokens.clone(), line);
        } else {
            self.scan_tokens(mac.tokens.clone(), line);
        }
        syn::visit::visit_macro(self, mac);
    }
}

/// Every file reachable from the given crate roots through `mod`, `#[path]`
/// and `include!`, with how it is used. Returns the reachable files and
/// findings for module references that cannot be followed.
fn reachable(
    repo: &Path,
    files: &BTreeSet<String>,
    starts: &[(String, Use)],
) -> Result<(BTreeMap<String, Use>, Vec<Finding>), String> {
    let mut seen: BTreeMap<String, Use> = BTreeMap::new();
    let mut visited: BTreeSet<(String, Vec<String>, bool, bool)> = BTreeSet::new();
    let mut findings = Vec::new();
    let mut queue: Vec<(String, Vec<String>, Use)> = starts
        .iter()
        .map(|(path, usage)| (path.clone(), vec![parent_dir(path).to_owned()], *usage))
        .collect();
    while let Some((path, bases, usage)) = queue.pop() {
        if !visited.insert((
            path.clone(),
            bases.clone(),
            usage.build_script,
            usage.check_unsafe,
        )) {
            continue;
        }
        let entry = seen.entry(path.clone()).or_default();
        entry.build_script |= usage.build_script;
        entry.check_unsafe |= usage.check_unsafe;
        if !files.contains(&path) {
            continue;
        }
        let source = std::fs::read_to_string(repo.join(&path))
            .map_err(|error| format!("{path}: {error}"))?;
        let Ok(file) = syn::parse_file(&source) else {
            continue; // the scan reports the parse failure
        };
        let file_dir = parent_dir(&path).to_owned();
        let mut modules = Modules {
            files,
            file_dir: &file_dir,
            bases: bases.clone(),
            found: Vec::new(),
            unresolved: Vec::new(),
        };
        modules.visit_file(&file);
        for (candidate, mod_rs) in modules.found {
            if !files.contains(&candidate) {
                continue;
            }
            // A file's own modules live beside it (mod.rs-like, `#[path]`,
            // `include!`) or in a directory named after it. Both are
            // considered, plus the includer's directories (an `include!`d
            // file is spliced into its includer), so nothing is missed.
            let mut child_bases = vec![parent_dir(&candidate).to_owned()];
            if !mod_rs && let Some(stem) = candidate.strip_suffix(".rs") {
                child_bases.push(stem.to_owned());
            }
            child_bases.extend(bases.iter().cloned());
            child_bases.sort();
            child_bases.dedup();
            queue.push((candidate, child_bases, usage));
        }
        for (line, name) in modules.unresolved {
            findings.push(Finding::at(
                &path,
                Some(line),
                format!("module or include `{name}` cannot be resolved to a tracked file, so it cannot be checked"),
            ));
        }
    }
    Ok((seen, findings))
}

/// Runs `unsafe-scope` and `native-runtimes` over tracked Rust files.
///
/// # Errors
///
/// Fails if Git or file reads fail.
pub fn check_sources(ctx: &Ctx) -> Result<Vec<Finding>, String> {
    let files: BTreeSet<String> = tracked(&ctx.repo)?.into_iter().collect();
    let (packages, mut findings) = production_packages(&ctx.repo, &files)?;
    let mut roots: BTreeMap<String, bool> = BTreeMap::new(); // root -> needs forbid
    let mut build_scripts = BTreeSet::new();
    let mut starts: Vec<(String, Use)> = Vec::new();
    for package in packages.values() {
        let text = std::fs::read_to_string(ctx.repo.join(&package.manifest))
            .map_err(|error| format!("{}: {error}", package.manifest))?;
        let Ok(document) = text.parse::<DocumentMut>() else {
            continue;
        };
        let (crate_roots, scripts) = package_targets(&package.manifest, &document, &files);
        for root in &crate_roots {
            let build = scripts.contains(root) || package.build_context;
            *roots.entry(root.clone()).or_default() |= !package.adapter;
            starts.push((
                root.clone(),
                Use {
                    build_script: build,
                    check_unsafe: !package.adapter,
                },
            ));
        }
        build_scripts.extend(scripts);
    }
    for path in &build_scripts {
        if !files.contains(path) {
            findings.push(Finding::at(
                path,
                None,
                "build script selected by Cargo.toml is not a tracked file",
            ));
        }
    }
    for path in roots.keys() {
        if !files.contains(path) && !build_scripts.contains(path) {
            findings.push(Finding::at(
                path,
                None,
                "crate target selected by Cargo.toml is not a tracked file",
            ));
        }
    }
    let (reached, unresolved) = reachable(&ctx.repo, &files, &starts)?;
    findings.extend(unresolved);
    // Every file Cargo compiles is scanned whatever its suffix or directory
    // (R3-02, R4-01). Other tracked `.rs` files outside `spikes/` are scanned
    // too, by path.
    let mut scanned: BTreeMap<String, Use> = files
        .iter()
        .filter(|path| path.ends_with(".rs") && in_scope(path))
        .map(|path| {
            (
                path.clone(),
                Use {
                    build_script: false,
                    check_unsafe: !in_solver_adapter(path),
                },
            )
        })
        .collect();
    for (path, usage) in reached {
        if files.contains(&path) {
            let entry = scanned.entry(path).or_default();
            entry.build_script |= usage.build_script;
            entry.check_unsafe |= usage.check_unsafe;
        }
    }
    for (path, usage) in &scanned {
        let source = std::fs::read_to_string(ctx.repo.join(path))
            .map_err(|error| format!("{path}: {error}"))?;
        findings.extend(scan_rust_with(
            path,
            &source,
            usage.build_script,
            usage.check_unsafe,
        ));
        if roots.get(path).copied().unwrap_or(false) {
            findings.extend(check_crate_root(path, &source));
        }
    }
    Ok(findings)
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
    let (packages, mut findings) = production_packages(&ctx.repo, &files)?;
    let mut saw_root = false;
    for path in files {
        let file_name = path.rsplit('/').next().unwrap_or(&path);
        let is_cargo_config = is_cargo_config(&path);
        if file_name != "Cargo.toml" && !is_cargo_config {
            continue;
        }
        // Production packages are checked wherever they live (R4-01).
        if !in_scope(&path) && !packages.contains_key(&path) {
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

    fn flagged(path: &str, source: &str) -> bool {
        !scan_rust(path, source).is_empty()
    }

    #[test]
    fn direct_allow_is_rejected_outside_solvers_only() {
        let source = "#![allow(unsafe_code)]\nfn main() {}\n";
        assert!(flagged("crates/domain/src/lib.rs", source));
        assert!(!flagged("crates/solver-highs/src/lib.rs", source));
        assert!(!flagged(
            "crates/domain/src/lib.rs",
            "#![forbid(unsafe_code)]\n#![deny(unsafe_code)]\n"
        ));
    }

    // ADV23-03: multiline attributes.
    #[test]
    fn adv23_03_multiline_attribute_is_rejected() {
        assert!(flagged(
            "crates/domain/src/lib.rs",
            "#[allow(\n    unsafe_code\n)]\npub fn f() {}\n"
        ));
    }

    // R5 / ADV23-05: cfg_attr and nested forms.
    #[test]
    fn r5_adv23_05_cfg_attr_is_rejected() {
        for source in [
            "#![cfg_attr(all(), allow(unsafe_code))]\n",
            "#[cfg_attr(not(test), cfg_attr(all(), expect(unsafe_code)))]\nfn f() {}\n",
            "#[warn(r#unsafe_code)]\nfn f() {}\n",
            "fn f() { #[allow(unsafe_code)] let x = 1; }\n",
            "macro_rules! m { () => { #[allow(unsafe_code)] fn g() {} } }\n",
            "impl S { #[allow(clippy::all, unsafe_code)] fn f() {} }\n",
        ] {
            assert!(flagged("crates/engine/src/lib.rs", source), "{source}");
        }
    }

    #[test]
    fn unparseable_rust_fails_closed() {
        assert!(flagged("crates/engine/src/lib.rs", "fn ("));
    }

    // ADV23-12: build scripts and link attributes naming Python or Julia.
    #[test]
    fn adv23_12_native_runtime_links_are_rejected() {
        assert!(flagged(
            "crates/engine/build.rs",
            "fn main() { println!(\"cargo:rustc-link-lib=python3.12\"); }\n"
        ));
        assert!(flagged(
            "crates/engine/src/lib.rs",
            "#[link(name = \"julia\")]\nunsafe extern \"C\" {}\n"
        ));
        assert!(!flagged(
            "crates/engine/build.rs",
            "fn main() { println!(\"cargo:rerun-if-changed=build.rs\"); }\n"
        ));
    }

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
