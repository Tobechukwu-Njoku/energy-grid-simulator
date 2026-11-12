# Software Dependency Licences & Hardware Platform Compatibility

_Last updated: 2026-11-12. Access date for all sources: 2026-11-12._

## Summary

This evaluation audits third-party software dependencies, mathematical solvers, frontend visualisation libraries, and compute hardware architectures against the architectural and business constraints of the GB + Ireland electricity grid and market simulator:
- **D-17:** Future commercial SaaS product (multi-tenant, closed-source cloud deployment).
- **D-20:** Rust is the primary language wherever code is needed.
- **D-42:** Lab hardware baseline: GTX 1080 Ti (Pascal), Quadro RTX 4000 (Turing), with planned consolidation to RTX 5060 Ti 16 GB (Blackwell).

### What is Permitted for a Closed-Source Commercial SaaS
1. **Rust Core Ecosystem:** Every single selected Rust crate (`highs`, `clarabel`, `faer`, `good_lp`, `nalgebra-sparse`, `sqlx`, `axum`, `tonic`, `polars`, `datafusion`, `arrow`) and the underlying HiGHS C++ solver is licensed under either **MIT** or **Apache-2.0** (or dual MIT/Apache-2.0). All allow unconditional inclusion in closed-source commercial SaaS binaries and cloud services without copyleft risk.
2. **In-Memory Caching & State:** **Valkey 9.1.3** is licensed under **BSD-3-Clause** (under Linux Foundation governance) and provides a completely clean, drop-in replacement for Redis with zero SaaS restrictions or license uncertainty.
3. **Database Architecture:**
   - **PostgreSQL 18** is licensed under the permissive **PostgreSQL Licence** (SPDX: `PostgreSQL`).
   - **PostGIS 3.7.0** is licensed under **GPL-2.0-or-later**. Because our architecture is a server-side SaaS (D-17) accessed over network APIs, running PostGIS inside the backend database server does not trigger GPL distribution obligations.
   - **TimescaleDB 2.31.0** is split between Apache-2.0 (core hypertables) and Timescale License (TSL). TSL explicitly authorizes free commercial use to power proprietary SaaS applications, provided the software is not resold as a managed Database-as-a-Service (DBaaS) to third parties.
4. **Solvers (Open & Weak Copyleft):**
   - **HiGHS v1.16.0** is fully permissive under **MIT**.
   - **Clarabel 0.11.1** is fully permissive under **Apache-2.0**.
   - **Ipopt 3.14.20** is licensed under **EPL-2.0** (weak file-level copyleft). Network SaaS execution does not trigger redistribution.
   - **SuiteSparse KLU v7.14.1** is licensed under **LGPL-2.1+**. Network SaaS execution does not trigger dynamic linking or relinking requirements.
   - **NVIDIA cuDSS v1.0.0** is licensed under the **NVIDIA Math Libraries SLA**, allowing royalty-free deployment and dynamic library runtime distribution.
5. **Frontend Geospatial Visualisation:** Both **MapLibre GL JS 6.18.0** (**BSD-3-Clause**) and **deck.gl 9.4.1** (**MIT**) are completely permissive for commercial SaaS web clients.

### What is Blocked or Requires Commercial Clearance / Caution
1. **Redis 8 Licensing Traps:** Redis Ltd moved from BSD-3-Clause to a dual RSALv2 / SSPLv1 model in March 2024, and subsequently introduced AGPLv3 in Redis 8. 
   - **AGPLv3** enforces strong network copyleft: any modifications served over a network must be open-sourced under AGPLv3.
   - **RSALv2 / SSPLv1** are source-available and non-OSI compliant, restricting managed services.
   - **Recommendation:** Do not use Redis 8. Standardize on **Valkey 9** (BSD-3-Clause) across all infrastructure.
