//! Compiler-enforced unsafe boundary and native-runtime ban (ADR 0001, NFR-10).
//!
//! `cargo xtask build-gate` builds the whole workspace under test (every
//! target, build scripts and proc macros included) in several feature
//! configurations, then runs its doctests, with the trusted xtask binary as
//! `RUSTC_WRAPPER` and as the linker. The wrapper, the linker guard and the
//! flags they add come from the trusted tree; the tree under test cannot
//! weaken them:
//!
//! - **Attribution by Cargo's package graph (R6-01).** Before building, the
//!   gate writes the resolved packages from `cargo metadata` (package id,
//!   canonical manifest directory, name, local or registry, reviewed adapter
//!   or not, and every target's canonical source file and kind) to a
//!   configuration file. The wrapper attributes each rustc invocation to a
//!   package by the manifest directory and package name Cargo sets, and to a
//!   target by the source file on the command line. A crate name never
//!   decides anything: a compilation the wrapper cannot attribute is refused,
//!   and an invocation without a source file (Cargo's target-information
//!   probe) gets the strictest flags. Only a `rustc -vV`-style version query
//!   passes through unchanged.
//! - **Unsafe boundary.** Every compilation of a local package gets
//!   `-F unsafe_code` appended and its lint caps removed, except the library,
//!   binary and test targets of a reviewed adapter: an exact package name at
//!   an exact directory listed in the trusted `repository.toml` (ADV23R6-01).
//!   Adapter build scripts are never exempt. A command-line `forbid` cannot be
//!   lowered by any attribute, `[lints]` entry or macro expansion.
//! - **Doctests (R6-04).** rustdoc builds every doctest as its own crate,
//!   which neither the wrapper nor the crate's root lint reaches. The static
//!   policy therefore requires `#![doc(test(attr(forbid(unsafe_code))))]` on
//!   every non-adapter library root, and the gate runs `cargo test --doc` in
//!   its sanitised environment (no `RUSTDOCFLAGS` from the caller, no
//!   repository Cargo config), so that attribute is what the compiler applies;
//!   doctest executables link through the same linker guard.
//! - **Feature configurations (R6-03, R7-02).** `--all-features` is not the
//!   union of all compiled code: `cfg(not(feature = ..))` code is only
//!   compiled without the feature, and Cargo unifies features across the
//!   packages of one command and from dev-dependencies. The gate builds the
//!   workspace with default, all and no default features, and every member
//!   alone with its default features, with none, and with each of its features
//!   alone. Each configuration is built three times: production targets
//!   (library and binaries, so no dev-dependency features), all targets, and
//!   doctests. Code that is compiled only under a combination of two or more
//!   non-default features of one member and not under all features is not
//!   built; ADR 0001 records that bound.
//! - **Linker guard (R6-02, ADV23R6-02, R7-01).** Every invocation gets
//!   `-C linker=<copy of xtask>`. The guard parses the C-driver command line,
//!   `-Wl,` lists and `-Xlinker` values, refuses every response file
//!   (`@file`, also inside `-Wl,`), linker script option, plugin, sysroot
//!   override and driver redirection, and accepts only allow-listed library
//!   names (`-l`, `-framework` and their weak/needed/re-export forms, from the
//!   trusted `gates.toml`) and input files that Rust itself produced: rlibs
//!   and codegen objects in the build's `deps`/`build` directories, rustc and
//!   rustdoc temporary objects, and the toolchain sysroot. An allow-listed name
//!   is then resolved through the line's search directories: a matching file
//!   in a directory that is not the sysroot, the C driver's own library
//!   directories or a trusted `link_search_roots` entry is refused whatever it
//!   contains (an OUT_DIR `libiconv.so` linker script, R7-01), and a system
//!   linker script's inputs are resolved and checked the same way. A direct
//!   input that is a script or thin archive is refused. Any name mentioning
//!   Python or Julia is reported as such. Only then is `cc` run.
//! - **Build-script output** is read from Cargo's JSON messages
//!   (`build-script-executed`): a `rustc-link-lib` naming a runtime, a
//!   `rustc-link-search` directory holding one, and any `rustc-env` that
//!   could redirect the toolchain or this gate are findings.
//!
//! Cargo runs from the trusted tree (no repository `.cargo/config` is
//! loaded) with every Cargo/rustc flag, wrapper and lint-cap variable and the
//! GitHub Actions file-command variables removed from its environment. This
//! runs the pull request's build scripts, proc macros and doctests, so it
//! belongs in an unprivileged CI job, as that job's last step, never in the
//! trusted gates. Code that runs at build time shares the job's user and could
//! tamper with the job itself; that residual risk is recorded in ADR 0001.

use std::collections::BTreeSet;
use std::ffi::OsString;
use std::io::{BufRead as _, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

use crate::rust_policy::{Metadata, cargo, metadata};
use crate::util::{Ctx, Finding, string_array};

/// Set for Cargo and inherited by the wrapper: the gate configuration file.
pub const WRAP_ENV: &str = "XTASK_BUILD_GATE_CONFIG";
/// Set by the wrapper for rustc (the configuration file path); its presence
/// also selects the linker guard.
pub const LINK_ENV: &str = "XTASK_BUILD_GATE_LINKER";
/// File name of the linker-guard copy of the xtask binary. Invoked under
/// this name, the binary is the linker guard.
pub const LINKER_NAME: &str = "egs-build-gate-linker";
/// Most feature configurations the gate builds before failing closed.
pub const MAX_FEATURE_SETS: usize = 256;

/// GitHub Actions file-command variables: removed from the gate's Cargo
/// environment so build-time code is not handed them (ADV23R6-03).
pub const FILE_COMMAND_ENV: &[&str] = &[
    "GITHUB_ENV",
    "GITHUB_PATH",
    "GITHUB_OUTPUT",
    "GITHUB_STATE",
    "GITHUB_STEP_SUMMARY",
];

/// Whether a library name or file names a Python or Julia runtime.
#[must_use]
pub fn names_runtime(text: &str) -> bool {
    let lower = text.to_lowercase();
    lower.contains("python") || lower.contains("julia")
}

/// Entry point used by `main` before argument parsing: runs the rustc
/// wrapper or the linker guard when the build gate invoked this binary.
#[must_use]
pub fn dispatch() -> Option<ExitCode> {
    let mut argv = std::env::args_os();
    let program = argv.next().unwrap_or_default();
    let args: Vec<OsString> = argv.collect();
    let invoked_as_linker = Path::new(&program)
        .file_stem()
        .and_then(|stem| stem.to_str())
        == Some(LINKER_NAME);
    if invoked_as_linker || std::env::var_os(LINK_ENV).is_some() {
        let config = std::env::var_os(LINK_ENV)
            .or_else(|| std::env::var_os(WRAP_ENV))
            .map(PathBuf::from);
        return Some(link(&args, config.as_deref()));
    }
    let config = std::env::var_os(WRAP_ENV)?;
    Some(wrap(Path::new(&config), &args))
}

fn exit(status: std::io::Result<std::process::ExitStatus>, what: &str) -> ExitCode {
    match status {
        Ok(status) => ExitCode::from(u8::try_from(status.code().unwrap_or(1)).unwrap_or(1)),
        Err(error) => {
            eprintln!("error: build gate could not run {what}: {error}");
            ExitCode::from(1)
        }
    }
}

fn canonical(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

// ---------------------------------------------------------------------------
// Configuration shared by the gate, the wrapper and the linker guard.

/// One target of a package, as the wrapper sees it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitTarget {
    /// Canonical source file.
    pub src: PathBuf,
    /// Whether the target is the package's build script.
    pub build_script: bool,
}

/// One package of the resolved graph, as the wrapper sees it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    /// Cargo package id.
    pub id: String,
    /// Package name.
    pub name: String,
    /// Canonical manifest directory.
    pub dir: PathBuf,
    /// A local (path) package: repository code under the unsafe boundary.
    pub local: bool,
    /// A reviewed adapter of the trusted policy.
    pub adapter: bool,
    /// Targets.
    pub targets: Vec<UnitTarget>,
}

/// The gate configuration: written by the gate, read by every wrapper and
/// linker-guard process.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Config {
    /// Canonical build target directory.
    pub target_dir: PathBuf,
    /// Canonical toolchain sysroot.
    pub sysroot: PathBuf,
    /// The linker-guard binary.
    pub linker: PathBuf,
    /// Library names the linker may be asked for.
    pub libraries: Vec<String>,
    /// Framework names the linker may be asked for.
    pub frameworks: Vec<String>,
    /// Trusted library directories besides the toolchain's own.
    pub search_roots: Vec<PathBuf>,
    /// Resolved packages.
    pub units: Vec<Unit>,
}

fn path_value(path: &Path) -> serde_json::Value {
    serde_json::Value::String(path.to_string_lossy().into_owned())
}

impl Config {
    /// Builds the unit list from `cargo metadata`.
    #[must_use]
    pub fn units(metadata: &Metadata) -> Vec<Unit> {
        metadata
            .packages
            .iter()
            .zip(&metadata.ids)
            .map(|(package, id)| Unit {
                id: id.clone(),
                name: package.name.clone(),
                dir: Path::new(&package.manifest_abs)
                    .parent()
                    .map(Path::to_path_buf)
                    .unwrap_or_default(),
                local: package.source.is_none(),
                adapter: package.adapter,
                targets: package
                    .targets
                    .iter()
                    .map(|target| UnitTarget {
                        src: PathBuf::from(&target.src_abs),
                        build_script: target.kinds.iter().any(|kind| kind == "custom-build"),
                    })
                    .collect(),
            })
            .collect()
    }

    /// Serialises the configuration.
    #[must_use]
    pub fn to_json(&self) -> String {
        let units: Vec<serde_json::Value> = self
            .units
            .iter()
            .map(|unit| {
                serde_json::json!({
                    "id": unit.id,
                    "name": unit.name,
                    "dir": path_value(&unit.dir),
                    "local": unit.local,
                    "adapter": unit.adapter,
                    "targets": unit.targets.iter().map(|target| serde_json::json!({
                        "src": path_value(&target.src),
                        "build_script": target.build_script,
                    })).collect::<Vec<_>>(),
                })
            })
            .collect();
        serde_json::json!({
            "target_dir": path_value(&self.target_dir),
            "sysroot": path_value(&self.sysroot),
            "linker": path_value(&self.linker),
            "libraries": self.libraries,
            "frameworks": self.frameworks,
            "search_roots": self.search_roots.iter().map(|root| path_value(root)).collect::<Vec<_>>(),
            "units": units,
        })
        .to_string()
    }

