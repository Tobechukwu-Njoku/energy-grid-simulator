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

const FORBID: &str = "#![forbid(unsafe_code)]\n#![doc(test(attr(forbid(unsafe_code))))]\n";

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
        .write(
            "crates/solver-highs/Cargo.toml",
            &package("egs-solver-highs"),
        )
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
        .write(
            "crates/solver-highs/Cargo.toml",
            &package("egs-solver-highs"),
        )
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
            .write(
                "crates/solver-highs/Cargo.toml",
                &package("egs-solver-highs"),
            )
            .write(
                "crates/solver-highs/src/lib.rs",
                &format!("{ADAPTER}{source}"),
            );
        assert_runtime_rejected(&fixture.build());
    }
    // Control: an ordinary system library links.
    let fixture = Fixture::new();
    fixture
        .write("crates/solver-highs/Cargo.toml", &package("egs-solver-highs"))
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
    fixture.write(
        "crates/solver-highs/Cargo.toml",
        &package("egs-solver-highs"),
    );
    fixture.write("crates/solver-highs/src/lib.rs", "//! Adapter.\n");
    let found = fixture.graph();
    let missing: Vec<&str> = found
        .iter()
        .filter(|f| {
            f.message
                .contains("must start with #![forbid(unsafe_code)]")
        })
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

/// A build script that writes an (empty, valid) archive named like the
/// Python runtime into OUT_DIR and returns its path; `emit` decides how it
/// reaches the linker.
fn archive_build_script(emit: &str) -> String {
    format!(
        "#![forbid(unsafe_code)]\nfn main() {{\n    let out = std::path::PathBuf::from(std::env::var_os(\"OUT_DIR\").unwrap());\n    let library = out.join(\"libpython3.12.a\");\n    std::fs::write(&library, \"!<arch>\\n\").unwrap();\n    {emit}\n}}\n"
    )
}

const ADAPTER_PACKAGE: &str = "egs-solver-highs";

// R6-01: a real library target named `___` was mistaken for Cargo's probe
// and got neither the forbid nor the linker guard. Compilations are now
// attributed through `cargo metadata` (manifest directory, package name,
// source file), never by crate name.
#[test]
fn r6_01_crate_named_like_the_probe_is_still_guarded() {
    let lib = "[lib]\nname = \"___\"\n";
    let link_arg = "println!(\"cargo:rustc-link-arg={}\", library.display());";
    let fixture = Fixture::new();
    fixture
        .write(
            "crates/solver-highs/Cargo.toml",
            &format!("{}{lib}", package(ADAPTER_PACKAGE)),
        )
        .write("crates/solver-highs/src/lib.rs", ADAPTER)
        .write(
            "crates/solver-highs/build.rs",
            &archive_build_script(link_arg),
        );
    assert_runtime_rejected(&fixture.build());
    // The same name on an ordinary package: unsafe code is still forbidden.
    let fixture = Fixture::new();
    fixture.write(
        "crates/engine/Cargo.toml",
        &format!("{}{lib}", package("engine")),
    );
    fixture.write("crates/engine/src/lib.rs", MACRO_OPT_OUT);
    assert_unsafe_rejected(&fixture.build());
    // Control: the same build script without the link argument.
    let fixture = Fixture::new();
    fixture
        .write(
            "crates/solver-highs/Cargo.toml",
            &format!("{}{lib}", package(ADAPTER_PACKAGE)),
        )
        .write("crates/solver-highs/src/lib.rs", ADAPTER)
        .write("crates/solver-highs/build.rs", &archive_build_script(""));
    assert_clean(&fixture.build());
}

