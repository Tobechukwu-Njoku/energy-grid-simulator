# Independent review of PR #23 — M0

- Date: 2026-11-13.
- Artifact kind: production-code/bootstrap-CI review under D-54.
- Author role/provider: code generation / Anthropic.
- Reviewer role/provider: independent code review / OpenAI.
- Reviewed head: `bebd708a574940805854886130cbf8f15907b29b`.
- Compared base: `b3f287d3c56c21a2b03395192a35616609315815` (`main`).
- Authority: `04-requirements.md` (M0, NFR-3, NFR-8, NFR-10, NFR-12,
  FR-18), accepted ADRs 0001 and 0003, D-52, D-54, D-65 and D-72.
- Verdict: **must-fix**. R1–R8 below require correction and exact-head re-review.
- Evidence source: direct source inspection, GitHub check/settings API reads,
  local baseline checks, scratch-Git negative cases and standalone Rust probes.
- This is independent-review evidence, not adversarial triage, a merge approval,
  or proof of branch protection. No later substantive head is covered.

## Scope and method

Read the repository instructions, requirements, decisions and accepted ADRs.
Reviewed an archive of the exact head in an isolated temporary directory, with
its Git objects/index available for tracked-file and commit-range checks.
Additional negative cases used disposable repositories/files, not edits to PR #23.
Only this review report is added on the review branch. `STATUS.md` and the
decisions log are unchanged, as requested.

All builds were small stub-only builds: no native solver, database, benchmark,
GPU workload, simulator execution or target-runner activation. The first local
nextest invocation could not link because `cc` is absent; rerunning with the
existing `/home/njotob/.local/bin/zigcc` linker succeeded. That environment issue
is not a finding against this PR.

## Findings requiring correction

### 1. R1 — High: a policy-changing PR can authorize its own weaker gates

**Locations:** `.github/workflows/ci.yml:77`, `.github/workflows/ci.yml:139`,
`.github/workflows/ci.yml:144`, `deny.toml:18`, `ci/policy_checks.py:71`.
**Requirements:** NFR-3.AC1/AC2, NFR-8.AC2, ADR 0001 trusted-base policy.

The licence job uses the checked-out PR's `deny.toml`; the repository-policy
checker and required-job list are also taken from that PR. There is no check
against the previous policy or independent disposition of a policy delta.
Reading the attribution driver from the base is useful but does not protect
the workflow step that invokes it or the presence of the attribution job.

**Reproduced:** a tiny scratch package with a publishable GPL-3.0-only path
dependency fails `cargo deny --locked check licenses` with the original policy
(exit 4). Adding GPL-3.0-only to that same scratch policy passes (exit 0).
Separately, removing the attribution job and its `needs` entry from a copy of
this workflow yields no errors from the unchanged `check_workflows()`.
Neither reproduction changes PR #23 or requires weakening the checker itself.

**Suggested correction:** validate the expected gate graph and policy deltas
using a trusted base/coordinator-controlled policy; check PR dependencies against
the base licence/advisory rules using an explicit trusted config path. A change
to those rules needs a separate, exact-head independent disposition and old/new
policy tests, not a green result under its own new allow-list. Document the
initial seed as an independently reviewed bootstrap, not a permanent fallback.
This does not require defending against a malicious owner-token holder; it
prevents an ordinary policy edit from silently weakening the checks judging it.

### 2. R2 — High: ordinary source-comment attribution is not detected

**Locations:** `ci/attribution_policy.py:152`, `ci/attribution-match.pl:61`.
**Requirements:** NFR-3.AC4, D-52, ADR 0001 added-line/header/comment coverage.

Added lines are passed unchanged to the whole-message-line regexes. The rules
are anchored, so a source comment prefix prevents a footer match. Hook parity
tests demonstrate commit-message parity, not correctness for source syntax.

**Reproduced:** using the existing `test_attribution.ATTRIBUTION_LINE` fixture,
add `// ` followed by that fixture to `src/lib.rs` in a scratch repository. The
end-to-end check reports one scanned added line and exits 0. The same fixture
on its own line inside a Rust block comment is rejected (exit 1), confirming
that the missed case is comment context, not an unsupported attribution phrase.

