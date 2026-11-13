//! End-to-end repository policy checks over scratch repositories
//! (F4, F7, F8, F9, ADV23R-06, ADV23R-09), using the real trusted policy.

#![forbid(unsafe_code)]

mod common;

use common::{Repo, TempDir, root};
use xtask::util::{Ctx, Finding};
use xtask::{repository, rust_policy};

fn ctx(repo: &Repo) -> Ctx {
    Ctx {
        repo: repo.path().to_path_buf(),
        policy: root(),
    }
}

fn files(findings: &[Finding]) -> Vec<String> {
    findings
        .iter()
        .map(|f| format!("{}: {}", f.file.as_deref().unwrap_or_default(), f.message))
        .collect()
}

const ENGINE_MANIFEST: &str =
    "[package]\nname = \"egs-engine\"\nlicense.workspace = true\n[lints]\nworkspace = true\n";

/// F4 reproduction: the lint name arrives as a macro parameter.
const MACRO_OPT_OUT: &str = r"//! Engine.

macro_rules! lowered {
    ($lint:ident) => {
        /// Value.
        #[allow($lint)]
        pub fn value() -> i32 {
            let n = 1;
            let p = &n as *const i32;
            // SAFETY: p refers to n, which is alive and aligned.
            unsafe { *p }
        }
    };
}
lowered!(unsafe_code);
";

fn rustc_accepts(dir: &TempDir, source: &str) -> bool {
    let file = dir.path().join("lib.rs");
    std::fs::write(&file, source).expect("write");
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_owned());
    std::process::Command::new(rustc)
        .args([
            "--edition",
            "2024",
            "--crate-type",
            "lib",
            "--emit=metadata",
            "-D",
            "unsafe_code",
            "--out-dir",
        ])
        .arg(dir.path())
        .arg(&file)
        .output()
        .expect("run rustc")
        .status
        .success()
}

// F4: a macro-parameter opt-out passes a name-based scan and the compiler's
// `deny`, so every non-adapter crate root must `forbid` unsafe code.
#[test]
fn f4_macro_parameter_opt_out_is_rejected() {
    let repo = Repo::new();
    repo.write("Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]\n");
    repo.write("crates/engine/Cargo.toml", ENGINE_MANIFEST);
    repo.write("crates/engine/src/lib.rs", MACRO_OPT_OUT);
    repo.commit("feat: engine");
    let found = rust_policy::check_sources(&ctx(&repo)).expect("check");
    assert_eq!(found.len(), 1, "{:?}", files(&found));
    assert!(found[0].message.contains("forbid(unsafe_code)"));

    let forbidden =
        MACRO_OPT_OUT.replace("//! Engine.\n", "//! Engine.\n#![forbid(unsafe_code)]\n");
    repo.write("crates/engine/src/lib.rs", &forbidden);
    repo.commit("fix: forbid");
    assert!(
        rust_policy::check_sources(&ctx(&repo))
            .expect("check")
            .is_empty()
    );

    // The compiler is what makes `forbid` effective through the macro.
    let dir = TempDir::new("rustc");
    assert!(rustc_accepts(&dir, MACRO_OPT_OUT), "deny alone is bypassed");
    assert!(
        !rustc_accepts(&dir, &forbidden),
        "forbid rejects the opt-out"
    );

    // forbid under cfg_attr can be switched off and does not count.
    repo.write(
        "crates/engine/src/lib.rs",
        "//! Engine.\n#![cfg_attr(test, forbid(unsafe_code))]\n",
    );
    repo.commit("chore: conditional");
    assert_eq!(
        rust_policy::check_sources(&ctx(&repo))
            .expect("check")
            .len(),
        1
    );
}

