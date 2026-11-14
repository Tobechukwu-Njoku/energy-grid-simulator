# ADR 0001 — Repository layout and staged CI gates

- **Status:** Accepted 2026-11-13 (D-72). Performance figures remain to be qualified on the D-65 target system.
- **Date:** 2026-11-13
- **Linked decisions:** D-3, D-8, D-16, D-17, D-19, D-20, D-22, D-28, D-29, D-31, D-43, D-48, D-51–D-54, D-56, D-58, D-61, D-62, D-65, D-73, D-74.
- **Related requirements:** FR-4, FR-13–FR-19; NFR-1–NFR-4, NFR-7–NFR-12.

## Context and authority

Requirements are not baselined. Only the D-31 data spike and the explicitly requested
versioned commit-message policy hook are executable additions permitted in this
revision; no workspace, engine, solver experiment or application CI is scaffolded.
A green documentation check or this Proposed ADR cannot open the code gate.

D-28 quality gates remain mandatory when applicable. They must not require a GPU
implementation, numerical fixtures or a benchmark baseline before any exist. D-58
chooses a public repository after open PRs merge, attribution history is cleaned and
all branches/history pass restricted-data/secret scans. Until that transition and
verified required-check protection, merge discipline is NOT enforced branch protection.
This revision neither flips visibility nor rewrites shared history.

ASSUMPTION (Q-N-11, alias OI-11): Use a staged, exact-head gate manifest to prevent
accidental omissions by cooperative contributors sharing one account. It cannot prove
provider identity, prevent a malicious owner-token holder bypassing policy or establish
scientific validity from review independence alone. No external attestor/App is needed.

## Proposed workspace after the baseline gate

Use edition 2024, a pinned reviewed MSRV/toolchain in rust-toolchain.toml, committed
Cargo.lock, shared workspace.dependencies and workspace.lints. Deny unsafe_code except
in individually reviewed solver-* adapters. CPU defaults never enable CUDA/licensed SDKs.

| Path | Responsibility |
|---|---|
| crates/domain, crates/scenario | IDs/units/contracts and canonical versioned TOML identity |
| crates/ingest | Sourced/dated inputs, lineage, licence tags and calendar/GSP alignment |
| crates/network | Topology, DC equations, sparse factors, screened flowgates/PTDF/LODF |
| crates/market | Date-effective GB/I-SEM rules, products, proxies and losses/FX policy |
| crates/optimisation | Sparse model IR and solver trait (ADR 0003), no market rules |
| crates/solver-highs | Default CPU native adapter and isolated worker support |
| crates/solver-gurobi, crates/solver-xpress, crates/solver-cuopt | Optional adapters; contracts/stubs without SDK linkage by default |
| crates/commitment, crates/redispatch, crates/pricing | Separate staged UC, network actions and national pricing |
| crates/engine | Orchestration, worker lifecycle and rolling/persisted states, not every model |
| crates/api, crates/sdk, crates/cli | Shared scenario/result semantics and interface composition |
| apps/web, schemas | TypeScript dashboard and versioned shared schemas |
| tests/cases, tests/reference, tests/acceptance | Hashed cases, uv-locked offline reference environment, API/UI/backtest tests |
| benchmarks, ci, .github/workflows | Frozen workloads, trusted classifiers/evidence and future workflow entry points |
| .githooks | Versioned local attribution policy and commit-msg hook |
| spikes | D-31 throwaway data work, never a production dependency |

Reuse mature solvers, cargo-nextest, cargo-deny, cargo-mutants and native build caches;
do not implement a new optimisation framework. Native libraries are permitted under
D-16; no Python/Julia runtime model dependency. PostgreSQL 18 + Valkey is the baseline
(D-67). Clients use only the shared Redis protocol, keeping Redis a compatible
alternative; P1 job dispatch uses PostgreSQL. Remaining storage details/TimescaleDB
adoption belong to proposed ADR 0004.
Software/service terms and dataset permissions are reviewed separately.

## Bootstrap profiles and transitions

