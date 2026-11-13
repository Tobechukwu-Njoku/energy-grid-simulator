# STATUS

_Last updated: 2026-11-13_

**Phase:** Requirements engineering. Decisions D-1 … D-62 (D-5 superseded by D-23).
Fable review (`reviews/claude-fable-5-2-2026-11-12.md`) processed in rounds A–C.
**Code allowed?** No, except the **data spike** (D-31) in `spikes/`.
**Repo:** private (to go public per D-58 once preconditions are met), https://github.com/Tobechukwu-Njoku/energy-grid-simulator

## Snapshot of what's decided

Research + market-forecasting simulator (short-term and seasonal, D-25) of **GB + all-island
Ireland** with interconnectors, for consultancy/research, forecast customers (battery operators,
traders, suppliers/large users) and grants (D-33, D-41).
**Guiding principle (D-44): answer the present first.** Model today's grid accurately, validate it,
then scale out. Phases (`06-roadmap.md`): **P1** today's full transmission network with
unit commitment + Balancing Mechanism redispatch and a historic backtest → **P2** distribution,
home DER, statistical behavioural demand, AC power flow and voltages → **P3** zonal pricing and
2030 scenarios → **P4** dynamic stability and SaaS hardening.
The owner's core questions are listed in D-40.
**Rust everywhere code is needed**; TypeScript web UI in a pro-analytics, dark-mode style (D-47).
PostgreSQL 18 + Redis. Linux machine: Ryzen 5900X / 128 GB, consumer NVIDIA GPU, so the **CPU
path is primary** (R-14). Pluggable solvers. Future commercial SaaS, so **no share-alike data**.
Team (`docs/process/team.md`): Claude Opus 5.5 codes, GPT 6.1 Sol architects and reviews,
Gemini Flash does checked donkey work. Strict merge gates (D-28).

## Still open (non-blocking)

- Linux distro (Q-H-7)
- Remaining unanswered questionnaire items have defaults or are P2+ concerns

## Work state (coordinator: Claude Opus 5.5) — 2026-11-13

**Merged to main:** GB data catalogue (PR #1, verified), IE/weather/dependency-licence catalogues (PR #2,
verified), reviews and triage for requirements, PR #4 and PR #5 (PRs #3, #6, #7, #8, #9).

**2026-11-13: all agent work stopped by the owner ahead of moving the workspace to an LXC container.** Nothing is running. PRs #4 and #5 are revised and await Claude and Gemini review; PR #12 verification must be re-run; the 2024 Irish price gathering must be re-run. Raw data (`~/energy-data/raw`, 4.3 GB) is outside git and must be copied.

| Item | State | Next action |
|---|---|---|
| PR #4: MVP requirements + ADR 0001/0003 | Reviewed (Claude PR #8, Gemini PR #6); needs revision | Author revision: all review findings, triage, D-51..D-62 |
| PR #5: data spike (R-12 "Partially") | Reviewed (Claude PR #7, Gemini PR #9); needs revision | Author revision per Claude review + PR5 triage + D-56/D-57 |
| PR #10: DNO 132 kV inventory | **Merged** 2026-11-13 after GPT verification (84 claims, 21% of determinate claims corrected, 8 UNVERIFIED) | Portal logins (UKPN, SPEN, NPg, ENWL) to close the gaps; commercial clearance in TODO-pre-commercial |
| PR #12: Elexon 2024 data gathered (4.2 GB, 3,673 files, 12 datasets, D-62) | Done; file counts and sizes spot-checked by coordinator | GPT verification (record counts, coverage claims) |
| PR #11: NESO/DNO/EirGrid/SEMO data (30 files) | **Merged** 2026-11-13 after GPT verification (hashes match; licences corrected) | Gap: 2024 Irish prices (have 2018–2020 only); owner logins for UKPN, SPEN, NPg, ENWL and an ENTSO-E token. Local-only T3 checkpoint ref `286d675` holds ETYS raw files: never push `refs/t3/*`, and exclude them from any public mirror |
| History cleanup (D-52) + public repo (D-58) | Waiting | After #4 and #5 merge: rewrite AI trailers, scan, make public, enable branch protection |

Rules in force: GPT for non-code (D-51), Claude reviews GPT docs (D-54), Gemini adversarial reviews (D-53)
and significant data gathering (D-62), no AI attribution (D-52), research-phase data use (D-56).
Only the coordinator edits STATUS and the decisions log.

## Next steps (agents may do now)

1. Data spike (D-31): permissively licensed GB + Ireland full transmission network + one weather
   year; record licence per dataset; answers R-12.
2. Add GB + Ireland data sources to `prior-art.md` with licences (review #7); verify licence
   flags for cuDSS, SuiteSparse/KLU, TimescaleDB on PG18 (review #11).
3. Draft MVP requirements baseline in `04-requirements.md` (review #12), marking open items `ASSUMPTION:`.
4. ADRs 0001–0006 (review §4 step 8).
5. Flip to "code allowed" once 3 and ADRs 0001–0003 are accepted.