// F4: every Cargo-selected root needs the forbid; solver adapters are exempt.
#[test]
fn f4_all_crate_roots_are_found() {
    let repo = Repo::new();
    repo.write("Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]\n");
    repo.write(
        "crates/engine/Cargo.toml",
        &format!("{ENGINE_MANIFEST}[[bin]]\nname = \"tool\"\npath = \"tools/tool.rs\"\n"),
    );
    let ok = "#![forbid(unsafe_code)]\nfn main() {}\n";
    for path in [
        "src/lib.rs",
        "src/main.rs",
        "src/bin/a.rs",
        "src/bin/b/main.rs",
        "tests/t.rs",
        "examples/e.rs",
        "benches/b.rs",
        "build.rs",
        "tools/tool.rs",
    ] {
        repo.write(&format!("crates/engine/{path}"), ok);
    }
    repo.write("crates/engine/tests/common/mod.rs", "pub fn helper() {}\n");
    repo.write("crates/solver-highs/Cargo.toml", ENGINE_MANIFEST);
    repo.write("crates/solver-highs/src/lib.rs", "//! Adapter.\n");
    repo.commit("feat: targets");
    assert!(
        rust_policy::check_sources(&ctx(&repo))
            .expect("check")
            .is_empty()
    );
    for path in [
        "tests/t.rs",
        "build.rs",
        "tools/tool.rs",
        "src/bin/b/main.rs",
    ] {
        repo.write(&format!("crates/engine/{path}"), "fn main() {}\n");
        repo.commit("chore: drop forbid");
        let found = rust_policy::check_sources(&ctx(&repo)).expect("check");
        assert_eq!(found.len(), 1, "{path}: {:?}", files(&found));
        repo.write(&format!("crates/engine/{path}"), ok);
        repo.commit("chore: restore");
    }
}

// F7: a build script selected with `build = "..."` is scanned as one, and
// conditional link attributes are inspected.
#[test]
fn f7_custom_build_script_and_conditional_link_are_rejected() {
    let script =
        "#![forbid(unsafe_code)]\nfn main() { println!(\"cargo:rustc-link-lib=python3.12\"); }\n";
    let repo = Repo::new();
    repo.write("Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]\n");
    repo.write(
        "crates/engine/Cargo.toml",
        &ENGINE_MANIFEST.replace(
            "license.workspace = true\n",
            "license.workspace = true\nbuild = \"native.rs\"\n",
        ),
    );
    repo.write(
        "crates/engine/src/lib.rs",
        "//! E.\n#![forbid(unsafe_code)]\n",
    );
    repo.write("crates/engine/native.rs", script);
    repo.commit("feat: native");
    let found = rust_policy::check_sources(&ctx(&repo)).expect("check");
    assert_eq!(found.len(), 1, "{:?}", files(&found));
    assert_eq!(found[0].file.as_deref(), Some("crates/engine/native.rs"));

    // The same file is ordinary source when Cargo does not select it.
    assert!(rust_policy::scan_rust("crates/engine/native.rs", script).is_empty());

    for source in [
        "#[cfg_attr(all(), link(name = \"julia\"))]\nunsafe extern \"C\" {}\n",
        "#[cfg_attr(unix, cfg_attr(all(), link(name = \"python3.12\")))]\nunsafe extern \"C\" {}\n",
        "macro_rules! m { () => { #[link(name = \"julia\")] unsafe extern \"C\" {} } }\n",
    ] {
        assert!(
            !rust_policy::scan_rust("crates/solver-highs/src/lib.rs", source).is_empty(),
            "{source}"
        );
    }
    assert!(
        rust_policy::scan_rust(
            "crates/solver-highs/src/lib.rs",
            "#[cfg_attr(all(), link(name = \"highs\"))]\nunsafe extern \"C\" {}\n"
        )
        .is_empty()
    );
}

// R3-01: only the bare Rust lint path in an inner `forbid` counts. A
// tool-qualified lint compiles, forbids nothing, and lets the macro opt-out
// through, so it must not satisfy the crate-root rule.
#[test]
fn r3_01_qualified_lint_does_not_satisfy_forbid() {
    let body = MACRO_OPT_OUT.replace("//! Engine.\n", "");
    let qualified =
        format!("//! Engine.\n#![allow(unknown_lints)]\n#![forbid(clippy::unsafe_code)]\n{body}");
    let repo = Repo::new();
    repo.write("Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]\n");
    repo.write("crates/engine/Cargo.toml", ENGINE_MANIFEST);
    repo.write("crates/engine/src/lib.rs", &qualified);
    repo.commit("feat: engine");
    let found = rust_policy::check_sources(&ctx(&repo)).expect("check");
    assert_eq!(found.len(), 1, "{:?}", files(&found));
    assert!(found[0].message.contains("forbid(unsafe_code)"));

    // The compiler proves the qualified form is ineffective.
    let dir = TempDir::new("rustc-r3");
    assert!(rustc_accepts(&dir, &qualified), "qualified lint enforced");

    for root in [
        "#![forbid(clippy::unsafe_code)]\n",
        "#![forbid(rustdoc::unsafe_code, dead_code)]\n",
        "#![forbid(lint(unsafe_code))]\n",
        "#![forbid(unsafe_code = \"x\")]\n",
        "#![forbid]\n",
        "#[forbid(unsafe_code)]\nfn f() {}\n",
    ] {
        assert!(
            !rust_policy::check_crate_root("crates/engine/src/lib.rs", root).is_empty(),
            "accepted: {root}"
        );
    }
    for root in [
        "#![forbid(unsafe_code)]\n",
        "#![forbid(dead_code, unsafe_code)]\n",
        "#![forbid(unsafe_code, reason = \"policy\")]\n",
    ] {
        assert!(
            rust_policy::check_crate_root("crates/engine/src/lib.rs", root).is_empty(),
            "rejected: {root}"
        );
        assert!(!rustc_accepts(&dir, &format!("{root}{body}")), "{root}");
    }
}