ASSUMPTION (Q-N-11): A trusted policy from the base branch determines applicable gates
from changed paths, repository state and declared installed capabilities. Authors cannot
select a cheaper profile with a label, missing files or a changed classifier in their PR.
A classifier/policy change is independently reviewed and tested against old and new rules.
Every gate emits pass/fail or explicit N/A with reason and evidence; missing is not N/A.

| Profile | Applicable quality evidence | Explicit non-applicability / transition |
|---|---|---|
| B0 — current documentation/data spike | Traceability and Markdown/local-link checks; spike tests if present; D-52, secrets/data/provenance, current-head independent review and adversarial triage | Rust/TS fmt/lint/build/reference/bench/GPU are N/A only while their implementation targets do not exist. A spike change runs its own applicable checks. |
| B1 — CPU implementation (M1 target-runner activation onward, after baseline) | fmt, clippy, nextest, licence/advisory closure, numerical reference, mutation scope, CPU benchmark, Linux CPU and applicable macOS build, plus all B0 policy gates | A new component must seed applicable tests/reference evidence in its first implementation PR. No blanket first-code exemption. GPU N/A without an enabled/touched GPU adapter. |
| B2 — optional native/GPU feature | All B1 gates plus changed-adapter conformance, actual-device numerical/benchmark evidence and licence/device checks | Missing REQUIRED supported device/runner blocks the feature PR; CPU-only P1 is not blocked by an absent optional adapter. |

First B0 activation: coordinator pins the exact candidate SHA and checks the checklist
manually with recorded commands/results plus independent and adversarial dispositions.
That narrowly permits a reviewed policy/workflow seed when no workflow yet exists; it
does not label unrun CI green. Later workflow activation uses the trusted baseline policy.
The first B1 benchmark run has an explicit reviewed 'seed: no prior implementation'
disposition, stores measurements against the exact merged SHA, then activates comparisons
for every subsequent relevant PR. Seeding requires an independently reviewed frozen
workload, not a claim that a missing baseline is a comparison pass. New workload seeds
and existing baseline resets are separate PRs from changes measured against them.
No bootstrap exception waives the requirements-code gate or D-28 independent review.

## Gate graph and quality criteria

Use pull_request opened/synchronize/reopened/edited/ready_for_review events and a
fresh base/head check immediately before merge. Title/body edits invalidate metadata
policy evidence even if code SHA is unchanged. Policy/review-only rechecks can be light;
never reuse numerical evidence for a different code/tree/input/runner identity.
One merge-gates aggregator runs with always() and fails if any REQUIRED gate is not
successful; a protected classifier's explicit N/A is different from a skipped job.

| Gate | Evidence / failure condition |
|---|---|
| format/lint/build | cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; pinned TypeScript formatter/linter when present; explicit CPU/optional-feature matrix |
| tests | cargo nextest run for workspace CPU targets, API/CLI contracts, UI E2E/accessibility and domain negative cases; failures or missing targets block |
| licence/advisory | cargo deny check plus native solver, service/container, npm and offline-reference inventories; unreviewed licence/advisory exceptions block |
| numerical-reference | Identical-input independent fixtures with NFR-2 tolerances; exact tests use matched solver feasibility and relative MIP gap ≤1e-6 plus tighter absolute certificates; no fabricated BM reference |
| benchmark-regression | NFR-4 paired serial-runner protocol, full model/solver/hardware identity; proposed >5% wall or >10% RSS significant-regression floor against BOTH protected main and the frozen last milestone/release; report trends below floors and unresolved above-floor uncertainty as blocking |
| reference-integrity/mutation | External fixture ownership, provenance and mutation policy below |
| provenance/secrets/data | Research-only raw/derived restricted data never committed; ODbL/share-alike excluded; manifests tagged open-commercial/research-only and publication preconditions scanned before D-58 visibility change |
| attribution-policy | PR-range messages, metadata and added-line checks below; no silent rewriting by CI |
| independent-review | Current-head role-compatible independent evidence plus adversarial findings and verified triage |
| gpu-validation | Actual installed supported device for relevant implemented feature, otherwise protected-classifier N/A |

