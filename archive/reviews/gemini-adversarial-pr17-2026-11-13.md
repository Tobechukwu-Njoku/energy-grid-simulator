# Adversarial Review — PR #17 System Topology and Architecture Decisions (ADRs 0002, 0004, 0005, 0006)

**Reviewer:** Gemini Flash 3.8 (Adversarial Review Sub-Agent)  
**Date:** 2026-11-13  
**Target:** PR #17 at head commit `254e0e1f` (`origin/t3/design-adrs-0002-0006`)  
**Target Documents:**
- `docs/architecture/topology.md` (lines 1–147)
- `docs/architecture/adr/0002-network-and-scenario-data-model.md` (lines 1–196)
- `docs/architecture/adr/0004-storage-topology.md` (lines 1–184)
- `docs/architecture/adr/0005-execution-topology.md` (lines 1–205)
- `docs/architecture/adr/0006-market-design-plugin-model.md` (lines 1–183)
- `docs/architecture/owner-questionnaire.md` (lines 1–82)

**Baseline Authority:**
- `AGENTS.md`
- `docs/requirements/03-decisions-log.md` (D-1 through D-65)
- PR #4 at head `1698c4be` (`docs/architecture/adr/0001-repo-layout-and-ci-gates.md`, `docs/architecture/adr/0003-optimisation-modelling-layer-and-solver-trait.md`, `docs/requirements/04-requirements.md`)

---

## Executive Summary

PR #17 proposes the core system topology and four Architecture Decision Records (ADRs 0002, 0004, 0005, and 0006) governing entity identities, hybrid storage, isolated process execution, and versioned market modules for the energy grid simulator. The proposal establishes commendable principles regarding immutability, content addressing, and explicit proxy disclosure.

However, an aggressive adversarial audit reveals critical architectural vulnerabilities, fatal licensing traps, unmitigated single points of failure, data-loss and state-corruption risks during worker and database crashes, reproducibility breakdowns, and operational deadlocks:

1. **Licence Trap & SaaS Contradiction (D-17 vs AGPLv3 Redis 8):** ADR 0004 and Q-O-3 recommend deploying Redis 8 under AGPLv3 for core queueing and live-state delivery. This creates an immediate licence hazard for future commercial SaaS (D-17) and appliance/on-premise deployment, while refusing to adopt the BSD-3-Clause fork (Valkey) despite acknowledging its superior permissive licensing.
2. **Worker Recovery Induces Non-Reproducible Divergence:** ADR 0005 discards native solver state upon crash and rolls back to the last 24-hour advance checkpoint. Because branch-and-bound MILP solvers explore alternative optimal paths non-deterministically without branch history, a simulation interrupted by a transient worker restart produces different unit commitment schedules, nodal prices, and unserved energy than an uninterrupted run of the identical scenario hash.
3. **Cascading OOM Collapse from Zombie Workers:** ADR 0005's lease recovery protocol reclaims expired leases and launches a new worker with token $T+1$ before the prior worker's native process tree is confirmed reaped. On a 128 GB host allocating 96 GiB (80% RAM) to a simulation, two concurrent native solver instances will immediately exhaust host physical memory and trigger the Linux OOM killer against PostgreSQL or the API orchestrator.
4. **Data Loss & Storage Leakage across PostgreSQL and Object Storage:** The two-phase upload protocol lacks atomic cross-store transactions. Blobs uploaded before a database failure cannot be swept by mark-and-sweep garbage collection because GC traverses PostgreSQL manifests, permanently leaking orphan storage. Furthermore, a race condition exists between 90-day expiration unlinking and concurrent user pinning.
5. **Operational Deadlock on D-60 132 kV Network:** ADR 0006 and ADR 0005 bind the simulator to ADR 0003's 3-iteration bounded repair loop. Coupling Stage 1 copper-plate UC with Stage 2 full D-60 132 kV network redispatch guarantees chronic repair exhaustion on congested distribution boundaries, resulting in aborted runs and an inability to simulate full weather years.
6. **Scaling Cliff in Data-Plane IPC:** The worker-to-control IPC relies on Linux file descriptor passing (`SCM_RIGHTS`), which is strictly local to a single kernel. Scaling to multi-node workers in P2/P3 requires completely replacing the data-plane protocol.
7. **Single Point of Failure (SPOF):** The modular monolith combines the API, scenario catalogue, job controller, outbox relay, and analytics into a single OS process without clustering or high-availability failover. A query memory spike in bounded analytics crashes the orchestrator and aborts running simulations.