2. **Commercial MIP Solvers (Gurobi & FICO Xpress):** Standard named-user or single-machine commercial licenses strictly prohibit SaaS hosting, multi-tenant access, and embedding. Running Gurobi in cloud containers requires an **ISV / OEM Agreement** with **Web License Service (WLS)** token pools. FICO Xpress Community Edition has an unviable 5,000-variable ceiling, and production SaaS requires an OEM distribution license.
3. **Ipopt Linear Solver Hidden Dependencies:** The core Ipopt code is EPL-2.0, but using proprietary linear solvers such as **HSL (MA27/MA57)** requires expensive commercial licenses from STFC/UKRI. Production SaaS builds of Ipopt should be compiled against open solvers: **MUMPS** (CeCILL-C) or **SPRAL** (BSD-3-Clause).
4. **CUDA 13 Architecture Drop (Pascal GTX 1080 Ti):** CUDA 13 officially **drops support for the Pascal architecture** (Compute Capability 6.0/6.1). The lab's GTX 1080 Ti 11 GB cannot be targeted by CUDA 13. The Quadro RTX 4000 (Turing, CC 7.5) is the absolute minimum supported GPU architecture in CUDA 13.
5. **RTX 5060 Ti FP64 Throughput Ratio (1:64):** Consumer Blackwell GPUs (including the planned RTX 5060 Ti 16 GB) have a severely restricted double-precision throughput ratio of **1:64** relative to single-precision (FP32). Numerically sensitive AC optimal power flow (ACOPF) algorithms requiring FP64 sparse Cholesky/LU factorizations will execute faster on the multi-core AMD Ryzen 9 5900X CPU (AVX2) than on the GPU. GPU acceleration on RTX 5060 Ti must be reserved for FP32/mixed-precision algorithms (e.g. PDHG for LP, batch Monte Carlo, spatial rendering).

---

## Dependency Master Catalogue

