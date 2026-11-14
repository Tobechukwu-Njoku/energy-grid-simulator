//! Workflow hardening and gate graph (NFR-12, ADR 0001), on parsed YAML.
//!
//! Every workflow is parsed with a YAML 1.2 parser, so flow mappings such as
//! `permissions: {contents: write}` are seen (R6, ADV23-15). Checks:
//! allow-listed triggers only; explicit top-level permissions; every
//! permission read or none; hosted runner labels only; every action and
//! reusable workflow pinned to a full SHA; checkout never persists
//! credentials; `run:` scripts contain only allow-listed `${{ }}` expressions
//! (ADV23-16) and never invoke Python (D-73); no secrets but `GITHUB_TOKEN`;
//! no `CARGO_*`, `RUSTC*`, `RUSTDOC*`, `RUSTUP_*` or `*RUSTFLAGS` variable set
//! in any `env:` unless the trusted policy allow-lists it (ADV23R5-01); and the
//! required gate graph from the trusted policy is intact (R1).
//!
//! GitHub Actions file commands (ADV23R6-03). A step can change the
//! environment, `PATH`, outputs or state of LATER steps of its job through the
//! files named by `$GITHUB_ENV`, `$GITHUB_PATH`, `$GITHUB_OUTPUT`,
//! `$GITHUB_STATE` and `$GITHUB_STEP_SUMMARY`. The guarantee does not rest on
//! reading shell text, which cannot be decided:
//!
//! 1. A step that executes pull-request code under a trusted check (the
//!    `build-gate` step) must be the LAST step of its job, have no `id` and
//!    sit in a job without `outputs`. Whatever that code writes to a
//!    file-command file can only reach steps that run after it; there are none
//!    but post-steps of actions, and a later step can only fail the job, never
//!    turn the gate's failure into success. The gate also removes those
//!    variables from the environment it builds in. Every other step of every
//!    gate job runs trusted code from `main` before any pull-request code.
//! 2. Workflow and action files are protected paths: any change needs the
//!    owner's exact-head approval from the trusted `policy-approval` gate.
//! 3. As a lint over the reviewed text, a `run:` script may name a
//!    file-command file only on a line the trusted `allowed_file_command_lines`
//!    lists exactly (none by default), and may not use the indirect forms
//!    that reach a variable without naming it (`${!x}`, `eval`, `printenv`,
//!    environment listings, namerefs, `ENVIRON`, `process.env`, the runner's
//!    file-command directory). Names are compared case-insensitively after
//!    removing shell quoting.

use yaml_rust2::{Yaml, YamlLoader};

use crate::util::{Ctx, Finding, glob_match, string_array, tracked};

/// Trusted gate-graph and workflow policy.
pub const GATES: &str = "xtask/policy/gates.toml";

/// Workflow policy loaded from the trusted tree.
#[derive(Debug, Clone, Default)]
pub struct Policy {
    triggers: Vec<String>,
    expressions: Vec<String>,
    runners: Vec<String>,
    env: Vec<String>,
    file_command_lines: Vec<String>,
    required_checks: Vec<String>,
    graphs: Vec<Graph>,
}

#[derive(Debug, Clone, Default)]
struct Graph {
    file: String,
    jobs: Vec<String>,
    aggregator: Option<String>,
    /// Triggers allowed for this workflow instead of the global list.
    triggers: Option<Vec<String>>,
    /// Runs from `main` via `pull_request_target` and never runs PR code.
    trusted: bool,
}

impl Policy {
    /// Loads the policy.
    ///
    /// # Errors
    ///
    /// Fails if the trusted file is missing or malformed.
    pub fn load(ctx: &Ctx) -> Result<Self, String> {
        Self::parse(&ctx.policy_text(GATES)?)
    }

    /// Parses policy text.
    ///
    /// # Errors
    ///
    /// Fails on malformed TOML.
    pub fn parse(text: &str) -> Result<Self, String> {
        let document: toml_edit::DocumentMut =
            text.parse().map_err(|error| format!("{GATES}: {error}"))?;
        let mut graphs = Vec::new();
        if let Some(tables) = document
            .get("workflow")
            .and_then(toml_edit::Item::as_array_of_tables)
        {
            for table in tables {
                graphs.push(Graph {
                    file: table
                        .get("file")
                        .and_then(toml_edit::Item::as_str)
                        .ok_or(format!("{GATES}: workflow without file"))?
                        .to_owned(),
                    jobs: table.get("jobs").map(string_array).unwrap_or_default(),
                    aggregator: table
                        .get("aggregator")
                        .and_then(toml_edit::Item::as_str)
                        .map(str::to_owned),
                    triggers: table.get("triggers").map(string_array),
                    trusted: table
                        .get("trusted")
                        .and_then(toml_edit::Item::as_bool)
                        .unwrap_or(false),
                });
            }
        }
        Ok(Self {
            triggers: string_array(&document["allowed_triggers"]),
            expressions: string_array(&document["run_expressions"]),
            runners: string_array(&document["runner_labels"]),
            env: document
                .get("allowed_env")
                .map(string_array)
                .unwrap_or_default(),
            file_command_lines: document
                .get("allowed_file_command_lines")
                .map(string_array)
                .unwrap_or_default(),
            required_checks: document
                .get("required_checks")
                .map(string_array)
                .unwrap_or_default(),
            graphs,
        })
    }
}

impl Policy {
    /// Trusted (`pull_request_target`) workflow files and their gate jobs.
    #[must_use]
    pub fn trusted_gates(&self) -> (Vec<String>, Vec<String>) {
        let trusted: Vec<&Graph> = self.graphs.iter().filter(|graph| graph.trusted).collect();
        (
            trusted.iter().map(|graph| graph.file.clone()).collect(),
            trusted
                .iter()
                .flat_map(|graph| graph.jobs.iter().cloned())
                .collect(),
        )
    }

    /// Every check name the trusted policy defines, with the workflow file
    /// that owns it: each gate job and aggregator of every gate graph
    /// (`merge-gates` included) and each `required_checks` entry, which no
    /// file owns. No other job may use one of these as its id or name
    /// (ADV23R8-01).
    #[must_use]
    pub fn protected_gates(&self) -> Vec<ProtectedGate> {
        let mut gates: Vec<ProtectedGate> = self
            .graphs
            .iter()
            .flat_map(|graph| {
                graph
                    .jobs
                    .iter()
                    .chain(graph.aggregator.iter())
                    .map(|job| ProtectedGate {
                        file: Some(graph.file.clone()),
                        name: job.clone(),
                    })
            })
            .collect();
        for name in &self.required_checks {
            if !gates
                .iter()
                .any(|gate| gate.name.eq_ignore_ascii_case(name))
            {
                gates.push(ProtectedGate {
                    file: None,
                    name: name.clone(),
                });
            }
        }
        gates
    }
}