---

## Findings Matrix

| ID | Severity | File & Line Citation | Category | Short Title |
|---|---|---|---|---|
| **ADV17-01** | CRITICAL | `docs/architecture/adr/0004-storage-topology.md:141-152`, `docs/architecture/owner-questionnaire.md:37-43` | Licence Trap / Decision Violation (D-17) | Recommending AGPLv3 Redis 8 contaminates commercial SaaS while rejecting permissive Valkey |
| **ADV17-02** | HIGH | `docs/architecture/adr/0005-execution-topology.md:108-110, 124-126`, `docs/architecture/adr/0002-network-and-scenario-data-model.md:150-152` | Reproducibility Breakdown / Non-Determinism | Mid-run worker crash recovery causes path-dependent MILP divergence across identical scenario hashes |
| **ADV17-03** | HIGH | `docs/architecture/adr/0005-execution-topology.md:82-85, 140-143` | Cascading Failure / OOM Vulnerability | Lease reclaim without confirmed worker process death triggers dual-worker OOM crash |
| **ADV17-04** | HIGH | `docs/architecture/adr/0004-storage-topology.md:69-75, 96-98` | Storage Leak / Protocol Inconsistency | Orphan Parquet blobs from failed DB transactions can never be collected by manifest mark-and-sweep |
| **ADV17-05** | HIGH | `docs/architecture/adr/0006-market-design-plugin-model.md:78-81`, `docs/architecture/adr/0005-execution-topology.md:103`, `docs/architecture/adr/0004-storage-topology.md:89` | Operational Deadlock / Feasibility Breakdown | 3-iteration UC repair loop guarantees simulation abortion on congested D-60 132 kV networks |
| **ADV17-06** | HIGH | `docs/architecture/adr/0004-storage-topology.md:99-102` | Race Condition / Data Loss | Unsynchronized pin-vs-expiration race causes partial deletion of pinned Parquet result files |
| **ADV17-07** | MEDIUM | `docs/architecture/adr/0005-execution-topology.md:51-56` | Scaling Cliff | File descriptor IPC creates an architectural dead end between single-node P1 and distributed P2/P3 |
| **ADV17-08** | MEDIUM | `docs/architecture/topology.md:23-28, 85-87`, `docs/architecture/adr/0005-execution-topology.md:31-43, 185` | Single Point of Failure (SPOF) | Unsegmented control-plane modular monolith collapses orchestrator during heavy analytics scans |
| **ADV17-09** | MEDIUM | `docs/architecture/adr/0002-network-and-scenario-data-model.md:81-86` | Untestable Criteria / Empirical Infeasibility | As-of knowledge cut requirement is unworkable with real historical UK/Irish energy data |
| **ADV17-10** | MEDIUM | `docs/architecture/adr/0002-network-and-scenario-data-model.md:136-145` | Non-Reproducibility / Hashing Instability | Floating-point CBOR normalization and unit conversion induce cross-platform scenario hash drift |
| **ADV17-11** | MEDIUM | `docs/architecture/adr/0004-storage-topology.md:28, 40-42, 126-131`, `docs/architecture/adr/0002-network-and-scenario-data-model.md:43-44` | Architectural Bloat / Licence Contamination | PostGIS introduces GPL-2.0 and native C extension overhead without topological necessity |
| **ADV17-12** | MEDIUM | `docs/architecture/adr/0005-execution-topology.md:128-132` | Untestable Claim / Process Hang | Two-second cooperative cancellation is unenforceable against blocking native solver routines |
| **ADV17-13** | LOW | `docs/architecture/adr/0002-network-and-scenario-data-model.md:147-152` | Vague Specification / Architectural Gap | Parquet schema migrations cannot reconcile immutable content-addressed storage |
| **ADV17-14** | LOW | `docs/architecture/topology.md:96`, `docs/architecture/adr/0005-execution-topology.md:140, 149` | Throughput Bottleneck | Single active simulation lockup prevents multi-year weather sweeps mandated by D-8 and D-14 |

---

## Detailed Findings

