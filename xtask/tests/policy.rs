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
    let record = "[[datasets]]\nname = \"r\"\nlicence_status = \"research-only\"\nfile = \"restricted.bin\"\n";
    for name in ["provenance.toml", "data-manifest.toml", "manifest.json"] {
        let repo = Repo::new();
        repo.write("data/manifest.toml", MANIFEST);
        repo.write(&format!("spikes/data/{name}"), record);
        repo.write("spikes/data/restricted.bin", "x\n");
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
    repo.write("spikes/data/restricted.bin", "x\n");
    repo.commit("data: restricted");
    let found = repository::provenance(&ctx(&repo)).expect("provenance");
    // ADV23R11-02: `.bin` is also an unlisted type, so it is data as well.
    assert!(
        found
            .iter()
            .all(|f| f.file.as_deref() == Some("spikes/data/restricted.bin")),
        "{:?}",
        files(&found)
    );
    assert!(found.iter().any(|f| f.message.contains("research-only")));
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

// ADV23R8-01: `cargo xtask policy` rejects a pull-request workflow that defines
// the required `merge-gates` check (the reviewer's input); the repository's
// own workflows keep their gate names.
#[test]
fn adv23r8_01_spoofed_merge_gates_workflow_end_to_end() {
    let repo = Repo::new();
    for path in [
        ".github/workflows/ci.yml",
        ".github/workflows/trusted-gates.yml",
    ] {
        repo.write(
            path,
            &std::fs::read_to_string(root().join(path)).expect("workflow"),
        );
    }
    repo.commit("ci: workflows");
    let clean = xtask::workflows::check(&ctx(&repo)).expect("workflows");
    assert!(
        !files(&clean).iter().any(|f| f.contains("check name")),
        "{:?}",
        files(&clean)
    );
    repo.write(
        ".github/workflows/spoof.yml",
        "name: spoof\non: [pull_request]\npermissions:\n  contents: read\njobs:\n  merge-gates:\n    runs-on: ubuntu-24.04\n    steps:\n      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1\n        with:\n          persist-credentials: false\n      - run: echo \"bypass\"\n",
    );
    repo.commit("ci: spoof");
    let found = files(&xtask::workflows::check(&ctx(&repo)).expect("workflows"));
    assert!(
        found
            .iter()
            .any(|f| f.starts_with(".github/workflows/spoof.yml:") && f.contains("`merge-gates`")),
        "{found:?}"
    );
}

/// A scratch repository holding the real workflows, with `edit` applied to
/// ci.yml and `extra` added as `.github/workflows/spoof.yml`.
fn workflow_findings(edit: &dyn Fn(String) -> String, extra: Option<&str>) -> Vec<String> {
    let repo = Repo::new();
    for path in [
        ".github/workflows/ci.yml",
        ".github/workflows/trusted-gates.yml",
    ] {
        let text = std::fs::read_to_string(root().join(path)).expect("workflow");
        let text = if path.ends_with("ci.yml") {
            edit(text)
        } else {
            text
        };
        repo.write(path, &text);
    }
    if let Some(extra) = extra {
        repo.write(".github/workflows/spoof.yml", extra);
    }
    repo.commit("ci: workflows");
    files(&xtask::workflows::check(&ctx(&repo)).expect("workflows"))
}

// R9-02 / ADV23R9-01: a job of another workflow publishing a gate job's
// display name (not its id) passed at e64acc5; now every gate's published
// name is protected, case-insensitively. Controls: the real workflows and an
// unrelated name pass.
#[test]
fn r9_02_adv23r9_01_gate_display_names_are_protected_end_to_end() {
    assert_eq!(workflow_findings(&|text| text, None), Vec::<String>::new());
    let spoof = |name: &str| {
        format!(
            "name: spoof\non: [pull_request]\npermissions: {{contents: read}}\njobs:\n  spoof:\n    name: {name}\n    runs-on: ubuntu-24.04\n    steps:\n      - run: echo bypass\n"
        )
    };
    for name in [
        "attribution policy (D-52)",
        "rust (fmt, clippy, nextest incl. xtask and hook tests) linux",
        "PROVENANCE, SECRETS AND RAW DATA (D-56)",
        "merge-gates",
    ] {
        let found = workflow_findings(&|text| text, Some(&spoof(&format!("\"{name}\""))));
        assert!(
            found
                .iter()
                .any(|f| f.starts_with(".github/workflows/spoof.yml:")
                    && f.contains("reuses the check name")),
            "{name}: {found:?}"
        );
    }
    let found = workflow_findings(&|text| text, Some(&spoof("release notes")));
    assert!(found.is_empty(), "{found:?}");
    // A gate job of ci.yml may not change its published name either.
    let found = workflow_findings(
        &|text| text.replace("name: attribution policy (D-52)", "name: attribution"),
        None,
    );
    assert!(
        found
            .iter()
            .any(|f| f.contains("must publish the check name")),
        "{found:?}"
    );
}

// ADV23R9-02: the aggregator must publish `merge-gates` and run main's exact
// steps, which evaluate every `needs` result. A bypass step and a renamed
// aggregator passed at e64acc5.
#[test]
fn adv23r9_02_aggregator_name_and_steps_end_to_end() {
    let start = "      - name: Require every applicable gate\n";
    let bypass = |text: String| {
        let at = text.find(start).expect("aggregator step");
        format!("{}      - run: echo \"bypassed\"\n", &text[..at])
    };
    let found = workflow_findings(&bypass, None);
    assert!(
        found.iter().any(|f| f.contains("steps differ from main's")),
        "{found:?}"
    );
    let renamed = |text: String| text.replace("    name: merge-gates\n", "    name: random-name\n");
    let found = workflow_findings(&renamed, None);
    assert!(
        found.iter().any(|f| f.contains("`merge-gates`")),
        "{found:?}"
    );
    let soft = |text: String| {
        text.replace(
            "    name: merge-gates\n",
            "    name: merge-gates\n    continue-on-error: true\n",
        )
    };
    let found = workflow_findings(&soft, None);
    assert!(
        found
            .iter()
            .any(|f| f.contains("may not set `continue-on-error`")),
        "{found:?}"
    );
    // One changed command inside the step is also a difference.
    let weakened = |text: String| text.replace("test \"$failed\" -eq 0", "true");
    let found = workflow_findings(&weakened, None);
    assert!(
        found.iter().any(|f| f.contains("steps differ from main's")),
        "{found:?}"
    );
}

// ADV23R9-03: `BASH_ENV` (and any variable not on the trusted allowlist) in a
// gate job's `env:` passed at e64acc5.
#[test]
fn adv23r9_03_startup_env_end_to_end() {
    for (name, why) in [
        ("BASH_ENV", "start-up"),
        ("PYTHONPATH", "start-up"),
        ("GIT_CONFIG_COUNT", "start-up"),
        ("SOME_NEW_HOOK", "allowed_env"),
    ] {
        let edit = |text: String| {
            text.replace(
                "    name: repository policy (unsafe scope, manifests, workflows, Python)\n",
                &format!(
                    "    name: repository policy (unsafe scope, manifests, workflows, Python)\n    env:\n      {name}: /tmp/injected.sh\n"
                ),
            )
        };
        let found = workflow_findings(&edit, None);
        assert!(
            found
                .iter()
                .any(|f| f.contains(&format!("`{name}`")) && f.contains(why)),
            "{name}: {found:?}"
        );
    }
}

// R9-03 / ADV23R9-05: the GPT RDF/XML fixture without an XML declaration,
// a prefixed root, and solver models by extension and content all passed the
// provenance gate at e64acc5. Control: the same text as Markdown prose passes.
#[test]
fn r9_03_adv23r9_05_rdf_and_solver_models_end_to_end() {
    let rdf = "<RDF xmlns=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"\n     xmlns:cim=\"http://iec.ch/TC57/2013/CIM-schema-cim16#\">\n  <cim:BaseVoltage>\n    <cim:IdentifiedObject.name>400kV</cim:IdentifiedObject.name>\n    <cim:BaseVoltage.nominalVoltage>400</cim:BaseVoltage.nominalVoltage>\n  </cim:BaseVoltage>\n</RDF>\n";
    let mps = "NAME          UC\nROWS\n N  COST\nCOLUMNS\n    X1        COST         1.0\nENDATA\n";
    for (path, text) in [
        ("spikes/network-notes.txt", rdf.to_owned()),
        (
            "spikes/prefixed.txt",
            rdf.replace("<RDF xmlns=", "<r:RDF xmlns:r=")
                .replace("</RDF>", "</r:RDF>"),
        ),
        ("spikes/unit_commitment.mps", mps.to_owned()),
        (
            "spikes/europe_opf.lp",
            "Minimize\n obj: x\nSubject To\n c: x >= 1\nEnd\n".to_owned(),
        ),
        ("spikes/uc-model.txt", mps.to_owned()),
    ] {
        let repo = Repo::new();
        repo.write("data/manifest.toml", MANIFEST);
        repo.write(path, &text);
        repo.commit("data: model");
        let found = repository::provenance(&ctx(&repo)).expect("provenance");
        assert!(
            found
                .iter()
                .any(|f| f.file.as_deref() == Some(path) && f.message.contains("D-56")),
            "{path}: {:?}",
            files(&found)
        );
    }
    let repo = Repo::new();
    repo.write("data/manifest.toml", MANIFEST);
    repo.write(
        "docs/notes.md",
        "# CIM\nRDF/XML uses `<RDF xmlns=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">`.\n",
    );
    repo.commit("docs: notes");
    assert!(
        repository::provenance(&ctx(&repo))
            .expect("provenance")
            .is_empty()
    );
}

// R10-01 / ADV23R10-01: the GPT node-root RDF fixture with nested entities
// and a solver solution file passed provenance at 88148f0. A DTD file listed
// in the trusted `xml_dtd_allowed` is not data; Markdown quoting it is fine.
#[test]
fn r10_01_adv23r10_01_dtd_and_solutions_end_to_end() {
    let gpt = "<!DOCTYPE r:Description [\n<!ENTITY tail \"rdf-&#x73;yntax-ns#\">\n<!ENTITY ns \"http://www.w3.org/1999/02/22-&tail;\">\n]>\n<r:Description xmlns:r=\"&ns;\" r:about=\"urn:grid:bus:1\">\n  <bus:nominalVoltage xmlns:bus=\"urn:grid:terms:\">400</bus:nominalVoltage>\n</r:Description>\n";
    for (path, text) in [
        ("spikes/network-notes.txt", gpt),
        (
            "spikes/dispatch.sol",
            "# Solution for model unit_commitment\ngenerator_1 100.0\n",
        ),
        ("spikes/unit_commitment.mst", "generator_1 1\n"),
    ] {
        let repo = Repo::new();
        repo.write("data/manifest.toml", MANIFEST);
        repo.write(path, text);
        repo.commit("data: file");
        let found = repository::provenance(&ctx(&repo)).expect("provenance");
        assert!(
            found
                .iter()
                .any(|f| f.file.as_deref() == Some(path) && f.message.contains("D-56")),
            "{path}: {:?}",
            files(&found)
        );
    }
    let repo = Repo::new();
    repo.write("data/manifest.toml", MANIFEST);
    repo.write("docs/figure.svg", gpt);
    repo.write("docs/review.md", &format!("# Review\n\n```xml\n{gpt}```\n"));
    repo.commit("docs: dtd");
    let found = repository::provenance(&ctx(&repo)).expect("provenance");
    assert_eq!(files(&found).len(), 1, "{:?}", files(&found));
    let policy = Repo::new();
    let text = std::fs::read_to_string(root().join(repository::POLICY)).expect("policy");
    copy_secret_policy(&policy);
    policy.write(
        repository::POLICY,
        &text.replace(
            "xml_dtd_allowed = []",
            "xml_dtd_allowed = [\"docs/figure.svg\"]",
        ),
    );
    let allowed = Ctx {
        repo: repo.path().to_path_buf(),
        policy: policy.path().to_path_buf(),
    };
    let found = repository::provenance(&allowed).expect("provenance");
    assert!(found.is_empty(), "{:?}", files(&found));
}

// ADV23R10-02 (D-73): a Julia script in a crate passed `python` at 88148f0.
#[test]
fn adv23r10_02_julia_outside_spikes_end_to_end() {
    for (path, text) in [
        ("crates/validator.jl", "println(\"Julia script\")\n"),
        ("crates/run", "#!/usr/bin/env julia\nprintln(1)\n"),
        ("ci/run.js", "console.log(1)\n"),
    ] {
        let repo = Repo::new();
        repo.write(path, text);
        repo.commit("feat: script");
        let found = repository::python(&ctx(&repo)).expect("python");
        assert_eq!(found.len(), 1, "{path}: {:?}", files(&found));
        assert_eq!(found[0].file.as_deref(), Some(path));
    }
    let repo = Repo::new();
    repo.write("spikes/validate.jl", "println(1)\n");
    repo.write("crates/a/src/lib.rs", "#![forbid(unsafe_code)]\n");
    repo.write(".githooks/hook", "#!/bin/sh\nexit 0\n");
    repo.commit("feat: allowed");
    let found = repository::python(&ctx(&repo)).expect("python");
    assert!(found.is_empty(), "{:?}", files(&found));
}

// R11-01: a GitHub-token signature written as UTF-16 text passed
// `provenance` at d71cf40; the UTF-8 control was already rejected.
#[test]
fn r11_01_encoded_secret_end_to_end() {
    let line = format!(
        "service_token={}{}\n",
        ["gh", "p_"].concat(),
        "0".repeat(36)
    );
    let le: Vec<u8> = [0xff, 0xfe]
        .into_iter()
        .chain(line.encode_utf16().flat_map(u16::to_le_bytes))
        .collect();
    let be: Vec<u8> = line.encode_utf16().flat_map(u16::to_be_bytes).collect();
    let utf32: Vec<u8> = line
        .chars()
        .flat_map(|c| u32::from(c).to_le_bytes())
        .collect();
    for bytes in [line.as_bytes().to_vec(), le, be, utf32] {
        let repo = Repo::new();
        repo.write("data/manifest.toml", MANIFEST);
        std::fs::create_dir_all(repo.path().join("docs")).expect("dir");
        std::fs::write(repo.path().join("docs/connection-notes.txt"), &bytes).expect("write");
        repo.commit("docs: notes");
        let found = repository::provenance(&ctx(&repo)).expect("provenance");
        assert_eq!(found.len(), 1, "{bytes:?}: {:?}", files(&found));
        assert!(found[0].message.contains("github"), "{:?}", files(&found));
        assert_eq!(found[0].line, Some(1));
    }
    // Undecodable bytes under an allowed text type are data.
    let repo = Repo::new();
    repo.write("data/manifest.toml", MANIFEST);
    std::fs::create_dir_all(repo.path().join("docs")).expect("dir");
    std::fs::write(repo.path().join("docs/blob.txt"), b"ab\x80\x81\xfe\xff").expect("write");
    repo.commit("docs: blob");
    let found = repository::provenance(&ctx(&repo)).expect("provenance");
    assert_eq!(found.len(), 1, "{:?}", files(&found));
    assert!(found[0].message.contains(repository::UNDECODABLE));
}

// ADV23R11-01: an extensionless Python script behind a UTF-8 BOM passed
// `python` at d71cf40.
#[test]
fn adv23r11_01_bom_script_end_to_end() {
    for interpreter in ["python3", "julia"] {
        let repo = Repo::new();
        let text = format!("\u{feff}#!/usr/bin/env {interpreter}\nprint(1)\n");
        repo.write("crates/run_tool", &text);
        repo.commit("feat: tool");
        let found = repository::python(&ctx(&repo)).expect("python");
        assert_eq!(found.len(), 1, "{interpreter}: {:?}", files(&found));
        assert_eq!(found[0].file.as_deref(), Some("crates/run_tool"));
    }
    let repo = Repo::new();
    repo.write(".githooks/hook", "\u{feff}#!/bin/sh\nexit 0\n");
    repo.commit("chore: hook");
    assert!(repository::python(&ctx(&repo)).expect("python").is_empty());
}

// ADV23R11-02: solver, SAT/SMT and AMPL files missing from the d71cf40 data
// list passed `provenance`; file types are now an allowlist.
#[test]
fn adv23r11_02_unlisted_types_end_to_end() {
    let repo = Repo::new();
    repo.write("data/manifest.toml", MANIFEST);
    let inputs = [
        ("spikes/conflict.iis", "CONFLICT\n"),
        ("spikes/model.svf", "SAVESTATE\n"),
        ("spikes/problem.cnf", "p cnf 3 2\n"),
        ("spikes/problem.smt2", "(assert true)\n"),
        ("crates/solve.run", "solve; display x;\n"),
        ("docs/table.txt", "a,b,c\n1,2,3\n4,5,6\n7,8,9\n10,11,12\n"),
    ];
    for (path, text) in inputs {
        repo.write(path, text);
    }
    repo.write("docs/notes.txt", "Plain notes, with a comma.\n");
    repo.write("crates/a/src/lib.rs", "#![forbid(unsafe_code)]\n");
    repo.commit("data: files");
    let found = repository::provenance(&ctx(&repo)).expect("provenance");
    let mut flagged: Vec<&str> = found.iter().filter_map(|f| f.file.as_deref()).collect();
    flagged.sort_unstable();
    let mut expected: Vec<&str> = inputs.iter().map(|(path, _)| *path).collect();
    expected.sort_unstable();
    assert_eq!(flagged, expected, "{:?}", files(&found));
}

/// Copies the trusted secret-scanning policy into a scratch policy tree.
fn copy_secret_policy(policy: &Repo) {
    for file in [
        xtask::secrets::LOCAL_RULES,
        "xtask/policy/gitleaks.toml",
        "xtask/policy/gitleaks-LICENSE.txt",
    ] {
        policy.write(
            file,
            &std::fs::read_to_string(root().join(file)).expect("secret policy"),
        );
    }
}

fn provenance_of(path: &str, bytes: &[u8]) -> Vec<Finding> {
    let repo = Repo::new();
    repo.write("data/manifest.toml", MANIFEST);
    let full = repo.path().join(path);
    std::fs::create_dir_all(full.parent().expect("parent")).expect("dir");
    std::fs::write(full, bytes).expect("write");
    repo.commit("docs: synthetic notes");
    repository::provenance(&ctx(&repo)).expect("provenance")
}

// R12-01 (CORE): the reviewer's exact 58-byte file passed `provenance` at
// d996aac: `41 00` selected UTF-16LE and the raw ASCII token was never scanned.
#[test]
fn r12_01_encoding_guess_cannot_hide_raw_token_end_to_end() {
    let token = ["gh", "p_", &"0".repeat(36)].concat();
    let bytes = [b"A\x00service_token=", token.as_bytes(), b"\n "].concat();
    assert_eq!(bytes.len(), 58);
    let found = provenance_of("docs/connection-notes.txt", &bytes);
    assert_eq!(found.len(), 1, "{:?}", files(&found));
    assert_eq!(found[0].line, Some(1));
    assert!(found[0].message.contains("raw view"), "{:?}", files(&found));
    // Harmless control with the same prefix.
    assert!(provenance_of("docs/connection-notes.txt", b"A\x00harmless\n ").is_empty());
}

// ADV23R12-01 (CORE): the reviewer's PuTTY key in `crates/putty_key.txt`
// passed `provenance` at d996aac.
#[test]
fn adv23r12_01_putty_key_end_to_end() {
    let key = concat!(
        "PuTTY-User-Key-File",
        "-3: ssh-ed25519\nEncryption: none\nComment: test-key\n",
        "Public-Lines: 2\nAAAAC3NzaC1lZDI1NTE5AAAAIGx5\nPrivate-Lines: 1\nAAAAYCe\n"
    );
    let found = provenance_of("crates/putty_key.txt", key.as_bytes());
    assert_eq!(found.len(), 1, "{:?}", files(&found));
    assert!(found[0].message.contains("putty"), "{:?}", files(&found));
}

// ADV23R12-02 (CORE): the reviewer's GitLab runner and Perplexity tokens in
// `crates/token.rs` passed `provenance` at d996aac.
#[test]
fn adv23r12_02_vendored_formats_end_to_end() {
    for text in [
        [
            "gitlab_runner_token=\"gl",
            "rt-1234567890abcdef1234567890\"\n",
        ]
        .concat(),
        [
            "perplexity_key=\"pp",
            "lx-abcdef1234567890abcdef1234567890abcdef1234567890\"\n",
        ]
        .concat(),
    ] {
        let found = provenance_of("crates/token.rs", text.as_bytes());
        assert_eq!(found.len(), 1, "{text}: {:?}", files(&found));
    }
}

// ADV23R12-01/02 class fix: the vendored ruleset is pinned by blob id, and a
// ruleset xtask cannot fully evaluate fails the check instead of skipping.
#[test]
fn vendored_secret_ruleset_fails_closed() {
    let repo = Repo::new();
    repo.write("data/manifest.toml", MANIFEST);
    repo.commit("docs: notes");
    let vendored =
        std::fs::read_to_string(root().join("xtask/policy/gitleaks.toml")).expect("vendored");
    let local = std::fs::read_to_string(root().join(xtask::secrets::LOCAL_RULES)).expect("local");
    let cases = [
        // A locally edited vendored copy no longer matches the pin.
        (
            vendored.replace("ghp_[0-9a-zA-Z]{36}", "ghp_[0-9a-zA-Z]{99}"),
            local.clone(),
        ),
        // A local rule that does not compile.
        (
            vendored.clone(),
            format!("{local}\n[[rules]]\nid = \"broken\"\nregex = \"(unclosed\"\n"),
        ),
        // A schema feature xtask does not evaluate.
        (
            vendored.clone(),
            format!(
                "{local}\n[[rules]]\nid = \"composite\"\nregex = \"x\"\n[[rules.required]]\nid = \"y\"\n"
            ),
        ),
    ];
    for (vendored, local) in cases {
        let policy = Repo::new();
        copy_secret_policy(&policy);
        policy.write(
            repository::POLICY,
            &std::fs::read_to_string(root().join(repository::POLICY)).expect("policy"),
        );
        policy.write("xtask/policy/gitleaks.toml", &vendored);
        policy.write(xtask::secrets::LOCAL_RULES, &local);
        let trusted = Ctx {
            repo: repo.path().to_path_buf(),
            policy: policy.path().to_path_buf(),
        };
        assert!(repository::provenance(&trusted).is_err());
    }
}

// The tracked vendored ruleset quotes its own patterns and upstream example
// keys: only the exact pinned copy is not scanned; any edit to it is.
#[test]
fn only_the_pinned_vendored_ruleset_is_not_scanned() {
    let vendored =
        std::fs::read_to_string(root().join("xtask/policy/gitleaks.toml")).expect("vendored");
    let repo = Repo::new();
    repo.write("data/manifest.toml", MANIFEST);
    repo.write("xtask/policy/gitleaks.toml", &vendored);
    repo.commit("chore: vendored ruleset");
    let found = repository::provenance(&ctx(&repo)).expect("provenance");
    assert!(found.is_empty(), "{:?}", files(&found));
    let token = ["gh", "p_", &"0".repeat(36)].concat();
    repo.write(
        "xtask/policy/gitleaks.toml",
        &format!("{vendored}# {token}\n"),
    );
    repo.commit("chore: edited");
    let found = repository::provenance(&ctx(&repo)).expect("provenance");
    assert!(
        found
            .iter()
            .any(|f| f.message.contains("local-github-token")),
        "{:?}",
        files(&found)
    );
    // The same pinned content at another path is scanned.
    let repo = Repo::new();
    repo.write("data/manifest.toml", MANIFEST);
    repo.write("docs/rules.toml", &vendored);
    repo.commit("docs: copy");
    assert!(
        !repository::provenance(&ctx(&repo))
            .expect("provenance")
            .is_empty()
    );
}

// R13-01 (CORE): at 9db2a40 the exemption for the round-12 report named only
// the indented PuTTY header line, so replacing the synthetic body and MAC
// below it with a real key still passed. Exemptions now hash the whole block.
#[test]
fn r13_01_exempt_header_does_not_admit_a_new_key_body() {
    const REPORT: &str = "docs/requirements/reviews/gemini-2026-11-14-pr23-r12.md";
    let text = std::fs::read_to_string(root().join(REPORT)).expect("report");
    let repo = Repo::new();
    repo.write("data/manifest.toml", MANIFEST);
    repo.write(REPORT, &text);
    repo.commit("docs: reviewed report");
    let found = repository::provenance(&ctx(&repo)).expect("provenance");
    assert!(found.is_empty(), "{:?}", files(&found));
    // Same approved headers, a different body and a MAC under ADV23R12-01.
    let body = "  Private-Lines: 1\n  AAAAYCe\n";
    assert_eq!(text.matches(body).count(), 1);
    let changed = text.replace(
        body,
        "  Private-Lines: 1\n  AAAAQLnew0therBodyFromARealKey\n  Private-MAC: 5e1a\n",
    );
    repo.write(REPORT, &changed);
    repo.commit("docs: edit report");
    let found = repository::provenance(&ctx(&repo)).expect("provenance");
    assert!(
        found
            .iter()
            .any(|f| f.message.contains("local-putty-private-key")),
        "{:?}",
        files(&found)
    );
}

fn history(repo: &Repo, base: &str) -> Vec<Finding> {
    let range = repository::History {
        base: Some(base.to_owned()),
        head: "HEAD".to_owned(),
    };
    repository::provenance_range(&ctx(repo), Some(&range)).expect("provenance")
}

// R13-03 (CORE): at 9db2a40 a secret added in one commit and deleted (or
// replaced) in the next left the final tree clean, and provenance passed.
// Every file version the range introduced is now scanned.
#[test]
fn r13_03_secrets_and_data_added_then_removed_in_a_range_fail() {
    let token = ["gh", "p_", &"0".repeat(36)].concat();
    let start = || {
        let repo = Repo::new();
        repo.write("data/manifest.toml", MANIFEST);
        let base = repo.commit("docs: start");
        (repo, base)
    };
    // Add, then delete.
    let (repo, base) = start();
    repo.write("notes.txt", &format!("service_token={token}\n"));
    let added = repo.commit("docs: notes");
    std::fs::remove_file(repo.path().join("notes.txt")).expect("rm");
    repo.commit("docs: tidy");
    assert!(
        repository::provenance(&ctx(&repo))
            .expect("tree")
            .is_empty()
    );
    let found = history(&repo, &base);
    assert_eq!(found.len(), 1, "{:?}", files(&found));
    assert!(found[0].message.contains("local-github-token"));
    assert!(
        found[0].message.contains(&added[..12]),
        "{:?}",
        files(&found)
    );
    // Add, then replace with harmless text.
    let (repo, base) = start();
    repo.write("notes.txt", &format!("service_token={token}\n"));
    repo.commit("docs: notes");
    repo.write("notes.txt", "harmless\n");
    repo.commit("docs: reword");
    assert_eq!(history(&repo, &base).len(), 1);
    // On a side branch, removed there, then merged.
    let (repo, base) = start();
    repo.git(&["checkout", "-q", "-b", "side"]);
    repo.write("docs/key.md", &format!("token {token}\n"));
    repo.commit("docs: side");
    repo.write("docs/key.md", "gone\n");
    repo.commit("docs: side tidy");
    repo.git(&["checkout", "-q", "main"]);
    repo.write("docs/other.md", "x\n");
    repo.commit("docs: other");
    repo.git(&["merge", "-q", "--no-ff", "-m", "merge side", "side"]);
    let found = history(&repo, &base);
    assert_eq!(found.len(), 1, "{:?}", files(&found));
    // Raw and research-only data, added then deleted.
    let (repo, _) = start();
    repo.write(
        "data/manifest.toml",
        &format!(
            "{MANIFEST}[[datasets]]\nname = \"prices\"\nfilename = \"prices.csv\"\nlicence_status = \"research-only\"\nlicence = \"Research only\"\n"
        ),
    );
    repo.commit("data: record");
    let base2 = repo.git(&["rev-parse", "HEAD"]);
    repo.write("data/raw/dump.txt", "x\n");
    repo.write("spikes/prices.csv", "a,b\n1,2\n");
    repo.commit("data: oops");
    for path in ["data/raw/dump.txt", "spikes/prices.csv"] {
        std::fs::remove_file(repo.path().join(path)).expect("rm");
    }
    repo.commit("data: remove");
    assert!(
        repository::provenance(&ctx(&repo))
            .expect("tree")
            .is_empty()
    );
    let found = history(&repo, &base2);
    let named: Vec<String> = files(&found);
    assert!(
        named.iter().any(|f| f.starts_with("data/raw/dump.txt")),
        "{named:?}"
    );
    assert!(
        named
            .iter()
            .any(|f| f.starts_with("spikes/prices.csv") && f.contains("research-only")),
        "{named:?}"
    );
    // Controls: a clean history passes, and so does a reviewed exempt block
    // added and later removed at its exempt path.
    let (repo, base) = start();
    repo.write("docs/a.md", "one\n");
    repo.commit("docs: a");
    repo.write("docs/a.md", "two\n");
    repo.commit("docs: b");
    assert!(history(&repo, &base).is_empty());
    const REPORT: &str = "docs/requirements/reviews/gemini-2026-11-14-pr23-r12.md";
    let text = std::fs::read_to_string(root().join(REPORT)).expect("report");
    let (repo, base) = start();
    repo.write(REPORT, &text);
    repo.commit("docs: report");
    std::fs::remove_file(repo.path().join(REPORT)).expect("rm");
    repo.commit("docs: drop report");
    let found = history(&repo, &base);
    assert!(found.is_empty(), "{:?}", files(&found));
}
