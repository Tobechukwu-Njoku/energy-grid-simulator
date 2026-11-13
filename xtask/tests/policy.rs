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
