# 02 — Requirements Questionnaire

_Last updated: 2026-11-12_

Each question has a stable ID, why it matters, options, and a **recommended default** (★) that
agents may assume *only* if the owner explicitly says "use defaults" for that section.

Status markers: `[ ]` open · `[x]` answered (see decisions log) · `[~]` partially answered.
Sections are ordered by how much they shape everything downstream: **A–E are critical-path.**

---

## A. Purpose, users & success

- [x] **Q-A-1 Primary purpose.** What is the tool *for*?
  Options: (a) research/what-if exploration · (b) utility/TSO-grade planning studies ·
  (c) market/price forecasting & trading · (d) education/visualization · (e) game/sandbox ·
  (f) a personal/portfolio flagship project.
  _Why:_ decides fidelity, validation burden, and UI emphasis.
- [x] **Q-A-2 Users.** Just you? A small team? Public SaaS with many tenants?
  ★ Single owner + small team, self-hosted.
- [ ] **Q-A-3 Expertise of users.** Power-systems engineers (expect PSS/E-like detail) or
  non-experts (expect sliders and maps)? Both?
- [ ] **Q-A-4 Success criteria.** What would make v1 a success? (e.g. "simulate ERCOT on a
  hot August week with 30 % more EVs and see where blackouts occur, in < 1 min").
- [x] **Q-A-6 Forecast horizon (from D-23):** day-ahead / intraday price forecasts ·
  seasonal · long-term 10–30-year market outlooks · several? Each implies different modelling.
- [ ] **Q-A-7 Funding target (from D-23):** grants (Innovate UK, Ofgem SIF, UKRI) · investors ·
  selling forecasts to customers? Decides validation/documentation/polish needed for v1.
- [ ] **Q-A-5 Example questions.** Please list 3–5 concrete questions you want the
  simulator to answer. *(This is the single most useful input.)*

## B. Simulation domain & physics fidelity

- [x] **Q-B-1 Which time-scale(s) of physics?** (multi-select)
  1. Capacity expansion / long-term planning (years–decades, hourly or representative days)
  2. Production cost / unit commitment & economic dispatch (hours–year, 5-min to hourly)
  3. Steady-state power flow — DC or AC (snapshot or time series)
  4. Contingency analysis (N-1, N-k), security-constrained OPF
  5. Dynamic / transient stability (ms–s; frequency, inertia, rotor angles)
  6. Electromagnetic transients (µs) — very specialized
  ★ 2 + 3 (AC) + 4 as core; 1 as phase 2; 5 later.
- [x] **Q-B-2 Network levels.** Transmission only? Distribution (feeders, households, DER)?
  Both (T&D co-simulation)?
- [ ] **Q-B-3 Sector coupling.** Electricity only, or also gas, heat, hydrogen, transport
  (EV charging), water?
- [x] **Q-B-4 Markets & economics.** Model wholesale markets (day-ahead, real-time, ancillary
  services), prices (LMP/zonal), costs, emissions?
- [x] **Q-B-5 Agent behavior.** Do market participants / consumers act as autonomous agents
  (agent-based modelling, bidding strategies, possibly AI agents), or is the grid optimized
  centrally?
- [ ] **Q-B-6 Uncertainty.** Deterministic runs only, or stochastic (Monte Carlo, ensemble
  weather years, probabilistic reliability metrics like LOLE/EUE)?
- [x] **Q-B-8 MVP market fidelity (review #8):** merit order · UC · UC + BM redispatch → D-26.
- [ ] **Q-B-7 Protection & failures.** Simulate equipment failures, cascading outages,
  protection relays, restoration/black-start?

## C. Geography, network data & real-world data

- [x] **Q-C-1 Which grid?** Real region(s) (which? e.g. ERCOT, CAISO, PJM, whole US,
  ENTSO-E Europe, a specific country) · synthetic test grids (IEEE, TAMU ACTIVSg) ·
  user-drawn grids · global.
- [ ] **Q-C-8 Distribution data (from D-12):** GB/Ireland distribution networks are mostly
  not public at LV. Options: DNO open-data portals (UKPN, SSEN, NGED, NPg, ENWL, SPEN) where
  available · synthetic representative feeders (e.g. LV network templates) · full-detail for
  one pilot DNO area and aggregated elsewhere?
- [ ] **Q-C-2 Network size target.** Approx. buses/nodes: 100s · 10k · 100k+ (continental AC)?
- [ ] **Q-C-3 Data sources.** Open data only (OSM, ENTSO-E, EIA, NREL, ERA5), or will you
  have licensed/proprietary data (e.g. FERC 715 / CEII, PLEXOS/PSS/E models)?
- [ ] **Q-C-4 Live data.** Ingest real-time/near-real-time feeds (grid operator APIs, weather)
  for "digital twin" / forecast mode, or historical/static data only?
- [ ] **Q-C-5 Weather.** Weather-driven renewables & demand (wind/solar/temperature from
  reanalysis like ERA5 or climate projections)? ★ Yes.
- [ ] **Q-C-6 Geospatial.** Is a map-based view with real coordinates required? ★ Yes (PostGIS).

## D. Scenarios ("different circumstances")

- [x] **Q-D-1 Which circumstances matter?** (multi-select) Extreme weather (heat waves,
  cold snaps like Winter Storm Uri, Dunkelflaute), demand growth (EVs, heat pumps, data
  centers/AI load), renewable/storage buildout, plant retirements, fuel price shocks,
  equipment outages/contingencies, cyber/physical attacks, policy (carbon price, mandates),
  interconnector changes, wildfires/PSPS, demand response, microgrids/islanding.
- [x] **Q-D-2 Scenario authoring.** How are scenarios defined? GUI forms · YAML/JSON files
  under version control · Python/DSL scripting · natural language via an LLM → all of these?
- [ ] **Q-D-3 Sweeps.** Batch thousands of scenario variants (parameter sweeps, Monte Carlo)
  and compare them? ★ Yes — this is where a high-end stack pays off.
- [x] **Q-D-4 Interactivity.** Real-time "what if I trip this line now" interactive
  sandbox while a simulation runs, or submit-and-wait batch jobs, or both?
- [ ] **Q-D-5 Reproducibility.** Must every run be bit-for-bit reproducible (versioned
  inputs, seeds, solver versions)? ★ Yes.

## E. Time horizon & resolution

- [ ] **Q-E-1 Horizon** per run: a snapshot · a day · a year (8,760 h) · multi-decade?
- [x] **Q-E-2 Resolution:** hourly · 15-min · 5-min · 1-sec (dynamics) · mixed?
- [ ] **Q-E-3 Speed target:** e.g. "1 year hourly, 10k-bus DC-OPF in < N minutes";
  "faster than real time for an interactive day"?

## F. Outputs, analytics & reporting

- [ ] **Q-F-1 Key outputs:** flows, line loading, voltages, prices, generation mix,
  emissions, costs, unserved energy, reliability indices, curtailment, frequency?
- [ ] **Q-F-2 Comparison:** side-by-side scenario diffing, dashboards, statistical summaries?
- [ ] **Q-F-3 Export:** CSV/Parquet, PDF reports, notebooks (Jupyter), BI tools (Grafana,
  Superset), API?
- [ ] **Q-F-4 AI assistance:** LLM features (explain results, generate scenarios from text,
  agentic analysis)?

## G. User interface

- [x] **Q-G-1 Primary interface:** web app · desktop app · CLI/Python SDK · Jupyter · all?
  ★ Web app + Python SDK + CLI over a shared API.
- [ ] **Q-G-2 Visualization:** 2D map (MapLibre/deck.gl), 3D globe, single-line diagrams,
  time-series charts, animated playback of a day?
- [ ] **Q-G-3 Grid editing:** draw/edit network topology in the UI?
- [ ] **Q-G-4 Design/aesthetic references** you like (apps, dashboards, games)?

## H. Performance, scale & deployment

- [x] **Q-H-1 Resource budget:** unconstrained → D-2.
- [x] **Q-H-2 Hardware actually available.** Your machine(s) now (CPU cores, RAM, GPU —
  e.g. Apple Silicon Mac? NVIDIA GPUs?), and is cloud spend acceptable?
  _Why:_ "unconstrained" still needs a target; GPU solvers (cuOpt, cuDSS, CUDA) need NVIDIA.
- [ ] **Q-H-3 Deployment target:** single beefy workstation (Docker Compose) · on-prem
  cluster / Kubernetes · cloud (AWS/GCP/Azure) · hybrid burst-to-cloud for sweeps?
- [ ] **Q-H-4 Concurrency:** how many simultaneous simulations / users?
- [ ] **Q-H-5 Data volume:** retain all time-series results of all runs forever, or prune?
  (A 100k-bus × 8,760 h × 1,000 scenarios run is ~TBs.)
- [ ] **Q-H-6 Availability:** is downtime acceptable (research tool) or does it need HA?

## I. Technology stack

- [~] **Q-I-1 Mandated components:** PostgreSQL 18, Redis → D-3. Others you want or
  refuse? (See `docs/architecture/stack-candidates.md`.)
- [x] **Q-I-2 Core numerical language:** Python (largest ecosystem: PyPSA, pandapower) ·
  Julia (fastest OPF ecosystem: PowerModels, Sienna) · Rust/C++ (custom engine, max speed) ·
  hybrid (Rust/Julia core + Python bindings)?
- [x] **Q-I-3 Optimization solvers:** open-source only (HiGHS, Ipopt, SCIP) or commercial
  allowed (Gurobi, CPLEX, Xpress, Mosek — 5–100× faster on big MILPs; academic licences free)?
- [x] **Q-I-4 Language preferences / aversions** for backend, frontend, scripting?
- [ ] **Q-I-5 Redis vs Valkey:** Redis 8 (AGPLv3/RSAL/SSPL licensing) or Valkey (BSD fork)?
  Matters only if you ever distribute/sell.
- [x] **Q-I-7 Rust scope (from D-9):** Rust for the simulation core only · core + backend
  services · everything incl. frontend (Leptos/Dioxus/WASM)? Keep Python/Julia OSS engines
  (PyPSA, PowerModels) as references/validators or wrap them?
- [x] **Q-C-7 UK boundary (from D-7):** Great Britain only (NESO area; England, Scotland,
  Wales) or also Northern Ireland (which is in the all-island SEM with Ireland)? Model
  interconnectors to Norway/France/Belgium/NL/Denmark/Ireland as fixed flows or full
  neighbouring markets?
- [ ] **Q-I-6 Time-series storage:** TimescaleDB extension on Postgres 18, or separate
  columnar store (ClickHouse / Parquet + DuckDB) for result data?

## J. Integration & extensibility

- [ ] **Q-J-1 Import/export formats:** MATPOWER, PSS/E RAW, CIM/CGMES, pandapower,
  PyPSA netCDF, OpenDSS, GridLAB-D?
- [ ] **Q-J-2 Plugins:** can users add custom models (new device types, control logic,
  market rules) via a plugin API?
- [ ] **Q-J-3 Co-simulation:** integrate external simulators via HELICS / FMI?
- [ ] **Q-J-4 Public API:** REST, GraphQL, gRPC, WebSocket streaming?

## K. Multi-model development process

- [ ] **Q-K-1 Team topology:** which models/providers will build (Claude, Codex/GPT,
  Gemini…)? Any role split you have in mind (architect, backend, solver, frontend, QA)?
- [x] **Q-K-2 Coordination:** GitHub repo with issues/PRs per task? Monorepo? Branch-per-agent
  with worktrees? Who reviews/merges — you, or a designated reviewer model?
- [x] **Q-K-3 Autonomy:** how much can agents decide without you (e.g. library choices
  within an accepted ADR)?
- [ ] **Q-K-4 Quality gates:** required test coverage, CI, benchmarks, validation against
  reference cases (e.g. pglib-opf, IEEE cases, PyPSA results) before merge?

## L. Non-functional: security, licensing, validation

- [x] **Q-L-1 Licence of this project:** private, open source (MIT/Apache-2.0/AGPL)?
  _Why:_ constrains which dependencies (GPL/AGPL) we can embed.
- [ ] **Q-L-2 Auth:** none (local) · single-user login · multi-user RBAC · SSO?
- [ ] **Q-L-3 Sensitive data:** will any data be CEII/critical-infrastructure restricted?
- [ ] **Q-L-4 Validation:** how will we know the simulator is *right*? Benchmark vs
  historical events, vs reference tools, vs published studies?

## M. Timeline & priorities

- [ ] **Q-M-1 Timeline:** first usable version when? Any hard deadline?
- [x] **Q-M-2 MVP slice:** if you could only have one thing working first, what is it?
- [ ] **Q-M-3 Non-goals:** anything explicitly out of scope?
