# Project review and next-steps plan

_Reviewer: claude-fable-5-2 · 2026-11-12 · Scope: every file in the repo as of this date (AGENTS.md, CLAUDE.md, docs/requirements/*, docs/architecture/stack-candidates.md, docs/research/prior-art.md)._

This follows `REVIEW-BRIEF.md`. Nothing outside this file was edited, and the decisions log was not touched.

## 1. Where the project stands

- **What it is:** a GB + all-island Ireland electricity grid and market simulator, Rust-first, PostgreSQL 18 + Redis, aimed at research, price forecasting and eventually a commercial SaaS.
- **Phase:** requirements engineering. 23 decisions logged across four rounds. Round 5 (11 questions) is asked and unanswered.
- **What exists:** 10 Markdown files. No code, no ADRs (`docs/architecture/adr/` is empty), no formal requirements (`04-requirements.md` is a template), no earlier reviews.
- **Git:** the repo is initialised but `main` has **no commits** and there is **no remote**. Every file is untracked.

The requirements work is well organised: stable IDs, an append-only decisions log, a clear rule against inventing decisions, and an honest risk list. The gaps below are mostly about scope control and things that must be settled before autonomous agents write code.

## 2. Findings

Severity: **H** blocks or endangers the build, **M** should be fixed before baselining, **L** tidy-up.

| # | Sev | Concerns | Finding | Suggested change |
|---|---|---|---|---|
| 1 | H | Repo | Nothing is committed and there is no GitHub remote. All work so far can be lost, and D-22 (agents plan, review and merge) cannot work without a remote, branches and PRs. | Make the first commit, create the GitHub repo, push. Do this before anything else. |
| 2 | H | D-6, D-12 vs D-18 | "All physics time-scales" and "transmission + distribution from the start" conflict with an MVP that is transmission-only dispatch. Agents reading D-12 literally will build distribution into the MVP. R-1 names the risk but there is no roadmap that resolves it. | Add a phased roadmap (suggested in section 4) and ask the owner to confirm that D-6 and D-12 describe the end state, with "from the start" meaning the data model allows for it, not that the MVP implements it. |
| 3 | H | D-23, D-18, R-8 | Price forecasting was added to the purpose after the MVP was defined. The MVP has no forecasting or backtest deliverable, yet R-8 says funders will ask for accuracy. A fundamentals simulator and a price forecaster are different products with different validation. | Add a backtest to the MVP acceptance criteria: simulate one historic year and report error against actual GB day-ahead prices and generation mix. Q-A-6 (horizon) must be answered before requirements are baselined. |
| 4 | H | D-22, Q-K-4 | Fully autonomous merging is decided, but the quality gates that make it safe are still an open question. | Treat Q-K-4 as a hard blocker for "code allowed". Proposed default in section 4, step 4. |
| 5 | H | Stack (D-20) | The biggest technical risk is not listed: Rust has no mature equivalent of JuMP/linopy/Pyomo for building large optimisation models, and no well-trodden path for AC-OPF with Ipopt. The project would be building its own modelling layer. Crate maturity (`highs`, Clarabel, `faer`, GPU FFI) is unverified. | Add as R-9. Run a time-boxed spike before the engine ADR: build a one-year GB dispatch LP in Rust against HiGHS and measure build and solve time. Needs a narrow "spikes allowed" exception to the no-code rule. |
| 6 | H | D-17, data | Data licensing for commercial use is not examined. OpenStreetMap is ODbL (share-alike on derived databases), and Elexon, NESO, DNO, SEMO and weather data each have their own terms. A SaaS built on a derived network dataset inherits these. | Add R-10 and a question (Q-L-5): which datasets may be used commercially, and is a share-alike network dataset acceptable? Record licence per dataset in prior-art.md. |
| 7 | M | prior-art.md | Data sources are US-centric (EIA, ERCOT, CAISO, PJM, ACTIVSg, NSRDB) although geography has been GB + Ireland since D-7/D-11. The sources the MVP actually needs are missing: Elexon BMRS/Insights, NESO Data Portal, ETYS network data, FES scenarios, DUKES, REPD, SEMO/SEMOpx, EirGrid/SONI data, DNO open-data portals, and PyPSA-GB as a reference model. | Add a GB + Ireland section, with licence and coverage for each. I have not verified current availability or terms of these; that check is part of the task. |
| 8 | M | D-18 | The MVP does not say where the transmission network model comes from, how detailed it is (zonal boundaries, ~30-bus reduced, or full 400/275 kV), or what "prices" means. GB has one national wholesale price plus Balancing Mechanism redispatch, so a nodal dispatch does not reproduce GB prices. | New questions: Q-C-9 (MVP network source and granularity) and Q-B-8 (MVP market fidelity: merit-order stack, unit commitment, or UC plus BM redispatch). |
| 9 | M | D-13 | Target hardware is Linux + NVIDIA, but the project folder is on a Mac. Where agents build, test and benchmark GPU code is undefined, and Q-H-3 (deployment) is open. | Ask where the Linux/NVIDIA machine is (owned, cloud, none yet). Keep GPU solvers behind the D-16 solver trait so the MVP runs CPU-only. |
| 10 | M | D-21 | 1-minute resolution over a weather year has no matching input data: ERA5 is hourly, GB settlement data is half-hourly. Sub-hourly runs need synthesised inputs. | Record as an assumption: MVP runs at 30 or 60 minutes; 1-minute is a capability of the engine, not an MVP data deliverable. |
| 11 | M | stack-candidates.md | Licence flags for a closed SaaS that are not in the risk list: cuDSS is proprietary (free to use, not open source); KLU/SuiteSparse is LGPL; TimescaleDB's compression features sit under the Timescale licence, not Apache-2.0; TimescaleDB support for PostgreSQL 18 should be confirmed. | Verify each and fold into R-3 and the storage ADR. |
| 12 | M | 04-requirements.md | Marked "not started until A–E answered", but 23 decisions are enough to derive a first set of requirements now, at least for the MVP. Waiting keeps the phase open indefinitely. | Derive a draft MVP baseline now; mark anything resting on an open question as `ASSUMPTION:`. |
| 13 | M | Round 5 | Eleven questions in one round is a lot, and several have good defaults. | Split into must-answer (Q-A-5, Q-A-6, Q-K-4, Q-H-3, Q-M-1) and "use defaults unless you object" (the rest). |
| 14 | M | Questionnaire | Missing questions: Q-M-3 non-goals is open and unasked in Round 5 (the cheapest scope control available); tenant isolation model for SaaS; who the forecast customer is; currency and cross-border handling for I-SEM (EUR) vs GB (GBP). | Add as new IDs. |
| 15 | L | stack-candidates.md | Stale after D-20: "Arrow (zero-copy between Python/Julia/Rust)". The Q-G-1 default still says "Python SDK". | Reword to match D-20. |
| 16 | L | 05-risks.md | R-7 cites D-5, which is superseded by D-23. | Cite D-23. |
| 17 | L | AGENTS.md | The "where things live" table omits `05-risks.md`, `REVIEW-BRIEF.md` and `reviews/`. | Add three rows. |
| 18 | L | 02-questionnaire.md | Q-C-7 sits in section I; Q-A-6 and Q-A-7 appear before Q-A-5. IDs are stable so this is cosmetic. | Move, do not renumber. |

## 3. What is good and should be kept

- The decisions log as sole source of truth, with supersede-not-edit (D-5 → D-23).
- OSS engines as offline validators only (D-10), which keeps the licence position clean and gives free reference results.
- Pluggable solvers and pluggable market designs. Both are the right seams.
- TOML scenario files as the canonical format (D-19), which gives reproducibility almost for free.
- The MVP choice itself. Weather-year dispatch is the right first slice.

## 4. Recommended next steps

In order. Steps 1–3 need the owner; the rest can be done by agents.

1. **Protect the work.** First commit, create the GitHub repository, push `main`. Add branch protection once CI exists.
2. **Owner confirms scope phasing** (finding 2). Proposed roadmap:
   - **Phase 1 (MVP):** GB + Ireland transmission, weather-year dispatch, map, prices, unserved energy, backtest vs one historic year.
   - **Phase 2:** GB market replica (day-ahead, BM redispatch), AC power flow and N-1, batch sweeps.
   - **Phase 3:** zonal/nodal alternative designs, I-SEM market detail, capacity expansion.
   - **Phase 4:** distribution, dynamics, SaaS multi-tenancy hardening.
3. **Close Round 5 in reduced form** (finding 13): five must-answer questions, defaults for the rest, plus the new questions from findings 6, 8, 9 and 14.
4. **Define quality gates** (Q-K-4). Proposed default: CI must pass `cargo fmt`, `clippy -D warnings`, tests, `cargo-deny` licence check; numerical results must match reference cases (pglib-opf / PyPSA) within stated tolerance; benchmark regressions fail the build; every PR reviewed by a model other than its author.
5. **Fix the doc inconsistencies** (findings 7, 11, 15–18) and add R-9, R-10.
6. **Derive the MVP requirements baseline** in `04-requirements.md` (FR/NFR with decision references and acceptance criteria).
7. **Run two de-risking spikes** under a narrow exception to the no-code rule:
   - Rust optimisation spike (finding 5).
   - Data spike: assemble a GB + Ireland transmission network and one weather year from open sources, and record what is missing and under which licence.
8. **Write the first ADRs**, informed by the spikes: 0001 repo layout and CI gates · 0002 network and scenario data model · 0003 optimisation modelling layer and solver trait · 0004 storage (Postgres/Timescale/Parquet split) · 0005 job execution · 0006 market-design plugin model.
9. **Flip STATUS to "code allowed"** once steps 2–4, 6 and ADRs 0001–0003 are accepted, then break the MVP into tasks:
   - M0 workspace, CI, licence gate
   - M1 data ingest and network model
   - M2 dispatch engine validated against PyPSA on the same inputs
   - M3 API and results storage
   - M4 map UI
   - M5 historic-year backtest report

## 5. Limits of this review

I read the documents only. I did not check current versions, licences or availability of any tool or dataset named here or in prior-art.md, so items in findings 5, 7 and 11 are things to verify, not confirmed facts.