**Suggested correction:** add a source-text mode that checks comment/header
payloads with the shared rules while retaining the original location. Cover
Rust line/block/doc comments and relevant script/markup comment forms; preserve
ordinary technical prose and non-attribution literals. Keep commit-hook parity
tests, but add independent source-context negative fixtures.

### 3. R3 — High: PR-controlled Git attributes can hide all added source lines

**Location:** `ci/attribution_policy.py:64`.
**Requirements:** NFR-3.AC4, ADR 0001 trusted classification and added-text scans.

The scan assumes ordinary textual patch output, but `git diff` honors the
checked-out PR's `.gitattributes`. Disabling external diff drivers does not
override a `-diff` attribute: Git emits a binary-change summary with no hunks.

**Reproduced:** add `.gitattributes` containing `*.rs -diff` and a `src/lib.rs`
block comment containing the same independently rejected footer from R2. The
end-to-end check exits 0 and scans only the `.gitattributes` line, not the source.
This also works with an unchanged trusted driver and unchanged patterns.

**Suggested correction:** use an attribute-independent, forced-text diff with
text conversion disabled for policy scanning, or enumerate changed blobs and
derive the text additions independently. Define explicit binary/encoding
handling. Test head-controlled attributes and a text file classified as binary.

### 4. R4 — Medium: committed attribution after a scissors marker is skipped

**Locations:** `ci/attribution_policy.py:147`, `ci/attribution-match.pl:60`.
**Requirements:** NFR-3.AC4, ADR 0001 scan of every actual commit message.

CI uses the hook's message mode for already-stored Git messages. Ignoring a
scissors suffix makes sense for an editable commit-message file, where Git may
discard it, but not for bytes that actually survived into a commit. The current
test explicitly expects CI to ignore this region.

**Reproduced:** create a clean-subject scratch commit with hooks disabled and
`--cleanup=verbatim`, followed by a Git scissors marker and the existing
`TRAILER_LINE` fixture. The attribution remains in `git log --format=%B`, yet
the full range scan passes with exit 0.

**Suggested correction:** scan stored commit messages in a mode that never
truncates at scissors. Keep the local hook's input-file behavior and test that
separately; add an end-to-end verbatim-commit regression fixture.

### 5. R5 — High: the unsafe boundary accepts effective lint opt-outs

**Locations:** `ci/policy_checks.py:25`, `ci/policy_checks.py:41`,
`ci/policy_checks.py:52`, `Cargo.toml:36`.
**Requirements:** ADR 0001 workspace unsafe prohibition and reviewed adapter scope.

The line regex catches a direct `allow(unsafe_code)` attribute, but not the
equivalent `#![cfg_attr(all(), allow(unsafe_code))]`. The manifest check also
accepts removal of the workspace `unsafe_code` setting, because an empty list
of matches passes. Inheriting `[lints]` does not prove the required lint exists.

**Reproduced:** the direct attribute is rejected; the conditional attribute and
a workspace manifest without `unsafe_code` each return zero policy errors.
An independent Rust probe using the conditional attribute and a real unsafe
pointer read compiles with `rustc --emit metadata -D unsafe_code`, confirming
that this is an effective override, not merely unusual syntax.

**Suggested correction:** require the workspace setting explicitly, parse TOML
structurally, and enforce the non-solver prohibition in a way conditional or
multiline attributes cannot lower. A compiler-enforced `forbid` boundary for
non-adapters is one option; preserve only the explicitly reviewed solver-adapter
exception. Add negative tests for absent settings and conditional/multiline
lint attributes.

### 6. R6 — High: valid YAML write permissions pass the workflow policy gate

**Location:** `ci/policy_checks.py:83`.
**Requirements:** NFR-12 M0 policy review, ADR 0001 hosted-runner hardening.

The checker searches line-oriented text instead of effective workflow/job
permission mappings. Replacing the existing permission block with the valid
YAML mapping `permissions: {contents: write}` leaves no matching write-scope
line, while satisfying the explicit-permissions check.

**Reproduced:** pass that modified workflow to the unchanged
`check_workflows()`; it returns `[]`. The control cases correctly reject an
unpinned checkout action and a `pull_request_target` trigger. A same-repository
workflow could therefore request write privileges while this policy job stays
green; fork token restrictions are not a substitute for the promised policy.

