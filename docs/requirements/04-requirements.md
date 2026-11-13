# 04 — Formal Requirements

_Status: PROPOSED MVP baseline draft, 2026-11-13. Not accepted; does not enable application code._

## Scope and interpretation

This document specifies **P1 only**: present-day GB and all-island Ireland transmission,
weather-year unit commitment (UC), cost-based constraint/Balancing Mechanism (BM) redispatch,
prices, unserved energy, a web map, and historical validation (D-18, D-26, D-32, D-44).
It does not bring P2 distribution, AC voltages, behavioural demand or live forecasting,
P3 alternative pricing/2030 scenarios, or P4 dynamics/multi-tenancy into P1. Present-day
capability is demonstrated by a dated asset snapshot; historical validation additionally
needs the assets and information available in the historical test period.

Every requirement cites owner decisions. Its implementation detail and numeric criteria
are **proposals**, not additional owner decisions. `MUST` means necessary for acceptance
of this proposed P1 baseline; `SHOULD` means a documented exception may be proposed to
the owner, not silently omitted. No requirement below authorises a scope reduction.

ASSUMPTION: Unless copied from an owner decision, all numeric acceptance targets and
implementation choices below are proposed engineering defaults (Q-N-1–Q-N-12), not
approved decisions. Baseline acceptance must explicitly adopt or amend them.

**Network feasibility flag (R-12):** FR-1 through FR-21 and NFR-1 through NFR-12 apply to
both full and reduced fixtures, except that **FR-2's full-network release gate cannot be
satisfied by a reduced network**. Reduced fixtures enable testing and honest diagnostic
results; shipping a reduced-network MVP requires a new owner decision amending D-32.
ADR 0002 and ADR 0004 remain pending the D-31 data spike.

## Functional requirements

### FR-1: Geography, assets and interconnector boundaries
- Statement: The system shall represent GB and the all-island Ireland system separately, including interconnections between them and the external boundaries named in D-11.
- Rationale / source: D-11, D-18, D-26, D-32, D-38.
- Acceptance criteria:
  - FR-1.AC1: A versioned fixture identifies every bus, generator, demand allocation and interconnector by stable ID and market, validates endpoints/units and contains no unexplained disconnected demand.
  - FR-1.AC2: Interconnector assets and trading regimes have effective-from/to dates. Backtests include only commissioned assets and the capacity-allocation/coupling regime in force on each date; future links or later rule changes cannot leak into earlier years.
  - FR-1.AC3: Each directional interconnector records sending MW, receiving MW and non-negative losses. Receiving MW equals sending MW minus losses; both endpoint balances include the same losses exactly once (NFR-2). A two-market fixture reconciles energy and daily GBP/EUR conversion without double counting.
  - FR-1.AC4: ASSUMPTION: External boundaries use historical flows for reconstruction; scenario flow/price policies must be declared. A daily UTC-effective FX series is exogenous, versioned and valued consistently in a common objective numeraire; no free FX trading variable is allowed (Q-N-1, Q-N-2).
  - FR-1.AC5: Dated fixture pairs cover Brexit coupling/decoupling, MHHS settlement changes, REMA national-versus-zonal rule status and new-link commissioning (including Viking and Greenlink). Each transition uses sourced effective dates; applying a later asset/rule snapshot to an earlier holdout fails validation. P3 zonal scenarios are never presented as historic P1 rules.
- Priority: MUST.

### FR-2: Network completeness and fidelity declaration
- Statement: The system shall ingest the D-32 voltage-level transmission network and disclose its completeness against a dated, licensed inventory.
- Rationale / source: D-30, D-31, D-32, D-40, D-44, D-56, D-60.
- Acceptance criteria:
  - FR-2.AC1: The inventory enumerates GB 400/275/132 kV and Ireland 400/220/110 kV assets. GB includes ALL 132 kV, including every England/Wales DNO LTDS area as well as Scottish transmission (D-60); this exception does not bring lower-voltage feeders or household DER into P1.
  - FR-2.AC2: Coverage tables report numerator/denominator by operator, voltage and asset class, including topology, ratings, impedance, generator/GSP mapping and inferred parameters with provenance. Unknown denominators fail completeness.
  - FR-2.AC3: 100% of dated inventory nodes, circuits and published secured boundaries are modelled or explicitly equivalenced, with owner acceptance of every equivalent or missing asset before full-network release. Missing assets never count as covered; an accepted gap requires an explicit scope decision to claim a reduced release.
  - FR-2.AC4: Reduced fixtures disclose aggregation maps, retained constraints and lost capabilities. Unknown completeness or unresolved R-12 blocks full-network release (Q-N-3).
- Priority: MUST.

### FR-3: Dated operational and weather inputs
- Statement: The system shall ingest the demand, generation availability, weather-derived renewable profiles, fuel/carbon inputs and operational data needed for a deterministic weather-year run and backtest.
- Rationale / source: D-14, D-18, D-23, D-26, D-30, D-35, D-44, D-55, D-56, D-57.
- Acceptance criteria:
  - FR-3.AC1: Every series has units, timezone, source/publication timestamp where available, checksum, licence tag and missing-data mask. Non-leap/leap UTC fixtures contain exactly 17,520/17,568 half-hour intervals with explicit GB/Irish local settlement mappings through both daylight-saving changes.
  - FR-3.AC2: Inputs explicitly include REMIT unavailability/outage notices, fuel/carbon costs, constraint/curtailment outturn and published operating restrictions, with effective dates and as-of publication/revision timestamps. Unavailable as-of vintages prevent forecast-validation claims.
  - FR-3.AC3: GSP demand identifies underlying demand and distribution-connected wind/solar, represented as weather-driven negative demand at the transmission interface. Fixtures reconcile this decomposition with net outturn and reject double subtraction; uncertain embedded estimates are labelled.
  - FR-3.AC4: Missing/imputed intervals and resampling are reported. Undeclared interpolation and silent future-data use fail validation; weather-year replay varies embedded as well as transmission-connected renewable output.
