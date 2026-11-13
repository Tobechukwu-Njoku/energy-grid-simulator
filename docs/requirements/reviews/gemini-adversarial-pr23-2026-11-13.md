# Adversarial Review — PR #23 Rust Workspace and GitHub Actions CI (Milestone M0)

**Reviewer:** Adversarial Review Sub-Agent  
**Date:** 2026-11-13  
**Target:** PR #23 at head commit `bebd708a574940805854886130cbf8f15907b29b` (`t3/m0-cargo-workspace-ci`)  
**Target Files:**
- `.github/workflows/ci.yml` (lines 1–189)
- `Cargo.toml` (lines 1–55)
- `deny.toml` (lines 1–56)
- `rust-toolchain.toml` (lines 1–6)
- `ci/policy_checks.py` (lines 1–118)
- `ci/attribution_policy.py` (lines 1–172)
- `ci/attribution-match.pl` (lines 1–67)
- `ci/attribution-exemptions.txt` (lines 1–13)
- `ci/doc_gates.py` (lines 1–208)
- `ci/tests/test_attribution.py` (lines 1–189)
- `docs/security/ci-runner-policy.md` (lines 1–88)
- `CONTRIBUTING.md` (lines 1–55)
- `SECURITY.md` (lines 1–25)
- `crates/*/Cargo.toml` and `crates/*/src/*.rs` (skeleton crates)

**Baseline Authority:**
- `AGENTS.md` (D-52, Rules 1–10)
- `docs/requirements/03-decisions-log.md` (D-1 through D-72)
- `docs/architecture/adr/0001-repo-layout-and-ci-gates.md`
- `docs/requirements/04-requirements.md` (Milestone M0, NFR-3, NFR-8, NFR-10, NFR-12)

---

## Executive Summary

PR #23 establishes the initial Milestone M0 Cargo workspace skeleton, GitHub Actions CI workflow, local and CI attribution policy checkers, licence and security policy verification, and documentation validation scripts.

An aggressive adversarial review targeting CI hardening, policy bypasses, vacuous gates, and compliance with ADR 0001 and NFR-12 reveals **critical security vulnerabilities, total bypasses of mandatory gates, and architectural omissions**:

1. **Complete Attribution Check Bypass via Source Comments & Punctuation:** The attribution scanner regex in `.githooks/attribution-patterns.regex` is line-anchored and ignores comment syntax. Adding any comment prefix (`//`, `/*`, `#`, `--`, `<!--`) or punctuation prefix (`+`) completely shields AI attribution from detection. The test suite in `ci/tests/test_attribution.py:164` inadvertently proves this by adding `// Generated with Claude Code` to `src/main.rs`, which the scanner silently ignores.
2. **Untrusted Workspace Self-Evaluation & Policy Waiving:** While `docs/security/ci-runner-policy.md:34` claims policy scripts are executed from the trusted base revision, `.github/workflows/ci.yml:145` executes `ci/policy_checks.py`, line 79 reads `deny.toml`, and line 138 executes `ci/doc_gates.py` directly from the PR branch checkout. An untrusted PR can modify `policy_checks.py` or `deny.toml` to exit 0 or permit forbidden dependencies, entirely bypassing CI validation.
3. **Multiline & `cfg_attr` Bypasses of `unsafe_code` Scope:** `ci/policy_checks.py` checks for `unsafe_code` line-by-line. Breaking the attribute across lines (`#[allow(\n unsafe_code\n)]`), using `#[cfg_attr(all(), allow(unsafe_code))]`, or using TOML sub-table syntax (`[lints.rust.unsafe_code] level = "allow"`) completely evades `check_unsafe_scope()`.
4. **Omission of the Mandatory `provenance/secrets/data` Gate:** ADR 0001 line 101 mandates a `provenance/secrets/data` gate in the gate graph. `.github/workflows/ci.yml` omits this job completely, and `merge-gates` does not even register it under `not_applicable`.
5. **Vacuous `independent-review` Gate:** `merge-gates` prints a string claiming review evidence is verified manually and exits 0. There is no automated verification ensuring that a review report exists, targets the current HEAD, or satisfies role separation.
6. **Committed Python Bytecode (`.pyc`) Artifacts:** Binary `.pyc` files (`ci/__pycache__/doc_gates.cpython-313.pyc` and `ci/tests/__pycache__/test_attribution.cpython-313.pyc`) are committed into git because `.gitignore` lacks global Python bytecode exclusions.
7. **Workflow Command Injection:** `ci/attribution_policy.py:148` directly prints unsanitized finding text into GitHub Actions `::error::` annotations, permitting runner command and annotation manipulation.

