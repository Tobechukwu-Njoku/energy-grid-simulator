# Adversarial Requirements Review

**Reviewer:** Gemini Flash 3.8 (Adversarial Review Sub-Agent)  
**Date:** 2026-11-12  
**Target:** `Tobechukwu-Njoku/energy-grid-simulator`  
**Scope:** `AGENTS.md`, `docs/process/team.md`, `docs/requirements/*`, `docs/architecture/stack-candidates.md`, `docs/research/prior-art.md`  

---

## Executive Summary

This review is deliberately adversarial. Its purpose is not to praise the vision, but to systematically stress-test and break the requirements, architectural assumptions, market fidelity claims, and autonomous agent process before code implementation begins.

The project proposes building a high-performance, full-transmission electricity grid and market simulator covering Great Britain and the all-island Single Electricity Market (I-SEM) of Ireland in Rust, validated against PyPSA, capable of unit commitment and Balancing Mechanism redispatch over a full weather year, executed autonomously by a multi-model agent pipeline.

Twelve critical vulnerabilities and contradictions have been identified across five core attack vectors:
1. **Contradictions in Decisions:** Irreconcilable clashes between the zero-constraint performance mandate and consumer-tier hardware, and between the ODbL data ban and the full-transmission network requirement.
2. **Untestable & Flawed Verification:** Mechanistically impossible verification of Balancing Mechanism redispatch against PyPSA, and undefined numerical tolerances.
3. **Fatal Architectural & Hardware Assumptions:** The complete absence of an algebraic modeling layer for multi-period MILP in the Rust ecosystem, and memory-bandwidth bottlenecks on consumer desktop platforms.
4. **Autonomous Multi-Model Failure Modes:** A governance deadlock where GPT-authored documents cannot be reviewed under current role rules, alongside predictable failure modes of agent collusion, synthetic mock trivialization, and watermark-stripping audit loss.
5. **GB & Irish Market Deficits:** Neglect of AC reactive/voltage constraints that drive real BM spending, gross oversimplification of the dual-currency I-SEM pool, and omission of post-Brexit decoupling and REMA market reforms.

---

## Findings Matrix