| Category | Dependency / Library | Official Version (as of 2026-11-12) | Official Licence & SPDX ID | Usable in Closed Commercial SaaS? | Notable Licensing Details & Operational Constraints |
|---|---|---|---|---|---|
| **Database** | **PostgreSQL** | `18.0` (minor 18.7) | PostgreSQL Licence (`PostgreSQL`) | **Yes** | Permissive open-source license. Native async I/O subsystem (AIO), native `uuidv7()`, virtual generated columns, and OAuth 2.0 authentication. |
| **Database** | **PostGIS** | `3.7.0` | GNU General Public License v2.0 or later (`GPL-2.0-or-later`) | **Yes** (Server-side SaaS only) | GPL-2.0 is triggered by binary distribution to third parties, not by network access over an API. Safe for closed-source SaaS hosted on backend cloud infrastructure. Client desktop binaries must not link PostGIS. |
| **Database** | **TimescaleDB** | `2.31.0` | Dual: Apache-2.0 (`Apache-2.0`) & Timescale License (`TSL`) | **Yes** (Application SaaS only) | Fully supports PostgreSQL 18 since v2.23. TSL explicitly permits free commercial SaaS use where TimescaleDB powers an application. Prohibits offering TimescaleDB as a managed DBaaS. Continuous aggregates, columnar compression, and data retention policies require TSL (`SHOW timescaledb.license` = `timescale`). |
| **In-Memory** | **Redis 8** | `8.0.x` / `8.2.x` | Tri-license: `RSALv2` / `SSPLv1` / `AGPLv3` | **Caution / Not Recommended** | Redis Ltd abandoned BSD-3-Clause. AGPLv3 imposes strong network copyleft if modified. RSALv2 and SSPLv1 are source-available. |
| **In-Memory** | **Valkey** | `9.1.3` (also 8.1.11, 8.0.12) | 3-Clause BSD License (`BSD-3-Clause`) | **Yes** (Recommended) | Official Linux Foundation fork of Redis. Clean BSD-3-Clause open-source governance. Complete drop-in wire compatibility with Redis 7/8 protocols. |
| **Rust Crates** | **highs** | `2.4.0` | `MIT` | **Yes** | Safe Rust bindings to HiGHS linear programming solver. Safe for proprietary SaaS. |
| **C++ Solver** | **HiGHS** (solver engine) | `v1.16.0` | `MIT` | **Yes** | High-performance C++ solver for LP, MILP, and QP developed at University of Edinburgh (ERGO). Permissive commercial re-use without royalties. |
| **Rust Crates** | **clarabel** | `0.11.1` | `Apache-2.0` | **Yes** | Pure Rust interior-point conic optimization solver (LP, QP, SOCP, exponential cones). Permissive commercial re-use. |
| **Rust Crates** | **faer** | `0.24.4` | `MIT` | **Yes** | Pure Rust high-performance dense and sparse linear algebra library (Cholesky, LU, QR, SVD, EVD). Permissive commercial re-use. |
| **Rust Crates** | **good_lp** | `1.15.3` | `MIT` | **Yes** | Mixed Integer Linear Programming modeling abstraction in Rust supporting backends: HiGHS, Clarabel, CBC, GLPK, SCIP, Cplex. |
| **Rust Crates** | **nalgebra-sparse** | `0.12.0` | `Apache-2.0` | **Yes** | CSR, CSC, and COO sparse matrix formats and arithmetic extending the `nalgebra` ecosystem. |
| **Rust Crates** | **sqlx** | `0.9.0` | `MIT OR Apache-2.0` | **Yes** | Async, pure-Rust SQL toolkit with compile-time checked queries for PostgreSQL. Dual permissive license. |
| **Rust Crates** | **axum** | `0.8.9` | `MIT` | **Yes** | Ergonomic, modular async web application framework built on Tokio, Tower, and Hyper. Permissive commercial re-use. |
| **Rust Crates** | **tonic** | `0.14.9` | `MIT` | **Yes** | Production gRPC client and server library over HTTP/2 and Tower. Permissive commercial re-use. |
| **Rust Crates** | **polars** | `0.55.2` | `MIT` | **Yes** | Blazingly fast multi-threaded DataFrame library utilizing Arrow in-memory format and SIMD vectorization. Permissive commercial re-use. |
| **Rust Crates** | **datafusion** | `55.2.0` | `Apache-2.0` | **Yes** | In-memory distributed SQL query engine and analytical dataframe framework based on Apache Arrow. Permissive commercial re-use. |
| **Rust Crates** | **arrow** (arrow-rs) | `60.1.0` | `Apache-2.0` | **Yes** | Official Rust implementation of the Apache Arrow columnar memory model and computational kernels. Permissive commercial re-use. |
| **Solvers / Libs** | **Ipopt** | `3.14.20` | Eclipse Public License 2.0 (`EPL-2.0`) | **Yes** (Server-side SaaS only) | Interior Point Optimizer for large-scale non-linear optimization. Weak file-level copyleft; network execution in SaaS does not trigger source distribution. Underlying linear solvers must be checked: MUMPS (CeCILL-C) or SPRAL (BSD) are permissible; HSL (MA27/MA57) requires commercial licensing. |
| **Solvers / Libs** | **SuiteSparse KLU** | `v7.14.1` (KLU component) | GNU Lesser General Public License v2.1+ (`LGPL-2.1+`) | **Yes** (Server-side SaaS only) | Sparse direct LU solver optimized for circuit and power network matrices. LGPL-2.1 does not trigger over network execution in SaaS. If client binaries are distributed, dynamic linking (`libklu.so`) is required. Dual commercial licensing available from authors. |
| **Solvers / Libs** | **NVIDIA cuDSS** | `v1.0.0` (GA) | NVIDIA Math Libraries SDK SLA | **Yes** (On NVIDIA hardware) | Direct sparse linear solver for NVIDIA GPUs. Proprietary royalty-free license for application development and deployment on NVIDIA hardware. Dynamic runtime libraries (`libcudss.so`) may be redistributed. Bundles AMD/COLAMD (BSD) and METIS (Apache-2.0). |
| **Solvers / Libs** | **NVIDIA cuOpt** | `26.10` (container `26.10.0`) | Dual: `Apache-2.0` (repo) / NVIDIA AI Enterprise (NVAIE) SLA | **Yes** (Open repo Apache-2.0; NVAIE fee for enterprise) | Open-source Python/C++ interfaces on GitHub are Apache 2.0. Production microservices (NIM) and enterprise container images require NVIDIA AI Enterprise subscription per GPU. |
| **Solvers / Libs** | **Gurobi** | `14.0` (`gurobipy` 14.0.0; maintenance 13.0.3) | Proprietary Commercial EULA | **Restricted** (Requires ISV / WLS agreement) | Standard commercial licenses forbid SaaS hosting or embedding. SaaS deployment requires an ISV/OEM partnership and Web License Service (WLS) token pools connecting via outbound HTTPS (`token.gurobi.com`). |
| **Solvers / Libs** | **FICO Xpress** | `10.0` | Proprietary Commercial EULA | **Restricted** (Requires OEM agreement) | Commercial MIP/NLP solver suite. Free Community Edition has a hard ceiling of 5,000 variables and constraints. Commercial SaaS embedding requires an OEM partner agreement with FICO. |
| **Visualisation** | **MapLibre GL JS** | `6.18.0` | 3-Clause BSD License (`BSD-3-Clause`) | **Yes** | Permissive open-source fork of Mapbox GL JS v1. No telemetry, no token fees, full commercial SaaS compatibility. |
| **Visualisation** | **deck.gl** | `9.4.1` | `MIT` | **Yes** | High-performance WebGL2/WebGPU spatial data visualization suite under Linux Foundation / UCF. Permissive commercial re-use. |