PR #23 must not be merged until these vulnerabilities are mitigated.

---

## Findings Matrix

| ID | Severity | File & Line Citation | Category | Short Title |
|---|---|---|---|---|
| **ADV23-01** | CRITICAL | `.githooks/attribution-patterns.regex:2, 4`, `ci/attribution_policy.py:73-76`, `ci/tests/test_attribution.py:164` | Policy Bypass / Attribution (D-52) | Code comments and file headers completely bypass the attribution check |
| **ADV23-02** | CRITICAL | `.github/workflows/ci.yml:79, 138, 145`, `docs/security/ci-runner-policy.md:34-36` | CI Vulnerability / Self-Evaluation | `policy_checks.py`, `deny.toml`, and `doc_gates.py` execute untrusted PR code |
| **ADV23-03** | CRITICAL | `ci/policy_checks.py:15, 36-39` | Policy Bypass / Unsafe Scope | Multiline attribute syntax bypasses `check_unsafe_scope()` |
| **ADV23-04** | CRITICAL | `.github/workflows/ci.yml:153-189`, ADR 0001:101 | Vacuous Gate / Missing Gate | Mandatory `provenance/secrets/data` gate is omitted entirely from CI |
| **ADV23-05** | HIGH | `ci/policy_checks.py:15, 36-39` | Policy Bypass / Unsafe Scope | `cfg_attr` conditional compilation allows unsafe code without detection |
| **ADV23-06** | HIGH | `ci/policy_checks.py:43-45, 51-56` | Policy Bypass / Unsafe Scope | TOML sub-table syntax in crate `Cargo.toml` overrides unsafe lint undetected |
| **ADV23-07** | HIGH | `.githooks/attribution-patterns.regex:2, 4`, `ci/attribution-match.pl:33-43` | Policy Bypass / Unicode | Zero-width character and homoglyph injection defeats attribution regex |
| **ADV23-08** | HIGH | `ci/attribution-match.pl:54`, `ci/attribution_policy.py:135-137` | Policy Bypass / Commit Parsing | Git scissors line (`# >8`) truncates commit scan, hiding trailers |
| **ADV23-09** | HIGH | `ci/attribution_policy.py:72-76` | Diagnostic Integrity / Line Drift | Diff context lines are not counted, causing line number drift in error reports |
| **ADV23-10** | HIGH | `.github/workflows/ci.yml:176-177`, ADR 0001:103, 117-124 | Vacuous Gate / Review Integrity | `independent-review` gate passes vacuously with zero automated validation |
| **ADV23-11** | HIGH | `ci/__pycache__/`, `ci/tests/__pycache__/`, `.gitignore:1-10` | Repository Security / Integrity | Python bytecode (`.pyc`) committed into git due to incomplete `.gitignore` |
| **ADV23-12** | MEDIUM | `deny.toml:34-45`, `crates/*/Cargo.toml` | Policy Bypass / Banned Runtimes | `build.rs` and FFI bypass `deny.toml` runtime bans (NFR-10) |
| **ADV23-13** | MEDIUM | `deny.toml:30-33` | Licence Risk / Private Crates | Private workspace crates are exempted from licence checks (`ignore = true`) |
| **ADV23-14** | MEDIUM | `ci/attribution_policy.py:148` | Workflow Injection | Untrusted match text interpolated directly into `::error::` workflow command |
| **ADV23-15** | MEDIUM | `ci/policy_checks.py:68` | Workflow Security / Permissions | Inline YAML object mapping (`permissions: { contents: write }`) evades check |
| **ADV23-16** | MEDIUM | `ci/policy_checks.py:18-21` | Workflow Security / Expression Filtering | Untrusted expression filter omits `client_payload`, `inputs`, and actor contexts |
| **ADV23-17** | MEDIUM | `ci/attribution-exemptions.txt:7-14`, ADR 0001:177-181 | Policy Compliance / Scope Creep | Blanket whole-file exemptions violate ADR 0001 section-scoping mandate |
| **ADV23-18** | MEDIUM | `ci/doc_gates.py:27, 61-75, 96-105` | Gate Robustness / Incomplete Scan | Reference-style markdown links and non-markdown references evade doc gates |

