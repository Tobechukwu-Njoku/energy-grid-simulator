//! The compiler-enforced build gate and the `cargo metadata` graph checks over
//! real Cargo workspaces (R5-01..04, ADV23R5-01..05 and the earlier unsafe and
//! native-runtime findings). Each fixture is built for real by Cargo with the
//! trusted xtask binary as rustc wrapper and linker guard.

#![forbid(unsafe_code)]

mod common;

use std::path::Path;
use std::process::Command;

use common::{Repo, TempDir, root};
use xtask::build_gate;
use xtask::rust_policy;
use xtask::util::{Ctx, Finding};

const WORKSPACE: &str = "[workspace]
resolver = \"3\"
members = [\"crates/*\"]
[workspace.package]
version = \"0.0.0\"
edition = \"2024\"
license = \"LicenseRef-EGS-Unreleased\"
[workspace.lints.rust]
unsafe_code = \"deny\"
";

const FORBID: &str = "#![forbid(unsafe_code)]\n";

/// A reviewed solver adapter opts out of the workspace `deny`.
const ADAPTER: &str = "#![allow(unsafe_code)]\n";

/// A raw-pointer read: compiles under `deny(unsafe_code)` only if lowered.
const UNSAFE_FN: &str = "pub fn value() -> i32 {
    let n = 1;
    let p = &n as *const i32;
    // SAFETY: p points at n.
    unsafe { *p }
}
";

/// F4: the lint name arrives as a macro parameter.
const MACRO_OPT_OUT: &str = "macro_rules! lowered {
    ($lint:ident) => {
        #[allow($lint)]
        pub fn value() -> i32 {
            let n = 1;
            let p = &n as *const i32;
            // SAFETY: p points at n.
            unsafe { *p }
        }
    };
}
lowered!(unsafe_code);
";

/// The build-script helper that asks Cargo to link Python.
const PY_HELPER: &str = "pub fn emit() { println!(\"cargo:rustc-link-lib=python3.12\"); }\n";

fn package(name: &str) -> String {
    format!(
        "[package]\nname = \"{name}\"\nversion.workspace = true\nedition.workspace = true\nlicense.workspace = true\n[lints]\nworkspace = true\n"
    )
}

struct Fixture {
    repo: Repo,
    target: TempDir,
}

impl Fixture {
    fn new() -> Self {
        let repo = Repo::new();
        repo.write("Cargo.toml", WORKSPACE);
        repo.write("crates/engine/Cargo.toml", &package("engine"));
        repo.write("crates/engine/src/lib.rs", FORBID);
        Self {
            repo,
            target: TempDir::new("target"),
        }
    }

    fn write(&self, path: &str, text: &str) -> &Self {
        self.repo.write(path, text);
        self
    }

    fn ctx(&self) -> Ctx {
        Ctx {
            repo: self.repo.path().to_path_buf(),
            policy: root(),
        }
    }

    fn commit(&self) {
        let output = rust_policy::cargo(&self.ctx())
            .args(["generate-lockfile", "--offline", "--manifest-path"])
            .arg(self.repo.path().join("Cargo.toml"))
            .output()
            .expect("cargo");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        self.repo.commit("fixture");
    }

    /// Findings of the compiler gate.
    fn build(&self) -> Vec<Finding> {
        self.commit();
        build_gate::check(
            &self.ctx(),
            Path::new(env!("CARGO_BIN_EXE_xtask")),
            self.target.path(),
        )
        .expect("build gate")
    }

    /// Findings of the static `cargo metadata` graph check.
    fn graph(&self) -> Vec<Finding> {
        self.commit();
        rust_policy::check_sources(&self.ctx()).expect("check")
    }
}

