# 04 — Formal Requirements

_Status: PROPOSED MVP baseline draft, 2026-11-12. Not accepted; does not enable application code._

## Scope and interpretation

This document specifies **P1 only**: present-day GB and all-island Ireland transmission,
weather-year unit commitment (UC), constraint/Balancing Mechanism (BM) redispatch,
prices, unserved energy, a web map, and historical validation (D-18, D-26, D-32, D-44).
It does not bring P2 distribution, AC voltages, behavioural demand or live forecasting,
P3 alternative pricing/2030 scenarios, or P4 dynamics/multi-tenancy into P1. Present-day
capability is demonstrated by a dated asset snapshot; historical validation additionally
needs the assets and information available in the historical test period.

Every requirement cites owner decisions. Its implementation detail and numeric criteria
are **proposals**, not additional owner decisions. `MUST` means necessary for acceptance
of this proposed P1 baseline; `SHOULD` means a documented exception may be proposed to
the owner, not silently omitted. No requirement below authorises a scope reduction.

**Network feasibility flag (R-12):** FR-1 through FR-20 and NFR-1 through NFR-12 apply to
both full and reduced fixtures, except that **FR-2's full-network release gate cannot be
satisfied by a reduced network**. Reduced fixtures enable testing and honest diagnostic
results; shipping a reduced-network MVP requires a new owner decision amending D-32.
ADR 0002 and ADR 0004 remain pending the D-31 data spike.

## Functional requirements

### FR-1: Geography, assets and interconnector boundaries
- Statement: The system shall represent GB and the all-island Ireland system separately, including interconnections between them and the external boundaries named in D-11.
- Rationale / source: D-11, D-18, D-26, D-32, D-38.
- Acceptance criteria: A versioned fixture identifies every modelled bus, generator, demand allocation and interconnector by stable ID and market; validates endpoints and units; contains no unexplained disconnected demand; and records capacities, losses policy, boundary-price/flow policy and daily GBP/EUR conversion source. A two-market fixture reproduces a known import/export schedule without counting an interconnector twice. Boundary assumptions are explicit (OI-1, OI-2).
- Priority: MUST.

### FR-2: Network completeness and fidelity declaration
- Statement: The system shall ingest the D-32 voltage-level transmission network and disclose its completeness against a dated, licensed inventory.
- Rationale / source: D-30, D-31, D-32, D-40, D-44.
- Acceptance criteria: The data-spike manifest enumerates GB 400/275/132 kV and Ireland 400/220/110 kV assets, known gaps, source coverage and provenance of every inferred parameter. All inventory assets are accounted for as modelled, equivalent or explicitly missing; no missing asset is silently treated as present. Reduced fixtures include their aggregation map, retained constraints and lost capabilities. D-32 release requires no unexplained inventory gaps and owner acceptance of any equivalent or missing assets. Unknown completeness or an unresolved R-12 is a release blocker, not a reason to relabel the reduced fixture “full” (OI-3).
- Priority: MUST.

### FR-3: Dated operational and weather inputs
- Statement: The system shall ingest the demand, generation availability, weather-derived renewable profiles, fuel/carbon inputs and operational data needed for a deterministic weather-year run and backtest.
- Rationale / source: D-14, D-18, D-23, D-26, D-30, D-35, D-44.
- Acceptance criteria: Every input series has units, timezone, source/publication timestamp where available, checksum, licence record and missing-data mask. A non-leap and a leap calendar fixture have exactly 17,520 and 17,568 half-hour intervals respectively in UTC, with explicit GB/Irish local settlement-day mappings through both daylight-saving changes. Missing or imputed intervals are reported; undeclared interpolation and silent future-data use fail validation (OI-4).
- Priority: MUST.