- Priority: MUST.

### FR-4: Canonical versioned TOML scenarios
- Statement: The system shall use versioned TOML files as the canonical scenario definition shared by Rust CLI/SDK, API and TypeScript UI.
- Rationale / source: D-8, D-19, D-20, D-35.
- Acceptance criteria:
  - FR-4.AC1: A schema version, geography/asset snapshot, input-manifest hashes, time range/resolution, market-rule version, network fidelity, solver settings, rolling-horizon settings and seed policy are included or immutably referenced.
  - FR-4.AC2: CLI-to-UI-to-CLI round trips preserve semantic scenario identity; invalid or unsupported schema versions fail with field-level errors; migration never overwrites an existing run's scenario (NFR-1).
- Priority: MUST.

### FR-5: Configurable time resolution
- Statement: The system shall default to 30-minute market runs and support configurable intervals down to one minute without misrepresenting source-data resolution.
- Rationale / source: D-18, D-21, D-35.
- Acceptance criteria:
  - FR-5.AC1: Constant-input 24-hour cases at 30, 15, 5 and 1 minute conserve MWh and currency totals within NFR-2, with documented duration rounding.
  - FR-5.AC2: Non-stationary fixtures at each resolution exercise ramp-limited demand changes, starts/stops spanning minimum up/down durations, an availability drop, a binding reserve and a storage cycle. MW/min ramps and physical hours are scaled by interval length; independent analytic/reference checks detect an intentionally unscaled duration or ramp.
  - FR-5.AC3: Finer-than-source inputs are marked synthetic/resampled. Unsupported mixed product/resolution combinations fail explicitly; no one-minute annual runtime promise is inferred from NFR-5.
- Priority: MUST.

### FR-6: Intertemporal unit commitment
- Statement: The system shall solve deterministic dispatch with generator commitment, start/stop decisions and intertemporal operating constraints.
- Rationale / source: D-18, D-26, D-35, D-44.
- Acceptance criteria:
  - FR-6.AC1: Small sourced fixtures exercise min/max output, initial state/time-in-state, minimum up/down time, ramps, starts/stops, availability and variable costs. Known optimum certificates match NFR-2; alternate optimal schedules are permitted.
  - FR-6.AC2: Each market holds aggregate upward reserve/response from eligible committed headroom and storage, with sourced largest-infeed and margin assumptions. Reserve is deliverable over its duration with energy and network/proxy restrictions checked; a binding-reserve fixture changes commitment. Product-level procurement replication is deferred.
  - FR-6.AC3: Rolling-boundary fixtures carry on/off duration, output, reserve obligations and other intertemporal states; no free restart or end-of-window energy is permitted. ADR 0003 stages national UC, cost-based redispatch and pricing rather than a monolithic full-network UC MILP.
- Priority: MUST.

### FR-7: National-market and redispatch stages
- Statement: The system shall distinguish the GB national and I-SEM market schedules/prices from subsequent network-constrained redispatch, actions and costs.
- Rationale / source: D-15, D-18, D-26, D-38, D-44, D-59.
- Acceptance criteria:
  - FR-7.AC1: A two-market congested fixture exports national pre-redispatch schedules and final network-feasible schedules with reconciled actions, costs, curtailment and unserved energy, separately labelled GB BM proxy and Ireland balancing proxy.
  - FR-7.AC2: P1 redispatch is COST-BASED using estimated plant costs to relieve network/proxy limits, not historic bid/offer replay (D-59). Compare its costs AND volumes with definition-matched historic outturn under FR-12.
  - FR-7.AC3: The dated rule inventory distinguishes GB national scheduling, I-SEM ex-ante DAM/IDM, balancing and imbalance pricing. ASSUMPTION: P1 models a DAM-like scheduling/pricing proxy and cost-based balancing; IDM auction replay and full imbalance settlement are deferred toward D-15 end-state fidelity, not advertised as implemented.
  - FR-7.AC4: GB↔SEM interconnector schedules use the date-effective explicit capacity/intraday trading regime where applicable, not an assumed permanently implicitly coupled auction. Nodal redispatch duals are diagnostic and never relabelled national traded prices.
- Priority: MUST.

### FR-8: Physical feasibility and shortage accounting
- Statement: The system shall enforce the accepted P1 network formulation and distinguish physical shortage from infeasible, unfinished or invalid optimisation.
- Rationale / source: D-18, D-26, D-32, D-40, D-44, D-55.
- Acceptance criteria:
  - FR-8.AC1: Accepted intervals satisfy nodal active-power balance, directional lossy interconnector balances and branch limits within NFR-2. P1 uses N-0 DC PLUS published secured flowgate/boundary limits and commitment/stability proxies (FR-21, D-55); pure N-0 thermal-only operation is a diagnostic mode.
  - FR-8.AC2: Island, bottleneck and shortage fixtures exercise explicitly penalised unserved energy/curtailment with location/interval slack ledgers. Timeouts without incumbents cannot produce zero-price/zero-shortage results.
  - FR-8.AC3: Outputs list active proxies and state that full AC voltage/reactive power, exhaustive N-1 contingency certification and dynamics are not certified. Published secured limits approximate security; they are not merely nameplate branch ratings.
