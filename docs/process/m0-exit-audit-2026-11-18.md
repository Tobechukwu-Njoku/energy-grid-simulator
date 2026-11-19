# M0 exit audit — 2026-11-18

**Verdict: M0 is partial, not complete.** The documentation/workspace bootstrap,
light CPU checks, attribution rejection, dependency checks and solver contract are
on `main`. The remaining closure work is principally review-evidence publication,
exact-head merge discipline and deployment of required checks. M0.5 has useful
acquisitions but incomplete network/weather coverage. M1 has inventories and
skeleton crates, not validated ingestion or an activated target runner.

This is a research audit, not an approving review, bootstrap attestation or merge
authorization. Statuses below mean **done / partial / missing at the allocated
milestone**, not completion of the whole P1 requirement. The exit checklist is
[06-roadmap.md:36](../requirements/06-roadmap.md#L36), interpreted with the
[staged requirements:364](../requirements/04-requirements.md#L364).

## Snapshot and method

- Audited `main` at `0efa30f720dcb7b596b2f6bff5f9b57a1e9be8d0`, fetched on
  2026-11-18 UTC. Source line references describe that revision.
- Read [AGENTS.md](../../AGENTS.md), requirements, roadmap, STATUS, accepted ADRs,
  `xtask/src/`, policy files, tests and both workflows. Queried live PR metadata,
  check results and failure logs for #53, #57 and #75. PR state can change after
  this snapshot; refresh it before merging.
- The starting checkout contained #75's unmerged commit. This audit branch was
  created from `origin/main` so it does not accidentally ship that policy change.
- Repository API confirms **public** visibility and `main.protected = true`.
  The branch response exposes empty required-check contexts/checks with
  `enforcement_level = off`; the rulesets list is empty. The detailed protection
  endpoint returns HTTP 404 with the current account, so other protection settings
  were not independently established. Do not read “protected” as proof that all
  three required status checks are enforced.

## M0 exit checklist

| Exit item | Status | Evidence | Remaining work / boundary |
|---|---|---|---|
| Documentation bootstrap and accepted baseline | **done** | [D-72:78](../requirements/03-decisions-log.md#L78) accepts requirements and ADRs 0001–0006; [Cargo.toml:1](../../Cargo.toml#L1) supplies the workspace; [xtask/src/docs.rs:283](../../xtask/src/docs.rs#L283) checks requirement traceability and [docs.rs:381](../../xtask/src/docs.rs#L381) checks Markdown. | Keep STATUS current. Historical pre-baseline prose in ADR 0001 and old STATUS sections is not current authority; D-72 opened the code gate. |
| Later CPU CI profile, at M0 scope | **done** | [ci.yml:30](../../.github/workflows/ci.yml#L30) runs Linux fmt/Clippy/nextest/doctests and macOS nextest; [ci.yml:186](../../.github/workflows/ci.yml#L186) aggregates jobs and states explicit N/A for absent numerical, mutation, benchmark and GPU targets. #57 and #75 have successful light CPU runs. | This is the light bootstrap CPU profile. Full B1 reference/mutation/benchmark execution belongs to M1 activation and later implemented components; [ADR 0001:69](../architecture/adr/0001-repo-layout-and-ci-gates.md#L69). Required missing future targets must block their applicable profile. |
| Independent and adversarial review, with verified triage | **partial** | [Trusted workflow:25](../../.github/workflows/trusted-gates.yml#L25), [review_gates.rs:328](../../xtask/src/review_gates.rs#L328), [review matrix:15](../../xtask/policy/review.toml#L15). #23 merged after the [sixth verification:32](../requirements/STATUS.md#L32) and [pinned seed:3](ci-bootstrap.toml#L3). | Evidence-publication PRs encounter the circular gate on current `main`. #75 proposes its repair but remains unmerged, lacks independent/adversarial evidence and cannot pass policy approval as currently authored. Reconcile #53's cumulative dispositions and refresh #57; obtain current-head evidence and triage rather than treating a green build as review. |
| Attribution rejection and local hook (D-52/D-61) | **done** | [ci.yml:88](../../.github/workflows/ci.yml#L88) scans messages, metadata and additions; [shared rules:1](../../.githooks/attribution-patterns.regex#L1), [hook](../../.githooks/commit-msg), [hook parity tests:1](../../xtask/tests/hook.rs#L1). | Bounded lowercase ordinary-word tool-name gaps remain documented follow-ups below. “Done” means the reviewed M0 gate exists and rejects its supported cases, not universal detection. Do not reopen #23's D-75 review loop. |
| Dependency policy / licence gate | **done** | [deny.toml:6](../../deny.toml#L6) enables all features, advisory checks, a licence allow-list and runtime bans; [ci.yml:69](../../.github/workflows/ci.yml#L69) uses trusted-main deny policy; [repository policy:23](../../xtask/policy/repository.toml#L23) checks the unreleased workspace licence. | Future native/service/container/frontend artifacts need their own inventories. The placeholder is not a selected public project licence. |
| NFR-3: enforced autonomous merge gates | **partial** | [NFR-3:259](../requirements/04-requirements.md#L259); [ci.yml:188](../../.github/workflows/ci.yml#L188) rejects non-success required jobs; [trusted-gates.yml:34](../../.github/workflows/trusted-gates.yml#L34) fetches current main/head; [gates.toml:57](../../xtask/policy/gates.toml#L57) names all three required checks. | Required status enforcement is off in the visible branch API response. Until configured, NFR-3.AC5 requires recorded coordinator verification of the current head/base, metadata, all three checks and review/triage immediately before merge. #75's red trusted checks demonstrate why `merge-gates` alone is insufficient. |
| NFR-8: licence-clean dependency closure, M0 portion | **partial** | [NFR-8:311](../requirements/04-requirements.md#L311), [deny.toml:16](../../deny.toml#L16), [dependency licence research:111](../research/dependency-licences.md#L111); #57/#75 cargo-deny checks pass. | The Rust bootstrap gate exists. Q-N-8's deployment-specific allow-list/native/service review and D-17 project-licence choice remain open; no complete production closure can be claimed for unbuilt deliverables. Before M0 sign-off, explicitly record current inventory/N/A boundaries and the review disposition of the starting policy. Later deployment inventory is not evidence that M0 CI is absent. |
| NFR-10: runtime/infrastructure boundary, M0 portion | **done** | [NFR-10:332](../requirements/04-requirements.md#L332); [rust_policy.rs:440](../../xtask/src/rust_policy.rs#L440) checks the graph; [build_gate.rs:1](../../xtask/src/build_gate.rs#L1) guards compiler/linker execution; [repository.toml:27](../../xtask/policy/repository.toml#L27) restricts script languages; [deny.toml:47](../../deny.toml#L47) bans Python/Julia runtimes. | Rust skeleton/default CPU startup and guarded adapter boundaries exist. PostgreSQL 18/Valkey selection is D-67; provisioning, shared-protocol clients and PostgreSQL dispatch are later implementation, not completed services. |
| FR-18 contract | **done** | [optimisation contract:1](../../crates/optimisation/src/lib.rs#L1), [capabilities:5](../../crates/optimisation/src/capability.rs#L5), [gap](../../crates/optimisation/src/gap.rs), [status](../../crates/optimisation/src/status.rs), [HiGHS stub:22](../../crates/solver-highs/src/lib.rs#L22). | Unavailable backends fail explicitly; no silent MILP relaxation. Sparse IR, load/solve/cancel sessions, native HiGHS, full adapter conformance and numerical/hardware validation remain M2a+ ([FR-18:197](../requirements/04-requirements.md#L197)), not M0 exit blockers. |
| NFR-12 policy review | **done** | [Policy scope:3](../security/ci-runner-policy.md#L3), [trusted workflow:1](../../.github/workflows/trusted-gates.yml#L1), [initial adversarial scope:17](../requirements/reviews/gemini-adversarial-pr23-2026-11-13.md#L17), merged #23 revisions and final D-75 verification/seed above. | M0 has reviewed security design and hosted-runner restrictions. Policy edits still require owner approval. The unchecked [M1 activation checklist:112](../security/ci-runner-policy.md#L112) is **missing M1 operational evidence**, not an M0 penetration-test obligation. No target isolation is certified by this audit. |

## Open PRs: what actually remains

| PR and audited head | Observed state | Consequence / next action |
|---|---|---|
| [#53](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/53), `9d1e63f70e5e34ebab4486b2e98f5050ea770fcb` | Open, **conflicting**; only CodeRabbit success appears in the rollup, no independent approval. Cumulative triage covers 182 IDs from 28 reports through round 14, leaving 12 round-14 rows open for then-paused revision 15. | Preserve historical findings but add a current reconciliation against #23's merged R15 head/verification. Resolve STATUS conflicts, restore applicable checks and obtain required review. Its old paused/owner-account/publication descriptions are historical, not current blockers. Do not merge an unreconciled cumulative ledger as final closure evidence. |
| [#57](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/57), `6d6f6b210faccd93353d155e457745b8ce967cff` | Open, mergeable. Light build/policy/licence/provenance/attribution/docs and `merge-gates` succeed; `policy-approval` succeeds; `independent-review` fails. [Failure log](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/actions/runs/43826238747/job/133160644760) reports no role-compatible approval and no valid adversarial report. | Useful task-board draft, not completion evidence. Replace the obsolete `egs-bot` creation task: [main policy:9](../../xtask/policy/repository.toml#L9) already names `automated-machines`. Preserve D-65 hardware as unconfirmed, refresh PR states and remove any suggestion that “merge now” waives review. Obtain review/triage on its final content. |
| [#75](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/75), `c938d79a80932cd85505e17b555a98730f05893c` | Open, mergeable, **not on main**. Latest successful light CPU/policy checks coexist with red `independent-review` and `policy-approval`; cancelled runs also appear. Proposed D-77/evidence-only exception is not deployed. | [Trusted run](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/actions/runs/44005324443) says no independent/adversarial evidence, and protected policy paths were changed in an owner-authored PR: GitHub forbids owner self-approval. Re-present through the D-74 machine account, secure exact-head owner approval, cross-provider review, adversarial triage and a concrete bootstrap plan for publishing that evidence under current-main policy. Do not presume #75 can authorize its own exception. |

A coordinator must resolve the evidence-publication deadlock explicitly. Merely
rerunning #75 while `main` still runs the old policy cannot activate its exception.
This audit does not supply a bypass or owner authorization. #53/#57 are supporting
closure records; their mere existence is not a separate roadmap exit criterion.

## Deferred #23 findings: retained limits versus later coverage

[D-75:81](../requirements/03-decisions-log.md#L81) closes the revision-15 review
loop and routes later findings into protected follow-up PRs. These items do not
independently overturn the accepted bootstrap seed.

| Follow-up | Current evidence and status | Remaining task |
|---|---|---|
| Double-encoded or invalid key spellings | **partial**. [Final decoder matrix:80](../requirements/reviews/gpt-2026-11-16-pr23-r15-verify-6.md#L80): four singly encoded recognized keys reject; eight double-encoded/invalid/incomplete/delimiter-bearing spellings accept. [credentials.rs:252](../../xtask/src/secrets/credentials.rs#L252) decodes once; [line 340](../../xtask/src/secrets/credentials.rs#L340) compares recognized names after that decode. | Establish supported driver/key grammar, distinguish malformed names from usable password parameters, and choose bounded normalization or conservative rejection. Preserve whole-placeholder and non-regression controls. Do not claim every accepted malformed key authenticates. |
| Alternate terminators and header syntax | **partial**. [R15V3-F01/F02:126](../requirements/reviews/gpt-2026-11-15-pr23-r15-verify-3.md#L126) deferred connection terminators and a header/backtick probe; driver/header validity was not proven. Later [assignment parser:193](../../xtask/src/secrets/credentials.rs#L193) and [delimiter regressions:1118](../../xtask/src/secrets/credentials.rs#L1118) retain ampersand/quote/backtick suffixes and strict format boundaries. CommonMark handling requires an actual span starting at the credential key ([line 176](../../xtask/src/secrets/credentials.rs#L176)). | Replay original probes against merged main and close already-covered cases with evidence; investigate only valid-format residuals. Add driver/HTTP grammar controls before expanding scanner behavior. The historical accepting result is not automatically a current leak. |
| Lowercase bare tool names | **partial**. [R14-04:228](../requirements/reviews/gpt-2026-11-14-pr23-r14.md#L228) records lowercase ordinary-word gaps; [shared rules:45](../../.githooks/attribution-patterns.regex#L45) distinguish trailer/authorship contexts from capitalized test/review/gratitude names. | Track the ordinary-word ambiguity explicitly. If narrowed credit contexts are expanded, retain human-name, parameter-sweep, black-box testing and ordinary prose controls and run both hook/CI paths. Do not describe uppercase coverage as complete lowercase coverage. |

## M0.5 and M1 readiness

**M0.5 — partial.** The [D-31 spike verdict:5](../../spikes/data/REPORT.md#L5)
is “research can proceed; coverage partial,” not full-network feasibility closure.
The [inventory:879](../research/m1-data-inventory.md#L879) still lacks all-area
132 kV denominators, dated operational reconciliation and complete parameter
handoff. [Weather:897](../research/m1-data-inventory.md#L897) records a request
template and zero downloaded bytes. Acquire and independently verify one common
weather year alongside both markets' inputs; 2024 is the candidate inventory year,
not an accuracy pass or an excuse to omit missing intervals.

D-56 permits research-only inputs for research with provenance and raw/derived
restricted records outside git. PC-1–PC-7 commercial clearance is a pre-commercial
blocker, not a blanket block on the research spike. Share-alike inputs remain
excluded. [M0.5 requirements:369](../requirements/04-requirements.md#L369) require
coverage, hashes, rights tags and explicit gaps, not merely catalogued URLs.

**M1 — missing implementation/activation evidence, partial preparation.**
[Ingest:4](../../crates/ingest/src/lib.rs#L4), [network:4](../../crates/network/src/lib.rs#L4)
and [scenario:3](../../crates/scenario/src/lib.rs#L3) are skeletons. The
[inventory:879](../research/m1-data-inventory.md#L879) identifies missing dated
assets/rules, bus/BMU/GSP mappings, gross/embedded-demand decomposition, loss
parameters, calendars and canonical identity fixtures. NFR-1/2 foundations and
FR-21 input rules need executable evidence. [#59](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/59)
(acceptance specification, `c15fbe17ab04ef9d6a57ddeb4fac19797c5dd4c9`) and
[#60](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/60)
(acquisition plan, `ea75ecba55088b562f4553a54f199117ebf3eef0`) remain open;
a plan is not an ingest pass.

Owner-only dependencies are the [CDS/ENTSO-E/DNO access prerequisites:831](../research/m1-data-inventory.md#L831),
actual D-65 target hardware/OS/access (Q-N-7/Q-H-7), required-check configuration
and answers to [Q-P-01–08:324](../requirements/02-questionnaire.md#L324).
Portal accounts do not guarantee export approval. CEDA/MIDAS and Renewables.ninja
accounts apply only if those optional validation sources are selected; they are
not unconditional M0.5 blockers. Q-P choices cover continuous ratings, Irish
micro-PV, HVDC losses, clearing/UC proxy, heat-rate/O&M, wind scaling, PV temperature
and fuel/carbon resampling. D-71 requires evidenced gaps or individually accepted
equivalents; unapproved proxies remain labelled assumptions. Heavy builds,
databases, weather processing and benchmarks stay off the coding container.

## Verification performed

Read-only Git/GitHub inspection and source review establish the snapshot above.
Validation: `git diff --check` passes; all 67 local evidence links resolve and their
cited line numbers exist. Twelve temporary-file hook probes confirm that lowercase
`blackbox`, `granite` and `sweep` credits in test/gratitude contexts are retained,
while the corresponding capitalized forms are stripped (all hook exits zero).
No probe text was added to repository artifacts. Current secret-scanner conclusions
above come from source/regression inspection and the cited prior reports; no new
secret-scanner CLI replay is claimed.

A cold `cargo run --locked -j 2 -p xtask -- docs` build was stopped after prolonged
dependency compilation without reaching xtask; it produced no local gate verdict.
The existing hosted check results above are historical PR evidence, not checks of
this audit branch. No heavy workload, database, benchmark, solver run or D-65
isolation test was performed. This audit adds no requirement, modelling decision
or deployment approval.

## Ordered backlog

Routing follows [D-76:82](../requirements/03-decisions-log.md#L82): **GPT 6.1 Sol
medium writes code; GPT high verifies/reviews and triages; Gemini high performs
adversarial review and significant data gathering.** D-76 changes authorship,
not D-28/D-54: GPT-authored artifacts still need a different-provider approving
review (currently Claude under [team.md:13](team.md#L13) and the trusted matrix).
GPT high checking GPT medium is verification, not that independent approval.
An owner-approved routing change is needed if the prescribed approver remains
paused; Gemini adversarial findings never substitute for approval.

| Order / milestone | Concrete task and completion evidence | Owner model / review | Dependencies | Owner-only blocker |
|---|---|---|---|---|
| 1 / M0 | Unblock evidence publication and deliver #75 through the correct account; record the bootstrap method, exact-head approval, review/triage and passing trusted checks. Verify evidence-only success and rejection of tampering, deletion and mixed payloads. | GPT medium code; GPT high verification; Gemini high adversarial; cross-provider approver as above. | Current-main review policy; D-74 account; final candidate head. | Owner policy approval and explicit resolution of the bootstrap deadlock; approver routing if unavailable. |
| 2 / M0 | Reconcile #53 through merged R15 without rewriting historical reports; resolve conflicts and refresh #57's task board/account/target facts. Publish exact-head closure evidence and an explicit NFR-8 current-inventory/N/A disposition. | GPT high research/reconciliation and verification; Gemini high adversarial; cross-provider approving review. | Task 1; #23 final verification and seed; current PR states. | Reviewer-routing decision only if prescribed cross-provider review remains paused. |
| 3 / M0 | Configure and verify required `merge-gates`, `independent-review`, `policy-approval` with current-head/base discipline; record an actual blocked-red/passing-green check and M0 sign-off. Confirm owner/code-owner approval behavior. | GPT high audits configuration and records evidence; GPT medium for any necessary tooling; Gemini high adversarial. | Tasks 1–2; all applicable checks/evidence current. | Owner/admin branch protection access and milestone sign-off. D-17 project licence/Q-N-8 deployment terms remain explicitly tracked, not silently selected. |
| 4 / M0 follow-up | Open bounded follow-up work for encoded keys, original terminator/header replays and lowercase bare names; close already-fixed cases, implement valid residuals in separate protected PRs with positive/negative controls. | GPT medium code; GPT high reproductions/review/triage; Gemini high adversarial. | Merged #23 baseline; task 1 for review publication. Independent of D-31 data downloads. | Exact-head owner approval for protected scanner/hook policy edits; no new #23 review round. |
| 5 / M0.5 | Complete D-31 acquisition and verified coverage/handoff: every GB DNO/voltage area plus all-island Ireland, one common weather year, hashes/terms/rights lineage and missing-parameter ledger. Escalate only evidenced D-71 gaps. | Gemini high data gathering; GPT high verifies and writes feasibility verdict; GPT medium writes maintained code. | Existing spike/manifests; acquisition plan #60 review; can start public-source work while M0 closes. | CDS terms/account/key; UKPN/SPEN/NPg/ENWL registrations/data approvals; ENTSO-E token for selected missing cross-border series. |
| 6 / M1 preparation | Finish reviewed #59 acceptance fixtures/#60 plan and dispose Q-P-01–08; pin dated topology/assets/rules, demand identities, losses, calendar/FX and canonical scenario/input/execution hashes. | GPT high specs/review; Gemini high source gathering/adversarial; GPT medium fixture/ingest code. | Task 5 verified handoff; rights lineage and dated source evidence; owner proxy dispositions before acceptance. | Q-P answers or explicitly accepted equivalents; optional CEDA/Ninja accounts only if selected. |
| 7 / M1 runner | Provision the confirmed D-65 target and pass every NFR-12.AC1/AC2 negative/isolation test before heavy B1 activation; freeze reviewed workload seeds separately from measured implementation. | GPT medium runner/test code; GPT high configuration/evidence review; Gemini high adversarial. | Task 1 trusted dispatch; target identity/access; reviewed security policy. Can proceed alongside tasks 5–6. | Owner supplies actual hardware/OS/access and authorizes runner installation; no assumed D-48 machine identity. |
| 8 / M1 delivery | Implement Rust ingest/network/scenario foundations and prove FR-1–5/19/21 inputs, NFR-1/2 identities/numerics on pinned fixtures; run applicable heavy checks only on the qualified target and publish acceptance results. | GPT medium code; GPT high verification/review/triage; Gemini high data/adversarial; cross-provider approving review. | Tasks 3, 5–7; dated complete inputs, approved assumptions and fixture rights. | Remaining source export permissions/Q-P decisions; qualified D-65 runner. Native solving remains M2a, not an invented M1 prerequisite. |
