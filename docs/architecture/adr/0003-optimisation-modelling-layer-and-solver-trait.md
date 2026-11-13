# ADR 0003 — Staged optimisation and a solver-neutral Rust contract

- **Status:** Proposed; not performance-qualified and not permission to write application code.
- **Date:** 2026-11-13
- **Linked decisions:** D-1, D-2, D-16, D-18–D-21, D-26, D-28–D-29, D-31–D-32, D-35, D-38, D-43–D-44, D-48, D-55–D-57, D-59–D-60.
- **Related requirements:** FR-1–FR-13, FR-18, FR-21; NFR-1–NFR-5, NFR-7–NFR-12.
- **Dependencies:** ADR 0001 governance; ADR 0002 network/data decisions await D-31. Storage remains ADR 0004's deferred responsibility.

## Context

P1 is full GB/all-island Ireland weather-year UC plus COST-BASED redispatch (D-59),
not historic bids/offers or a complete auction/imbalance-settlement replica. D-55 adds
published secured boundaries, voltage/stability proxies and associated constraint costs
to DC network physics. Full AC and dynamics remain later phases. D-60 includes ALL
GB 132 kV, including DNO-owned England/Wales areas; a reduced graph cannot qualify P1.

Rust constructs/orchestrates models; mature native solvers are permitted (D-16/D-20).
Do not build a branch-and-bound solver or general symbolic modelling framework.
No optimisation spike is authorised: all implementation, scale experiments and solver
benchmarks wait for the coordinator's requirements-baseline gate (D-31).

ASSUMPTION (Q-N-1/2/5/6, former OI aliases): The engineering pipeline and numeric
settings below require baseline adoption. D-55 and D-59 are owner decisions, not open
questions; this ADR cannot lower their fidelity or declare throughput without evidence.

## Proposed decision: energy pre-pass, UC, redispatch, pricing

| Stage | Formulation and retained evidence |
|---|---|
| 0 — long-horizon energy | Annual/seasonal copper-plate LP without commitment binaries, including storage/hydro inflow/energy limits and directional interconnector losses. Derive dated boundary SoC targets or water values; record approximations and residuals. |
| 1 — national schedules | Coupled GB/I-SEM copper-plate UC MILP, with min up/down, ramp/start costs, availability, market reserves, energy targets and commitment-type regional minimum-unit/inertia/SNSP proxies. Export pre-network schedules. |
| 2 — cost-based actions | Full-network DC redispatch LP with stage-1 commitment fixed, directional lossy interconnector injections, secured boundary/flowgate limits, reserves and applicable published proxies. Minimise estimated incremental operating cost relative to the national schedule, plus explicit curtailment/shortage penalties. Export actions/costs, not historic bid acceptance. |
| 3 — national prices | Continuous national pricing LP fixing the stage-1 UC decisions, same costs/reserves and labelled scarcity policy. Reconcile marginal prices and unrecovered non-convex costs; stage-2 nodal/proxy duals remain diagnostic. |

The pre-pass is an energy signal, not an annual globally optimal UC certificate.
Annual targets can be unreachable in a short window; represent a declared target band,
penalty and repair rule, and report deviations. Physical storage bounds, reserve delivery,
minimum up/down and security constraints are never softened silently to hit targets.
A long low-wind event crossing the rolling boundary must not benefit from emptying
storage beyond the look-ahead or from a cost-free restart.

Fixed commitment can make redispatch infeasible. Identify violated rows, feed a
regional commitment requirement or valid feasibility cut to stage 1, and re-solve UC
then redispatch. ASSUMPTION: limit to three repair iterations per window, then mark
failed/partial with diagnostics; never drop the proxy or present the unsolved schedule
as valid. A future fast-start-only MILP requires explicit capability/equivalence tests;
it is not an automatic relaxation of this contract. Save the revised commitment used
for final schedules and re-run its pricing LP; distinguish original and repaired stages.

This decomposition avoids a monolithic branch-flow UC MILP. It does NOT guarantee
that national commitment is globally network-optimal; report repair counts, incremental
cost and approximation bias versus a longer/joint small reference. No fixed unit/bus
counts are asserted before the data inventory. Benchmark the actual D-60 full graph.

## Proxy/security model and loss/currency conservation