- Priority: MUST.

### FR-9: Existing grid-scale storage and energy budgets
- Statement: The system shall account for existing grid-scale storage and energy-limited generation where present in the accepted P1 asset inventory.
- Rationale / source: D-18, D-26, D-32, D-40, D-44.
- Acceptance criteria:
  - FR-9.AC1: Storage/hydro fixtures enforce power/energy limits, efficiencies, initial state, inter-window continuity and sourced inflow/water budgets. Unknown parameters cannot imply unlimited energy; simultaneous charging/discharging is prevented or explicitly constrained with a tested economic/physical justification.
  - FR-9.AC2: An annual/seasonal continuous pre-pass supplies boundary energy targets or water values to rolling UC. Terminal target/value and any slack penalty are versioned; a low-wind episode crossing a window boundary cannot silently drain storage for free.
  - FR-9.AC3: A longer-horizon fixture checks rolling bias and initial/terminal states, including a discontinuous look-ahead outage and minimum down-time carry-over. Violations are invalid results, not shortage. No household DER or 2030 expansion is implemented.
- Priority: MUST.

### FR-10: Prices, currencies and economic ledgers
- Statement: The system shall report GB prices in GBP/MWh and I-SEM prices in EUR/MWh, with separately defined market, imbalance/proxy and physical-dispatch quantities.
- Rationale / source: D-15, D-26, D-38, D-41, D-57, D-59.
- Acceptance criteria:
  - FR-10.AC1: Every series names product, delivery interval, currency, derivation, rule version and validity. ASSUMPTION: GB Market Index and I-SEM DAM are reference products subject to D-57 availability; the former is NOT an auction day-ahead price. No paid feed is assumed.
  - FR-10.AC2: National prices come from a fixed-UC pricing LP without redispatch network rents. Uniquely priced merit-order, shortage and negative-cost fixtures match analytic marginal prices within 0.01 native currency/MWh. Degenerate prices are flagged with the selected dual convention, not asserted unique.
  - FR-10.AC3: Price-cap/floor and scarcity-penalty policies are explicit. Start/minimum-output fixtures report per-unit revenue versus variable/start/no-load costs and any unrecovered cost; no claim that fixed-LP prices recover non-convex costs or implement uplift is allowed.
  - FR-10.AC4: Daily FX fixtures reconcile native and converted cashflows including midnight transitions and signed prices. A changed exogenous FX input can change dispatch and is disclosed; the test forbids created energy, conversion double counting and an independent currency-arbitrage decision, not genuine economic price differences.
- Priority: MUST.

### FR-11: Complete weather-year execution and run states
- Statement: The system shall execute a full weather year and preserve valid intertemporal state through all solve windows.
- Rationale / source: D-18, D-21, D-26, D-35.
- Acceptance criteria:
  - FR-11.AC1: A full 30-minute test year commits exactly one result per delivery interval without overlap/gaps, using rolling state from the committed advance boundary rather than the end of look-ahead.
  - FR-11.AC2: Window outcomes preserve incumbent/bound/gap/residuals and full run states: queued, running, completed, partial, cancelled and failed. Only complete valid intervals enter a validated annual result.
  - FR-11.AC3: Cancellation, solver crash and restart fixtures preserve prior committed outputs and replay from a persisted boundary state. Worker termination cannot promote partial vectors or create fabricated prices/shortage.
- Priority: MUST.

### FR-12: Historical GB and I-SEM accuracy backtest
- Statement: The system shall publish a reproducible, held-out historical validation report for both markets' prices and generation mix.
- Rationale / source: D-18, D-23, D-26, D-33, D-44, D-55, D-57, D-59; roadmap M5 and risk R-8.
- Acceptance criteria:
  - FR-12.AC1: Freeze a full historical holdout year, calibration/test split and date-effective asset/rule/input vintages before tuning. Separate each market/product/currency and publish matched coverage and all exclusions; thresholds are the ASSUMPTION table below (Q-N-4).
  - FR-12.AC2: Report price MAE/RMSE/bias, negative-price and tail errors, skill against previous-week persistence, technology-share/energy errors, constraint costs, redispatch volumes, curtailment and unserved energy. Use free, honestly labelled sources under D-57.
  - FR-12.AC3: For BM costs/volumes and curtailment, provide a source-definition reconciliation and scored comparable coverage. Missing or incompatible observations are UNVALIDATED, not a pass or permission to drop the metric; no validated-MVP claim until a compatible source or explicit owner scope disposition exists.
  - FR-12.AC4: Publish perfect-information reconstruction separately from an as-of forecast backtest. Matching PyPSA validates implementation only, not historic BM or market accuracy.
- Priority: MUST.