// R6-02: a response file whose embedded quotes spell `libpython3.12.a` in
// pieces. Response files are refused whatever their content; any linker
// input that is not Rust's own artifact is refused too.
#[test]
fn r6_02_linker_response_files_and_foreign_inputs_are_refused() {
    let rsp = "let rsp = out.join(\"native.rsp\");\n    std::fs::write(&rsp, format!(\"\\\"{}\\\"/libpyth\\\"on\\\"3.12.a\\n\", out.display())).unwrap();\n    println!(\"cargo:rustc-link-arg=@{}\", rsp.display());";
    let fixture = Fixture::new();
    fixture
        .write(
            "crates/engine/src/main.rs",
            &format!("{FORBID}fn main() {{}}\n"),
        )
        .write("crates/engine/build.rs", &archive_build_script(rsp));
    let found = fixture.build();
    assert!(
        found
            .iter()
            .any(|f| f.message.contains("NFR-10") && f.message.contains("response files")),
        "{}",
        text(&found)
    );
    // A library a build script made, whatever its name, is not allow-listed.
    let fixture = Fixture::new();
    fixture
        .write("crates/engine/src/main.rs", &format!("{FORBID}fn main() {{}}\n"))
        .write(
            "crates/engine/build.rs",
            &archive_build_script(
                "let other = out.join(\"libnative.a\");\n    std::fs::copy(&library, &other).unwrap();\n    println!(\"cargo:rustc-link-arg={}\", other.display());",
            ),
        );
    assert_runtime_rejected(&fixture.build());
    // Control: no extra linker input.
    let fixture = Fixture::new();
    fixture
        .write(
            "crates/engine/src/main.rs",
            &format!("{FORBID}fn main() {{}}\n"),
        )
        .write("crates/engine/build.rs", &archive_build_script(""));
    assert_clean(&fixture.build());
}

// ADV23R6-02: a framework link request on the command line and from a
// build script reaches the allowlist (end to end through the CLI guard).
#[test]
fn adv23r6_02_framework_requests_are_refused_by_the_linker_guard() {
    for args in [
        &["-framework", "Python"][..],
        &["-Wl,-framework,Python"][..],
        &["-Xlinker", "-framework", "-Xlinker", "Julia"][..],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_xtask"))
            .args(args)
            .env(build_gate::LINK_ENV, "1")
            .output()
            .expect("run linker guard");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(1), "{args:?}: {stderr}");
        assert!(stderr.contains("NFR-10"), "{args:?}: {stderr}");
    }
    let fixture = Fixture::new();
    fixture
        .write("crates/engine/src/main.rs", &format!("{FORBID}fn main() {{}}\n"))
        .write(
            "crates/engine/build.rs",
            "#![forbid(unsafe_code)]\nfn main() { println!(\"cargo:rustc-link-arg=-Wl,-framework,Python\"); }\n",
        );
    assert_runtime_rejected(&fixture.build());
}

// R6-03: code compiled only without a feature, or with one feature alone, is
// built: default, all, no-default and each single feature are configurations.
#[test]
fn r6_03_feature_disabled_code_is_built_and_checked() {
    let features = "[features]\npure = []\n";
    let fixture = Fixture::new();
    fixture
        .write(
            "crates/solver-highs/Cargo.toml",
            &format!("{}{features}", package(ADAPTER_PACKAGE)),
        )
        .write(
            "crates/solver-highs/src/lib.rs",
            &format!(
                "{ADAPTER}#[cfg(not(feature = \"pure\"))]\n#[link(name = \"python3.12\")]\nunsafe extern \"C\" {{}}\n"
            ),
        );
    assert_runtime_rejected(&fixture.build());
    // Ordinary package: unsafe behind `not(feature)` and behind one feature
    // that another feature switches off.
    for (manifest, guard) in [
        ("[features]\nsafe = []\n", "not(feature = \"safe\")"),
        (
            "[features]\na = []\nb = []\n",
            "all(feature = \"a\", not(feature = \"b\"))",
        ),
    ] {
        let fixture = Fixture::new();
        fixture
            .write(
                "crates/engine/Cargo.toml",
                &format!("{}{manifest}", package("engine")),
            )
            .write(
                "crates/engine/src/lib.rs",
                &format!("{FORBID}#[cfg({guard})]\nmod hidden {{\n#![allow(unsafe_code)]\n}}\n"),
            );
        assert_unsafe_rejected(&fixture.build());
    }
    // Control: no feature-dependent link request.
    let fixture = Fixture::new();
    fixture
        .write(
            "crates/solver-highs/Cargo.toml",
            &format!("{}{features}", package(ADAPTER_PACKAGE)),
        )
        .write(
            "crates/solver-highs/src/lib.rs",
            &format!("{ADAPTER}#[cfg(not(feature = \"pure\"))]\npub fn f() {{}}\n"),
        );
    assert_clean(&fixture.build());
}