### FR-4: Canonical versioned TOML scenarios
- Statement: The system shall use versioned TOML files as the canonical scenario definition shared by Rust CLI/SDK, API and TypeScript UI.
- Rationale / source: D-8, D-19, D-20, D-35.
- Acceptance criteria: A schema version, geography/asset snapshot, input-manifest hashes, time range/resolution, market-rule version, network fidelity, solver settings, rolling-horizon settings and seed policy are included or immutably referenced. CLI-to-UI-to-CLI round trips preserve semantic scenario identity; invalid or unsupported schema versions fail with field-level errors; migration never overwrites an existing run's scenario (NFR-1).
- Priority: MUST.

### FR-5: Configurable time resolution
- Statement: The system shall default to 30-minute market runs and support configurable intervals down to one minute without misrepresenting source-data resolution.
- Rationale / source: D-18, D-21, D-35.
- Acceptance criteria: Identical constant-input 24-hour cases at 30, 15, 5 and 1 minute conserve MWh, currency totals and minimum up/down durations within NFR-2 tolerances. Fractional commitment durations have a documented rounding/constraint policy. Finer-than-source inputs are marked synthetic/resampled, not observed; unsupported mixed product/resolution combinations fail explicitly. No one-minute annual runtime promise is inferred from NFR-5 (OI-5).
- Priority: MUST.

### FR-6: Intertemporal unit commitment
- Statement: The system shall solve deterministic dispatch with generator commitment, start/stop decisions and intertemporal operating constraints.
- Rationale / source: D-18, D-26, D-35, D-44.
- Acceptance criteria: Licensed small fixtures exercise minimum/maximum output, minimum up/down time, initial state/time-in-state, ramps, start/stop costs, availability and variable cost. Known optimal objectives and feasible schedules match the offline reference under NFR-2; alternate optimal schedules are allowed. Carry-over of on/off duration, output and other intertemporal state is tested across a rolling boundary; no end-of-window free energy or free restart is permitted (OI-1, OI-5).
- Priority: MUST.

### FR-7: National-market and redispatch stages
- Statement: The system shall distinguish the GB national and I-SEM market schedules/prices from subsequent network-constrained redispatch, actions and costs.
- Rationale / source: D-15, D-18, D-26, D-38, D-44.
- Acceptance criteria: A two-market congested fixture produces both pre-redispatch schedules and final feasible schedules, with a reconciled action ledger and separate GB BM and Ireland balancing labels. It exposes incremental redispatch costs, curtailment and unserved energy and never presents a nodal shadow price as a national traded price. A versioned P1 rule inventory identifies modelled rules, proxies and deferred rules; owner agreement on OI-2 is required before claiming P1 market fidelity. A cost-based proxy is not advertised as historical bid/offer replay.
- Priority: MUST.

### FR-8: Physical feasibility and shortage accounting
- Statement: The system shall enforce the accepted P1 network formulation and distinguish physical shortage from infeasible, unfinished or invalid optimisation.
- Rationale / source: D-18, D-26, D-32, D-40, D-44.
- Acceptance criteria: Under the proposed linear DC, N-0 formulation (OI-1), every accepted interval satisfies nodal active-power balance and branch/interconnector limits within NFR-2. Island, bottleneck and insufficient-capacity fixtures exercise explicitly penalised unserved energy and curtailment; slack use is exported by location and interval. Timeouts without an incumbent cannot create zero-price/zero-shortage results. Outputs explicitly state that AC voltage, reactive power, N-1 security and dynamics are not certified by P1.
- Priority: MUST.

### FR-9: Existing grid-scale storage and energy budgets
- Statement: The system shall account for existing grid-scale storage and energy-limited generation where present in the accepted P1 asset inventory.
- Rationale / source: D-18, D-26, D-32, D-40, D-44.
- Acceptance criteria: Storage fixtures enforce power/energy bounds, charge/discharge efficiencies, initial state of charge, terminal policy and inter-window continuity; a full cycle cannot generate free net energy. Simultaneous charge/discharge treatment and reservoir/hydro energy budgets are documented. Assets without required parameters are flagged rather than given unlimited energy. This does not implement household DER or 2030 configuration comparisons (OI-1).
- Priority: MUST.

