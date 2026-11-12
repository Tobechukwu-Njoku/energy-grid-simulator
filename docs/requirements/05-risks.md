# 05 — Risks & Flags

| ID | Risk | Source | Mitigation / question |
|---|---|---|---|
| R-1 | Scope is very large (all physics time-scales × T+D × full market replica × all-island). Risk of never shipping. | D-6, D-12, D-15 | Strict phased roadmap; define MVP slice (Q-M-2) |
| R-2 | Redis 8 licensing (RSALv2 / SSPLv1 / AGPLv3) vs future SaaS. Running unmodified Redis server is generally fine under AGPL, but needs legal confirmation. | D-3, D-17 | Keep Redis behind an interface; Valkey (BSD) is a drop-in fallback |
| R-3 | GPL/AGPL dependencies (e.g. ANDES GPL-3) cannot be embedded in a closed SaaS. | D-17 | Licence allow-list in CI (cargo-deny); GPL tools only as offline validators |
| R-4 | Commercial solver licences for SaaS (Gurobi cloud/ISV licensing) are costly. | D-16, D-17 | Open-source default path must be production-capable |
| R-5 | GB/Ireland distribution network data is largely not public at LV. | D-12 | Q-C-8 |
| R-6 | Full market replica (BM, CfD, CM, BSUoS, I-SEM) rules change frequently (e.g. REMA reforms). | D-15 | Market rules as versioned, pluggable modules |
| R-7 | Validation of "research-grade" credibility across many domains. | D-23 | Q-L-4; validate against historical events (e.g. 9 Aug 2019 outage, Jan 2025 tight margins) |
| R-8 | Market forecasting (D-23) needs a measurable track record; funders will ask "how accurate is it?" | D-23 | Backtest harness vs historic GB/I-SEM prices (Elexon, SEMO) as a first-class requirement; publish accuracy metrics |
| R-9 | Rust lacks a mature algebraic modelling layer (JuMP/linopy equivalent) and a well-trodden AC-OPF + Ipopt path; crate maturity (`highs`, Clarabel, `faer`, GPU FFI) unverified. MVP UC+BM (D-26) is a large MILP, which raises the stakes | D-20, D-26 | Time-boxed Rust optimisation spike before engine ADR (review #5) |
| R-10 | Data licensing for commercial SaaS: OSM is ODbL (share-alike on derived databases); Elexon, NESO, DNO, SEMO, weather data each have own terms | D-17 | Q-L-5; record licence per dataset in prior-art.md |
| R-11 | MVP grew: UC + BM redispatch (D-26) needs a GB network model with constraint boundaries and BM data, making P1 substantially larger than plain dispatch | D-26, D-18 | Keep P1 transmission-only; BM modelled as constraint redispatch, not full bid/offer replay, unless owner says otherwise |
