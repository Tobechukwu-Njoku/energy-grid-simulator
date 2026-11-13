# 02 — Requirements Questionnaire

_Last updated: 2026-11-13_

Each question has a stable ID, why it matters, options, and a **recommended default** (★) that
agents may assume *only* if the owner explicitly says "use defaults" for that section.

Status markers: `[ ]` open · `[x]` answered (see decisions log) · `[~]` partially answered.
Sections are ordered by how much they shape everything downstream: **A–E are critical-path.**

For PR #4's P1 baseline, section N is the active owner queue: **Q-N-4 and,
conditionally, Q-N-3 only**. Earlier unanswered items remain a historical/P2+ backlog,
not additional immediate owner requests. Engineering defaults below are proposals;
they are not marked answered without an owner decision or explicit baseline adoption.

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
- [x] **Q-A-7 Funding target (from D-23):** grants (Innovate UK, Ofgem SIF, UKRI) · investors ·
  selling forecasts to customers? Decides validation/documentation/polish needed for v1.
- [x] **Q-A-8 Forecast customers (review #14, D-33):** which segment first: traders, battery
  operators, generators, suppliers, developers? What do they buy: price curves, spreads, risk?
- [x] **Q-A-5 Example questions.** Please list 3–5 concrete questions you want the
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
- [x] **Q-B-6 Uncertainty.** Deterministic runs only, or stochastic (Monte Carlo, ensemble
  weather years, probabilistic reliability metrics like LOLE/EUE)?
- [x] **Q-B-8 MVP market fidelity (review #8):** merit order · UC · UC + BM redispatch → D-26.
- [x] **Q-B-9 Behavioural demand detail (from D-40):** → D-50 (statistical profiles).
- [ ] **Q-B-7 Protection & failures.** Simulate equipment failures, cascading outages,
  protection relays, restoration/black-start?

## C. Geography, network data & real-world data

- [x] **Q-C-1 Which grid?** Real region(s) (which? e.g. ERCOT, CAISO, PJM, whole US,
  ENTSO-E Europe, a specific country) · synthetic test grids (IEEE, TAMU ACTIVSg) ·
  user-drawn grids · global.
- [x] **Q-C-8 Distribution data (from D-12):** GB/Ireland distribution networks are mostly
  not public at LV. Options: DNO open-data portals (UKPN, SSEN, NGED, NPg, ENWL, SPEN) where
  available · synthetic representative feeders (e.g. LV network templates) · full-detail for
  one pilot DNO area and aggregated elsewhere?
- [x] **Q-C-9 MVP network granularity (review #8):** → D-32 (full transmission).
- [x] **Q-C-10 Currency (review #14):** → D-38.
- [x] **Q-C-11 Network scope / connecting voltages (spike PR #5):** → D-11/D-32
  (GB + Northern Ireland + Republic of Ireland; NI 275 kV transmission included).
  D-60 confirms all GB 132 kV. Scope is answered; the named snapshot date is
  tracked separately in Q-C-13.
- [ ] **Q-C-12 Weather candidate approval (spike PR #5):** use 0.25° hourly ERA5
  over [61 N, 12 W, 48 N, 10 E], eight fields including precipitation for hydro
  and direct radiation for PV, or a different footprint/resolution/field set?
  **★ Recommended default:** ASSUMPTION: validate this candidate against actual assets, units
  and conversion models before retrieving monthly 2024 jobs. The roughly 1.33 GB
  float32 budget is an assumption, not a measured download or owner decision.
- [ ] **Q-C-13 Network snapshot date (spike PR #5):** which named operational
  date defines the present-day model, separately from the 2024 backtest?
  **★ Recommended default:** ASSUMPTION: reconcile each source's study year and
  effective changes to one owner-selected date before claiming an operational
  snapshot. No specific date is assumed and no network scope is reopened.
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
  ★ Web app + Rust SDK/CLI over a shared API (D-20).
- [ ] **Q-G-2 Visualization:** 2D map (MapLibre/deck.gl), 3D globe, single-line diagrams,
  time-series charts, animated playback of a day?
- [ ] **Q-G-3 Grid editing:** draw/edit network topology in the UI?
- [x] **Q-G-4 Design/aesthetic references** you like (apps, dashboards, games)?

## H. Performance, scale & deployment

- [x] **Q-H-1 Resource budget:** unconstrained → D-2.
- [x] **Q-H-2 Hardware actually available.** Your machine(s) now (CPU cores, RAM, GPU —
  e.g. Apple Silicon Mac? NVIDIA GPUs?), and is cloud spend acceptable?
  _Why:_ "unconstrained" still needs a target; GPU solvers (cuOpt, cuDSS, CUDA) need NVIDIA.
- [x] **Q-H-3 Deployment target:** single beefy workstation (Docker Compose) · on-prem
  cluster / Kubernetes · cloud (AWS/GCP/Azure) · hybrid burst-to-cloud for sweeps?
- [x] **Q-H-7 GPU machine specs (from D-29):** GPU model(s) and VRAM, CPU cores, RAM, OS/distro, reachable remotely by agents?
- [ ] **Q-H-4 Concurrency:** how many simultaneous simulations / users?
- [x] **Q-H-5 Data volume:** retain all time-series results of all runs forever, or prune?
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

- [x] **Q-K-1 Team topology:** which models/providers will build (Claude, Codex/GPT,
  Gemini…)? Any role split you have in mind (architect, backend, solver, frontend, QA)?
- [x] **Q-K-2 Coordination:** GitHub repo with issues/PRs per task? Monorepo? Branch-per-agent
  with worktrees? Who reviews/merges — you, or a designated reviewer model?
- [x] **Q-K-3 Autonomy:** how much can agents decide without you (e.g. library choices
  within an accepted ADR)?
- [x] **Q-K-4 Quality gates:** required test coverage, CI, benchmarks, validation against
  reference cases (e.g. pglib-opf, IEEE cases, PyPSA results) before merge?

## L. Non-functional: security, licensing, validation

- [x] **Q-L-1 Licence of this project:** private, open source (MIT/Apache-2.0/AGPL)?
  _Why:_ constrains which dependencies (GPL/AGPL) we can embed.
- [ ] **Q-L-2 Auth:** none (local) · single-user login · multi-user RBAC · SSO?
- [ ] **Q-L-3 Sensitive data:** will any data be CEII/critical-infrastructure restricted?
- [x] **Q-L-5 Commercial data licensing (review #6):** share-alike acceptable? → D-30 (no).
- [x] **Q-L-6 SaaS tenant isolation (review #14):** → D-39 (deferred to P4).
- [x] **Q-L-7 Price data cost and product labelling:** → D-57 (free historical
  sources, paid only if minuscule; MID and imbalance are not day-ahead prices).
- [ ] **Q-L-4 Validation:** how will we know the simulator is *right*? Benchmark vs
  historical events, vs reference tools, vs published studies?

## M. Timeline & priorities

- [x] **Q-M-1 Timeline:** first usable version when? Any hard deadline?
- [x] **Q-M-2 MVP slice:** if you could only have one thing working first, what is it?
- [x] **Q-M-3 Non-goals:** anything explicitly out of scope?
- [ ] **Q-M-4 Data-spike fallback and effort assumptions (PR #5 §7):** detailed
  assembly A; reduction B after A; synthetic C; two-zone D; regional pilot E;
  boundary-zone F? **★ Recommended default:** ASSUMPTION: proceed with research assembly A
  under D-56/D-60 and evaluate F as a D-55 proxy prototype alongside it. B–F do
  not silently replace required full-network coverage. The person-week ranges
  (including F's candidate 20–40 zones) are assumptions, exclude procurement,
  external access delays, engine/UI work and independent D-28 numerical acceptance,
  and need owner review before they become schedule commitments.

## N. P1 baseline dispositions and engineering defaults

Stable aliases preserve the draft's former OI-1–OI-12 references. `proposed default`
means awaiting baseline adoption, not an invented owner answer. All defaults below
are **ASSUMPTION:** unless an existing decision is explicitly cited.

### Owner questions (two, one conditional)

- [ ] **Q-N-3 Network/data gaps (alias OI-3, conditional).** If the verified D-31
  inventory cannot satisfy D-32/D-60 or backtest observations, acquire missing data,
  accept specific equivalents, or explicitly change release scope?
  ★ Keep full all-132-kV scope; acquire/derive missing parameters with provenance and
  accept equivalents individually. Reduced networks remain diagnostic fixtures, not
  a reduced MVP. Escalate only evidenced gaps, not speculative scope questions. Known gap:
  2024 I-SEM prices are not yet gathered/verified (2018–2020 availability does not
  qualify 2024); a separate gathering task is running. The 2024 Irish price gate is
  BLOCKED. Verify both markets' price/constraint/volume coverage before selecting
  and freezing the common holdout year; an alternate year requires Q-N-4 disposition.
- [ ] **Q-N-4 Validation bar (alias OI-4; refines Q-L-4/Q-A-4).** Accept or amend the
  proposed numeric holdout/coverage gates in `04-requirements.md`?
  ★ Adopt the table as gates for **perfect-information reconstruction**; as-of
  forecast backtest is report-only in P1 and cannot claim forecast validation. Accept
  or amend that scope together with skill ≥ 0.15, absolute AND relative mix errors,
  constraint-only BM costs (NESO constraint breakdown), separate up/down volumes,
  curtailment and two-sided adequacy/over-commitment gates. Freeze year/products and
  commit the hashed holdout manifest before calibration; missing comparable evidence
  (including 2024 Irish prices) blocks a validated-MVP claim. FR-17 screens three
  consecutive days with 72-hour mean wind CF < 10% and each daily mean demand above
  the year's 75th percentile of daily means. Confirm candidate events from data; if
  none qualifies, accept/amend the threshold or permit a labelled synthetic stress
  fixture alongside the closest observed episode, never claim it is historical.

### Proposed engineering defaults (not additional owner questions)

| ID / legacy alias | Status | Recommended default and alternatives |
|---|---|---|
| Q-N-1 / OI-1 | proposed default; D-55 fixes physics | DC plus secured boundaries and published commitment/stability proxies, reserves, directional interconnector losses and long-horizon storage targets; pure N-0 diagnostic only. Full AC remains P2. |
| Q-N-2 / OI-2 | core settled D-59; detail proposed | Cost-based redispatch, never historic bid replay; GB Market Index and I-SEM DAM reference products if available (D-57); fixed historical external flows in reconstruction; explicit proxy/deferred market-rule inventory. Plant cost estimates include dated renewable lost RO/CfD subsidy and storage opportunity cost; compare with NESO constraint-only outturn and reconcile commercial transfers. Alternative boundary policies must be scenario-labelled. |
| Q-N-5 / OI-5 | proposed default | 48 h look-ahead / 24 h advance, 0.1% UC relative gap or 1 native-objective-currency absolute gap, annual energy pre-pass; NFR-5 CPU budgets. Bound UC repair at three re-solves, signed loss-segment trials at two extra LPs and recovery at one network-aware MILP (300 s), within a 900 s total window budget; report non-convergence and fail/partial if fallback is invalid. Tighten after measurement on the D-65 target, not an optimisation spike. Direct sparse production builder; no competing production-builder promotion contest. |
| Q-N-6 / OI-6 | proposed default; relates Q-D-5 | Manifest-level numerical reproducibility with NFR-2 tolerances, not identical MILP schedules/bitwise cross-platform results. Exact reference solves use matched tolerances and relative gap ≤1e-6; rerun fixtures use deterministic work/node limits or a completed gap certificate, not wall-clock-limited incumbents. |
| Q-N-7 / OI-7 | target details pending (D-65); GPU inventory non-blocking for CPU P1 | HiGHS CPU, self-hosted runner, heavy builds, databases and benchmarks run on the separate D-65 target system, NEVER the coding container. Owner supplies target hardware/OS/access before runner provisioning/qualification. ASSUMPTION: D-48's 5900X/128 GB is provisional sizing only, not confirmed target identity. Inventory actual GPU/licences before enabling optional adapters; no P1 cuOpt implementation or assumed speedup. |
| Q-N-8 / OI-8 | proposed default | Review all dependency/native/service terms; start with MIT, Apache-2.0, BSD-2/3-Clause, ISC, Zlib, Unicode-3.0; MPL-2.0 only after file-level obligation review. Isolate offline validators; block unreviewed exceptions. Retain Redis/PostgreSQL 18; no unapproved Valkey or TimescaleDB adoption. D-56 dataset research tags are separate from software licence approval. |
| Q-N-9 / OI-9 | proposed default | Archive legally retainable pinned inputs as well as pinned results; manifests/summaries indefinite and unpinned full results 90 days (D-36). If rights prohibit retention, report rerun limitations and retain manifests/scripts only. |
| Q-N-10 / OI-10 | proposed default | Submit/monitor/cancel/inspect, sequential CLI batches and limited two-run comparison; NFR-6 latency/viewports. In-place solver editing and distributed sweeps are later features. |
| Q-N-11 / OI-11 | core settled D-54/D-58/D-61; detail proposed | ADR 0001 bootstrap profiles and shared-identity review evidence; proposed >5% wall / >10% RSS significant-regression gate with five paired repetitions against BOTH main and the frozen last milestone/release baseline, on the D-65 target. M0 approves security policy; M1 executes runner isolation/access-denial tests before heavy B1 activation. Public required checks after D-58 scans/merges; do not change bot summaries. |
| Q-N-12 / OI-12 | proposed default; relates Q-L-2/Q-L-3 | Localhost/private VPN, no public P1 endpoints. Resolve auth/sensitive-data questions before external exposure; no P4 tenancy work pulled forward. |

D-60 resolves the 132 kV boundary; D-58 resolves the public-repository direction;
D-61 resolves the bot-summary policy. Do not reopen these as baseline questions.