### FR-13: Identical-input offline reference validation
- Statement: The system shall compare its optimisation and network results against PyPSA on identical input data and mathematical assumptions.
- Rationale / source: D-10, D-20, D-26, D-28, D-33.
- Acceptance criteria:
  - FR-13.AC1: A pinned offline PyPSA environment consumes the same checksummed tables for costs, bounds, network equations, snapshots/weights, UC states and rolling-window policy.
  - FR-13.AC2: Its manifest records any unsupported feature and prevents it being silently omitted from the comparison.
  - FR-13.AC3: Small exact UC, dispatch and congested-network cases pass NFR-2; compare objectives, independently checked feasibility and aggregates rather than a unique commitment schedule.
  - FR-13.AC4: Larger representative windows compare incumbent/bound intervals if either MILP is time-limited.
  - FR-13.AC5: Matching PyPSA establishes implementation agreement, not historical market accuracy (FR-12).
  - FR-13.AC6: Reference scripts stay outside production dependencies.
- Priority: MUST.

### FR-14: Analyst web map and dashboard
- Statement: The system shall provide a TypeScript, web-only, dense dark-mode analytics interface showing the present grid and run results.
- Rationale / source: D-8, D-18, D-20, D-46, D-47.
- Acceptance criteria:
  - FR-14.AC1: A Playwright end-to-end script loads a scenario, starts/cancels a run, selects a timestamp/asset and shows GB/I-SEM prices, mix, line loading, curtailment and unserved energy with units and legends.
  - FR-14.AC2: The same script checks filters, time-series drill-down and keyboard navigation without colour-only signals; axe-core reports zero serious/critical violations in these flows.
  - FR-14.AC3: Fidelity, synthetic inputs, partial/invalid solves and proxy prices remain visible; NFR-6 sets proposed responsiveness targets. No native mobile app is required.
- Priority: MUST.

### FR-15: Shared Rust/API interaction and deterministic batches
- Statement: The system shall expose the same scenario submission, status, cancellation and results semantics through its Rust SDK/CLI and API as through the web UI.
- Rationale / source: D-8, D-19, D-20, D-33.
- Acceptance criteria:
  - FR-15.AC1: A contract fixture submits the same TOML through each interface and obtains the same scenario hash and result schema; invalid requests return equivalent structured errors.
  - FR-15.AC2: A CLI batch of two deterministic scenarios produces two independently identified runs and a machine-readable manifest.
  - FR-15.AC3: No distributed sweep capacity, live in-place grid editing or later Python SDK is implied (Q-N-10).
- Priority: MUST.

### FR-16: Result provenance, exports and retention
- Statement: The system shall preserve run summaries and pinned full-resolution results and apply D-36's retention policy to other full-resolution results.
- Rationale / source: D-3, D-19, D-33, D-36.
- Acceptance criteria:
  - FR-16.AC1: Summary records, scenario identity and validation status survive deletion of an unpinned run's full-resolution results after 90 days; pinned results are not deleted.
  - FR-16.AC2: A simulated-clock test covers the 90-day boundary and pin/unpin transitions.
  - FR-16.AC3: Machine-readable exports include units, currencies, timestamps, fidelity and run-manifest identity.
  - FR-16.AC4: Missing archived inputs or expired results are reported honestly; summaries alone do not promise an exact rerun (Q-N-9).
  - FR-16.AC5: Storage layout is reserved for ADR 0004.
- Priority: MUST.

### FR-17: Deterministic extreme-weather scenarios
- Statement: The system shall support reproducible replay of historical weather extremes using the canonical scenario format.
- Rationale / source: D-14, D-18, D-19, D-35, D-44.
- Acceptance criteria:
  - FR-17.AC1: A sourced weather year identifies at least 72 consecutive hours of GB wind capacity factor below 10% with interval demand above that year’s 90th percentile. ASSUMPTION: if no historical joint event meets this definition, publish a synthetic 72-hour stress case alongside the observed low-wind episode, never label it observed (Q-N-4).
  - FR-17.AC2: The interval can run independently with documented initial-state differences. Weather/availability changes, including embedded generation, appear in the diff and manifest.
  - FR-17.AC3: Report deterministic unserved MWh/duration, not probabilistic LOLE/EUE from a single year.
- Priority: MUST.

### FR-18: Solver selection and explicit capability checks
- Statement: The system shall provide a production-capable open-source CPU LP/MILP route and a pluggable boundary for HiGHS, Gurobi, Xpress and cuOpt.
- Rationale / source: D-16, D-20, D-26, D-29, D-43, D-48.
- Acceptance criteria:
  - FR-18.AC1: HiGHS completes the required P1 validation and hardware benchmarks without a commercial licence or GPU.
  - FR-18.AC2: Adapter conformance fixtures cover load, integrality, objective sense, statuses, incumbent/bound/gap, cancellation and invalid input.
  - FR-18.AC3: Optional backends declare supported features, installed/runtime versions and licence/device requirements; unavailable capabilities fail explicitly and never silently relax a MILP to an LP.
  - FR-18.AC4: Commercial production adapters may follow without blocking the default path; the interface must not require their licences in standard CI.
  - FR-18.AC5: ADR 0003 defines the proposed boundary (Q-N-7).
- Priority: MUST.

