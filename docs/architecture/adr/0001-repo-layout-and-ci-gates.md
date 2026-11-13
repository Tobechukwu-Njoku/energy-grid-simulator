# ADR 0001 — Repository layout and staged CI gates

- **Status:** Proposed; bootstrap policy is specified, not deployed by this ADR.
- **Date:** 2026-11-13
- **Linked decisions:** D-3, D-8, D-16, D-17, D-19, D-20, D-22, D-28, D-29, D-31, D-43, D-48, D-51–D-54, D-56, D-58, D-61, D-62, D-65.
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
D-16; no Python/Julia runtime model dependency. PostgreSQL 18 and Redis remain the
baseline; service roles/TimescaleDB adoption are not decided here (ADR 0004 deferred).
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
the exact code/doc head they reviewed. A later committed evidence-only change may refer
to that ancestor only if a base-policy validator proves the entire intervening diff is
review-evidence metadata and unchanged reviewed-tree hashes. For the simplest path,
use a structured PR comment on the final SHA. Any substantive edit invalidates approval.
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