const UNSAFE_DOCTEST: &str = "/// ```\n/// let n = 1;\n/// let p = &n as *const i32;\n/// assert_eq!(unsafe { *p }, 1);\n/// ```\npub fn f() {}\n";

// R6-04: doctests are separate crates. The policy requires the doctest
// forbid on every non-adapter library root, and the gate runs the doctests
// so the compiler applies it; doctest links go through the linker guard.
#[test]
fn r6_04_doctests_are_checked() {
    let fixture = Fixture::new();
    fixture.write(
        "crates/engine/src/lib.rs",
        &format!("#![forbid(unsafe_code)]\n{UNSAFE_DOCTEST}"),
    );
    let graph = fixture.graph();
    assert!(
        graph
            .iter()
            .any(|f| f.message.contains("doc(test(attr(forbid(unsafe_code))))")),
        "{}",
        text(&graph)
    );
    fixture.write(
        "crates/engine/src/lib.rs",
        &format!("{FORBID}{UNSAFE_DOCTEST}"),
    );
    assert_clean(&fixture.graph());
    assert_unsafe_rejected(&fixture.build());
    // An adapter's doctest that links a runtime reaches the linker guard.
    let fixture = Fixture::new();
    fixture
        .write("crates/solver-highs/Cargo.toml", &package(ADAPTER_PACKAGE))
        .write(
            "crates/solver-highs/src/lib.rs",
            &format!(
                "{ADAPTER}/// ```\n/// #[link(name = \"python3.12\")]\n/// unsafe extern \"C\" {{}}\n/// fn main() {{}}\n/// ```\npub fn f() {{}}\n"
            ),
        );
    assert_runtime_rejected(&fixture.build());
    // Control: a safe doctest passes.
    let fixture = Fixture::new();
    fixture.write(
        "crates/engine/src/lib.rs",
        &format!("{FORBID}/// ```\n/// assert_eq!(1 + 1, 2);\n/// ```\npub fn f() {{}}\n"),
    );
    assert_clean(&fixture.build());
}

