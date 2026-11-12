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
| D-40 | Q-A-5 | Core questions the simulator must answer (owner's words, summarised): **(1)** model the *current* GB + Ireland grid accurately enough to be trusted; **(2)** predict behaviour under **zonal pricing**; **(3)** compare **2030 deployment configurations**, e.g. grid-scale storage (pumped hydro + battery) vs distributed household supply (home batteries + rooftop solar); **(4)** **voltage excursions and instability** at different levels of each; **(5)** **emulate human activity and workloads** (behaviour-driven demand) | 2026-11-12 | Owner, round D |
| D-41 | Q-A-8 | First forecast customers: **battery operators, traders, suppliers/large energy users** | 2026-11-12 | Owner, round D |
| D-42 | Q-H-7 | GPU hardware today: GTX 1080 Ti 11 GB + Quadro RTX 4000 8 GB; plan to consolidate to **one RTX 5060 Ti 16 GB**. CPU/RAM/distro TBD | 2026-11-12 | Owner, round D |
| D-43 | Q-H-7 (access) | Agent access to GPU machine (delegated): **both** — SSH for interactive dev/benchmarking, and a **self-hosted GitHub Actions runner** for GPU CI gates (D-28). Merge gates run via CI only | 2026-11-12 | Owner delegated, round D |
| D-44 | Review #2, R-13 (refines D-24) | Priority principle: **"answer the present first."** Simulate the current grid very accurately, then scale out from that validated model to zonal pricing and future scenarios. Phase order: **P1** current transmission (MVP) → **P2 distribution first** (distribution networks, home batteries/solar, behaviour-driven demand, AC power flow and voltages, still for the present day) → **P3** zonal pricing and 2030 deployment scenarios → **P4** dynamic stability, SaaS hardening | 2026-11-12 | Owner, round E |
| D-45 | Q-K-1 | Build team: **Claude, OpenAI/Codex and Google Gemini** models (role split to be proposed) | 2026-11-12 | Owner, round E |
| D-46 | Q-M-3 | Explicit non-goal: **native mobile apps** (web only). No other exclusions declared | 2026-11-12 | Owner, round E |
| D-47 | Q-G-4 | Look and feel: **pro analytics**, dense, dark-mode, Bloomberg/Grafana-style dashboards for analysts and traders | 2026-11-12 | Owner, round E |
| D-48 | Q-H-7 | Linux machine: **AMD Ryzen 9 5900X (12 cores / 24 threads), 128 GB RAM**; planned upgrade to Intel Core Ultra 7 270K Plus (pending RAM availability). Distro TBD | 2026-11-12 | Owner, round F |
| D-49 | Q-K-1 (refines D-45) | Model roles: **Claude Opus 5.5 (medium effort)** does most coding; **GPT 6.1 Sol** does everything else (architecture, reviews, specs, planning); **Gemini Flash 3.8 (high)** does only low-complexity, low-scrutiny "donkey work" (e.g. data retrieval, collation, formatting) and its output is always checked by another model. See `docs/process/team.md` | 2026-11-12 | Owner, round F |
| D-50 | Q-B-9 | Behaviour-driven demand (P2): **statistical demand profiles** by household/business type, scaled and varied (no agent-based simulation of individuals) | 2026-11-12 | Owner, round F |
| D-51 | Q-K-1 (refines D-49) | **Use GPT 6.1 Sol as much as possible for everything except code generation** (research, verification of Gemini output, reviews, specs, ADRs, reports). Claude Opus 5.5 is reserved for writing code. Gemini Flash remains for checked donkey work | 2026-11-12 | Owner |
| D-52 | Q-K-5 | **No AI watermarking, ever.** No AI attribution in code, comments, commit messages, PR titles/bodies, release notes or file headers (no `Co-Authored-By` AI trailers, no "Generated with …" footers). Rationale: AI cannot take responsibility for defects; accountability rests with the human owner. Model names may appear only in process docs (e.g. `docs/process/team.md`, review reports) | 2026-11-12 | Owner |
| D-53 | Q-K-4 (extends D-28) | **Commit and push to GitHub frequently** (small commits, push at least after every meaningful step; never leave work only local). **Adversarial reviews by Gemini**: in addition to the GPT review, every substantive PR and document gets a Gemini adversarial review that tries to break it (contradictions, wrong facts, edge cases, failure modes). Gemini findings are triaged by GPT/coordinator before action (D-49) | 2026-11-12 | Owner |