// R3-02: Cargo does not require `.rs` for an explicit target or build
// script; every Cargo-selected file is scanned whatever its name.
#[test]
fn r3_02_targets_without_rs_suffix_are_scanned() {
    let body = MACRO_OPT_OUT.replace("//! Engine.\n", "");
    let repo = Repo::new();
    repo.write("Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]\n");
    repo.write(
        "crates/engine/Cargo.toml",
        &(ENGINE_MANIFEST.replace(
            "license.workspace = true\n",
            "license.workspace = true\nbuild = \"native\"\n",
        ) + "[lib]\npath = \"engine\"\n[[bin]]\nname = \"tool\"\npath = \"tool.txt\"\n"),
    );
    repo.write("crates/engine/engine", &format!("//! Engine.\n{body}"));
    repo.write(
        "crates/engine/native",
        "fn main() { println!(\"cargo:rustc-link-lib=python3.12\"); }\n",
    );
    repo.write("crates/engine/tool.txt", "fn main() {}\n");
    repo.commit("feat: engine");
    let found = rust_policy::check_sources(&ctx(&repo)).expect("check");
    let text = files(&found);
    assert!(
        text.iter()
            .any(|f| f.starts_with("crates/engine/engine:") && f.contains("forbid")),
        "{text:?}"
    );
    assert!(
        text.iter()
            .any(|f| f.starts_with("crates/engine/native:") && f.contains("Python")),
        "{text:?}"
    );
    assert!(
        text.iter()
            .any(|f| f.starts_with("crates/engine/tool.txt:") && f.contains("forbid")),
        "{text:?}"
    );

    // A selected target that is not tracked fails rather than being skipped.
    repo.git(&["rm", "-q", "crates/engine/tool.txt"]);
    repo.commit("chore: remove");
    let found = rust_policy::check_sources(&ctx(&repo)).expect("check");
    assert!(
        files(&found)
            .iter()
            .any(|f| f.starts_with("crates/engine/tool.txt:") && f.contains("not a tracked")),
        "{:?}",
        files(&found)
    );

    // Control: with the forbid in place the same layout passes.
    let ok = "#![forbid(unsafe_code)]\nfn main() {}\n";
    repo.write(
        "crates/engine/engine",
        &format!("//! Engine.\n#![forbid(unsafe_code)]\n{body}"),
    );
    repo.write("crates/engine/native", ok);
    repo.write("crates/engine/tool.txt", ok);
    repo.commit("fix: forbid");
    let found = rust_policy::check_sources(&ctx(&repo)).expect("check");
    assert!(found.is_empty(), "{:?}", files(&found));
}

// ADV23R-06: an extensionless `.cargo/config` is checked, and Cargo config
// may hold only aliases.
#[test]
fn adv23r_06_cargo_config_variants_are_rejected() {
    for (path, text) in [
        (
            ".cargo/config",
            "[build]\nrustflags = [\"-A\", \"unsafe_code\"]\n",
        ),
        (
            ".cargo/config.toml",
            "[build]\nrustc-workspace-wrapper = \"./w\"\n",
        ),
        (
            "crates/engine/.cargo/config",
            "[target.x86_64-unknown-linux-gnu]\nlinker = \"./l\"\n",
        ),
        (".cargo/config.toml", "[env]\nRUSTFLAGS = \"-Awarnings\"\n"),
        (".cargo/config.toml", "build.rustc=\"./r\"\n"),
    ] {
        let repo = Repo::new();
        std::fs::copy(root().join("Cargo.toml"), repo.path().join("Cargo.toml")).expect("copy");
        repo.write(path, text);
        repo.commit("chore: config");
        let found = rust_policy::check_manifests(&ctx(&repo)).expect("check");
        assert!(
            found.iter().any(|f| f.file.as_deref() == Some(path)),
            "{path}: {text}: {:?}",
            files(&found)
        );
    }
    assert!(rust_policy::is_cargo_config(".cargo/config"));
    assert!(!rust_policy::is_cargo_config("docs/cargo/config"));
    assert!(
        rust_policy::check_cargo_config(".cargo/config.toml", "[alias]\nx = \"run\"\n").is_empty()
    );
}