Each proxy has a source document/version/hash, effective dates, geography, units,
coefficient derivation and active/missing status. Published secured boundary transfer
limits become flowgates over signed branch flows; minimum online-unit/inertia floors
and the dated Irish SNSP convention enter UC where needed. Do not invent an SNSP
number or use a permanently fixed proxy for every backtest year. Missing evidence
is a declared data gap (FR-21), not an implicit disabled limit.

Apply proxy constraints again in redispatch when dispatch-dependent. Export binding
rows, slack and cost impacts. Commitment rows have no LP dual: use a clearly labelled
fixed-commitment LP sensitivity or finite-difference UC re-solve, never a MILP shadow
price. Proxy security approximates published restrictions; it does not certify full AC,
reactive power, exhaustive contingencies or dynamic stability.

Directional HVDC/interconnector variables distinguish sending and receiving MW with
receiving = sending − non-negative losses. Endpoint nodal balances account for loss
once, even though internal AC DC equations are lossless. Use sourced piecewise loss
curves or an explicit tested approximation; prevent simultaneous opposite-direction
flows from manufacturing negative-price revenue or energy. A continuous relaxation
is permitted only when its no-counterflow property is checked; otherwise use a
supported direction formulation and declare any stage-2 integrality explicitly.

Report native GBP/EUR prices/ledgers, but convert the joint objective into a declared
numeraire with D-38 daily exogenous FX and effective timezone. No FX trading variable,
mixed-currency sum or duplicated midnight conversion. A daily-rate discontinuity may
legitimately change dispatch; reject fictitious energy/cashflow, not the mere existence
of a dispatch response. Test signed prices, both directions and a midnight boundary.

## Sparse construction and network screening

Use one direct column-oriented sparse builder as the production path, emitting a
solver-neutral immutable IR; good_lp plus HiGHS is a readable SMALL-case construction
oracle, not a second production engine or a promotion contest. Pin its solver feature
explicitly, since default features select a different backend. Reuse mature sparse
linear algebra and maintained solver bindings rather than an expression DAG at scale.

The IR includes objective sense/offset, column bounds/integrality, row bounds, CSC
coefficients, stable compact row/column IDs and an external metadata index. Use 64-bit
logical indices with checked native-width conversion. Sum duplicate coefficients,
canonicalise ordering, distinguish valid infinite bounds from illegal NaN/non-finite
coefficients, validate dimensions/scales before FFI, and fingerprint the complete model.
Record columns by type, rows, nonzeros, coefficient ranges and peak matrix/conversion
bytes. Preallocate window blocks; avoid dense time-by-node-by-branch tensors and
millions of heap-allocated row names. Count CSC-to-CSR copies for any future adapter.

Network screening belongs to crates/network, not vendor-specific solver logic. Start
with explicit power-balance/connectivity and active secured boundaries, solve stage 2,
then check ALL omitted branch/proxy limits using sparse DC factors/selected PTDF rows.
Add violated rows, restore an LP basis and re-solve until every retained or omitted row
passes NFR-2, or declare failure. Incremental row addition and LP basis/warm-start
support are REQUIRED HiGHS adapter capabilities, not optional speedups. Cold-start
fallback must be an explicit slow path with equivalence/throughput evidence.

Do not precompute a dense all-bus PTDF as an assumed optimisation; compare its actual
nonzeros/memory with sparse angle/factorisation paths on the inventory. Treat each
island with its own reference and energy balance. Optional critical-contingency LODF
screening needs a sourced shortlist, islanding checks and small-case oracle; full N-1
certification is not added to P1. A final all-row check prevents a screened model from
silently ignoring the constraint that invalidates its cheap solution.

## Solver trait, outcomes and numerical contract