/// A check name defined by the trusted policy (ADV23R8-01).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtectedGate {
    /// The workflow file whose job of the same id owns the name; `None` when
    /// no workflow may define it.
    pub file: Option<String>,
    /// Job id and check name.
    pub name: String,
}

fn get<'a>(yaml: &'a Yaml, key: &str) -> Option<&'a Yaml> {
    yaml.as_hash()?.get(&Yaml::String(key.to_owned()))
}

fn entries(yaml: &Yaml) -> Vec<(String, &Yaml)> {
    yaml.as_hash()
        .map(|hash| {
            hash.iter()
                .map(|(key, value)| (scalar(key), value))
                .collect()
        })
        .unwrap_or_default()
}

fn scalar(yaml: &Yaml) -> String {
    match yaml {
        Yaml::String(text) | Yaml::Real(text) => text.clone(),
        Yaml::Integer(value) => value.to_string(),
        Yaml::Boolean(value) => value.to_string(),
        Yaml::Null => "null".to_owned(),
        _ => String::new(),
    }
}

fn strings(yaml: &Yaml, out: &mut Vec<String>) {
    match yaml {
        Yaml::Array(items) => items.iter().for_each(|item| strings(item, out)),
        Yaml::Hash(hash) => hash.iter().for_each(|(key, value)| {
            strings(key, out);
            strings(value, out);
        }),
        Yaml::Alias(_) | Yaml::BadValue | Yaml::Null => {}
        other => out.push(scalar(other)),
    }
}

fn check_permissions(path: &str, scope: &str, yaml: &Yaml, findings: &mut Vec<Finding>) {
    match yaml {
        Yaml::Hash(_) => {
            for (name, level) in entries(yaml) {
                let level = scalar(level);
                if level != "read" && level != "none" {
                    findings.push(Finding::at(
                        path,
                        None,
                        format!("{scope}: permission `{name}: {level}` is not read-only (NFR-12)"),
                    ));
                }
            }
        }
        Yaml::String(text) if text == "read-all" || text == "{}" => {}
        other => findings.push(Finding::at(
            path,
            None,
            format!(
                "{scope}: permissions `{}` is not read-only (NFR-12)",
                scalar(other)
            ),
        )),
    }
}