// The repository itself passes every policy and provenance check.
#[test]
fn repository_passes_its_own_policy() {
    let ctx = Ctx::local(&root());
    assert_eq!(
        files(&rust_policy::check_sources(&ctx).expect("sources")),
        Vec::<String>::new()
    );
    assert_eq!(
        files(&rust_policy::check_manifests(&ctx).expect("manifests")),
        Vec::<String>::new()
    );
    assert_eq!(
        files(&repository::codeowners(&ctx).expect("codeowners")),
        Vec::<String>::new()
    );
    assert_eq!(
        files(&repository::provenance(&ctx).expect("provenance")),
        Vec::<String>::new()
    );
}

const MANIFEST: &str = "[[datasets]]\nname = \"demand\"\nfilename = \"demand.csv\"\nlicence_status = \"open-commercial\"\nlicence = \"NESO Open Data Licence\"\n";

// F9: every committed sample resolves to an open-commercial provenance record.
#[test]
fn f9_unregistered_sample_is_rejected() {
    let repo = Repo::new();
    repo.write("data/manifest.toml", MANIFEST);
    repo.write("spikes/data/samples/unregistered.csv", "a,b\n1,2\n");
    repo.commit("data: sample");
    let found = repository::provenance(&ctx(&repo)).expect("provenance");
    assert_eq!(found.len(), 1, "{:?}", files(&found));
    assert_eq!(
        found[0].file.as_deref(),
        Some("spikes/data/samples/unregistered.csv")
    );

    // Registered as open-commercial: accepted.
    let repo = Repo::new();
    repo.write("data/manifest.toml", MANIFEST);
    repo.write("spikes/data/samples/demand.csv", "a,b\n1,2\n");
    repo.commit("data: sample");
    assert!(
        repository::provenance(&ctx(&repo))
            .expect("provenance")
            .is_empty()
    );

    // Registered as research-only (sources.tsv): rejected.
    let repo = Repo::new();
    repo.write("data/manifest.toml", MANIFEST);
    repo.write(
        "spikes/data/sources.tsv",
        "# clearance\tlocal-name\turl\nresearch-only\tflows.json\thttps://example.org/f\n",
    );
    repo.write("spikes/data/samples/flows.json", "{}\n");
    repo.commit("data: sample");
    let found = repository::provenance(&ctx(&repo)).expect("provenance");
    assert!(
        found.iter().any(|f| f.message.contains("research-only")),
        "{:?}",
        files(&found)
    );
}

// F8: a narrower CODEOWNERS rule cannot take a protected file from the owner.
#[test]
fn f8_codeowners_override_is_rejected_end_to_end() {
    let repo = Repo::new();
    let codeowners = std::fs::read_to_string(root().join(".github/CODEOWNERS")).expect("read");
    repo.write(".github/CODEOWNERS", &codeowners);
    repo.write("xtask/src/lib.rs", "//! x\n");
    repo.commit("chore: owners");
    assert!(
        repository::codeowners(&ctx(&repo))
            .expect("owners")
            .is_empty()
    );
    repo.write(
        ".github/CODEOWNERS",
        &format!("{codeowners}/xtask/src/lib.rs @someone\n"),
    );
    repo.commit("chore: override");
    let found = repository::codeowners(&ctx(&repo)).expect("owners");
    assert_eq!(found.len(), 1, "{:?}", files(&found));
    assert!(found[0].message.contains("xtask/src/lib.rs"));
}