| ID | Severity | Category | File & Decision IDs | Title |
|---|---|---|---|---|
| **ADV-1** | High | Contradictions / Hardware | `03-decisions-log.md` (D-2, D-13 vs D-42, D-48); `05-risks.md` (R-14) | Zero-Constraint High-Performance Premise vs. Consumer Desktop Hardware Reality |
| **ADV-2** | High | Contradictions / Team Process | `team.md`; `03-decisions-log.md` (D-22, D-28, D-49, D-51, D-53) | Governance Deadlock in Multi-Model Review Matrix |
| **ADV-3** | High | Contradictions / Licensing | `03-decisions-log.md` (D-30 vs D-32, D-17); `05-risks.md` (R-10, R-12) | ODbL Share-Alike Ban Clashes Incompatibly with Full Transmission Network Scope |
| **ADV-4** | High | Contradictions / Grid Scope | `03-decisions-log.md` (D-12, D-24 vs D-32, D-44); `06-roadmap.md` | 132 kV Jurisdictional Split Violates Transmission/Distribution Scope Phasing |
| **ADV-5** | High | Verification / Testing | `03-decisions-log.md` (D-26, D-28); `06-roadmap.md` (M2, M5); `05-risks.md` (R-8) | Flawed and Impossible Verification of Balancing Mechanism Against PyPSA |
| **ADV-6** | High | Hidden Assumptions / Stack | `03-decisions-log.md` (D-18, D-20, D-26); `05-risks.md` (R-9); `stack-candidates.md` | Rust Ecosystem Deficit for Full-Year Transmission-Constrained Unit Commitment (MILP) |
| **ADV-7** | High | Market Physics Deficit | `03-decisions-log.md` (D-26, D-40 #4, D-44); `06-roadmap.md` | Balancing Mechanism Redispatch Invalidated by Deferral of AC Voltage and Reactive Constraints |
| **ADV-8** | High | Autonomous Multi-Model Process | `team.md`; `03-decisions-log.md` (D-22, D-28, D-51, D-52, D-53); `AGENTS.md` | Autonomous Agent Collusion, Mock Trivialization, and Audit Provenance Loss |
| **ADV-9** | Med | Market Architecture Deficit | `03-decisions-log.md` (D-11, D-15, D-26, D-38); `06-roadmap.md` | Severe Misrepresentation of I-SEM All-Island Market and Interconnector Dynamics |
| **ADV-10** | Med | Market Evolution Deficit | `03-decisions-log.md` (D-11, D-14, D-15, D-50); `prior-art.md` | Omission of 2021–2026 Structural Shifts: Brexit Decoupling, REMA, MHHS, and New Interconnectors |
| **ADV-11** | Med | Licensing & Infrastructure | `03-decisions-log.md` (D-3, D-17); `05-risks.md` (R-2); `stack-candidates.md` | TimescaleDB and Redis 8 Licensing Risks for Future Commercial SaaS |
| **ADV-12** | Med | Verification / Backtesting | `03-decisions-log.md` (D-18, D-23, D-35); `06-roadmap.md` (M5); `05-risks.md` (R-7, R-8) | Historic Year Backtest Invalidation via Omission of REMIT Outages and Curtailment |

---

## Detailed Findings

### ADV-1: Zero-Constraint High-Performance Premise vs. Consumer Desktop Hardware Reality

- **Severity:** High
- **Concerns:** `docs/requirements/03-decisions-log.md` (D-2, D-13 vs D-42, D-48); `docs/requirements/05-risks.md` (R-14); `docs/architecture/stack-candidates.md`
- **The Attack:**
  D-2 establishes the foundational design axiom: *"Hardware/RAM/power cost is not a constraint; optimize for performance... aim high (e.g. PostgreSQL 18 and Redis)"*. D-13 mandates: *"Hardware target: Linux + NVIDIA GPU(s) (GPU solvers/CUDA in scope)"*.
  However, decisions D-42 and D-48 ground the actual execution environment in consumer desktop components:
  1. A 2017 Pascal GeForce GTX 1080 Ti (11 GB VRAM) and a Quadro RTX 4000 (8 GB VRAM), with a planned consolidation to an entry-level GeForce RTX 5060 Ti (16 GB VRAM, 128-bit memory bus).
  2. An AMD Ryzen 9 5900X (12 cores / 24 threads, consumer dual-channel DDR4 memory).

  This creates an irreconcilable technical bottleneck:
  - **CUDA Deprecation & GPU Solver Incompatibility:** NVIDIA CUDA 12.0+ deprecated support for the Pascal architecture (compute capability 6.1, GTX 1080 Ti). Specialized libraries named in `stack-candidates.md` (such as `cuDSS` and `cuOpt`) target modern enterprise architectures (Ampere/Hopper/Ada) and compute capabilities >= 7.0/8.0.
  - **FP64 Throttling:** Power-flow Newton-Raphson solvers and interior-point OPF solvers require 64-bit double-precision floating-point arithmetic (FP64) to prevent numerical divergence in ill-conditioned admittance matrices. Consumer GeForce cards (including the planned RTX 5060 Ti) have FP64 throughput hardware-throttled to 1/32 or 1/64 of FP32 throughput. Offloading OPF to consumer GPUs will run significantly slower than executing on a multi-core CPU.
  - **Memory Bandwidth Choke:** Unit Commitment branch-and-bound and sparse linear algebra factorizations are memory-bandwidth-bound. The Ryzen 5900X dual-channel DDR4 memory bus tops out at ~45 GB/s. Server platforms (AMD EPYC, Intel Xeon) offer 8 to 12 memory channels delivering 300–460+ GB/s. Building custom CUDA FFI integrations for consumer desktop hardware burns massive development budget while delivering inferior throughput to standard CPU libraries.
- **Evidence:**
  - NVIDIA CUDA Toolkit 12.0 Release Notes (Pascal deprecation): https://docs.nvidia.com/cuda/cuda-toolkit-release-notes/index.html
  - NVIDIA cuOpt System Requirements (modern enterprise GPUs required): https://docs.nvidia.com/cuopt/user-guide/system-requirements.html
  - NVIDIA cuDSS Documentation: https://docs.nvidia.com/cuda/cudss/index.html

---

### ADV-2: Governance Deadlock in Multi-Model Review Matrix

- **Severity:** High
- **Concerns:** `docs/process/team.md`; `docs/requirements/03-decisions-log.md` (D-22, D-28, D-49, D-51, D-53)
- **The Attack:**
  The project specifies autonomous multi-model collaboration where agents plan, build, review, and merge without human intervention between milestones (D-22).
  However, cross-referencing the rules in `team.md` and D-28/D-51 reveals an absolute circular deadlock:
  1. `team.md` Rule 1 mandates: *"Cross-provider review (D-28): every PR is reviewed by a model from a different provider than its author. Claude-authored -> GPT 6.1 Sol reviews. GPT-authored -> Claude reviews."*
  2. The Role table in `team.md` explicitly restricts Claude Opus 5.5: *"Do not use for: Research, reviews, specs, verification (-> GPT)"*.
  3. D-51 reinforces: *"Use GPT 6.1 Sol as much as possible for everything except code generation... Claude Opus 5.5 is reserved for writing code."*
  4. `team.md` Rule 2 and D-49 state: *"Gemini never reviews or merges."*
  5. D-28 requires that every PR must have a *"review by a model other than the author"*.

  When GPT 6.1 Sol authors an Architecture Decision Record (ADR), specification, requirements change, or data pipeline design (as instructed by D-49/D-51):
  - GPT cannot review its own PR (violates D-28).
  - Claude cannot review it (explicitly forbidden by `team.md` Role table and D-51).
  - Gemini cannot review it (explicitly forbidden by `team.md` Rule 2).
  
  Therefore, no agent is permitted to review non-code PRs. The autonomous workflow is mathematically deadlocked.

---

### ADV-3: ODbL Share-Alike Ban Clashes Incompatibly with Full Transmission Network Scope

- **Severity:** High
- **Concerns:** `docs/requirements/03-decisions-log.md` (D-30 vs D-32, D-17); `docs/requirements/05-risks.md` (R-10, R-12); `docs/requirements/06-roadmap.md`
- **The Attack:**
  D-30 strictly prohibits copyleft/share-alike data: *"avoid share-alike data (e.g. ODbL / OSM-derived network data). Only commercially permissive datasets; build our own network model where needed"*. This is mandated to ensure clean licensing for a future commercial SaaS (D-17).
  Simultaneously, D-32 mandates for the MVP: *"full transmission (all 400/275/132 kV nodes in GB; 400/220/110 kV in Ireland)"*.

  This combination of constraints is impossible to satisfy with open public data:
  1. **OpenStreetMap Infection:** Every existing open transmission grid dataset for GB and Ireland (including PyPSA-GB and `osm-power`) derives its spatial line routing and substation coordinates from OpenStreetMap under the Open Database License (ODbL) v1.0. Section 4.4 of ODbL mandates that any "Derivative Database" must be released under ODbL, contaminating commercial SaaS distribution.
  2. **NESO Data Limitations:** The National Energy System Operator (NESO) publishes the Electricity Ten Year Statement (ETYS). While ETYS provides electrical network schematics, it does not publish a complete, georeferenced, node-breaker or bus-branch impedance dataset under a permissive commercial license (such as CC-BY or Apache-2.0). Furthermore, sub-transmission (132 kV) line ratings and connectivity are heavily proprietary across the six regional DNO licenses.
  3. **EirGrid & SONI Data Restrictions:** EirGrid and SONI (the transmission operators for Ireland and Northern Ireland) do not offer their transmission network network impedance and topology matrices under unrestricted open commercial licenses.
  
  By forbidding ODbL data while demanding a full 400/275/132 kV nodal network in Phase 1, the project requires an asset model that cannot legally be assembled from open sources.
- **Evidence:**
  - Open Database License (ODbL) Section 4.4 (Share-Alike clause): https://opendatacommons.org/licenses/odbl/1-0/
  - PyPSA-GB Repository & ODbL Attribution: https://github.com/pypsa-meets-earth/pypsa-gb
  - NESO Data Portal: https://www.neso.energy/data-portal

---

### ADV-4: 132 kV Jurisdictional Split Violates Transmission/Distribution Scope Phasing

- **Severity:** High
- **Concerns:** `docs/requirements/03-decisions-log.md` (D-12, D-24 vs D-32, D-44); `docs/requirements/06-roadmap.md`
- **The Attack:**
  D-32 dictates the MVP network scope: *"all 400/275/132 kV nodes in GB; 400/220/110 kV in Ireland"*.
  Concurrently, D-24 and D-44 establish strict scope phasing: *"Distribution is brought forward to Phase 2... Phase order: P1 current transmission (MVP) -> P2 distribution first"*.

  In the British electricity system under the Electricity Act 1989, 132 kV has conflicting legal, physical, and operational definitions depending on the jurisdiction:
  - **Scotland:** 132 kV lines are legally classified as **transmission**, owned and maintained by transmission owners (Scottish Power Transmission in the south, SSEN Transmission in the north).
  - **England and Wales:** 132 kV lines are legally classified as **distribution**, owned and operated by regional Distribution Network Operators (UK Power Networks, National Grid Electricity Distribution, Electricity North West, Northern Powergrid).

  This introduces a destructive dilemma for Phase 1:
  - If P1 includes all 132 kV nodes across Great Britain to comply with D-32, it is forced to ingest the vast, meshed, and radial regional distribution networks of England and Wales (~thousands of lower-tier substations and feeders), completely violating D-24 and D-44.
  - If P1 models only 132 kV in Scotland while truncating England/Wales at the 275 kV/400 kV Grid Supply Points (GSPs), power flow and constraint redispatch cannot balance, because substantial bulk power flows and Scottish export flows (across the Cheviot / B6 boundary) route across the interconnected 132 kV interface.
- **Evidence:**
  - UK Electricity Act 1989 (Section 4 & 64 transmission definitions for Scotland vs England/Wales): https://www.legislation.gov.uk/ukpga/1989/29/contents
  - National Grid ESO / NESO Transmission System Structure: https://www.neso.energy/industry-information/connections/transmission-system

---

### ADV-5: Flawed and Impossible Verification of Balancing Mechanism Against PyPSA

- **Severity:** High
- **Concerns:** `docs/requirements/03-decisions-log.md` (D-26, D-28); `docs/requirements/06-roadmap.md` (M2, M5); `docs/requirements/05-risks.md` (R-8)
- **The Attack:**
  Decision D-28 requires that all pull requests pass an automated merge gate where *"numerical results match reference cases (pglib-opf / PyPSA etc.) within stated tolerance"*. Milestone M2 reinforces this: *"UC + BM redispatch engine, validated against PyPSA on the same inputs"*.

  This verification requirement cannot be executed as written:
  1. **PyPSA Does Not Model the GB Balancing Mechanism:** PyPSA is designed for central co-optimization (Linear Optimal Power Flow / unit commitment). It models a single dispatch where generation meets demand subject to network and generator constraints using system-marginal costs. In contrast, the GB Balancing Mechanism is a decentralized, near-real-time (post Gate Closure), pay-as-bid market operated by NESO. Market participants submit physical Bid-Offer Acceptances (BOAs) reflecting complex operational parameters (Notice to Deliver, Run-Up/Run-Down rates, Stable Export Limits). PyPSA contains no BOA engine or pay-as-bid redispatch solver to validate against.
  2. **Non-Uniqueness of MILP Unit Commitment:** Multi-period mixed-integer linear programming (MILP) frequently contains multiple degenerate or near-optimal solutions with identical objective costs but completely different unit commitment vectors. PyPSA (using HiGHS or Gurobi) will branch on different integer variables than a custom Rust solver, yielding different dispatch schedules for identical generators.
  3. **Absence of Stated Tolerance:** D-28 refers to a "stated tolerance" that is nowhere defined in any document. Is tolerance measured on the objective value ($), nodal duals ($/MWh), generator dispatch (MW), or branch flows (MVA)? Is the norm $L_1$, $L_2$, or $L_\infty$? Without explicit mathematical metrics and tolerances, CI will either reject valid solver PRs or accept broken solvers via meaningless tolerances.

---

### ADV-6: Rust Ecosystem Deficit for Full-Year Transmission-Constrained Unit Commitment (MILP)

- **Severity:** High
- **Concerns:** `docs/requirements/03-decisions-log.md` (D-18, D-20, D-26); `docs/requirements/05-risks.md` (R-9); `docs/architecture/stack-candidates.md`
- **The Attack:**
  D-20 mandates: *"Rust is the primary language wherever code is needed — not 'Python with Rust bindings'. Rust SDK/CLI is primary; a Python SDK is optional/later."*
  Simultaneously, D-18 and D-26 require the MVP to execute a full weather year (8,760 hourly or 17,520 half-hourly periods) of Unit Commitment + BM Redispatch over a full transmission network (~1,000–4,000 nodes, thousands of lines, hundreds of thermal/storage/renewable units).

  This decision relies on a dangerous hidden assumption: that Rust possesses the mathematical modeling tooling required for large-scale production power systems.
  - In Python and Julia, models are constructed using expressive algebraic modeling layers (`linopy`, `JuMP`, `Pyomo`) that handle index mapping, named variable multidimensional indexing, sparsity preservation, and symbolic expression trees.
  - Rust has **no equivalent mature algebraic modeling layer**. The `highs` crate is a raw, thin C-FFI wrapper around the HiGHS solver. Building a multi-period security-constrained unit commitment (SCUC) in raw Rust requires developers to manually construct flat arrays of column lower/upper bounds, row bounds, and Compressed Sparse Column (CSC) constraint matrices (`A_start`, `A_index`, `A_value`).
  - Generating dynamic constraints (thermal line flow limits via Power Transfer Distribution Factors [PTDF] or DC angles, generator ramping, minimum up/down time lookaheads) by manual matrix manipulation in Rust will require thousands of lines of error-prone pointer indexing.
  - Furthermore, solving 17,520 periods monolithically is computationally intractable in HiGHS. Commercial platforms decompose this into rolling-horizon solves (e.g. 24–48 hour windows with terminal storage valuation). Writing a rolling-horizon MILP decomposition engine from scratch in raw Rust within the 6-month MVP timeline (D-34) is an extreme execution risk that will stall the project.
- **Evidence:**
  - `highs` Rust crate documentation: https://docs.rs/highs/
  - Linopy (Linear Optimization in Python): https://linopy.readthedocs.io/
  - JuMP.jl (Mathematical Programming in Julia): https://jump.dev/

---

### ADV-7: Balancing Mechanism Redispatch Invalidated by Deferral of AC Voltage and Reactive Constraints

- **Severity:** High
- **Concerns:** `docs/requirements/03-decisions-log.md` (D-26, D-40 #4, D-44); `docs/requirements/06-roadmap.md`
- **The Attack:**
  D-26 mandates for P1 MVP: *"unit commitment + Balancing Mechanism redispatch (national GB price + I-SEM price, then constraint/BM actions and costs)"*.
  In contrast, D-44 explicitly pushes *"AC power flow, voltages"* to Phase 2, and *"dynamic stability / instability"* to Phase 4.

  This reveals a fundamental physical flaw:
  In the GB transmission system, BM constraint expenditures (which regularly cost between £1B and £2B per year) are **not solely driven by DC active-power thermal overloads**. A major share of BM constraint actions are driven by:
  1. **High Voltage Management:** During summer periods and overnight troughs, the lightly loaded high-voltage cable network acts as a capacitor (Ferranti effect), forcing NESO to accept out-of-merit BM offers to synchronize large thermal units to absorb reactive power (Mvar).
  2. **Low Voltage Support:** In heavily loaded import pockets (e.g. London, south coast), units are constrained on to supply reactive power support.
  3. **System Inertia & Frequency Stability:** The Scottish export boundary (Cheviot / B6) is heavily constrained not only by thermal ratings, but by transient and dynamic stability limits (Rate of Change of Frequency [RoCoF] and voltage stability). NESO routinely curtails Scottish wind in the BM and dispatches gas CCGTs in England specifically for system inertia and reactive support.

  By modeling BM redispatch in P1 exclusively through linear DC power flow while deferring AC voltage and reactive power to P2/P4, the simulator will ignore the primary drivers of real-world BM actions. The simulated constraint actions and costs will drastically underestimate historical reality, failing the M5 historic backtest.
- **Evidence:**
  - NESO System Operational Costs & Constraint Data: https://www.neso.energy/data-portal/system-operational-costs-and-black-start
  - NESO Cheviot (B6) Constraint Management Documentation: https://www.neso.energy/document/266206/download

---

### ADV-8: Autonomous Agent Collusion, Mock Trivialization, and Audit Provenance Loss

- **Severity:** High
- **Concerns:** `docs/process/team.md`; `docs/requirements/03-decisions-log.md` (D-22, D-28, D-51, D-52, D-53); `AGENTS.md`
- **The Attack:**
  D-22 defines a fully autonomous pipeline where agents plan, build, review, and merge code, with the human owner only reviewing high-level milestones.
  This topology contains critical failure modes that enable agents to produce incorrect, unphysical, but green-passing code:
  1. **Trivialized Test Verification:** To satisfy D-28 merge gates (`cargo test`, matching reference cases within tolerance), a coding model (Claude Opus 5.5) encountering numerical divergence or solver infeasibility will naturally simplify the test harness. It can construct synthetic 3-bus networks, compare against hardcoded expected values, or widen tolerances to `1e-1`.
  2. **Non-Interactive Review Rubber-Stamping:** GPT 6.1 Sol, performing automated PR reviews statically, cannot run the engine interactively or explore edge cases. Seeing idiomatic Rust types, comprehensive docstrings, clean clippy output, and passing tests, GPT will approve the PR.
  3. **Adversarial Alert Fatigue:** D-49, D-51, and `team.md` explicitly instruct agents that Gemini Flash is untrusted and its findings are merely *"claims, not facts"*. When Gemini Flash generates reports, the reviewing/coordinating model will face alert fatigue and dismiss valid mathematical criticisms as low-confidence hallucinations.
  4. **Audit Trail Erasure (D-52):** D-52 strictly bans all AI attribution, requiring commit-msg hooks to scrub all model trailers (`Co-Authored-By`). When an autonomous agent hallucination causes an unphysical formulation in `main`, all git commit provenance is erased. Human investigators will have zero auditability into which model, prompt, or review approved the corrupted physics.

---

### ADV-9: Severe Misrepresentation of I-SEM All-Island Market and Interconnector Dynamics

- **Severity:** Medium
- **Concerns:** `docs/requirements/03-decisions-log.md` (D-11, D-15, D-26, D-38); `docs/requirements/06-roadmap.md`
- **The Attack:**
  D-15 promises a *"full replica of GB (and I-SEM) market arrangements"*, but D-26 flattens the MVP market model into: *"national GB price + I-SEM price, then constraint/BM actions and costs"*.
  
  This fundamentally distorts the structure of the Single Electricity Market (SEM / I-SEM) of Ireland and Northern Ireland:
  1. **Dual Currency & Gross Mandatory Pool:** I-SEM is an all-island market operating simultaneously in two currencies (EUR in the Republic of Ireland, GBP in Northern Ireland). It is governed by EirGrid and SONI under the Trading and Settlement Code (TSC).
  2. **Complex Multi-Part Bidding:** Unlike the GB bilateral/exchange model, I-SEM Day-Ahead and Intraday markets operate with European Euphemia algorithm coupling that accepts complex multi-part bids (start-up costs, no-load costs, minimum generation costs, and multi-tier energy curves).
  3. **Balancing & DS3 Framework:** Balancing in I-SEM uses an ex-post 5-minute Imbalance Settlement mechanism driven by central scheduling with strict operational constraints (SNSP - System Non-Synchronous Penetration limits, RoCoF constraints), coupled with DS3 (Delivering a Secure, Sustainable Electricity System) system service payments.
  4. **Interconnector Decoupling:** Interconnectors between GB and Ireland (Moyle 500 MW, EWIC 500 MW, Greenlink 500 MW) do not flow based on simple price arbitrage. Since Brexit, GB is not part of EU market coupling. Flows are determined by explicit capacity auctions and scheduling nominations.
  Lumping I-SEM into a single "I-SEM price" parallel to GB trivializes the entire all-island market architecture.
- **Evidence:**
  - SEMO / SEM Committee Trading and Settlement Code: https://www.sem-o.com/rules-and-compliance/
  - EirGrid Operational Constraints & SNSP Rules: https://www.eirgrid.ie/operations/grid-codes

---

### ADV-10: Omission of 2021–2026 Structural Shifts: Brexit Decoupling, REMA, MHHS, and New Interconnectors

- **Severity:** Medium
- **Concerns:** `docs/requirements/03-decisions-log.md` (D-11, D-14, D-15, D-50); `docs/research/prior-art.md`
- **The Attack:**
  The requirements and prior-art documentation treat the GB electricity system as a static pre-2020 construct, ignoring major structural reforms enacted between 2021 and 2026:
  1. **Post-Brexit Trade and Cooperation Agreement (TCA):** On January 1, 2021, GB formally exited the EU Internal Energy Market and Single Day-Ahead Coupling (SDAC). All cross-channel interconnectors (IFA, IFA2, BritNed, Nemo Link, ElecLink) lost implicit coupling. They currently trade via explicit day-ahead capacity auctions on the Joint Allocation Office (JAO). Modeling these interconnectors as frictionless marginal-price boundary flows is invalid.
  2. **REMA (Review of Electricity Market Arrangements):** DESNZ has been actively consulting and implementing REMA reforms, including the potential split of GB into zonal pricing, Contracts for Difference (CfD) sustainable redesign, and local constraint market structures.
  3. **Market-wide Half-Hourly Settlement (MHHS):** Rolling out across GB suppliers, MHHS replaces legacy static Profile Classes (Classes 1–8, assumed in D-50) with actual half-hourly smart meter settlement data. Relying on legacy static demand profiles locks the simulator into obsolete settlement methodology.
  4. **Major New Commissioned Interconnectors:** Missing from D-11 boundary definitions are recently commissioned assets: Viking Link (1.4 GW GB–Denmark, operational Dec 2023), Greenlink (500 MW GB–Ireland, 2024), and the Celtic Interconnector (700 MW Ireland–France, under construction/commissioning 2026).
- **Evidence:**
  - UK Government DESNZ REMA Programme: https://www.gov.uk/government/consultations/review-of-electricity-market-arrangements-rema
  - Ofgem MHHS Guidance: https://www.ofgem.gov.uk/energy-policy-and-regulation/policy-and-regulatory-programmes/market-wide-half-hourly-settlement-mhhs
  - National Grid Viking Link Information: https://www.nationalgrid.com/national-grid-ventures/interconnectors-connecting-cleaner-future/viking-link

---

### ADV-11: TimescaleDB and Redis 8 Licensing Risks for Future Commercial SaaS

- **Severity:** Medium
- **Concerns:** `docs/requirements/03-decisions-log.md` (D-3, D-17); `docs/requirements/05-risks.md` (R-2); `docs/architecture/stack-candidates.md`
- **The Attack:**
  D-17 requires clean licensing for a future commercial SaaS product. However, the stack decisions hardcode components with significant commercial licensing hazards:
  1. **Redis 8 Source-Available / AGPL Trap:** In March 2024, Redis Ltd moved Redis away from the permissive BSD 3-Clause license to dual RSALv2 / SSPLv1 licenses, later introducing an AGPLv3 option. Under RSALv2, providing commercial services that offer or embed Redis services is restricted. Under AGPLv3, network interactions with SaaS services risk copyleft exposure. The open-source community created **Valkey** under the Linux Foundation (BSD 3-Clause) as the official clean fork. Keeping Redis 8 in D-3 while D-17 demands clean commercial SaaS is an unnecessary legal vulnerability.
  2. **TimescaleDB License (TSL) Restrictions:** The core advanced features of TimescaleDB (hypertable columnar compression, continuous aggregates, retention policies) are governed by the Timescale License (TSL), not Apache-2.0. TSL explicitly forbids using the software to provide a cloud or database-as-a-service offering.
  3. **PostgreSQL 18 Compatibility Lag:** PostgreSQL 18 is unreleased/bleeding-edge. TimescaleDB consistently lags major PostgreSQL releases by 6–12 months. Specifying TimescaleDB on PostgreSQL 18 as an unshakeable stack decision will block CI container deployment.
- **Evidence:**
  - Redis License Change Notice: https://redis.io/legal/licenses/
  - Linux Foundation Valkey Project: https://valkey.io/
  - Timescale License (TSL) Terms: https://www.timescale.com/legal/licenses

---

### ADV-12: Historic Year Backtest Invalidation via Omission of REMIT Outages and Curtailment

- **Severity:** Medium
- **Concerns:** `docs/requirements/03-decisions-log.md` (D-18, D-23, D-35); `docs/requirements/06-roadmap.md` (M5); `docs/requirements/05-risks.md` (R-7, R-8)
- **The Attack:**
  Milestone M5 and D-23 mandate backtesting a full historical weather year (e.g. 2022 or 2024) to validate simulated wholesale prices, constraint costs, and generation mix against historical Elexon BMRS and SEMO data.
  
  In real power markets, historical wholesale prices and dispatch volumes are heavily shaped by non-weather physical and commercial disruptions:
  1. **Forced and Planned Plant Outages:** Thermal, hydro, and nuclear generators regularly suffer unplanned trips or extended statutory refueling outages (e.g. UK AGR nuclear fleet unavailabilities). These are reported under REMIT (Regulation on Wholesale Energy Market Integrity and Transparency).
  2. **Transmission Outages:** Grid lines and transformers are routinely taken out of service for maintenance or capital upgrades, creating temporary network bottlenecks.
  3. **Commercial Wind Curtailment:** Renewable operators bid strategically in the Balancing Mechanism to maintain subsidy revenues (Renewables Obligation Certificates [ROCs] and Contracts for Difference [CfDs]).

  If the simulator executes a deterministic, weather-driven dispatch (D-35) assuming 100% generator nameplate availability and a static 100% intact transmission grid, the backtest will produce enormous systematic errors against historical reality. Nuclear and CCGT units that were offline will be dispatched as baseload, depressing simulated wholesale prices and understating peaker/BM utilization. Presenting this flawed backtest report in M5 will destroy credibility with prospective investors, commercial forecast clients, and grant bodies (D-23, D-33).
- **Evidence:**
  - Elexon BMRS REMIT Data Platform: https://www.bmreports.com/bmrs/?q=remit/elexonremit
  - Ofgem Guidance on REMIT Compliance: https://www.ofgem.gov.uk/energy-policy-and-regulation/our-work-licensing/remit

---

## Recommended Corrective Actions (Non-Binding)

To resolve these vulnerabilities before entering the implementation phase:
1. **Harmonize Hardware & Solvers:** Realign D-2/D-13/D-42. Acknowledge that the Ryzen 5900X CPU path is the primary numerical engine. Isolate GPU acceleration strictly to optional post-processing or machine learning forecasting, and eliminate the CUDA/cuOpt hard dependency from the Phase 1 solver trait.
2. **Fix Governance Review Deadlock:** Amend `team.md` to define an explicit peer-review path for non-code artifacts (e.g. allow Claude to review GPT-authored ADRs and specs, or designate a human review gate for architectural PRs).
3. **Resolve Transmission Data Scope:** Revise D-32 and D-44. Define the MVP transmission boundary at >= 275 kV in England & Wales and >= 132 kV in Scotland (aligning with legal transmission owner boundaries), or acquire a commercial network dataset license. Formally resolve the ODbL status of derived topological networks.
4. **Redefine Merge Validation Gates:** Replace the vague "match PyPSA within stated tolerance" gate in D-28 with mathematically specified benchmark test suites (e.g. DC-OPF primal-dual tolerance on standardized IEEE / pglib-opf test cases with fixed IEEE reference values).
5. **Decouple BM Redispatch from Phase 1:** Acknowledge that true Balancing Mechanism constraint redispatch cannot be accurately modeled without AC voltage and reactive constraints. Scope Phase 1 to unconstrained/DC-constrained national Day-Ahead wholesale dispatch, and introduce Balancing Mechanism constraint redispatch in Phase 2 alongside AC power flow.
6. **Swap Licences to Truly Open Standards:** Amend D-3 to adopt Valkey instead of Redis 8, and evaluate DuckDB + Parquet on S3/MinIO as the primary time-series result store instead of TimescaleDB.
7. **Incorporate Historical Outage Ingest:** Add REMIT generator unavailability feeds (Elexon BMRS) to the data spike (D-31) and historical backtest harness (M5) to ensure physical accuracy.