| Operation | Required semantics |
|---|---|
| capabilities/load | Explicit LP/MILP/duals, update/basis/interrupt/diagnostic/device support; validate and load IR without changing integrality, objective or currency |
| add_rows/update/basis | Checked stable IDs, updated model fingerprint, incremental sparse rows, bound/cost updates and LP basis save/restore; reject unsupported operations |
| solve | Time/threads/seed/presolve/feasibility/gap settings are typed, validated and persisted; backend extension options are explicit |
| cancel | Parent-to-worker command with bounded acknowledgement/grace and process-group termination fallback; never concurrent raw-handle calls |
| outcome | Common/native status, feasible incumbent if any, objective/offset, valid best bound, absolute/canonical/vendor gaps, original-unit residuals and termination cause |
| lp_sensitivity | Duals/reduced costs only for a solved continuous model, with sign/scale/interval conversion and degeneracy convention; no MILP dual invention |
| diagnostics/release | Logs/fingerprint and optional IIS/conflict support, clear ownership and RAII cleanup; no forged diagnosis when the backend lacks it |

Map optimal-with-declared-tolerance, feasible_time_limit, infeasible, unbounded,
infeasible_or_unbounded, interrupted, no_incumbent, invalid_model and backend_error
separately. A solver's zero objective on an invalid result cannot become a valid
zero-cost dispatch. Independently recompute physical feasibility before accepting
any incumbent, including after cancellation/time limit.

For incumbent I and best bound B in the same objective units including the same offset:

- Minimisation requires B ≤ I within numeric tolerance; maximisation requires B ≥ I.
  An incompatible bound, missing incumbent or non-finite value gives an UNKNOWN gap,
  not zero. Preserve the raw backend status/value.
- Absolute gap A = abs(I − B). Canonical reporting ratio R = A / max(U, abs(I)),
  where U is one unit of the declared objective currency, with the model scale stored.
  Negative objectives do not reverse the bound relation or make this ratio negative.
- ASSUMPTION (Q-N-5): ordinary UC accepts A ≤ 1 objective-currency unit OR R ≤ 0.001,
  in addition to feasibility. Configure native mip_abs_gap/mip_rel_gap consistently;
  record native gap differences rather than treating vendor denominators as identical.
  The £0.50 versus £0 example passes the absolute economic tolerance, not a bogus
  zero ratio. Test zero, near-zero, negative and sign-crossing objectives, both senses.
- ASSUMPTION (Q-N-6): exact reference fixtures set relative MIP gap ≤1e-6 and absolute
  gap ≤0.001 objective-currency units on BOTH implementations, plus matched feasibility
  tolerances; require a final certificate tight enough for NFR-2's objective comparison.
  A loose default native gap cannot justify a tighter claimed reference match.

Each stage's objective units/offset and tolerances are persisted. LPs use feasibility/
dual optimality certificates, not a fabricated MIP gap. Changing an offset can change
a relative ratio; preserve canonical model identity and report absolute gaps too.

## Safe native lifecycle and HiGHS reuse

Run every native solver session in a worker subprocess, never inside the API process.
The orchestrator exchanges validated models/options/results over bounded versioned
IPC, monitors progress/RSS, and commits only independently checked complete intervals.
A solver crash, OOM, hung callback or invalid output yields backend_error/partial,
not service death or a promoted incomplete incumbent. Kill the entire process group
on forced cancellation and reap it; resume only from persisted committed states.

Each worker exclusively owns its native handle. The public solver/session trait does
NOT require Send + Sync. A separate cancellation token/IPC reader may update an atomic
flag read by the vendor callback; it never moves or concurrently mutates a native
handle. No unsafe Send/Sync implementation merely to fit an async runtime. A mutex
held through a blocking solve cannot be the cancellation design.

ASSUMPTION (Q-N-5): request cooperative interrupt, allow two seconds' grace, then kill
and reap within five seconds total; cancellation from a hung callback and a crash
must be tested. Only an already copied, validated checkpointed incumbent may survive
worker termination, and the run remains interrupted/partial unless all intervals exist.

Reuse highs/highs-sys for maintained build/link/bindings and add only a thin reviewed
safe facade in solver-highs for required C functions. The 2026-11-13 API check found:
Model supports row addition, option/solution updates and raw-pointer access and is
!Send/!Sync; SolvedModel exposes mip_gap and generic double_info_value queries. Thus
it is wrong to claim all bound/gap access requires new FFI. Public callback registration
and LP basis access are not in that high-level method list; use highs-sys bindings
for Highs_setCallback/startCallback and getBasis/setBasis as needed. Pin compatible
wrapper/native versions; never link two HiGHS ABIs in one worker. C callbacks/info
query semantics are documented upstream [S1–S4]. This is API inspection, not a build
or cancellation proof; Linux/macOS conformance is still required after baseline.

