# Software Dependency Licences & Hardware Platform Compatibility

_Reviewed: 2026-11-12. Versions are stable releases observed on that date, not dependency pins._

## Scope and conclusions

This supports **D-17** (commercial SaaS), **D-20** (Rust), **D-42** (hardware) and the **D-3
PostgreSQL 18 + Redis baseline**. It does not accept an ADR, replace the owner's Redis decision
or certify an unbuilt application's complete dependency graph.

- All **11 named Rust crate versions and SPDX expressions were confirmed on crates.io**.
  Direct-package licences are permissive; optional/native/transitive dependencies still need review.
- **TimescaleDB is now 2.32.0**, not 2.31.0. PostgreSQL 18 support really began in 2.23.0.
  Apache-2.0 and Timescale-licensed portions are distinct, not a free choice for every feature.
- **Redis 8.10.3** offers RSALv2, SSPLv1 or AGPLv3. It is **not categorically forbidden in a
  proprietary application SaaS**. Licence choice, boundaries and modifications matter.
  **Valkey 9.1.3** has a BSD-3-Clause core and avoids Redis's service/copyleft conditions, but
  changing the baseline needs an owner decision and compatibility validation.
- PostGIS's GPL does not itself require publishing a separate SaaS application merely because
  the database is accessed over a network. Distribution is a separate assessment.
- cuOpt's open **solver implementation**, not just interfaces, is Apache-2.0. A paid enterprise
  subscription is not a blanket prerequisite for self-hosting that code.
- Current Gurobi terms include a **qualified value-added hosting exception**. Neither a universal
  SaaS ban nor mandatory ISV-plus-WLS deployment is supported by those terms.
- CUDA 13 drops **offline compilation targets below capability 7.5**. Previously compiled Pascal
  applications do not necessarily stop running.
- The **RTX 5060 Ti FP64:FP32 arithmetic-throughput ratio is 1:64**, supported by NVIDIA's
  documentation. It does **not** prove that a CPU is faster for a particular sparse solve.

**UNVERIFIED** identifies an outstanding check, not permission to ship. This is an engineering
inventory; actual use must satisfy the applicable licence/contract.

## Databases and cache