    /// Parses a configuration.
    ///
    /// # Errors
    ///
    /// Fails on malformed JSON.
    pub fn from_json(text: &str) -> Result<Self, String> {
        let value: serde_json::Value =
            serde_json::from_str(text).map_err(|error| format!("build gate config: {error}"))?;
        let path = |value: &serde_json::Value| PathBuf::from(value.as_str().unwrap_or_default());
        let strings = |value: &serde_json::Value| -> Vec<String> {
            value
                .as_array()
                .map(Vec::as_slice)
                .unwrap_or_default()
                .iter()
                .filter_map(|item| item.as_str().map(str::to_owned))
                .collect()
        };
        let units = value["units"]
            .as_array()
            .ok_or("build gate config: no units")?
            .iter()
            .map(|unit| Unit {
                id: unit["id"].as_str().unwrap_or_default().to_owned(),
                name: unit["name"].as_str().unwrap_or_default().to_owned(),
                dir: path(&unit["dir"]),
                local: unit["local"].as_bool().unwrap_or(true),
                adapter: unit["adapter"].as_bool().unwrap_or(false),
                targets: unit["targets"]
                    .as_array()
                    .map(Vec::as_slice)
                    .unwrap_or_default()
                    .iter()
                    .map(|target| UnitTarget {
                        src: path(&target["src"]),
                        build_script: target["build_script"].as_bool().unwrap_or(true),
                    })
                    .collect(),
            })
            .collect();
        Ok(Self {
            target_dir: path(&value["target_dir"]),
            sysroot: path(&value["sysroot"]),
            linker: path(&value["linker"]),
            libraries: strings(&value["libraries"]),
            frameworks: strings(&value["frameworks"]),
            search_roots: strings(&value["search_roots"])
                .into_iter()
                .map(PathBuf::from)
                .collect(),
            units,
        })
    }

    fn read(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path)
            .map_err(|error| format!("build gate config {}: {error}", path.display()))?;
        Self::from_json(&text)
    }
}

// ---------------------------------------------------------------------------
// The rustc wrapper.

/// How the wrapper changes one rustc command line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Wrapped {
    /// Run with these arguments.
    Run(Vec<String>),
    /// Refuse, with this reason.
    Refuse(String),
}

/// Who a compilation belongs to, from the package graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Owner {
    /// A local package. `exempt` is true only for a non-build-script target of
    /// a reviewed adapter.
    Repository {
        /// Whether the target may use unsafe code.
        exempt: bool,
    },
    /// A registry package: its own lint levels, only the linker guard.
    External,
}

/// What kind of rustc invocation a command line is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Invocation {
    /// `rustc -vV` and similar: no input, no output.
    Version,
    /// No source file (Cargo's target-information probe reads stdin).
    Probe,
    /// A compilation of this source file.
    Compile(String),
    /// A command line the wrapper does not understand (refused).
    Unclear(String),
}

/// rustc options that take their value as the next argument.
const RUSTC_VALUE_FLAGS: &[&str] = &[
    "--crate-name",
    "--edition",
    "--error-format",
    "--json",
    "--crate-type",
    "--emit",
    "--out-dir",
    "-o",
    "-C",
    "--codegen",
    "-L",
    "-l",
    "--extern",
    "--cfg",
    "--check-cfg",
    "--cap-lints",
    "-A",
    "-W",
    "-D",
    "-F",
    "--allow",
    "--warn",
    "--deny",
    "--forbid",
    "--force-warn",
    "--target",
    "--print",
    "--explain",
    "--sysroot",
    "-Z",
    "--diagnostic-width",
    "--remap-path-prefix",
    "--env-set",
    "--color",
    "--crate-attr",
];

const VERSION_FLAGS: &[&str] = &["-vV", "-Vv", "-V", "--version", "-v", "--verbose"];

/// Classifies a rustc command line (without the rustc path).
#[must_use]
pub fn classify(args: &[String]) -> Invocation {
    let mut inputs = Vec::new();
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg.starts_with('@') {
            return Invocation::Unclear(format!("argument file `{arg}`"));
        }
        if RUSTC_VALUE_FLAGS.contains(&arg.as_str()) {
            iter.next();
        } else if arg == "-" || !arg.starts_with('-') {
            inputs.push(arg.clone());
        }
    }
    match inputs.as_slice() {
        [] if !args.is_empty() && args.iter().all(|arg| VERSION_FLAGS.contains(&arg.as_str())) => {
            Invocation::Version
        }
        [] => Invocation::Probe,
        [input] if input == "-" => Invocation::Probe,
        [input] => Invocation::Compile(input.clone()),
        _ => Invocation::Unclear(format!("several inputs {inputs:?}")),
    }
}

/// Attributes a compilation to a package and target of the configuration:
/// the manifest directory and package name Cargo set, and the source file
/// (relative to `cwd`). `None` if anything does not match.
#[must_use]
pub fn attribute(
    config: &Config,
    manifest_dir: Option<&Path>,
    package_name: Option<&str>,
    cwd: &Path,
    source: &str,
) -> Option<Owner> {
    let dir = canonical(manifest_dir?);
    let name = package_name?;
    let src = canonical(&cwd.join(source));
    let unit = config
        .units
        .iter()
        .find(|unit| unit.dir == dir && unit.name == name)?;
    let target = unit.targets.iter().find(|target| target.src == src)?;
    Some(if unit.local {
        Owner::Repository {
            exempt: unit.adapter && !target.build_script,
        }
    } else {
        Owner::External
    })
}

/// Rewrites one rustc argument list (without the rustc path). `owner` is the
/// attribution of a compilation (`None` if it could not be attributed).
#[must_use]
pub fn rewrite(args: &[String], owner: Option<Owner>, linker: &str) -> Wrapped {
    let owner = match classify(args) {
        Invocation::Version => return Wrapped::Run(args.to_vec()),
        Invocation::Unclear(reason) => {
            return Wrapped::Refuse(format!("rustc command line it cannot read ({reason})"));
        }
        // Nothing to attribute: the strictest treatment (fail closed).
        Invocation::Probe => Owner::Repository { exempt: false },
        Invocation::Compile(source) => match owner {
            Some(owner) => owner,
            None => {
                return Wrapped::Refuse(format!(
                    "compilation of `{source}` that it cannot attribute to a package and target of `cargo metadata` (R6-01)"
                ));
            }
        },
    };
    let mut out = Vec::with_capacity(args.len() + 4);
    match owner {
        Owner::Repository { exempt } => {
            let mut iter = args.iter();
            while let Some(arg) = iter.next() {
                if arg == "--cap-lints" {
                    iter.next();
                    continue;
                }
                if arg.starts_with("--cap-lints=") {
                    continue;
                }
                out.push(arg.clone());
            }
            if !exempt {
                out.extend(["-F".to_owned(), "unsafe_code".to_owned()]);
            }
        }
        Owner::External => out.extend(args.iter().cloned()),
    }
    out.extend(["-C".to_owned(), format!("linker={linker}")]);
    Wrapped::Run(out)
}

fn wrap(config_path: &Path, args: &[OsString]) -> ExitCode {
    let Some((rustc, rest)) = args.split_first() else {
        eprintln!("error: build gate wrapper called without rustc");
        return ExitCode::from(1);
    };
    let Some(rest) = rest
        .iter()
        .map(|arg| arg.to_str().map(str::to_owned))
        .collect::<Option<Vec<_>>>()
    else {
        eprintln!("error: build gate wrapper: non-UTF-8 rustc argument");
        return ExitCode::from(1);
    };
    let config = match Config::read(config_path) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("error: build gate wrapper: {error}");
            return ExitCode::from(1);
        }
    };
    let owner = match classify(&rest) {
        Invocation::Compile(source) => {
            let cwd = std::env::current_dir().unwrap_or_default();
            let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from);
            let name = std::env::var("CARGO_PKG_NAME").ok();
            attribute(
                &config,
                manifest_dir.as_deref(),
                name.as_deref(),
                &cwd,
                &source,
            )
        }
        _ => None,
    };
    match rewrite(&rest, owner, &config.linker.to_string_lossy()) {
        Wrapped::Run(args) => exit(
            Command::new(rustc)
                .args(&args)
                .env(LINK_ENV, config_path)
                .env_remove(WRAP_ENV)
                .status(),
            "rustc",
        ),
        Wrapped::Refuse(reason) => {
            eprintln!("error: build gate refuses {reason}");
            ExitCode::from(1)
        }
    }
}

// ---------------------------------------------------------------------------
// The linker guard.

/// What the linker guard accepts.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LinkPolicy {
    /// Allowed library names (`-lNAME`).
    pub libraries: Vec<String>,
    /// Allowed framework names (`-framework NAME`).
    pub frameworks: Vec<String>,
    /// Build target directory: Rust's own rlibs and codegen objects.
    pub target_dir: PathBuf,
    /// Toolchain sysroot.
    pub sysroot: PathBuf,
    /// System temporary directory (rustc and rustdoc work directories).
    pub temp: PathBuf,
    /// The C driver's own library search directories (`cc
    /// -print-search-dirs`): the toolchain's system directories.
    pub system_dirs: Vec<PathBuf>,
    /// Further trusted library directories from the trusted `gates.toml`
    /// (`link_search_roots`).
    pub search_roots: Vec<PathBuf>,
    /// The linker's working directory: it opens a linker script's relative
    /// inputs there first.
    pub cwd: PathBuf,
}

/// Driver (cc) options that take the next argument as a plain value.
const DRIVER_VALUES: &[&str] = &[
    "-o",
    "-arch",
    "-target",
    "--target",
    "-isysroot",
    "-L",
    "-F",
    "-e",
    "-u",
    "-z",
    "-install_name",
    "-current_version",
    "-compatibility_version",
    "-rpath",
    "-soname",
    "-syslibroot",
    "-undefined",
    "-macos_version_min",
    "-mmacosx-version-min",
];