### FR-10: Prices, currencies and economic ledgers
- Statement: The system shall report GB prices in GBP/MWh and I-SEM prices in EUR/MWh, with separately defined market, imbalance/proxy and physical-dispatch quantities.
- Rationale / source: D-15, D-26, D-38, D-41.
- Acceptance criteria: Each price series names its product, delivery interval, derivation, currency, rule version and validity status. The selected post-commitment pricing method passes a small price-setting fixture and clearly labels any missing non-convex cost recovery/uplift. Fixed-commitment LP prices are not described as MILP duals or exact auction replication. Daily FX fixtures reconcile converted interconnector cashflows; no GBP/EUR addition occurs without explicit conversion (OI-2).
- Priority: MUST.

### FR-11: Complete weather-year execution and run states
- Statement: The system shall execute a full weather year and preserve valid intertemporal state through all solve windows.
- Rationale / source: D-18, D-21, D-26, D-35.
- Acceptance criteria: A full 30-minute test year has one committed result per delivery interval with no overlap or gap. Each window records horizon/look-ahead, bound, incumbent, gap, time limit and termination reason. The annual result is labelled “rolling-horizon”, never “globally annual-optimal”. Cancellation, crash and restart fixtures do not duplicate committed intervals; partial runs cannot be marked complete. A completed time-limited run must satisfy the accepted per-window gap policy (OI-5).
- Priority: MUST.

### FR-12: Historical GB and I-SEM accuracy backtest
- Statement: The system shall publish a reproducible, held-out historical validation report for both markets' prices and generation mix.
- Rationale / source: D-18, D-23, D-25, D-26, D-33, D-40, D-41, D-44.
- Acceptance criteria: A frozen full-year test manifest identifies distinct calibration data, asset vintages and selected observed GB/I-SEM price products. On identical valid intervals report price MAE, RMSE, signed bias, negative-price and upper-decile-price errors, with sample counts and missing-data coverage. Report generation MWh, annual technology-share error in percentage points and daily technology-mix normalised absolute error for each market. Compare price errors against a same-product previous-week baseline. Report GB constraint/BM action volumes and costs only against definition-compatible observations; otherwise disclose missing coverage. Proposed pass thresholds and coverage are in OI-4; unmet/unapproved thresholds block a “validated MVP” claim. Publish matching native-currency results, not an FX-mixed aggregate.
- Priority: MUST.

### FR-13: Identical-input offline reference validation
- Statement: The system shall compare its optimisation and network results against PyPSA on identical input data and mathematical assumptions.
- Rationale / source: D-10, D-20, D-26, D-28, D-33.
- Acceptance criteria: A pinned offline PyPSA environment consumes the same checksummed tables for costs, bounds, network equations, snapshots/weights, UC states and rolling-window policy. Its manifest records any unsupported feature and prevents it being silently omitted from the comparison. Small exact UC, dispatch and congested-network cases pass NFR-2; compare objectives, independently checked feasibility and aggregates rather than a unique commitment schedule. Larger representative windows compare incumbent/bound intervals if either MILP is time-limited. Matching PyPSA establishes implementation agreement, not historical market accuracy (FR-12). Reference scripts stay outside production dependencies.
- Priority: MUST.

### FR-14: Analyst web map and dashboard
- Statement: The system shall provide a TypeScript, web-only, dense dark-mode analytics interface showing the present grid and run results.
- Rationale / source: D-8, D-18, D-20, D-46, D-47.
- Acceptance criteria: An end-to-end fixture loads a scenario, starts/cancels a run, selects a timestamp and asset on a map, and displays GB/I-SEM prices, generation mix, line loading, curtailment and unserved energy with units and legends. Network fidelity, synthetic data, partial/invalid solves and proxy prices remain visible. Filters and time-series drill-down work without relying solely on colour; controls are keyboard-operable. Proposed viewport and responsiveness criteria are NFR-6/OI-10. No native mobile app is required.
- Priority: MUST.