// R3-04: the CODEOWNERS matcher agrees with Git's own gitignore matcher
// (`git check-ignore --no-index`) on the syntax GitHub supports. The one
// documented divergence, a final `/*` that does not reach nested files, is
// covered by the unit tests and excluded here.
#[test]
fn r3_04_codeowners_matcher_agrees_with_git() {
    let repo = Repo::new();
    let paths = [
        "xtask/src/lib.rs",
        "xtask/Cargo.toml",
        "deny.toml",
        "a/b/c.rs",
        "a/xtask/src/lib.rs",
        "docs/x/y/z.md",
        "src/main.rs",
        // R4-02: a filename prefix before the literal final name.
        "xtask/evillib.rs",
        "xtask/lib.rs",
        "evillib.rs",
    ];
    for path in paths {
        repo.write(path, "x\n");
    }
    let patterns = [
        "*",
        "**",
        "*.rs",
        "/xtask/",
        "xtask/",
        "/xtask",
        "xtask",
        "src/",
        "/src/",
        "src",
        "sr?",
        "/xtask/sr?/lib.rs",
        "x?ask/",
        "?",
        "??",
        "/?/",
        "a/?/c.rs",
        "a/??/c.rs",
        "a/b/c.r?",
        "**/src",
        "**/src/",
        "**/src/*.rs",
        "a/**",
        "a/**/c.rs",
        "docs/**/z.md",
        "/docs/**",
        "b/c.rs",
        "c.rs",
        "lib.rs",
        "/lib.rs",
        "*/src/lib.rs",
        "/*/src/lib.rs",
        "x*k/s*/l*.rs",
        "deny.toml",
        "/deny.toml",
        "*.toml",
        "/a/b/",
        "a/b",
        "**/b/**",
        "d?cs/",
        "docs/*/y/",
        "*/x/",
        "?tas?/",
        "xtask/src/lib.rs/",
        // R4-02: `**/` must match whole directories only.
        "/xtask/**/lib.rs",
        "xtask/**/lib.rs",
        "**/lib.rs",
        "**/evillib.rs",
        "/**/lib.rs",
        "a**/c.rs",
        "a/**b/c.rs",
        "a/b**",
        "x**/",
        "***/lib.rs",
        "a/***/c.rs",
        "**/**/c.rs",
        "docs/**/",
        "xtask/**",
        "/xtask/**/*.rs",
        "**lib.rs",
    ];
    for pattern in patterns {
        repo.write(".gitignore", &format!("{pattern}\n"));
        let output = std::process::Command::new("git")
            .arg("-C")
            .arg(repo.path())
            .args(["check-ignore", "--no-index"])
            .args(paths)
            .output()
            .expect("git check-ignore");
        let ignored: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(str::to_owned)
            .collect();
        for path in paths {
            assert_eq!(
                repository::codeowners_match(pattern, path),
                ignored.iter().any(|line| line == path),
                "pattern {pattern:?} path {path:?}"
            );
        }
    }
}

// ADV23R3-07: provenance records under a non-canonical file name would not be
// read, so the file is rejected with an explanation; `manifest.toml` is the
// one canonical name.
#[test]
fn adv23r3_07_non_canonical_manifest_is_rejected_end_to_end() {
    let record = "[[datasets]]\nname = \"r\"\nlicence_status = \"research-only\"\nfile = \"restricted.dat\"\n";
    for name in ["provenance.toml", "data-manifest.toml", "manifest.json"] {
        let repo = Repo::new();
        repo.write("data/manifest.toml", MANIFEST);
        repo.write(&format!("spikes/data/{name}"), record);
        repo.write("spikes/data/restricted.dat", "x\n");
        repo.commit("data: restricted");
        let found = repository::provenance(&ctx(&repo)).expect("provenance");
        assert!(
            found.iter().any(|f| {
                f.file.as_deref() == Some(&format!("spikes/data/{name}")[..])
                    && f.message.contains("manifest.toml")
            }),
            "{name}: {:?}",
            files(&found)
        );
    }
    // The canonical name is read, and the restricted file is caught.
    let repo = Repo::new();
    repo.write("data/manifest.toml", MANIFEST);
    repo.write("spikes/data/manifest.toml", record);
    repo.write("spikes/data/restricted.dat", "x\n");
    repo.commit("data: restricted");
    let found = repository::provenance(&ctx(&repo)).expect("provenance");
    assert_eq!(found.len(), 1, "{:?}", files(&found));
    assert!(found[0].message.contains("research-only"));
}

// R4-03: a restricted file nested in an array of arrays is registered, so the
// full gate reports the committed research-only file.
#[test]
fn r4_03_nested_array_manifest_end_to_end() {
    let repo = Repo::new();
    repo.write(
        "data/manifest.toml",
        "[[datasets]]\nname = \"r\"\nlicence_status = \"research-only\"\nfiles = [[\"restricted.txt\"]]\n",
    );
    repo.write("data/restricted.txt", "synthetic\n");
    repo.commit("data: nested");
    let found = repository::provenance(&ctx(&repo)).expect("provenance");
    assert!(
        found.iter().any(|f| {
            f.file.as_deref() == Some("data/restricted.txt") && f.message.contains("research-only")
        }),
        "{:?}",
        files(&found)
    );
}