/// Linker (ld) options that take the next argument as a plain value.
const LINKER_VALUES: &[&str] = &[
    "-o",
    "-L",
    "-F",
    "-z",
    "-m",
    "-e",
    "--entry",
    "-u",
    "--undefined",
    "-soname",
    "-h",
    "-rpath",
    "-rpath-link",
    "-install_name",
    "-current_version",
    "-compatibility_version",
    "-syslibroot",
    "-arch",
    "-undefined",
    "-dynamic-linker",
    "--hash-style",
    "--build-id",
    "-headerpad",
];

/// Options whose value is a file the linker reads; accepted only inside a
/// rustc temporary directory (rustc writes export lists there).
const LINKER_FILES: &[&str] = &[
    "--version-script",
    "--dynamic-list",
    "-exported_symbols_list",
    "-unexported_symbols_list",
    "--retain-symbols-file",
];

/// Options that name a library and take it as the next argument.
const NAME_FLAGS: &[&str] = &["-l", "--library"];

/// Options that name a framework and take it as the next argument.
const FRAMEWORK_FLAGS: &[&str] = &[
    "-framework",
    "-weak_framework",
    "-needed_framework",
    "-reexport_framework",
    "-lazy_framework",
    "-upward_framework",
];

/// Options that name a library file and take it as the next argument.
const LIBRARY_FILE_FLAGS: &[&str] = &[
    "-weak_library",
    "-needed_library",
    "-reexport_library",
    "-lazy_library",
    "-upward_library",
    "-load_hidden",
];

/// Attached library-name prefixes (`-weak-lfoo`, `-needed-lfoo`, ...).
const NAME_PREFIXES: &[&str] = &[
    "--library=",
    "-weak-l",
    "-needed-l",
    "-reexport-l",
    "-lazy-l",
    "-upward-l",
    "-hidden-l",
    "-l",
];

/// Options refused outright: they read scripts, plugins or argument files,
/// or change which programs the driver runs.
fn refused_option(piece: &str) -> bool {
    let lower = piece.to_ascii_lowercase();
    piece.starts_with('@')
        || piece == "-T"
        || piece.starts_with("-T")
            && !piece.starts_with("-Tbss")
            && !piece.starts_with("-Tdata")
            && !piece.starts_with("-Ttext")
        || lower.starts_with("--script")
        || lower.starts_with("-dt")
        || lower.starts_with("--default-script")
        || lower.starts_with("-plugin")
        || lower.starts_with("--plugin")
        || lower.starts_with("-fplugin")
        || lower.starts_with("-specs")
        || lower.starts_with("--specs")
        || piece == "-x"
        || lower.starts_with("-wrapper")
        || lower.starts_with("--ld-path")
        || lower.starts_with("-filelist")
        || lower == "-include"
        || lower.starts_with("--just-symbols")
        || lower.starts_with("--sysroot")
        || piece == "-Y"
        || piece.starts_with("-YP,")
}

#[derive(Default)]
struct LinkScan {
    problems: Vec<String>,
    /// Every library, framework or file the command line names.
    named: Vec<String>,
    /// Allow-listed library names requested (`-lNAME`, `-l:FILE`).
    libraries: Vec<String>,
    /// Allow-listed framework names requested.
    frameworks: Vec<String>,
    /// Library search directories (`-L`, `--library-path`), in order.
    lib_dirs: Vec<String>,
    /// Framework search directories (`-F`), in order.
    framework_dirs: Vec<String>,
}

/// What a file the linker opens contains.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileKind {
    /// An object file, archive, shared library or text stub.
    Binary,
    /// A thin archive: its members are other files, named by path.
    ThinArchive,
    /// Anything else: GNU ld reads it as a linker script.
    Script,
}

fn file_kind(path: &Path) -> Option<FileKind> {
    use std::io::Read as _;
    let mut head = [0_u8; 16];
    let mut file = std::fs::File::open(path).ok()?;
    let read = file.read(&mut head).ok()?;
    let head = &head[..read];
    let binary: &[&[u8]] = &[
        b"\x7fELF",
        b"!<arch>\n",
        b"\xfe\xed\xfa\xce",
        b"\xfe\xed\xfa\xcf",
        b"\xce\xfa\xed\xfe",
        b"\xcf\xfa\xed\xfe",
        b"\xca\xfe\xba\xbe",
        b"\xbe\xba\xfe\xca",
        b"--- !tapi-tbd",
        b"MZ",
        b"\x00\x00\xff\xff",
    ];
    Some(if head.starts_with(b"!<thin>\n") {
        FileKind::ThinArchive
    } else if binary.iter().any(|magic| head.starts_with(magic))
        // A COFF object starts with its machine type; rustc's own objects
        // on Linux and macOS are ELF/Mach-O, so only these two are listed.
        || head.starts_with(&[0x64, 0x86])
        || head.starts_with(&[0x64, 0xaa])
    {
        FileKind::Binary
    } else {
        FileKind::Script
    })
}

/// The file names `-lNAME` (or `-l:FILE`) may open in one directory.
fn library_candidates(name: &str) -> Vec<String> {
    if let Some(file) = name.strip_prefix(':') {
        return vec![file.to_owned()];
    }
    [
        "lib{}.so",
        "lib{}.a",
        "lib{}.dylib",
        "lib{}.tbd",
        "lib{}.dll.a",
        "{}.lib",
        "{}.dll",
    ]
    .iter()
    .map(|pattern| pattern.replace("{}", name))
    .collect()
}

/// The commands a trusted system linker script may use; anything else
/// (`SEARCH_DIR`, `INCLUDE`, `STARTUP`, sections) is refused (R7-01).
const SCRIPT_COMMANDS: &[&str] = &[
    "GROUP",
    "INPUT",
    "AS_NEEDED",
    "OUTPUT_FORMAT",
    "OUTPUT_ARCH",
    "TARGET",
];

/// The input names of a linker script, or the reason it is refused.
fn script_inputs(text: &str) -> Result<Vec<String>, String> {
    let mut clean = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find("/*") {
        clean.push_str(&rest[..start]);
        match rest[start + 2..].find("*/") {
            Some(end) => rest = &rest[start + 2 + end + 2..],
            None => return Err("unterminated comment".to_owned()),
        }
    }
    clean.push_str(rest);
    let mut inputs = Vec::new();
    let mut stack: Vec<String> = Vec::new();
    let mut pending: Option<String> = None;
    let spaced = clean
        .replace('(', " ( ")
        .replace(')', " ) ")
        .replace(',', " ");
    for token in spaced.split_whitespace() {
        match token {
            "(" => stack.push(pending.take().ok_or("unexpected `(`")?),
            ")" => {
                stack.pop().ok_or("unbalanced `)`")?;
            }
            word if pending.is_some() => return Err(format!("`{word}` after a command")),
            word if stack.is_empty() || word == "AS_NEEDED" => {
                if !SCRIPT_COMMANDS.contains(&word) {
                    return Err(format!("command `{word}`"));
                }
                pending = Some(word.to_owned());
            }
            word => {
                let inside = stack.last().map(String::as_str).unwrap_or_default();
                if matches!(inside, "GROUP" | "INPUT" | "AS_NEEDED") {
                    inputs.push(word.trim_matches('"').to_owned());
                }
            }
        }
    }
    if !stack.is_empty() || pending.is_some() {
        return Err("unbalanced `(`".to_owned());
    }
    Ok(inputs)
}