// ADV23R6-01: adapters are an explicit list of package names and
// directories in the trusted policy; a `crates/solver-*` directory or a
// borrowed adapter name elsewhere is ordinary code.
#[test]
fn adv23r6_01_adapters_come_from_the_trusted_list() {
    for (dir, name) in [
        ("crates/solver-evil", "solver-evil"),
        ("crates/solver-evil", ADAPTER_PACKAGE),
        ("crates/solver-highs", "solver-highs"),
    ] {
        let fixture = Fixture::new();
        fixture
            .write(&format!("{dir}/Cargo.toml"), &package(name))
            .write(
                &format!("{dir}/src/lib.rs"),
                &format!("{ADAPTER}{UNSAFE_FN}"),
            );
        assert_unsafe_rejected(&fixture.build());
        let graph = fixture.graph();
        assert!(
            graph.iter().any(
                |f| f.file.as_deref() == Some(format!("{dir}/src/lib.rs").as_str())
                    && f.message.contains("must start with")
            ),
            "{dir} {name}: {}",
            text(&graph)
        );
    }
    // Control: the listed package at its listed directory.
    let fixture = Fixture::new();
    fixture
        .write("crates/solver-highs/Cargo.toml", &package(ADAPTER_PACKAGE))
        .write(
            "crates/solver-highs/src/lib.rs",
            &format!("{ADAPTER}{UNSAFE_FN}"),
        );
    assert_clean(&fixture.build());
    assert_clean(&fixture.graph());
    // The list lives in a protected policy file (owner approval, D-74).
    let ctx = Ctx::local(&root());
    let adapters = rust_policy::adapters(&ctx).expect("adapters");
    assert_eq!(adapters.len(), 4);
    let policy = ctx
        .policy_toml(rust_policy::REPOSITORY_POLICY)
        .expect("policy");
    let protected = xtask::util::string_array(&policy["protected_paths"]);
    assert!(
        protected.iter().any(|pattern| xtask::util::glob_match(
            pattern.trim_start_matches('/'),
            rust_policy::REPOSITORY_POLICY
        )),
        "{protected:?}"
    );
    // Malformed entries fail closed.
    for bad in [
        "[[adapter]]\npackage = \"x\"\npath = \"crates/solver-*\"\n",
        "[[adapter]]\npackage = \"x\"\n",
        "[[adapter]]\npackage = \"x\"\npath = \"../x\"\n",
        "adapter = \"crates/solver-x\"\n",
    ] {
        assert!(
            rust_policy::parse_adapters(&bad.parse().expect("toml")).is_err(),
            "{bad}"
        );
    }
}

// Build scripts may not hand the compiler variables that redirect the
// toolchain or the gate (`cargo:rustc-env` reaches the wrapper).
#[test]
fn build_script_rustc_env_cannot_redirect_the_gate() {
    let fixture = Fixture::new();
    fixture.write(
        "crates/engine/build.rs",
        "#![forbid(unsafe_code)]\nfn main() { println!(\"cargo:rustc-env=XTASK_BUILD_GATE_CONFIG=/nonexistent\"); }\n",
    );
    let found = fixture.build();
    assert!(
        found
            .iter()
            .any(|f| f.message.contains("XTASK_BUILD_GATE_CONFIG")),
        "{}",
        text(&found)
    );
    fixture.write(
        "crates/engine/build.rs",
        "#![forbid(unsafe_code)]\nfn main() { println!(\"cargo:rustc-env=GIT_HASH=abc\"); }\n",
    );
    assert_clean(&fixture.build());
}