**Suggested correction:** parse workflow structure with an appropriate pinned
YAML parser/validator and inspect effective top-level and job permissions,
including flow mappings. Apply structured checks to triggers, runner labels,
actions, checkout options and secrets rather than extending a syntax blacklist.
Add negative fixtures and run the validator through the trusted-policy path in R1.

### 7. R7 — Medium: the outcome contract forbids an incumbent after cancellation

**Locations:** `crates/optimisation/src/status.rs:3`,
`crates/optimisation/src/status.rs:34`.
**Requirements:** FR-18.AC2, ADR 0003 outcome/cancellation semantics.

`may_have_incumbent()` permits only `Optimal` and `FeasibleTimeLimit`, and the
type documentation states that only those statuses can carry an incumbent.
The ADR explicitly requires independent feasibility checking of any incumbent,
including after cancellation. A solver interrupted after finding a feasible
schedule would lose that incumbent or have to mislabel the termination cause.

**Reproduced:** `SolveStatus::Interrupted.may_have_incumbent()` is false; the
unit test locks in that restriction.

**Suggested correction:** keep termination cause separate from incumbent
presence/validation, or allow an interrupted outcome to retain its incumbent
without asserting completeness/acceptance. Test interruption both before and
after an incumbent, with physical validity checked independently. This is an
M0 contract correction, not a request to implement a worker or solver now.

### 8. R8 — Medium: finite inputs can produce a non-finite known MIP gap

**Location:** `crates/optimisation/src/gap.rs:79`.
**Requirements:** FR-18.AC2, ADR 0003 numerical outcome contract.

Only the inputs are checked for finiteness. Subtraction or division can overflow
after those checks, producing `Gap::Known` with infinite values instead of an
explicit unknown/invalid diagnostic.

**Reproduced against the unmodified source:** minimisation with incumbent
`f64::MAX`, bound `-f64::MAX`, currency unit 1 and tolerance 0 returns
`Known { absolute: inf, ratio: inf }`. With incumbent 0, bound -1 and currency
unit `f64::MIN_POSITIVE / 8`, the absolute gap is 1 but the known ratio is infinite.

**Suggested correction:** validate the derived absolute gap and ratio before
constructing `Known`, and report an explicit unknown reason on overflow. Add
tests for both arithmetic stages and both objective senses. Retain the existing
correct behavior for missing values, wrong-side bounds and negative objectives.

## Additional observations, not merge blockers

- **C1 — Low, `ci/doc_gates.py:29`:** a missing inline-link target is rejected,
  but `[broken][target]` with `[target]: missing.md` passes. Either support
  reference-style links/images and test them, or document the deliberately
  narrower supported subset instead of claiming every relative Markdown link.
- **C2 — Low, committed cache artifacts:** remove
  `ci/__pycache__/doc_gates.cpython-313.pyc` and
  `ci/tests/__pycache__/test_attribution.cpython-313.pyc` from a follow-up patch
  and ignore Python bytecode generally. They are interpreter-specific build
  outputs, and merely running the review tests changes those tracked copies.
- **C3 — Medium, enforcement deployment:** the repository API observed on
  2026-11-13 reports public visibility, `required_status_checks: null`, and admin
  enforcement disabled. `CONTRIBUTING.md:53` should not imply `merge-gates` is
  already a required protected-branch check. The coordinator still needs the
  documented fresh head/base/metadata check, independent evidence and
  adversarial triage. No repository setting was changed by this review.

## Baseline and negative-test evidence

Baseline on the exact reviewed source:

- Versioned hook tests: **130 fixtures passed**, plus the runner's pattern,
  encoding, Git commit/merge and cleanup cases.
- `python3 -m unittest discover -s ci/tests -v`: **10 tests passed**.
- `python3 ci/doc_gates.py`: all six categories passed, 60 Markdown files.
- `python3 ci/policy_checks.py`: all three categories passed.
- `cargo fmt --all --check` and workspace/all-targets clippy with
  `--locked -- -D warnings`: passed on Rust 1.99.1.
- `cargo nextest run --workspace --locked --no-fail-fast`: **21 tests passed**
  across 17 binaries using the existing Zig linker. Workspace doctest commands
  passed; there are currently no doctest examples.
- `cargo deny --locked check` with cargo-deny 0.20.2: all four categories passed.
- Local PR-range scan: all five commits and 2,775 added lines passed; the
  supplied PR title was scanned. Live title/body evidence came from the GitHub
  attribution job rather than a fabricated local body scan.