## Adapters, capability and qualification

| Backend | Planned role and activation evidence |
|---|---|
| HiGHS | Required open-source CPU LP/MILP, incremental rows/basis, native interrupt, incumbent/bound queries; Linux and macOS builds plus conformance before M2 exit |
| Gurobi / Xpress | Optional feature-gated native adapters; approved runtime/hosting/redistribution terms, matching status/gap/dual/interrupt semantics; licences never required by default CI |
| cuOpt | Trait/capability stub only in P1; no implementation scheduled without a supported actual device. Later pinned ABI/layout/callback/maturity and actual-device CI evidence required; no speedup or MIP maturity assumed |

An unavailable backend returns a capability error. Never silently relax UC to LP,
fall back to another solver or describe a CPU run as a GPU pass. Stub crates must not
link unavailable SDKs. Optional integration terms are reviewed independently of a
wrapper's source licence and D-56 dataset tags.

After baseline, qualify small/medium/full inventory windows and an annual 30-minute
run on D-48's Ryzen 5900X/128 GB Linux host. Record assembly/load/solve/export/p95/max,
process-tree RSS, window dimensions/nonzeros, gaps and repair counts. NFR-5 proposes
48 h look-ahead/24 h advance, 30 s build/load p95, 300 s combined stage solve p95 and
48 h annual elapsed time including pre-pass/output, with no assumed upgrade. Report
peak RSS against the smaller of 96 GiB or 80% measured RAM. These are proposals,
not measurements; reduced cases cannot qualify the full-network claim.

Measure 1/6/12/24 threads and actual scenario/block parallelism. Sequential rolling
windows cannot simply run concurrently: optional weekly parallel blocks must receive
validated pre-pass boundary states and reconcile commitment/storage at joins, with
bias versus sequential results disclosed. Do not sacrifice scientific consistency
for thread utilisation. Benchmark implementation performance, not two production
builder selections. Mutation/reference tests independently perturb losses, reserves,
proxy limits, rolling state and pricing; PyPSA checks UC/network equations, not BM.

## Consequences and unresolved matters

Benefits: much smaller integer core, explicit cost-based BM interpretation, sourced
proxy security, safe cancellation/crash containment and mature solver reuse. Costs:
pre-pass/rolling/repair approximation, worker IPC and sparse update/basis integration
must all be tested. Full-network data/parameters and definition-compatible historic
BM/curtailment evidence remain release risks, not reasons to claim reduced success.

Only Q-N-4 (accuracy acceptance) and conditional Q-N-3 (verified gaps) need owner
answers now. Engineering defaults need explicit baseline adoption. Actual optional
GPU/licence inventory is informational; do not reopen D-59 or D-60. Acceptance of
this Proposed ADR specifies architecture and a post-baseline qualification plan,
not successful benchmarks, a data/storage choice or permission to implement early.

## Primary references (API inspection 2026-11-13; pin at implementation)

- S1 — highs 2.4.0 Model API: https://docs.rs/highs/2.4.0/highs/struct.Model.html
- S2 — highs 2.4.0 SolvedModel API: https://docs.rs/highs/2.4.0/highs/struct.SolvedModel.html
- S3 — highs-sys C bindings: https://docs.rs/highs-sys/latest/highs_sys/
- S4 — HiGHS callbacks and options: https://ergo-code.github.io/HiGHS/dev/callbacks/ and https://ergo-code.github.io/HiGHS/dev/options/definitions/
- S5 — good_lp solver features: https://docs.rs/good_lp/latest/good_lp/
- S6 — PyPSA UC/rolling horizons: https://docs.pypsa.org/latest/user-guide/optimization/unit-commitment/
- S7 — fixed-MIP dual limitations: https://support.gurobi.com/hc/en-us/articles/360034305272-How-do-I-retrieve-the-dual-Pi-values-for-a-MIP-problem
- S8 — cuOpt device requirements: https://docs.nvidia.com/cuopt/user-guide/latest/system-requirements.html

References document capabilities, not this project's performance, commercial clearance
or completed native conformance tests. Numerical thresholds remain ASSUMPTION above.