### ADV17-01: Recommending AGPLv3 Redis 8 Contaminates Commercial SaaS While Rejecting Permissive Valkey
- **Severity:** CRITICAL
- **Location:** `docs/architecture/adr/0004-storage-topology.md:141-152`, `docs/architecture/owner-questionnaire.md:37-43`
- **Contradiction:** D-17 ("Distribution: future commercial SaaS — design for auth, multi-tenancy, and licence-clean dependencies from day one"), D-28 (`cargo-deny` licence check), D-30/D-56.
- **Analysis:**
  1. In March 2024, Redis Ltd transitioned Redis from BSD-3-Clause to dual/triple commercial licenses (AGPLv3 / RSALv2 / SSPLv1).
  2. ADR 0004 lines 141–148 and questionnaire Q-O-3 (`owner-questionnaire.md:37-43`) recommend adopting Redis 8 under **AGPLv3**, rationalizing that:
     > *"unmodified private Redis under AGPLv3, independent protocol client, with source/notice/distribution and any combined-work assessment recorded. AGPL §13 concerns remote interaction with a modified covered program; it does not automatically relicense every independent API client."*
  3. This rationale is legally hazardous and directly violates D-17:
     - D-17 requires "licence-clean dependencies from day one" specifically for a **commercial SaaS** product.
     - While AGPL §13 triggers source code disclosure upon network interaction with a *modified* work, enterprise SaaS buyers, venture funders, and commercial partners routinely blacklist AGPLv3 dependencies in their software supply chains regardless of modification status.
     - Moreover, D-33 targets "consultancy/research, forecast customers, grants". In consultancy deployments, the platform may be distributed as on-premise container images or appliance bundles. Conveying an AGPLv3 runtime alongside proprietary binaries creates significant copyleft exposure and distribution compliance burdens.
  4. The open-source community created **Valkey** (under the Linux Foundation, licensed under permissive BSD-3-Clause) precisely to avoid this AGPLv3/SSPL trap.
  5. ADR 0004 lines 149–152 concedes that Valkey 9.1.3 is the "simpler permissive alternative", yet Q-O-3 inexplicably designates AGPLv3 Redis 8 as the *recommended default* while requiring an explicit owner amendment to D-3 to adopt Valkey.
  6. Recommending AGPLv3 when a drop-in BSD-3-Clause fork exists injects avoidable legal risk into the core architecture.

---

### ADV17-02: Mid-Run Worker Crash Recovery Causes Path-Dependent MILP Divergence Across Identical Scenario Hashes
- **Severity:** HIGH
- **Location:** `docs/architecture/adr/0005-execution-topology.md:108-110, 124-126`, `docs/architecture/adr/0002-network-and-scenario-data-model.md:150-152`
- **Analysis:**
  1. ADR 0005 line 108–110 establishes that checkpoints commit the 24-hour advance boundary.
  2. ADR 0005 lines 124–126 explicitly states:
     > *"A solver LP basis/MIP start is an optional version-specific acceleration, never the only durable state or a portable mid-solve checkpoint. A crash during native solve restarts that window from committed physical state."*
  3. In a rolling-horizon unit commitment (ADR 0003), each 48-hour solve is a mixed-integer linear program (MILP). MILP solvers (HiGHS, Gurobi, SCIP) rely on internal branch-and-bound trees, dynamic cut generation, node selection heuristics, and pseudocosts. When multiple integer solutions have identical or near-identical objective values (degenerate alternative optima, ubiquitous in thermal generation fleets), the solver's branch selection path determines which specific generators are committed.
  4. In an uninterrupted simulation run, the worker carries warm heuristic information, solver state, and memory structures across rolling windows.
  5. If a worker crashes at hour 23 of a 24-hour window and resumes from the physical state checkpoint at hour 0, the native solver re-initializes cold without the identical branch state or basis. Due to alternative optima and solver tie-breaking, the recovered solve selects a **different valid commitment schedule** than the uninterrupted run.
  6. Because unit commitment decisions are hard integers that fix thermal generator availability, ramp trajectories, and minimum up/down timers for subsequent rolling windows, this deviation compounds across the remaining 300+ days of the weather year.
  7. Consequently, two runs with identical scenario hashes, identical solver versions, and identical input data will yield **substantively different dispatch schedules, nodal price series, and unserved energy outturns** solely because one experienced a transient worker crash and restart. This destroys scientific reproducibility.

---