/// R7-01: a build script that hides an archive named like the Python
/// runtime behind an OUT_DIR `lib<name>.so` linker script and adds OUT_DIR as
/// a link search directory.
const SHADOW_BUILD_SCRIPT: &str = "#![forbid(unsafe_code)]
fn main() {
    let out = std::path::PathBuf::from(std::env::var_os(\"OUT_DIR\").unwrap());
    let hidden = out.join(\"hidden\");
    std::fs::create_dir_all(&hidden).unwrap();
    let library = hidden.join(\"libpython3.12.a\");
    std::fs::write(&library, \"!<arch>\\n\").unwrap();
    std::fs::write(out.join(\"libiconv.so\"), format!(\"INPUT ( {} )\\n\", library.display())).unwrap();
    println!(\"cargo:rustc-link-search=native={}\", out.display());
}
";

fn shadow_adapter(link: &str) -> String {
    format!(
        "{ADAPTER}#[link(name = \"{link}\")]\nunsafe extern \"C\" {{}}\n/// ```\n/// assert_eq!(egs_solver_highs::value(), 23);\n/// ```\npub fn value() -> i32 {{\n    23\n}}\n"
    )
}

// R7-01: an allow-listed library name (`iconv`) resolves, through a build
// script's OUT_DIR search directory, to a linker script whose INPUT is an
// archive named like the Python runtime. The guard resolves the name
// through the search directories and refuses the shadowing file.
#[test]
fn r7_01_allow_listed_names_cannot_resolve_to_build_script_files() {
    let fixture = Fixture::new();
    fixture
        .write("crates/solver-highs/Cargo.toml", &package(ADAPTER_PACKAGE))
        .write("crates/solver-highs/src/lib.rs", &shadow_adapter("iconv"))
        .write("crates/solver-highs/build.rs", SHADOW_BUILD_SCRIPT);
    let found = fixture.build();
    assert_runtime_rejected(&found);
    assert!(
        found.iter().any(|f| f.message.contains("R7-01")),
        "{}",
        text(&found)
    );
    // Control: the same OUT_DIR, but a system library it does not shadow.
    let fixture = Fixture::new();
    fixture
        .write("crates/solver-highs/Cargo.toml", &package(ADAPTER_PACKAGE))
        .write("crates/solver-highs/src/lib.rs", &shadow_adapter("m"))
        .write("crates/solver-highs/build.rs", SHADOW_BUILD_SCRIPT);
    assert_clean(&fixture.build());
}

const HIDDEN_LINK: &str = "#[link(name = \"python3.12\")]\nunsafe extern \"C\" {}\n";

// R7-02: a self dev-dependency enables `b`, so every all-targets and doctest
// build hides code compiled only with `a` and without `b`. The production
// build (library and binaries, no dev-dependencies) is built on its own.
#[test]
fn r7_02_dev_dependency_features_cannot_hide_production_code() {
    let manifest = |dev: bool| {
        format!(
            "{}[features]\na = []\nb = []\n[lib]\ncrate-type = [\"rlib\", \"cdylib\"]\n{}",
            package(ADAPTER_PACKAGE),
            if dev {
                "[dev-dependencies]\negs-solver-highs = { path = \".\", features = [\"b\"] }\n"
            } else {
                ""
            }
        )
    };
    let build_script = "#![forbid(unsafe_code)]\nfn main() {}\n";
    let fixture = Fixture::new();
    fixture
        .write("crates/solver-highs/Cargo.toml", &manifest(true))
        .write("crates/solver-highs/build.rs", build_script)
        .write(
            "crates/solver-highs/src/lib.rs",
            &format!("{ADAPTER}#[cfg(all(feature = \"a\", not(feature = \"b\")))]\n{HIDDEN_LINK}"),
        );
    assert_runtime_rejected(&fixture.build());
    // Control: the same manifest without the hidden link request.
    let fixture = Fixture::new();
    fixture
        .write("crates/solver-highs/Cargo.toml", &manifest(true))
        .write("crates/solver-highs/build.rs", build_script)
        .write(
            "crates/solver-highs/src/lib.rs",
            &format!(
                "{ADAPTER}#[cfg(all(feature = \"a\", not(feature = \"b\")))]\npub fn f() {{}}\n"
            ),
        );
    assert_clean(&fixture.build());
}

// R7-02: another workspace member enables `b`, so every `--workspace` build
// hides code compiled with the member's default features alone; a single
// feature never enables both defaults. The member is built alone.
#[test]
fn r7_02_workspace_feature_unification_cannot_hide_default_code() {
    let fixture = Fixture::new();
    fixture
        .write(
            "crates/solver-highs/Cargo.toml",
            &format!(
                "{}[features]\ndefault = [\"a\", \"c\"]\na = []\nb = []\nc = []\n[lib]\ncrate-type = [\"rlib\", \"cdylib\"]\n",
                package(ADAPTER_PACKAGE)
            ),
        )
        .write(
            "crates/solver-highs/src/lib.rs",
            &format!(
                "{ADAPTER}#[cfg(all(feature = \"a\", feature = \"c\", not(feature = \"b\")))]\n{HIDDEN_LINK}"
            ),
        )
        .write(
            "crates/engine/Cargo.toml",
            &format!(
                "{}[dependencies]\negs-solver-highs = {{ path = \"../solver-highs\", features = [\"b\"] }}\n",
                package("engine")
            ),
        );
    assert_runtime_rejected(&fixture.build());
}
