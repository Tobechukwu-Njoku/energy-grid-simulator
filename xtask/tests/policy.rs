//! End-to-end repository policy checks over scratch repositories
//! (F4, F7, F8, F9, ADV23R-06, ADV23R-09), using the real trusted policy.

#![forbid(unsafe_code)]

mod common;

use common::{Repo, root};
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
        "***/lib.rs",
        "a/***/c.rs",
        "**/**/c.rs",
        "docs/**/",
        "xtask/**",
        "/xtask/**/*.rs",
    ];
    // Partial `**` runs (`a**/c.rs`, `x/**y`) are matched differently by Git
    // releases; CODEOWNERS rejects them (`repository::r4_02_...`).
    for pattern in patterns {
        assert!(!xtask::util::partial_globstar(pattern), "{pattern}");
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
