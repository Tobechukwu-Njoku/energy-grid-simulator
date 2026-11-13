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