### FR-19: Dataset licence and lineage enforcement
- Statement: The system shall separate research admission from commercial clearance and retain per-dataset licence evidence and derived-data lineage without share-alike inputs.
- Rationale / source: D-17, D-30, D-31, D-32, D-56.
- Acceptance criteria:
  - FR-19.AC1: Every dataset and transitive derivative records source/version, acquisition date, checksum, terms evidence, attribution obligations, licence status and lineage. Tags are exactly open-commercial or research-only; unknown commercial permission remains research-only, never presumed cleared.
  - FR-19.AC2: D-56 permits publicly downloadable research-only inputs for non-commercial research. Keep their raw files and derived restricted extracts outside git and redistribution; commit download scripts/manifests only. Commercial mode rejects research-only ancestors until tracked clearance in TODO-pre-commercial.md.
  - FR-19.AC3: OSM/ODbL/share-alike inputs remain prohibited even for research. A permissively labelled derivative with a prohibited ancestor fails admission. A tool’s source-code licence does not license its datasets; reference fixtures need test-use rights.
- Priority: MUST.

### FR-20: Analyst scenario comparison
- Statement: The system should allow an analyst to compare two completed deterministic P1 scenarios without altering either run.
- Rationale / source: D-8, D-19, D-23, D-47.
- Acceptance criteria:
  - FR-20.AC1: A comparison shows input differences and deltas in generation, native-currency price summaries, redispatch cost and unserved MWh.
  - FR-20.AC2: Incompatible temporal coverage, currencies or fidelity are flagged rather than silently aligned.
  - FR-20.AC3: This is a limited P1 analytics convenience, not P2 distributed batch sweeps (Q-N-10).
- Priority: SHOULD.

### FR-21: Published security and stability proxies
- Statement: The system shall apply dated published voltage/security/stability constraints as explicit P1 proxies alongside DC network equations, without claiming full AC or dynamic simulation.
- Rationale / source: D-26, D-40, D-44, D-55, D-59.
- Acceptance criteria:
  - FR-21.AC1: Each boundary/flowgate, regional minimum-unit, inertia-floor and Irish SNSP/minimum-conventional-unit proxy records source document, effective date range, units, applicability and inference/missing-data status. Unsupported or missing published constraints are declared, never silently disabled.
  - FR-21.AC2: A secured-boundary fixture binds before a thermal nameplate limit; an Irish SNSP fixture forces conventional commitment and renewable curtailment. An inertia/minimum-unit fixture changes UC and preserves reserves and loss accounting.
  - FR-21.AC3: Export active/binding proxies, slack, redispatch cost contributions and duals where meaningful. Commitment-MILP rows have no invented shadow price; proxy sensitivity is from a labelled fixed-commitment LP or finite-difference re-solve.
  - FR-21.AC4: Known violated rows after constraint screening prevent acceptance. A proxy-induced infeasible fixed commitment triggers a documented bounded UC re-solve with added constraints or a failed/partial outcome, not relaxed security limits.
- Priority: MUST.

## Non-functional requirements

### NFR-1: Reproducible run identity
- Statement: The system shall make each accepted result traceable to immutable scenario, data, engine, modelling and solver identities.
- Rationale / source: D-19, D-20, D-28, D-33, D-35.
- Acceptance criteria:
  - NFR-1.AC1: The run manifest includes canonical scenario hash, input and model hashes, engine commit, dependency locks, solver/build versions, parameters, seeds, thread counts, platform, pricing method and timestamp policy.
  - NFR-1.AC2: Two pinned deterministic CPU reruns on the same environment agree within NFR-2 and preserve the same input identity; differing solver/platform settings produce distinct execution identities.
  - NFR-1.AC3: Bitwise cross-platform/MILP reproducibility is not asserted without Q-N-6 approval.
- Priority: MUST.

### NFR-2: Numerical correctness and reference tolerances
- Statement: The system shall use stated, independently checked numerical tolerances for feasibility and reference agreement.
- Rationale / source: D-26, D-28, D-33, D-40.
- Acceptance criteria:
  - NFR-2.AC1: Proposed Q-N-6 thresholds: active-power residual or limit excess at most 0.001 MW plus 0.000001 times the relevant local MW scale; integrality distance at most 0.000001; exact small-case objective difference at most 0.01 native currency plus 0.000001 times the absolute reference objective.
  - NFR-2.AC2: Energy recurrences use the equivalent MWh tolerance after interval scaling.
  - NFR-2.AC3: Tests independently recompute residuals in original units.
  - NFR-2.AC4: For non-unique/time-limited MILPs compare feasibility and compatible objective/bound certificates, not equal binaries; unsupported or incomparable cases cannot count as reference passes.
  - NFR-2.AC5: Price-method tests use 0.01 native currency/MWh tolerance on uniquely priced small fixtures.
  - NFR-2.AC6: Exact reference solves use matched feasibility tolerances, relative MIP gap at most 1e-6 and absolute gap at most 0.001 objective-currency units on both sides, with final certificates tight enough for AC1. ADR 0003 defines signed/zero-objective gap handling; missing certificates are not exact-reference passes.
- Priority: MUST.

