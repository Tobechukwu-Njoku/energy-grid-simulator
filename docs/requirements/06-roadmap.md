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