---

## Detailed Component Analysis

### 1. Database & In-Memory Storage

#### PostgreSQL 18 & TimescaleDB Compatibility
- **PostgreSQL 18 Release Status:** Released September 25, 2025; active support runs to November 2030. Key additions include the asynchronous I/O subsystem (AIO), native `uuidv7()` functions, and virtual generated columns.
- **TimescaleDB PG18 Support:** TimescaleDB officially introduced PostgreSQL 18 support in the **TimescaleDB 2.23** release series. The latest version (**2.31.0**) fully supports PostgreSQL 18 on Linux (Debian, Ubuntu, RHEL/EL9) and official Docker containers (`timescale/timescaledb:2.31.0-pg18`).
- **Timescale License (TSL) vs Apache 2.0 Feature Breakdown:**
  - **Apache 2.0 Core Features:** Basic hypertables, automatic table chunking, standard time-series SQL manipulation, chunk boundary indexing, basic `time_bucket()` functionality.
  - **Timescale License (TSL) Community Features:**
    1. *Native Columnar Compression:* Typically yields 90%+ disk compression and accelerates analytical time-range queries.
    2. *Continuous Aggregates:* Real-time, materialized, self-invalidating rollups for high-resolution operational data.
    3. *Data Retention Policies:* Automatic deletion and chunk dropping based on age.
    4. *Downsampling & Data Tiering:* Automated movement between storage tiers.
    5. *Hyperfunctions:* Statistical percentiles, time-weighted averages, financial candlestick aggregations, and state transition counters.
- **SaaS Permissibility:** Under the Timescale License, running TimescaleDB community features to power a commercial SaaS application (such as an energy market simulation platform) is **completely free and legally permitted**. The sole restriction is that the licensee cannot offer TimescaleDB itself as a managed Database-as-a-Service (DBaaS) to third parties.

#### PostGIS 3.7.0 & GPLv2 Boundary
- PostGIS 3.7.0 is licensed under **GPL-2.0-or-later**.
- In an application where PostGIS is installed as a PostgreSQL extension on a hosted backend server, end users interact only over HTTP/REST, gRPC, or WebSocket network connections. Under copyright law and the Free Software Foundation's interpretation, **GPLv2 does not consider network access to constitute "distribution"**. Therefore, copyleft is not triggered, and closed-source proprietary code in the SaaS backend does not need to be published.
- *Warning:* If client-side desktop software or on-premises binaries embedding PostGIS are ever distributed to customers, full GPL source code disclosure would be triggered. For D-17 (cloud SaaS), this constraint is satisfied.

