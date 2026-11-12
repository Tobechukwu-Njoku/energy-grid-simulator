# ADR 0003 — Rust optimisation modelling layer and solver trait

- **Status:** Proposed
- **Date:** 2026-11-12
- **Linked decisions:** D-1, D-2, D-10, D-16, D-18, D-19, D-20, D-21, D-26, D-28, D-29, D-31, D-32, D-35, D-38, D-43, D-48.
- **Related requirements:** FR-5–FR-13, FR-18; NFR-1–NFR-5, NFR-7–NFR-11 in `../../requirements/04-requirements.md`.
- **Dependencies:** ADR 0001 for workspace/CI; ADR 0002 remains unwritten pending the data spike. No dependency on a proposed storage design.

## Context

P1 requires intertemporal UC and network-constrained redispatch, not only a continuous
dispatch LP. D-20 requires Rust production modelling, but D-16 explicitly permits
native open-source/commercial solvers. The objective is therefore **Rust-first model
construction and orchestration with native solver adapters**, not a new pure-Rust
branch-and-bound engine. Python/Julia modelling remains offline validation only.

The earlier review correctly raised maturity risk, but “there is no Rust modelling
layer” overstates it. `good_lp` has an LP/MILP expression API and multiple solver
features, and `highs` supports direct row/column construction including integrality
[S1, S2]. The unknown is full GB+IE UC scale, not whether Rust can express an MILP.
HiGHS itself provides LP/MIP solvers and native interfaces [S3]. None of those sources
demonstrates this project's annual performance on the owned 12-core Ryzen/128 GB host.

**D-31 restriction:** do not implement or run the optimisation benchmark until the
requirements/ADR baseline permits application code. This ADR is a proposal that can
be accepted with a later measurement checkpoint; a pre-baseline engine spike is not
a prerequisite disguised as documentation. No crate/version/backend is declared
performance-qualified here.

## Decision (proposed)

### Reuse solvers; keep our model boundary small

Adopt a narrow solver-neutral **linear mixed-integer model contract** in the
`optimisation` crate. Use a frozen sparse representation at the native solver
boundary, not solver-specific expression objects throughout the domain engine.
Keep two benchmarkable construction/load paths initially:

1. Reuse `good_lp` + HiGHS as the high-level modelling candidate and small-model oracle.
2. Assemble the same UC/dispatch coefficient specification into compact sparse arrays
   and load through the `highs` crate; use a narrowly scoped C-API shim only when a
   measured required feature is unavailable through the maintained wrapper.

Disable `good_lp`'s default solver features and enable HiGHS explicitly for this
comparison; the documented default is a different native solver [S1].

The **provisional production candidate** is direct sparse loading for predictable
bulk assembly and consistent adapter semantics. This is not a claim of superior
measured speed. Prefer the reused high-level path if it meets the same contract,
scale and memory targets without a demonstrated reason for the custom assembly.
The benchmark checkpoint below decides; do not maintain two full production model
engines indefinitely or implement a general-purpose symbolic/algebraic framework.

The neutral contract is necessary for adapters regardless of which assembly helper
wins. In a `good_lp`-backed adapter it can be translated into expressions and loaded
through that library; **do not assume good_lp exposes a solver-neutral sparse export
API**. Measure conversion costs and any duplicate matrix allocations. A direct
`good_lp` domain-construction benchmark also measures reuse without the neutral-IR
conversion tax, so the comparison does not rig the outcome against reuse.

### Existing Rust options to evaluate