### ADV17-03: Lease Reclaim Without Confirmed Worker Process Death Triggers Dual-Worker OOM Crash
- **Severity:** HIGH
- **Location:** `docs/architecture/adr/0005-execution-topology.md:82-85, 140-143`
- **Analysis:**
  1. ADR 0005 line 78 sets a 60-second lease with 10-second heartbeats.
  2. ADR 0005 lines 82–85 defines the recovery loop:
     > *"A recovery loop reclaims expired leases with a new token and resumes from the last committed checkpoint. A late/stalled old worker may still compute but cannot publish. Stop its local process group and reserve resources conservatively until death is confirmed; lease expiry alone does not free physical RAM/VRAM/licences."*
  3. ADR 0005 lines 140–143 sets the resource ceiling:
     > *"Start with one active simulation and PR #4's proposed 96 GiB/80%-of-RAM process-tree ceiling."*
  4. Consider a worker executing a heavy Stage 1 MILP or large matrix factorization. If the host experiences transient I/O saturation or thread preemption, the worker's heartbeat loop may fail to reach PostgreSQL within 60 seconds.
  5. PostgreSQL marks the lease expired. The recovery loop immediately reclaims the lease, increments the fencing token, and launches Worker 2.
  6. Worker 1 is NOT dead; it is actively computing in native C++ code, currently consuming up to 96 GiB of RAM.
  7. Worker 2 initializes, loads the 24-hour advance network snapshot, builds sparse model matrices, and allocates its own memory pool (up to 96 GiB).
  8. Total resident memory demand immediately hits $96\,\text{GiB} + 96\,\text{GiB} = 192\,\text{GiB}$, vastly exceeding the physical 128 GB RAM floor of the D-48 target machine.
  9. The Linux kernel invokes the Out-Of-Memory (OOM) killer. Under standard `oom_score_adj`, the kernel will terminate the largest memory-consuming processes—or worse, kill the PostgreSQL server or the API orchestrator daemon, corrupting shared memory and aborting all simulations.
  10. Fencing tokens prevent duplicate *writes* to the database, but they provide zero protection against concurrent physical memory starvation. A lease reclaim must strictly block until the supervisor confirms `SIGKILL` termination and process reap of the prior worker.

---

### ADV17-04: Orphan Parquet Blobs from Failed DB Transactions Can Never Be Collected by Manifest Mark-and-Sweep
- **Severity:** HIGH
- **Location:** `docs/architecture/adr/0004-storage-topology.md:60-75, 96-98`
- **Analysis:**
  1. ADR 0004 lines 56–65 defines the publication sequence across stores:
     - Worker writes Parquet result files to staging / object store (`lines 56–63`).
     - Worker records an immutable manifest naming every key, digest, and size (`lines 63–64`).
     - Worker initiates a PostgreSQL transaction to advance the checkpoint pointer and commit the result manifest (`lines 64–65`).
  2. ADR 0004 lines 69–71 notes:
     > *"An object-store success followed by database failure leaves an invisible orphan, not a completed result."*
  3. Now examine the garbage collection mechanism defined in ADR 0004 lines 96–98:
     > *"Deletion is mark-and-sweep from durable manifest references, with scope checks and object reference counts/reader grace, not prefix deletion of everything under a dataset hash."*
  4. **The architectural flaw:** If the database transaction fails (e.g., deadlock, serialization failure, network timeout, crash), the Parquet blobs exist in object storage, but NO durable manifest reference exists in PostgreSQL!
  5. Because mark-and-sweep starts strictly *from durable PostgreSQL manifests* (`line 96`), and object store directory/bucket listing is explicitly rejected (`line 62`: *"do not rely on atomic directory rename or LIST order"*), the GC worker has no index or pointer to locate the uncommitted orphan blobs.
  6. ADR 0004 line 73 mentions a *"proposed 24-hour orphan grace"*, but provides no operational mechanism to discover unreferenced keys without an object store `LIST` scan.
  7. Over months of batch sweeps, network retries, and aborted runs, gigabytes of stranded Parquet result files will leak silently in the object store, escaping automated reclamation.

---