---

## Detailed Findings

### ADV23-01: Code Comments and File Headers Completely Bypass the Attribution Check
- **File & Line:** `.githooks/attribution-patterns.regex:2, 4`, `ci/attribution_policy.py:73-76`, `ci/tests/test_attribution.py:164`
- **Severity:** CRITICAL
- **Category:** Policy Bypass / Attribution (D-52)
- **Description:**  
  AGENTS.md Rule 8 and ADR 0001 (lines 176–177) require scanning and rejecting added lines across the diff, specifically naming "code comments and headers".
  However, both regex patterns in `.githooks/attribution-patterns.regex` are anchored at the beginning of the line:
  - Pattern 1 (trailers): `^\s*(?:>\s*)?[\w\-\x{2010}-\x{2015}]*[\-\x{2010}-\x{2015}](?:by|with):`
  - Pattern 2 (footers): `^\s*(?:>\s*)?(?:\x{1F916}\x{FE0F}?\s*)?(?:\*{1,2})?(?:generated|written...`
  
  Neither regex allows leading comment syntax. Any comment prefix such as `//` (Rust/JS/C++), `/*` (C/Rust/CSS), `#` (Python/Shell/YAML), `--` (SQL/Lua), or `<!--` (HTML/Markdown) prevents the line from matching.
  
  In fact, `ci/tests/test_attribution.py` line 164 contains:
  ```python
  self.write("src/main.rs", f"// {ATTRIBUTION_LINE}\n")
  self.write("NOTES.md", f"{ATTRIBUTION_LINE}\n")
  self.commit("docs: notes")
  result = self.run_check()
  self.assertEqual(result.returncode, 1)
  self.assertIn("added line NOTES.md:1", result.stdout)
  ```
  The test author explicitly committed `// Generated with Claude Code` to `src/main.rs`, but `run_check()` only reported `NOTES.md:1`. The test passed solely because `NOTES.md` was also modified. The source code comment was completely ignored.
- **Impact:** Any contributor or subagent can add watermarks or attribution headers in source code comments across all crates without triggering CI failure.
- **Recommendation:** Modify pattern matching in `ci/attribution-match.pl` to strip leading comment delimiters (e.g. `//`, `/*`, `*`, `#`, `--`, `<!--`) and optional markdown prefixes before matching attribution patterns.

---

### ADV23-02: `policy_checks.py`, `deny.toml`, and `doc_gates.py` Execute Untrusted PR Code
- **File & Line:** `.github/workflows/ci.yml:79, 138, 145`, `docs/security/ci-runner-policy.md:34-36`
- **Severity:** CRITICAL
- **Category:** CI Vulnerability / Self-Evaluation
- **Description:**  
  `docs/security/ci-runner-policy.md:34` asserts:
  > "Policy scripts judging a PR come from the base revision where they exist (attribution check), so a PR cannot weaken the rules applied to itself."
  
  This claim is false for the rest of CI:
  - Line 145: `run: python3 ci/policy_checks.py` runs directly from the PR checkout.
  - Line 79: `run: cargo deny --locked check` uses `deny.toml` directly from the PR checkout.
  - Line 138: `run: python3 ci/doc_gates.py` runs directly from the PR checkout.
  - Line 90: `run: python3 .githooks/tests/run.py` runs directly from the PR checkout.
  - Line 123: In `.github/workflows/ci.yml`, the bootstrap fallback copies `ci/attribution_policy.py` from the PR branch if the base lacks it. For PR #23 itself, CI executed the PR's own unverified script.
- **Impact:** A malicious or non-compliant PR can modify `ci/policy_checks.py` (e.g. returning 0 immediately), relax `deny.toml` (allowing copyleft licenses or removing bans on Python/Julia), or weaken `doc_gates.py`. CI will run the altered scripts and greenlight the PR.
- **Recommendation:** Update `.github/workflows/ci.yml` so that all policy scripts (`policy_checks.py`, `doc_gates.py`) and configuration files (`deny.toml`, `.githooks/attribution-patterns.regex`) are extracted from `$base` into `$RUNNER_TEMP/trusted-ci` and executed from there.