Protect main with a single required merge-gates check and strict up-to-date branch
requirement after D-58 preconditions permit it. Use required status evidence, NOT
GitHub approving-review counts: a shared owner identity cannot approve its own PR.
No merge queue or merge_group path is assumed. Before protection is available, the
coordinator's exact-head check/merge procedure is documented discipline only. A future
versioned ci/merge.sh must re-query head/base and gates immediately before merging;
it is not implemented or claimed unbypassable in this documentation revision.
D-53 push frequency is a working rule, not a CI-verifiable fact.

## Review evidence and protection against trivial tests

Review evidence lives in a review report or structured PR comment with artifact-kind,
author-role/provider, reviewer-role/provider, reviewed-head-sha, verdict, finding
IDs/dispositions and evidence source. Role/provider names are confined to process/review
records (D-52); the role matrix is defined by docs/process/team.md, D-51, D-54 and D-62.
Check both differing providers AND compatible author/reviewer roles. Data gathering
requires verified output; adversarial review alone never supplies independent approval.
D-53 triage by the designated verifier/coordinator is required in addition to review.

Shared-identity evidence is attestation, not cryptographic proof. Review reports bind
the exact code/doc head they reviewed. Review evidence lands on `main`, never on the
pull-request branch: every tree entry the pull request owns must be unchanged between
the reviewed commit and the head (a merge of `main` is allowed). No directory is exempt,
because a file under the reviews directory can be included in compiled code (R3-03).
Any substantive edit invalidates approval.
A same-PR reviewer-authored approval file cannot prove its own independence.

Reference fixtures in tests/cases and tests/reference have source, licence, checksum,
expected-result derivation and originating PR. An author's change to expected outputs
requires a separate reviewed fixture PR OR explicit independent reviewer sign-off
explaining the changed oracle; routine approval is insufficient. Tests with no nontrivial
failure assertions do not satisfy reference agreement. Deliberately perturb a cost,
constraint, unit conversion, loss and rolling state to prove the oracle detects them.
ASSUMPTION (Q-N-11): Run cargo-mutants on changed commitment/redispatch/network/pricing/
optimisation logic; zero unexplained surviving or timed-out non-equivalent mutants.
Equivalent mutants require individually recorded reviewer justification; expand nightly
scope instead of hiding changed-core survivors in a percentage budget.

## Attribution policy and versioned local hook (D-52 / D-61)

Enable the committed hook explicitly in each clone (worktrees share common config):

    git config core.hooksPath .githooks

.githooks/commit-msg reads the shared .githooks/attribution-patterns.regex using Perl
(bootstrap prerequisite). It removes whole-line identity trailers and attribution
footers from the body; it NEVER removes the subject. A matching subject aborts with
an actionable diagnostic and leaves the entire message unchanged. Whole-line anchoring
preserves ordinary technical prose and robot-icon descriptions. Complete known model/
tool identities or known service/bot emails identify trailers; a human surname or
`google.com` alone does not. All `*-by:` keys, `*-with:` pairing keys, Unicode hyphens,
quoted trailers, creation verbs and the wider model/tool families have fixtures.

Decode both files strictly as UTF-8 before producing output. Malformed bytes, invalid
regexes, an empty rule set or an empty-string-matching pattern fail without rewriting
the message. Ignore blank/comment pattern lines and accept CRLF pattern files. Preserve
retained bytes/blank lines and the Git scissors suffix; ordinary Git empty-message rules
still apply. --no-verify is not a CI waiver. Run `python3 .githooks/tests/run.py` for the
committed review fixtures, failure cases and scratch-Git commit/merge/encoding checks;
Python is a test-only prerequisite. The runner exercises paths containing spaces and
temporary-file cleanup. It needs no application build, database or target-system access.

The future attribution-policy CI check uses the SAME committed regex list, Unicode
handling and fixtures, but REJECTS matches rather than deleting them. It scans:

1. Every commit message in base..head, including merge commits. Process-file exemptions
   never exempt commit messages, PR titles or agent-written body text.
2. The current PR title/body, on edited as well as synchronize events. D-61 exempts only
   the exact bot-managed summary block with verified bot edit provenance; do not disable
   or remove that integration. Spoofed markers or an unknown editor do not gain an
   exemption; human/agent-written surrounding text remains checked.