// R4-04: an extensionless manifest beside a valid canonical one is rejected;
// the decision depends neither on its suffix nor on the payload's.
#[test]
fn r4_04_extensionless_manifest_end_to_end() {
    let record = "[[datasets]]\nname = \"r\"\nlicence_status = \"research-only\"\nfile = \"restricted.txt\"\n";
    for name in ["provenance", "records", "records.txt"] {
        let repo = Repo::new();
        repo.write("data/manifest.toml", MANIFEST);
        repo.write(&format!("data/{name}"), record);
        repo.write("data/restricted.txt", "synthetic\n");
        repo.commit("data: extensionless");
        let found = repository::provenance(&ctx(&repo)).expect("provenance");
        assert!(
            found.iter().any(|f| {
                f.file.as_deref() == Some(&format!("data/{name}")[..])
                    && f.message.contains("manifest.toml")
            }),
            "{name}: {:?}",
            files(&found)
        );
    }
}

const WORKSPACE: &str = "[workspace]\nmembers = [\"crates/*\"]\n";

// R4-01: a production target, module or package under `spikes/` does not
// inherit the spikes exemption; only files no production package reaches do.
#[test]
fn r4_01_production_code_under_spikes_is_checked() {
    let body = MACRO_OPT_OUT.replace("//! Engine.\n", "");
    let forbid = "//! E.\n#![forbid(unsafe_code)]\n";
    let has = |found: &[Finding], file: &str, needle: &str| {
        found
            .iter()
            .any(|f| f.file.as_deref() == Some(file) && f.message.contains(needle))
    };

    // The reviewer's exact layout: `[lib] path` into spikes/.
    let repo = Repo::new();
    repo.write("Cargo.toml", WORKSPACE);
    repo.write(
        "crates/engine/Cargo.toml",
        &format!("{ENGINE_MANIFEST}[lib]\npath = \"../../spikes/engine.rs\"\n"),
    );
    repo.write("spikes/engine.rs", &body);
    repo.write(
        "spikes/standalone.rs",
        "#![allow(unsafe_code)]\nfn main() {}\n",
    );
    repo.commit("feat: engine in spikes");
    let found = rust_policy::check_sources(&ctx(&repo)).expect("check");
    assert!(
        has(&found, "spikes/engine.rs", "forbid(unsafe_code)"),
        "{:?}",
        files(&found)
    );
    // Control: an unreachable standalone spike stays exempt.
    assert!(
        !found
            .iter()
            .any(|f| f.file.as_deref() == Some("spikes/standalone.rs"))
    );
    repo.write(
        "spikes/engine.rs",
        &format!("#![forbid(unsafe_code)]\n{body}"),
    );
    repo.commit("fix: forbid");
    let found = rust_policy::check_sources(&ctx(&repo)).expect("check");
    assert!(found.is_empty(), "{:?}", files(&found));
    // Compiler control: deny alone admits the opt-out; forbid rejects it.
    let dir = TempDir::new("rustc-r4");
    assert!(rustc_accepts(&dir, &body));
    assert!(!rustc_accepts(
        &dir,
        &format!("#![forbid(unsafe_code)]\n{body}")
    ));

    // A module reached by `#[path]` and an `include!`, a build script, and a
    // `[[bin]]` in spikes/ are all checked.
    let repo = Repo::new();
    repo.write("Cargo.toml", WORKSPACE);
    repo.write(
        "crates/engine/Cargo.toml",
        &(ENGINE_MANIFEST.replace(
            "license.workspace = true\n",
            "license.workspace = true\nbuild = \"../../spikes/build.rs\"\n",
        ) + "[[bin]]\nname = \"t\"\npath = \"../../spikes/tool.rs\"\n"),
    );
    repo.write(
        "crates/engine/src/lib.rs",
        &format!("{forbid}#[path = \"../../../spikes/helper.rs\"]\nmod helper;\ninclude!(\"../../../spikes/inc.rs\");\n"),
    );
    repo.write("spikes/helper.rs", "#[allow(unsafe_code)]\npub fn f() {}\n");
    repo.write("spikes/inc.rs", "#[allow(unsafe_code)]\npub fn g() {}\n");
    repo.write("spikes/build.rs", "fn main() {}\n");
    repo.write("spikes/tool.rs", "fn main() {}\n");
    repo.commit("feat: reached");
    let found = rust_policy::check_sources(&ctx(&repo)).expect("check");
    for (file, needle) in [
        ("spikes/helper.rs", "lowered"),
        ("spikes/inc.rs", "lowered"),
        ("spikes/build.rs", "forbid(unsafe_code)"),
        ("spikes/tool.rs", "forbid(unsafe_code)"),
    ] {
        assert!(has(&found, file, needle), "{file}: {:?}", files(&found));
    }

    // A workspace member and a path dependency under spikes/ are production
    // packages: their manifests and roots are checked.
    let repo = Repo::new();
    repo.write("Cargo.toml", "[workspace]\nmembers = [\"crates/*\", \"spikes/member\"]\n[workspace.package]\nlicense = \"LicenseRef-EGS-Unreleased\"\n[workspace.lints.rust]\nunsafe_code = \"deny\"\n");
    repo.write(
        "crates/engine/Cargo.toml",
        &format!("{ENGINE_MANIFEST}[dependencies]\ndep = {{ path = \"../../spikes/dep\" }}\n"),
    );
    repo.write("crates/engine/src/lib.rs", forbid);
    repo.write("spikes/member/Cargo.toml", "[package]\nname = \"m\"\n");
    repo.write("spikes/member/src/lib.rs", "pub fn m() {}\n");
    repo.write("spikes/dep/Cargo.toml", "[package]\nname = \"d\"\n");
    repo.write("spikes/dep/src/lib.rs", "pub fn d() {}\n");
    repo.commit("feat: spike packages");
    let found = rust_policy::check_sources(&ctx(&repo)).expect("check");
    for file in ["spikes/member/src/lib.rs", "spikes/dep/src/lib.rs"] {
        assert!(
            has(&found, file, "forbid(unsafe_code)"),
            "{file}: {:?}",
            files(&found)
        );
    }
    let found = rust_policy::check_manifests(&ctx(&repo)).expect("manifests");
    for file in ["spikes/member/Cargo.toml", "spikes/dep/Cargo.toml"] {
        assert!(has(&found, file, "[lints]"), "{file}: {:?}", files(&found));
    }
}