/// Whether a `uses:` reference is pinned. A local reference is accepted only
/// under `./.github/actions/` (steps) or `./.github/workflows/` (reusable
/// workflows), without `..`, where every action file is checked (ADV23R4-03).
fn pinned(action: &str) -> bool {
    if let Some(local) = action.strip_prefix("./") {
        return (local.starts_with(".github/actions/") || local.starts_with(".github/workflows/"))
            && !local.split('/').any(|part| part == ".." || part == ".");
    }
    if let Some(image) = action.strip_prefix("docker://") {
        return image.contains("@sha256:");
    }
    let Some((name, reference)) = action.rsplit_once('@') else {
        return false;
    };
    let mut parts = name.split('/');
    let owner_ok = parts.next().is_some_and(|p| !p.is_empty());
    let repo_ok = parts.next().is_some_and(|p| !p.is_empty());
    owner_ok
        && repo_ok
        && reference.len() == 40
        && reference
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

fn expressions(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("${{") {
        let after = &rest[start + 3..];
        let end = after.find("}}").unwrap_or(after.len());
        found.push(after[..end].trim().to_owned());
        rest = &after[end.min(after.len())..];
    }
    found
}

/// Interpreter command families CI may not run (D-73: Rust for tooling, CI
/// and tests; NFR-10: no Python or Julia runtime). A family matches its
/// versioned and variant spellings (`python3.12`, `pythonw`, `pypy3`,
/// `julia-1.11`, `python.exe`), whatever directory it is called from.
const INTERPRETERS: &[&str] = &[
    "python",
    "pypy",
    "ipython",
    "jython",
    "graalpy",
    "py",
    "pip",
    "pipx",
    "pipenv",
    "conda",
    "mamba",
    "micromamba",
    "uv",
    "uvx",
    "poetry",
    "pdm",
    "hatch",
    "tox",
    "nox",
    "pytest",
    "jupyter",
    "julia",
    "juliaup",
    "node",
    "nodejs",
    "npx",
    "deno",
    "bun",
    "bunx",
    "perl",
    "npm",
    "pnpm",
    "yarn",
    "ruby",
    "php",
    "lua",
    "luajit",
    "rscript",
    "tclsh",
    "osascript",
];

/// Script files for those interpreters.
const INTERPRETER_SCRIPTS: &[&str] = &[
    ".py", ".pyw", ".pyz", ".pyc", ".ipynb", ".jl", ".pl", ".pm", ".rb", ".php", ".lua", ".js",
    ".mjs", ".cjs", ".ts", ".mts", ".r", ".tcl",
];

/// Whether one command word names a forbidden interpreter: a family name,
/// then only a version (`3`, `3.12`, `-1.11`), an ABI/variant letter
/// (`w`, `t`, `m`, `d`) and an `.exe` suffix (ADV23R7-03).
fn interpreter_word(word: &str) -> bool {
    let lower = word.to_ascii_lowercase();
    let lower = lower.strip_suffix(".exe").unwrap_or(&lower);
    crate::util::RUNTIME_FAMILIES
        .iter()
        .chain(INTERPRETERS)
        .any(|family| {
            let Some(rest) = lower.strip_prefix(family) else {
                return false;
            };
            let rest = rest.trim_start_matches(['-', '_']);
            let rest = rest.trim_start_matches(|c: char| c.is_ascii_digit() || c == '.');
            matches!(rest, "" | "w" | "t" | "m" | "d" | "dm" | "mu")
        })
        || INTERPRETER_SCRIPTS
            .iter()
            .any(|ext| lower.len() > ext.len() && lower.ends_with(ext))
}

/// Whether a script runs an interpreter other than the allowed shells.
#[must_use]
pub fn invokes_interpreter(script: &str) -> bool {
    script
        .split(|c: char| !(c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-')))
        .flat_map(|word| {
            // `python3.12-config` and `julia-1.11` both reach the check: the
            // whole word and its `-`-separated pieces.
            std::iter::once(word).chain(word.split('-'))
        })
        .any(interpreter_word)
}

/// Shells a `run:` step may use. Any other value, including a custom
/// `interpreter {0}` template, runs the script under another interpreter
/// (Python, Node, Perl, ...), so shells are allowed by name rather than
/// banned by name (D-73, ADV23R3-05).
const ALLOWED_SHELLS: &[&str] = &["bash", "sh", "pwsh", "powershell", "cmd"];

/// Checks a `shell` value wherever it is set: on a step, or inherited through
/// `defaults.run.shell` of a job or the workflow (ADV23R3-05).
fn check_shell(path: &str, scope: &str, shell: &Yaml, findings: &mut Vec<Finding>) {
    let value = scalar(shell);
    if !ALLOWED_SHELLS.contains(&value.trim()) {
        findings.push(Finding::at(
            path,
            None,
            format!(
                "{scope}: shell `{value}` is not allowed; use one of {ALLOWED_SHELLS:?} (D-73)"
            ),
        ));
    }
}

/// Checks a `defaults:` mapping of a workflow or job.
fn check_defaults(path: &str, scope: &str, holder: &Yaml, findings: &mut Vec<Finding>) {
    if let Some(shell) = get(holder, "defaults")
        .and_then(|defaults| get(defaults, "run"))
        .and_then(|run| get(run, "shell"))
    {
        check_shell(path, &format!("{scope} defaults.run"), shell, findings);
    }
}

/// Checks one parsed workflow.
#[must_use]
pub fn check_workflow(path: &str, text: &str, policy: &Policy) -> Vec<Finding> {
    let mut findings = Vec::new();
    let documents = match YamlLoader::load_from_str(text) {
        Ok(documents) => documents,
        Err(error) => return vec![Finding::at(path, None, format!("invalid YAML: {error}"))],
    };
    let [workflow] = documents.as_slice() else {
        return vec![Finding::at(
            path,
            None,
            "expected exactly one YAML document",
        )];
    };
    if workflow.as_hash().is_none() {
        return vec![Finding::at(path, None, "workflow is not a mapping")];
    }
    let mut all = Vec::new();
    strings(workflow, &mut all);
    if has_alias(workflow) {
        findings.push(Finding::at(
            path,
            None,
            "YAML aliases are not allowed in workflows",
        ));
    }

    let triggers: Vec<String> = match get(workflow, "on") {
        Some(Yaml::String(name)) => vec![name.clone()],
        Some(Yaml::Array(items)) => items.iter().map(scalar).collect(),
        Some(map @ Yaml::Hash(_)) => entries(map).into_iter().map(|(key, _)| key).collect(),
        _ => {
            findings.push(Finding::at(path, None, "missing or invalid `on:` triggers"));
            Vec::new()
        }
    };
    let graph = policy.graphs.iter().find(|graph| graph.file == path);
    let allowed = graph
        .and_then(|graph| graph.triggers.as_ref())
        .unwrap_or(&policy.triggers);
    let trusted = graph.is_some_and(|graph| graph.trusted);
    for trigger in &triggers {
        if trigger == "pull_request_target" && !trusted {
            findings.push(Finding::at(
                path,
                None,
                "`pull_request_target` is allowed only in a trusted-gates workflow listed in the trusted gate graph",
            ));
            continue;
        }
        if !allowed.contains(trigger) {
            findings.push(Finding::at(
                path,
                None,
                format!("trigger `{trigger}` is not allowed (NFR-12)"),
            ));
        }
    }

    match get(workflow, "permissions") {
        Some(permissions) => check_permissions(path, "workflow", permissions, &mut findings),
        None => findings.push(Finding::at(
            path,
            None,
            "needs an explicit top-level `permissions:` block",
        )),
    }

    for (name, value) in entries(get(workflow, "env").unwrap_or(&Yaml::Null)) {
        check_env(path, &name, value, policy, &mut findings);
    }
    check_defaults(path, "workflow", workflow, &mut findings);

    let jobs = get(workflow, "jobs");
    let job_entries = jobs.map(entries).unwrap_or_default();
    if job_entries.is_empty() {
        findings.push(Finding::at(path, None, "workflow has no jobs"));
    }
    for (job_name, job) in &job_entries {
        check_job(path, job_name, job, policy, &mut findings);
        if triggers.iter().any(|t| t == "pull_request_target") {
            check_trusted_job(path, job_name, job, &mut findings);
        }
    }

    for value in &all {
        let mut rest = value.as_str();
        while let Some(index) = rest.find("secrets.") {
            let name: String = rest[index + 8..]
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            if name != "GITHUB_TOKEN" {
                findings.push(Finding::at(
                    path,
                    None,
                    format!("secret `{name}` is not allowed in hosted CI (NFR-12)"),
                ));
            }
            rest = &rest[index + 8..];
        }
        if value.contains("secrets[") {
            findings.push(Finding::at(
                path,
                None,
                "dynamic secret access is not allowed",
            ));
        }
    }

    check_graph(path, &job_entries, policy, &mut findings);
    findings
}

/// Extra rules for a `pull_request_target` job, which runs with the base
/// branch's context: it must never check out, build or run pull-request code
/// (F1). Checkout takes no `ref` or `repository`, no local action is used
/// (it would be read from the PR checkout in other workflows) and no cache is
/// written.
fn check_trusted_job(path: &str, name: &str, job: &Yaml, findings: &mut Vec<Finding>) {
    let scope = format!("job `{name}` (pull_request_target)");
    if get(job, "uses").is_some() {
        findings.push(Finding::at(
            path,
            None,
            format!("{scope}: reusable workflows are not allowed"),
        ));
    }
    let steps = get(job, "steps")
        .and_then(Yaml::as_vec)
        .cloned()
        .unwrap_or_default();
    for (index, step) in steps.iter().enumerate() {
        let action = get(step, "uses").map(scalar).unwrap_or_default();
        if action.starts_with("./")
            || action.contains("/cache@")
            || action.starts_with("actions/cache")
        {
            findings.push(Finding::at(
                path,
                None,
                format!(
                    "{scope} step {}: `{action}` is not allowed in a trusted job",
                    index + 1
                ),
            ));
        }
        if action.starts_with("actions/checkout@")
            && let Some(with) = get(step, "with")
        {
            for key in ["ref", "repository"] {
                if get(with, key).is_some() {
                    findings.push(Finding::at(
                        path,
                        None,
                        format!(
                            "{scope} step {}: checkout `{key}` is not allowed; the PR head is fetched only as data",
                            index + 1
                        ),
                    ));
                }
            }
        }
    }
}

fn has_alias(yaml: &Yaml) -> bool {
    match yaml {
        Yaml::Alias(_) => true,
        Yaml::Array(items) => items.iter().any(has_alias),
        Yaml::Hash(hash) => hash.iter().any(|(k, v)| has_alias(k) || has_alias(v)),
        _ => false,
    }
}

/// Whether a variable name can change the toolchain, compiler flags, rustc
/// wrappers, linkers or lint caps (ADV23R5-01).
#[must_use]
pub fn is_toolchain_env(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    upper == "CARGO"
        || upper.starts_with("CARGO_")
        || upper.starts_with("RUSTC")
        || upper.starts_with("RUSTDOC")
        || upper.starts_with("RUSTUP_")
        || upper.contains("RUSTFLAGS")
}

/// Whether a variable steers the dynamic loader (`LD_PRELOAD`,
/// `LD_LIBRARY_PATH`, `DYLD_INSERT_LIBRARIES`, ...), which could inject code
/// into every process of a step (ADV23R8-02).
#[must_use]
pub fn is_loader_env(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    upper.starts_with("LD_") || upper.starts_with("DYLD_")
}

fn check_env(path: &str, name: &str, _value: &Yaml, policy: &Policy, findings: &mut Vec<Finding>) {
    if is_loader_env(name) {
        findings.push(Finding::at(
            path,
            None,
            format!(
                "`{name}` may not be set in workflows: dynamic-loader variables can inject code into every process (ADV23R8-02)"
            ),
        ));
        return;
    }
    if is_toolchain_env(name) && !policy.env.iter().any(|allowed| allowed == name) {
        findings.push(Finding::at(
            path,
            None,
            format!(
                "`{name}` may not be set in workflows unless allowed_env in the trusted policy lists it (compiler flags, wrappers and lint levels stay out of CI)"
            ),
        ));
    }
}

fn check_job(path: &str, name: &str, job: &Yaml, policy: &Policy, findings: &mut Vec<Finding>) {
    let scope = format!("job `{name}`");
    if let Some(permissions) = get(job, "permissions") {
        check_permissions(path, &scope, permissions, findings);
    }
    if let Some(uses) = get(job, "uses") {
        let action = scalar(uses);
        if !pinned(&action) {
            findings.push(Finding::at(
                path,
                None,
                format!("{scope}: `{action}` must be pinned to a full commit SHA, or be local under ./.github/"),
            ));
        }
    }
    if matches!(get(job, "secrets"), Some(Yaml::String(text)) if text == "inherit") {
        findings.push(Finding::at(
            path,
            None,
            format!("{scope}: `secrets: inherit` is not allowed"),
        ));
    }
    match get(job, "runs-on") {
        Some(Yaml::String(label)) if policy.runners.iter().any(|r| glob_match(r, label)) => {}
        Some(other) => findings.push(Finding::at(
            path,
            None,
            format!(
                "{scope}: runner `{}` is not an allowed hosted label (self-hosted waits for M1)",
                scalar(other)
            ),
        )),
        None if get(job, "uses").is_some() => {}
        None => findings.push(Finding::at(path, None, format!("{scope}: missing runs-on"))),
    }
    for (key, value) in entries(get(job, "env").unwrap_or(&Yaml::Null)) {
        check_env(path, &key, value, policy, findings);
    }
    check_defaults(path, &scope, job, findings);
    for key in ["container", "services"] {
        if get(job, key).is_some() {
            findings.push(Finding::at(
                path,
                None,
                format!("{scope}: `{key}` needs a reviewed policy exception"),
            ));
        }
    }
    let steps = get(job, "steps")
        .and_then(Yaml::as_vec)
        .cloned()
        .unwrap_or_default();
    for (index, step) in steps.iter().enumerate() {
        let step_scope = format!("{scope} step {}", index + 1);
        for (key, value) in entries(get(step, "env").unwrap_or(&Yaml::Null)) {
            check_env(path, &key, value, policy, findings);
        }
        if let Some(uses) = get(step, "uses") {
            let action = scalar(uses);
            if !pinned(&action) {
                findings.push(Finding::at(
                    path,
                    None,
                    format!("{step_scope}: `{action}` must be pinned to a full commit SHA, or be local under ./.github/actions/"),
                ));
            }
            if action.starts_with("actions/checkout@") {
                let persist = get(step, "with").and_then(|with| get(with, "persist-credentials"));
                let off = matches!(persist, Some(Yaml::Boolean(false)))
                    || matches!(persist, Some(Yaml::String(text)) if text == "false");
                if !off {
                    findings.push(Finding::at(
                        path,
                        None,
                        format!(
                            "{step_scope}: actions/checkout must set `persist-credentials: false`"
                        ),
                    ));
                }
            }
        }
        if let Some(run) = get(step, "run") {
            let script = scalar(run);
            for expression in expressions(&script) {
                let allowed = policy
                    .expressions
                    .iter()
                    .any(|pattern| glob_match(pattern, &expression));
                if !allowed {
                    findings.push(Finding::at(
                        path,
                        None,
                        format!(
                            "{step_scope}: `${{{{ {expression} }}}}` inside run:; pass values through env:"
                        ),
                    ));
                }
            }
            for problem in file_command_problems(&script, &policy.file_command_lines) {
                findings.push(Finding::at(
                    path,
                    None,
                    format!("{step_scope}: run: {problem} (ADV23R6-03); set env: per step"),
                ));
            }
            if runs_untrusted_build(&script) {
                if index + 1 != steps.len() {
                    findings.push(Finding::at(
                        path,
                        None,
                        format!(
                            "{step_scope}: the build-gate step runs pull-request code and must be the last step of its job, so file-command writes reach no later step (ADV23R6-03)"
                        ),
                    ));
                }
                if get(step, "id").is_some() || get(job, "outputs").is_some() {
                    findings.push(Finding::at(
                        path,
                        None,
                        format!(
                            "{step_scope}: the build-gate step may not have an id and its job may not declare outputs (ADV23R6-03)"
                        ),
                    ));
                }
            }
            if invokes_interpreter(&script) {
                findings.push(Finding::at(
                    path,
                    None,
                    format!(
                        "{step_scope}: CI must not run Python, Julia or another script interpreter (D-73, NFR-10); use cargo xtask"
                    ),
                ));
            }
        }
        if let Some(shell) = get(step, "shell") {
            check_shell(path, &step_scope, shell, findings);
        }
    }
}

/// File-command variable names (lower case) and the runner's file names.
const FILE_COMMANDS: &[&str] = &[
    "github_env",
    "github_path",
    "github_output",
    "github_state",
    "github_step_summary",
    "runner_file_commands",
    "set_env_",
    "add_path_",
    "set_output_",
    "save_state_",
    "step_summary_",
];

/// Shell and interpreter forms that reach a variable or a file-command file
/// without naming it.
const INDIRECT: &[&str] = &[
    r"\$\{!",
    r"(?:^|[^\w-])eval(?:$|[^\w-])",
    r"(?:^|[^\w-])printenv(?:$|[^\w-])",
    r"(?:^|[^\w-])compgen(?:$|[^\w-])",
    r"(?:^|[^\w-])envsubst(?:$|[^\w-])",
    r"(?:^|[^\w-])(?:declare|local|typeset|readonly)\s+-\w*[np]",
    r"(?:^|[^\w-])export\s+-p",
    r"(?:^|[^\w$-])(?:env|set)\s*(?:$|[|>;&)])",
    r"\benviron\b",
    r"process\.env",
    r"%env\b|\$env\{|\benv\[",
];

/// Problems with one `run:` script's use of file-command files.
#[must_use]
pub fn file_command_problems(script: &str, allowed: &[String]) -> Vec<String> {
    let indirect: Vec<fancy_regex::Regex> = INDIRECT
        .iter()
        .filter_map(|pattern| fancy_regex::Regex::new(&format!("(?i){pattern}")).ok())
        .collect();
    let mut problems = Vec::new();
    for line in script.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }
        // Remove shell quoting so `GITHU""B_ENV` and `GITHU\B_ENV` read as
        // the name they spell.
        let plain: String = trimmed
            .chars()
            .filter(|c| !matches!(c, '"' | '\'' | '\\' | '`'))
            .collect::<String>()
            .to_lowercase();
        let names = FILE_COMMANDS.iter().any(|name| plain.contains(name));
        if names && !allowed.iter().any(|line| line.trim() == trimmed) {
            problems.push(format!(
                "may not name a GitHub file-command file outside allowed_file_command_lines: `{trimmed}`"
            ));
        }
        if let Some(pattern) = indirect
            .iter()
            .find(|pattern| pattern.is_match(&plain).unwrap_or(true))
        {
            problems.push(format!(
                "may not reach environment variables indirectly (`{}`): `{trimmed}`",
                pattern.as_str().trim_start_matches("(?i)")
            ));
        }
    }
    problems
}

/// Whether a `run:` script runs the build gate, which executes pull-request
/// build scripts, proc macros and doctests.
#[must_use]
pub fn runs_untrusted_build(script: &str) -> bool {
    script.contains("build-gate")
}

fn check_graph(path: &str, jobs: &[(String, &Yaml)], policy: &Policy, findings: &mut Vec<Finding>) {
    let Some(graph) = policy.graphs.iter().find(|graph| graph.file == path) else {
        return;
    };
    let names: Vec<&str> = jobs.iter().map(|(name, _)| name.as_str()).collect();
    for required in &graph.jobs {
        if !names.contains(&required.as_str()) {
            findings.push(Finding::at(
                path,
                None,
                format!("required gate job `{required}` is missing (trusted gate graph)"),
            ));
        }
        if let Some((_, job)) = jobs.iter().find(|(name, _)| name == required) {
            if get(job, "if").is_some() {
                findings.push(Finding::at(
                    path,
                    None,
                    format!("required gate job `{required}` may not be conditional"),
                ));
            }
            if matches!(get(job, "continue-on-error"), Some(value) if scalar(value) != "false") {
                findings.push(Finding::at(
                    path,
                    None,
                    format!("required gate job `{required}` may not continue on error"),
                ));
            }
        }
    }
    let Some(aggregator) = &graph.aggregator else {
        return;
    };
    let Some((_, job)) = jobs.iter().find(|(name, _)| name == aggregator) else {
        findings.push(Finding::at(
            path,
            None,
            format!("aggregator job `{aggregator}` is missing"),
        ));
        return;
    };
    let needs: Vec<String> = match get(job, "needs") {
        Some(Yaml::Array(items)) => items.iter().map(scalar).collect(),
        Some(single) => vec![scalar(single)],
        None => Vec::new(),
    };
    for required in &graph.jobs {
        if required != aggregator && !needs.contains(required) {
            findings.push(Finding::at(
                path,
                None,
                format!("`{aggregator}` must need required gate `{required}`"),
            ));
        }
    }
    if get(job, "if").map(scalar).as_deref() != Some("always()") {
        findings.push(Finding::at(
            path,
            None,
            format!("`{aggregator}` must run with `if: always()`"),
        ));
    }
}

/// Runs the workflow checks over every tracked workflow.
///
/// # Errors
///
/// Fails if Git, file reads or the trusted policy fail.
pub fn check(ctx: &Ctx) -> Result<Vec<Finding>, String> {
    let policy = Policy::load(ctx)?;
    let mut findings = Vec::new();
    let files: Vec<String> = tracked(&ctx.repo)?
        .into_iter()
        .filter(|path| {
            path.starts_with(".github/workflows/")
                && (path.ends_with(".yml") || path.ends_with(".yaml"))
        })
        .collect();
    if files.is_empty() {
        findings.push(Finding::general(".github/workflows: no workflows found"));
    }
    for graph in &policy.graphs {
        if !files.contains(&graph.file) {
            findings.push(Finding::at(
                &graph.file,
                None,
                "required workflow is missing (trusted gate graph)",
            ));
        }
    }
    let mut texts = Vec::new();
    for path in &files {
        let text = std::fs::read_to_string(ctx.repo.join(path))
            .map_err(|error| format!("{path}: {error}"))?;
        findings.extend(check_workflow(path, &text, &policy));
        texts.push((path.clone(), text));
    }
    // ADV23R8-01: no job anywhere may take a check name the policy defines,
    // except the owning workflow's job of that id.
    findings.extend(crate::review_gates::spoofed_gate_names(
        &texts,
        &policy.protected_gates(),
    ));
    // Every action definition anywhere in the repository is checked, not
    // only those under .github/actions/ (ADV23R4-03).
    for path in tracked(&ctx.repo)? {
        if is_action_file(&path) {
            let text = std::fs::read_to_string(ctx.repo.join(&path))
                .map_err(|error| format!("{path}: {error}"))?;
            findings.extend(check_action(&path, &text, &policy));
        }
    }
    Ok(findings)
}

/// Whether a tracked path is a GitHub action definition (`action.yml` or
/// `action.yaml`, any directory, any letter case).
#[must_use]
pub fn is_action_file(path: &str) -> bool {
    let name = path.rsplit('/').next().unwrap_or(path).to_ascii_lowercase();
    name == "action.yml" || name == "action.yaml"
}

/// Checks a local composite action: pinned `uses`, no Python, allow-listed
/// expressions in `run:` and no secrets.
#[must_use]
pub fn check_action(path: &str, text: &str, policy: &Policy) -> Vec<Finding> {
    let documents = match YamlLoader::load_from_str(text) {
        Ok(documents) => documents,
        Err(error) => return vec![Finding::at(path, None, format!("invalid YAML: {error}"))],
    };
    let Some(action) = documents.first() else {
        return vec![Finding::at(path, None, "empty action")];
    };
    let runs = get(action, "runs").unwrap_or(&Yaml::Null);
    if get(runs, "using").map(scalar).as_deref() != Some("composite") {
        return vec![Finding::at(
            path,
            None,
            "only composite local actions are allowed",
        )];
    }
    let mut job = yaml_rust2::yaml::Hash::new();
    job.insert(
        Yaml::String("runs-on".into()),
        Yaml::String("ubuntu-24.04".into()),
    );
    job.insert(
        Yaml::String("steps".into()),
        get(runs, "steps")
            .cloned()
            .unwrap_or(Yaml::Array(Vec::new())),
    );
    let mut findings = Vec::new();
    check_job(path, "composite", &Yaml::Hash(job), policy, &mut findings);
    let mut all = Vec::new();
    strings(action, &mut all);
    if all
        .iter()
        .any(|value| value.contains("secrets.") || value.contains("secrets["))
    {
        findings.push(Finding::at(
            path,
            None,
            "secrets are not allowed in local actions",
        ));
    }
    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    const POLICY: &str = r#"
allowed_triggers = ["pull_request", "push"]
run_expressions = ["runner.temp"]
runner_labels = ["ubuntu-*", "macos-*"]
allowed_env = ["CARGO_TERM_COLOR"]
[[workflow]]
file = "w.yml"
jobs = ["a", "b"]
aggregator = "gate"
[[workflow]]
file = "t.yml"
jobs = ["review"]
triggers = ["pull_request_target"]
trusted = true
"#;

    const GOOD: &str = r#"
on:
  pull_request:
  push: {branches: [main]}
permissions:
  contents: read
jobs:
  a:
    runs-on: ubuntu-24.04
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1
        with: {persist-credentials: false}
      - run: echo "$HOME ${{ runner.temp }}"
  b:
    runs-on: macos-15
    steps:
      - run: cargo xtask docs
  gate:
    if: always()
    needs: [a, b]
    runs-on: ubuntu-24.04
    steps:
      - run: echo ok
"#;

    fn run(text: &str) -> Vec<Finding> {
        check_workflow("w.yml", text, &Policy::parse(POLICY).expect("policy"))
    }

    #[test]
    fn hardened_workflow_passes() {
        assert_eq!(run(GOOD), Vec::new());
    }

    // R6 / ADV23-15: flow-mapping write permissions.
    #[test]
    fn r6_adv23_15_flow_mapping_write_is_rejected() {
        let flow = GOOD.replace(
            "permissions:\n  contents: read",
            "permissions: {contents: write}",
        );
        assert_eq!(run(&flow).len(), 1, "{:?}", run(&flow));
        let all = GOOD.replace("permissions:\n  contents: read", "permissions: write-all");
        assert_eq!(run(&all).len(), 1);
        let job = GOOD.replace(
            "  b:\n    runs-on: macos-15",
            "  b:\n    permissions: { pull-requests: write }\n    runs-on: macos-15",
        );
        assert_eq!(run(&job).len(), 1);
        let missing = GOOD.replace("permissions:\n  contents: read\n", "");
        assert_eq!(run(&missing).len(), 1);
    }

    #[test]
    fn forbidden_triggers_and_runners() {
        let target = GOOD.replace("  pull_request:\n", "  pull_request_target:\n");
        assert!(!run(&target).is_empty());
        let list = GOOD.replace(
            "on:\n  pull_request:\n  push: {branches: [main]}",
            "on: [push, workflow_run]",
        );
        assert!(!run(&list).is_empty());
        let hosted = GOOD.replace("runs-on: macos-15", "runs-on: [self-hosted, linux]");
        assert!(!run(&hosted).is_empty());
        let dynamic = GOOD.replace("runs-on: macos-15", "runs-on: ${{ github.event.inputs.r }}");
        assert!(!run(&dynamic).is_empty());
    }

    #[test]
    fn pins_and_checkout() {
        let tag = GOOD.replace(
            "actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1",
            "actions/checkout@v4",
        );
        assert!(!run(&tag).is_empty());
        let persist = GOOD.replace(
            "with: {persist-credentials: false}",
            "with: {fetch-depth: 0}",
        );
        assert_eq!(run(&persist).len(), 1);
    }

    // ADV23-16: every non-allow-listed expression in run:, including
    // client_payload, inputs, actor and multiline forms.
    #[test]
    fn adv23_16_untrusted_expressions_in_run_are_rejected() {
        for expression in [
            "github.event.client_payload.x",
            "github.event.inputs.x",
            "github.actor",
            "github.triggering_actor",
            "github.event.sender.login",
            "github.head_ref",
            "\n github.event.pull_request.title\n",
        ] {
            let text = GOOD.replace("${{ runner.temp }}", &format!("${{{{{expression}}}}}"));
            assert_eq!(run(&text).len(), 1, "{expression}");
        }
        let block = GOOD.replace(
            "      - run: echo \"$HOME ${{ runner.temp }}\"",
            "      - run: |\n          echo start\n          echo ${{\n            github.event.pull_request.body }}",
        );
        assert_eq!(run(&block).len(), 1);
    }

    #[test]
    fn secrets_python_and_rustflags_are_rejected() {
        let secret = GOOD.replace("${{ runner.temp }}", "").replace(
            "run: cargo xtask docs",
            "run: cargo xtask docs\n        env: {T: \"${{ secrets.LICENCE }}\"}",
        );
        assert!(!run(&secret).is_empty());
        let python = GOOD.replace("run: cargo xtask docs", "run: python3 - <<'PY'");
        assert!(!run(&python).is_empty());
        let script = GOOD.replace("run: cargo xtask docs", "run: ./ci/check.py");
        assert!(!run(&script).is_empty());
        let flags = GOOD.replace(
            "permissions:",
            "env: {RUSTFLAGS: \"-A unsafe_code\"}\npermissions:",
        );
        assert!(!run(&flags).is_empty());
    }

    // ADV23R5-01: every Cargo/rustc variable is rejected unless the trusted
    // policy allow-lists it, at workflow, job and step level, and scripts
    // cannot set variables for later steps.
    #[test]
    fn adv23r5_01_toolchain_env_is_allow_listed() {
        for name in [
            "RUSTC_WRAPPER",
            "RUSTC_WORKSPACE_WRAPPER",
            "CARGO_BUILD_RUSTC_WRAPPER",
            "CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER",
            "CARGO_BUILD_RUSTFLAGS",
            "RUSTFLAGS",
            "CARGO_ENCODED_RUSTFLAGS",
            "CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER",
            "RUSTC",
            "RUSTC_BOOTSTRAP",
            "RUSTDOCFLAGS",
            "RUSTUP_TOOLCHAIN",
            "CARGO",
        ] {
            for text in [
                GOOD.replace(
                    "permissions:",
                    &format!("env: {{{name}: \"\"}}\npermissions:"),
                ),
                GOOD.replace(
                    "    runs-on: macos-15\n",
                    &format!("    runs-on: macos-15\n    env: {{{name}: x}}\n"),
                ),
                GOOD.replace(
                    "run: cargo xtask docs",
                    &format!(
                        "run: cargo xtask docs\n        env: {{{name}: /usr/local/bin/wrapper}}"
                    ),
                ),
            ] {
                let found = run(&text);
                assert_eq!(found.len(), 1, "{name}: {found:?}");
                assert!(found[0].message.contains(name));
            }
        }
        let allowed = GOOD.replace(
            "permissions:",
            "env: {CARGO_TERM_COLOR: always, NEXTEST_VERSION: x}\npermissions:",
        );
        assert!(run(&allowed).is_empty());
        for script in [
            "echo \"RUSTC_WRAPPER=/w\" >> \"$GITHUB_ENV\"",
            "echo /evil >> $GITHUB_PATH",
        ] {
            let text = GOOD.replace("run: cargo xtask docs", &format!("run: {script}"));
            assert_eq!(run(&text).len(), 1, "{script}");
        }
    }

    // ADV23R6-03: dynamic references to file-command files, and the
    // structural rule that the untrusted build step is its job's last step.
    #[test]
    fn adv23r6_03_file_commands_are_unreachable_from_scripts() {
        for script in [
            "echo \"RUSTFLAGS=-Aunsafe_code\" >> $(env | grep ^GITHUB_E | cut -d= -f2)",
            "target=GITHU; target+=B_ENV; echo x >> \"${!target}\"",
            "eval \"echo RUSTFLAGS=-Aunsafe_code >> \\$GITHU\"B_ENV",
            "echo x >> \"$GITHU\"\"B_ENV\"",
            "echo x >> $github_output",
            "echo x >> \"$GITHUB_STEP_SUMMARY\"",
            "printenv GITHUB_E",
            "declare -n ref=GITHUB_PATH_X",
            "f=$(ls \"$RUNNER_TEMP\"/_runner_file_commands/*)",
            "awk 'BEGIN { print ENVIRON[\"GITHU\" \"B_ENV\"] }'",
            "node -e 'console.log(process.env.X)'",
            "perl -e 'print $ENV{X}'",
            "set > /tmp/vars",
        ] {
            let problems = file_command_problems(script, &[]);
            assert!(!problems.is_empty(), "{script}");
        }
        for script in [
            "cargo xtask docs",
            "env RUST_LOG=info cargo test",
            "cd \"$TRUSTED\" && cargo run -- policy",
            "set -euo pipefail",
            "echo \"$NEEDS\" | jq .",
        ] {
            assert_eq!(
                file_command_problems(script, &[]),
                Vec::<String>::new(),
                "{script}"
            );
        }
        let allowed = vec!["echo \"dir=$x\" >> \"$GITHUB_OUTPUT\"".to_owned()];
        assert!(
            file_command_problems("echo \"dir=$x\" >> \"$GITHUB_OUTPUT\"", &allowed).is_empty()
        );
        // The build-gate step must be last, without id or job outputs.
        let gate = "run: cargo xtask docs\n      - run: cargo xtask build-gate";
        let last = GOOD.replace("run: cargo xtask docs", gate);
        assert!(run(&last).is_empty(), "{:?}", run(&last));
        let not_last = GOOD.replace(
            "run: cargo xtask docs",
            "run: cargo xtask build-gate\n      - run: cargo xtask docs",
        );
        assert!(
            run(&not_last)
                .iter()
                .any(|f| f.message.contains("last step"))
        );
        let with_id = GOOD.replace(
            "run: cargo xtask docs",
            "run: cargo xtask docs\n      - id: gate\n        run: cargo xtask build-gate",
        );
        assert!(
            run(&with_id)
                .iter()
                .any(|f| f.message.contains("may not have an id"))
        );
    }

    // ADV23R4-03: actions outside .github/actions/ are checked, and local
    // references must point under .github/.
    #[test]
    fn adv23r4_03_actions_anywhere_are_checked() {
        let policy = Policy::parse(POLICY).expect("policy");
        assert!(is_action_file("actions/helper/action.yml"));
        assert!(is_action_file("tools/ci/action.yaml"));
        assert!(is_action_file("x/Action.YML"));
        assert!(!is_action_file("x/myaction.yml"));
        let action = "name: helper\nruns:\n  using: composite\n  steps:\n    - shell: python\n      run: print(1)\n";
        assert!(!check_action("actions/helper/action.yml", action, &policy).is_empty());
        for (uses, ok) in [
            ("./.github/actions/trusted-policy", true),
            ("./actions/helper", false),
            ("./tools/ci", false),
            ("./.github/actions/../../actions/helper", false),
            ("./.github/workflows/reusable.yml", true),
        ] {
            assert_eq!(pinned(uses), ok, "{uses}");
        }
    }

    // ADV23R3-05: a shell inherited from workflow or job defaults is checked,
    // and shells are allowed by name, so any interpreter template is rejected.
    #[test]
    fn adv23r3_05_inherited_shells_are_checked() {
        let script = "run: |\n          import sys\n          sys.exit(0)";
        let body = GOOD.replace("run: cargo xtask docs", script);
        let job = body.replace(
            "    runs-on: macos-15\n",
            "    runs-on: macos-15\n    defaults:\n      run:\n        shell: python\n",
        );
        assert!(!run(&job).is_empty(), "job defaults accepted");
        let workflow = body.replace(
            "permissions:",
            "defaults:\n  run:\n    shell: python {0}\npermissions:",
        );
        assert!(!run(&workflow).is_empty(), "workflow defaults accepted");
        for shell in ["node {0}", "perl {0}", "/usr/bin/env python3 {0}", "py"] {
            let step = GOOD.replace(
                "run: cargo xtask docs",
                &format!("run: cargo xtask docs\n        shell: \"{shell}\""),
            );
            assert!(!run(&step).is_empty(), "step shell {shell} accepted");
        }
        let controls = GOOD.replace(
            "permissions:",
            "defaults:\n  run:\n    shell: bash\npermissions:",
        );
        assert!(run(&controls).is_empty(), "{:?}", run(&controls));
    }

    // R1: the trusted gate graph cannot be weakened by the workflow under test.
    #[test]
    fn r1_gate_graph_is_enforced() {
        let dropped = GOOD.replace("needs: [a, b]", "needs: [a]");
        assert_eq!(run(&dropped).len(), 1);
        let removed = GOOD.replace(
            "  b:\n    runs-on: macos-15\n    steps:\n      - run: cargo xtask docs\n",
            "",
        );
        assert!(!run(&removed).is_empty());
        let conditional = GOOD.replace(
            "  b:\n    runs-on: macos-15",
            "  b:\n    if: false\n    runs-on: macos-15",
        );
        assert_eq!(run(&conditional).len(), 1);
        let soft = GOOD.replace(
            "  b:\n    runs-on: macos-15",
            "  b:\n    continue-on-error: true\n    runs-on: macos-15",
        );
        assert_eq!(run(&soft).len(), 1);
        let not_always = GOOD.replace("    if: always()\n", "");
        assert_eq!(run(&not_always).len(), 1);
    }
    // F5: an env value interpolated into a script is as unsafe as the event
    // value it holds; only the shell variable form passes.
    #[test]
    fn f5_env_expression_in_run_is_rejected() {
        let job = "env:\n      TITLE: ${{ github.event.pull_request.title }}\n    steps:\n      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1\n        with: {persist-credentials: false}\n";
        let unsafe_form = GOOD.replace(
            "steps:\n      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1\n        with: {persist-credentials: false}\n",
            job,
        ).replace("echo \"$HOME ${{ runner.temp }}\"", "echo \"${{ env.TITLE }}\"");
        let found = run(&unsafe_form);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].message.contains("env.TITLE"));
        let safe = unsafe_form.replace("echo \"${{ env.TITLE }}\"", "echo \"$TITLE\"");
        assert_eq!(run(&safe), Vec::new());
        let real = Policy::parse(
            &std::fs::read_to_string(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("policy/gates.toml"),
            )
            .expect("gates"),
        )
        .expect("policy");
        assert!(!real.expressions.iter().any(|e| e.starts_with("env")));
    }

    const TRUSTED: &str = r#"