| Option | Documented capability / fit | Limit or evidence still required | Proposed use |
|---|---|---|---|
| `good_lp` | Rust linear expressions, integer variables, multiple solver features, including HiGHS; MIT metadata [S1] | Solver-specific feature coverage, bound/gap/cancellation access and expression/translation memory at full UC size must be tested | First reuse candidate and readable small-case oracle; production if it satisfies the contract/benchmarks |
| `highs` crate | Rust wrapper over native HiGHS; row/column model APIs and integer columns [S2] | Native build portability, exposed options/status/bound information, bulk-load/update costs and version pinning | Default CPU adapter candidate; reuse the wrapper before adding FFI |
| `lp-modeler` | Rust LP/MILP DSL and solver bindings; text-model examples [S4] | Its relevant solve path may involve LP/MPS text/process overhead; direct bulk HiGHS and full status support must not be assumed | Small-model comparator; not the leading large-array integration |
| `microlp` | Pure-Rust linear/mixed-integer option exposed by `good_lp` [S1] | No evidence here of competitive full-network UC scaling | Tiny portable test baseline, not the required performance solver |
| Clarabel / Ipopt | Continuous conic and nonlinear optimisation respectively [S5, S6] | Neither supplies this UC branch-and-bound MILP path | Possible later continuous subproblems, not substitutes for P1 MILP |
| Project-owned sparse assembler | Direct domain loops into compact coefficient arrays, with complete model metadata | Additional correctness/maintenance burden; must earn its complexity against reuse | Bounded candidate, not a new solver, autodiff engine or universal DSL |

Do not infer dataset/native-library rights from crate metadata. Pin the actual Cargo
and native solver versions, checksums, build options and licence evidence when the
benchmark is implemented. Current “latest” documentation is capability evidence, not
a reproducible dependency lock or an endorsement of an untested future release.

### Sparse model and assembly contract

Represent only the P1 linear/quadratic-free MILP/LP subset:

- Objective sense, coefficient vector and constant offset, with native-currency and
  interval weighting defined by the market model rather than guessed by adapters.
- Continuous/integer/binary column types, lower/upper bounds and initial-state data
  compiled into explicit bounds/rows. Binary bounds are validated as 0/1-compatible.
- Row lower/upper bounds and a canonical compressed sparse column matrix, with stable
  row/column IDs tied to asset/time/constraint metadata. Use 64-bit logical indices;
  range-check conversions for a backend exposing narrower native indices.
- Deterministic ordering, duplicate-coefficient summation and explicit treatment of
  zeros. Reject NaN/non-finite coefficients; bounds may use a documented unbounded
  sentinel translated safely to each backend. Check dimensions, finite objectives,
  row/column bounds and coefficient scaling before invoking native code.
- Immutable model fingerprint and statistics: variables by type, rows, nonzeros,
  coefficient range and approximate matrix bytes. Avoid storing millions of allocated
  names/strings in the hot matrix; use compact IDs and a separate metadata lookup.

Build local sparse blocks for unit/time constraints and network/time constraints,
then combine with checked offsets. Preallocate from counted rows/nonzeros where
possible; stream or reuse window buffers instead of cloning an annual matrix.
Start serial and deterministic; parallelise block assembly only after measured need
and stable canonical ordering. Do not introduce dense time-by-bus-by-branch tensors.

Compare sparse angle/flow formulations against sensitivity formulations only after
the network is known: a PTDF can be dense and is not automatically the faster or
smaller representation. Electrical formulation/loss/security choices are OI-1 and
ADR 0002 concerns, not settled by the sparse container. Keep network data structures
out of this ADR's solver abstraction.

Adapters convert CSC to another required layout, such as cuOpt's documented CSR,
without changing row semantics; include that allocation/time in measurements [S9].

### Solver trait and lifecycle

The trait describes semantics rather than prescribing a vendor's object API:

| Operation / value | Required contract |
|---|---|
| `capabilities` | LP, MILP, LP duals, warm starts, model updates, cancellation, diagnostics and device requirements are explicit; unsupported operations return errors |
| `load` | Validate and load the frozen model; report dimensions/load time; do not change integrality or objective sense silently |
| `solve` | Typed options for time limit, threads, seed where supported, presolve, tolerances and optimality targets; validated backend-specific extensions are recorded |
| `cancel` | Cooperative interruption/timeout with bounded lifecycle and explicit interrupted state; declare whether a real native interrupt or an isolated worker termination is used |
| `outcome` | Common status, optional primal vector, objective including offset, best bound, canonical absolute/relative gap, feasibility residuals and native termination details |
| `lp_sensitivity` | Row dual/reduced-cost data only for a solved continuous model with a declared sign convention; unsupported/degenerate cases are not fabricated |
| `diagnostics` | Native log, model fingerprint and optional IIS/conflict output where supported; missing IIS is not a fabricated diagnosis |
| `release` | RAII-owned native resources and clear transfer/copy ownership; no unsafe cross-thread reuse of vendor handles |

