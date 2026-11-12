# 06 — Phased Roadmap

_Source: D-24 (phasing), D-18 (MVP), D-25 (forecast horizons), D-26 (MVP market fidelity). Phases 2–4 are DRAFT until the owner confirms._

The data model must represent everything in the end state (D-6, D-12) from day one; phases
control which features are *implemented*.

| Phase | Scope | Status |
|---|---|---|
| **P1 — MVP** | GB + all-island Ireland **transmission**; weather-year dispatch with **unit commitment + Balancing Mechanism redispatch** (D-26); national GB price, I-SEM price, constraint costs, unserved energy; map UI; **backtest vs one historic year** (prices, generation mix) | Confirmed |
| **P2** | **Distribution + DER** (EVs, heat pumps, rooftop solar) (D-24); AC power flow + N-1; batch scenario sweeps; **short-term (day-ahead/intraday) forecasting** with live data feeds (D-25) | DRAFT |
| **P3** | Zonal / nodal alternative market designs; full I-SEM detail; capacity expansion; **seasonal forecasting** (D-25) | DRAFT |
| **P4** | Dynamic stability; SaaS multi-tenancy hardening | DRAFT |