---

### ADV23-03: Multiline Attribute Syntax Bypasses `check_unsafe_scope()`
- **File & Line:** `ci/policy_checks.py:15, 36-39`
- **Severity:** CRITICAL
- **Category:** Policy Bypass / Unsafe Scope
- **Description:**  
  `ALLOW_UNSAFE` is defined as:
  ```python
  ALLOW_UNSAFE = re.compile(r"#!?\[\s*(?:allow|expect|warn)\s*\([^)]*\bunsafe_code\b")
  ```
  In `check_unsafe_scope()`, `path.read_text().splitlines()` evaluates the file line by line:
  ```python
  for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
      if ALLOW_UNSAFE.search(line):
          errors.append(...)
  ```
  If a developer formats the attribute across multiple lines:
  ```rust
  #[allow(
      unsafe_code
  )]
  pub fn vulnerable_function() { ... }
  ```
  Line 1 (`#[allow(`) does not contain `unsafe_code`, and Line 2 (`unsafe_code`) does not contain `#[allow`. Neither line matches `ALLOW_UNSAFE`. Rust accepts this multiline attribute and suppresses the `unsafe_code` lint, while `ci/policy_checks.py` reports zero errors.
- **Impact:** Unsafe code can be introduced into any non-solver crate without triggering CI failure.
- **Recommendation:** Evaluate the entire file text using `re.DOTALL` or `re.MULTILINE`, or use `tree-sitter-rust` / `syn` to reliably detect attribute invocations regardless of whitespace formatting.

---

### ADV23-04: Mandatory `provenance/secrets/data` Gate is Omitted Entirely from CI
- **File & Line:** `.github/workflows/ci.yml:153-189`, ADR 0001:101
- **Severity:** CRITICAL
- **Category:** Vacuous Gate / Missing Gate
- **Description:**  
  ADR 0001 Section "Gate graph and quality criteria" specifies the required gate graph in lines 93–105. Line 101 explicitly mandates:
  > `provenance/secrets/data`: "Research-only raw/derived restricted data never committed; ODbL/share-alike excluded; manifests tagged open-commercial/research-only and publication preconditions scanned before D-58 visibility change"
  
  In `.github/workflows/ci.yml`, there is no job implementing or checking `provenance/secrets/data`.
  Furthermore, in the `merge-gates` aggregator job (lines 165–177), `not_applicable` lists `typescript fmt/lint/tests`, `numerical-reference`, `benchmark-regression`, `mutation`, and `gpu-validation`. `provenance/secrets/data` is not listed under `needs`, nor under `not_applicable`, nor under `MANUAL`. It was completely omitted.
- **Impact:** Restricted research-only datasets, unmanifested proprietary files, or credentials could be committed without any CI gate verifying compliance.
- **Recommendation:** Implement a dedicated `provenance-secrets` job or explicitly register its status and applicability in `merge-gates` with documented rationale.

---

### ADV23-05: `cfg_attr` Conditional Compilation Allows Unsafe Code Without Detection
- **File & Line:** `ci/policy_checks.py:15, 36-39`
- **Severity:** HIGH
- **Category:** Policy Bypass / Unsafe Scope
- **Description:**  
  Rust provides the `cfg_attr` attribute to conditionally apply attributes:
  ```rust
  #[cfg_attr(all(), allow(unsafe_code))]
  ```
  Because `ALLOW_UNSAFE` strictly requires `(?:allow|expect|warn)` immediately following `#[`, a line containing `cfg_attr` does not match the regex. `cfg_attr(all(), ...)` is unconditionally active during compilation, successfully silencing compiler errors while bypassing `ci/policy_checks.py`.
- **Impact:** Completely bypasses unsafe code restrictions in core crates.
- **Recommendation:** Broaden the pattern to detect any occurrence of `unsafe_code` within any inner or outer attribute, or disallow `unsafe_code` as an identifier anywhere in attributes outside solver crates: `r"#!?\[[^\]]*\bunsafe_code\b"`.

---