### NFR-3: Enforced autonomous merge gates
- Statement: Every PR shall satisfy applicable D-28 quality gates and independent cross-provider review under ADR 0001's staged bootstrap; activated automated gates shall be decided by CI only, never replaced by an unrecorded local pass.
- Rationale / source: D-22, D-28, D-43, D-49, D-51, D-52, D-53, D-54, D-58, D-61, D-62.
- Acceptance criteria:
  - NFR-3.AC1: ADR 0001 defines documentation bootstrap, CPU implementation and optional GPU profiles. The merge-gates aggregator verifies the CURRENT head/base and every applicable D-28 gate; required failures, pending/cancelled/skipped/inconclusive checks block.
  - NFR-3.AC2: Profile selection is derived from trusted base policy, not an author skip label. Documentation-only checks give explicit N/A for absent numerical/build targets without pretending they ran. First benchmark baselines require a separate reviewed seed step; later baseline resets cannot occur in the measured PR.
  - NFR-3.AC3: Current-head independent review, artifact-to-role checks and D-53 adversarial triage are required status evidence, not self-approving GitHub reviews. Under a shared identity this is attested cooperation, not cryptographic provider proof.
  - NFR-3.AC4: D-52 scans PR-range commit messages, title/body and added lines, including metadata edits; D-61 exempts only the bot-managed summary region, never agent-written text. A versioned local commit hook strips attribution; CI rejects rather than edits.
  - NFR-3.AC5: Until D-58 publicity preconditions and required-check protections are in place, the coordinator verifies the same exact-head gate manifest before merging. This is discipline, not enforced branch protection; changing visibility is outside this PR.
- Priority: MUST.

### NFR-4: No benchmark regression
- Statement: A PR shall not merge with a demonstrated benchmark regression relative to the protected-main baseline on matched hardware and workloads.
- Rationale / source: D-2, D-28, D-43, D-48.
- Acceptance criteria:
  - NFR-4.AC1: Benchmarks report build/load/solve/export and wall times, peak process-tree RSS, model dimensions/nonzeros and gaps. Paired main/candidate runs use identical hardware, inputs and solver settings.
  - NFR-4.AC2: ASSUMPTION: At least five serial paired repetitions; block if median wall time regresses by more than 5% or RSS by more than 10% AND the bootstrap 95% confidence interval of that ratio excludes 1.0. Record smaller changes as trends. A threshold-exceeding but inconclusive result requires reruns or remains blocking (Q-N-11).
  - NFR-4.AC3: Reviewed baseline seeds/resets use separate changes; the measured PR cannot rebaseline itself. This proposed noise/effect-size policy needs explicit baseline adoption under D-28, not an assertion that regressions are authorised.
- Priority: MUST.

### NFR-5: CPU performance on the actual owner workstation
- Statement: The default CPU path shall be measured against a fixed P1 workload on the D-48 Ryzen 9 5900X, 12 cores/24 threads, 128 GB RAM Linux workstation.
- Rationale / source: D-2, D-18, D-21, D-26, D-29, D-34, D-48.
- Acceptance criteria:
  - NFR-5.AC1: Proposed Q-N-5 targets: with one active simulation, 30-minute resolution, 48-hour look-ahead/24-hour advance and UC relative MIP gap at most 0.1% OR absolute gap at most 1 objective-currency unit (plus NFR-2 feasibility), model build/load p95 is at most 30 seconds per window, combined UC/redispatch/pricing p95 at most 300 seconds per window, and a full non-leap year including energy pre-pass, repair and output writing at most 48 hours.
  - NFR-5.AC2: Peak simulation process-tree RSS is at most the smaller of 96 GiB and 80% of measured system RAM, with no swap dependence.
  - NFR-5.AC3: Report max window time and gap failures as well as p95.
  - NFR-5.AC4: A licensed, frozen full D-32 fixture is required to establish full-network feasibility; reduced-case success cannot satisfy that claim.
  - NFR-5.AC5: These are unmeasured proposals, not promises or grounds to assume a hardware upgrade.
- Priority: MUST.

### NFR-6: Responsive analytics during a long solve
- Statement: The UI/API shall remain usable while a weather-year job runs.
- Rationale / source: D-8, D-18, D-41, D-47, D-48.
- Acceptance criteria:
  - NFR-6.AC1: Proposed Q-N-10 target: at a 1440 × 900 desktop viewport, with one solver job on the reference workstation and a frozen map fixture, cached run status and selected-asset/interval results have p95 API latency at most 2 seconds over 100 requests; timestamp/filter interaction is acknowledged within 200 ms, with loading state for pending data.
  - NFR-6.AC2: The fixture records feature counts, result volume, browser and network conditions.
  - NFR-6.AC3: Map level-of-detail may change rendering, never the numerical network.
  - NFR-6.AC4: No whole-year all-node payload is required for first render.
- Priority: SHOULD.

### NFR-7: CPU-only portability and optional GPU execution
- Statement: The Rust core shall build and run CPU-only on Linux and the owner's Mac; GPU features shall remain explicit and optional.
- Rationale / source: D-13, D-16, D-20, D-29, D-42, D-43, D-48.
- Acceptance criteria:
  - NFR-7.AC1: Linux and macOS CI build/test the default core without CUDA or commercial libraries.
  - NFR-7.AC2: GPU CI uses the actual installed supported NVIDIA device and logs model, compute capability, driver/toolkit/backend versions and peak VRAM; it never claims a planned RTX 5060 Ti is installed.
  - NFR-7.AC3: GPU results pass the same numerical checks as CPU; unsupported Pascal/device combinations fail capability checks.
  - NFR-7.AC4: A missing required GPU run blocks the relevant gate, not silently falling back to a CPU “GPU pass” (Q-N-7).
  - NFR-7.AC5: GPU gates are applicable only to an implemented/enabled GPU-feature change under ADR 0001; a capability stub is not a GPU implementation. No cuOpt implementation is scheduled in CPU-first P1.
- Priority: MUST.