### ADV17-05: 3-Iteration UC Repair Loop Guarantees Simulation Abortion on Congested D-60 132 kV Networks
- **Severity:** HIGH
- **Location:** `docs/architecture/adr/0006-market-design-plugin-model.md:78-81`, `docs/architecture/adr/0005-execution-topology.md:103`, `PR #4 ADR 0003:42-48`, `03-decisions-log.md:D-60`
- **Analysis:**
  1. D-60 mandates that the MVP network contains **all 132 kV circuits in GB**, including DNO-owned assets across England and Wales. This expands the network graph to thousands of buses and low-voltage transmission branches.
  2. ADR 0006 lines 62–81 adopts ADR 0003's decomposition:
     - Stage 1 solves a national copper-plate Unit Commitment (MILP) with zero network constraint awareness.
     - Stage 2 solves full-network DC redispatch with Stage 1 commitment fixed.
     - If Stage 2 redispatch is infeasible due to transmission congestion, it feeds violated constraints back to Stage 1 and re-solves.
     - ADR 0003 line 45 establishes a hard bound: *"limit to three repair iterations per window, then mark failed/partial with diagnostics; never drop the proxy or present the unsolved schedule as valid."*
     - ADR 0006 line 80 reiterates: *"The market module cannot skip the final full-row network check or turn an unsolved window into shortage."*
     - ADR 0005 line 103 mandates: *"Invalid data, infeasible models and exhausted UC repair are reported, not retried blindly."*
  3. In real electrical distribution grids (132 kV), thermal constraints are highly localized. Copper-plate Stage 1 unit commitment routinely commits large, efficient CCGTs or Scottish wind while de-committing expensive peakers or local distributed generation in southern load pockets.
  4. In Stage 2, severe 132 kV line overloads occur. Redispatch LP cannot relieve these overloads because the local units capable of relieving them are turned OFF, and unit commitments cannot be changed in a continuous LP.
  5. The model feeds feasibility cuts back to Stage 1. However, with thousands of 132 kV lines and complex network interactions, integer branch-and-bound cuts rarely achieve global feasibility in 3 iterations.
  6. Upon reaching iteration 3, the repair loop terminates. Because lines cannot be dropped and artificial shortage is forbidden, the window is marked `failed`.
  7. Under ADR 0005 lines 99–103, exhausted UC repair aborts the simulation as `failed`. A 365-day weather-year simulation on a full D-60 network will almost certainly abort within the first few winter weeks.

---

### ADV17-06: Unsynchronized Pin-vs-Expiration Race Causes Partial Deletion of Pinned Parquet Result Files
- **Severity:** HIGH
- **Location:** `docs/architecture/adr/0004-storage-topology.md:99-102`
- **Analysis:**
  1. D-36 and ADR 0004 line 89 dictate that unpinned full results expire 90 days after completion, while pinned runs are retained indefinitely.
  2. ADR 0004 lines 99–101 describes the concurrency control:
     > *"Pin and expiration race in one PostgreSQL transaction: a pin acquired before a deletion tombstone protects the data; a later request reports detail already unavailable."*
  3. While the database state transition is transactional, the physical result data resides in an external object store or filesystem (`adr/0004:30`).
  4. Consider the execution sequence:
     - Background GC job queries PostgreSQL for expired runs (>90 days old, unpinned). It finds Run `R1` and issues a tombstone transaction.
     - The GC worker begins issuing asynchronous `DeleteObject` or `unlink` calls against the 50 Parquet partition files comprising Run `R1`.
     - After 25 files are deleted, an analyst in the Web UI pins Run `R1`.
     - The user's request races in PostgreSQL. If the GC tombstone transaction committed just before the pin request, the UI receives an error. BUT if the GC worker executed in two phases (read unpinned -> delete files -> tombstone row), or if the pin transaction commits concurrently while the storage worker is midway through unlinking files, a catastrophic split-brain state occurs.
  5. PostgreSQL commits the pin status as valid, but half of the underlying Parquet partition files have been permanently deleted from disk.
  6. Any subsequent analytical query against the pinned run crashes with `FileNotFoundException` or corrupt Parquet footer errors.

---

### ADV17-07: File Descriptor IPC Creates an Architectural Dead End Between Single-Node P1 and Distributed P2/P3
- **Severity:** MEDIUM
- **Location:** `docs/architecture/adr/0005-execution-topology.md:51-56`, `docs/architecture/topology.md:97-98`
- **Analysis:**
  1. ADR 0005 lines 51–53 specifies the worker host IPC contract for P1:
     > *"Use bounded local IPC with control messages and checksummed Arrow/file descriptors for large inputs/results; do not serialise a matrix into Redis or execute one RPC per coefficient."*
  2. Passing file descriptors (`SCM_RIGHTS` over Unix domain sockets) is strictly an intra-kernel mechanism. It cannot cross machine boundaries or container namespaces without shared host volumes.
  3. In P2 and P3 (`adr/0005:150-151`, `topology.md:97-98`), the architecture claims:
     > *"P2: add worker hosts/nodes when measured work requires it... P3: Multiple worker pools/API replicas and shared private object storage"*.
  4. However, moving workers to remote nodes invalidates the entire P1 data-plane IPC layer:
     - Remote nodes cannot receive file descriptors.
     - Matrix inputs, Arrow arrays, and result streams cannot be shared via local handles.
  5. While ADR 0005 line 54 notes that remote hosts will later exchange coarse envelopes over authenticated transport, building the initial engine around local file descriptor passing means that transitioning from P1 to P2 requires rewriting the worker-host runtime transport, contrary to the claim that domain and task envelopes survive without rewrite (`adr/0005:173-174`).