Normalised outcomes distinguish `optimal`, `feasible_time_limit`, `infeasible`,
`unbounded`, `infeasible_or_unbounded`, `interrupted`, `no_incumbent`, `invalid_model`
and `backend_error`. “Optimal” must include the declared tolerance/gap, not imply
exact rational optimality. Preserve the raw native status as evidence. Canonical
absolute gap is the absolute incumbent/bound difference; the proposed relative
gap uses `max(1, abs(incumbent))` as denominator. Record vendor-reported gap too;
do not compare incompatible vendor denominators, missing bounds or near-zero
objectives without the OI-5 absolute-gap policy. Test both minimisation and maximisation.

An engine/window orchestrator owns solver sessions, carries commitment/output/storage
state between windows, and commits only the advance interval. It may reuse models or
warm starts when the backend supports them, but correctness cannot depend on those
optional accelerators. Initial/terminal policies and look-ahead bias remain visible.
No fallback relaxes binary variables or switches solver without recording a new
execution identity and explicitly applying the selected scenario policy.

### Adapter plan

| Backend | Integration and role | Evidence needed before enabling |
|---|---|---|
| HiGHS | Default open-source CPU path through the `highs` crate; native sparse LP/MIP solver [S2, S3] | Linux/macOS builds; integrality/status/bound/gap/LP-dual access; actual memory/thread scaling |
| Gurobi | Optional feature-gated Rust adapter over the vendor C API; bulk sparse load [S7] | Licensed runtime, index/ABI checks, interrupt/status/dual semantics and entitlement for intended deployment |
| Xpress | Optional feature-gated native Optimizer C adapter; FICO documents C model-load/solve examples [S8] | Approved SDK/runtime, correct integer-column load path, limits/status mapping, licence availability and SaaS terms |
| cuOpt | Optional CUDA adapter using the documented LP/MILP C interface, not a Python service [S9] | Supported installed device/driver/toolkit, host-device transfer accounting, VRAM, capability parity and native-library terms |

HiGHS must pass the P1 suite without CUDA or commercial licences. Commercial/GPU
adapters can remain unavailable until their capabilities are verified; the boundary
and conformance tests exist without making those SDKs default build dependencies.
All four adapters must preserve MILP semantics when enabled; unavailable optional
adapters are not claimed as implemented. Ipopt is a later NLP extension, not forced
into this linear trait; cuDSS is a linear-system component, not a UC MILP backend.

NVIDIA's checked 26.10 overview labels MIP beta, and its 26.10 release notes split
the native package into client, mathopt and routing components [S12]. Pin and test
the exact C headers/library ABI and scalar/index widths; do not bind a Rust adapter
to an assumed stable older LP/MILP URL or package layout. GPU capability is more
than successful CUDA linkage.

For prices, the engine explicitly fixes accepted commitment and solves the agreed
continuous pricing problem where that is the approved method. Gurobi's guidance
confirms that ordinary MIP duals are unavailable and fixed-model duals have limitations
[S10]. Such prices remain labelled proxies unless the owner accepts the market
method; native duals do not by themselves replicate GB/I-SEM traded prices or uplift.

### Benchmark and qualification plan (after baseline only)

**No results have been measured.** Commit fixture manifests and the protocol before
timing implementations. Use the actual D-48 Ryzen 5900X/128 GB Linux host; record
measured RAM, OS, compiler/native versions, power state and thread configuration.
Run CPU counts 1/6/12/24 where supported; more threads are not presumed faster.
Mac testing establishes portability, not D-48 throughput. GPU tests record the
actually installed card; neither planned upgrade is used as an acceptance baseline.

1. **Exact conformance:** small synthetic/licensed LP and MILP cases with known
   optima, alternate optima, negative/zero objectives, offsets, mixed row bounds,
   integer infeasibility, disconnected networks, shortage and time limits. Check
   original-unit residuals independently. Check continuous pricing separately.