### NFR-8: Licence-clean production dependency closure
- Statement: Production deliverables shall have an explicitly approved dependency and runtime-service licence inventory compatible with the intended future commercial service.
- Rationale / source: D-1, D-3, D-16, D-17, D-28, D-30.
- Rationale detail: Software copyleft, native redistribution and dataset permissions are different obligations; neither a blanket ban on all GPL/AGPL use nor a wrapper's licence establishes the entire deployment's rights.
- Acceptance criteria:
  - NFR-8.AC1: CI checks locked Rust dependencies and advisories; separately inventories native solver/CUDA binaries, TypeScript dependencies, container images and service licences that `cargo-deny` cannot see.
  - NFR-8.AC2: Unreviewed/disallowed licences or unapproved advisories block.
  - NFR-8.AC3: Offline validators are isolated from production packaging.
  - NFR-8.AC4: The exact allow-list and Redis/native/commercial deployment terms require Q-N-8 review; no Valkey substitution is authorised by this draft.
- Priority: MUST.

### NFR-9: Diagnostic and incomplete-result integrity
- Statement: The system shall preserve useful diagnostics without presenting invalid or incomplete solves as scientifically valid results.
- Rationale / source: D-18, D-26, D-28, D-33, D-40.
- Acceptance criteria:
  - NFR-9.AC1: Tests cover infeasible, unbounded, time-limit-with/without-incumbent, cancelled, solver-error and invalid-data states.
  - NFR-9.AC2: Every API/export/UI exposes status, validity, residuals and provenance; an annual completeness assertion fails on any missing interval.
  - NFR-9.AC3: Summaries reconcile interval MWh and costs within NFR-2 and name any dropped or invalid interval.
  - NFR-9.AC4: Infeasibility diagnostics identify affected constraints where backend capabilities allow.
- Priority: MUST.

### NFR-10: Runtime language and infrastructure boundaries
- Statement: Production numerical modelling and backend code shall be Rust, with a TypeScript web frontend and PostgreSQL 18 plus Redis infrastructure.
- Rationale / source: D-3, D-10, D-16, D-19, D-20.
- Acceptance criteria:
  - NFR-10.AC1: The production dependency graph contains no Python/Julia modelling engine or subprocess; those languages appear only in pinned offline validation.
  - NFR-10.AC2: Native solver libraries are isolated behind Rust FFI/adapters under D-16 rather than rewritten to achieve literal pure Rust.
  - NFR-10.AC3: Default CPU startup does not require CUDA.
  - NFR-10.AC4: PostgreSQL/Redis are retained; their detailed storage/job roles are not settled here and ADR 0004 remains deferred.
- Priority: MUST.

### NFR-11: Reviewable scientific method and fidelity limits
- Statement: Each P1 validation release shall include methods and limitations sufficient for independent technical review.
- Rationale / source: D-23, D-26, D-33, D-40, D-44.
- Acceptance criteria:
  - NFR-11.AC1: The release report links scenario/data identities, model equations, UC and pricing assumptions, network fidelity, data gaps, calibration/test split, reference results, error tables and hardware measurements.
  - NFR-11.AC2: It distinguishes perfect-information historical reconstruction from an as-of forecast backtest and makes no live/day-ahead forecasting claim without information-availability controls.
  - NFR-11.AC3: All failed targets and unresolved owner items are listed; reports disclose proprietary input restrictions instead of redistributing unauthorised data.
- Priority: MUST.

### NFR-12: Protected execution and secret handling
- Statement: CI and optional solver integration shall protect credentials, licensed binaries and the self-hosted workstation from untrusted PR execution.
- Rationale / source: D-17, D-22, D-28, D-43.
- Acceptance criteria:
  - NFR-12.AC1: Negative tests reject an unauthorised SHA, a fork PR on the self-hosted runner, licence/SSH-secret reads, host/container escape via a Docker socket mount and a stale review. No pull_request_target job executes PR code; no licence secrets or tokens enter result manifests.
  - NFR-12.AC2: Trusted exact-SHA jobs run as an unprivileged account in ephemeral rootless containers with no Docker socket or host credentials. The limited runner token is the only host credential; jobs cannot read it. Enabling a GPU adapter requires actual device isolation/toolkit verification and its applicable CI gate.
  - NFR-12.AC3: P1 application access defaults to localhost/private VPN without public endpoints; Q-L-2/Q-L-3 must be resolved before external exposure. Missing REQUIRED runners block only their applicable profile, never silently become a CPU GPU-pass.
- Priority: MUST.

## Staged P1 delivery and acceptance

Milestones retain the order in 06-roadmap.md (D-18, D-26, D-28, D-31, D-44).
M2 subdivisions are engineering sequencing, not new owner milestones or scope cuts.
A MUST spanning stages is not complete until its final evidence exists.