impl LinkPolicy {
    /// Whether a directory is a rustc or rustdoc work directory: named
    /// `rustc*` or `rustdoctest*`, created next to rustc's output (`deps` or
    /// `build/<unit>` of this build, never an OUT_DIR), in the system
    /// temporary directory, or inside a rustdoc doctest directory there.
    fn work_dir(&self, dir: &Path) -> bool {
        let named = |dir: &Path, prefixes: &[&str]| {
            dir.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| prefixes.iter().any(|prefix| name.starts_with(prefix)))
        };
        if !named(dir, &["rustc", "rustdoctest"]) {
            return false;
        }
        let Some(parent) = dir.parent() else {
            return false;
        };
        let parent = canonical(parent);
        let output = parent.file_name().is_some_and(|name| name == "deps")
            || parent
                .parent()
                .and_then(Path::file_name)
                .is_some_and(|name| name == "build");
        parent == self.temp
            || (output
                && !self.target_dir.as_os_str().is_empty()
                && parent.starts_with(&self.target_dir))
            || (named(&parent, &["rustdoctest"])
                && parent
                    .parent()
                    .is_some_and(|dir| canonical(dir) == self.temp))
    }

    fn rustc_temp(&self, path: &Path) -> bool {
        path.parent().is_some_and(|dir| self.work_dir(dir))
    }

    /// Whether an input file was produced by Rust itself.
    fn rust_artifact(&self, file: &str) -> bool {
        let path = canonical(Path::new(file));
        if !self.sysroot.as_os_str().is_empty() && path.starts_with(&self.sysroot) {
            return true;
        }
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();
        if self.rustc_temp(&path) {
            return Path::new(name)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("o"));
        }
        if self.target_dir.as_os_str().is_empty() || !path.starts_with(&self.target_dir) {
            return false;
        }
        let parent = path.parent().and_then(Path::file_name);
        let grandparent = path
            .parent()
            .and_then(Path::parent)
            .and_then(Path::file_name);
        let deps = parent.is_some_and(|dir| dir == "deps");
        let build = grandparent.is_some_and(|dir| dir == "build");
        let rlib = name.ends_with(".rlib");
        let object = name.ends_with(".rcgu.o");
        let dylib = name.ends_with(".so") || name.ends_with(".dylib");
        (deps && (rlib || object || dylib)) || (build && object)
    }

    fn library(&self, scan: &mut LinkScan, name: &str) {
        scan.named.push(name.to_owned());
        if self.libraries.iter().any(|allowed| allowed == name) {
            scan.libraries.push(name.to_owned());
        } else {
            scan.problems.push(format!(
                "library `{name}` is not in the trusted link_libraries allowlist"
            ));
        }
    }

    fn framework(&self, scan: &mut LinkScan, name: &str) {
        scan.named.push(name.to_owned());
        if self.frameworks.iter().any(|allowed| allowed == name) {
            scan.frameworks.push(name.to_owned());
        } else {
            scan.problems.push(format!(
                "framework `{name}` is not in the trusted link_frameworks allowlist"
            ));
        }
    }

    fn input(&self, scan: &mut LinkScan, file: &str) {
        scan.named.push(file.to_owned());
        if !self.rust_artifact(file) {
            scan.problems.push(format!(
                "linker input `{file}` is not a Rust artifact of this build or the toolchain"
            ));
        } else if let Some(kind @ (FileKind::Script | FileKind::ThinArchive)) =
            file_kind(Path::new(file))
        {
            // A Rust artifact is an object or an archive; a script or thin
            // archive in its place names other files (R7-01).
            scan.problems.push(format!(
                "linker input `{file}` is a {} rather than an object or archive (R7-01)",
                if kind == FileKind::Script {
                    "linker script"
                } else {
                    "thin archive"
                }
            ));
        }
    }

    fn search_dir(scan: &mut LinkScan, option: &str, dir: &str) {
        if dir.starts_with('=') || dir.starts_with("$SYSROOT") {
            scan.problems.push(format!(
                "`{option}{dir}`: sysroot-relative search directories are refused (R7-01)"
            ));
        } else if option == "-F" {
            scan.framework_dirs.push(dir.to_owned());
        } else {
            scan.lib_dirs.push(dir.to_owned());
        }
    }

    /// Whether a directory is one of the toolchain's or the system's own
    /// library directories (or a trusted root of `gates.toml`), and not a
    /// directory this build or its build scripts write (R7-01).
    #[must_use]
    pub fn trusted_dir(&self, dir: &Path) -> bool {
        let dir = canonical(dir);
        let inside = |root: &Path| !root.as_os_str().is_empty() && dir.starts_with(root);
        if inside(&self.target_dir) || inside(&self.temp) {
            return false;
        }
        inside(&self.sysroot)
            || self
                .system_dirs
                .iter()
                .chain(&self.search_roots)
                .any(|root| root.is_absolute() && inside(&canonical(root)))
    }

    fn temp_file(&self, scan: &mut LinkScan, option: &str, file: &str) {
        scan.named.push(file.to_owned());
        if !self.rustc_temp(&canonical(Path::new(file))) {
            scan.problems.push(format!(
                "`{option}` file `{file}` is not a rustc temporary file"
            ));
        }
    }

    /// One piece of a library-naming option; returns true if it consumed
    /// the piece.
    fn attached(&self, scan: &mut LinkScan, piece: &str) -> bool {
        if let Some(dir) = piece
            .strip_prefix("--library-path=")
            .or_else(|| piece.strip_prefix("-L"))
        {
            Self::search_dir(scan, "-L", dir);
            return true;
        }
        if let Some(dir) = piece.strip_prefix("-F") {
            Self::search_dir(scan, "-F", dir);
            return true;
        }
        if let Some(dir) = piece
            .strip_prefix("-rpath-link=")
            .or_else(|| piece.strip_prefix("--rpath-link="))
        {
            self.rpath_link(scan, dir);
            return true;
        }
        for prefix in NAME_PREFIXES {
            if let Some(name) = piece.strip_prefix(prefix) {
                if name.is_empty() {
                    return false;
                }
                self.library(scan, name);
                return true;
            }
        }
        for option in LINKER_FILES {
            if let Some(file) = piece
                .strip_prefix(option)
                .and_then(|rest| rest.strip_prefix('='))
            {
                self.temp_file(scan, option, file);
                return true;
            }
        }
        false
    }

    /// Scans one level (driver or linker) of arguments.
    fn scan(&self, scan: &mut LinkScan, pieces: &[String], driver: bool) {
        let values = if driver { DRIVER_VALUES } else { LINKER_VALUES };
        let mut iter = pieces.iter();
        while let Some(piece) = iter.next() {
            let piece = piece.as_str();
            if piece.is_empty() {
                continue;
            }
            if refused_option(piece) {
                scan.problems.push(format!(
                    "linker option `{piece}` (response files, scripts, plugins and driver redirection are refused)"
                ));
                continue;
            }
            if driver && (piece == "-Wl" || piece.starts_with("-Wl,")) {
                // Handled by the caller (collected into the linker level).
                continue;
            }
            if driver && piece == "-Xlinker" {
                iter.next();
                continue;
            }
            if NAME_FLAGS.contains(&piece) {
                match iter.next() {
                    Some(name) => self.library(scan, name),
                    None => scan.problems.push(format!("`{piece}` without a value")),
                }
            } else if FRAMEWORK_FLAGS.contains(&piece) {
                match iter.next() {
                    Some(name) => self.framework(scan, name),
                    None => scan.problems.push(format!("`{piece}` without a value")),
                }
            } else if LIBRARY_FILE_FLAGS.contains(&piece) {
                match iter.next() {
                    Some(file) => self.input(scan, file),
                    None => scan.problems.push(format!("`{piece}` without a value")),
                }
            } else if LINKER_FILES.contains(&piece) {
                match iter.next() {
                    Some(file) => self.temp_file(scan, piece, file),
                    None => scan.problems.push(format!("`{piece}` without a value")),
                }
            } else if piece == "-platform_version" {
                iter.next();
                iter.next();
                iter.next();
            } else if matches!(piece, "-L" | "-F" | "--library-path") {
                match iter.next() {
                    Some(dir) => {
                        Self::search_dir(scan, if piece == "-F" { "-F" } else { "-L" }, dir);
                    }
                    None => scan.problems.push(format!("`{piece}` without a value")),
                }
            } else if matches!(piece, "-rpath-link" | "--rpath-link") {
                match iter.next() {
                    Some(dir) => self.rpath_link(scan, dir),
                    None => scan.problems.push(format!("`{piece}` without a value")),
                }
            } else if values.contains(&piece) {
                if piece == "-o" {
                    iter.next();
                } else if let Some(value) = iter.next() {
                    scan.named.push(value.clone());
                }
            } else if driver && piece == "-B" {
                match iter.next() {
                    Some(dir) => self.driver_prefix(scan, dir),
                    None => scan.problems.push("`-B` without a value".to_owned()),
                }
            } else if driver && let Some(dir) = piece.strip_prefix("-B") {
                self.driver_prefix(scan, dir);
            } else if let Some(linker) = piece.strip_prefix("-fuse-ld=") {
                if !matches!(linker, "lld" | "bfd" | "gold" | "mold") {
                    scan.problems
                        .push(format!("`{piece}` selects a linker program by path"));
                }
            } else if piece.starts_with('-') {
                self.attached(scan, piece);
            } else {
                self.input(scan, piece);
            }
        }
    }

    /// `-rpath-link` directories resolve shared-library dependencies: only
    /// trusted directories (R7-01).
    fn rpath_link(&self, scan: &mut LinkScan, dir: &str) {
        for dir in dir.split(':').filter(|dir| !dir.is_empty()) {
            if !self.trusted_dir(Path::new(dir)) {
                scan.problems.push(format!(
                    "`-rpath-link` directory `{dir}` is not a trusted system directory (R7-01)"
                ));
            }
        }
    }

    /// Resolves every allow-listed library and framework to the files the
    /// linker could open (R7-01). A candidate file in a search directory that
    /// is not a trusted system or toolchain directory is refused, whatever it
    /// contains: such a directory could shadow the system library with a
    /// linker script or another library. The file found in the trusted
    /// directories is then inspected: a thin archive is refused, and a
    /// system linker script's inputs are resolved and checked the same way.
    fn resolve(&self, scan: &mut LinkScan) {
        let mut seen = BTreeSet::new();
        for name in std::mem::take(&mut scan.libraries) {
            self.resolve_library(scan, &name, false, 0, &mut seen);
        }
        for name in std::mem::take(&mut scan.frameworks) {
            let bundle = format!("{name}.framework");
            for dir in &scan.framework_dirs {
                let file = Path::new(dir).join(&bundle);
                if !self.trusted_dir(Path::new(dir)) && file.exists() {
                    scan.problems.push(format!(
                        "framework `{name}` would resolve to `{}` in search directory `{dir}`, which is not a trusted system or toolchain directory (R7-01)",
                        file.display()
                    ));
                }
            }
        }
    }

    /// `script_relative`: a relative input of a linker script, which the
    /// linker also looks for in its working directory.
    fn resolve_library(
        &self,
        scan: &mut LinkScan,
        name: &str,
        script_relative: bool,
        depth: usize,
        seen: &mut BTreeSet<PathBuf>,
    ) {
        let candidates = if script_relative {
            vec![name.to_owned()]
        } else {
            library_candidates(name)
        };
        let mut dirs: Vec<PathBuf> = Vec::new();
        if script_relative {
            dirs.push(self.cwd.clone());
        }
        dirs.extend(scan.lib_dirs.iter().map(PathBuf::from));
        let mut found = None;
        for dir in &dirs {
            for candidate in &candidates {
                let file = dir.join(candidate);
                if !file.exists() {
                    continue;
                }
                if !self.trusted_dir(dir) {
                    scan.named.push(file.to_string_lossy().into_owned());
                    if let Ok(text) = std::fs::read_to_string(&file) {
                        scan.named.extend(script_inputs(&text).unwrap_or_default());
                    }
                    scan.problems.push(format!(
                        "library `{name}` would resolve to `{}` in search directory `{}`, which is not a trusted system or toolchain directory (R7-01)",
                        file.display(),
                        dir.display()
                    ));
                } else if found.is_none() {
                    found = Some(file);
                }
            }
        }
        if found.is_none() {
            found = self.system_dirs.iter().find_map(|dir| {
                candidates
                    .iter()
                    .map(|candidate| dir.join(candidate))
                    .find(|file| file.exists())
            });
        }
        if let Some(file) = found {
            self.inspect(scan, &file, depth, seen);
        }
    }

    /// Checks a file in a trusted directory that the linker opens.
    fn inspect(
        &self,
        scan: &mut LinkScan,
        file: &Path,
        depth: usize,
        seen: &mut BTreeSet<PathBuf>,
    ) {
        let file = canonical(file);
        if !seen.insert(file.clone()) {
            return;
        }
        scan.named.push(file.to_string_lossy().into_owned());
        match file_kind(&file) {
            None | Some(FileKind::Binary) => {}
            Some(FileKind::ThinArchive) => scan.problems.push(format!(
                "`{}` is a thin archive, whose members are other files (R7-01)",
                file.display()
            )),
            Some(FileKind::Script) => {
                if depth >= 8 {
                    scan.problems.push(format!(
                        "linker script `{}` nests too deeply (R7-01)",
                        file.display()
                    ));
                    return;
                }
                let inputs = std::fs::read_to_string(&file)
                    .map_err(|error| error.to_string())
                    .and_then(|text| script_inputs(&text));
                let inputs = match inputs {
                    Ok(inputs) => inputs,
                    Err(reason) => {
                        scan.problems.push(format!(
                            "linker script `{}` is refused: {reason} (R7-01)",
                            file.display()
                        ));
                        return;
                    }
                };
                for input in inputs {
                    if let Some(name) = input.strip_prefix("-l") {
                        scan.named.push(name.to_owned());
                        if self.libraries.iter().any(|allowed| allowed == name) {
                            self.resolve_library(scan, name, false, depth + 1, seen);
                        } else {
                            scan.problems.push(format!(
                                "linker script `{}` asks for library `{name}`, which is not in the trusted link_libraries allowlist (R7-01)",
                                file.display()
                            ));
                        }
                    } else if Path::new(&input).is_absolute() {
                        let path = Path::new(&input);
                        if path.parent().is_some_and(|dir| self.trusted_dir(dir)) {
                            self.inspect(scan, path, depth + 1, seen);
                        } else {
                            scan.named.push(input.clone());
                            scan.problems.push(format!(
                                "linker script `{}` names `{input}`, outside the trusted system and toolchain directories (R7-01)",
                                file.display()
                            ));
                        }
                    } else {
                        scan.named.push(input.clone());
                        self.resolve_library(scan, &input, true, depth + 1, seen);
                    }
                }
            }
        }
    }

    fn driver_prefix(&self, scan: &mut LinkScan, dir: &str) {
        if self.sysroot.as_os_str().is_empty()
            || !canonical(Path::new(dir)).starts_with(&self.sysroot)
        {
            scan.problems.push(format!(
                "`-B{dir}` points the driver at programs outside the toolchain"
            ));
        }
    }

    /// Problems with one linker (C driver) command line: an empty list means
    /// it may be run.
    #[must_use]
    pub fn check(&self, args: &[String]) -> Vec<String> {
        let mut scan = LinkScan::default();
        let mut linker_level = Vec::new();
        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            if let Some(list) = arg.strip_prefix("-Wl,") {
                linker_level.extend(list.split(',').map(str::to_owned));
            } else if arg == "-Xlinker"
                && let Some(value) = iter.next()
            {
                linker_level.push(value.clone());
            }
        }
        self.scan(&mut scan, args, true);
        self.scan(&mut scan, &linker_level, false);
        self.resolve(&mut scan);
        let runtimes: BTreeSet<&String> = scan
            .named
            .iter()
            .filter(|name| names_runtime(name))
            .collect();
        let mut problems = scan.problems;
        if !runtimes.is_empty() {
            problems.insert(
                0,
                format!(
                    "linking a Python or Julia runtime is forbidden; the linker was asked for {}",
                    runtimes
                        .iter()
                        .map(|name| name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            );
        }
        problems
    }
}