2. **Assembly/load comparison:** build the same frozen coefficient specification
   through `good_lp` and direct sparse/HiGHS paths, with a small `lp-modeler` comparator.
   Test a 24-hour toy, progressively enlarged 48-hour cases and the spike's full
   network 48-hour case. Count preprocessing, expression construction, canonical
   assembly, conversion, native copy/load and teardown; count every live matrix
   allocation. Model equality is established with coefficient/bound/type checks
   on the shared specification and exported/native inspection where supported,
   plus independently checked optima/feasibility, not only a same-sized matrix.
3. **Solver comparison:** solve identical MILPs with matched original-unit feasibility
   tolerances and canonical gaps. HiGHS is mandatory; Gurobi/Xpress only when entitled;
   cuOpt only when supported. Include host-device transfer/setup and pricing LP time.
   Record time-to-first-incumbent, final incumbent/bound/gap, nodes where exposed,
   p50/p95/max time, process-tree peak RSS and peak device memory. Infeasible/no-incumbent
   runs cannot win by finishing quickly.
4. **Reference agreement:** offline PyPSA consumes identical manifests/equations and
   UC states. Run hand-derived tests as an additional independent check; matching
   the same underlying solver alone does not validate model assembly. Compare
   objective certificates/aggregates when commitments are non-unique. Use NFR-2.
5. **Intertemporal correctness:** compare 48-hour look-ahead/24-hour advance to longer
   windows on a fixed week, including startup/min-up/down, storage and hydro. Quantify
   objective/price/shortage differences and boundary artefacts before approving
   terminal policies; an annual rolling result is not an annual optimum certificate.
6. **Annual acceptance:** run the frozen 30-minute weather year with UC, redispatch,
   pricing, export and manifests; score FR-12 accuracy separately. Proposed NFR-5
   targets are build/load p95 ≤30 seconds/window, solve stages p95 ≤300 seconds/window,
   annual wall time ≤48 hours, relative gap ≤0.1% plus the approved absolute policy,
   and RSS ≤min(96 GiB, 80% of measured RAM). A reduced case cannot qualify full D-32.

For timed assembly/load comparisons, use one untimed warm-up and at least five paired
baseline/candidate repetitions; retain raw runs and uncertainty estimates. Use fresh
isolated processes for peak-memory measurement and explicitly state warm/cold data
cache conditions. A single full-year run is acceptance evidence, not enough samples
to claim a statistical no-regression gate; per-PR regression uses the repeated,
representative suite in ADR 0001/NFR-4.

**Proposed promotion rule (OI-5):** retain a project-specific sparse assembly path only
if it supplies a required missing capability or demonstrates at least 20% lower median
build/load time or 25% lower peak process-tree memory against the fair reused path,
without numerical, solve-time or maintainability regressions. These are candidate
engineering thresholds, not observed gains or a licence to regress D-28 benchmarks.
If reuse meets full-model targets and custom assembly cannot justify itself, select
reuse. If neither path meets requirements, stop and present measured options to the
owner; do not switch production modelling to Python or silently ship a reduced network.

## Alternatives considered

- **JuMP/linopy/Pyomo in production:** mature modelling ecosystems, but conflict with
  D-20. Their role is independent offline reference, not hidden Rust subprocesses.
- **Only good_lp everywhere, without measurement:** maximises reuse, but assumes its
  feature surface and conversion memory suit all four backends at full UC scale.
  It remains a genuine production candidate, not rejected on language stereotypes.
- **Only handwritten native calls:** can minimise overhead but spreads unsafe/vendor
  semantics into the engine. Prefer maintained Rust wrappers; isolate required shims.
- **Build a universal Rust algebraic modeller or MILP solver:** excessive scope and
  validation burden; not needed to generate a sparse domain-specific UC matrix.
- **Use Clarabel/Ipopt as the UC solver:** does not supply the required integer solve.
- **One annual monolithic UC model:** preserves an annual objective but has unproven
  size/search memory and turnaround on D-48; retain only as a small-case reference
  where tractable, not the default annual assumption.
- **GPU-first or mandatory commercial solver:** fails the independently viable
  open-source CPU path; optional acceleration must earn its place by full-cost timing.