// ADV23R4-01: string escapes, raw strings and split literals are decoded
// before link names are matched.
#[test]
fn adv23r4_01_escaped_link_names_are_rejected() {
    for source in [
        "#[link(name = \"\\x70ython3.12\")]\nunsafe extern \"C\" {}\n",
        "#[link(name = \"\\u{6a}ulia\")]\nunsafe extern \"C\" {}\n",
        "#[link(name = \"\\u{50}YTHON\")]\nunsafe extern \"C\" {}\n",
        "#[link(name = r\"python3\")]\nunsafe extern \"C\" {}\n",
        "#[cfg_attr(all(), link(name = \"\\x6a\\x75lia\"))]\nunsafe extern \"C\" {}\n",
    ] {
        assert!(
            !rust_policy::scan_rust("crates/solver-highs/src/lib.rs", source).is_empty(),
            "{source}"
        );
    }
    for source in [
        "fn main() { println!(\"cargo:rustc-link-lib=\\x70ython3.12\"); }\n",
        "fn main() { println!(concat!(\"cargo:rustc-link-lib=py\", \"thon3\")); }\n",
        "fn main() { let n = b\"\\x70ython3\"; let _ = n; }\n",
    ] {
        assert!(
            !rust_policy::scan_rust("crates/engine/build.rs", source).is_empty(),
            "{source}"
        );
    }
    // Control: an escaped permitted library is still allowed.
    assert!(
        rust_policy::scan_rust(
            "crates/solver-highs/src/lib.rs",
            "#[link(name = \"\\x68ighs\")]\nunsafe extern \"C\" {}\n"
        )
        .is_empty()
    );
}