| Milestone | Requirement allocation | Exit evidence |
|---|---|---|
| M0 — governance/bootstrap | NFR-3, NFR-8, NFR-10, NFR-12; FR-18 contract | Adopted ADRs, versioned hook and applicable documentation gates; no application code until coordinator baseline |
| M0.5 — data feasibility | FR-2, FR-3, FR-19 (D-31 only) | Hashed provenance/coverage inventory including all DNO 132 kV; explicit R-12 gaps and research/commercial tags |
| M1 — ingest/network | FR-1–FR-5, FR-19, FR-21 inputs; NFR-1–NFR-2 foundations | Dated assets/rules/calendar, embedded generation and lossy boundary fixtures; model identities |
| M2a — schedules | FR-6, FR-9, FR-11, FR-18; NFR-2, NFR-5, NFR-7, NFR-9 | Long-horizon energy pre-pass then two-market UC with reserves/commitment proxies; rolling-state/cancellation tests |
| M2b — constraints | FR-7–FR-8, FR-21; NFR-4 | Fixed-commitment DC cost-based redispatch, full-row feasibility checks and bounded UC repair |
| M2c — prices/reference | FR-10, FR-13; NFR-2, NFR-4–NFR-5 | National pricing LP, independent UC/network reference fixtures and full-network CPU timing; no PyPSA BM claim |
| M3 — service/results | FR-4, FR-11, FR-15–FR-16; NFR-1, NFR-6, NFR-9–NFR-10, NFR-12 | Shared SDK/CLI/API contracts, worker isolation, retention and pinned-input archival |
| M4 — map/analytics | FR-14, FR-20; NFR-6 | E2E/accessibility flows, visible fidelity/validity and responsive comparison |
| M5 — historical validation | FR-12, FR-17; NFR-11; final FR-2 and NFR-5 evidence | Frozen holdout, all accuracy/coverage gates, stress results, full-network completeness and limitations report |

All 21 FRs and 12 NFRs are allocated above. P1 release requires every MUST, including
full D-60 network coverage; completing an early reduced fixture is not release.

## Proposed numeric backtest accuracy thresholds

ASSUMPTION (Q-N-4, alias OI-4): These are business acceptance proposals for owner
approval, not measured accuracy or new decisions. Freeze year/products/denominators
before tuning; score GB and I-SEM separately with native currency and dated assets.
Use S = mean absolute observed price (not signed mean); require S > 0. Report absolute
metrics too. Missing/zero denominators yield unvalidated metrics, never automatic passes.

| Metric | Proposed acceptance gate |
|---|---|
| Price coverage | At least 99% of eligible intervals per market/product; disclose excluded and tail intervals |
| Price error | MAE / S ≤ 15%; RMSE / S ≤ 30%; absolute bias / S ≤ 5% |
| Price baseline skill | 1 − MAE(model) / MAE(previous-week same-interval persistence) > 0 on identical matched intervals; zero baseline error requires an explicit owner disposition |
| Generation mix | Annual technology-share absolute error ≤ 5 percentage points per technology; mean daily sum of absolute technology-MWh errors / observed daily total ≤ 15% |
| BM/constraint cost | Per market, monthly absolute cost errors summed / sum of absolute observed monthly costs ≤ 25%; annual absolute bias ≤ 15% of that denominator |
| Redispatch volume | Monthly absolute errors summed / observed annual gross up-plus-down MWh ≤ 20%; annual absolute volume bias ≤ 10% |
| Curtailment | Monthly absolute MWh errors summed / observed annual curtailed MWh ≤ 20%; annual absolute energy bias ≤ 10% |
| BM/curtailment coverage | Definition-matched observations for all 12 months and ≥99% of eligible source intervals, or ≥99% of eligible source records for genuinely monthly-only sources; gaps in either cost OR volume block that validation claim |
| Unserved energy | Zero false-shortage intervals on the held-out observed-adequate set at NFR-2 tolerance; if observed shortage exists, event total MWh error ≤ 10% and duration error ≤ one model interval per event |
| Stress/tails | Publish errors on top/bottom 1% observed-price intervals and FR-17 stress cases even when aggregate gates pass; never exclude them to pass |

Cost/volume signs, constraint versus energy-balancing actions and settlement versus
production-cost definitions require an explicit reconciliation. Compare against
published outturn, not PyPSA or a differently defined aggregate. If compatible historic
BM/curtailment observations or adequacy evidence cannot be obtained, report the
metric as unvalidated and raise Q-N-3/Q-N-4 before any validated-MVP release.

## Owner queue and engineering defaults

The canonical register is questionnaire section N (Q-N-1–Q-N-12); OI-1–OI-12 remain
aliases, not renumbered or deleted history. Only **two owner questions** remain for
this baseline: Q-N-4 (accept/edit accuracy gates) and conditional Q-N-3 (dispose of
verified data gaps). D-58, D-59, D-60 and D-61 settle publicity, BM fidelity, all-132-kV
scope and the bot-summary policy respectively; do not ask them again.

ASSUMPTION: Q-N-1/2/5/6/7/8/9/10/11/12 are proposed engineering defaults, not owner
answers. Their concrete policies appear here and in ADRs 0001/0003; baseline adoption
must accept/amend them explicitly. Actual GPU/licence inventory is informational and
non-blocking for CPU P1. Licence substitution or external exposure still needs a
separate future owner decision if a verified conflict is found.

## Baseline acceptance checklist

The coordinator may propose acceptance after Q-N-4 disposition, D-31 feasibility/gap
handoff (Q-N-3 if needed), explicit adoption/amendment of engineering assumptions,
and accepted ADRs 0001–0003. No optimisation spike, application code, storage choice,
branch-protection claim or benchmark success is authorised by this draft. ADR 0004
and data-model ADR 0002 await the spike. STATUS and decisions log remain coordinator-owned.
P1 release additionally needs every MUST, documented SHOULD disposition and separate
implementation-agreement, historic-accuracy and full-network performance evidence.
