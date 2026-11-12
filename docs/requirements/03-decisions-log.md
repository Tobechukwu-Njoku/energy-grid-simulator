# 03 — Decisions Log (append-only)

Format: `D-<n> | <question IDs> | <decision> | <date> | <source>`

| ID | Questions | Decision | Date | Source |
|---|---|---|---|---|
| D-1 | — | Build a grid simulation platform from the ground up, reusing OSS where it helps | 2026-11-12 | Owner, kickoff |
| D-2 | Q-H-1 | Hardware/RAM/power cost is not a constraint; optimize for performance | 2026-11-12 | Owner, kickoff |
| D-3 | Q-I-1 | PostgreSQL 18 and Redis are part of the stack | 2026-11-12 | Owner, kickoff |
| D-4 | Q-K-* | Multi-model development; all planning artifacts live as Markdown in repo | 2026-11-12 | Owner, kickoff |
| D-5 | Q-A-1 | ~~Primary purpose: research / what-if exploration~~ SUPERSEDED by D-23 | 2026-11-12 | Owner, Round 1 |
| D-6 | Q-B-1 | Physics scope: **all time-scales** — long-term capacity expansion, dispatch/unit commitment, power flow + contingencies, dynamic stability | 2026-11-12 | Owner, Round 1 |
| D-7 | Q-C-1 | First geography: **UK** (primary) | 2026-11-12 | Owner, Round 1 |
| D-8 | Q-D-4, Q-G-1 | Interaction: **all** — interactive map sandbox, batch scenario sweeps, and API/code access | 2026-11-12 | Owner, Round 1 |
| D-9 | Q-I-2, Q-I-4 | Language preference: **Rust** (scope to be clarified, Q-I-7) | 2026-11-12 | Owner, Round 1 |
| D-10 | Q-I-7 | Rust scope: **Rust simulation core + backend/API services**; TypeScript web UI; thin Python SDK via PyO3; existing OSS engines (PyPSA, PowerModels, pandapower) used as **validators**, not runtime dependencies | 2026-11-12 | Owner, Round 2 |
| D-11 | Q-C-7 | Geography: **GB + Northern Ireland + Republic of Ireland (all-island SEM)**, with GB interconnectors (Norway, France, Belgium, NL, Denmark) as boundaries | 2026-11-12 | Owner, Round 2 |
| D-12 | Q-B-2 | Network levels: **transmission + distribution** from the start | 2026-11-12 | Owner, Round 2 |
| D-13 | Q-H-2 | Hardware target: **Linux + NVIDIA GPU(s)** (GPU solvers/CUDA in scope) | 2026-11-12 | Owner, Round 2 |
| D-14 | Q-D-1 | v1 scenario priority: **weather extremes** (Dunkelflaute, storms, cold/heat, multi-year weather). Other scenario classes later | 2026-11-12 | Owner, Round 3 |
| D-15 | Q-B-4, Q-B-5 | Markets: **full replica of GB (and I-SEM) market arrangements**, with the ability to simulate **alternative designs such as zonal (and by implication nodal) pricing** — market design must be pluggable | 2026-11-12 | Owner, Round 3 |
| D-16 | Q-I-3 | Solvers: **pluggable** — open-source default (HiGHS, Ipopt, cuOpt), commercial (Gurobi/Xpress) when licensed | 2026-11-12 | Owner, Round 3 |
| D-17 | Q-L-1, Q-A-2 | Distribution: **future commercial SaaS** — design for auth, multi-tenancy, and licence-clean dependencies from day one | 2026-11-12 | Owner, Round 3 |
| D-18 | Q-M-2 | MVP: **weather-year dispatch** — GB + all-island Ireland transmission, dispatch over a full weather year (e.g. replay a historic Dunkelflaute), shown on a map with prices and unserved energy | 2026-11-12 | Owner, Round 4 |
| D-19 | Q-D-2 | Scenario authoring (delegated to architect): **versioned TOML scenario files are canonical**; web UI and SDK read/write the same schema; natural-language (LLM) authoring is a later phase that emits scenario files | 2026-11-12 | Owner delegated, Round 4 |
| D-20 | Q-I-4 (amends D-10) | **Rust is the primary language wherever code is needed** — not "Python with Rust bindings". Rust SDK/CLI is primary; a Python SDK is optional/later. Python/Julia allowed only for offline validation against reference tools and the TS frontend | 2026-11-12 | Owner, Round 4 |
| D-21 | Q-E-2 | Time resolution: **configurable per run down to 1-minute**; 30-min (GB settlement period) default for market sims | 2026-11-12 | Owner, Round 4 |
| D-22 | Q-K-2, Q-K-3 | Team process: **fully autonomous** — agents plan, build, review and merge; owner checks milestones. Implies strict automated quality gates (Q-K-4) | 2026-11-12 | Owner, Round 4 |
| D-23 | Q-A-1 (supersedes D-5) | Primary purpose: **research / what-if exploration AND market / price forecasting**, with **potential future funding** in view (results must be credible and demonstrable to funders/investors) | 2026-11-12 | Owner, re-answer |
| D-24 | Review #2 (D-6, D-12) | Phased delivery: D-6 and D-12 describe the **end state**; the data model must allow for all of it from day one, but features ship in phases. **Distribution (incl. DER: EVs, heat pumps, rooftop solar) is brought forward to Phase 2.** See `06-roadmap.md` | 2026-11-12 | Owner, review round A |
| D-25 | Q-A-6 | Forecast horizons: **short-term (day-ahead / intraday)** and **seasonal (weeks to a year)**. Long-term 10–30-year outlooks are not a forecasting target (capacity-expansion what-ifs remain in scope per D-6) | 2026-11-12 | Owner, review round A |
| D-26 | Q-B-8 | MVP market fidelity: **unit commitment + Balancing Mechanism redispatch** (national GB price + I-SEM price, then constraint/BM actions and costs) | 2026-11-12 | Owner, review round A |
| D-27 | Review #1 | Repository: first commit + **private GitHub repo**, pushed | 2026-11-12 | Owner, review round A |
| D-28 | Q-K-4 | Merge gates (all required): `cargo fmt --check`, `cargo clippy -D warnings`, tests, `cargo-deny` licence/advisory check, numerical results match reference cases (pglib-opf / PyPSA etc.) within stated tolerance, no benchmark regression, review by a model other than the author | 2026-11-12 | Owner, review round B |
| D-29 | Q-H-3 | Owner **owns a Linux + NVIDIA machine** (specs TBD, Q-H-7) for GPU build/test/benchmark; Mac remains a dev machine, so the core must also build and run CPU-only | 2026-11-12 | Owner, review round B |
| D-30 | Q-L-5 | Data licensing: **avoid share-alike data** (e.g. ODbL / OSM-derived network data). Only commercially permissive datasets; build our own network model where needed. Licence recorded per dataset | 2026-11-12 | Owner, review round B |
| D-31 | Review #5, #7 | Exception to "no code yet": **data spike only** (assemble GB + Ireland network and one weather year from permissively licensed sources; throwaway code in `spikes/`; write-up informs ADRs). No engine/optimisation spike; engine code waits for baseline | 2026-11-12 | Owner, review round B |
| D-32 | Q-C-9 | MVP network: **full transmission** (all 400/275/132 kV nodes in GB; 400/220/110 kV in Ireland), built from permissively licensed sources (D-30). Feasibility to be confirmed by the data spike (D-31) | 2026-11-12 | Owner, review round C |
| D-33 | Q-A-7 | Funding/customer targets: **consultancy/research, forecast customers, grants** (not VC-first). Implies documented methods, published validation, reliable API | 2026-11-12 | Owner, review round C |
| D-34 | Q-M-1 | MVP timeline: **6+ months; quality and validation first** | 2026-11-12 | Owner, review round C |
| D-35 | Q-B-6 | Uncertainty: deterministic MVP; Monte Carlo across historic weather years with LOLE/EUE in P2 | 2026-11-12 | Owner accepted default, round C |
| D-36 | Q-H-5 | Retention: summaries kept forever; full-resolution results kept for pinned runs, others deleted after 90 days | 2026-11-12 | Owner accepted default, round C |
| D-37 | Q-C-8 | Distribution data (P2): DNO open-data portals where licence allows (D-30), synthetic representative feeders elsewhere | 2026-11-12 | Owner accepted default, round C |
| D-38 | Q-C-10 | Currency: each market in native currency (GBP / EUR); daily FX rates for cross-border flows | 2026-11-12 | Owner accepted default, round C |
| D-39 | Q-L-6 | SaaS tenant isolation model: deferred to P4 | 2026-11-12 | Owner accepted default, round C |