### FR-15: Shared Rust/API interaction and deterministic batches
- Statement: The system shall expose the same scenario submission, status, cancellation and results semantics through its Rust SDK/CLI and API as through the web UI.
- Rationale / source: D-8, D-19, D-20, D-33.
- Acceptance criteria: A contract fixture submits the same TOML through each interface and obtains the same scenario hash and result schema; invalid requests return equivalent structured errors. A CLI batch of two deterministic scenarios produces two independently identified runs and a machine-readable manifest. No distributed sweep capacity, live in-place grid editing or later Python SDK is implied (OI-10).
- Priority: MUST.

### FR-16: Result provenance, exports and retention
- Statement: The system shall preserve run summaries and pinned full-resolution results and apply D-36's retention policy to other full-resolution results.
- Rationale / source: D-3, D-19, D-33, D-36.
- Acceptance criteria: Summary records, scenario identity and validation status survive deletion of an unpinned run's full-resolution results after 90 days; pinned results are not deleted. A simulated-clock test covers the 90-day boundary and pin/unpin transitions. Machine-readable exports include units, currencies, timestamps, fidelity and run-manifest identity. Missing archived inputs or expired results are reported honestly; summaries alone do not promise an exact rerun (OI-9). Storage layout is reserved for ADR 0004.
- Priority: MUST.

### FR-17: Deterministic extreme-weather scenarios
- Statement: The system shall support reproducible replay of historical weather extremes using the canonical scenario format.
- Rationale / source: D-14, D-18, D-19, D-35, D-44.
- Acceptance criteria: One licensed weather year includes an identified low-wind/high-demand stress interval; the same interval can be run independently with documented initial-state differences. Changes in weather/availability appear in the input diff and result manifest. P1 reports deterministic unserved MWh and duration, not probabilistic LOLE/EUE from a single year.
- Priority: MUST.

### FR-18: Solver selection and explicit capability checks
- Statement: The system shall provide a production-capable open-source CPU LP/MILP route and a pluggable boundary for HiGHS, Gurobi, Xpress and cuOpt.
- Rationale / source: D-16, D-20, D-26, D-29, D-43, D-48.
- Acceptance criteria: HiGHS completes the required P1 validation and hardware benchmarks without a commercial licence or GPU. Adapter conformance fixtures cover load, integrality, objective sense, statuses, incumbent/bound/gap, cancellation and invalid input. Optional backends declare supported features, installed/runtime versions and licence/device requirements; unavailable capabilities fail explicitly and never silently relax a MILP to an LP. Commercial production adapters may follow without blocking the default path; the interface must not require their licences in standard CI. ADR 0003 defines the proposed boundary (OI-7).
- Priority: MUST.

### FR-19: Dataset licence and lineage enforcement
- Statement: The system shall admit only datasets approved for the intended commercial use without share-alike obligations and retain per-dataset licence evidence and derived-data lineage.
- Rationale / source: D-17, D-30, D-31, D-32.
- Acceptance criteria: Each admitted dataset and transitive derivative has source/version, acquisition date, terms evidence, attribution obligations and an approval disposition. Unknown terms and OSM/ODbL-derived production-network data are rejected or quarantined. A fixture with a permissive export but prohibited ancestor fails admission. Permissive tool source code is not treated as permission for the tool's datasets; reference fixtures also require documented test-use rights (OI-8).
- Priority: MUST.

### FR-20: Analyst scenario comparison
- Statement: The system should allow an analyst to compare two completed deterministic P1 scenarios without altering either run.
- Rationale / source: D-8, D-19, D-23, D-47.
- Acceptance criteria: A comparison shows input differences and deltas in generation, native-currency price summaries, redispatch cost and unserved MWh. Incompatible temporal coverage, currencies or fidelity are flagged rather than silently aligned. This is a limited P1 analytics convenience, not P2 distributed batch sweeps (OI-10).
- Priority: SHOULD.

