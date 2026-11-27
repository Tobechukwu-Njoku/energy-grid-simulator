# Cross-provider review of PR #4 — MVP requirements, ADR 0001, ADR 0003

_Reviewer: Claude Opus 5.5 (D-54: Claude reviews GPT-authored documents) · 2026-11-12_
_Author under review: GPT 6.1 Sol · PR #4 head `ee6239a` (`t3/design-review-mvp-requirements-adrs`)_

Files reviewed: `docs/requirements/04-requirements.md` (FR-1–FR-20, NFR-1–NFR-12, OI-1–OI-12),
`docs/architecture/adr/0001-repo-layout-and-ci-gates.md`,
`docs/architecture/adr/0003-optimisation-modelling-layer-and-solver-trait.md`,
`docs/requirements/reviews/gpt-6.1-sol-2026-11-12.md`. Checked against D-1–D-55, `05-risks.md`,
`06-roadmap.md`, `docs/process/team.md` and `reviews/triage-gemini-adversarial-2026-11-12.md`.

Line numbers refer to the files at the PR head. Severity is relative to **baseline acceptance**
of `04-requirements.md` and acceptance of the ADRs, not to merging the PR as a *Proposed* draft.

- **blocker**: must be fixed before the requirements are baselined or the ADR accepted.
- **major**: materially affects correctness, tractability or enforceability; fix before baseline.
- **minor**: clarity, testability or hygiene; fix when convenient.

Findings tagged **[expected gap]** stem from D-54, D-55 or the Gemini triage, all recorded after
the author started. They are listed for completeness and are **not** counted as author errors.

## Summary

The draft is careful and honest. It invents no decisions: every numeric target is labelled as a
proposal and routed to an OI item. It protects D-32 against a silent downgrade to a reduced
network, and it keeps three kinds of evidence apart: implementation agreement (FR-13), historical
accuracy (FR-12) and hardware throughput (NFR-5). The tolerance definitions in NFR-2 are among
the best parts of the draft. Every FR/NFR cites real decision IDs, and I found no requirement
that contradicts a decision log entry as it stood when the author started.

The weaknesses are in four areas:

1. **Physics fidelity.** D-55 and the Gemini triage items are missing (expected). Separately, the
   author's own N-0 assumption and the missing reserve requirements would make constraint costs
   and prices systematically wrong.
2. **Tractability.** ADR 0003 does not commit to the staged formulation (national UC MILP, then
   network redispatch LP, then pricing LP) that makes a full-network year feasible on a 5900X.
3. **Proportionality of enforcement.** ADR 0001 designs controls (provider attestor, VM-passthrough
   runner) that cannot be built or verified with one shared GitHub identity and no GitHub Pro,
   while leaving cheap, concrete controls (versioned D-52 hook, CodeRabbit config, CI minute
   budget) unspecified.
4. **Owner load.** Twelve OI items go to the owner. Most of them can be agent defaults.

## Findings

### A. Requirements (`04-requirements.md`)

**1. blocker [expected gap] — FR-8 (L70–74), OI-1 (L230): D-55 proxy constraints are absent.**
*Problem:* FR-8 specifies "linear DC, N-0" and only *disclaims* voltage and stability. D-55 now
requires known voltage and stability limits to enter the MVP as published constraint
boundaries/limits and constraint costs.
*Fix:* add an FR, e.g. "FR-21 Proxy security constraints", with versioned, sourced, dated inputs:

- GB: NESO boundary capabilities (ETYS boundaries such as B4/B6/SC1/EC5) as flowgate limits on
  sums of branch flows.
- GB: published voltage- or stability-driven requirements (minimum units online in a region, a
  system inertia floor) as commitment constraints.
- Ireland: SNSP cap, minimum number of conventional units online and inertia floor as published
  in EirGrid/SONI operational constraints.

Acceptance criteria:

- each proxy constraint has a source document ID and an effective date range;
- binding proxies and their shadow costs appear in the constraint-cost ledger;
- a fixture shows a binding SNSP cap forcing extra conventional commitment and wind curtailment
  in Ireland;
- outputs state which proxies were active.

Update FR-8's disclaimer wording so it does not read as "no voltage/stability content in P1".

**2. blocker [expected gap] — FR-1, FR-2, FR-3, FR-7, FR-12: Gemini-triage acceptances are not reflected.**
The triage accepted these items for the requirements:

- **ADV-4 (FR-2, L37):** "GB 400/275/132 kV" does not define the 132 kV boundary. In England and
  Wales 132 kV is DNO-owned (distribution); in Scotland it is transmission. *Fix:* define the P1
  boundary, e.g. Scottish 132 kV plus E&W 132 kV only where it appears in ETYS. This changes the
  literal wording of D-32, so it needs owner confirmation (owner question 3).
- **ADV-9 (FR-7/FR-10):** I-SEM needs explicit requirements. Name the ex-ante DAM/IDM, the
  balancing market and imbalance pricing. Also cover post-Brexit GB↔SEM trading: explicit
  capacity allocation and intraday auctions instead of implicit coupling. Interconnector
  scheduling must follow the regime in force for each backtest date.
- **ADV-10 (FR-1/FR-12):** the asset and rule snapshot must be date-effective. Required changes
  include Brexit decoupling (2021), new interconnectors (e.g. Viking 2023, Greenlink 2025), MHHS,
  and the 2025 REMA outcome (zonal rejected), so backtest years use the rules and assets in force.
- **ADV-12 (FR-3, L40–44):** list REMIT unavailability messages and curtailment/constraint data
  explicitly as backtest inputs, with as-of timestamps (FR-3 already has the right
  publication-timestamp machinery).
- **ADV-5:** largely addressed by FR-12/FR-13 (credit to the author). FR-12 still lacks a numeric
  BM/constraint-cost metric. Add one, e.g. monthly GB constraint cost and volume error vs NESO
  published constraint costs, with a proposed tolerance.