#### Redis 8 Pivot vs Valkey 9
- In March 2024, Redis Ltd abandoned the BSD-3-Clause license for future releases (starting with 7.4 and Redis 8), adopting a dual RSALv2 / SSPLv1 model, and subsequently adding AGPLv3 in Redis 8.
  - **RSALv2:** Non-OSI compliant source-available license. Permitted for internal application caching, but prohibited if any managed cache service is offered.
  - **AGPLv3:** OSI-compliant copyleft, but Section 13 mandates that anyone running a modified version over a network must provide complete source code to all network users.
- **Resolution:** Migrate completely to **Valkey 9.1.3**. Valkey was created by the original Redis core maintainers under the Linux Foundation and remains permanently licensed under the permissive **3-Clause BSD License (`BSD-3-Clause`)**. It is 100% wire-compatible with Redis clients (e.g. `redis-rs` in Rust).

---

### 2. Rust Ecosystem Crates

All audited Rust crates were queried directly against the official `crates.io` registry on 2026-11-12:

```
highs            2.4.0   MIT
clarabel         0.11.1  Apache-2.0
faer             0.24.4  MIT
good_lp          1.15.3  MIT
nalgebra-sparse  0.12.0  Apache-2.0
sqlx             0.9.0   MIT OR Apache-2.0
axum             0.8.9   MIT
tonic            0.14.9  MIT
polars           0.55.2  MIT
datafusion       55.2.0  Apache-2.0
arrow            60.1.0  Apache-2.0
```

- **Compliance Assessment:** Every crate in the simulation pipeline is covered by either **MIT** or **Apache-2.0**. Both licenses are permissive, grant explicit patent licenses (Apache-2.0), allow commercial exploitation in proprietary closed-source SaaS products, and require only simple copyright attribution in compiled binary notices or documentation.

---

### 3. Optimization Solvers & Linear Algebra

#### Open & Permissive Solvers
- **HiGHS (v1.16.0):** Developed at the University of Edinburgh. Full C++ source code is licensed under the **MIT License**. Handles linear programming (dual and primal simplex, interior point), mixed-integer programming (branch-and-cut), and quadratic programming. Can be statically or dynamically linked into commercial proprietary software without royalties or code disclosure.
- **Clarabel (0.11.1):** Developed at the University of Oxford. Written in pure Rust and licensed under **Apache-2.0**. Solves conic optimization problems (LP, QP, SOCP, exponential and power cones) using an interior-point method with homogeneous self-dual embedding. Permissive for SaaS.

#### Weak Copyleft Solvers: Ipopt & SuiteSparse KLU
- **Ipopt (3.14.20):** Licensed under the **Eclipse Public License 2.0 (EPL-2.0)**. EPL-2.0 is a weak copyleft license applying at the file level. Modifying Ipopt files requires sharing those specific modifications if distributed. Running Ipopt inside a closed-source SaaS backend triggers no source sharing requirements.
  - *Linear Solver Trap:* Ipopt itself does not solve linear systems; it relies on sparse symmetric linear solvers. The standard free solver is **MUMPS** (CeCILL-C, LGPL-like) or **SPRAL** (BSD-3-Clause). Academic linear solvers like **HSL (MA27, MA57, MA86)** are proprietary and strictly forbid commercial SaaS use without purchasing an expensive commercial license from STFC/UKRI.
- **SuiteSparse KLU (v7.14.1):** Authored by Prof. Tim Davis. KLU is specifically designed for circuit simulation and power network matrices (bordered block diagonal / unsymmetric sparse systems). Licensed under **LGPL-2.1+**. In a server-side SaaS application, LGPL copyleft does not trigger across network boundaries. If on-premises client binaries are distributed, dynamic linking (`libklu.so`) must be used so that end users can relink the library.

#### NVIDIA GPU Acceleration Libraries
- **NVIDIA cuDSS (v1.0.0):** NVIDIA's direct sparse linear solver running on Tensor Cores and CUDA cores. General Availability v1.0.0 was released under the **License Agreement for NVIDIA Math Libraries Software Development Kits**. The license grants a royalty-free right to use, link, and run cuDSS on NVIDIA GPU architectures. Closed-source commercial SaaS backends may deploy cuDSS on cloud or lab GPU hardware without license fees. Unmodified dynamic runtime libraries (`libcudss.so`) may be redistributed.
- **NVIDIA cuOpt (26.10):** Route optimization and LP/MILP heuristics engine. The Python/C++ interfaces on GitHub are open-source under **Apache-2.0**. However, the high-throughput containerized microservices and NIM endpoints distributed via NVIDIA NGC are commercial products covered by the **NVIDIA AI Enterprise (NVAIE)** license, requiring paid subscription licenses per GPU for enterprise production deployment.