### ADV23-06: TOML Sub-Table Syntax in Crate `Cargo.toml` Overrides Unsafe Lint Undetected
- **File & Line:** `ci/policy_checks.py:43-45, 51-56`
- **Severity:** HIGH
- **Category:** Policy Bypass / Unsafe Scope
- **Description:**  
  In `ci/policy_checks.py:43`, crate `Cargo.toml` files are scanned for unsafe lint overrides using:
  ```python
  levels = re.findall(r"^\s*unsafe_code\s*=\s*(?:\{[^}]*level\s*=\s*)?\"(\w+)\"", text, re.M)
  ```
  In TOML, a crate can define lints using sub-tables:
  ```toml
  [lints]
  workspace = true

  [lints.rust.unsafe_code]
  level = "allow"
  ```
  This is valid Cargo configuration. `check_crate_lints()` passes because `[lints]\nworkspace = true` exists.
  However, `check_unsafe_scope()` finds zero matches for `unsafe_code =` because the key is `level = "allow"`. The override succeeds and allows unsafe code, while `policy_checks.py` reports `PASS`.
- **Impact:** Any crate can disable the workspace `unsafe_code` deny lint in `Cargo.toml` without detection.
- **Recommendation:** Parse `Cargo.toml` using `tomllib` (standard library in Python 3.11+) instead of regular expressions, and inspect the parsed `lints.rust.unsafe_code` table directly.

---

### ADV23-07: Zero-Width Character and Homoglyph Injection Defeats Attribution Regex
- **File & Line:** `.githooks/attribution-patterns.regex:2, 4`, `ci/attribution-match.pl:33-43`
- **Severity:** HIGH
- **Category:** Policy Bypass / Unicode
- **Description:**  
  `ci/attribution-match.pl` enforces strict UTF-8 decoding, but performs no normalization (such as Unicode NFKC) or sanitization of invisible characters.
  An author can insert:
  - Zero-width spaces (`U+200B`): `Co\u200B-Authored-By: Claude` or `Cl\u200Baude`
  - Cyrillic homoglyphs: `Cl\u0430ude` (using Cyrillic small letter 'а' U+0430) or `C\u043Epilot` (Cyrillic 'о' U+043E)
  - Soft hyphens (`U+00AD`): `Co\u00ADAuthored-By`
  
  These strings render identically or near-identically in standard editors, GitHub UI, and terminals, but fail byte-level ASCII regex comparisons.
- **Impact:** Trivial evasion of the D-52 attribution check.
- **Recommendation:** Apply Unicode NFKC normalization and strip non-printable zero-width/formatting characters (`\p{Cf}`) before executing pattern matching in `ci/attribution-match.pl`.

---

### ADV23-08: Git Scissors Line (`# >8`) Truncates Commit Scan, Hiding Trailers
- **File & Line:** `ci/attribution-match.pl:54`, `ci/attribution_policy.py:135-137`
- **Severity:** HIGH
- **Category:** Policy Bypass / Commit Parsing
- **Description:**  
  `ci/attribution-match.pl` line 54 includes:
  ```perl
  last if $item->{mode} eq 'message' && $line =~ /^# -+ >8 -+/;
  ```
  This logic halts line processing when encountering a Git scissors line (`# ------------------------ >8 ------------------------`).
  While Git commit editors use scissors lines during interactive commits (`git commit -v`), any text committed into the repository history retains the full message body when extracted with `git log -1 --format=%B`.
  If an author crafts a commit message containing:
  ```
  feat: add network solver stub
  # ------------------------ >8 ------------------------
  Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
  ```
  The commit object permanently stores the trailer in Git history, but `attribution-match.pl` aborts scanning at the scissors line and reports zero findings.
- **Impact:** Attribution trailers can be hidden in committed repository history, violating D-52 and ADR 0001.
- **Recommendation:** Do not truncate message scanning at the scissors line in CI mode. CI inspects committed history; every line of the stored commit message must be scanned.

---

### ADV23-09: Diff Context Lines Are Not Counted, Causing Line Number Drift in Error Reports
- **File & Line:** `ci/attribution_policy.py:72-76`
- **Severity:** HIGH
- **Category:** Diagnostic Integrity / Line Drift
- **Description:**  
  When parsing diff output in `added_lines()`, hunk headers initialize the line count:
  ```python
  elif line.startswith(b"@@"):
      in_header = False
      number = int(line.split(b" ")[2].split(b",")[0].lstrip(b"+"))
  elif not in_header and line.startswith(b"+") and path:
      found.append((path, number, line[1:]))
      number += 1
  ```
  In unified diff format, context lines begin with a space (` `) and consume line numbers in the target file.
  Because `attribution_policy.py` only increments `number` when `line.startswith(b"+")`, it completely ignores context lines.
  If a diff hunk contains 20 lines of context before an added line, `number` remains at the hunk start line, reporting an error offset by 20 lines.
