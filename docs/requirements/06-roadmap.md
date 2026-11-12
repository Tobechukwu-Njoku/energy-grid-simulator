# 06 — Phased Roadmap

_Source: D-24 (phasing), D-18 (MVP), D-25 (forecast horizons), D-26 (MVP market fidelity). Phases 2–4 are DRAFT until the owner confirms._

The data model must represent everything in the end state (D-6, D-12) from day one; phases
control which features are *implemented*.

| Phase | Scope | Status |
|---|---|---|
| **P1 — MVP** (6+ months, D-34) | GB + all-island Ireland **full transmission network** (D-32); weather-year dispatch with **unit commitment + Balancing Mechanism redispatch** (D-26); national GB price, I-SEM price, constraint costs, unserved energy; map UI; **backtest vs one historic year** (prices, generation mix) | Confirmed |
| **P2** | **Distribution + DER** (EVs, heat pumps, rooftop solar) (D-24); AC power flow + N-1; batch scenario sweeps; **short-term (day-ahead/intraday) forecasting** with live data feeds (D-25) | DRAFT |
| **P3** | Zonal / nodal alternative market designs; full I-SEM detail; capacity expansion; **seasonal forecasting** (D-25) | DRAFT |
| **P4** | Dynamic stability; SaaS multi-tenancy hardening | DRAFT |

## MVP milestones (draft, from Fable review)

- **M0** workspace, CI, merge gates (D-28), licence gate
- **M0.5** data spike (D-31): network + one weather year, licence per dataset
- **M1** data ingest and network model
- **M2** UC + BM redispatch engine, validated against PyPSA on the same inputs
- **M3** API and results storage
- **M4** map UI
- **M5** historic-year backtest report (prices, generation mix, constraint costs)