---

### ADV17-08: Unsegmented Control-Plane Modular Monolith Collapses Orchestrator During Heavy Analytics Scans
- **Severity:** MEDIUM
- **Location:** `docs/architecture/topology.md:23-28, 85-87`, `docs/architecture/adr/0005-execution-topology.md:31-43, 185`
- **Analysis:**
  1. Topology diagram (`topology.md:23-28`) and ADR 0005 line 31 define a single "modular monolith" Rust process housing:
     - Web UI API and Scenario Catalogue
     - Job controller and outbox relay
     - Result publication and bounded analytics
  2. ADR 0004 line 31 places "selected time-series and map-ready slices" in PostgreSQL query projections, while line 33 suggests Rust DataFusion/Polars for bounded Parquet queries inside this control service.
  3. In D-47, analysts and traders are promised "pro analytics, dense, dark-mode, Bloomberg/Grafana-style dashboards".
  4. When an analyst loads a dashboard requesting multi-bus time-series rollups or spatial map projections across an entire simulated weather year, the DataFusion engine inside the control service executes columnar scans and aggregations.
  5. Because the API, outbox relay, and job heartbeat controllers share the same process memory and Tokio async runtime:
     - Heavy analytical queries will saturate CPU worker threads and induce memory allocation spikes.
     - Thread starvation delays outbox event dispatch and job lease heartbeat renewal (`adr/0005:78`).
     - Stalled heartbeats cause PostgreSQL to expire active worker leases, triggering false recovery cycles and killing active simulation runs.
  6. Combining high-throughput analytical query processing with real-time job orchestration inside a single OS process creates an acute availability hazard.

---

### ADV17-09: As-Of Knowledge Cut Requirement is Unworkable with Real Historical UK/Irish Energy Data
- **Severity:** MEDIUM
- **Location:** `docs/architecture/adr/0002-network-and-scenario-data-model.md:81-86`
- **Analysis:**
  1. ADR 0002 lines 81–86 specifies temporal selection rules:
     > *"An as-of forecast backtest selects only revisions provably published by its knowledge cut and records when that cut advances across forecast origins. A present-day download of a revised historic table is not proof of what was available then. Unknown publication history blocks an as-of claim; acquisition time never fabricates it."*
  2. In the real UK and Irish energy sectors:
     - Elexon BMRS/Insights publishes settlement data that is continuously revised at RF (Reconciliation Final, 14 months) and DF (Dispute Final, 28 months). Historical bulk downloads do not preserve point-in-time publication timestamps for intermediate settlement runs (SF, R1, R2).
     - National Grid ESO (NESO) publishes ETYS and network models as annual static PDF/Excel snapshots without machine-readable publication audit trails.
     - SEMO and SEMOpx historical data portals overwrite or amend pricing tables without publishing cryptographic timestamp receipts.
  3. Under ADR 0002 line 84: *"Unknown publication history blocks an as-of claim."*
  4. Therefore, strictly adhering to ADR 0002 makes it **impossible to ever execute an `as_of` backtest** on historical UK/Irish grid data. Every backtest must be demoted to an explicitly labelled `reconstruction`, rendering the proposed "as-of knowledge cut" mechanism an unusable theoretical construct.

---