## Consequences

- Production code remains Rust while leveraging mature native optimisers; this is not
  a claim of a Rust-only dependency closure or a GPU-independent performance guarantee.
- A sparse model contract makes solver substitution/test evidence explicit but incurs
  design and conversion costs; those costs are included rather than hidden in benchmarks.
- Bounds, gaps, units and statuses become scientific output contracts instead of
  solver-specific strings. This is essential for honest comparisons and time limits.
- Rolling horizons constrain memory but introduce terminal/look-ahead approximation;
  longer-horizon sensitivity and observed accuracy remain separate acceptance tests.
- Commercial adapters add deployment/licensing work, and the GPU path adds ABI,
  driver and memory constraints. An interface is not evidence that an adapter works.
- Hardware/model feasibility remains unresolved until the post-baseline benchmark;
  the data spike can settle input feasibility but cannot qualify the optimisation path.

## Open items for the owner

- **ASSUMPTION (OI-1/OI-2):** linear DC/N-0 physical scope and cost-based national
  scheduling/redispatch/pricing proxies are acceptable for P1. Approve them explicitly
  before treating this model architecture as an implementation of the full market.
- **ASSUMPTION (OI-3):** reduced networks remain development fixtures pending a new
  owner decision; full-network qualification depends on the spike's licensed manifest.
- **ASSUMPTION (OI-5):** approve/revise the rolling-window/gap/RAM/runtime proposals,
  absolute-gap/terminal policy and benchmark/promotion protocol, including the proposed
  20% build-time or 25% memory justification for custom assembly. No optimisation spike
  occurs until the existing baseline/code gate opens.
- **ASSUMPTION (OI-6):** use numerical reproducibility and stated scaled tolerances,
  not bitwise equality of parallel MILP schedules.
- **ASSUMPTION (OI-7/OI-8):** optional solver/device use depends on verified installed
  hardware and approved native/redistribution/commercial terms; no upgrade or speedup
  is assumed. Unknown licence dispositions block enabled integrations.

See the full register in `../../requirements/04-requirements.md`. Acceptance of this
ADR would approve architecture and a qualification plan, not certify benchmarks,
select network data/storage, change D-32 or enable application code by itself.

## Primary references (checked 2026-11-12)

- [S1 — good_lp, solver features and crate metadata](https://docs.rs/good_lp/latest/good_lp/).
- [S2 — highs Rust API](https://docs.rs/highs/latest/highs/).
- [S3 — HiGHS project, native solvers/interfaces and licensing](https://github.com/ERGO-Code/HiGHS).
- [S4 — lp-modeler API](https://docs.rs/lp-modeler/latest/lp_modeler/).
- [S5 — Clarabel continuous conic solver](https://clarabel.org/stable/).
- [S6 — Ipopt nonlinear optimiser](https://coin-or.github.io/Ipopt/).
- [S7 — Gurobi C model API](https://docs.gurobi.com/projects/optimizer/en/current/reference/c/model.html).
- [S8 — FICO Xpress C Optimizer model-loading example](https://examples.xpress.fico.com/example.pl?id=loadlp).
- [S9 — cuOpt MIP C API](https://docs.nvidia.com/cuopt/user-guide/latest/cuopt-c/mip/mip-c-api.html), [convex C API](https://docs.nvidia.com/cuopt/user-guide/latest/cuopt-c/convex/convex-c-api.html) and [system requirements](https://docs.nvidia.com/cuopt/user-guide/latest/system-requirements.html).
- [S10 — Gurobi MIP/fixed-model dual limitations](https://support.gurobi.com/hc/en-us/articles/360034305272-How-do-I-retrieve-the-dual-Pi-values-for-a-MIP-problem).
- [S11 — PyPSA UC and rolling horizons](https://docs.pypsa.org/latest/user-guide/optimization/unit-commitment/).
- [S12 — cuOpt overview and maturity labels](https://docs.nvidia.com/cuopt/user-guide/latest/introduction.html) and [26.10 release notes](https://docs.nvidia.com/cuopt/user-guide/latest/release-notes.html).
