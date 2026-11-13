# 06 — Phased Roadmap

_Source: D-24, D-44 (phasing), D-18 (MVP), D-25 (forecast horizons), D-26 (MVP market fidelity). Phase order confirmed by D-44._

The data model must represent everything in the end state (D-6, D-12) from day one; phases
control which features are *implemented*.

| Phase | Scope | Status |
|---|---|---|
| **P1 — MVP** (6+ months, D-34) | **Today's** GB + all-island Ireland **full transmission network** (D-32); weather-year dispatch with **unit commitment + Balancing Mechanism redispatch** (D-26); national GB price, I-SEM price, constraint costs, unserved energy; pro-analytics map UI (D-47); **backtest vs historic years** (prices, generation mix) | Confirmed |
| **P2 — Distribution (present day)** | Distribution networks + DER (home batteries, rooftop solar, EVs, heat pumps) (D-24, D-44); **behaviour-driven demand** (D-40 #5); **AC power flow, voltages** (D-40 #4); short-term forecasting with live feeds (D-25); Monte Carlo weather years (D-35); batch sweeps | Confirmed order (D-44) |
| **P3 — Market designs & futures** | **Zonal pricing** (and nodal) (D-40 #2); **2030 deployment scenarios**, grid-scale vs distributed storage (D-40 #3); capacity expansion; full I-SEM detail; seasonal forecasting (D-25) | Confirmed order (D-44) |
| **P4 — Dynamics & SaaS** | Dynamic stability / instability (D-40 #4); SaaS multi-tenancy hardening (D-39) | Confirmed order (D-44) |

## MVP milestones (draft, from Fable review)

- **M0** workspace, CI, merge gates (D-28), licence gate
- **M0.5** data spike (D-31): network + one weather year, licence per dataset
- **M1** data ingest and network model
- **M2** UC + BM redispatch engine, validated against PyPSA on the same inputs
- **M3** API and results storage
- **M4** map UI
- **M5** historic-year backtest report (prices, generation mix, constraint costs)

## Proposed acceptance decomposition (PR #4 revision, 2026-11-13)

ASSUMPTION: The following is engineering sequencing within the milestones above,
not new phase/scope decisions. See `04-requirements.md` for individually numbered
acceptance criteria, numeric proposals and the complete FR/NFR-to-milestone table.
D-55 adds published security/stability proxies; D-59 fixes cost-based redispatch;
D-60 includes every GB 132 kV area. D-56 permits tagged research-only data, but not
committed restricted raw files, and does not waive pre-commercial clearance.

| Milestone | Staged evidence and requirement ownership |
|---|---|
| M0 | Documentation bootstrap and later CPU CI profile, independent/adversarial review, attribution rejection and dependency policy (NFR-3/8/10, FR-18 contract); NFR-12 policy review only, no operational security/penetration exit tests. Baseline gate still controls code. |
| M0.5 | D-31 only: per-operator/voltage coverage, provenance and missing-parameter handoff (FR-2/3/19); input rights tagged separately from commercial clearance. |
| M1 | Dated topology/assets/rules, underlying/embedded demand, losses, calendar and shared scenario identity (FR-1–5/19/21 inputs, NFR-1/2 foundations). Provision the separate D-65 target runner and pass NFR-12.AC1/AC2 configuration/access-denial tests before the first heavy B1 job; no heavy builds, databases or benchmarks on the coding container. |
| M2a | Long-horizon energy LP pre-pass, then two-market UC MILP with reserves and commitment proxies; rolling-state and cancellation tests (FR-6/9/11/18; NFR-2/5/7/9). |
| M2b | Full-network fixed-commitment DC cost-based redispatch, secured boundaries/proxies, constraint screening, signed affine loss segments and bounded UC repair/recovery with explicit non-convergence outcomes (FR-7/8/21, NFR-4). |
| M2c | National fixed-UC pricing LP and independent UC/network reference cases; full-network CPU benchmarks (FR-10/13, NFR-2/4/5). PyPSA agreement is not BM-outturn validation. |
| M3 | SDK/CLI/API identity, solve workers, persisted results, retention and input archival (FR-4/11/15/16, NFR-1/6/9/10/12). |
| M4 | Dense map/analytics, comparison, keyboard/accessibility and responsiveness evidence (FR-14/20, NFR-6). |
| M5 | Frozen historical holdout: perfect-information price/mix/BM constraint-only cost AND separate up/down volume/curtailment/two-sided adequacy gates and observed stress report; as-of backtest report-only; final full-inventory/throughput evidence (FR-12/17, NFR-11, FR-2, NFR-5). |

M2a–c are parts of M2, not independently shippable reduced MVPs. M5 cannot be passed
with incompatible or unavailable BM/curtailment observations labelled as success.

ASSUMPTION (Q-N-4): The 2024 I-SEM price gate remains BLOCKED pending separately
gathered, verified Irish prices. Select/freeze a common year only after checking
both markets' inputs; this sequencing does not waive an accuracy gate. Target-runner
specifications are pending D-65 owner details (Q-N-7), not inferred from D-48.
