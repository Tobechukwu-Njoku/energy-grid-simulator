# PR #4 revision — finding dispositions and coordinator handoff

_Date: 2026-11-13. Proposed baseline/ADRs remain unaccepted; no application code is authorised._

## Inputs, scope and authority

- Reviewed the 25 findings in `claude-review-pr4-2026-11-12.md` (PR #8), all 12
  `ADV4-*` findings in `gemini-adversarial-pr4-2026-11-12.md` (PR #6), and all 12
  earlier `ADV-*` dispositions in `triage-gemini-adversarial-2026-11-12.md`.
- Also reconcile the original 12 findings in `gpt-6.1-sol-2026-11-12.md` below so
  its historic OI queue is not mistaken for current guidance. Original reviews are
  preserved as dated evidence, not rewritten into endorsements of this revision.
- Checked out PR #4 in its existing clean worktree; merged main before revisions
  and refreshed it again to retain the coordinator's latest progress text. Only
  upstream merges change STATUS or the decisions log; this revision authors neither.
- D-51–D-65 govern the revision (D-65 added in round 3). In particular D-55 fixes proxy physics, D-56 permits
  tagged research-only inputs, D-57 limits price-data spending, D-58 chooses future
  publicity, D-59 fixes cost-based BM, D-60 requires every GB 132 kV area, and D-61
  keeps existing bot summaries. D-62 data gathering still needs independent verification.

Locations below use stable FR/NFR acceptance IDs and ADR section titles. `Accepted`
means the finding's actionable defect is addressed in the proposal, not that an
unimplemented gate, historical benchmark or scientific validation has passed.
`Rejected` always names the superseding decision or technical reason. `Deferred`
names an owner/implementation dependency. Compound findings are split explicitly.

## PR #8 review — all 25 findings

| Finding | Disposition and reason | Fix / remaining dependency |
|---|---|---|
| C-1: missing D-55 proxies | Accepted | FR-8.AC1/3 and new FR-21.AC1–4; ADR 0003 proxy/security section requires sourced, effective-dated boundaries, inertia/min-units/SNSP and binding-proxy evidence. |
| C-2: accepted earlier adversarial findings absent | Accepted for the missing scope/rules/inputs/backtest content; rejected its suggested Scottish-plus-ETYS-only 132 kV subset, superseded by D-60 | FR-2.AC1 includes every DNO LTDS area; FR-1.AC2/5 and FR-7.AC3/4 cover dated assets/I-SEM/trading changes; FR-3.AC2 names REMIT and curtailment; FR-12 and numeric table cover BM cost AND volume. |
| C-3: pure N-0 understates constraints | Accepted | FR-8.AC1 makes pure thermal N-0 diagnostic only; FR-21 and ADR 0003 require published secured flowgates, screened full-row checks and explicit proxy limits. Optional sourced critical-contingency screening is not exhaustive N-1 certification. |
| C-4: absent reserve/response | Accepted | FR-6.AC2 requires sourced market reserve, eligible/deliverable headroom/storage and binding fixtures; ADR 0003 stages 1/2 retain reserve constraints. Product-level procurement stays out of P1. |
| C-5: embedded generation/net demand ambiguity | Accepted | FR-3.AC3/4 separates underlying GSP demand from weather-driven embedded output and rejects double subtraction; FR-17 replay includes both embedded and transmission generation. |
| C-6: zero-effect benchmark rule | Accepted as a proposed engineering policy, not an owner waiver of D-28 | NFR-4.AC2 and Q-N-11 propose five paired repetitions, >5% wall or >10% RSS plus 95% ratio confidence; above-floor uncertainty blocks. Baseline adoption is still needed. |
| C-7: dense/uncitable criteria | Accepted | All FR/NFR criteria now have stable .AC numbers; FR-2 requires 100% explicit coverage accounting, FR-14 specifies Playwright/axe, FR-17 defines stress/fallback, NFR-12 lists negative tests, NFR-8 legal nuance moves to rationale. |
| C-8: year-dependent absolute price gates | Accepted, with more robust denominator | Numeric backtest table uses mean ABSOLUTE observed price (not a signed/near-zero mean), MAE/RMSE/bias ratios and positive persistence skill; absolute metrics/coverage still reported. Owner acceptance deferred to Q-N-4. |
| C-9: backtest decision-source nits | Accepted | FR-12 rationale cites D-18/23/26/33/44/55/57/59, M5 and R-8 rather than using forecast horizon as a P1 forecasting claim; FR-5 retains D-21's one-minute requirement. |
| C-10: parallel OI ID scheme | Accepted | Questionnaire section N supplies Q-N-1–Q-N-12; OI aliases retained historically. Active owner queue is Q-N-4 and conditional Q-N-3 only; other policies are explicitly proposed engineering defaults. |
| C-11: unavailable private-plan enforcement/shared identity | Accepted; rejected a new Pro-upgrade question because D-58 already chooses publicity | ADR 0001 uses one merge-gates status, strict up-to-date main and no merge queue/approving-review requirement; pre-publicity procedure is labelled discipline, not branch protection. Deployment remains coordinator-owned. |
| C-12: unavailable provider attestor | Accepted | ADR 0001 review evidence specifies role/provider/head/verdict fields, current-tree binding and attested shared-identity limitations; no external App or unprovable cryptographic identity gate. |
| C-13: disproportionate mandatory GPU isolation | Accepted | ADR 0001 B0/B1/B2 profiles, rootless unprivileged runner and relevant-feature GPU applicability; NFR-7.AC5/NFR-12 align. Actual GPU setup is deferred and cannot block CPU-only P1. |
| C-14: hosted-minute waste | Accepted | ADR 0001 runner section puts heavy Linux work on D-48, caches native builds, schedules macOS checks and requires them on native/build changes. Current billing must be verified at activation; no stale minute quota is asserted. Superseded target identity: see round-3 M-4 / D-65; D-48 is provisional sizing only. |
| C-15: unversioned/partial attribution controls | Accepted for versioned hook/shared patterns and range/metadata/diff checks; rejected removing/disabling bot summaries under D-61 | `.githooks/commit-msg`, shared regex, ADR 0001 attribution section and NFR-3.AC4; enable with `git config core.hooksPath .githooks`. CI remains a specified future rejection check, not claimed deployed. |
| C-16: trivial-test/collusion controls missing | Accepted | ADR 0001 reference-integrity section requires fixture provenance, external ownership or explicit oracle-change sign-off, deliberate perturbation and zero unexplained surviving/timed-out changed-core mutants. |
| C-17: review-role conflict | Accepted; resolved by D-54 rather than a new owner question | NFR-3 and ADR 0001 use artifact-to-role validation with docs/process/team.md as authority; review evidence and adversarial triage are separate checks. |
| C-18: workspace omissions | Accepted | ADR 0001 adds network/ingest/market and separate commitment/redispatch/pricing, edition/toolchain/MSRV/shared dependencies/lints, nextest and uv-locked reference environment. No crates scaffolded. |
| C-19: monolithic UC/network formulation | Accepted | ADR 0003 stages energy pre-pass → two-market UC → network cost redispatch → pricing, with bounded UC repair and required incremental rows/basis. M2a/b/c map the work in requirements and roadmap. |
| C-20: native cancellation/info/crash lifecycle | Accepted for worker isolation, thin native facade, callbacks and basis integration; rejected the broad implication that high-level gap/info access is absent | ADR 0003 native lifecycle records the inspected Model/SolvedModel API: mip_gap and generic double-info already exist; use highs-sys only where needed. Cancellation is IPC/token plus bounded worker kill, not cross-thread handle sharing. Builds/conformance remain deferred. |
| C-21: no long-horizon storage signal | Accepted | FR-9.AC2/3 and ADR 0003 stage 0 require annual/seasonal LP energy signals, target deviation reporting and longer-horizon boundary tests; parallel blocks require validated boundary reconciliation. |
| C-22: unnecessary production-builder contest | Accepted | ADR 0003 sparse section selects direct sparse IR production construction, with good_lp only as small-case oracle; Q-N-5 drops the 20%/25% promotion contest. |
| C-23: reference gap tighter than native defaults | Accepted | NFR-2.AC6 and ADR 0003 exact-reference rule specify matched feasibility, relative gap ≤1e-6 and absolute gap ≤0.001 plus a tight enough final certificate. |
| C-24: cuOpt must not block P1 | Accepted | NFR-7.AC5, Q-N-7 and ADR 0003 adapter table state trait/capability stub only and no P1 cuOpt implementation scheduled. Actual-device qualification remains future work. |
| C-25: original review sizing/role/status nits | Accepted; historic sizing is an illustration, not the chosen architecture | ADR 0003 replaces monolithic framing and closes D-54 conflict; original report dispositions below supersede its OI queue. STATUS interpretation/update is deferred to the coordinator by explicit task boundary. |

## PR #6 adversarial review — all 12 findings

| Finding | Disposition and reason | Fix / remaining dependency |
|---|---|---|
| ADV4-1: D-55 absent | Accepted | FR-8/FR-21 and ADR 0003 proxy/security section; same fix as C-1. |
| ADV4-2: BM missing-data escape and absent curtailment/adequacy gates | Accepted | FR-12.AC2/3 and numeric table require BM cost AND gross volume, curtailment, adequacy and explicit coverage. Missing/incompatible observations are unvalidated and block the validated-MVP claim; owner Q-N-4 accepts thresholds. |
| ADV4-3: near-zero/negative objective gap | Accepted near-zero economic-tolerance issue; rejected the claim that abs-based gaps become negative/inverted for negative objectives | ADR 0003 preserves correct min/max bound direction, abs(I−B)/max(U,abs(I)), raw native gaps, and absolute OR relative acceptance. NFR-5/Q-N-5 align; zero/negative/sign-crossing cases required. |
| ADV4-4: day-one fail-closed deadlock | Accepted | ADR 0001 bootstrap profiles, manual exact-SHA seed evidence, explicit applicability and first baseline-seed protocol. Missing required checks still block once applicable; no invented GPU/reference passes. |
| ADV4-5: unscreened monolithic full-network MILP | Accepted risk; not accepted as proof that the unmeasured solver must collapse | ADR 0003 staged UC/LP decomposition, sparse factors/selected PTDF rows, incremental rows/basis and final ALL-row checks; full-network NFR-5 throughput remains unmeasured. |
| ADV4-6: cancellation deadlock/unsound Send+Sync | Accepted | ADR 0003 worker owns a !Send/!Sync native handle; IPC and a callback-read token, two-second grace/five-second kill/reap proposal, no mutex-held solve cancellation or unsafe sharing. |
| ADV4-7: myopic depletion and carry-over infeasibility | Accepted | FR-6.AC3, FR-9.AC2/3, FR-11 and ADR 0003 pre-pass/committed-boundary continuity, longer-horizon outage tests and bounded repair. |
| ADV4-8: lossy interconnector balance and midnight FX | Accepted conservation/accounting issue; rejected any implication that D-38 daily FX must be replaced or dispatch cannot react to an exogenous daily change | FR-1.AC3/4, FR-8.AC1, FR-10.AC4 and ADR 0003 distinguish sending/receiving power, single-count losses, objective numeraire and no independent currency trading variable. |
| ADV4-9: constant-input one-minute tests are vacuous | Accepted | FR-5.AC2 adds non-stationary ramps, starts/stops, availability, binding reserve and storage with physical-unit scaling and intentional-error detection. Constant case is only a conservation baseline. |
| ADV4-10: zero-tolerance flaky benchmarks | Accepted proposed effect-size/noise treatment; not a measured noise claim | NFR-4 and ADR 0001 specify paired serial repetitions, median effect floor and confidence interval; see C-6. |
| ADV4-11: arbitrary fixed-UC prices | Accepted testability issue; rejected imposing non-convex cost recovery as a property of an LP dual | FR-10.AC2/3 and NFR-2 require uniquely priced marginal/scarcity/negative-cost fixtures and unrecovered-cost ledger, explicit caps/degeneracy and no exact-auction/uplift claim. |
| ADV4-12: watermark lint mistaken for review, role bypass | Accepted role-validation and false-positive concerns; rejected treating an attribution lint as a scientific-truth detector | ADR 0001 checks artifact roles, independently owned references/mutation and scoped shared-pattern lint; process explanations/test literals and verified D-61 summary block are narrowly distinguished. Provider identity remains attested, not proven. |

## Earlier triage — all 12 ADV findings

| Finding | Disposition and reason | Fix / remaining dependency |
|---|---|---|
| ADV-1: hardware versus aim-high | Accepted, already R-14 | NFR-5/7 and ADR 0003 qualify CPU on D-48 actual host; no uninstalled upgrade or GPU performance assumed. Superseded target identity: see round-3 M-4 / D-65; qualification uses the separate target system, not the coding container. |
| ADV-2: review deadlock | Accepted, resolved D-54 | NFR-3, ADR 0001 role matrix; no new owner question. |
| ADV-3: full-network rights/availability | Accepted data-risk concern; rejected treating uncertain commercial rights as an absolute research blocker after D-56 | FR-2/19 and Q-N-3 require actual topology/parameter evidence and research-only tagging. Licence clearance remains pre-commercial; ODbL/share-alike still excluded. |
| ADV-4: GB 132 kV split | Accepted clarification; rejected suggested scope reduction under D-60 | FR-2.AC1 and M0.5/M1 explicitly include all DNO LTDS areas. |
| ADV-5: PyPSA is not BM validation | Accepted | FR-12/13 and numeric table separate UC/network reference agreement from historic cost/volume validation. |
| ADV-6: Rust MILP integration risk | Accepted, already R-9 | ADR 0003 direct sparse native integration and post-baseline qualification; no forbidden optimisation spike. |
| ADV-7: voltage/stability missing | Accepted as resolved scope direction under D-55 | FR-21 published proxies plus honest no-AC/no-dynamics certification disclaimer. |
| ADV-8: collusion/trivial tests | Accepted | ADR 0001 fixture independence, provenance and mutation policy; role evidence alone is not scientific assurance. |
| ADV-9: I-SEM/coupling misrepresentation | Accepted | FR-7.AC3/4 dated DAM/IDM/balancing/imbalance inventory, explicit proxy/deferred fields and date-effective GB↔SEM capacity regime. |
| ADV-10: market/asset changes | Accepted | FR-1.AC2/5, FR-3 and FR-12 require vintage-correct Brexit/MHHS/REMA/link commissioning fixtures and holdout. |
| ADV-11: Redis/Timescale terms | Accepted need for verified terms; owner choice now D-67 | D-67 replaces the former Redis-retention policy: PostgreSQL 18 + Valkey is the baseline, Redis remains a compatible shared-protocol alternative, and P1 dispatch uses PostgreSQL. NFR-8/10, Q-N-8 and ADR 0001 agree; deployment obligations still require review. |
| ADV-12: REMIT/curtailment inputs | Accepted | FR-3.AC2 explicitly requires outage/unavailability, curtailment/constraints, publication/revision and effective timestamps. |

## Original PR #4 review — all 12 findings reconciled

| Finding | Disposition | Current location / limit |
|---|---|---|
| G-1 market scope | Accepted; D-59 settles BM | FR-7/10 rule inventory and cost-based label; exact full-market replica not claimed. |
| G-2 tractability | Accepted as unmeasured risk, monolithic framing superseded | ADR 0003 staged formulation and NFR-5 qualification; illustrative old counts are not inventory facts. |
| G-3 reduced-network drift | Accepted | FR-2 plus Q-N-3 preserves full-network release gate. |
| G-4 reconstruction versus forecasting | Accepted | FR-12, NFR-11, numeric coverage/error table; unavailable as-of vintages forbid forecast claims. |
| G-5 Rust/native maturity | Accepted | ADR 0003 maintained bindings, sparse IR and native workers; no pure-Rust solver mandate. |
| G-6 GPU capability versus performance | Accepted | NFR-5/7, Q-N-7; actual optional-device qualification deferred. |
| G-7 resolution/unit scaling | Accepted | FR-5 dynamic tests, FR-10 loss/FX and ADR 0003 model units. |
| G-8 identical-equation reference | Accepted | FR-13/NFR-2; certificates/aggregates instead of equal alternate binaries. |
| G-9 review enforcement | Accepted principle; external attestor superseded | ADR 0001 proportionate shared-identity evidence and D-54 role resolution. |
| G-10 reproducibility/archival | Accepted | FR-4/16, NFR-1, Q-N-6/9; pinned legally retainable inputs needed for reruns. |
| G-11 software versus dataset rights | Accepted, updated by D-56 | FR-19 research/commercial admission and NFR-8 deployment inventory. |
| G-12 interaction/status ambiguity | Accepted interaction clarification; deferred coordinator status update | FR-14/15/20 and Q-N-10 restrict P1 interaction; no direct STATUS edit. |

## Deliverable and decision traceability

- **M0–M5 decomposition:** `04-requirements.md` staged table and `06-roadmap.md`
  acceptance decomposition allocate every FR-1–21/NFR-1–12; M2a/b/c are substeps,
  not reduced releases. M0.5 remains the sole D-31 exploratory-code permission.
- **Bootstrap/quality policy:** ADR 0001 covers B0/B1/B2 applicability, review/gate
  seeding, quality/reference/mutation/benchmark/licence/data/secrets checks, safe
  self-hosted execution, current-head evidence and explicit enforcement limits.
- **D-52 hook:** `.githooks/commit-msg` and shared regex strip matching whole lines,
  were intended to preserve human trailers and text. Rev-2 review disproved that claim:
  human names and legitimate subjects could be deleted and malformed UTF-8 rewritten.
  Round 3 below supersedes the original smoke-test claim with committed conformance
  fixtures and strict failure behaviour. Activation:
  `git config core.hooksPath .githooks`. This command is documented, not silently
  applied to the shared repository configuration by this revision.
- **D-52 rejection policy:** ADR 0001 specifies PR-range commit messages, title/body
  on edits and added lines, shared patterns, narrow process/test-literal exemptions
  and D-61's bot-managed block exception. No future CI deployment is claimed.
- **D-55/D-59 solver design:** ADR 0003 adds secured/proxy-aware stages, reserve,
  energy pre-pass, sparse screening, bounded UC repair, native worker containment,
  correct absolute/relative gaps and independently checked price/cost ledgers.
- **Owner queue:** Q-N-4 accuracy and conditional Q-N-3 verified gaps (two). Other
  choices are proposed defaults awaiting explicit baseline adoption, not owner answers.
- **Numeric accuracy:** Proposed price/mix/cost/volume/curtailment/adequacy/coverage
  thresholds are marked ASSUMPTION; no result is declared to meet them.

## Validation and remaining work

Local structural checks passed for 33 unique requirement sections, sequential numbered
acceptance criteria, cited decision IDs, all 12 Q-N entries, the two-question active
queue, all 61 finding dispositions, complete milestone allocation, local Markdown
links and `git diff --check`. STATUS and the decisions log matched main after upstream
refresh. Hook smoke tests cover
ordinary messages, mixed human/model trailers, case-insensitive banners, Unicode,
all-attribution/empty messages, paths with spaces and temporary-file cleanup. Failure
tests confirm an invalid regex or missing input fails without altering the message
or leaking temporary files. The
upstream high-level HiGHS API was inspected, not executed or performance-qualified.

Unresolved issues handed to the coordinator:

1. Apply D-66/D-70/D-71 answers to Q-N-4/Q-N-6/Q-N-3 and explicitly adopt/amend
   remaining engineering assumptions; escalate only verified data/parameter gaps.
2. Confirm a compatible free holdout dataset for BOTH BM costs/volumes, curtailment
   and adequacy; missing evidence blocks the corresponding validated-MVP claim. The
   2024 I-SEM prices are now gathered and verified (merged PR #18); acquisition is
   no longer blocked. Check the remaining COVERAGE.md limits and both markets before
   freezing the holdout year (Q-N-3/Q-N-4).
3. ADR 0002 remains dependent on the data spike; ADR 0004/storage is not selected.
   No requirements-baseline or implementation permission is implied by this revision.
4. CI/protection/runner provisioning and D-58 history/secret/restricted-data scans are
   future deployment tasks. The hook cleans new messages, not historic commits.
5. Native ABI/basis/callback conformance, throughput, storage bias and backtest accuracy
   remain post-baseline evidence, not successful experiments recorded here.
6. Obtain independent and adversarial review/triage on the revised final head. The
   historic reviews and this response are NOT a current-head approval or merge waiver.
7. Update coordinator-owned STATUS and decisions log after dispositions/acceptance.
   No review-gate or publication-setting changes are made by this design revision.

## Round 3 — rev-2 reviews and coordinator triage (2026-11-13)

Inputs: `claude-review-pr4-rev2-2026-11-13.md` (M-1–M-6, S-1–S-12) and
`gemini-adversarial-pr4-rev2-2026-11-13.md` (ADV4-R2-01–12), both reviewing
`1698c4be`. Checked out PR #4 and merged current main before making these changes.
The coordinator's triage authorises the fixes below; proposed thresholds, budgets
and formulation choices remain ASSUMPTION under the existing Q-N register. No owner
answer, requirements baseline, workflow deployment or performance result is invented.
Only the upstream merge brings changes to STATUS/decisions; neither file is authored
by this revision. The PR body and repository settings are unchanged. Refreshed main again after it
advanced during this revision; preserved its new Q-M-4 question unchanged before
the P1 Q-N register when resolving the questionnaire append conflict.

### Must-fix dispositions

| Finding / triage | Disposition and concrete change |
|---|---|
| M-1 / ADV4-R2-01: subject and human false positives | Accepted. The hook protects the first nonblank/non-comment subject, aborting with a clear diagnostic if it matches; the complete message stays unchanged. Anchored whole-line footer rules preserve technical cursor prose, human Claude/Claudette/Grokowski names, human Google/GitHub-noreply addresses and robot-icon descriptions. Trailer rules use complete known identities or known service/bot emails, not substrings or bare google.com. The earlier claim that the rev-2 hook preserved human trailers was false and is corrected above. |
| M-2 / ADV4-R2-01: trailer/footer gaps | Accepted. Every `*-by:` key and pairing `*-with:` key is supported, including Assisted-, Generated-, Co-developed-, Authored-, Prompted-, Signed-off- and Helped-by. Creation verbs include Made, Created, Built, Drafted and Crafted; model names include GPT6, Claude3, Claude_Code and the wider families. Quoted trailers, Unicode hyphens, NBSP and markdown-link footers have fixtures. |
| M-3: malformed input/pattern robustness | Accepted. Strict UTF-8 decode with FB_CROAK precedes output; invalid messages/patterns fail without rewriting. Blank/comment rules are ignored, CRLF rules work, and invalid/empty-string-matching patterns or an empty rule set fail safely. Retained bytes and blanks are preserved, as is the scissors suffix. Committed fixtures and `python3 .githooks/tests/run.py` replace the earlier ad hoc smoke-test evidence. |
| M-4 / ADV4-R2-03 / triage 2: D-65 | Accepted. NFR-4/5/6/12, ADRs 0001/0003 and Q-N-7 now place simulator execution, self-hosted heavy CI, databases and benchmarks on the separate D-65 target, never the coding container. D-48 hardware is explicitly a provisional sizing assumption, not confirmed target identity. Target hardware/OS/access is a coordinator dependency below. |
| ADV4-R2-02 / triage 3: REMA/MHHS | Accepted. FR-1.AC2 retains date-effective data-model capability. AC5 tests historical Brexit and commissioned links only; MHHS settlement is deferred to P2 and REMA alternative pricing to P3 under D-44. No hypothetical REMA configuration or retail settlement fixture gates P1. |
| ADV4-R2-04 / triage 4: M0 security deadlock | Accepted. M0 reviews NFR-12 policy only. M1 provisions the D-65 target runner and executes configuration/access-denial tests before heavy B1 activation; M3 retains service/exposure evidence. Tests use canary credentials and denied socket mounts, not live secrets or a host-escape exploit. Both milestone tables and ADR 0001 agree. |
| M-5 / triage 5: historic Dunkelflaute | Accepted. FR-17.AC1 uses three complete UTC days, 72-hour mean wind CF <10% and each daily mean demand above the year's 75th percentile of daily means. Daily cycles can qualify. Candidate screening, coverage and selection evidence precede replay; no candidate is claimed found here. No historical match requires explicit Q-N-4 disposition, with synthetic cases labelled. |
| M-6 / triage 6: gate scope | Accepted. FR-12.AC4, the numeric table preamble, Q-N-4 and M5 explicitly gate perfect-information reconstruction. All P1 as-of metrics are report-only and confer no forecast-validation claim. The owner must adopt/amend this distinction with Q-N-4. |
| ADV4-R2-05 / triage 7: weak/gameable thresholds | Accepted. Skill ≥0.15 replaces >0. Technology shares must meet both 5 pp absolute and 20% annual energy-relative error, with a separate zero-observed-energy check. Adequacy also limits committed available-capacity error/positive capacity-hours bias, reports reserve headroom and requires controlled de-rating shortage-onset/oracle checks, so zero shortage achieved by excess commitment cannot pass alone. Missing commitment truth blocks that gate. |
| ADV4-R2-06 / triage 7: BM comparator | Accepted. FR-7/12 and the cost/volume gates compare constraint-only actions with NESO's constraint-cost breakdown, never total BM outturn. Up/offer and down/bid volumes and costs are scored separately; commercial transfer versus estimated economic cost is reconciled. Missing directional/compatible observations remain blocked. Renewable lost RO/CfD subsidy and storage opportunity costs are explicit; no bid replay is introduced. |
| ADV4-R2-07 / triage 8: LP counterflow/loss sink | Accepted. ADR 0003 uses one signed flow per link/interval, frozen signed breakpoint bounds and an affine loss EQUALITY from the stage-1 selected segment. Neither opposing directional columns nor a loose loss epigraph exists in stage 2; negative prices cannot manufacture counterflow/losses. Stage 2 stays an LP with conditional duals. At most two extra segment/zero-flow LP trials and bounded UC reselection disclose the direction/segment restriction; compare its bias with a small exact MILP oracle after baseline. |
| ADV4-R2-08 / triage 8: repair non-convergence | Accepted for bounded behaviour/fallback; rejected any claim of guaranteed convergence or permission to accept security slack. ADR 0003 allows initial solve + three UC repair re-solves, detects repeated fingerprints and records cuts/violations. Failure invokes one labelled network-aware commitment recovery MILP (300 s), then a checked fixed-commitment LP, within 900 s total window budget. Unsupported/invalid recovery preserves the last committed boundary and stops failed/partial. Diagnostic elastic slacks never become accepted physical results. |

### Remaining Claude should-fix dispositions

| Finding | Disposition and reason / location |
|---|---|
| S-1: time-varying boundary limits | Accepted. FR-3.AC2, FR-21.AC1 and ADR 0003 require NESO day-ahead boundary limits where published, with interval/publication vintage, coverage and labelled ETYS fallback. This is an ingest requirement, not a claim the dataset has already been acquired. |
| S-2: opportunity costs | Accepted. FR-7.AC2, Q-N-2 and ADR 0003 include dated lost RO/CfD subsidy (including signed CfD effects) and storage energy/water values, with no double counting. |
| S-3: wrong up/down split | Accepted. FR-12 and the table score up/offer and down/bid cost/volume independently, in addition to gross reporting; missing compatible breakdown is a data block. |
| S-4: small-share relative errors | Accepted. Both absolute-share and relative-MWh gates apply per technology; observed zero energy has an explicit absolute-energy test. |
| S-5: cumulative benchmark drift | Accepted. NFR-4.AC3, ADR 0001 and Q-N-11 compare against both main and the frozen last milestone/release with the same >5% wall/>10% RSS thresholds and uncertainty policy; hardware changes need reviewed bridge evidence, not a self-reset. |
| S-6: Irish 2024 gap/holdout selection | Accepted; acquisition block resolved by merged PR #18. Numeric-table data note, Q-N-3/Q-N-4, roadmap and this handoff now point to verified 2024 prices and COVERAGE.md limits: imbalance timezone, IDC identities, missing observations and commercial reuse designation remain open. Both markets’ coverage controls the common holdout-year selection before calibration; no accuracy pass is claimed. |
| S-7: disconnections/materiality | Accepted exclusion of sourced frequency/security disconnections, including LFDD, from adequacy-shortage truth. Retained the NFR-2 false-shortage tolerance rather than adding an unevidenced materiality waiver: a new MW/MWh allowance could hide a real modelling shortage. The owner may amend it in Q-N-4 once the observed-series resolution is known. Two-sided commitment and controlled de-rating checks supplement it. |
| S-8: GitHub squash commit gap | Accepted. Verified repository API setting COMMIT_MESSAGES on 2026-11-13. ADR 0001 pins it, checks possible title sources, forbids PR_BODY, requires an immediate pre-merge settings/message check and a post-merge main scan, with publication/further merges blocked on a violation. No setting or workflow is changed by this design revision; D-61 remains confined to PR bot text. |
| S-9: hook activation | Accepted. ADR 0001 and this handoff explicitly require `git config core.hooksPath .githooks` in each clone, followed by the conformance runner and `git config --get core.hooksPath` verification. Shared repository config is not silently changed; revision commits invoke the versioned hook explicitly with `git -c core.hooksPath=.githooks commit`. Coordinator owns rollout to other worktrees/clones. |
| S-10: model/tools/phrases | Accepted. Patterns/fixtures cover Devin, aider, Windsurf/Codeium, Amazon Q, Jules, DeepSeek, Mistral, Llama, Qwen, Sonnet/Opus/Haiku, o1/o3, `with:`, `with the help of`, and generic whole-line AI/LLM banners. Matching complete identities avoids treating human surnames as a tool. |
| S-11: freeze-before-tuning evidence | Accepted. FR-12.AC1 requires a committed hashed manifest before the first calibration artifact, ancestor/hash recording by the entry point and future CI ancestry/identity validation. A changed manifest invalidates calibration evidence and needs a new split. |
| S-12: deterministic rerun limits | Accepted. NFR-1.AC2 and Q-N-6 require deterministic work/node limits with pinned settings, or completion to the required gap certificate. Wall-clock-limited incumbents do not establish fixture reproducibility. |

### Additional adversarial findings

| Finding | Disposition and reason |
|---|---|
| ADV4-R2-09: defer all stability proxies | Rejected. D-55 explicitly brings known published voltage/stability restrictions into P1 as proxies. FR-21 requires sourced linear/commitment limits, not nonlinear dynamics or a RoCoF simulation. For positive demand-plus-export, the SNSP ratio limit can be multiplied through to a linear inequality; that is not a dynamic-stability solver. Full AC/dynamics stay P2/P4. |
| ADV4-R2-10: defer embedded generation | Rejected. Weather-year replay (D-18/D-35) cannot treat historic net GSP demand as unchanged when weather-driven embedded output changes. FR-3 requires a labelled transmission-interface decomposition/reconciliation, not distribution feeder/household DER optimisation or detailed distribution asset registries. P2 distribution scope remains deferred. |
| ADV4-R2-11: remove mutation proposal | Rejected as a claimed owner-decision contradiction. ADR 0001 already labels mutation policy ASSUMPTION under Q-N-11, requires explicit adoption and restricts PR scope to changed numerical logic; D-28 is a minimum quality bar, not a ban on proposing stronger oracle checks. Runtime is unmeasured and cannot justify deleting the proposal; execution belongs on D-65 and owner baseline adoption can amend it. |
| ADV4-R2-12: pricing after network repair | Partially accepted for disclosure. ADR 0003 now exports original-national and final-commitment conditional prices plus repair-induced delta. Rejected the assertion that an out-of-merit unit must always set price: a unit at its minimum can have a binding-bound dual instead. Neither fixed-UC proxy is claimed to reproduce auction formation; repair-induced national-price bias remains an explicit limitation subject to the same reconstruction gates. |

### Round 3 verification and coordinator handoff

- The committed runner passes **121 message fixtures**, including both reviews' concrete
  examples, leading-blank/comment subjects, versioned identities, mixed human/model
  trailers, Unicode and invalid UTF-8. Additional checks cover blank/comment/CRLF and
  malformed/empty-string pattern rules, scratch Git commit/merge paths, ISO-8859-1
  rejection, paths with spaces and temporary-file cleanup.
- Lightweight structural checks validate stable FR/NFR/AC IDs, decision citations,
  Q-N aliases, milestone allocation, local Markdown links and `git diff --check`.
  STATUS and the decisions log have no diff from merged main. No heavy build,
  database, optimiser, reference solve or benchmark ran in the coding container.
- Confirm D-65 target-system hardware, OS, access, runner labels and isolation before
  M1 activation/qualification; D-48 sizing is provisional. Provision the runner and
  execute NFR-12's access-denial suite before any heavy job. Deploy squash-setting
  verification and main post-merge scanning with the future CI policy.
- Activate/verify the versioned hook in each clone as described in S-9. D-58 history
  cleanup, secret/restricted-data scans and later protection/visibility remain the
  coordinator's tasks; no historical commits or PR bot summaries are rewritten here.
- D-66 accepts Q-N-4 reconstruction gates and the daily stress screen; D-70 answers
  Q-N-6 reproducibility/labels; D-71 answers Q-N-3 with full all-132-kV scope and
  evidenced-gap escalation. D-67 selects Valkey and P1 PostgreSQL dispatch.
  Adopt/amend only the remaining engineering details at baseline. The 2024 I-SEM
  prices are gathered and independently verified (merged PR #18); before freezing a
  compatible common holdout, apply the remaining limits in
  [COVERAGE.md](../../../spikes/data-gathering/semo-2024/COVERAGE.md), especially
  imbalance timezone confirmation and IDC interval identities. Research-only tags,
  missing observations and reuse designation remain; availability is not validation.
- ADRs 0001/0003 remain Proposed; segment approximation bias, fallback success,
  numerical/native conformance, full-network performance and actual accuracy still
  need post-baseline evidence. Revised-head independent/adversarial approval is
  pending; historic-head reviews and this disposition report are not merge approval.

### Owner-answer reconciliation (2026-11-13)

D-66/D-67/D-70/D-71 supersede earlier pending-owner and Redis-retention statements
in the dated rounds above. Q-N-3/Q-N-4/Q-N-6 are answered; remaining engineering
details still require baseline adoption. Merged PR #18 resolves 2024 price
acquisition, not the documented calibration/reuse limitations. The unchanged
latest hook revision passes `.githooks/tests/run.py`: **130 fixtures** plus
pattern-file, encoding and Git commit/merge checks. STATUS and the decisions log
are inherited from main without edits.
