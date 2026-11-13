# STATUS

_Last updated: 2026-11-13_

**Phase:** Build (M0). Requirements engineering complete. Decisions D-1 … D-73 (D-5 superseded by D-23).
Fable review (`reviews/claude-fable-5-2-2026-11-12.md`) processed in rounds A–C.
**Code allowed?** **Yes** (D-72, 2026-11-13). Requirements baselined; ADRs 0001–0006 accepted. Current milestone: **M0** (workspace, CI, licence gate).
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

**PR #23 revision-2 review:** exact head `b38db0675fa3b2eb6a05600fc6fb22d85e0dd8cd`
re-reviewed; verdict **must-fix**. All 81 submitted tests pass, but nine remaining
enforcement findings require correction and exact-head re-review. Evidence and
R1–R8 / ADV23-01..18 dispositions:
[revision-2 report](reviews/gpt-2026-11-13-pr23-r2.md). PR #23 was not modified.

**Merged to main:** GB data catalogue (PR #1, verified), IE/weather/dependency-licence catalogues (PR #2,
verified), reviews and triage for requirements, PR #4 and PR #5 (PRs #3, #6, #7, #8, #9).

**2026-11-13: workspace moved to the coding container (D-65); work resumed.** Running: Claude and Gemini reviews of the revised PRs #4 and #5, GPT re-verification of PR #12, and Gemini gathering of the 2024 Irish prices. Next: merge, clean the history and make the repo public, then GPT drafts the topology ADRs 0002/0004/0005/0006 (recommended: a modular monolith).

| Item | State | Next action |
|---|---|---|
| PR #4: MVP requirements + ADR 0001/0003 | Round 3 revision pushed (`52fda46`); reviews #13, #15 merged | Claude re-check of round 3; then owner approves accuracy thresholds/defaults; merge |
| PR #17: topology ADRs 0002/0004/0005/0006 | Round 2 revision pushed (`d99ceab`); reviews #19, #20 merged | Claude re-check; then 4 owner questions (Valkey, reconstruction label, no uploaded market code before P4, forecast priority); merge |
| PR #5 data spike | **Merged** (`0a14265`) | none |
| PRs #10, #11, #12, #18 data | **Merged** (verified by GPT; known source gaps in each COVERAGE.md) | none |
| History cleanup (D-52) + public repo (D-58) | **Done** 2026-11-13: trailers stripped from all commits, history scanned (no secrets, no raw research-only data), repo public, `main` protected (no force-push, linear history) | Add required checks once M0 CI exists |
| Owner inputs | Open | Copernicus account (ERA5), ENTSO-E token, UKPN/SPEN/NPg portal logins, target-system specs (D-65) |

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