fn text(findings: &[Finding]) -> String {
    findings
        .iter()
        .map(|f| {
            format!(
                "{}:{} {}",
                f.file.as_deref().unwrap_or_default(),
                f.line.unwrap_or_default(),
                f.message
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn assert_unsafe_rejected(found: &[Finding]) {
    assert!(
        found.iter().any(|f| f.message.contains("unsafe")),
        "expected a compiler unsafe_code error:\n{}",
        text(found)
    );
}

fn assert_runtime_rejected(found: &[Finding]) {
    assert!(
        found.iter().any(|f| f.message.contains("NFR-10")),
        "expected an NFR-10 native-runtime finding:\n{}",
        text(found)
    );
}

fn assert_clean(found: &[Finding]) {
    assert!(found.is_empty(), "expected no findings:\n{}", text(found));
}

// F4 / R3-01 through the compiler: a macro-parameter opt-out passes `deny`
// but not the gate's command-line `forbid`, even with no root attribute or a
// tool-qualified one; with the root forbid the ordinary build rejects it too.
#[test]
fn f4_macro_parameter_opt_out_is_rejected_by_the_compiler() {
    for root_attr in [
        "",
        "#![allow(unknown_lints)]\n#![forbid(clippy::unsafe_code)]\n",
    ] {
        let fixture = Fixture::new();
        fixture.write(
            "crates/engine/src/lib.rs",
            &format!("{root_attr}{MACRO_OPT_OUT}"),
        );
        assert_unsafe_rejected(&fixture.build());
        assert!(
            fixture
                .graph()
                .iter()
                .any(|f| f.message.contains("forbid(unsafe_code)")),
            "root rule"
        );
    }
    let fixture = Fixture::new();
    fixture.write(
        "crates/engine/src/lib.rs",
        &format!("{FORBID}pub fn f() {{}}\n"),
    );
    assert_clean(&fixture.build());
    assert_clean(&fixture.graph());
}

// ADV23-03, R5/ADV23-05: multiline, cfg_attr, nested and expect forms.
#[test]
fn attribute_spellings_cannot_lower_the_command_line_forbid() {
    for lowering in [
        "#![cfg_attr(all(), allow(unsafe_code))]\n",
        "#![allow(\n    unsafe_code\n)]\n",
        "#![cfg_attr(not(test), cfg_attr(all(), expect(unsafe_code)))]\n",
        "#![allow(r#unsafe_code)]\n",
    ] {
        let fixture = Fixture::new();
        fixture.write(
            "crates/engine/src/lib.rs",
            &format!("{lowering}{UNSAFE_FN}"),
        );
        assert_unsafe_rejected(&fixture.build());
    }
}

// R5-01: a computed include whose text merely contains OUT_DIR, of a tracked
// helper under spikes/, in a build script.
#[test]
fn r5_01_out_dir_substring_include_is_built_and_checked() {
    let fixture = Fixture::new();
    fixture
        .write("crates/engine/Cargo.toml", &package("engine"))
        .write("spikes/OUT_DIR-helper.rs", PY_HELPER)
        .write(
            "crates/engine/build.rs",
            "#![forbid(unsafe_code)]\ninclude!(concat!(\"../../spikes/\", \"OUT_DIR-helper.rs\"));\nfn main() { emit(); }\n",
        );
    assert_runtime_rejected(&fixture.build());
    // Unsafe code in the same kind of include is a compile error.
    fixture.write(
        "spikes/OUT_DIR-helper.rs",
        &format!("pub fn emit() {{}}\n{MACRO_OPT_OUT}"),
    );
    assert_unsafe_rejected(&fixture.build());
    // Control: a clean helper passes.
    fixture.write("spikes/OUT_DIR-helper.rs", "pub fn emit() {}\n");
    assert_clean(&fixture.build());
}

// R5-02: a renamed include macro in a build script and in a library.
#[test]
fn r5_02_renamed_include_macro_is_built_and_checked() {
    let fixture = Fixture::new();
    fixture.write("spikes/helper.rs", PY_HELPER).write(
        "crates/engine/build.rs",
        "#![forbid(unsafe_code)]\nuse std::include as load;\nload!(\"../../spikes/helper.rs\");\nfn main() { emit(); }\n",
    );
    assert_runtime_rejected(&fixture.build());

    let fixture = Fixture::new();
    fixture.write("spikes/opt_out.rs", MACRO_OPT_OUT).write(
        "crates/engine/src/lib.rs",
        "use std::include as load;\nload!(\"../../../spikes/opt_out.rs\");\n",
    );
    assert_unsafe_rejected(&fixture.build());
    // Control: the same alias of a clean file passes.
    fixture.write("spikes/opt_out.rs", "pub fn f() {}\n").write(
        "crates/engine/src/lib.rs",
        &format!("{FORBID}use std::include as load;\nload!(\"../../../spikes/opt_out.rs\");\n"),
    );
    assert_clean(&fixture.build());
}

// R5-03: a workspace member selected by a bracket glob under spikes/.
#[test]
fn r5_03_cargo_glob_members_come_from_cargo() {
    let fixture = Fixture::new();
    fixture
        .write(
            "Cargo.toml",
            &WORKSPACE.replace("[\"crates/*\"]", "[\"crates/*\", \"spikes/[m]ember\"]"),
        )
        .write("spikes/member/Cargo.toml", &package("member"))
        .write("spikes/member/src/lib.rs", MACRO_OPT_OUT);
    assert_unsafe_rejected(&fixture.build());
    let graph = fixture.graph();
    assert!(
        graph
            .iter()
            .any(|f| f.file.as_deref() == Some("spikes/member/src/lib.rs")
                && f.message.contains("forbid(unsafe_code)")),
        "{}",
        text(&graph)
    );
    // Control: not a member, so neither Cargo nor the policy builds it.
    fixture.write("Cargo.toml", WORKSPACE);
    assert_clean(&fixture.build());
    assert_clean(&fixture.graph());
}

// R5-04 / ADV23R5-05: a build-dependency resolved through a local
// `[patch.crates-io]` runs in the build script.
#[test]
fn r5_04_patched_build_dependency_is_built_and_checked() {
    let fixture = Fixture::new();
    fixture
        .write(
            "Cargo.toml",
            &format!("{WORKSPACE}[patch.crates-io]\nhelper = {{ path = \"crates/helper\" }}\n"),
        )
        .write(
            "crates/engine/Cargo.toml",
            &format!(
                "{}[build-dependencies]\nhelper = \"0.0.0\"\n",
                package("engine")
            ),
        )
        .write("crates/helper/Cargo.toml", &package("helper"))
        .write("crates/helper/src/lib.rs", &format!("{FORBID}{PY_HELPER}"))
        .write(
            "crates/engine/build.rs",
            "#![forbid(unsafe_code)]\nfn main() { helper::emit(); }\n",
        );
    assert_runtime_rejected(&fixture.build());
    // The patched package is a local package to the graph check as well.
    fixture.write(
        "crates/helper/src/lib.rs",
        &format!("{PY_HELPER}{MACRO_OPT_OUT}"),
    );
    assert_unsafe_rejected(&fixture.build());
    assert!(
        fixture
            .graph()
            .iter()
            .any(|f| f.file.as_deref() == Some("crates/helper/src/lib.rs")),
        "patched package root"
    );
    // Control: a clean helper.
    fixture.write(
        "crates/helper/src/lib.rs",
        &format!("{FORBID}pub fn emit() {{}}\n"),
    );
    assert_clean(&fixture.build());
}

// ADV23R5-02: an `include!` of a file generated into OUT_DIR from a tracked
// spikes/ file: the generated code is compiled, so it is checked.
#[test]
fn adv23r5_02_out_dir_includes_are_built_and_checked() {
    let build = "#![forbid(unsafe_code)]\nfn main() {\n    let out = std::env::var(\"OUT_DIR\").unwrap();\n    std::fs::copy(\"../../spikes/native.inc\", format!(\"{out}/native.rs\")).unwrap();\n    println!(\"cargo:rerun-if-changed=../../spikes/native.inc\");\n}\n";
    let include = "include!(concat!(env!(\"OUT_DIR\"), \"/native.rs\"));\n";
    let fixture = Fixture::new();
    fixture
        .write("spikes/native.inc", MACRO_OPT_OUT)
        .write("crates/engine/build.rs", build)
        .write("crates/engine/src/lib.rs", &format!("{FORBID}{include}"));
    assert_unsafe_rejected(&fixture.build());

    // In an adapter (unsafe allowed) the generated link request reaches the
    // linker through the library's test harness.
    let fixture = Fixture::new();
    fixture
        .write(
            "spikes/native.inc",
            "#[link(name = \"python3.12\")]\nunsafe extern \"C\" {}\n",
        )
        .write("crates/solver-highs/Cargo.toml", &package("solver-highs"))
        .write("crates/solver-highs/build.rs", build)
        .write(
            "crates/solver-highs/src/lib.rs",
            &format!("{ADAPTER}{include}"),
        );
    assert_runtime_rejected(&fixture.build());
    // Control: a clean generated file.
    fixture.write("spikes/native.inc", "pub fn f() {}\n");
    assert_clean(&fixture.build());
}

// ADV23R5-03: a macro-generated `#[path]` module next to a decoy file.
#[test]
fn adv23r5_03_macro_generated_path_modules_are_built_and_checked() {
    let decl = "macro_rules! decl {\n    () => {\n        #[path = \"native.inc\"]\n        mod native;\n    };\n}\ndecl!();\n";
    let fixture = Fixture::new();
    fixture
        .write("crates/engine/src/lib.rs", &format!("{FORBID}{decl}"))
        .write("crates/engine/src/native.rs", "// clean\n")
        .write("crates/engine/src/native.inc", UNSAFE_FN);
    assert_unsafe_rejected(&fixture.build());

    let fixture = Fixture::new();
    fixture
        .write("crates/solver-highs/Cargo.toml", &package("solver-highs"))
        .write(
            "crates/solver-highs/src/lib.rs",
            &format!("{ADAPTER}{decl}"),
        )
        .write("crates/solver-highs/src/native.rs", "// clean\n")
        .write(
            "crates/solver-highs/src/native.inc",
            "#[link(name = \"python3.12\")]\nunsafe extern \"C\" {}\n",
        );
    assert_runtime_rejected(&fixture.build());
}

// ADV23R5-04 (and ADV23-12, ADV23R4-01, F7): link requests assembled by a
// macro, written with escapes or behind cfg_attr reach the linker guard.
#[test]
fn adv23r5_04_generated_link_attributes_reach_the_linker_guard() {
    for source in [
        "macro_rules! link_lib {\n    ($lib:expr) => {\n        #[link(name = $lib)]\n        unsafe extern \"C\" {}\n    };\n}\nlink_lib!(\"python3.12\");\n",
        "#[link(name = \"\\x70ython3.12\")]\nunsafe extern \"C\" {}\n",
        "#[cfg_attr(all(), link(name = \"julia\"))]\nunsafe extern \"C\" {}\n",
    ] {
        let fixture = Fixture::new();
        fixture
            .write("crates/solver-highs/Cargo.toml", &package("solver-highs"))
            .write(
                "crates/solver-highs/src/lib.rs",
                &format!("{ADAPTER}{source}"),
            );
        assert_runtime_rejected(&fixture.build());
    }
    // Control: an ordinary system library links.
    let fixture = Fixture::new();
    fixture
        .write("crates/solver-highs/Cargo.toml", &package("solver-highs"))
        .write(
            "crates/solver-highs/src/lib.rs",
            &format!("{ADAPTER}macro_rules! link_lib {{\n    ($lib:expr) => {{\n        #[link(name = $lib)]\n        unsafe extern \"C\" {{}}\n    }};\n}}\nlink_lib!(\"m\");\n"),
        );
    assert_clean(&fixture.build());
}

// F7 / ADV23R4-02: a custom-path build script and its helper module ask
// Cargo for a runtime with an escaped name; the build-script output is read.
#[test]
fn build_script_link_requests_are_read_from_cargo_output() {
    let fixture = Fixture::new();
    fixture
        .write(
            "crates/engine/Cargo.toml",
            &package("engine").replace(
                "license.workspace = true\n",
                "license.workspace = true\nbuild = \"native\"\n",
            ),
        )
        .write(
            "crates/engine/native",
            "#![forbid(unsafe_code)]\n#[path = \"src/build_helper.rs\"]\nmod build_helper;\nfn main() { build_helper::emit(); }\n",
        )
        .write(
            "crates/engine/src/build_helper.rs",
            "pub fn emit() { println!(\"cargo::rustc-link-lib=static=\\x70ython3\"); }\n",
        );
    let found = fixture.build();
    assert!(
        found
            .iter()
            .any(|f| f.message.contains("engine") && f.message.contains("NFR-10")),
        "{}",
        text(&found)
    );
}

// ADV23R5-01 and the trusted flags: the gate's Cargo ignores every flag,
// wrapper and lint-cap variable of its caller, end to end through the CLI.
#[test]
fn adv23r5_01_caller_environment_cannot_weaken_the_gate() {
    let fixture = Fixture::new();
    fixture.write("crates/engine/src/lib.rs", MACRO_OPT_OUT);
    fixture.commit();
    let output = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .arg("--repo")
        .arg(fixture.repo.path())
        .arg("--policy")
        .arg(root())
        .arg("build-gate")
        .arg("--target-dir")
        .arg(fixture.target.path())
        .env("RUSTFLAGS", "--cap-lints=allow")
        .env("CARGO_ENCODED_RUSTFLAGS", "--cap-lints=allow")
        .env("CARGO_BUILD_RUSTFLAGS", "--cap-lints=allow")
        .env("RUSTC_WRAPPER", "/bin/true")
        .env("RUSTC_WORKSPACE_WRAPPER", "/bin/true")
        .env("CARGO_BUILD_RUSTC_WRAPPER", "/bin/true")
        .env("CARGO_TARGET_DIR", "/nonexistent/target")
        .output()
        .expect("run xtask");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(output.status.code(), Some(1), "{stdout}");
    assert!(stdout.contains("unsafe"), "{stdout}");
    assert!(!rust_policy::is_sanitised("CARGO_HOME"));
    for name in [
        "RUSTFLAGS",
        "CARGO_ENCODED_RUSTFLAGS",
        "RUSTC_WORKSPACE_WRAPPER",
        "CARGO_BUILD_RUSTC_WRAPPER",
        "CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER",
        "RUSTDOCFLAGS",
        "RUSTC",
    ] {
        assert!(rust_policy::is_sanitised(name), "{name}");
    }
}

// R4-01 / F4 / R3-02 on the metadata graph: every target Cargo selects,
// wherever its file lives and whatever its suffix, needs the root forbid;
// adapters are exempt; untracked or out-of-repository code fails.
#[test]
fn metadata_selects_every_target() {
    let fixture = Fixture::new();
    fixture.write(
        "crates/engine/Cargo.toml",
        &format!(
            "{}[lib]\npath = \"../../spikes/engine.rs\"\n[[bin]]\nname = \"tool\"\npath = \"tool.txt\"\n",
            package("engine").replace(
                "license.workspace = true\n",
                "license.workspace = true\nbuild = \"native\"\n"
            )
        ),
    );
    let plain = "fn main() {}\n";
    for path in [
        "crates/engine/tool.txt",
        "crates/engine/native",
        "crates/engine/src/bin/a.rs",
        "crates/engine/tests/t.rs",
        "crates/engine/examples/e.rs",
        "crates/engine/benches/b.rs",
    ] {
        fixture.write(path, plain);
    }
    fixture.write("spikes/engine.rs", "pub fn f() {}\n");
    fixture.write(
        "spikes/standalone.rs",
        "#![allow(unsafe_code)]\nfn main() {}\n",
    );
    fixture.write("crates/solver-highs/Cargo.toml", &package("solver-highs"));
    fixture.write("crates/solver-highs/src/lib.rs", "//! Adapter.\n");
    let found = fixture.graph();
    let missing: Vec<&str> = found
        .iter()
        .filter(|f| f.message.contains("forbid(unsafe_code)"))
        .filter_map(|f| f.file.as_deref())
        .collect();
    for path in [
        "spikes/engine.rs",
        "crates/engine/tool.txt",
        "crates/engine/native",
        "crates/engine/src/bin/a.rs",
        "crates/engine/tests/t.rs",
        "crates/engine/examples/e.rs",
        "crates/engine/benches/b.rs",
    ] {
        assert!(missing.contains(&path), "{path}: {}", text(&found));
    }
    assert_eq!(missing.len(), 7, "{}", text(&found));

    // A dependency outside the repository cannot be checked.
    let outside = TempDir::new("outside");
    std::fs::create_dir_all(outside.path().join("src")).expect("mkdir");
    std::fs::write(
        outside.path().join("Cargo.toml"),
        "[package]\nname = \"outside\"\nversion = \"0.0.0\"\nedition = \"2024\"\n",
    )
    .expect("write");
    std::fs::write(outside.path().join("src/lib.rs"), "").expect("write");
    let fixture = Fixture::new();
    fixture.write(
        "crates/engine/Cargo.toml",
        &format!(
            "{}[dependencies]\noutside = {{ path = {:?} }}\n",
            package("engine"),
            outside.path().display().to_string()
        ),
    );
    let found = fixture.graph();
    assert!(
        found.iter().any(|f| f.message.contains("`outside`")),
        "{}",
        text(&found)
    );

    // A package outside spikes/ that the workspace does not build.
    let fixture = Fixture::new();
    fixture.write("tools/loose/Cargo.toml", &package("loose"));
    fixture.write("tools/loose/src/lib.rs", UNSAFE_FN);
    let found = fixture.graph();
    assert!(
        found
            .iter()
            .any(|f| f.file.as_deref() == Some("tools/loose/Cargo.toml")),
        "{}",
        text(&found)
    );

    // A manifest Cargo cannot resolve fails closed.
    let fixture = Fixture::new();
    fixture.write("crates/engine/Cargo.toml", "[package\n");
    fixture.repo.commit("broken");
    let found = rust_policy::check_sources(&fixture.ctx()).expect("check");
    assert!(
        found
            .iter()
            .any(|f| f.message.contains("cargo metadata failed")),
        "{}",
        text(&found)
    );
}