/// Environment variables that change which programs the C driver runs or
/// where it searches; removed before `cc` runs.
const DRIVER_ENV: &[&str] = &["COMPILER_PATH", "GCC_EXEC_PREFIX", "LIBRARY_PATH"];

/// The C driver's own library search directories, as `cc
/// -print-search-dirs` reports them (in the guard's sanitised environment).
fn driver_search_dirs() -> Vec<PathBuf> {
    let mut command = Command::new("cc");
    command
        .arg("-print-search-dirs")
        .env_remove(LINK_ENV)
        .env_remove(WRAP_ENV)
        .stdin(Stdio::null())
        .stderr(Stdio::null());
    for name in DRIVER_ENV {
        command.env_remove(name);
    }
    let Ok(output) = command.output() else {
        return Vec::new();
    };
    parse_search_dirs(&String::from_utf8_lossy(&output.stdout))
}

/// The `libraries:` line of `cc -print-search-dirs`: absolute directories
/// only.
#[must_use]
pub fn parse_search_dirs(text: &str) -> Vec<PathBuf> {
    text.lines()
        .filter_map(|line| line.strip_prefix("libraries:"))
        .flat_map(|list| list.trim().trim_start_matches('=').split(':'))
        .map(Path::new)
        .filter(|dir| dir.is_absolute())
        .map(canonical)
        .collect()
}

fn link(args: &[OsString], config: Option<&Path>) -> ExitCode {
    let text: Vec<String> = args
        .iter()
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect();
    let config = config
        .filter(|path| path.is_file())
        .map(Config::read)
        .transpose();
    let config = match config {
        Ok(config) => config.unwrap_or_default(),
        Err(error) => {
            eprintln!("error: NFR-10: build gate linker guard: {error}");
            return ExitCode::from(1);
        }
    };
    let policy = LinkPolicy {
        libraries: config.libraries,
        frameworks: config.frameworks,
        target_dir: config.target_dir,
        sysroot: config.sysroot,
        temp: canonical(&std::env::temp_dir()),
        system_dirs: driver_search_dirs(),
        search_roots: config.search_roots,
        cwd: std::env::current_dir().unwrap_or_default(),
    };
    let problems = policy.check(&text);
    if !problems.is_empty() {
        for problem in problems {
            eprintln!("error: NFR-10: build gate linker guard: {problem}");
        }
        return ExitCode::from(1);
    }
    let mut command = Command::new("cc");
    command.args(args).env_remove(LINK_ENV).env_remove(WRAP_ENV);
    for name in DRIVER_ENV {
        command.env_remove(name);
    }
    exit(command.status(), "the linker (cc)")
}

// ---------------------------------------------------------------------------
// The gate.

fn package_name(id: &str) -> String {
    // `path+file:///r/crates/engine#egs-engine@0.0.0`, or `...#0.0.0` when
    // the name is the directory name, or `registry+...#name@1.0.0`.
    let (url, tail) = id.rsplit_once('#').unwrap_or((id, ""));
    match tail.split_once('@') {
        Some((name, _)) => name.to_owned(),
        None => url.rsplit('/').next().unwrap_or(id).to_owned(),
    }
}

/// Whether a build script's `cargo:rustc-env` may not set this variable: it
/// would reach the wrapper, rustc or the linker guard and could redirect the
/// toolchain or this gate.
#[must_use]
pub fn is_protected_build_env(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    [
        "XTASK_", "CARGO", "__CARGO", "RUSTC", "RUSTDOC", "RUSTUP", "RUST_", "LD_", "DYLD_",
        "GITHUB_", "RUNNER_",
    ]
    .iter()
    .any(|prefix| upper.starts_with(prefix))
        || upper.contains("RUSTFLAGS")
        || [
            "PATH",
            "CC",
            "CXX",
            "AR",
            "LD",
            "LIBRARY_PATH",
            "COMPILER_PATH",
            "GCC_EXEC_PREFIX",
            "TMPDIR",
            "TMP",
            "TEMP",
            "HOME",
            "SDKROOT",
        ]
        .contains(&upper.as_str())
}

/// Findings from Cargo's JSON message stream.
#[must_use]
pub fn analyse(messages: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    for line in messages.lines() {
        let Ok(message) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        let package = package_name(message["package_id"].as_str().unwrap_or_default());
        match message["reason"].as_str() {
            Some("build-script-executed") => {
                for library in message["linked_libs"].as_array().into_iter().flatten() {
                    let library = library.as_str().unwrap_or_default();
                    if names_runtime(library) {
                        findings.push(Finding::general(format!(
                            "build script of `{package}` asks Cargo to link `{library}`: linking a Python or Julia runtime is forbidden (NFR-10)"
                        )));
                    }
                }
                for search in message["linked_paths"].as_array().into_iter().flatten() {
                    let search = search.as_str().unwrap_or_default();
                    let dir = search.split_once('=').map_or(search, |(_, dir)| dir);
                    let Ok(entries) = std::fs::read_dir(dir) else {
                        continue;
                    };
                    for entry in entries.flatten() {
                        let name = entry.file_name().to_string_lossy().into_owned();
                        let library = [".a", ".so", ".dylib", ".tbd", ".lib", ".dll"]
                            .iter()
                            .any(|ext| name.to_ascii_lowercase().ends_with(ext))
                            || name.contains(".so.")
                            || name.to_ascii_lowercase().ends_with(".framework");
                        if names_runtime(&name) && library {
                            findings.push(Finding::general(format!(
                                "build script of `{package}` adds link search path `{dir}` holding `{name}`: linking a Python or Julia runtime is forbidden (NFR-10)"
                            )));
                        }
                    }
                }
                for pair in message["env"].as_array().into_iter().flatten() {
                    let name = pair
                        .as_array()
                        .and_then(|pair| pair.first())
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or_default();
                    if is_protected_build_env(name) {
                        findings.push(Finding::general(format!(
                            "build script of `{package}` sets `{name}` for the compiler: build scripts may not redirect the toolchain or the build gate (NFR-10)"
                        )));
                    }
                }
            }
            Some("compiler-message") if message["message"]["level"] == "error" => {
                let inner = &message["message"];
                let span = inner["spans"]
                    .as_array()
                    .and_then(|spans| spans.iter().find(|s| s["is_primary"] == true));
                let mut text = format!(
                    "`{package}`: {}",
                    inner["message"].as_str().unwrap_or("compiler error")
                );
                let rendered = inner["rendered"].as_str().unwrap_or_default();
                for line in rendered.lines().filter(|line| line.contains("NFR-10")) {
                    text.push_str(" | ");
                    text.push_str(line.trim());
                }
                findings.push(match span {
                    Some(span) => Finding::at(
                        span["file_name"].as_str().unwrap_or_default(),
                        span["line_start"]
                            .as_u64()
                            .and_then(|line| usize::try_from(line).ok()),
                        text,
                    ),
                    None => Finding::general(text),
                });
            }
            _ => {}
        }
    }
    findings
}