**3. major — OI-1 (L230), FR-8: N-0 with thermal branch ratings will understate GB constraint costs.**
*Problem:* GB redispatch is driven by *secured* boundary transfers (N-1/N-D), not by intact-network
thermal limits. A pure N-0 DC model with nameplate ratings will show far fewer binding
constraints than reality. FR-12's BM/constraint comparison would then fail for structural
reasons, not calibration ones. This is the author's own assumption, independent of D-55.
*Fix:* make the P1 security model "N-0 DC plus secured limits": (a) NESO boundary capabilities
as flowgates (these already embed post-fault security), and optionally (b) a short list of
critical contingencies via LODF, added lazily (see #8). State in OI-1 that pure N-0 is a
diagnostic mode only. Agents can set this default; no owner input is needed beyond D-55.

**4. major — FR-6 (L58–62), OI-1: no reserve or response requirement in UC.**
*Problem:* No FR requires holding reserve. OI-1 lists "reserves" only as something to approve.
UC without reserve requirements under-commits thermal plant, under-states start-up costs and
mis-prices scarcity. That hits FR-12 price accuracy directly. GB (response/reserve products) and
I-SEM (operating reserve categories, DS3) both hold significant headroom.
*Fix:* add to FR-6 an aggregate upward reserve requirement per market (largest infeed plus a
response margin), met by committed headroom and storage, with the source of the requirement
recorded. A fixture shows the reserve constraint binding and changing commitment. Product-level
procurement replication stays out of P1. Agent default.

**5. major — FR-1, FR-3: embedded (distribution-connected) generation is not specified.**
*Problem:* The P1 network is transmission-only, but GB has roughly 20+ GW of distribution-connected
solar and wind. If transmission demand comes from historical outturn, the weather-year replay
(D-14, D-18) cannot vary embedded output with the weather. If underlying demand is used, embedded
output must be modelled.
*Fix:* FR-3 should require GSP-level demand split into underlying demand and weather-driven
embedded wind/solar (treated as negative demand at the GSP). FR-17 then re-weights both for a
replayed weather year. Agent default. This keeps P2 distribution networks out of P1.

**6. major — NFR-4 (L168–172): "any detected slowdown blocks" with no threshold is untestable.**
*Problem:* With realistic benchmark noise, a significance test with no effect-size floor either
flakes, because tiny real differences become significant over many repetitions, or needs constant
manual reruns. "No fixed percentage allowance" makes the gate either always red or arbitrarily
judged. That breaks D-22's autonomous merging.
*Fix:* the agent proposes the default; the owner does not need to set it. Block if the median
wall time regresses by more than 5% (peak RSS by more than 10%) and the bootstrap 95% CI of the
ratio excludes 1.0, over at least 5 paired repetitions on the serial runner. Below the threshold:
pass, with the trend recorded. Rebaselining only via a reviewed main-branch PR.

**7. minor — acceptance-criteria form (all FR/NFR).**
*Problem:* Each "Acceptance criteria" field is one dense paragraph with 5–9 conditions. Tests and
reviewers cannot cite an individual criterion, and partial passes are hard to record.
*Fix:* number criteria as `FR-7.AC1 … AC5`, one testable statement each. Several criteria need
concrete measures:

- **FR-2:** give a coverage number, e.g. 100% of ETYS boundary circuits and at least 99% of
  inventory nodes modelled or explicitly equivalenced.
- **FR-14 (L109):** "filters and drill-down work" and "keyboard-operable" become a Playwright E2E
  script plus axe-core with zero serious/critical violations.
- **FR-17 (L127):** define the stress interval, e.g. at least 72 h with GB wind capacity factor
  below 10% and demand above the year's 90th percentile.
- **NFR-12 (L219):** replace "threat-model test demonstrates" with the concrete negative tests
  from ADR 0001 §Enforcement 4.
- **NFR-8 (L195):** "No blanket legal claim … is accepted" is not testable as a requirement.
  Move it to rationale.

**8. minor — OI-4 (L233): absolute £/MWh thresholds depend on the year chosen.**
*Problem:* MAE ≤ 20 and RMSE ≤ 40 is a loose test in a low-price year and very tight in 2022-type
crisis years.
*Fix:* express the thresholds relative to the test year: MAE ≤ 15% of the mean observed price,
plus a skill score > 0 against the previous-week persistence baseline (already proposed). Report
absolute values alongside. Keep the ≥ 99% coverage rule.

**9. minor — FR-12 (L96), FR-5: traceability nits.**
FR-12 cites D-25 (forecast horizons) while correctly stating P1 is not forecasting. Cite D-23,
D-33 and D-44 as the drivers and R-8 as the risk. Note that the P1 backtest comes from the
roadmap/Fable review rather than an explicit decision. FR-5's one-minute support is legitimately
from D-21 (not phased), and the draft is right not to promise a one-minute annual runtime.

**10. minor — OI register (L222–241): parallel ID scheme.**
AGENTS.md rule 2 uses `Q-<section>-<n>` for open questions. Convert OI-1–OI-12 to questionnaire
IDs, keep "OI-n" as an alias, and close those that become agent defaults (see §D).

### B. ADR 0001 — repository layout and CI gates

**11. major — ADR 0001 L131–139, NFR-3 (L165): required enforcement does not exist on this plan.**
*Problem:* Rulesets and branch protection are unavailable for private repos on GitHub Free, so
NFR-3's "Protected-main/ruleset tests" cannot be satisfied today. Two further points:

- Every agent pushes with the owner's single GitHub identity, and GitHub does not let an author
  approve their own PR. "Required approving reviews" are therefore unusable even after an upgrade.
  The cross-provider check must be a *status check*, not a GitHub review.
- Merge queues are an organisation feature (private repos also need Enterprise Cloud), so the
  `merge_group` path (L93) will never run on this personal repo.

*Fix:*

- Owner question 1: upgrade to GitHub Pro. It enables rulesets and required status checks on
  private repos.
- Make `merge-gates` the single required status check, with "require branch up to date" (strict)
  instead of a merge queue. Drop `merge_group`.
- Until Pro is available, merge only through a versioned script (`ci/merge.sh` wrapping
  `gh pr merge`) that refuses unless `merge-gates` is green on the current head. Record in
  STATUS that this is discipline, not enforcement.

**12. major — ADR 0001 L141–156: the provider attestor cannot be built or verified here.**
*Problem:* The ADR asks for a "separately trusted GitHub App" that verifies "against
orchestrator/provider records" and rejects free-text model names. No such record source is
reachable from CI. Every agent holds the same owner token, so any agent can mint whatever the
attestor accepts. The design adds provisioning cost without adding real assurance, and it blocks
M0.
*Fix:* adopt a proportionate design with an explicit threat model: *accidental* omission by
cooperative agents, not malicious ones.

- Each review is a committed file in `docs/requirements/reviews/` (or a structured PR comment)
  with `author-provider`, `reviewer-provider`, `head-sha` and `verdict` fields.
- A CI check verifies that a review exists for the current head SHA, that the providers differ
  and that the verdict is not "request changes".
- Accept that provider identity is attested, not proven. Real protection against collusion and
  weak tests comes from ADV-8 controls (#16), not from identity cryptography.

**13. major — ADR 0001 L164–189, NFR-7 (L186–190): GPU-runner isolation is disproportionate and would block all merges in P1.**
*Problem:* A disposable VM with GPU passthrough on the owner's workstation is substantial
provisioning work. Under "block the GPU gate if isolation cannot be supplied", the gate stays
red, yet P1 needs no GPU code (HiGHS CPU is the default; R-14). GitHub's self-hosted-runner
warnings mainly concern public repos and fork PRs. This repo is private and only the owner's
agents push to it.
*Fix:*

- Run the self-hosted runner as a dedicated unprivileged user, with each job in an ephemeral
  rootless container (NVIDIA container toolkit for GPU jobs).
- Keep no secrets on the host beyond the runner token. No Docker socket mount, no
  `pull_request_target`.
- Make `gpu-validation` required only for changes to GPU-feature crates once a GPU adapter
  exists, reporting explicit N/A otherwise (the ADR already allows a classifier). This keeps
  D-43 intact without blocking P1.

**14. major — ADR 0001 gate table (L95–108): the Actions minute budget is not considered.**
*Problem:* On current GitHub billing (verify before adopting), private repos get 2,000 Actions
minutes/month on Free and 3,000 on Pro, and macOS minutes count 10×. A Rust workspace that builds
HiGHS from source (`highs-sys`, cmake + C++) on Linux and macOS for every PR, plus the PyPSA
reference job, will use up the budget within days of active agent work.
*Fix:*

- Run heavy Linux jobs (tests, clippy, numerical-reference) on the self-hosted 5900X runner.
- Use `sccache` and cache the native HiGHS build.
- Run the macOS CPU job on pushes to main, nightly, and as required on PRs touching native or
  build files. Alternatively, register the owner's Mac as a second self-hosted runner.

Agent default; no owner input needed unless spend is involved.

**15. major — ADR 0001 L158–162, NFR-3: D-52 enforcement is under-specified, and PR #4 itself shows the gap.**
*Problem:*

- The ADR states the principle but not the mechanism. The existing `commit-msg` hook lives only
  in this machine's `.git/hooks`, so it is not versioned and does not apply to agents on other
  checkouts.
- The hook covers commit messages only.
- **PR #4's own body contains a CodeRabbit "auto-generated … Summary by CodeRabbit" block.** That
  is AI-generated content inserted into a PR body, which D-52 forbids. CodeRabbit is also a
  fourth AI reviewer that `team.md` does not mention.

*Fix:*

- Version the hook under `.githooks/` and set `core.hooksPath` in a bootstrap script.
- Make the `attribution-policy` CI job scan (a) every commit message in the PR range, (b) PR title
  and body, re-run on the `edited` event because bots edit bodies after opening, and (c) added
  lines in the diff. Exempt `docs/process/**`, `docs/requirements/reviews/**` and the decisions
  log.
- Use the hook's regex as the shared pattern list.
- Add `.coderabbit.yaml` with the PR-description summary disabled (keep line comments if wanted),
  or remove CodeRabbit. Coordinator to confirm with the owner (owner question 5).

**16. major [expected gap] — ADR 0001: ADV-8 controls missing.**
The triage accepted three controls for ADR 0001:

- reference-data tests owned outside the authoring PR: changes under `tests/reference/` and
  `tests/cases/` need a separate PR or reviewer sign-off;
- mutation testing on core crates (`cargo-mutants` on `optimisation` and `engine`, with a
  surviving-mutant budget);
- provenance (source, hash, author PR) for every fixture.

**17. minor [expected gap] — ADR 0001 L150–156, OI-11, NFR-3 rationale: D-51 versus cross-provider review conflict.**
D-54 resolves this: Claude reviews GPT-written documents. Remove the open conflict from OI-11
and ADR 0001. Triage ownership is already set in `team.md` rule 6 (GPT or coordinator).

**18. minor — ADR 0001 L37–64: workspace layout gaps.**

- Add crates the roadmap needs: `network` (topology, DC model, PTDF/LODF, flowgates), `ingest`
  (M1 data pipelines, licence/lineage manifests per FR-19) and `market` (versioned GB/I-SEM rule
  modules, per R-6).
- Split `engine` so it does not hold UC, redispatch, pricing and orchestration together.
- Specify `[workspace.dependencies]`, `[workspace.lints]` (deny `unsafe_code` outside `solver-*`),
  edition 2024, an MSRV in `rust-toolchain.toml`, `cargo-nextest`, and a `uv`-locked Python
  environment for `tests/reference/`.

D-53 ("commit and push often") cannot be enforced by CI. Saying so explicitly is fine.

### C. ADR 0003 — modelling layer and solver trait

**19. major — ADR 0003 L96–109, L135–140 (and GPT review finding 2): commit to a staged formulation.**
*Problem:* The ADR treats the network as part of one UC MILP, and the companion review sizes the
problem that way (157.68 M variables per year; 864 k per 48 h window). That framing makes P1 look
riskier than it is and leaves the main tractability decision open. GB's actual arrangement has
the same structure as a staged model: self-dispatch or national scheduling first, then
network-driven BM redispatch.
*Fix:* make the default pipeline per window:

1. Two-market UC MILP on a copper plate per market (plus interconnector coupling, reserves (#4)
   and commitment-type proxies such as SNSP/min-units/inertia (#1)). Only about 10² committable
   units carry binaries, giving tens of thousands of binaries per window instead of millions.
2. Network redispatch on the full DC network with commitments fixed. This is an LP, or a small
   MILP that releases only fast-start units. Add branch, flowgate and contingency constraints
   lazily: solve, check flows via PTDF, add violated rows and warm-start. Most of thousands of
   branches never bind.
3. A pricing LP on stage-1 commitments.

This also matches FR-7's required separation of pre- and post-redispatch schedules. Consequence
for the trait (L113–124): incremental row addition and LP warm start become core capabilities of
the HiGHS adapter, not optional accelerators.

**20. major — ADR 0003 L66, L146: the `highs` crate is unlikely to cover cancellation, incumbent and bound access; plan the FFI layer and a worker process now.**
*Problem:* FR-11 and FR-18 require cancellation, incumbent/bound/gap reporting and crash-safe
runs. The high-level `highs` wrapper exposes a limited surface. HiGHS interrupt and incumbent
reporting go through its C callback API, and MIP bound/gap through info queries
(e.g. `mip_dual_bound`, `mip_gap`). This is predictable, so "add a shim only when measured" defers
a known need. Separately, a segfault or out-of-memory in native code inside the API process would
kill the service.
*Fix:* specify a thin `highs-sys` adapter (reuse the crate for build and linking) that wraps
callbacks and info queries. Run each solve in a **worker subprocess** owned by the orchestrator.
That gives crash isolation, hard cancellation by process kill (which ADR L120 already allows),
and per-process peak RSS for NFR-4/NFR-5 with no extra instrumentation. Verify the current
wrapper surface before finalising.

**21. major — ADR 0003 L135–140, FR-9: rolling horizon plus storage and hydro needs a long-horizon signal.**
*Problem:* With 48 h look-ahead and 24 h advance, pumped storage (Dinorwig, Cruachan, Foyers,
Turlough Hill) and reservoir hydro lack any value signal beyond the window. They either empty
into the terminal boundary or follow arbitrary terminal rules, which biases prices across the
Dunkelflaute events D-14 targets.
*Fix:* add an annual (or seasonal) LP pre-pass, copper-plate and without binaries, that produces
storage and hydro state-of-charge targets or water values per window boundary. This is the
standard medium-term/short-term split in production-cost models. It also enables an optional
speed-up: solve weekly blocks in parallel from pre-pass states. Without that, rolling windows are
strictly sequential, and HiGHS's branch-and-bound gains little from more threads, so the 24
threads of D-48 would mostly sit idle. Keep the 1/6/12/24-thread measurement (L175), but plan
parallelism at window or scenario level.

**22. minor — ADR 0003 L37–59, L218–225: the dual construction path and promotion thresholds defer a decision agents can make.**
*Problem:* The neutral sparse IR is mandatory anyway (four adapters, cuOpt CSR, fingerprints). A
`good_lp` production path would build expressions and then convert to the IR, an extra
allocation-heavy step at million-nonzero scale. Benchmarking two production engines, plus the
20%/25% promotion rule as an owner item (OI-5), adds process without much expected information.
*Fix:* decide now: a column-oriented sparse builder writing the IR directly is the production
path. Use `good_lp` + HiGHS only as a readable small-case oracle in tests (an independent
model-construction path, which suits FR-13). Remove the promotion rule from OI-5. Keep the
benchmark as a performance *check*, not a selection contest.

**23. minor — ADR 0003 L130–133: gap and tolerance definitions.**
The canonical relative gap denominator `max(1, |incumbent|)` is sensible. Also require
reference/exact tests (FR-6, FR-13) to run with `mip_rel_gap` ≤ 1e-6 and matched feasibility
tolerances on both sides. Otherwise NFR-2's objective tolerance (0.01 + 1e-6·|obj|) is tighter
than the HiGHS default 1e-4 gap, and tests fail spuriously.

**24. minor — ADR 0003 L158–162: cuOpt maturity.**
The careful wording (MIP beta, pinned ABI, card-specific capability) is right. Add one explicit
statement: no cuOpt work is scheduled in P1 (R-14, D-44), so the adapter crate is a trait stub
until a supported card is installed (OI-7).

### D. GPT review report (`gpt-6.1-sol-2026-11-12.md`)

**25. minor — findings 2 and 9.**
Finding 2's sizing arithmetic is correct, but it assumes a monolithic network MILP; see #19.
Finding 9's role conflict is now resolved by D-54. The remaining findings are sound and
well-sourced. Finding 12, that STATUS calls open items "non-blocking" while accuracy, scope and
data dispositions are baseline blockers, is valid; the coordinator should act on it.

## Which open items need the owner

| OI | Who | Reason / proposed default |
|---|---|---|
| OI-1 physical scope | **Agent** (D-55 settles the core) | DC + secured boundary/flowgate limits + stability proxies (#1, #3); explicit interconnector losses; aggregate reserves (#4); LP pre-pass storage targets (#21) |
| OI-2 market rules/products | **Owner, one yes/no**; rest agent | Owner: accept a cost-based constraint-redispatch proxy (no historical bid/offer replay) as P1's "BM redispatch" (R-11 mitigation never confirmed). Agent: GB reference product = Market Index Price (half-hourly), I-SEM = DAM; external boundaries fixed to historical flows in backtests |
| OI-3 reduced network | **Owner, later** | Only when the D-31 spike reports gaps. Ask together with the 132 kV boundary (ADV-4) |
| OI-4 accuracy bar | **Owner** (accept or edit a default) | What "validated" means to funders is the owner's call; agent supplies relative thresholds (#8) |
| OI-5 windows/gap/runtime | **Agent** | 48 h/24 h, 0.1% gap, ≤ 48 h/year as provisional targets, revised after measurement; drop the promotion rule (#22) |
| OI-6 reproducibility | **Agent** | Numerical, manifest-level reproducibility; closes Q-D-5 as an agent default |
| OI-7 GPU/licences | **Owner info, non-blocking** | Which GPU is installed now; any Gurobi/Xpress licence. HiGHS CPU path needs neither |
| OI-8 licence allow-lists | **Agent** | Standard permissive allow-list (MIT, Apache-2.0, BSD, ISC, Zlib, Unicode-3.0, MPL-2.0 file-level); Redis-vs-Valkey goes to the owner only after PR #2 verification (ADV-11) |
| OI-9 input archival | **Agent** | Archive licensed inputs for pinned runs (storage cost is not a constraint, D-2) |
| OI-10 UI/interaction | **Agent** | P1 = submit/monitor/cancel/inspect + sequential CLI batches |
| OI-11 enforcement | **Owner: GitHub Pro only** | Role conflict closed by D-54; attestor/bootstrap/benchmark protocol are agent defaults (#6, #11, #12) |
| OI-12 exposure/auth | **Agent** | P1 private (localhost/VPN); ask Q-L-2/Q-L-3 only before external exposure |

## Verdict

**Request changes.** The authoring is strong, and none of the blockers comes from an invented
or contradicted decision. Before baseline acceptance:

- fix blockers #1 and #2 (both caused by later decisions);
- fix majors #3–#6 (physics and testability) and #19–#21 (tractability);
- make ADR 0001 proportionate to a single-identity private repo (#11–#15);
- reduce the owner's queue to the questions below.

Merging the PR as a clearly *Proposed* draft is acceptable once the coordinator has triaged
these findings, provided STATUS does not treat it as baselined.

## Owner questions I consider genuinely necessary

1. **GitHub Pro:** upgrade the account so required status checks can be enforced on the private
   repo? Without it, D-28 gates are discipline, not enforcement.
2. **BM fidelity:** is a cost-based constraint-redispatch model, with no replay of historical
   bids and offers, acceptable as P1's "Balancing Mechanism redispatch"?
3. **132 kV boundary (D-32 wording):** include Scottish 132 kV plus England & Wales 132 kV only
   where it appears in NESO's ETYS, and leave the rest of E&W 132 kV (DNO-owned) to P2?
4. **Validation bar:** accept the proposed relative accuracy thresholds (#8) as the condition for
   calling the MVP "validated", or set your own?
5. **CodeRabbit:** keep it (with the AI summary removed from PR bodies, per D-52) or remove it?

Optional and non-blocking: which GPU is installed today, and whether any commercial solver
licence is available.