### ADV17-10: Floating-Point CBOR Normalization and Unit Conversion Induce Cross-Platform Scenario Hash Drift
- **Severity:** MEDIUM
- **Location:** `docs/architecture/adr/0002-network-and-scenario-data-model.md:136-145`
- **Analysis:**
  1. ADR 0002 lines 136–140 defines canonical scenario identity:
     > *"Hash its validated semantic form after explicit default expansion, unit conversion and reference resolution, using a schema-specific deterministic CBOR encoding [S1] and SHA-256... decimal coefficients are normalised without lossy floating-point parsing."*
  2. In TOML scenario files (D-19), electrical parameters (line resistances, generator heat rates, fuel prices, availability factors) are parsed into IEEE 754 double-precision floats (`f64`).
  3. ADR 0002 requires hashing *after* unit conversion (e.g. MW to kW, or £/MWh to €/MWh via daily FX rates per ADR 0006:109–111).
  4. Floating-point arithmetic is non-associative and subject to subtle compiler vectorization (FMA/AVX instructions) and platform differences between x86-64 (AMD 5900X) and aarch64 (developer laptops).
  5. If `0.1 * 3.0` yields `0.30000000000000004` on one platform and `0.3` after decimal normalization on another, or if RFC 8949 deterministic CBOR encodes a float using double-precision while an alternative parser reduces it to single-precision, the resulting CBOR byte string diverges.
  6. A single bit difference in the CBOR byte stream completely alters the SHA-256 scenario hash, causing two logically identical scenarios to produce different cryptographic identities and breaking result cache lookups.

---

### ADV17-11: PostGIS Introduces GPL-2.0 and Native C Extension Overhead Without Topological Necessity
- **Severity:** MEDIUM
- **Location:** `docs/architecture/adr/0004-storage-topology.md:28, 40-42, 126-131`, `docs/architecture/adr/0002-network-and-scenario-data-model.md:43-44`, `docs/architecture/owner-questionnaire.md:17-27`
- **Analysis:**
  1. ADR 0004 line 28 proposes the **PostGIS** extension for asset/zone geometry, spatial indexing, and viewport queries.
  2. ADR 0004 lines 126–131 acknowledges that PostGIS is licensed under **GPL-2.0-or-later**, which creates appliance/container distribution obligations.
  3. Yet, ADR 0002 lines 43–44 explicitly states:
     > *"Geography and display coordinates do not determine electrical connectivity."*
  4. Power grid simulation operates entirely on a topological bus-branch graph (admittance matrices, PTDFs, branch connections). Geospatial coordinates are purely decorative metadata used by the frontend map UI (Deck.gl/Mapbox) to render points and line vectors.
  5. Storing basic `(latitude, longitude)` floats directly in PostgreSQL tables and querying them using standard B-tree range queries or bounding boxes completely satisfies all UI viewport requirements without any spatial extensions.
  6. Introducing PostGIS brings:
     - Substantial GPL-2.0 compliance scrutiny (contrary to D-17 and D-30).
     - Heavy native C/C++ dependencies (`geos`, `proj`, `gdal`) into the PostgreSQL container image.
     - Known upgrade frictions across major PostgreSQL versions (e.g. PG18).
  7. Adopting PostGIS violates the core principle of D-1 ("reusing OSS where it helps") by incurring heavy licence and operational penalties for zero mathematical benefit to grid physics.

---

### ADV17-12: Two-Second Cooperative Cancellation is Unenforceable Against Blocking Native Solver Routines
- **Severity:** MEDIUM
- **Location:** `docs/architecture/adr/0005-execution-topology.md:128-132`
- **Analysis:**
  1. ADR 0005 lines 128–130 proposes:
     > *"Preserve ADR 0003's cooperative cancellation contract: proposed two-second solver grace and five-second total native process-group kill/reap bound."*
  2. Native solvers like HiGHS, Ipopt, and cuOpt run compiled C++ / Fortran loops. During computationally intensive phases—such as solving large LU factorizations, dense simplex pivots, or interior-point linear systems—native code executes inside BLAS/LAPACK routines that do not poll external flags.
  3. Unless an explicit, re-entrant interrupt callback (e.g., `Highs_setInterruptCallback`) is actively supported by the solver C-API and safely bridged to Rust across FFI, the worker cannot cooperate.
  4. Consequently, virtually all cancellations during difficult solves will exceed the 2-second grace and fall back to a hard `SIGKILL` at 5 seconds.
  5. While ADR 0005 line 131 provides for a supervisor process-group kill, claiming a "cooperative cancellation" contract without verifying C-API interrupt callback support across HiGHS and Ipopt represents an unvalidated design assumption.

---