- GitHub checks for this head: all eight workflow jobs observed successful;
  CodeRabbit was pending at inspection. Green checks do not negate the
  reproduced false negatives.

Negative controls that **did** fail as intended:

- Unpinned action and forbidden `pull_request_target` trigger.
- Direct `allow(unsafe_code)` outside a solver adapter.
- Broken inline Markdown link, absent requirement decision source and duplicate
  requirement heading.
- Ordinary matching added footer and the existing suite's attributed commits,
  merges, PR metadata, invalid patterns and attempts to weaken base patterns.
- GPL-3.0-only dependency under the original licence allow-list.
- `paste` 1.0.15 with its unmaintained advisory: cargo-deny rejected it with
  exit 1. In the pinned version, `unmaintained = "all"` is **not** a silent
  warning-only bypass; no advisory-severity defect is alleged here.

False-negative controls: R1–R6 and C1 above. Rust semantic probes: R7–R8.
The scratch tests exercised the shipped functions and CLI drivers; they do not
claim to emulate GitHub branch protection or a provisioned target runner.

## Requirement and milestone fit

- **M0 / D-72:** the Cargo layout, minimal domain types, capability descriptions
  and unavailable adapter stubs are in scope. `egs solvers` is a small contract
  demonstrator, not premature simulation logic. No broad scope reduction is
  warranted. Remove the accidental bytecode, not the intended workspace.
- **NFR-3 / ADR 0001:** SHA-pinned action references, read-only current workflow,
  edited-metadata trigger, full-history attribution checkout, disabled credential
  persistence and `always()` aggregation are good starting points. Aggregation
  rejects non-successful declared dependencies. The base driver/matcher/pattern
  reads are real; missing base files use an explicit seed warning, not an
  undisclosed trust claim. R1–R6 prevent treating these as complete gates.
- **NFR-8:** the present locked graph passes the proposed licence/advisory rules;
  unknown sources and named Python/Julia bindings are denied. R1 is the policy
  approval weakness. Native SDK, container, frontend and offline-reference
  inventories become applicable when those artifacts exist; their absence is
  not a reason to invent M0 dependencies or claim future inventory completion.
- **NFR-10:** production code is Rust; Python/Perl are development policy tools.
  All solver crates currently link no SDK/runtime, so the default build needs
  neither CUDA nor commercial licences. PostgreSQL/Valkey deployment is not an
  M0 implementation deliverable.
- **NFR-12:** the actual workflow has no `pull_request_target`, write permission,
  self-hosted runner, privileged cache or licence secret. The policy correctly
  defers target provisioning, denial/isolation tests and device validation to
  M1. Those operational tests must not block M0. R6 concerns enforcement of
  that stated policy, not an existing credential leak.
- **FR-18 / ADR 0003:** opaque non-nil IDs, exact licence tags/tainting, native
  currencies, explicit unavailable errors, no silent LP relaxation and
  vendor-neutral declarations are appropriate. Planned capabilities do not
  claim a linked solver is available. The sparse IR, load/solve/basis/cancel
  implementation and real HiGHS conformance remain later-stage work, as stated
  by the stubs. Fix R7–R8 now so later adapters do not inherit the wrong contract.
- **Profile limitations:** numerical, benchmark, mutation, frontend and GPU N/A
  reasons are accurate for this exact M0 tree. The hard-coded reasons are not
  a future trusted classifier and must be replaced before those targets exist.
  Manual independent review is an explicitly disclosed transitional procedure,
  not completed automated review-attestation enforcement.

## Final disposition

**Verdict: must-fix.** Must-fix items are:

1. R1: trusted policy/gate-graph and licence-rule evaluation, including reviewed
   bootstrap/policy-change handling.
2. R2: source-comment/header attribution detection.
3. R3: attribute-independent added-text scanning.
4. R4: complete scanning of stored commit messages after scissors markers.
5. R5: effective non-adapter unsafe enforcement and a mandatory workspace lint.
6. R6: structural workflow permission validation with negative fixtures.
7. R7: retain independently validated incumbents on interrupted outcomes.
8. R8: reject non-finite derived gap values.

Re-run the baseline suite and these negative cases on the corrected head, then
obtain current-head independent re-review and coordinator adversarial triage.
Do not interpret this report or the existing green aggregator as merge-ready.