/// One feature configuration the gate builds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeatureSet {
    /// Every member with its default features.
    Default,
    /// Every member with every feature.
    All,
    /// Every member without default features.
    NoDefault,
    /// One member (package id) alone with its default features.
    PackageDefault {
        /// Package id spec.
        package: String,
        /// Package name, for messages.
        name: String,
    },
    /// One member (package id) alone without default features.
    PackageNoDefault {
        /// Package id spec.
        package: String,
        /// Package name, for messages.
        name: String,
    },
    /// One member (package id) with exactly one feature.
    Single {
        /// Package id spec.
        package: String,
        /// Package name, for messages.
        name: String,
        /// The feature.
        feature: String,
    },
}

impl FeatureSet {
    /// Cargo arguments selecting this configuration.
    #[must_use]
    pub fn args(&self) -> Vec<String> {
        let strings = |items: &[&str]| items.iter().map(|item| (*item).to_owned()).collect();
        match self {
            Self::Default => strings(&["--workspace"]),
            Self::All => strings(&["--workspace", "--all-features"]),
            Self::NoDefault => strings(&["--workspace", "--no-default-features"]),
            Self::PackageDefault { package, .. } => vec!["--package".to_owned(), package.clone()],
            Self::PackageNoDefault { package, .. } => vec![
                "--package".to_owned(),
                package.clone(),
                "--no-default-features".to_owned(),
            ],
            Self::Single {
                package, feature, ..
            } => vec![
                "--package".to_owned(),
                package.clone(),
                "--no-default-features".to_owned(),
                "--features".to_owned(),
                feature.clone(),
            ],
        }
    }

    /// A short label for messages.
    #[must_use]
    pub fn label(&self) -> String {
        match self {
            Self::Default => "default features".to_owned(),
            Self::All => "all features".to_owned(),
            Self::NoDefault => "no default features".to_owned(),
            Self::PackageDefault { name, .. } => format!("`{name}` alone, default features"),
            Self::PackageNoDefault { name, .. } => format!("`{name}` alone, no default features"),
            Self::Single { name, feature, .. } => format!("`{name}` with only `{feature}`"),
        }
    }
}

/// The feature configurations to build (R6-03, R7-02): the workspace with
/// default, all and no default features, and every member alone with its
/// default features, without them, and with each of its features alone.
/// Building a member alone keeps other members from unifying features into
/// it; [`Mode`] keeps dev-dependencies from doing so.
#[must_use]
pub fn feature_sets(metadata: &Metadata) -> Vec<FeatureSet> {
    let mut sets = vec![FeatureSet::Default, FeatureSet::All, FeatureSet::NoDefault];
    for (package, id) in metadata.packages.iter().zip(&metadata.ids) {
        if !metadata.members.contains(id) {
            continue;
        }
        sets.push(FeatureSet::PackageDefault {
            package: id.clone(),
            name: package.name.clone(),
        });
        // Without a `default` feature, no default features is the same
        // configuration as the default one.
        if package.features.iter().any(|feature| feature == "default") {
            sets.push(FeatureSet::PackageNoDefault {
                package: id.clone(),
                name: package.name.clone(),
            });
        }
        for feature in &package.features {
            if feature != "default" {
                sets.push(FeatureSet::Single {
                    package: id.clone(),
                    name: package.name.clone(),
                    feature: feature.clone(),
                });
            }
        }
    }
    sets
}

/// How one configuration is built (R7-02). Cargo resolves features per
/// command: building tests, examples or doctests activates dev-dependencies,
/// whose features unify into the package. A production build (library and
/// binaries, no dev-dependencies) is therefore its own configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `cargo build`: libraries and binaries, without dev-dependencies.
    Production,
    /// `cargo build --all-targets`: tests, examples and benches too.
    AllTargets,
    /// `cargo test --doc`.
    Doctests,
}

impl Mode {
    /// Every mode, in build order.
    pub const ALL: [Self; 3] = [Self::Production, Self::AllTargets, Self::Doctests];

    /// The Cargo subcommand and its arguments.
    #[must_use]
    pub fn args(self) -> &'static [&'static str] {
        match self {
            Self::Production => &["build", "--keep-going"],
            Self::AllTargets => &["build", "--all-targets", "--keep-going"],
            Self::Doctests => &["test", "--doc", "--no-fail-fast"],
        }
    }

    /// A short label for messages.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Production => "build (production targets)",
            Self::AllTargets => "build --all-targets",
            Self::Doctests => "test --doc",
        }
    }
}

/// Options of one gate run.
#[derive(Debug, Clone, Default)]
pub struct Options {
    /// `--jobs` for Cargo (the coding container builds with few jobs).
    pub jobs: Option<u32>,
}

struct Run {
    messages: String,
    other: Vec<String>,
    stderr: String,
    success: bool,
}

fn run_cargo(mut command: Command) -> Result<Run, String> {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("cargo: {error}"))?;
    let stderr = child.stderr.take().map(|stderr| {
        std::thread::spawn(move || {
            let mut text = String::new();
            for line in BufReader::new(stderr).lines().map_while(Result::ok) {
                text.push_str(&line);
                text.push('\n');
            }
            text
        })
    });
    let mut messages = String::new();
    let mut other = Vec::new();
    if let Some(stdout) = child.stdout.take() {
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            if line.starts_with('{') {
                messages.push_str(&line);
                messages.push('\n');
            } else {
                other.push(line);
            }
        }
    }
    let status = child.wait().map_err(|error| format!("cargo: {error}"))?;
    Ok(Run {
        messages,
        other,
        stderr: stderr
            .and_then(|handle| handle.join().ok())
            .unwrap_or_default(),
        success: status.success(),
    })
}

fn failure_finding(what: &str, run: &Run) -> Finding {
    let mut lines: Vec<&str> = run
        .other
        .iter()
        .map(String::as_str)
        .filter(|line| line.contains("error") || line.contains("NFR-10") || line.contains("unsafe"))
        .collect();
    let tail: Vec<&str> = run.stderr.lines().rev().take(20).collect();
    lines.extend(tail.into_iter().rev());
    Finding::general(format!(
        "{what} failed under the build gate: {}",
        lines.join("\n")
    ))
}

fn rustc_sysroot(ctx: &Ctx) -> Result<PathBuf, String> {
    let rustc = cargo(ctx)
        .get_envs()
        .find(|(name, _)| *name == "RUSTC")
        .and_then(|(_, value)| value.map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("rustc"));
    let output = Command::new(rustc)
        .args(["--print", "sysroot"])
        .current_dir(&ctx.policy)
        .output()
        .map_err(|error| format!("rustc --print sysroot: {error}"))?;
    if !output.status.success() {
        return Err("rustc --print sysroot failed".to_owned());
    }
    Ok(canonical(Path::new(
        String::from_utf8_lossy(&output.stdout).trim(),
    )))
}

/// Builds the workspace under test with the compiler enforcing the unsafe
/// boundary and the linker guard enforcing the native-runtime ban.
/// `wrapper` is the trusted xtask binary; `target_dir` receives the build.
///
/// # Errors
///
/// Fails if Cargo cannot be started or the trusted policy is unreadable.
pub fn check(ctx: &Ctx, wrapper: &Path, target_dir: &Path) -> Result<Vec<Finding>, String> {
    check_with(ctx, wrapper, target_dir, &Options::default())
}

