//! Rust source and manifest policy (ADR 0001, NFR-10).
//!
//! - `unsafe-scope`: outside `crates/solver-*`, no attribute anywhere lowers
//!   the `unsafe_code` lint. Files are parsed with `syn`, so multiline,
//!   `cfg_attr`, nested and macro-generated attributes are all seen
//!   (R5, ADV23-03, ADV23-05). Unparseable Rust fails the check.
//! - `manifests`: the workspace keeps `unsafe_code` at deny/forbid; every
//!   package inherits workspace lints and nothing else (ADV23-06); every package
//!   inherits the workspace licence (ADV23-13); Cargo config cannot add lint
//!   flags.
//! - `native-runtimes`: no build script or `#[link]` names a Python or Julia
//!   library (ADV23-12).

use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::ToTokens as _;
use syn::ext::IdentExt as _;
use syn::spanned::Spanned as _;
use syn::visit::Visit;
use toml_edit::{DocumentMut, Item};

use crate::util::{Ctx, Finding, tracked};

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
        if attribute.path().is_ident("link") && names_runtime(&tokens.to_string()) {
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
        if self.build_script && names_runtime(&mac.tokens.to_string()) {
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
}

fn quote_attr(attribute: &syn::Attribute) -> TokenStream {
    attribute.meta.to_token_stream()
}

/// Scans one Rust file.
#[must_use]
pub fn scan_rust(path: &str, source: &str) -> Vec<Finding> {
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
        build_script: path == "build.rs" || path.ends_with("/build.rs"),
        check_unsafe: !in_solver_adapter(path),
        findings: Vec::new(),
    };
    scan.visit_file(&file);
    scan.findings
}

/// Runs `unsafe-scope` and `native-runtimes` over tracked Rust files.
///
/// # Errors
///
/// Fails if Git or file reads fail.
pub fn check_sources(ctx: &Ctx) -> Result<Vec<Finding>, String> {
    let mut findings = Vec::new();
    for path in tracked(&ctx.repo)? {
        if !path.ends_with(".rs") || !in_scope(&path) {
            continue;
        }
        let source = std::fs::read_to_string(ctx.repo.join(&path))
            .map_err(|error| format!("{path}: {error}"))?;
        findings.extend(scan_rust(&path, &source));
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

/// Checks Cargo configuration files for lint-changing flags.
#[must_use]
pub fn check_cargo_config(path: &str, text: &str) -> Vec<Finding> {
    let lower = text.to_ascii_lowercase();
    if [
        "unsafe_code",
        "cap-lints",
        "rustflags",
        "rustc-wrapper",
        "rustc =",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
    {
        vec![Finding::at(
            path,
            None,
            "Cargo config may not set rustflags, a rustc wrapper or lint levels",
        )]
    } else {
        Vec::new()
    }
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
    let mut findings = Vec::new();
    let mut saw_root = false;
    for path in tracked(&ctx.repo)? {
        if !in_scope(&path) {
            continue;
        }
        let file_name = path.rsplit('/').next().unwrap_or(&path);
        let is_cargo_config =
            path.contains(".cargo/") && (file_name == "config" || file_name == "config.toml");
        if file_name != "Cargo.toml" && !is_cargo_config {
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