## Non-functional requirements

### NFR-1: Reproducible run identity
- Statement: The system shall make each accepted result traceable to immutable scenario, data, engine, modelling and solver identities.
- Rationale / source: D-19, D-20, D-28, D-33, D-35.
- Acceptance criteria: The run manifest includes canonical scenario hash, input and model hashes, engine commit, dependency locks, solver/build versions, parameters, seeds, thread counts, platform, pricing method and timestamp policy. Two pinned deterministic CPU reruns on the same environment agree within NFR-2 and preserve the same input identity; differing solver/platform settings produce distinct execution identities. Bitwise cross-platform/MILP reproducibility is not asserted without OI-6 approval.
- Priority: MUST.

### NFR-2: Numerical correctness and reference tolerances
- Statement: The system shall use stated, independently checked numerical tolerances for feasibility and reference agreement.
- Rationale / source: D-26, D-28, D-33, D-40.
- Acceptance criteria: Proposed OI-6 thresholds: active-power residual or limit excess at most 0.001 MW plus 0.000001 times the relevant local MW scale; integrality distance at most 0.000001; exact small-case objective difference at most 0.01 native currency plus 0.000001 times the absolute reference objective. Energy recurrences use the equivalent MWh tolerance after interval scaling. Tests independently recompute residuals in original units. For non-unique/time-limited MILPs compare feasibility and compatible objective/bound certificates, not equal binaries; unsupported or incomparable cases cannot count as reference passes. Price-method tests use 0.01 native currency/MWh tolerance on uniquely priced small fixtures.
- Priority: MUST.