- **Impact:** Developers receive misleading file and line citations when CI fails, complicating triage and remediation.
- **Recommendation:** Increment `number += 1` on context lines (`line.startswith(b" ")`) as well as added lines (`line.startswith(b"+")`).

---

### ADV23-10: `independent-review` Gate Passes Vacuously With Zero Automated Validation
- **File & Line:** `.github/workflows/ci.yml:176-177`, ADR 0001:103, 117-124
- **Severity:** HIGH
- **Category:** Vacuous Gate / Review Integrity
- **Description:**  
  ADR 0001 line 103 mandates an `independent-review` gate verifying role-compatible independent evidence, current-head SHA binding, and adversarial triage.
  In `.github/workflows/ci.yml:176-177`, the `merge-gates` job outputs:
  ```python
  print("MANUAL: independent-review evidence and adversarial triage are verified by the "
        "coordinator on the exact head SHA (NFR-3.AC3, NFR-3.AC5)")
  sys.exit(1 if failed else 0)
  ```
  There is zero script logic checking whether a review report exists in `docs/requirements/reviews/`, whether it cites the pull request's exact HEAD SHA, or whether author and reviewer roles are distinct.
  As long as the lightweight automated checks pass, `merge-gates` returns exit code 0.
- **Impact:** A pull request can achieve a green CI status without any independent review or adversarial audit having taken place.
- **Recommendation:** Implement automated validation in `ci/policy_checks.py` or a dedicated review-checker script that inspects `docs/requirements/reviews/` or PR comments for a structured verdict matching the current HEAD SHA.

---

### ADV23-11: Python Bytecode (`.pyc`) Committed Into Git Due to Incomplete `.gitignore`
- **File & Line:** `ci/__pycache__/doc_gates.cpython-313.pyc`, `ci/tests/__pycache__/test_attribution.cpython-313.pyc`, `.gitignore:1-10`
- **Severity:** HIGH
- **Category:** Repository Security / Integrity
- **Description:**  
  PR #23 commits binary compiled Python bytecode files into Git:
  - `ci/__pycache__/doc_gates.cpython-313.pyc` (14,171 bytes)
  - `ci/tests/__pycache__/test_attribution.cpython-313.pyc` (15,807 bytes)
  
  Inspection of `.gitignore` reveals that only `/spikes/data/__pycache__/` was ignored:
  ```gitignore
  .DS_Store
  /target/
  node_modules/
  .env
  *.local
  /t3-*/
  /spikes/data/raw/
  /spikes/data/output/
  /spikes/data/__pycache__/
  /spikes/data/.venv/
  ```
  Because `__pycache__/` and `*.pyc` are not ignored repository-wide, bytecode artifacts were tracked and committed.
- **Impact:** Committing binary bytecode into source control introduces potential backdoors (precompiled bytecode executing different logic than source), platform incompatibility, and repository bloat.
- **Recommendation:** Add `__pycache__/` and `*.pyc` to the root `.gitignore` and execute `git rm -r --cached ci/__pycache__ ci/tests/__pycache__`.

---

### ADV23-12: `build.rs` and FFI Bypass `deny.toml` Runtime Bans (NFR-10)
- **File & Line:** `deny.toml:34-45`, `crates/*/Cargo.toml`
- **Severity:** MEDIUM
- **Category:** Policy Bypass / Banned Runtimes
- **Description:**  
  `deny.toml` bans specific crate names (`pyo3`, `pyo3-ffi`, `cpython`, `python3-sys`, `jlrs`, `jl-sys`) to satisfy NFR-10.AC1 ("no Python or Julia runtime in the production graph").
  However, `cargo-deny` only inspects package names in `Cargo.lock`. It does not inspect build scripts (`build.rs`).
  A crate can introduce a `build.rs` that links directly to `libpython3.so` or `libjulia.so` via `println!("cargo:rustc-link-lib=python3")`, or compiles C wrappers using `cc-rs`, evading `cargo-deny` entirely.
