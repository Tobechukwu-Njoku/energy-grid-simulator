# STATUS

_Last updated: 2026-11-12_

**Phase:** Requirements engineering. Decisions D-1 … D-39 (D-5 superseded by D-23).
Fable review (`reviews/claude-fable-5-2-2026-11-12.md`) processed in rounds A–C.
**Code allowed?** No, except the **data spike** (D-31) in `spikes/`.
**Repo:** private, https://github.com/Tobechukwu-Njoku/energy-grid-simulator

## Snapshot of what's decided

Research + market-forecasting simulator (short-term and seasonal, D-25) of **GB + all-island
Ireland** with interconnectors. Targets: consultancy/research, forecast customers, grants (D-33).
End state: transmission + distribution, all physics time-scales, full market replica with
pluggable designs (zonal/nodal). Delivered in phases (`06-roadmap.md`, D-24); distribution is P2.
**MVP (6+ months, quality first):** full transmission network, weather-year dispatch with
**unit commitment + Balancing Mechanism redispatch**, map, prices, historic backtest.
**Rust everywhere code is needed**; TypeScript web UI. PostgreSQL 18 + Redis; owner's
Linux + NVIDIA machine plus Mac (CPU-only must work). Pluggable solvers. Future commercial SaaS,
so **no share-alike data** (D-30). Fully autonomous team with strict merge gates (D-28).

## Still open (owner)

- Q-A-5 example questions the simulator must answer (most valuable input)
- Q-A-8 first forecast customer segment and what they buy
- Q-H-7 GPU machine specs and agent access
- Q-K-1 which models/providers build it
- Q-M-3 explicit non-goals
- Q-G-4 look and feel references
- Confirm draft phases P2–P4 in `06-roadmap.md`

## Next steps (agents may do now)

1. Data spike (D-31): permissively licensed GB + Ireland full transmission network + one weather
   year; record licence per dataset; answers R-12.
2. Add GB + Ireland data sources to `prior-art.md` with licences (review #7); verify licence
   flags for cuDSS, SuiteSparse/KLU, TimescaleDB on PG18 (review #11).
3. Draft MVP requirements baseline in `04-requirements.md` (review #12), marking open items `ASSUMPTION:`.
4. ADRs 0001–0006 (review §4 step 8).
5. Flip to "code allowed" once 3 and ADRs 0001–0003 are accepted.