### NFR-3: Enforced autonomous merge gates
- Statement: Every PR shall satisfy all D-28 quality gates and a review from a different provider before merge, with merge gates decided by CI only.
- Rationale / source: D-22, D-28, D-43, D-49, D-51, D-52, D-53.
- Acceptance criteria: Protected-main/ruleset tests demonstrate that a failed or missing fmt, clippy, tests, licence/advisory, numerical-reference, benchmark or cross-provider check blocks merge; stale-head reviews/checks also block. CI runs `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, tests and `cargo deny check` with an explicit feature matrix. The protected aggregator fails on cancelled, skipped, pending or inconclusive required jobs. Review attestations bind author/reviewer providers, current head SHA and independent review evidence; a bot self-assertion alone cannot approve itself. D-53 adversarial findings require verified triage in addition to approval; D-52 attribution-policy checks cover commits, source and PR metadata with the process-document exception. Documentation-only policies must perform applicable checks and report explicit non-applicability, not invent passing numerical runs. ADR 0001 specifies the proposal (OI-11).
- Priority: MUST.

### NFR-4: No benchmark regression
- Statement: A PR shall not merge with a demonstrated benchmark regression relative to the protected-main baseline on matched hardware and workloads.
- Rationale / source: D-2, D-28, D-43, D-48.
- Acceptance criteria: Benchmarks report build/load/solve/export times, total wall time, peak process-tree RSS, model size/nonzeros and achieved gap; paired baseline/candidate runs use the same runner, inputs and solver settings. Any detected slowdown or memory increase blocks; overlapping uncertainty triggers reruns or an inconclusive blocking outcome, not a pass. Initial baselines require recorded review and cannot be reset by the PR under test. Statistical protocol and any noise equivalence band require OI-11 agreement; no fixed percentage regression allowance is assumed.
- Priority: MUST.

### NFR-5: CPU performance on the actual owner workstation
- Statement: The default CPU path shall be measured against a fixed P1 workload on the D-48 Ryzen 9 5900X, 12 cores/24 threads, 128 GB RAM Linux workstation.
- Rationale / source: D-2, D-18, D-21, D-26, D-29, D-34, D-48.
- Acceptance criteria: Proposed OI-5 targets: with one active simulation, 30-minute resolution, 48-hour look-ahead/24-hour advance and relative MIP gap at most 0.1%, model build/load p95 is at most 30 seconds per window, combined UC/redispatch/pricing p95 at most 300 seconds per window, and a full non-leap year including output writing at most 48 hours. Peak simulation process-tree RSS is at most the smaller of 96 GiB and 80% of measured system RAM, with no swap dependence. Report max window time and gap failures as well as p95. A licensed, frozen full D-32 fixture is required to establish full-network feasibility; reduced-case success cannot satisfy that claim. These are unmeasured proposals, not promises or grounds to assume a hardware upgrade.
- Priority: MUST.

### NFR-6: Responsive analytics during a long solve
- Statement: The UI/API shall remain usable while a weather-year job runs.
- Rationale / source: D-8, D-18, D-41, D-47, D-48.
- Acceptance criteria: Proposed OI-10 target: at a 1440 × 900 desktop viewport, with one solver job on the reference workstation and a frozen map fixture, cached run status and selected-asset/interval results have p95 API latency at most 2 seconds over 100 requests; timestamp/filter interaction is acknowledged within 200 ms, with loading state for pending data. The fixture records feature counts, result volume, browser and network conditions. Map level-of-detail may change rendering, never the numerical network. No whole-year all-node payload is required for first render.
- Priority: SHOULD.

### NFR-7: CPU-only portability and optional GPU execution
- Statement: The Rust core shall build and run CPU-only on Linux and the owner's Mac; GPU features shall remain explicit and optional.
- Rationale / source: D-13, D-16, D-20, D-29, D-42, D-43, D-48.
- Acceptance criteria: Linux and macOS CI build/test the default core without CUDA or commercial libraries. GPU CI uses the actual installed supported NVIDIA device and logs model, compute capability, driver/toolkit/backend versions and peak VRAM; it never claims a planned RTX 5060 Ti is installed. GPU results pass the same numerical checks as CPU; unsupported Pascal/device combinations fail capability checks. A missing required GPU run blocks the relevant gate, not silently falling back to a CPU “GPU pass” (OI-7).
- Priority: MUST.

### NFR-8: Licence-clean production dependency closure
- Statement: Production deliverables shall have an explicitly approved dependency and runtime-service licence inventory compatible with the intended future commercial service.
- Rationale / source: D-1, D-3, D-16, D-17, D-28, D-30.
- Acceptance criteria: CI checks locked Rust dependencies and advisories; separately inventories native solver/CUDA binaries, TypeScript dependencies, container images and service licences that `cargo-deny` cannot see. Unreviewed/disallowed licences or unapproved advisories block. Offline validators are isolated from production packaging. No blanket legal claim that all GPL/AGPL use is forbidden or that an Apache wrapper licenses its native dependencies is accepted. The exact allow-list and Redis/native/commercial deployment terms require OI-8 review; no Valkey substitution is authorised by this draft.
- Priority: MUST.

### NFR-9: Diagnostic and incomplete-result integrity
- Statement: The system shall preserve useful diagnostics without presenting invalid or incomplete solves as scientifically valid results.
- Rationale / source: D-18, D-26, D-28, D-33, D-40.
- Acceptance criteria: Tests cover infeasible, unbounded, time-limit-with/without-incumbent, cancelled, solver-error and invalid-data states. Every API/export/UI exposes status, validity, residuals and provenance; an annual completeness assertion fails on any missing interval. Summaries reconcile interval MWh and costs within NFR-2 and name any dropped or invalid interval. Infeasibility diagnostics identify affected constraints where backend capabilities allow.
- Priority: MUST.

### NFR-10: Runtime language and infrastructure boundaries
- Statement: Production numerical modelling and backend code shall be Rust, with a TypeScript web frontend and PostgreSQL 18 plus Redis infrastructure.
- Rationale / source: D-3, D-10, D-16, D-19, D-20.
- Acceptance criteria: The production dependency graph contains no Python/Julia modelling engine or subprocess; those languages appear only in pinned offline validation. Native solver libraries are isolated behind Rust FFI/adapters under D-16 rather than rewritten to achieve literal pure Rust. Default CPU startup does not require CUDA. PostgreSQL/Redis are retained; their detailed storage/job roles are not settled here and ADR 0004 remains deferred.
- Priority: MUST.

### NFR-11: Reviewable scientific method and fidelity limits
- Statement: Each P1 validation release shall include methods and limitations sufficient for independent technical review.
- Rationale / source: D-23, D-26, D-33, D-40, D-44.
- Acceptance criteria: The release report links scenario/data identities, model equations, UC and pricing assumptions, network fidelity, data gaps, calibration/test split, reference results, error tables and hardware measurements. It distinguishes perfect-information historical reconstruction from an as-of forecast backtest and makes no live/day-ahead forecasting claim without information-availability controls. All failed targets and unresolved owner items are listed; reports disclose proprietary input restrictions instead of redistributing unauthorised data.
- Priority: MUST.

### NFR-12: Protected execution and secret handling
- Statement: CI and optional solver integration shall protect credentials, licensed binaries and the self-hosted workstation from untrusted PR execution.
- Rationale / source: D-17, D-22, D-28, D-43.
- Acceptance criteria: A CI threat-model test demonstrates that unapproved PR code cannot acquire GPU-runner access, solver licence secrets, SSH credentials or protected attestation credentials. GPU work executes only after trusted exact-SHA authorisation in an isolated runner; secrets are not committed or exported in run manifests. Required jobs remain blocking if the runner is unavailable. External application exposure is not authorised until the P1 access policy in OI-12 is resolved; this does not implement P4 tenant isolation.
- Priority: MUST.

## Open items for the owner

These items are requests for acceptance/clarification, **not decisions**. The coordinator
should transfer them to stable questionnaire IDs; this task deliberately edits neither
the questionnaire nor STATUS. Numeric criteria above remain proposed until accepted.

| Item | Proposed assumption / question | Affected requirements |
|---|---|---|
| OI-1 | **ASSUMPTION:** P1 uses linear DC, lossless AC branches, explicit interconnector losses, N-0 network constraints, existing grid-scale storage and documented hydro budgets; no AC/N-1 certification. Approve equations, losses, reserves, storage terminal treatment and historical/current asset vintages. | FR-1, FR-6, FR-8, FR-9 |
| OI-2 | **ASSUMPTION:** P1 models cost-based two-market UC and a clearly labelled constraint-redispatch proxy, with post-commitment LP pricing; it does not replay every GB BM bid/offer or full I-SEM settlement rule. Which observed day-ahead auction products, Ireland balancing treatment, external boundary policy and uplift/settlement rules define P1? D-15's full replica remains the end-state obligation; D-26 does not itself approve these proxies. | FR-1, FR-7, FR-10, FR-12 |
| OI-3 | **ASSUMPTION:** Reduced networks are development/validation fixtures only. If the D-31 spike cannot satisfy D-32, accept gaps/equivalents explicitly, commission additional data, or make a new reduced-MVP decision. No fallback is pre-authorised. | FR-2, NFR-5 |
| OI-4 | **ASSUMPTION:** Freeze one full historical test year outside calibration, with at least 99% matched observations per market/product; select the year/products after licence and vintage checks. Proposed price gates: MAE ≤ 20 and RMSE ≤ 40 native currency/MWh, absolute bias ≤ 5 native currency/MWh, and MAE no worse than the previous-week baseline. Proposed mix gates: annual technology-share error ≤ 5 percentage points and mean daily sum of absolute technology-energy errors divided by observed total daily generation ≤ 15%, per market. Zero-denominator days are disclosed, not scored. Approve these business thresholds and a definition-compatible BM/constraint-cost target; absent agreement there is no validated-market claim. Stress/tail errors are reported even when aggregate gates pass. | FR-3, FR-12, NFR-11 |
| OI-5 | **ASSUMPTION:** Use 48-hour UC look-ahead, 24-hour advance, ≤ 0.1% relative MIP gap, the NFR-5 30-second/300-second/48-hour CPU targets and one concurrent simulation. Validate rolling-horizon bias against a longer-horizon fixture. Approve or revise after measurement; agree the absolute gap for near-zero/negative objectives, terminal policies, per-window time limits and one-minute input policy. Approve ADR 0003's benchmark protocol and proposed custom-assembly justification thresholds (20% build/load time or 25% peak memory improvement, or a required missing capability). No optimisation spike is permitted before the code gate opens (D-31). | FR-5, FR-6, FR-11, NFR-5 |
| OI-6 | **ASSUMPTION:** Numerical, manifest-level reproducibility rather than bitwise identical MILP schedules, and the explicit NFR-2 feasibility/objective/price tolerances. Approve scale conventions and repeatability expectations; Q-D-5 remains unanswered. | FR-13, NFR-1, NFR-2 |
| OI-7 | **ASSUMPTION:** HiGHS CPU is the default, with optional GPU/commercial adapters and no assumed GPU speedup. Confirm actual installed GPU, supported Linux/toolkit combination, runner availability and commercial licence entitlements. Benchmark the Ryzen, not the uninstalled CPU/GPU upgrades. | FR-18, NFR-5, NFR-7 |
| OI-8 | **ASSUMPTION:** Unknown or unreviewed licences are blocked pending explicit review. Approve production dependency/service allow-list, data licence evidence process, native solver redistribution and Redis deployment terms; D-30's data prohibition is not automatically a ban on every copyleft software licence. | FR-19, NFR-8 |
| OI-9 | **ASSUMPTION:** Immutable manifests and summaries are retained, but guaranteed reruns require legally retainable, pinned inputs as well as results. Approve input archival/retention and treatment of externally withdrawn datasets; D-36 currently specifies result retention only. | FR-16, NFR-1 |
| OI-10 | **ASSUMPTION:** P1 UI/API interaction means submit, monitor, cancel and inspect deterministic jobs, not in-place live solver editing; simple sequential CLI batches satisfy P1 access while large sweeps wait for P2. Approve FR-20 comparison and NFR-6 viewport/latency targets and clarify D-8's interactive sandbox boundary. | FR-14, FR-15, FR-20, NFR-6 |
| OI-11 | **ASSUMPTION:** Required current-head checks plus a trusted cross-provider attestation enforce D-28. Resolve the current private-plan branch-protection limitation reported in STATUS, review-attestor authority, bootstrap policy and benchmark statistical/noise protocol. Clarify D-51's code-only role versus team.md's retained cross-provider approval rule for specification PRs, and confirm D-53 adversarial-review triage ownership. No failing/missing gate may be waived by an author; no blanket regression allowance is assumed. | NFR-3, NFR-4, NFR-12 |
| OI-12 | **ASSUMPTION:** P1 runs in a private trusted environment; external access waits for an explicit authentication/exposure policy. Confirm Q-L-2/Q-L-3 before exposure, independently of P4 tenant isolation. | NFR-12 |

## Baseline acceptance checklist

The coordinator may propose acceptance only after owner disposition of the scope,
accuracy and performance assumptions; a D-31 feasibility report; and accepted ADRs
0001–0003 under the existing process. This draft does not approve ADR 0002, choose
storage, waive D-28, assert successful benchmarks or change the “no code” phase.
P1 release later requires all MUST criteria and documented treatment of SHOULD items,
including separate evidence for mathematical agreement, historical accuracy and
full-network hardware performance. None substitutes for the others.