- **Impact:** Python or Julia runtime dependencies can infiltrate production builds without violating `deny.toml`.
- **Recommendation:** Add a lint in `ci/policy_checks.py` that scans `build.rs` and crate code for direct FFI links or dynamic loading of `python` or `julia` libraries.

---

### ADV23-13: Private Workspace Crates Are Exempted From Licence Checks (`ignore = true`)
- **File & Line:** `deny.toml:30-33`
- **Severity:** MEDIUM
- **Category:** Licence Risk / Private Crates
- **Description:**  
  `deny.toml` configures:
  ```toml
  [licenses.private]
  ignore = true
  ```
  Because all crates in `crates/*` declare `publish = false`, they are classified as private crates and completely ignored by `cargo deny check licenses`.
  If a developer introduces a copyleft or proprietary source file directly into a private crate, or sets an unreviewed `license` field in a crate's `Cargo.toml`, `cargo-deny` ignores it.
- **Impact:** Unreviewed or incompatible licenses within internal crate manifests or source trees remain undetected.
- **Recommendation:** Enforce license validation across private crates or explicitly validate crate-level license fields in `ci/policy_checks.py`.

---

### ADV23-14: Untrusted Match Text Interpolated Directly Into `::error::` Workflow Command
- **File & Line:** `ci/attribution_policy.py:148`
- **Severity:** MEDIUM
- **Category:** Workflow Injection
- **Description:**  
  When reporting findings, `ci/attribution_policy.py` executes:
  ```python
  print(f"::error::{location}: {finding['reason']}: {finding['text']}")
  ```
  In GitHub Actions runner environments, lines starting with `::` are parsed as runner workflow commands.
  If an untrusted PR contains attribution lines with embedded control characters (such as `%0A`, `%0D`, or escaped `::`), printing the raw `finding['text']` allows workflow annotation manipulation or command injection.
- **Impact:** Untrusted PR input can inject spurious GitHub Actions annotations or alter workflow log presentation.
- **Recommendation:** Sanitize `finding['text']` by stripping control characters and escaping colons/newlines before formatting GitHub Actions workflow commands.

---

### ADV23-15: Inline YAML Object Mapping (`permissions: { contents: write }`) Evades Check
- **File & Line:** `ci/policy_checks.py:68`
- **Severity:** MEDIUM
- **Category:** Workflow Security / Permissions
- **Description:**  
  In `ci/policy_checks.py:68`, workflow permissions are audited using:
  ```python
  for match in re.finditer(r"^\s*([\w-]+):\s*write\b|write-all", text, re.M):
      errors.append(...)
  ```
  The regex expects multiline YAML entries where a word starts the line (`^\s*([\w-]+):`).
  If a workflow or job specifies inline YAML/JSON object mapping:
  ```yaml
  permissions: { contents: write }
  ```
  The line begins with `permissions: {`, so `([\w-]+):` does not match. The write permission is granted by GitHub Actions, but `check_workflows()` detects no error.
- **Impact:** Workflows can escalate token permissions beyond read-only without failing `ci/policy_checks.py`.
- **Recommendation:** Parse workflow YAML files using a YAML parser (`yaml.safe_load` or `ruamel.yaml`) and inspect the parsed `permissions` dictionary directly rather than using line-based regex.

---

### ADV23-16: Untrusted Expression Filter Omits `client_payload`, `inputs`, and Actor Contexts
- **File & Line:** `ci/policy_checks.py:18-21`
- **Severity:** MEDIUM
- **Category:** Workflow Security / Expression Filtering
- **Description:**  
  `UNTRUSTED_EXPRESSIONS` in `ci/policy_checks.py` matches:
  ```python
  UNTRUSTED_EXPRESSIONS = re.compile(
      r"\$\{\{\s*(?:github\.event\.(?:pull_request|issue|comment|review|head_commit|commits)"
      r"|github\.head_ref)"
  )
  ```
  This filter omits untrusted contexts including:
  - `${{ github.event.client_payload.* }}`
  - `${{ github.event.inputs.* }}`
  - `${{ github.actor }}` and `${{ github.triggering_actor }}`
  - `${{ github.event.sender.login }}`
  - Multiline expressions where `${{\n` is broken across lines.
- **Impact:** Workflow inline script injection remains possible if any of these contexts are used directly in `run:` commands.
- **Recommendation:** Forbid all `${{ ... }}` expressions within `run:` script blocks except for explicitly whitelisted trusted values.