/// [`check`] with options.
///
/// # Errors
///
/// Fails if Cargo cannot be started or the trusted policy is unreadable.
pub fn check_with(
    ctx: &Ctx,
    wrapper: &Path,
    target_dir: &Path,
    options: &Options,
) -> Result<Vec<Finding>, String> {
    let repo = ctx
        .repo
        .canonicalize()
        .map_err(|error| format!("{}: {error}", ctx.repo.display()))?;
    let metadata = match metadata(ctx) {
        Ok(metadata) => metadata,
        Err(message) => return Ok(vec![Finding::at("Cargo.toml", None, message)]),
    };
    let gates = ctx.policy_toml(crate::workflows::GATES)?;
    std::fs::create_dir_all(target_dir)
        .map_err(|error| format!("{}: {error}", target_dir.display()))?;
    let target_dir = canonical(target_dir);
    let gate_dir = target_dir.join("egs-build-gate");
    std::fs::create_dir_all(&gate_dir)
        .map_err(|error| format!("{}: {error}", gate_dir.display()))?;
    let linker = gate_dir.join(format!("{LINKER_NAME}{}", std::env::consts::EXE_SUFFIX));
    let _ = std::fs::remove_file(&linker);
    std::fs::copy(wrapper, &linker).map_err(|error| format!("{}: {error}", linker.display()))?;
    let config = Config {
        target_dir: target_dir.clone(),
        sysroot: rustc_sysroot(ctx)?,
        linker: linker.clone(),
        libraries: gates
            .get("link_libraries")
            .map(string_array)
            .unwrap_or_default(),
        frameworks: gates
            .get("link_frameworks")
            .map(string_array)
            .unwrap_or_default(),
        search_roots: gates
            .get("link_search_roots")
            .map(string_array)
            .unwrap_or_default()
            .into_iter()
            .map(PathBuf::from)
            .collect(),
        units: Config::units(&metadata),
    };
    let config_path = gate_dir.join("config.json");
    std::fs::write(&config_path, config.to_json())
        .map_err(|error| format!("{}: {error}", config_path.display()))?;

    let sets = feature_sets(&metadata);
    let mut findings = Vec::new();
    if sets.len() > MAX_FEATURE_SETS {
        findings.push(Finding::general(format!(
            "{} feature configurations exceed the build gate's bound of {MAX_FEATURE_SETS}; the gate fails closed (R6-03)",
            sets.len()
        )));
        return Ok(findings);
    }
    let mut seen = BTreeSet::new();
    for set in &sets {
        for mode in Mode::ALL {
            let mut command = cargo(ctx);
            command.args(mode.args());
            command
                .args(set.args())
                .args(["--locked", "--message-format=json", "--manifest-path"])
                .arg(repo.join("Cargo.toml"))
                .arg("--target-dir")
                .arg(&target_dir);
            if let Some(jobs) = options.jobs {
                command.arg("--jobs").arg(jobs.to_string());
            }
            for name in FILE_COMMAND_ENV {
                command.env_remove(name);
            }
            command
                .env("RUSTC_WRAPPER", wrapper)
                .env(WRAP_ENV, &config_path)
                .env(
                    "CARGO_ENCODED_RUSTDOCFLAGS",
                    format!("-C\u{1f}linker={}", linker.display()),
                );
            let run = run_cargo(command)?;
            let mut new = analyse(&run.messages);
            let what = format!("cargo {} ({})", mode.label(), set.label());
            if !run.success && new.is_empty() {
                new.push(failure_finding(&what, &run));
            }
            for finding in new {
                let key = (finding.file.clone(), finding.line, finding.message.clone());
                if seen.insert(key) {
                    findings.push(finding);
                }
            }
        }
    }
    for finding in &mut findings {
        if let Some(file) = &finding.file
            && let Ok(rest) = Path::new(file).strip_prefix(&repo)
        {
            finding.file = Some(rest.to_string_lossy().into_owned());
        }
    }
    Ok(findings)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(args: &[&str]) -> Vec<String> {
        args.iter().map(|arg| (*arg).to_owned()).collect()
    }

    const REPO: Owner = Owner::Repository { exempt: false };
    const ADAPTER: Owner = Owner::Repository { exempt: true };

    #[test]
    fn wrapper_forbids_unsafe_and_strips_lint_caps_for_repository_crates() {
        let args = strings(&[
            "--crate-name",
            "egs_engine",
            "--edition=2024",
            "crates/engine/src/lib.rs",
            "--cap-lints",
            "allow",
            "--cap-lints=warn",
            "-A",
            "unsafe_code",
        ]);
        let Wrapped::Run(out) = rewrite(&args, Some(REPO), "/x") else {
            panic!("refused");
        };
        assert!(!out.iter().any(|arg| arg.contains("cap-lints")), "{out:?}");
        assert_eq!(
            out[out.len() - 4..],
            strings(&["-F", "unsafe_code", "-C", "linker=/x"])
        );
        let Wrapped::Run(out) = rewrite(&args, Some(ADAPTER), "/x") else {
            panic!("refused");
        };
        assert!(!out.contains(&"-F".to_owned()));
        // Registry crates keep Cargo's cap and only get the linker guard.
        let Wrapped::Run(out) = rewrite(&args, Some(Owner::External), "/x") else {
            panic!("refused");
        };
        assert!(out.contains(&"--cap-lints".to_owned()));
        assert!(!out.contains(&"-F".to_owned()));
        // Argument files are refused for every crate.
        for owner in [Some(REPO), Some(Owner::External)] {
            assert!(matches!(
                rewrite(
                    &strings(&["--crate-name", "a", "a.rs", "@flags"]),
                    owner,
                    "/x"
                ),
                Wrapped::Refuse(_)
            ));
        }
    }

    // R6-01: a crate name decides nothing. A real compilation called `___`
    // that cannot be attributed is refused; one that can is treated by its
    // package; Cargo's stdin probe gets the strictest flags.
    #[test]
    fn r6_01_crate_names_never_exempt_a_compilation() {
        let compile = strings(&["--crate-name", "___", "--edition=2024", "src/lib.rs"]);
        assert!(matches!(rewrite(&compile, None, "/x"), Wrapped::Refuse(_)));
        let Wrapped::Run(out) = rewrite(&compile, Some(REPO), "/x") else {
            panic!("refused");
        };
        assert!(out.ends_with(&strings(&["-F", "unsafe_code", "-C", "linker=/x"])));
        let probe = strings(&[
            "-",
            "--crate-name",
            "___",
            "--print=file-names",
            "--crate-type",
            "bin",
        ]);
        assert_eq!(classify(&probe), Invocation::Probe);
        let Wrapped::Run(out) = rewrite(&probe, None, "/x") else {
            panic!("refused");
        };
        assert!(out.ends_with(&strings(&["-F", "unsafe_code", "-C", "linker=/x"])));
        // Only a bare version query passes through unchanged.
        let version = strings(&["-vV"]);
        assert_eq!(rewrite(&version, None, "/x"), Wrapped::Run(version.clone()));
        assert!(matches!(
            classify(&strings(&["a.rs", "b.rs"])),
            Invocation::Unclear(_)
        ));
    }

    #[test]
    fn r6_01_attribution_uses_the_package_graph() {
        let dir = std::env::temp_dir();
        let src = dir.join("lib.rs");
        let script = dir.join("build.rs");
        let unit = |name: &str, local: bool, adapter: bool| Unit {
            id: format!("path+file://{}#{name}@0.0.0", dir.display()),
            name: name.to_owned(),
            dir: canonical(&dir),
            local,
            adapter,
            targets: vec![
                UnitTarget {
                    src: canonical(&src),
                    build_script: false,
                },
                UnitTarget {
                    src: canonical(&script),
                    build_script: true,
                },
            ],
        };
        let config = Config {
            units: vec![unit("egs-solver-highs", true, true)],
            ..Config::default()
        };
        let attr =
            |name: Option<&str>, source: &str| attribute(&config, Some(&dir), name, &dir, source);
        assert_eq!(attr(Some("egs-solver-highs"), "lib.rs"), Some(ADAPTER));
        // Its build script is never exempt.
        assert_eq!(attr(Some("egs-solver-highs"), "build.rs"), Some(REPO));
        // Wrong name, unknown source or no manifest directory: unattributed.
        assert_eq!(attr(Some("other"), "lib.rs"), None);
        assert_eq!(attr(Some("egs-solver-highs"), "other.rs"), None);
        assert_eq!(attr(None, "lib.rs"), None);
        assert_eq!(
            attribute(&config, None, Some("egs-solver-highs"), &dir, "lib.rs"),
            None
        );
        let config = Config {
            units: vec![unit("serde", false, false)],
            ..Config::default()
        };
        assert_eq!(
            attribute(&config, Some(&dir), Some("serde"), &dir, "lib.rs"),
            Some(Owner::External)
        );
        let round = Config::from_json(&config.to_json()).expect("json");
        assert_eq!(round, config);
    }

    fn policy() -> LinkPolicy {
        LinkPolicy {
            libraries: strings(&["c", "m", "gcc_s", "util", "rt", "pthread", "dl"]),
            frameworks: strings(&["CoreFoundation"]),
            target_dir: PathBuf::from("/t"),
            sysroot: PathBuf::from("/s"),
            temp: PathBuf::from("/tmp"),
            system_dirs: Vec::new(),
            search_roots: Vec::new(),
            cwd: PathBuf::from("/t"),
        }
    }

    #[test]
    fn linker_guard_accepts_an_ordinary_rust_link_line() {
        let args = strings(&[
            "-m64",
            "/tmp/rustcAbc/symbols.o",
            "/t/debug/deps/x-1.x.abc-cgu.0.rcgu.o",
            "-Wl,--as-needed",
            "-Wl,-Bstatic",
            "/t/debug/deps/libegs_domain-1.rlib",
            "/s/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-2.rlib",
            "-Wl,-Bdynamic",
            "-lgcc_s",
            "-lutil",
            "-lrt",
            "-lpthread",
            "-lm",
            "-ldl",
            "-lc",
            "-L",
            "/t/debug/build/x-1/out",
            "-Wl,--eh-frame-hdr",
            "-Wl,-z,noexecstack",
            "-o",
            "/t/debug/deps/x-1",
            "-Wl,--gc-sections",
            "-pie",
            "-Wl,-z,relro,-z,now",
            "-nodefaultlibs",
            "-fuse-ld=lld",
            "-B/s/lib/rustlib/x86_64-unknown-linux-gnu/bin/gcc-ld",
            "-Wl,--version-script=/tmp/rustcAbc/list",
            "/t/debug/build/x-2/build_script_build-2.build_script_build.a-cgu.0.rcgu.o",
        ]);
        assert_eq!(policy().check(&args), Vec::<String>::new());
    }

    // R6-02: a response file is refused whatever its quoting, also forwarded.
    #[test]
    fn r6_02_response_files_are_refused() {
        for args in [
            strings(&["@/t/debug/build/x/out/native.rsp"]),
            strings(&["-Wl,@/tmp/x.rsp"]),
            strings(&["-Xlinker", "@/tmp/x.rsp"]),
        ] {
            let problems = policy().check(&args);
            assert!(
                problems.iter().any(|p| p.contains("response files")),
                "{args:?}: {problems:?}"
            );
        }
    }

    // R6-02 / ADV23R6-02: every library-naming spelling is checked against
    // the allowlist, and runtime names are reported as such.
    #[test]
    fn adv23r6_02_framework_and_library_spellings_are_allow_listed() {
        for args in [
            strings(&["-framework", "Python"]),
            strings(&["-Wl,-framework,Python"]),
            strings(&["-Xlinker", "-framework", "-Xlinker", "Python"]),
            strings(&["-weak_framework", "Julia"]),
            strings(&["-Wl,-needed-lpython3.12"]),
            strings(&["-Wl,--library=python3"]),
            strings(&["-l", "julia"]),
            strings(&["-l:libpython3.12.so"]),
            strings(&["-weak_library", "/usr/lib/libpython3.12.dylib"]),
            strings(&["/t/debug/build/x/out/libpython3.12.a"]),
            strings(&["/t/debug/build/x/out/libpyth\"on\"3.12.a"]),
        ] {
            let problems = policy().check(&args);
            assert!(!problems.is_empty(), "{args:?}");
        }
        let problems = policy().check(&strings(&["-framework", "Python"]));
        assert!(problems[0].contains("Python or Julia"), "{problems:?}");
        // Not a runtime but not allow-listed either: still refused.
        for args in [
            strings(&["-lssl"]),
            strings(&["-framework", "Metal"]),
            strings(&["/t/debug/build/x/out/libnative.a"]),
            strings(&["/opt/lib/libanything.so"]),
            strings(&["/t/debug/build/x/out/notes.txt"]),
            strings(&["-Wl,-T,/t/script.ld"]),
            strings(&["-B/evil"]),
            strings(&["-fuse-ld=/evil/ld"]),
            strings(&["-specs=/evil.specs"]),
            strings(&["-Wl,--version-script=/t/out/list"]),
        ] {
            assert!(!policy().check(&args).is_empty(), "{args:?}");
        }
        // Allow-listed names pass.
        assert!(
            policy()
                .check(&strings(&["-framework", "CoreFoundation", "-lm"]))
                .is_empty()
        );
    }

    /// A scratch directory for the resolution tests.
    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("egs-r7-01-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch");
        canonical(&dir)
    }

    const ELF: &[u8] = b"\x7fELF\x02\x01\x01\x00";

    fn write(path: &Path, bytes: &[u8]) {
        std::fs::create_dir_all(path.parent().expect("parent")).expect("dir");
        std::fs::write(path, bytes).expect("write");
    }

    /// A policy whose trusted system directory is `<root>/sys`; `<root>/out`
    /// stands for an OUT_DIR and `<root>/t` for the build directory.
    fn resolving(root: &Path) -> LinkPolicy {
        LinkPolicy {
            libraries: strings(&["c", "m", "gcc_s", "gcc", "iconv"]),
            frameworks: strings(&["CoreFoundation"]),
            target_dir: root.join("t"),
            sysroot: PathBuf::from("/s"),
            temp: PathBuf::from("/nonexistent-temp"),
            system_dirs: vec![root.join("sys")],
            search_roots: Vec::new(),
            cwd: root.join("pkg"),
        }
    }

    // R7-01: an allow-listed name that an untrusted search directory would
    // resolve to a linker script, a static or dynamic library or a framework
    // is refused, in every search-path spelling.
    #[test]
    fn r7_01_untrusted_search_directories_cannot_shadow_allowed_names() {
        let root = scratch("shadow");
        let out = root.join("out");
        let hidden = out.join("hidden/libpython3.12.a");
        write(&hidden, b"!<arch>\n");
        write(
            &out.join("libiconv.so"),
            format!("INPUT ( {} )\n", hidden.display()).as_bytes(),
        );
        write(&out.join("libm.a"), b"!<arch>\n");
        write(&out.join("libc.dylib"), ELF);
        std::fs::create_dir_all(out.join("CoreFoundation.framework")).expect("dir");
        let dir = out.display().to_string();
        let policy = resolving(&root);
        for args in [
            vec!["-L".to_owned(), dir.clone(), "-liconv".to_owned()],
            vec![format!("-L{dir}"), "-l".to_owned(), "iconv".to_owned()],
            vec![format!("-Wl,-L,{dir},-liconv")],
            vec![
                format!("-Wl,--library-path={dir}"),
                "-Wl,--library=iconv".to_owned(),
            ],
            vec![
                "-Xlinker".to_owned(),
                "--library-path".to_owned(),
                "-Xlinker".to_owned(),
                dir.clone(),
                "-lm".to_owned(),
            ],
            vec![format!("-L{dir}"), "-lc".to_owned()],
            vec![
                "-F".to_owned(),
                dir.clone(),
                "-framework".to_owned(),
                "CoreFoundation".to_owned(),
            ],
            vec![
                format!("-F{dir}"),
                "-Wl,-framework,CoreFoundation".to_owned(),
            ],
        ] {
            let problems = policy.check(&args);
            assert!(
                problems.iter().any(|p| p.contains("R7-01")),
                "{args:?}: {problems:?}"
            );
        }
        // The script's hidden input is reported as a runtime.
        let problems = policy.check(&[format!("-L{dir}"), "-liconv".to_owned()]);
        assert!(problems[0].contains("Python or Julia"), "{problems:?}");
        // Control: the same directory without a request it could shadow.
        assert_eq!(
            policy.check(&[format!("-L{dir}"), "-lgcc_s".to_owned()]),
            Vec::<String>::new()
        );
        // Sysroot-relative search paths, sysroot overrides and untrusted
        // -rpath-link directories are refused.
        for args in [
            strings(&["-L=/lib", "-lc"]),
            strings(&["-Wl,-L,$SYSROOT/lib"]),
            strings(&["--sysroot=/x"]),
            strings(&["-Wl,--sysroot=/x"]),
            vec![format!("-Wl,-rpath-link,{dir}")],
            strings(&["-Wl,-Y,P,/x"]),
        ] {
            assert!(!policy.check(&args).is_empty(), "{args:?}");
        }
        let _ = std::fs::remove_dir_all(&root);
    }

    // R7-01: the file found in the trusted directories is inspected. A
    // system linker script's inputs must stay in trusted directories and on
    // the allowlist, and may not be shadowed from the working directory.
    #[test]
    fn r7_01_system_linker_scripts_are_resolved_and_checked() {
        let root = scratch("system");
        let sys = root.join("sys");
        write(&sys.join("libc.so.6"), ELF);
        write(&sys.join("ld.so"), ELF);
        write(&sys.join("libgcc_s.so.1"), ELF);
        write(&sys.join("libgcc.a"), b"!<arch>\n");
        write(
            &sys.join("libc.so"),
            format!(
                "/* GNU ld script\n   Use the shared library. */\nOUTPUT_FORMAT(elf64-x86-64)\nGROUP ( {}/libc.so.6 AS_NEEDED ( {}/ld.so ) )\n",
                sys.display(),
                sys.display()
            )
            .as_bytes(),
        );
        write(
            &sys.join("libgcc_s.so"),
            b"/* GNU ld script */\nGROUP ( libgcc_s.so.1 -lgcc )\n",
        );
        let policy = resolving(&root);
        // Ordinary glibc/libgcc scripts pass.
        assert_eq!(
            policy.check(&strings(&["-lc", "-lgcc_s"])),
            Vec::<String>::new()
        );
        // A relative script input placed in the working directory is refused.
        write(&root.join("pkg/libgcc_s.so.1"), ELF);
        let problems = policy.check(&strings(&["-lgcc_s"]));
        assert!(problems.iter().any(|p| p.contains("R7-01")), "{problems:?}");
        let _ = std::fs::remove_dir_all(root.join("pkg"));
        // Inputs outside the trusted directories, unlisted libraries and other
        // script commands are refused.
        for script in [
            format!("INPUT ( {}/out/libx.a )\n", root.display()),
            "GROUP ( -lpython3.12 )\n".to_owned(),
            format!("SEARCH_DIR({}/out)\nINPUT(libx.a)\n", root.display()),
            "INCLUDE other.ld\n".to_owned(),
            "INPUT ( libc.so.6\n".to_owned(),
        ] {
            write(&sys.join("libm.so"), script.as_bytes());
            let problems = policy.check(&strings(&["-lm"]));
            assert!(
                problems.iter().any(|p| p.contains("R7-01")),
                "{script}: {problems:?}"
            );
        }
        // A thin archive names other files.
        write(&sys.join("libm.so"), b"!<thin>\n");
        assert!(!policy.check(&strings(&["-lm"])).is_empty());
        let _ = std::fs::remove_dir_all(&root);
    }

    // R7-01: a direct input in Rust's own output places must be an object or
    // archive, not a script that names other files.
    #[test]
    fn r7_01_direct_inputs_must_not_be_scripts() {
        let root = scratch("direct");
        let deps = root.join("t/debug/deps");
        write(
            &deps.join("libx-1.rlib"),
            b"INPUT ( /tmp/libpython3.12.a )\n",
        );
        write(&deps.join("liby-1.rlib"), b"!<arch>\n");
        let policy = resolving(&root);
        let problems = policy.check(&[deps.join("libx-1.rlib").display().to_string()]);
        assert!(
            problems.iter().any(|p| p.contains("linker script")),
            "{problems:?}"
        );
        assert_eq!(
            policy.check(&[deps.join("liby-1.rlib").display().to_string()]),
            Vec::<String>::new()
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn r7_01_driver_search_dirs_and_scripts_are_parsed() {
        let dirs = parse_search_dirs(
            "install: /usr/lib/gcc/x86_64-linux-gnu/13/\nprograms: =/usr/bin\nlibraries: =/usr/lib/gcc/x86_64-linux-gnu/13/:relative:/lib/x86_64-linux-gnu/\n",
        );
        assert_eq!(dirs.len(), 2, "{dirs:?}");
        assert_eq!(
            script_inputs(
                "OUTPUT_FORMAT(elf64-x86-64)\nGROUP ( /a/libc.so.6 /a/x.a AS_NEEDED ( /a/ld.so ) )"
            ),
            Ok(strings(&["/a/libc.so.6", "/a/x.a", "/a/ld.so"]))
        );
        assert!(script_inputs("SECTIONS { }").is_err());
        assert!(script_inputs("GROUP ( a").is_err());
        assert!(script_inputs("/* open").is_err());
    }

    // R7-02: every member is also built alone, with and without its default
    // features; the production build is a mode of its own.
    #[test]
    fn r7_02_modes_separate_production_builds() {
        assert_eq!(Mode::Production.args(), ["build", "--keep-going"]);
        assert!(!Mode::Production.args().contains(&"--all-targets"));
        assert!(Mode::AllTargets.args().contains(&"--all-targets"));
        let set = FeatureSet::PackageDefault {
            package: "p".to_owned(),
            name: "p".to_owned(),
        };
        assert_eq!(set.args(), strings(&["--package", "p"]));
        let set = FeatureSet::PackageNoDefault {
            package: "p".to_owned(),
            name: "p".to_owned(),
        };
        assert_eq!(
            set.args(),
            strings(&["--package", "p", "--no-default-features"])
        );
    }

    #[test]
    fn cargo_messages_are_analysed() {
        let messages = [
            r#"{"reason":"build-script-executed","package_id":"path+file:///r/crates/engine#egs-engine@0.0.0","linked_libs":["dylib=python3.12","m"],"linked_paths":[],"env":[["GIT_HASH","x"],["XTASK_BUILD_GATE_CONFIG","/x"],["PATH","/evil"]]}"#,
            r#"{"reason":"compiler-message","package_id":"path+file:///r/crates/x#x@0.0.0","message":{"level":"error","message":"usage of an `unsafe` block","rendered":"error: usage of an `unsafe` block","spans":[{"is_primary":true,"file_name":"src/lib.rs","line_start":3}]}}"#,
            r#"{"reason":"compiler-message","package_id":"x","message":{"level":"warning","rendered":"w","spans":[]}}"#,
            "not json",
        ]
        .join("\n");
        let found = analyse(&messages);
        assert_eq!(found.len(), 4, "{found:?}");
        assert!(found[0].message.contains("egs-engine"));
        assert!(found[1].message.contains("XTASK_BUILD_GATE_CONFIG"));
        assert!(found[2].message.contains("`PATH`"));
        assert_eq!(found[3].line, Some(3));
        assert_eq!(found[3].message, "`x`: usage of an `unsafe` block");
        assert_eq!(package_name("path+file:///r/xtask#0.0.0"), "xtask");
        assert!(!is_protected_build_env("GIT_HASH"));
    }
}
