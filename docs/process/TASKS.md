# Task Status Board

_Last updated: 2026-11-18_

Snapshot of repository work, open PRs and review threads on 2026-11-18 UTC.
M0 remains partial: PR #23 merged after revision-15 verify-6 ATTEST, but review
publication, required-check deployment and explicit milestone closure still need
evidence. Green light CPU CI alone is not review approval. D-76 changes code
authorship; D-28/D-54 still require the prescribed cross-provider approving review.

## Tasks

| ID | Task | Owner / Model | Status | PR or Link | Blocker | Last Updated |
|---|---|---|---|---|---|---|
| T-1 | M0 CI merge gates & Cargo workspace: revision 15 fixes | GPT 6.1 Sol (medium) / Coordinator (verification & merge) | done | [#23](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/23) (merged `3cf6a3f`) | Merged on 2026-11-16 after bootstrap seed attested in `docs/process/ci-bootstrap.toml` (`260d5f9`, commit `5c58e03`) and CI green. | 2026-11-18 |
| T-2 | Cumulative triage record of PR #23 review findings | GPT 6.1 Sol | in review | [#53](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/53) (`t3/pr23-review-triage-record`) | Refresh in progress: main merged, both STATUS histories retained, ledger reconciled through verify-6. Final-head review/triage still required. | 2026-11-18 |
| T-3 | M1 open-data acquisition plan (43 public-scope entries) | GPT 6.1 Sol | in review | [#60](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/60) (`docs/m1-acquisition-plan`) | Request-changes review is on main via #77; repairs underway. Final revised head needs re-review. | 2026-11-18 |
| T-4 | Candidate sources for the 9 M1 data deficits | Gemini Flash 3.8 | done | [#58](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/58) (merged `1734f39`) | Six round-3 corrections applied; merged. | 2026-11-15 |
| T-5 | M1 acceptance test spec (34 tests, 10 requirements covered, 8 flagged items) | GPT 6.1 Sol | in review | [#59](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/59) (`docs/m1-acceptance-tests`) | Request-changes review is on main via #76; repairs underway. Final revised head needs re-review. | 2026-11-18 |
| T-6 | Follow-up issues for deferred PR #23 findings | GPT 6.1 Sol / Coordinator | in progress | `fix/xtask-deferred-policy-followups` | Separate protected follow-up PR being prepared for encoded keys, terminator/header replays and lowercase attribution contexts. | 2026-11-18 |
| T-7 | GPT fact-check of PR #58 | GPT 6.1 Sol | done | [#61](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/61) (merged) | Reject verdict with 16 findings; review report merged. | 2026-11-15 |
| T-8 | Revise PR #58 against 16 review findings | Gemini (Homelab) | done | [#58](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/58) (head `67a9d3d`) | Addressed 16 review findings from PR #61. | 2026-11-15 |
| T-9 | GPT re-check of PR #58 | GPT 6.1 Sol | done | [#62](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/62) (merged) | Reject verdict with 7 findings; review report merged. | 2026-11-15 |
| T-10 | Gemini (Personal) revises PR #58 for round 2 | Gemini (Personal) | done | [#58](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/58) (head `739bb70`) | Addressed 7 round-2 findings; added Section P to questionnaire. | 2026-11-15 |
| T-11 | GPT round-3 re-check of PR #58 | GPT 6.1 Sol | done | [#63](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/63) (merged) | Accept-with-corrections verdict; review report merged. | 2026-11-15 |
| T-12 | Verify PR #23 against round-14 reproductions | GPT 6.1 Sol (high) | done | [#64](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/64)–[#69](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/69) (merged) | Six verification rounds completed; final report merged in [#69](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/69), unblocking PR #23 attestation and merge. | 2026-11-18 |

## Waiting on the owner

### Active task decisions

Required reviews are not optional merge choices. #59/#60 have request-changes
reports; their repairs and re-review are active. #53/#57 need current-head review
and triage after this refresh. #75 needs a concrete evidence-publication bootstrap
and exact-head policy approval under the deployed main policy. Escalate reviewer
routing only if the prescribed approver is unavailable.

### Open Questionnaire Items (Section P — M1 Deficit Methodologies)
- **Q-P-01**: Post-fault to continuous thermal rating proxy (M1 Deficit 1: universal 0.88 multiplier vs asset-class proxies vs explicit continuous ratings under D-71).
- **Q-P-02**: ROI micro-PV capacity interpolation and rooftop scaling (M1 Deficit 3: linear interpolation between annual endpoints with utility profile scaling vs monthly steps vs independent model).
- **Q-P-03**: HVDC interconnector loss formulation and parameters (M1 Deficit 4: calibrated three-term power-loss model vs constant exploratory proxy vs commercial deemed factors).
- **Q-P-04**: Wholesale market clearing proxy vs unit commitment (M1 Deficit 5: baseline MILP unit commitment per D-26 / FR-7 vs exploratory continuous SRMC merit-order proxy).
- **Q-P-05**: Thermal generation efficiency and variable O&M defaults (M1 Deficit 6: author-proposed technology proxies from DESNZ projections vs calibrated plant-specific heat rates).
- **Q-P-06**: Wind speed hub-height scaling shear exponent (M1 Deficit 7: multi-level ERA5 fields vs constant power-law shear exponents alpha=0.143 onshore / 0.11 offshore).
- **Q-P-07**: Solar PV temperature derating coefficient (M1 Deficit 7: constant gamma = 0.004 / deg C default vs manufacturer-specific coefficients).
- **Q-P-08**: Fuel/carbon price resampling (M1 Deficit 6: two-sided interpolation in reconstruction vs release-time step-holding for as-of vs deferral).

### Owner To-Dos
- **D-74 policy approval**: `automated-machines` is already configured on main. Use the separate account for protected policy PRs and obtain a non-self owner approval on the exact head; do not recreate the old bot-account task.
- **Required-check deployment**: Configure and verify `merge-gates`, `independent-review` and `policy-approval`, plus owner/code-owner behavior. The #78 audit reports required-check enforcement off in the visible API; record blocked-red/passing-green evidence and verify current head/base before merging.
- **Target system provisioning (D-65)**: Confirm actual hardware, OS and access, then authorize runner installation and NFR-12 isolation tests before heavy B1 work. Q-N-7/Q-H-7 remain open; no particular processor or RAM configuration is confirmed here.
- **External portal accounts and API logins**: Obtain logins and API credentials for M1 data sources:
  - **ENTSO-E Transparency Platform**: Personal REST API security token (`transparency@entsoe.eu`).
  - **Copernicus Climate Data Store (CDS)**: Registered account, accept ERA5 terms, and configure API key in `~/.cdsapirc`.
  - **Distribution Network Operators (DNOs)**: Open data portal accounts for UK Power Networks (UKPN), SP Energy Networks (SPEN), Northern Powergrid (NPg), and Electricity North West Limited (ENWL).
  - **CEDA Archive (optional)**: Registration only if MIDAS Open validation is selected.
  - **Renewables.ninja (optional)**: Account/token only if this validation source is selected.
- **Security configuration**: Enable private vulnerability reporting on the GitHub repository.

## Current open PRs

Source: `gh pr list --repo Tobechukwu-Njoku/energy-grid-simulator --limit 100 --json number,title,headRefName,headRefOid,state,isDraft,reviewDecision,statusCheckRollup`,
queried 2026-11-18 05:36 UTC. All six are open and non-draft. GitHub's
`reviewDecision` is empty on each; report verdicts below come from repository
evidence, not that field. Checks describe this snapshot before refresh pushes;
re-query the final heads before merge. Older cancelled/failed runs can coexist
with newer results in the rollup.

| PR | Branch / work | State and next action |
|---|---|---|
| [#53](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/53) | `t3/pr23-review-triage-record` — cumulative triage | Only CodeRabbit success in the queried rollup. Main merge/conflict repair and current reconciliation underway; final-head CI and independent/adversarial evidence required. |
| [#57](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/57) | `docs/status-board` — this board | Light CI / merge-gates and policy-approval pass; independent-review fails; CodeRabbit skipped. Refresh underway; obtain final-head review/triage. |
| [#59](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/59) | `docs/m1-acceptance-tests` | CodeRabbit success only in rollup. [Independent report](../requirements/reviews/claude-2026-11-18-pr59.md), merged via #76, requests changes at `c15fbe1`. Repairs active; no approval inferred. |
| [#60](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/60) | `docs/m1-acquisition-plan` | CodeRabbit success only in rollup. [Independent report](../requirements/reviews/claude-2026-11-18-pr60.md), merged via #77, requests changes at `ea75ecb`. Repairs active; no ingest acceptance claimed. |
| [#75](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/75) | `fix/review-evidence-loop` — D-77 evidence publication | Latest light CI / merge-gates succeed; independent-review and policy-approval fail. Code/adversarial reviews active. Current main still runs the old policy; this PR cannot authorize its own exception. Resolve publication bootstrap and exact-head owner approval. |
| [#78](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/78) | `docs/m0-exit-audit-2026-11-18` | Audit proposes M0 partial verdict and ordered backlog. Latest documentation gates, independent-review and merge-gates fail; Rust, policy-approval and remaining light checks pass. Review, documentation repair and triage remain required. |

## Review and implementation threads

Thread states were read at 2026-11-18 04:28 UTC. Running means work is active,
not that a verdict or a passing check has been published. Thread links are local
workspace navigation; the repository review reports and PRs are the durable evidence.

| Thread | Observed state | Scope / remaining evidence |
|---|---|---|
| [GPT review PR75 (retry)](t3-thread://v2/adb19ff8-1f2a-4536-a512-c8adcb377922/thread:delegated-task:command%3Amcp%3Aa8a09679-5656-4924-9297-04b4c99d787b%3Adelegate-task%3Ad2e8b17d-ef63-4db7-8e79-b65aa8d86835) | running at 2026-11-18 04:28 UTC | Review of #75; no completed verdict inferred. |
| [GPT revise PR59 PR60](t3-thread://v2/adb19ff8-1f2a-4536-a512-c8adcb377922/thread:delegated-task:command%3Amcp%3Aa8a09679-5656-4924-9297-04b4c99d787b%3Adelegate-task%3Aa46c16a4-cd6d-40f9-b4e2-1ca274a016db) | running at 2026-11-18 04:28 UTC | Repairs to #59/#60 underway; final-head review remains required. |
| [GPT scanner follow-ups](t3-thread://v2/adb19ff8-1f2a-4536-a512-c8adcb377922/thread:delegated-task:command%3Amcp%3Aa8a09679-5656-4924-9297-04b4c99d787b%3Adelegate-task%3A608bc11e-cd5b-41ee-ab29-00a16f266ac7) | running at 2026-11-18 04:28 UTC | Separate protected follow-up; PR number pending. |
| [Gemini adversarial PR75](t3-thread://v2/adb19ff8-1f2a-4536-a512-c8adcb377922/thread:delegated-task:command%3Amcp%3Aa8a09679-5656-4924-9297-04b4c99d787b%3Adelegate-task%3Ab6ea82f7-67fb-42eb-8fab-2a04c5e179e5) | running at 2026-11-18 04:28 UTC | Review of #75; no completed verdict inferred. |
| [Claude review PR59 PR60](t3-thread://v2/adb19ff8-1f2a-4536-a512-c8adcb377922/thread:delegated-task:command%3Amcp%3Aa8a09679-5656-4924-9297-04b4c99d787b%3Adelegate-task%3A1102f712-4fa5-45ad-b061-b3de0e93e0d3) | completed at 2026-11-18 04:28 UTC | Request-changes reports published via merged #76/#77; re-review needed after repairs. |

## Ordered milestone backlog from #78

Source: [docs/process/m0-exit-audit-2026-11-18.md in PR #78](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/blob/docs/m0-exit-audit-2026-11-18/docs/process/m0-exit-audit-2026-11-18.md#ordered-backlog),
branch head `3419e4f` at this snapshot. This is an audit work queue, not a new
owner decision or M0 sign-off. The audit is still unmerged.

| Order / milestone | Work and completion evidence | Dependency / owner-only blocker |
|---|---|---|
| 1 / M0 | Deliver #75 through the correct account with recorded publication bootstrap, exact-head approval, review/triage and trusted checks. Prove evidence-only success and rejection of tampering, deletion and mixed payloads. | Current-main policy; owner policy approval/bootstrap resolution; approver routing if unavailable. |
| 2 / M0 | Reconcile #53 through merged R15; refresh #57; publish final-head closure evidence and explicit NFR-8 inventory/N/A disposition. | Task 1 for publication; #23 verification/seed; review/triage. This refresh advances the documents, not all closure evidence. |
| 3 / M0 | Enforce all three required checks, verify blocked-red/passing-green and current head/base discipline, then record M0 sign-off and owner/code-owner behavior. | Tasks 1–2; admin access/sign-off. D-17 project licence and Q-N-8 terms remain tracked. |
| 4 / M0 follow-up | Separate protected scanner/hook work for encoded keys, terminator/header replays and lowercase names; close already-fixed cases and test valid residuals with positive/negative controls. | Merged #23; task 1 for review publication; exact-head owner approval. Active branch `fix/xtask-deferred-policy-followups`; no new #23 review round. |
| 5 / M0.5 | Finish D-31 acquisition/verified coverage: every GB DNO/voltage area and all-island Ireland, common weather year, hashes/terms/rights lineage and missing-parameter ledger; escalate evidenced D-71 gaps. | Reviewed #60 and verified handoff; CDS terms/key, DNO registrations/export approvals, selected ENTSO-E token. Public acquisition can proceed alongside M0 closure. |
| 6 / M1 preparation | Finish reviewed #59 fixtures/#60 plan; resolve Q-P-01–08; pin dated assets/rules, demand identities, losses, calendar/FX and canonical scenario/input/execution hashes. | Task 5, rights/source evidence and owner-approved proxy dispositions. Optional CEDA/Ninja access only if selected. |
| 7 / M1 runner | Provision confirmed D-65 system, pass every NFR-12.AC1/AC2 negative/isolation test before heavy B1 activation, and freeze reviewed workload seeds separately from measured implementation. | Trusted dispatch; actual target identity/OS/access and authorized runner installation. Can run alongside tasks 5–6. |
| 8 / M1 delivery | Implement Rust ingest/network/scenario foundations; prove FR-1–5/19/21 inputs and NFR-1/2 identities/numerics on pinned fixtures; publish acceptance results from the qualified heavy target. | Tasks 3, 5–7; complete dated inputs, approved assumptions, fixture rights/export permissions and qualified runner. Native solving remains M2a. |

D-76 routes code to GPT medium, verification/triage to GPT high and adversarial
review/data gathering to Gemini high. Cross-provider approving review remains
required under D-28/D-54; same-provider verification is not that approval.
