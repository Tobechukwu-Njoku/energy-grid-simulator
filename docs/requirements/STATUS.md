# STATUS

_Last updated: 2026-11-12_

**Phase:** Requirements engineering — Rounds 1–4 answered (D-5 … D-23; D-5 superseded by D-23). Round 5 pending.
**Code allowed?** No.

## Snapshot of what's decided

Research + market-forecasting simulator (future funding in view) of **GB + all-island Ireland** (with interconnectors), **transmission +
distribution**, **all physics time-scales** (planning → dispatch → power flow/N-1 → dynamics),
**full market replica** with pluggable alternative designs (zonal/nodal). **Rust everywhere code
is needed** (core, backend, SDK/CLI), TypeScript web UI. PostgreSQL 18 + Redis, Linux + NVIDIA
GPUs, pluggable OSS/commercial solvers. Future **commercial SaaS**. Interactive sandbox + batch
sweeps + API. MVP = **weather-year dispatch** with weather extremes. Built by **fully autonomous**
team of models. See `03-decisions-log.md`; risks in `05-risks.md`.

## Progress by section (see 02-questionnaire.md)

| Section | State |
|---|---|
| A Purpose & users | partial (A-1, A-2 done; A-3..A-5 open) |
| B Physics fidelity | partial (B-1, B-2, B-4, B-5 done) |
| C Geography & data | partial (C-1, C-7 done) |
| D Scenarios | partial (D-1, D-2, D-4 done) |
| E Time horizon | partial (E-2 done) |
| F Outputs | open |
| G UI | partial (G-1 done) |
| H Performance/deploy | partial (H-1, H-2 done) |
| I Stack | mostly done |
| J Integration | open |
| K Multi-model process | partial (K-2, K-3 done; K-1, K-4 open) |
| L Non-functional | partial (L-1 done) |
| M Timeline | partial (M-2 done) |

## Next steps

1. Round 5 (asked, awaiting owner): Q-A-5 (example questions), Q-A-6 (forecast horizon), Q-A-7 (funding target),
   Q-C-8 (distribution data), Q-K-1 (which models), Q-K-4 (quality gates), Q-H-3 (deployment),
   Q-H-5 (retention), Q-B-6 (uncertainty), Q-M-1 (timeline), Q-G-4 (look & feel).
2. Derive FR/NFR in `04-requirements.md`.
3. ADRs: engine architecture, market-design plugin model, storage, distributed execution.
4. Break work into tasks for the autonomous build team.