### ADV17-13: Parquet Schema Migrations Cannot Reconcile Immutable Content-Addressed Storage
- **Severity:** LOW
- **Location:** `docs/architecture/adr/0002-network-and-scenario-data-model.md:147-152`, `docs/architecture/adr/0004-storage-topology.md:46-52`
- **Analysis:**
  1. ADR 0002 lines 147–149 asserts:
     > *"Schema migrations are explicit, deterministic transforms producing a new revision and before/after hashes. Existing manifests remain readable through their versioned reader or retained compatible build."*
  2. Full-resolution simulation results are stored as immutable Parquet files in object storage, referenced by exact content digests (`adr/0004:27, 46`).
  3. If a schema migration occurs (e.g., adding a new electrical attribute, renaming an output column, or altering physical units):
     - If existing Parquet files are rewritten, their SHA-256 byte hashes change, invalidating every historical scenario manifest, dataset release, and cryptographic audit log that references them.
     - If existing Parquet files are NOT rewritten, the reader must support multi-version deserialization. However, Parquet files are read directly via DataFusion/Polars (`adr/0004:33-34`), which enforce strict static schema expectations across scanned file partitions.
  4. The PR provides no architectural bridge explaining how Arrow/Parquet analytical scans will query across pre- and post-migration dataset boundaries without either rewriting immutable files or incurring schema-merge failures.

---

### ADV17-14: Single Active Simulation Lockup Prevents Multi-Year Weather Sweeps Mandated by D-8 and D-14
- **Severity:** LOW
- **Location:** `docs/architecture/topology.md:96`, `docs/architecture/adr/0005-execution-topology.md:140, 149`
- **Analysis:**
  1. D-8 and D-14 mandate support for "batch scenario sweeps" and "weather extremes (multi-year weather)".
  2. ADR 0005 line 140 and `topology.md:96` mandate:
     > *"Start with one active simulation and PR #4's proposed 96 GiB/80%-of-RAM process-tree ceiling... Private single runtime node, Compose/system services, one service and isolated worker processes; CLI batch with two independent runs may execute sequentially"*.
  3. A full weather year comprises 17,520 half-hourly dispatch intervals across GB and Ireland. Even with an aggressive solve time of 1.5 seconds per half-hour across the 4-stage pipeline, one weather year requires over 7.3 hours of continuous, 100% CPU execution.
  4. Restricting P1 to a single active simulation executing sequentially means that a modest 10-member scenario sweep (e.g., testing Dunkelflaute resilience across 10 historical weather years) will lock the single node for **over 3 continuous days**.
  5. The architecture lacks any mechanism for intra-run temporal parallelization (which ADR 0005:158 concedes requires unproven boundary reconciliation algorithms), creating an immediate operational bottleneck for analysts.

---

## Remediation Recommendations

To resolve these architectural defects before baselining, the coordinator and architect should execute the following revisions:

1. **Adopt Valkey 9 over Redis 8 (Fix for ADV17-01):**
   - Update `owner-questionnaire.md` (Q-O-3) to recommend Valkey 9 (BSD-3-Clause) as the primary baseline, amending D-3. Remove the recommendation of AGPLv3 Redis 8 to protect future commercial SaaS distribution (D-17).
2. **Synchronous Worker Reaping in Recovery Loop (Fix for ADV17-03):**
   - Modify `adr/0005-execution-topology.md:82-85` to mandate that the recovery loop MUST synchronously confirm `SIGKILL` and process reaping of the expired worker process tree before allocating resources to, or launching, a successor worker.
3. **Formalize Non-Deterministic MILP Tolerance Bands (Fix for ADV17-02):**
   - Explicitly document in ADR 0002 and ADR 0005 that mid-run crash recovery guarantees physical mass-and-energy conservation, but does NOT guarantee bitwise identical unit commitment paths due to alternative integer optima. Reconcile this with PR #4 NFR-2 validation tolerances.
4. **Active Object Store Orphan Sweeper (Fix for ADV17-04):**
   - Update `adr/0004-storage-topology.md:69-75` to require a periodic decoupled object-store reconciler that performs a low-frequency bucket `LIST` scan to identify and garbage-collect unreferenced staging blobs older than 24 hours.
5. **Dynamic Relaxation for D-60 132 kV Network Congestion (Fix for ADV17-05):**
   - Amend `adr/0006-market-design-plugin-model.md:78-81` and ADR 0003 to allow explicit localized slack variables with high penalty costs on lower-voltage 132 kV distribution constraints during Stage 2 redispatch, preventing complete simulation aborts while accurately surfacing localized congestion costs.
6. **Eliminate PostGIS in Favor of Native Coordinates (Fix for ADV17-11):**
   - Drop the PostGIS extension requirement from ADR 0004 and Q-O-2. Use standard PostgreSQL numeric columns `(latitude, longitude)` and client-side bounding box queries to eliminate GPL-2.0 exposure and container bloat.