3. Added text lines across the diff, including release notes, code comments and headers.
   Process-policy explanations in AGENTS.md, docs/process/**, requirements questionnaire/
   decisions/reviews and these governance ADR sections may name the policy/roles;
   exemptions are narrowly path/section-scoped by trusted policy, not all docs/**.
   Pattern literals in the hook/regex and test fixtures are syntax/test data, not an
   authorship claim; only those literals are exempt, never their surrounding banners.

The check rejects attribution, not code that merely lacks attribution. Scientific
correctness comes from independent references, mutation and review, not watermark lint.
GitHub composes squash commits outside the hook and pre-merge message scan. Pin
`squash_merge_commit_message = COMMIT_MESSAGES` (verified via repository API on
2026-11-13); never use PR_BODY, which could copy a D-61-exempt bot summary into a
non-exempt commit. With the current `squash_merge_commit_title = COMMIT_OR_PR_TITLE`,
scan both candidate subjects; immediately before merging check settings, title and
all messages again. The future main-push policy scan checks the actual merge/squash
commit using the same message rules. A post-merge violation fails the check, blocks
publication/further merges through the coordinator procedure and requires a reviewed
correction; a pre-merge pass is not proof of a clean final commit. This ADR specifies
these checks without changing repository settings or claiming a deployed workflow.

Existing unclean branch history is reported, not rewritten by the hook or hidden by
scanning only the tip. Coordinator-owned D-58 cleanup is still required before publicity.

## Runner security, throughput and spend

D-65 separates coding/coordination from execution. Heavy builds, databases,
CPU/reference/benchmark jobs and simulator runs execute only on the separate target
system, NEVER in the coding LXC container or nested job containers inside it. Trusted
exact-SHA dispatch queues heavy Linux jobs to an explicit target-runner label; an
absent/misidentified target blocks execution, without a local fallback. The coding
container may edit/review docs and run lightweight hook/structural checks only.

ASSUMPTION (Q-N-7/Q-N-11/Q-N-12): Provision a dedicated unprivileged target-runner
account and ephemeral rootless per-job containers. Target hardware/OS/access details
remain owner-supplied; D-48's 5900X/128 GB is a provisional sizing assumption, not
confirmation that the target is the D-48 host or the coding container.
No Docker socket, SSH keys, provider credentials or solver licence secrets are mounted.
The runner token is inaccessible to job containers; network and filesystem access are
restricted. Untrusted fork PRs never execute on this runner. Do not use
pull_request_target to run PR code. A trusted exact-SHA dispatch is required before
self-hosted execution; without safe runner isolation the applicable job blocks.
Rootless containers reduce accidental exposure; they do not defend against a malicious
host admin, compromised kernel or the shared owner identity.

Separate GPU jobs require supported actual hardware and tested isolation/toolkit first;
no disposable GPU-passthrough VM is a prerequisite for CPU P1. No cuOpt implementation
is scheduled in P1. No licence/device availability is inferred from a planned purchase.
M0 reviews this policy only. M1 provisions the separate target runner and executes
NFR-12.AC1/AC2 before the first heavy B1 job: unauthorised/stale SHA and fork dispatch
rejection, denied canary credential reads/socket mounts, missing-required-runner
behaviour, untrusted policy edits and spoofed review/summary metadata. This is a
bounded configuration/access-denial suite, not a host-escape exploit or a request to
expose real SSH/licence secrets. A failed suite keeps that runner disabled; M3 adds
service/exposure checks. No operational penetration evidence blocks M0 baseline.

Use sccache and native HiGHS build caches keyed by toolchain/platform/source/flags.
Never use writable untrusted caches in privileged jobs. Run macOS CPU builds on main
pushes/scheduled runs, and require them before merge for native/build/toolchain/FFI
changes; schedule a full CPU portability check for releases. Numerical-core changes
require Linux reference evidence; cached results must match tree and input hashes.
Run slow mutation suites on the D-65 target system, and serialize paired benchmarks
there against both main and the frozen milestone/release baseline (NFR-4).
No specific hosted-minute allowance is assumed; verify current billing/visibility
before activating hosted matrices. Report utilization and stop unapproved paid usage,
not quality checks. Resource cost does not justify a lower-fidelity model (D-2).

## Acceptance and limitations

Before accepting this ADR: approve/adopt Q-N-11 assumptions, validate B0 applicability,
exercise hook/CI pattern parity, seed benchmark/review policy without circular gating,
and record the remaining enforcement deployment steps. After baseline, demonstrate
that failing/stale/missing applicable gates prevent the merge procedure, a doc-only
PR reports honest N/A, a CPU change cannot skip numerical tests, and only a relevant
GPU feature change needs the device gate. No current workflow or protected-main
configuration is claimed by this ADR. Real publication and STATUS updates remain the
coordinator's responsibilities, not edits made by this revision.

## Primary references

- GitHub branch protection: https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches
- GitHub event and workflow syntax: https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows
- GitHub self-hosted security: https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions
- Cargo workspace policy: https://doc.rust-lang.org/cargo/reference/workspaces.html
- Git hooks and core.hooksPath: https://git-scm.com/docs/githooks

References identify implementation specifications, not evidence that those controls
are deployed in this repository. Verify pinned tooling and plan capabilities at activation.

## Addendum — 2026-11-13: trusted gates and owner approval (D-74, PR #23 revision 3)

This addendum records how the M0 implementation meets this ADR after the PR #23
reviews (F1, F2; ADV23R-08). It adds no decision beyond D-73 and D-74.

- **Trusted entry point.** `independent-review` and `policy-approval` run from
  `.github/workflows/trusted-gates.yml` on `main` via `pull_request_target`. That
  workflow builds `main`'s xtask in a checkout of `main` and fetches the pull-request
  head only as Git objects; it never checks out, builds or runs pull-request code,
  holds no secrets and has a read-only token. "Do not use pull_request_target to run
  PR code" above still holds. Both gates compare against the current tip of `main`.
- **Pull-request jobs.** Build, test, cargo-deny and the xtask policy jobs read their
  policy from the current tip of `main` and fail closed when `main` has no policy.
  Their workflow and action files are defined by the pull request, but they are
  protected paths: the trusted `policy-approval` gate fails for any change to them
  without the owner's exact-head approval, and rejects a pull-request job that
  reuses a trusted gate's name.
- **Bootstrap.** The only exception to failing closed is the first B0 activation
  described above: an exact head SHA the coordinator attests on `main` in
  `docs/process/ci-bootstrap.toml` (`seed_sha = "<40 hex>"`). An absent policy is
  never a fallback.
- **Owner approval (D-74).** Agents push and open pull requests from a separate
  machine account (`agent_account` in `xtask/policy/repository.toml`; placeholder
  until created). The owner approves from `Tobechukwu-Njoku`; an APPROVED review on
  the exact head is the policy-approval and code-owner evidence. For policy approval
  this replaces "a shared owner identity cannot approve its own PR" above; required
  status checks remain the merge gate. A policy change opened by the owner cannot
  pass, and the gate says so.
- **Review evidence.** The role/vendor matrix of D-28, D-51, D-53 and D-54 is the
  trusted file `xtask/policy/review.toml`. Every adversarial finding needs a closing
  disposition in a current triage record (format in `docs/requirements/REVIEW-BRIEF.md`).

Residual risks, not closed by code:

1. Until `trusted-gates.yml` is on `main`, no CI result for the bootstrap pull request
   is independent of it; the coordinator's manual exact-SHA check is the control.
2. Pull-request jobs are trustworthy only while branch protection requires the
   trusted gates and `merge-gates`. Configuring that is an owner action; no
   repository setting is changed by this addendum.
3. GitHub may show a spoofed status beside a trusted one of the same name. The
   trusted gate fails in that case, but the coordinator confirms that the passing
   checks come from the `pull_request_target` run of `trusted-gates.yml`.
4. `pull_request_target` does not run on review events: after approving, the owner
   re-runs the trusted-gates run. A compromised owner or machine-account token
   defeats these controls.

## Addendum — 2026-11-13: compiler-enforced unsafe boundary (PR #23 revision 6)

Revisions 4 and 5 tried to prove statically which Rust files Cargo compiles; each
round found another macro, include, glob or patch form the walker missed (R5-01..04,
ADV23R5-02..05). That approach is withdrawn. It adds no decision beyond D-73.

- **What is built** comes from `cargo metadata --locked --all-features` (resolved
  members, targets, path dependencies, patches, build-dependencies), never from our
  own reading of manifests or globs. `cargo xtask policy` keeps the cheap static
  rules on that graph: every target root of a local package outside
  `crates/solver-*` starts with `#![forbid(unsafe_code)]`; local packages and
  targets are tracked files in the repository; dependencies come from crates.io or
  the repository; manifests and Cargo config as before.
- **The unsafe boundary is enforced by the compiler.** `cargo xtask build-gate`, a
  step of the unprivileged `policy` job, builds the pull request's workspace (all
  targets, all features, build scripts and proc macros) with `main`'s xtask binary
  as `RUSTC_WRAPPER`. For every crate whose package is in the repository it removes
  `--cap-lints`, refuses argument files and appends `-F unsafe_code` (library and
  test targets of `crates/solver-*` adapters excepted; their build scripts are not).
  A command-line `forbid` cannot be lowered by any attribute, `[lints]` entry or
  macro expansion. Cargo runs from the trusted tree, so no repository Cargo config
  is loaded, and every `CARGO_*`/`RUSTC*`/`RUSTDOC*`/`*RUSTFLAGS` variable is
  removed from its environment.
- **Native runtimes (NFR-10)** are checked on the build's output: the wrapper also
  sets `-C linker=<xtask>`, a guard that refuses any linker input naming Python or
  Julia before running `cc`; Cargo's `build-script-executed` messages are read for
  `rustc-link-lib` requests and link-search directories holding such libraries.
- **Workflows** may not set `CARGO_*`, `RUSTC*`, `RUSTDOC*`, `RUSTUP_*` or
  `*RUSTFLAGS` variables unless `allowed_env` in the trusted `gates.toml` lists
  them, nor write `$GITHUB_ENV`/`$GITHUB_PATH` (ADV23R5-01).

Residual risks of this approach:

5. The build gate runs the pull request's build scripts and proc macros. They run
   as the job's user and could tamper with that job (for example replace the
   wrapper binary for later units). The job is unprivileged (read-only token, no
   secrets) and is never a trusted gate; build-time code is reviewed like any
   other code, and the root `forbid` rule still holds in every ordinary build.
6. Only code that is compiled for the CI host is checked by the compiler: code
   behind another target's `cfg` is covered by the root `forbid` rule (which
   applies whenever it is compiled) but not by the gate itself. *Corrected in the
   revision-7 addendum:* doctests do NOT inherit the root `forbid` (R6-04), and
   all-features builds omit code that a feature switches off (R6-03).
7. A native library linked statically from a `#[link(kind = "static")]` attribute
   is bundled into the rlib without a linker flag. It is caught only when its
   search path comes from a build script; Cargo config and `RUSTFLAGS` cannot add
   one in the gate.

## Addendum — 2026-11-14: build-gate attribution, linker allowlist, features, doctests (PR #23 revision 7)

Round-6 reviews (R6-01..04, ADV23R6-01..04) found false negatives in the revision-6
build gate. The compiler-enforced design stays; these changes close the gaps. It adds
no decision beyond D-73 and D-74.

- **Reviewed adapters are an explicit list (ADV23R6-01).** `[[adapter]]` entries in
  the trusted `xtask/policy/repository.toml` name each adapter's exact Cargo package
  AND its directory; both must match. A `crates/solver-*` directory confers nothing.
  The file is under the protected `/xtask/` path, so adding or changing an adapter
  needs the owner's exact-head approval (D-74). Adapter build scripts are never exempt.
- **Wrapper attribution by the package graph (R6-01).** The gate writes the resolved
  packages from `cargo metadata` (package id, canonical manifest directory, name,
  local or registry, adapter or not, each target's canonical source and kind) to a
  configuration file. The wrapper attributes each rustc compilation by Cargo's
  `CARGO_MANIFEST_DIR` and `CARGO_PKG_NAME` and by the source file on the command
  line; a compilation it cannot attribute is refused. A crate name decides nothing;
  Cargo's stdin target-information probe gets the strictest flags and only a bare
  `rustc -vV` passes unchanged. A build script's `cargo:rustc-env` naming a
  toolchain, Cargo, `XTASK_*`, `PATH`, loader or temp-directory variable is a finding.
- **Linker allowlist (R6-02, ADV23R6-02).** The linker guard (a copy of `main`'s
  xtask) parses the C-driver command line, `-Wl,` lists and `-Xlinker` values. It
  refuses every response file (`@file`, also forwarded), linker script, plugin,
  `-specs`, `-B`/`-fuse-ld=` outside the toolchain, and accepts only: library and
  framework names in `link_libraries`/`link_frameworks` of the trusted `gates.toml`
  (every `-l`, `--library`, `-l:`, `-framework`, weak/needed/re-export/lazy/upward
  form), and input files Rust produced (rlibs and codegen objects in the build's
  `deps`/`build/<unit>` directories, rustc/rustdoc work directories, the toolchain
  sysroot). A library a build script writes into `OUT_DIR` is refused whatever its
  name. Python and Julia names are reported as NFR-10 explicitly.
- **Feature configurations (R6-03).** The gate builds default features, all
  features, no default features, and each feature of each workspace member alone on
  top of no default features (bounded at 256 configurations; beyond that it fails
  closed). *Bound:* code compiled only under a combination of two or more
  non-default features that all-features also excludes is not built. Such code is
  still subject to the root `forbid` in any build that compiles it.
- **Doctests (R6-04).** rustdoc compiles each doctest as its own crate, which
  neither the wrapper nor the crate's root lint reaches (`RUSTDOCFLAGS=-F` does not
  either). Every non-adapter library root must carry
  `#![doc(test(attr(forbid(unsafe_code))))]` (`cargo xtask policy`), and the build
  gate runs `cargo test --doc` in every feature configuration in its sanitised
  environment, so that attribute is applied by the compiler; doctest executables
  link through the same linker guard.
- **File commands (ADV23R6-03).** Guarantee: the build-gate step, the only step of
  a gate job that runs pull-request code, must be the job's last step, without an
  `id`, in a job without `outputs` (`cargo xtask policy`), and the gate removes the
  file-command variables from the environment it builds in. Whatever PR code writes
  to `$GITHUB_ENV`, `$GITHUB_PATH`, `$GITHUB_OUTPUT` or `$GITHUB_STATE` therefore
  reaches no later step except action post-steps, and a later step can only fail a
  job, never turn its failure into success. Every earlier step runs `main`'s code.
  Workflow and action files are protected paths. The script lint (no file-command
  name outside `allowed_file_command_lines`, compared without case or quoting; no
  indirect variable access) is defence in depth, not the guarantee: shell text
  cannot be decided.
- **Provenance path rules ignore letter case (ADV23R6-04)**: `raw/`, `__pycache__/`,
  Python and data extensions and research-only record names.

Residual risks (in addition to 5 and 7 above, which stand):

8. The bound in the feature bullet above: multi-feature-only code is not compiled by
   the gate.
9. The link allowlist trusts files under the build's `deps` and `build/<unit>`
   directories and rustc work directories by location. A build script that writes
   outside its `OUT_DIR` into those directories is the same-user tampering of
   risk 5, not a normal build-script output.
10. If rustc must fall back to a linker response file for a very long command line,
    the gate fails closed; the supported fix is a shorter link line, not a parser.

## Addendum — 2026-11-14: library resolution, production feature builds (PR #23 revision 8)

Round-7 reviews (GPT R7-01..03, Gemini ADV23R7-01..08) found false negatives in
revision 7. The design stays; these changes close the gaps and correct risk 8. It
adds no decision beyond D-52, D-56, D-73 and D-74.

- **Libraries are allowlisted by the file the linker opens (R7-01).** An allow-listed
  name is not enough: `-liconv` with a build script's `OUT_DIR` on the search path
  opened an `OUT_DIR/libiconv.so` linker script whose `INPUT` was an archive named
  like the Python runtime. The guard now resolves every allow-listed `-l` name (and
  `-l:file`, `--library`, the weak/needed forms, and frameworks) through the link
  line's `-L`/`--library-path`/`-F` directories at both the driver and linker level.
  A candidate file (`lib<name>.so`, `.a`, `.dylib`, `.tbd`, ...) in a search
  directory that is not *trusted* is refused, whatever it contains. Trusted means
  the toolchain sysroot, the C driver's own library directories (`cc
  -print-search-dirs`, in the guard's sanitised environment) or a
  `link_search_roots` entry of the trusted `gates.toml` (empty); the build directory
  and the temporary directory never are. The file found in the trusted directories
  is inspected: a thin archive is refused, and a linker script (glibc's `libc.so`,
  `libm.so`, libgcc's `libgcc_s.so` are scripts) may only use `GROUP`, `INPUT`,
  `AS_NEEDED` and `OUTPUT_FORMAT`/`OUTPUT_ARCH`/`TARGET`; each input must be an
  allow-listed `-l` name resolved the same way, an absolute path in a trusted
  directory, or a relative name not shadowed by the linker's working directory or an
  untrusted search directory. Every other linker script is refused, as before, by
  option (`-T`, `--script`, ...) and now also by content: a direct input in Rust's
  own output places must be an object or archive, not a script. Sysroot-relative
  search paths (`=dir`, `$SYSROOT`), `--sysroot`, `-Y` and untrusted `-rpath-link`
  directories are refused.
- **Production builds are separate configurations (R7-02).** Cargo resolves
  features per command: building tests, examples or doctests activates
  dev-dependencies, which unify their features into the package, and
  `--workspace` unifies the features that one member enables in another. Each
  configuration is now built three times: production targets (`cargo build`:
  library and binaries, no dev-dependencies), all targets, and doctests. Besides the
  workspace with default/all/no default features, every member is built alone with
  its default features, with no default features (when it has a `default`), and
  with each of its features alone.
- **Gate identities compare case-insensitively (ADV23R7-01).** A pull-request job
  without `name:` is named by its id; ids and names are compared with the trusted
  gate names ignoring ASCII case and surrounding space, and a computed or non-string
  name fails closed.
- **Data by content (ADV23R7-02).** Compressed, archive, columnar and database
  extensions were added (`.zst`, `.lz4`, `.xz`, `.bz2`, `.7z`, `.br`, `.rar`,
  `.avro`, `.duckdb`, ...), and the provenance gate also detects data by its leading
  bytes (gzip, zstd, lz4, xz, lzma, bzip2, 7z, zip, rar, tar, Parquet, Arrow, Avro,
  HDF5, NetCDF, GRIB, SQLite, DuckDB, NumPy, pickle, OLE2, PDF, MATLAB, shapefile)
  whatever the file is called.
- **Interpreters by command family (ADV23R7-03).** Workflow scripts may not run any
  command of a Python, Julia or other script-interpreter family (`python*`,
  `pythonw`, `pypy*`, `pip*`, `uv`, `conda`, `julia*`, `node`, `perl`, `ruby`,
  `Rscript`, ...) with any version or variant suffix, from any directory, or a script
  file of those languages (D-73, NFR-10).
- **Attribution (R7-03, ADV23R7-04..08)** stays one shared pattern file: prompting
  and guidance verbs, assistance nouns (input, guidance, suggestions,
  contributions, ...), Markdown summary headings, HTML comment markers that credit a
  tool, review-bot, v0/Vercel and Augment names and domains, Continue and Supermaven
  domains, and generic coding/code/programming/review assistants, agents and bots.
  Human co-author controls are unchanged.

Residual risk 8 is replaced:

8. Code compiled only under a combination of two or more non-default features of
   one member, that neither that member's default features nor all features
   compile, is not built by the gate (single features, defaults and dependency
   unification are). Such code still meets the root `forbid` in any build that
   compiles it.
11. The guard trusts the C driver's own and the sysroot's library directories by
    location. A build script that writes there is the same-user tampering of risk 5.