on:
  pull_request_target:
    types: [opened]
permissions:
  contents: read
jobs:
  review:
    runs-on: ubuntu-24.04
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1
        with: {persist-credentials: false, fetch-depth: 0}
      - run: cargo run -p xtask -- review --main refs/remotes/origin/main
"#;

    fn run_as(path: &str, text: &str) -> Vec<Finding> {
        check_workflow(path, text, &Policy::parse(POLICY).expect("policy"))
    }

    // F1: pull_request_target only in the listed trusted workflow, and a
    // trusted job never checks out the PR head, uses local actions or caches.
    #[test]
    fn f1_trusted_workflow_rules() {
        assert_eq!(run_as("t.yml", TRUSTED), Vec::new());
        assert_eq!(
            run_as("w2.yml", TRUSTED).len(),
            1,
            "unlisted pull_request_target"
        );
        for (from, to) in [
            (
                "with: {persist-credentials: false, fetch-depth: 0}",
                "with: {persist-credentials: false, ref: \"${{ github.event.pull_request.head.sha }}\"}",
            ),
            (
                "with: {persist-credentials: false, fetch-depth: 0}",
                "with: {persist-credentials: false, repository: someone/fork}",
            ),
            (
                "      - run: cargo",
                "      - uses: ./.github/actions/trusted-policy\n      - run: cargo",
            ),
            (
                "      - run: cargo",
                "      - uses: actions/cache@0400d5f644dc74513175e3cd8d07132dd4860809\n      - run: cargo",
            ),
        ] {
            let text = TRUSTED.replace(from, to);
            assert!(!run_as("t.yml", &text).is_empty(), "accepted: {to}");
        }
        // The trusted graph: the gate job cannot be dropped or made conditional.
        let dropped = TRUSTED.replace("  review:", "  other:");
        assert!(!run_as("t.yml", &dropped).is_empty());
        let conditional = TRUSTED.replace("  review:\n", "  review:\n    if: false\n");
        assert!(!run_as("t.yml", &conditional).is_empty());
        let (files, jobs) = Policy::parse(POLICY).expect("policy").trusted_gates();
        assert_eq!(files, ["t.yml"]);
        assert_eq!(jobs, ["review"]);
    }

    // ADV23R8-02: dynamic-loader variables are rejected at workflow, job and
    // step level, whatever the allow-list says.
    #[test]
    fn adv23r8_02_loader_env_is_rejected() {
        for name in [
            "LD_PRELOAD",
            "LD_LIBRARY_PATH",
            "LD_AUDIT",
            "ld_preload",
            "DYLD_INSERT_LIBRARIES",
            "DYLD_LIBRARY_PATH",
            "DYLD_FALLBACK_LIBRARY_PATH",
        ] {
            for text in [
                GOOD.replace(
                    "permissions:",
                    &format!("env: {{{name}: /tmp/injected.so}}\npermissions:"),
                ),
                GOOD.replace(
                    "    runs-on: macos-15\n",
                    &format!("    runs-on: macos-15\n    env: {{{name}: x}}\n"),
                ),
                GOOD.replace(
                    "run: cargo xtask docs",
                    &format!("run: cargo xtask docs\n        env: {{{name}: /tmp/x.so}}"),
                ),
            ] {
                let found = run(&text);
                assert_eq!(found.len(), 1, "{name}: {found:?}");
                assert!(found[0].message.contains(name), "{found:?}");
            }
            let policy = POLICY.replace("allowed_env = [", &format!("allowed_env = [\"{name}\", "));
            let text = GOOD.replace("permissions:", &format!("env: {{{name}: x}}\npermissions:"));
            assert_eq!(
                check_workflow("w.yml", &text, &Policy::parse(&policy).expect("policy")).len(),
                1,
                "{name} allow-listed"
            );
        }
        // Controls: other variables named like them are not loader variables.
        assert!(!is_loader_env("OLD_PRELOAD"));
        assert!(!is_loader_env("BUILD_DIR"));
        // The build environment drops them too.
        assert!(crate::rust_policy::is_sanitised("LD_PRELOAD"));
        assert!(crate::rust_policy::is_sanitised("DYLD_INSERT_LIBRARIES"));
        let mut command = std::process::Command::new("cc");
        command
            .env("LD_PRELOAD", "/tmp/injected.so")
            .env("PATH", "/usr/bin");
        crate::build_gate::scrub_loader_env(&mut command);
        let envs: Vec<_> = command.get_envs().collect();
        assert!(
            envs.contains(&(std::ffi::OsStr::new("LD_PRELOAD"), None)),
            "{envs:?}"
        );
        assert!(envs.contains(&(
            std::ffi::OsStr::new("PATH"),
            Some(std::ffi::OsStr::new("/usr/bin"))
        )));
    }

    // ADV23R7-03: interpreters are matched by command-name family, not by
    // exact strings: Python variants, PyPy, Julia and other script runtimes.
    #[test]
    fn adv23r7_03_interpreters_are_matched_by_family() {
        for script in [
            "pypy3 -c 'print(1)'",
            "julia -e 'println(1)'",
            "pythonw x",
            "python3.12 -m venv v",
            "/usr/bin/python3.13t -c 1",
            "PYTHON3 -c 1",
            "python.exe -c 1",
            "env python -c 1",
            "pip3 install x",
            "uv run tool",
            "uvx ruff",
            "ipython -c 1",
            "julia-1.11 -e 1",
            "node -e 1",
            "npx something",
            "perl -e 1",
            "ruby -e 1",
            "Rscript -e 1",
            "deno run x",
            "./tools/check.jl",
            "bash -c 'conda run x'",
            "sh scripts/x.py",
        ] {
            assert!(invokes_interpreter(script), "{script}");
        }
        for script in [
            "set -euo pipefail",
            "cargo xtask all",
            "printf '%s' \"$NEEDS\" | jq -r .",
            "echo typescript notes; echo pipeline nodes ok",
            "rustup toolchain install && rustc --version",
            "cargo nextest run --workspace --locked",
        ] {
            assert!(!invokes_interpreter(script), "{script}");
        }
    }
}