| Component | Stable version observed | Licence | Commercial application-SaaS position and evidence |
|---|---|---|---|
| **PostgreSQL** | **18.7** in the selected 18 major line | `PostgreSQL` | Permissive. [Version policy](https://www.postgresql.org/support/versioning/) lists 18.7, released 2026-11-12; not `18.0 (minor 18.7)`. [Licence](https://www.postgresql.org/about/licence/). |
| **PostGIS** | **3.7.0** | `GPL-2.0-or-later` | Suitable for server-side SaaS; comply with GPL if covered software is conveyed. [Release](https://postgis.net/2026/10/PostGIS-3.7.0/), [COPYING](https://raw.githubusercontent.com/postgis/postgis/3.7.0/COPYING), [source notice](https://raw.githubusercontent.com/postgis/postgis/3.7.0/liblwgeom/lwgeom_geos.c). |
| **TimescaleDB** | **2.32.0**, published 2026-11-12; changelog dated 2026-11-11 | Apache-2.0 portions plus **Timescale License (TSL)** portions; not a licence choice for all code | Qualifying value-added application SaaS is permitted; managed DBaaS/interface exposure is restricted. [Release](https://github.com/timescale/timescaledb/releases/tag/2.32.0), [boundary](https://raw.githubusercontent.com/timescale/timescaledb/2.32.0/LICENSE), [TSL](https://raw.githubusercontent.com/timescale/timescaledb/2.32.0/tsl/LICENSE-TIMESCALE). |
| **Redis** | **8.10.3** | Choice of **RSALv2 / SSPLv1 / AGPLv3** | Conditional, not prohibited merely because the application is commercial. [Release](https://github.com/redis/redis/releases/tag/8.10.3), [licence](https://raw.githubusercontent.com/redis/redis/8.10.3/LICENSE.txt), [official guide](https://redis.io/legal/licenses/). |
| **Valkey** | **9.1.3**; maintenance tags **8.1.11 / 8.0.12** also confirmed | Core `BSD-3-Clause`; bundled-code notices also apply | Permissive commercial service use. [Stable release](https://github.com/valkey-io/valkey/releases/tag/9.1.3), [COPYING](https://raw.githubusercontent.com/valkey-io/valkey/9.1.3/COPYING), [releases](https://github.com/valkey-io/valkey/releases). **9.2.0-rc2 is a prerelease**, not the stable recommendation. |

### TimescaleDB: PG18, feature boundary and SaaS

The [changelog](https://raw.githubusercontent.com/timescale/timescaledb/2.32.0/CHANGELOG.md)
records full PostgreSQL 18 support in **2.23.0 (2025-10-29)**. Release 2.32.0 is the current
source checked here; its [build-version guard](https://raw.githubusercontent.com/timescale/timescaledb/2.32.0/CMakeLists.txt) accepts PG18. The [compatibility docs](https://docs.tigerdata.com/self-hosted/latest/upgrades/upgrade-pg/)
also identify PG18-compatible releases. A particular Docker tag, Linux package or extension
binary remains **UNVERIFIED** until checked; do not infer `2.31.0-pg18` artifact availability.

| Feature | Licence boundary verified from release-pinned source |
|---|---|
| Core hypertables / ordinary chunk operations | Apache-2.0: [hypertables](https://raw.githubusercontent.com/timescale/timescaledb/2.32.0/src/hypertable.c), [chunks](https://raw.githubusercontent.com/timescale/timescaledb/2.32.0/src/chunk.c). Manual chunk dropping is not automated retention policy. |
| Basic `time_bucket()` | Apache-2.0: [implementation](https://raw.githubusercontent.com/timescale/timescaledb/2.32.0/src/time_bucket.c). |
| Continuous aggregates | TSL: [materialization](https://raw.githubusercontent.com/timescale/timescaledb/2.32.0/tsl/src/continuous_aggs/materialize.c). |
| Native compression / columnstore implementation | TSL: [compression API](https://raw.githubusercontent.com/timescale/timescaledb/2.32.0/tsl/src/compression/api.c). No guaranteed compression percentage/speedup is inferred. |
| Automated retention policies | TSL: [retention API](https://raw.githubusercontent.com/timescale/timescaledb/2.32.0/tsl/src/bgw_policy/retention_api.c). |
| Toolkit hyperfunctions | A **separate extension/repository** with its own [licence](https://raw.githubusercontent.com/timescale/timescaledb-toolkit/main/LICENSE). Do not treat all hyperfunctions as bundled core or assume identical terms for every function. |
| Tiering / cloud-product features | Availability and terms in a selected self-hosted/community build are **UNVERIFIED**. Blanket community availability is not established. |

The [TSL](https://raw.githubusercontent.com/timescale/timescaledb/2.32.0/tsl/LICENSE-TIMESCALE)
has separate internal-use and **Value Added Products or Services** grants. Section 2.1(a)'s
internal-use grant excludes exposing DDL/DML interfaces to outsiders, including through wrappers.
The value-added grant in 2.1(b), with definition 3.10, permits qualifying application services:
they must add substantial non-database value, not primarily offer database storage/operations,
and prohibit customers from changing database schemas/structural objects, technically or
contractually. Customer-notice, distribution and other conditions also apply. The vendor's
[explanation](https://www.tigerdata.com/legal/licenses) distinguishes application use from hosted
DBaaS. **Our energy SaaS is not DBaaS**, as specified for this review, but must meet these
conditions. TSL does **not** give a blanket right to offer managed Timescale databases;
“the sole restriction is resale” was inadequate.

### Redis versus Valkey for commercial SaaS

Redis 7.2 and earlier BSD licensing, Redis 7.4's RSAL/SSPL change and Redis 8's added AGPL option
are separate version-specific facts. Do not apply today's licence retroactively to every older
release. For Redis 8, document **one applicable licence path**:

- **AGPLv3:** a separately running Redis does not automatically relicense an independently
  communicating application. Conveying covered software entails distribution obligations;
  [section 13](https://www.gnu.org/licenses/agpl-3.0.html) additionally requires a source offer to
  remote users interacting with a modified covered program. Assess modifications, combined
  works and service users; “every SaaS must publish everything” is incorrect.
- **RSALv2:** [limitations](https://redis.io/legal/rsalv2-agreement/) restrict offering Redis
  functionality to third parties as a service/in distributed form, including products whose value
  primarily derives from Redis or accomplishes its primary purpose. This is broader than selling
  something labelled DBaaS. A private simulator cache is not automatically such a service;
  assess the actual boundary.
- **SSPLv1:** service-source conditions can apply when offering the program's functionality as a
  service. It is not a permissive substitute for assessing architecture; Redis's release licence
  and official guide link the governing text.
- **Valkey:** the BSD-3-Clause core has no equivalent AGPL network-source or Redis service-use
  condition. Preserve notices and audit bundled code/modules. Protocol ancestry does **not**
  prove compatibility with all Redis 8 commands, modules and features.

**Position:** Valkey is the simpler permissive alternative; Redis 8 remains possible for
commercial application SaaS under an appropriate, complied-with licence. This does **not** adopt
Valkey or override **D-3**. Resolve the Redis assessment or obtain an owner-approved replacement
decision before implementation.

### PostGIS and copyleft boundaries

PostGIS's GPL is not AGPL: running the database on our servers and returning API results does
not trigger a network-source requirement. The [GPL FAQ](https://www.gnu.org/licenses/gpl-faq.html)
distinguishes internal use, distribution and separate-program communication. Distributing
PostGIS, modifications or combined GPL-covered software entails separate obligations. A frontend
querying a backend is not linking PostGIS into a distributed proprietary binary. Neither “GPL
is forbidden for SaaS” nor “server use makes all distributions safe” is accurate.

## Rust crates: registry-verified versions and licences

Each version's `license`, publication timestamp and yanked/prerelease status was checked against
crates.io. **All 11 original pairs were correct**; the blanket licence conclusions were overstated.

| Crate | Latest stable observed | SPDX expression | Official version metadata |
|---|---|---|---|
| `highs` | `2.4.0` | `MIT` | [crates.io](https://crates.io/api/v1/crates/highs/2.4.0) |
| `clarabel` | `0.11.1` | `Apache-2.0` | [crates.io](https://crates.io/api/v1/crates/clarabel/0.11.1) |
| `faer` | `0.24.4` | `MIT` | [crates.io](https://crates.io/api/v1/crates/faer/0.24.4) |
| `good_lp` | `1.15.3` | `MIT` | [crates.io](https://crates.io/api/v1/crates/good_lp/1.15.3) |
| `nalgebra-sparse` | `0.12.0` | `Apache-2.0` | [crates.io](https://crates.io/api/v1/crates/nalgebra-sparse/0.12.0) |
| `sqlx` | `0.9.0` | `MIT OR Apache-2.0` | [crates.io](https://crates.io/api/v1/crates/sqlx/0.9.0) |
| `axum` | `0.8.9` | `MIT` | [crates.io](https://crates.io/api/v1/crates/axum/0.8.9) |
| `tonic` | `0.14.9` | `MIT` | [crates.io](https://crates.io/api/v1/crates/tonic/0.14.9) |
| `polars` | `0.55.2` | `MIT` | [crates.io](https://crates.io/api/v1/crates/polars/0.55.2) |
| `datafusion` | `55.2.0` | `Apache-2.0` | [crates.io](https://crates.io/api/v1/crates/datafusion/55.2.0) |
| `arrow` | `60.1.0` | `Apache-2.0` | [crates.io](https://crates.io/api/v1/crates/arrow/60.1.0) |

MIT/Apache-2.0 permit proprietary use subject to conditions, including notices. Apache-2.0 also
contains an express patent grant and modified-file/NOTICE conditions. This is **not a `cargo-deny`
result**: optional `good_lp` backends, native bindings and the eventual feature-selected transitive
graph can introduce other licences. No production engine lockfile/build configuration exists yet.

## Solvers, sparse libraries and frontend dependencies

| Component | Stable version observed | Licence / commercial position | Official evidence |
|---|---|---|---|
| **HiGHS C++** | **1.16.0** | `MIT`; commercial use with notices. Not the Rust binding's version. | [Release](https://github.com/ERGO-Code/HiGHS/releases/tag/v1.16.0), [licence](https://raw.githubusercontent.com/ERGO-Code/HiGHS/v1.16.0/LICENSE.txt). |
| **Ipopt** | **3.14.20** | `EPL-2.0`; server execution is not distribution. Conveyed covered code/modifications and native dependencies need compliance. | [Release](https://github.com/coin-or/Ipopt/releases/tag/releases/3.14.20), [licence](https://raw.githubusercontent.com/coin-or/Ipopt/releases/3.14.20/LICENSE). |
| **SuiteSparse / KLU** | SuiteSparse **7.14.1**, **KLU 2.3.6** | KLU `LGPL-2.1-or-later` (`LGPL-2.1+` in source). Network SaaS use is not relinkable application distribution. | [Component versions](https://github.com/DrTimothyAldenDavis/SuiteSparse/releases/tag/v7.14.1), [KLU licence](https://raw.githubusercontent.com/DrTimothyAldenDavis/SuiteSparse/v7.14.1/KLU/Doc/License.txt). |
| **NVIDIA cuDSS** | **1.0.0** | Proprietary **License Agreement for NVIDIA Math Libraries Software Development Kits**. Conditional SDK use/application-bundled distribution, not open source or unrestricted redistribution. | [Release notes](https://docs.nvidia.com/cuda/cudss/latest/release_notes.html), [agreement](https://docs.nvidia.com/cuda/cudss/latest/license.html). |
| **NVIDIA cuOpt** | **26.10.01**, released 2026-11-12 | Open solver `Apache-2.0`; self-hosting this code does not require enterprise support. Selected NIM/NGC artifacts/contracts need separate terms. | [Release](https://github.com/NVIDIA/cuopt/releases/tag/v26.10.01), [licence](https://raw.githubusercontent.com/NVIDIA/cuopt/v26.10.01/LICENSE), [published licence](https://docs.nvidia.com/cuopt/user-guide/latest/license.html). |
| **Gurobi / gurobipy** | **14.0 / 14.0.0** | Proprietary; production rights/order scope govern. Qualified hosting exception; no universal ISV/WLS requirement. | [Release notes](https://docs.gurobi.com/projects/optimizer/en/current/reference/releasenotes.html), [vendor metadata](https://pypi.org/pypi/gurobipy/json), [legal centre](https://www.gurobi.com/company/legal/overview). |
| **FICO Xpress** | **10.0.0** vendor package | Proprietary; standard shrinkwrap restricts third-party/service-bureau use. Obtain written hosting/embedding rights. | [Package](https://pypi.org/project/xpress/10.0.0/), [metadata](https://pypi.org/pypi/xpress/10.0.0/json), [shrinkwrap page](https://www.fico.com/en/shrinkwrap-license-agreement-fico-xpress-optimization-suite-on-premises). The actual licence in the versioned wheel was inspected. |
| **MapLibre GL JS** | **6.18.0** | `BSD-3-Clause`; notices apply to browser distribution too. | [Release](https://github.com/maplibre/maplibre-gl-js/releases/tag/v6.18.0), [npm metadata](https://registry.npmjs.org/maplibre-gl/6.18.0). |
| **deck.gl** | **9.4.1** | `MIT`; preserve notices for browser-distributed code. | [npm metadata](https://registry.npmjs.org/deck.gl/9.4.1), [publisher repository](https://github.com/visgl/deck.gl). npm's stable tag is 9.4.1; GitHub maintenance release 9.3.12 does not disprove it. |

### Linking, optional solvers and SDKs

- **KLU:** 7.14.1 was the suite version, **not KLU's**. Dynamic linking is one distribution
  compliance route, not a universal LGPL requirement; compliant static distribution can provide
  relinking materials. Audit each SuiteSparse component separately.
- **Ipopt's linear solver:** [MUMPS licensing](https://mumps-solver.org/index.php?page=dwnld)
  identifies CeCILL-C with component exceptions; [SPRAL](https://raw.githubusercontent.com/ralna/spral/master/LICENCE)
  is BSD-3-Clause. Optional ordering/BLAS libraries need review. [HSL licensing](https://www.hsl.rl.ac.uk/licensing.html)
  distinguishes main-library academic/commercial and archive personal-use/incorporation terms.
  The claim that every HSL package forbids any commercial use without an expensive purchase was
  too broad; package/version and hosting matter.
- **cuDSS:** distribution requires material additional application functionality and application-only
  access to bundled distributable portions. Preserve notices and assess critical-application,
  export and prerelease conditions. Unmodified `libcudss.so` is not an unconditional standalone
  redistribution right; blanket royalty-free rights were not established by the inspected grant.
- **cuOpt:** Apache-2.0 covers more than wrappers. The mandatory-per-GPU NVAIE-fee claim is
  withdrawn. Selected NIM images, enterprise packages and support terms remain **UNVERIFIED**.

### Gurobi and Xpress: hosting permission versus deployment

The [current Gurobi EULA, version 14.0](https://cdn.bfldr.com/KUZMZM51/as/5zgkx5ng7whk9gm8j8j8xgx3/gurobi-end-user-license-agreement)
section 3(v) excepts hosting inside a customer-developed application with significant additional
functionality that does not let users formulate/solve arbitrary optimization models. Order,
licence scope and controlling agreements still apply. Our simulator's eventual fit is **UNVERIFIED**
until its boundary/contract are assessed. Do not automatically require ISV terms or assume this
exception covers a general-purpose hosted solver API.

The [Product Licensing Policy](https://cdn.bfldr.com/KUZMZM51/as/99cj7jn9fhvtbh4ff45m8n7r/gurobi-product-licensing-policy)
section 2.6 lists multiple container arrangements: WLS, clients of appropriate licence/compute
servers, and Instant Cloud. **WLS is a technology, not synonymous with ISV rights**. Its
[setup guide](https://support.gurobi.com/hc/en-us/articles/13232844297489-How-do-I-set-up-a-Web-License-Service-WLS-license)
describes container/non-container licences; token pools are not the sole SaaS architecture.

The **Xpress 10.0.0 wheel's `dist-info/licenses/LICENSE.txt`** defines Internal Use and restricts
third-party benefit/access and service-bureau use (sections 1, 2.1 and 2.4); community use is also
internal (2.1.3). Appropriate signed terms can change those rights; exact OEM/ISV terms/prices
are **UNVERIFIED**. The public FICO page was script-only, so the versioned vendor package supplied
primary legal evidence. The asserted **5,000 variables and 5,000 constraints** is **UNVERIFIED**,
not an established pair of independent ceilings; the inspected agreement does not substantiate
it. Confirm size limits, GPU features and entitlements for the selected product regardless of fit.

## CUDA 13 and lab hardware

NVIDIA's [support explanation](https://developer.nvidia.com/blog/navigating-gpu-architecture-support-a-guide-for-nvidia-cuda-developers/)
states CUDA 13 removes **offline compilation** below **7.5**: Maxwell, Pascal and Volta. CUDA
12.9 is the last toolkit generation for those targets. Applications compiled with older supported
toolkits can still run on compatible drivers. Driver lifetime, compilation and library targets differ.

| Lab GPU | Architecture / capability | Consequence |
|---|---|---|
| GTX 1080 Ti | Pascal, `sm_61` | Not targetable by CUDA 13's compiler. Retain appropriate older toolkit/application/driver support if used; “cannot run any CUDA application” is incorrect. |
| Quadro RTX 4000 | Turing, `sm_75` | Meets the CUDA 13 compiler floor, not a guarantee for every library/package. |
| RTX 5060 Ti | Consumer Blackwell, `sm_120` | Listed as **12.0** in NVIDIA's [GPU table](https://developer.nvidia.com/cuda-gpus), not data-centre Blackwell `sm_100`. Check library/driver/platform requirements. |

[cuDSS support](https://docs.nvidia.com/cuda/cudss/latest/index.html) is conditional on the matching
toolkit; [cuOpt requirements](https://docs.nvidia.com/cuopt/user-guide/latest/system-requirements.html)
also depend on package/toolkit. One CUDA version does not establish universal library support.
The planned 16 GB is dedicated GPU memory, not CPU-shared physical RAM; CUDA Unified Memory
does not make this a unified-RAM machine. Guaranteed driver-conflict elimination is not established.

### RTX 5060 Ti FP64:FP32 ratio

This GPU is **compute capability 12.0**. The official
[CUDA 12.9.1 Programming Guide, Table 7](https://docs.nvidia.com/cuda/archive/12.9.1/cuda-c-programming-guide/index.html#arithmetic-instructions-throughput-native-arithmetic-instructions)
gives **128 FP32** versus **2 FP64** add/multiply/multiply-add results per SM clock for 12.0.
**Derived ratio: 2 / 128 = 1 / 64 = 1.5625%** for these pipelines. The
[RTX Blackwell whitepaper](https://images.nvidia.com/aem-dam/Solutions/geforce/blackwell/nvidia-rtx-blackwell-gpu-architecture.pdf)
also states 1/64 FP64-to-FP32 TFLOPs for GB202. That chip-specific statement alone does not
specify every Blackwell GPU; the 5060 Ti conclusion additionally uses the capability table.
Throughput entries are not, by themselves, physical core counts.

**Performance is UNVERIFIED:** the 0.5–1.0 TFLOPS estimate and categorical “Ryzen 5900X is faster
at FP64 sparse solves” had no measured evidence. Peak arithmetic throughput is not end-to-end
sparse performance. Retain the CPU-primary baseline and evaluate GPU/mixed-precision paths with
numerical validation/benchmarks, not the ratio alone. The unsupported AVX-512 comparison is also
withdrawn.

## Verification

**Date: 2026-11-12.** All 25 original software rows and seven architecture rows, repeated versions,
licence summaries, solver details and hardware assertions were reviewed against registries, releases, source
licences, legal agreements and NVIDIA docs, including same-day PostgreSQL/TimescaleDB/cuOpt
releases. Candidate versions are not silently adopted as stack decisions.

### Changes made

1. Updated **TimescaleDB 2.31.0 → 2.32.0**, **Redis 8.0/8.2 placeholders → 8.10.3** and
   **cuOpt 26.10/container assumption → 26.10.01**. Corrected PostgreSQL's format and
   **KLU 7.14.1 → KLU 2.3.6 within SuiteSparse 7.14.1**.
2. Confirmed PostgreSQL **18.7**, PostGIS **3.7.0**, Valkey **9.1.3 / 8.1.11 / 8.0.12**, HiGHS
   **1.16.0**, Ipopt **3.14.20**, cuDSS **1.0.0**, Gurobi **14.0 / gurobipy 14.0.0**, Xpress
   **10.0.0**, MapLibre **6.18.0** and deck.gl **9.4.1**. FICO package metadata is verified;
   separate release notes were not recovered; extra claimed GPU features remain **UNVERIFIED**.
3. Confirmed **all 11 Rust pairs**; replaced unconditional/permissive-entire-graph language with
   notice, feature-selection and transitive-audit conditions.
4. Confirmed **PG18 since TimescaleDB 2.23.0** and release-pinned licence headers; distinguished
   Apache core/manual operations, TSL policies/compression/aggregates, Toolkit and unverified cloud
   features. Qualified the application-SaaS/DBaaS boundary and other TSL conditions.
5. Replaced blanket Redis rejection/unilateral Valkey adoption with licence-specific analysis;
   preserved **D-3** and removed the Redis 8 feature-compatibility guarantee.
6. Corrected GPL/LGPL implications: network use is not distribution; dynamic linking is not the
   only LGPL-compliance route.
7. Corrected cuOpt's open-engine licensing/mandatory enterprise fee and cuDSS's conditional SDK
   distribution instead of asserting unrestricted royalty-free redistribution.
8. Inspected Gurobi's **hosting exception**/multiple container routes and Xpress's shrinkwrap.
   Removed mandatory ISV/WLS claims, flagged unconfirmed Xpress limits/contracts, qualified HSL.
9. Narrowed the Pascal drop to compilation/support; confirmed **1:64** and removed unmeasured
   peak/CPU-speed, physical unified-memory and AVX-512 claims.
10. Replaced the nonexistent cuOpt embedded URL/generic sources with specific evidence. Across
    both files, **68 original URLs** were checked: 48 HTTP 200, 12 HTTP 404, one HTTP 400, five
    HTTP 403 and two fetch failures. HTTP 200 did not prove a licence/dataset existed. All **134
    distinct final source URLs** returned HTTP 200, including an official article-API replacement
    for an ENTSO-E legal-page link blocked by access controls.

### Remaining checks before implementation

- **UNVERIFIED:** eventual lockfile, native/optional libraries, flags and transitive licences;
  perform D-28 `cargo-deny` once an application exists.
- **UNVERIFIED:** binary artifacts, individual Toolkit functions, cloud tiers and proprietary
  packages not covered by the inspected terms.
- **UNVERIFIED:** final Redis licence/architecture, Gurobi hosting-exception fit/order scope,
  Xpress SaaS rights/size limits and negotiated prices.
- **UNVERIFIED:** actual CPU/GPU performance and numerical suitability.

**Verdict: merge after fixes made**, as qualified research, not procurement approval, a completed
dependency audit or an accepted ADR.