#### Commercial Solvers: Gurobi & FICO Xpress
- **Gurobi (14.0):** The industry standard for commercial MILP and non-convex quadratic programming. 
  - *SaaS Restriction:* Standard Gurobi licenses (Named-User, Single-Machine) explicitly prohibit multi-tenant hosting, service bureaus, and SaaS embedding.
  - *SaaS Architecture:* Running Gurobi in a containerized SaaS backend requires negotiating an **ISV / OEM Agreement** with Gurobi and deploying the **Web License Service (WLS)**. Under WLS, container pods authenticate against Gurobi's cloud license servers (`token.gurobi.com`) using cryptographic JSON Web Tokens (`WLSACCESSID`, `WLSSECRET`, `LICENSEID`) from a contracted concurrency token pool.
- **FICO Xpress (10.0):** Commercial solver suite with GPU-accelerated MIP heuristics and PDHG.
  - *Community License Limits:* The free Community Edition distributed via pip/conda restricts problem sizes to 5,000 variables and 5,000 constraints. A national GB + Ireland transmission model exceeds this within a single time slice.
  - *SaaS Architecture:* Requires a negotiated commercial OEM/ISV server license.

---

### 4. Hardware Platform Compatibility & CUDA 13

#### CUDA 13 Architecture Deprecation
NVIDIA officially drew the support cutoff line at the **Turing** architecture in CUDA 13:

| Architecture | Compute Capability | Flag | CUDA 13 Support Status | Lab Hardware Impact |
|---|---|---|---|---|
| **Maxwell** | 5.0, 5.2 | `sm_50`, `sm_52` | **DROPPED** | N/A |
| **Pascal** | 6.0, 6.1 | `sm_60`, `sm_61` | **DROPPED** | **GTX 1080 Ti 11 GB (Pascal, `sm_61`) CANNOT run or build on CUDA 13.** Must retain CUDA 12.x or retire. |
| **Volta** | 7.0 | `sm_70` | **DROPPED** | N/A |
| **Turing** | 7.5 | `sm_75` | **SUPPORTED** (Minimum Baseline) | **Quadro RTX 4000 8 GB (`sm_75`) is supported as the minimum architecture.** |
| **Ampere** | 8.0, 8.6 | `sm_80`, `sm_86` | **SUPPORTED** | Fully supported |
| **Ada Lovelace** | 8.9 | `sm_89` | **SUPPORTED** | Fully supported |
| **Blackwell** | 10.0, 12.0 | `sm_100`, `sm_120` | **SUPPORTED** | **RTX 5060 Ti 16 GB (Blackwell, `sm_120`) has full primary support in CUDA 13.** |

#### Hardware Consolidation Recommendation (D-42)
- The GTX 1080 Ti (Pascal) is incompatible with the CUDA 13 toolchain.
- The Quadro RTX 4000 (Turing) remains functional on CUDA 13 as the minimum baseline.
- Consolidating the lab workstation to the single **RTX 5060 Ti 16 GB (Blackwell)** aligns with CUDA 13, provides 16 GB of unified high-speed VRAM for batch dispatch jobs, and eliminates legacy driver conflicts.

#### RTX 5060 Ti Double-Precision (FP64) Throughput & Simulation Implications
- **FP64:FP32 Hardware Ratio:** In all consumer GeForce Blackwell GPUs (GB20x silicon, including RTX 5090, 5080, and 5060 Ti), the double-precision (FP64) compute throughput is hardware-limited to **1 : 64** (~1.56%) of single-precision (FP32) performance.
  - Each Blackwell Streaming Multiprocessor (SM) contains **2 FP64 cores** for every **128 FP32 cores**.
  - NVIDIA includes these minimal FP64 cores solely for numerical correctness and API compatibility, not high-performance double-precision computing.