// ADV23R4-02: every module a build script includes is build-script code, as
// is a path build-dependency.
#[test]
fn adv23r4_02_build_script_modules_are_scanned() {
    let forbid = "#![forbid(unsafe_code)]\n";
    let helper = "pub fn link() { println!(\"cargo:rustc-link-lib=python3.12\"); }\n";
    for (script, helper_path) in [
        (
            "#[path = \"src/build_helper.rs\"]\nmod build_helper;\nfn main() { build_helper::link(); }\n",
            "crates/engine/src/build_helper.rs",
        ),
        (
            "mod helper;\nfn main() { helper::link(); }\n",
            "crates/engine/helper.rs",
        ),
        (
            "include!(\"src/gen.rs\");\nfn main() { link(); }\n",
            "crates/engine/src/gen.rs",
        ),
        (
            "mod outer { pub mod inner; }\nfn main() { outer::inner::link(); }\n",
            "crates/engine/outer/inner.rs",
        ),
    ] {
        let repo = Repo::new();
        repo.write("Cargo.toml", WORKSPACE);
        repo.write("crates/engine/Cargo.toml", ENGINE_MANIFEST);
        repo.write("crates/engine/src/lib.rs", forbid);
        repo.write("crates/engine/build.rs", &format!("{forbid}{script}"));
        repo.write(helper_path, helper);
        repo.commit("feat: build helper");
        let found = rust_policy::check_sources(&ctx(&repo)).expect("check");
        assert!(
            found
                .iter()
                .any(|f| f.file.as_deref() == Some(helper_path) && f.message.contains("Python")),
            "{helper_path}: {:?}",
            files(&found)
        );
        // Control: the same file used only by the library is not build code.
        repo.write(
            "crates/engine/build.rs",
            &format!("{forbid}fn main() {{}}\n"),
        );
        repo.commit("chore: drop helper");
        let found = rust_policy::check_sources(&ctx(&repo)).expect("check");
        assert!(found.is_empty(), "{helper_path}: {:?}", files(&found));
    }

    // A path build-dependency's code runs inside the build script.
    let repo = Repo::new();
    repo.write("Cargo.toml", WORKSPACE);
    repo.write(
        "crates/engine/Cargo.toml",
        &format!("{ENGINE_MANIFEST}[build-dependencies]\nhelper = {{ path = \"../helper\" }}\n"),
    );
    repo.write("crates/engine/src/lib.rs", forbid);
    repo.write(
        "crates/engine/build.rs",
        &format!("{forbid}fn main() {{ helper::link(); }}\n"),
    );
    repo.write("crates/helper/Cargo.toml", ENGINE_MANIFEST);
    repo.write("crates/helper/src/lib.rs", &format!("{forbid}{helper}"));
    repo.commit("feat: build dependency");
    let found = rust_policy::check_sources(&ctx(&repo)).expect("check");
    assert!(
        found
            .iter()
            .any(|f| f.file.as_deref() == Some("crates/helper/src/lib.rs")
                && f.message.contains("Python")),
        "{:?}",
        files(&found)
    );

    // A module that cannot be resolved fails closed.
    let repo = Repo::new();
    repo.write("Cargo.toml", WORKSPACE);
    repo.write("crates/engine/Cargo.toml", ENGINE_MANIFEST);
    repo.write("crates/engine/src/lib.rs", forbid);
    repo.write(
        "crates/engine/build.rs",
        &format!("{forbid}mod missing;\ninclude!(concat!(\"sr\", \"c/x.rs\"));\nfn main() {{}}\n"),
    );
    repo.commit("feat: unresolved");
    let found = rust_policy::check_sources(&ctx(&repo)).expect("check");
    assert_eq!(
        found
            .iter()
            .filter(|f| f.message.contains("cannot be resolved"))
            .count(),
        2,
        "{:?}",
        files(&found)
    );
}

// ADV23R4-03: a composite action outside .github/actions/ is inspected, and a
// workflow cannot reference a local action outside .github/actions/.
#[test]
fn adv23r4_03_action_outside_github_actions_end_to_end() {
    let repo = Repo::new();
    repo.write(
        "actions/helper/action.yml",
        "name: helper\nruns:\n  using: composite\n  steps:\n    - shell: python\n      run: |\n        import os\n        print(1)\n",
    );
    repo.write(
        ".github/workflows/ci.yml",
        "on: pull_request\npermissions: {contents: read}\njobs:\n  a:\n    runs-on: ubuntu-24.04\n    steps:\n      - uses: ./actions/helper\n",
    );
    repo.commit("ci: helper");
    let found = xtask::workflows::check(&ctx(&repo)).expect("workflows");
    let text = files(&found);
    assert!(
        text.iter()
            .any(|f| f.starts_with("actions/helper/action.yml:")),
        "{text:?}"
    );
    assert!(
        text.iter()
            .any(|f| f.starts_with(".github/workflows/ci.yml:") && f.contains("./actions/helper")),
        "{text:?}"
    );
}