---

### ADV23-17: Blanket Whole-File Exemptions Violate ADR 0001 Scope Mandate
- **File & Line:** `ci/attribution-exemptions.txt:7-14`, ADR 0001:177-181
- **Severity:** MEDIUM
- **Category:** Policy Compliance / Scope Creep
- **Description:**  
  ADR 0001 lines 177–181 explicitly state:
  > "exemptions are narrowly path/section-scoped by trusted policy, not all docs/**. Pattern literals in the hook/regex and test fixtures are syntax/test data, not an authorship claim; only those literals are exempt, never their surrounding banners."
  
  In direct contrast, `ci/attribution-exemptions.txt` exempts entire directory trees:
  - `AGENTS.md`
  - `docs/process/*`
  - `docs/requirements/02-questionnaire.md`
  - `docs/requirements/03-decisions-log.md`
  - `docs/requirements/reviews/*`
  - `docs/architecture/adr/0001-repo-layout-and-ci-gates.md`
  
  A PR can add full AI-generated attribution banners, robot emojis, or `Co-Authored-By:` trailers to any file in `docs/process/`, `docs/requirements/reviews/`, or `AGENTS.md`, and CI will pass without warning.
- **Impact:** Violates ADR 0001 specification and allows unvetted attribution in process documentation.
- **Recommendation:** Implement line-level or block-level exemption scoping for known pattern literals, rather than exempting entire documentation files.

---

### ADV23-18: Reference-Style Markdown Links and Non-Markdown References Evade Doc Gates
- **File & Line:** `ci/doc_gates.py:27, 61-75, 96-105`
- **Severity:** MEDIUM
- **Category:** Gate Robustness / Incomplete Scan
- **Description:**  
  `ci/doc_gates.py` contains several gaps in document consistency enforcement:
  1. `LINK = re.compile(r"(?<!!)\[[^\]]*\]\(\s*<?([^)\s>]+)>?(?:\s+\"[^\"]*\")?\s*\)")` only parses inline markdown links `[text](target)`. Reference-style markdown links (`[text][id]` and `[id]: target`) are completely ignored; broken reference targets are never caught.
  2. `check_references` only validates `D-<n>` decision references in `04-requirements.md`, `06-roadmap.md`, and ADRs. It skips `01-vision.md`, `05-risks.md`, `AGENTS.md`, and architecture documents such as `docs/architecture/topology.md`.
  3. `milestone_allocation` checks `sections - allocated`, which ensures all requirements in `04-requirements.md` are in the milestone table. However, it does not check the inverse (`allocated - sections`); hallucinated or invalid requirement IDs (e.g. `FR-999`) in the milestone table pass without error.
- **Impact:** Broken links, invalid decision citations, and phantom requirement allocations can enter documentation unnoticed.
- **Recommendation:** Enhance `ci/doc_gates.py` to parse reference-style links, check decision references across all tracked markdown files, and verify bidirectional consistency between requirement definitions and milestone allocations.

---

## Conclusion & Recommendations

PR #23 successfully establishes the foundational workspace crates and provides an initial CI structure. However, because of critical vulnerabilities that allow trivial bypasses of the attribution, unsafe-code, and licensing checks, along with the omission of the required `provenance/secrets/data` gate and the presence of committed `.pyc` binaries, **PR #23 cannot be accepted in its current state**.

### Required Action Items Before Merge
1. **Fix Comment & Punctuation Attribution Bypasses:** Update `attribution-match.pl` to strip leading comment tokens (`//`, `/*`, `#`, `--`, `<!--`) and leading diff artifacts before matching.
2. **Execute Policy Scripts from Base:** Modify `.github/workflows/ci.yml` so that `policy_checks.py`, `doc_gates.py`, and `deny.toml` are read from the trusted `$base` branch.
3. **Harden `check_unsafe_scope()`:** Evaluate file contents multiline and disallow `cfg_attr` or TOML sub-table tricks that suppress the `unsafe_code` lint.
4. **Restore `provenance/secrets/data` Gate:** Add the missing gate to `.github/workflows/ci.yml` and register it in `merge-gates`.
5. **Clean Up Bytecode & `.gitignore`:** Remove all `.pyc` files from git tracking and add `__pycache__/` and `*.pyc` to the root `.gitignore`.