- **Power System Numerical Implications:**
  - Standard AC power flow (Newton-Raphson) and AC Optimal Power Flow (ACOPF) algorithms require solving ill-conditioned non-linear equations where double-precision (FP64) is essential to prevent divergence or ill-conditioned matrix factorization errors.
  - Attempting to solve FP64 sparse linear systems (e.g. KLU or Cholesky factorizations) on the RTX 5060 Ti will yield severely degraded performance (~0.5–1.0 TFLOPS FP64) compared to AVX2/AVX-512 execution on the AMD Ryzen 9 5900X CPU.
  - **Optimal Workload Partitioning:**
    1. **CPU (Ryzen 9 5900X):** Primary execution engine for ACOPF, Newton-Raphson power flow, KLU/HiGHS sparse linear algebra, and strict FP64 matrix operations.
    2. **GPU (RTX 5060 Ti 16 GB):** Utilised for highly parallel, single-precision (FP32) or mixed-precision workloads:
       - First-order LP algorithms (e.g. cuOpt / PDHG - Primal-Dual Hybrid Gradient).
       - Massive Monte Carlo contingency simulations (N-1 screening where DC power flow approximation is evaluated in FP32).
       - Time-series weather and solar geospatial raster processing.
       - MapLibre / deck.gl client-side map rendering.

---

## Sources Checked

All official sources accessed and verified on **2026-11-12**:
1. PostgreSQL Global Development Group Versioning & Release Policy: `https://www.postgresql.org/support/versioning/` and `https://www.postgresql.org/docs/18/`
2. PostGIS Official Portal & Release Notes: `https://postgis.net`
3. TimescaleDB GitHub Releases & Compatibility Matrix: `https://github.com/timescale/timescaledb/releases`
4. Timescale Licensing & Terms (TSL vs Apache-2.0): `https://www.timescale.com/legal/licenses`
5. Redis Licensing Notice & Redis 8 Repository: `https://redis.io/legal/licenses/`
6. Valkey Official Repository & Releases: `https://github.com/valkey-io/valkey/releases`
7. Crates.io Official API (Version & License verification): `https://crates.io/api/v1/crates/` for crates `highs`, `clarabel`, `faer`, `good_lp`, `nalgebra-sparse`, `sqlx`, `axum`, `tonic`, `polars`, `datafusion`, `arrow`
8. HiGHS Solver Official GitHub Repository: `https://github.com/ERGO-Code/HiGHS`
9. Ipopt Official GitHub Repository: `https://github.com/coin-or/Ipopt`
10. SuiteSparse & KLU Official Repository: `https://github.com/DrTimothyAldenDavis/SuiteSparse`
11. NVIDIA cuDSS Documentation & SLA: `https://developer.nvidia.com/cudss`
12. NVIDIA cuOpt Documentation & License Terms: `https://developer.nvidia.com/cuopt-embedded` and `https://github.com/NVIDIA/cuopt`
13. Gurobi Optimizer Reference Manual & WLS Licensing Guide: `https://www.gurobi.com/documentation/` and `https://www.gurobi.com/solutions/licensing/`
14. FICO Xpress Optimization Documentation: `https://www.fico.com/en/products/fico-xpress-optimization`
15. MapLibre GL JS (npm registry & GitHub): `https://registry.npmjs.org/maplibre-gl` and `https://github.com/maplibre/maplibre-gl-js`
16. deck.gl (npm registry & GitHub): `https://registry.npmjs.org/deck.gl` and `https://github.com/visgl/deck.gl`
17. NVIDIA CUDA 13 Release Notes & Compute Capability Support Guide: `https://docs.nvidia.com/cuda/cuda-toolkit-release-notes/`
18. NVIDIA Blackwell Architecture Technical Whitepaper (GeForce RTX 50-Series SM & FP64 Specifications): `https://www.nvidia.com/en-us/geforce/graphics-cards/50-series/`
