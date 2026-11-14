# STATUS

_Last updated: 2026-11-14_

**Phase:** Build (M0). Requirements engineering complete. Decisions D-1 … D-74 (D-5 superseded by D-23).
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

## Work state (coordinator: Claude Opus 5.5) — 2026-11-14

**PR #23 revision 7 (implementation, 2026-11-14):** R6-01..04 (GPT) and ADV23R6-01..10
(Gemini) addressed; the compiler-enforced design stays. Reviewed adapters are an explicit
`[[adapter]]` list (package name AND directory) in the trusted, protected
`repository.toml`. The rustc wrapper attributes every compilation through a `cargo
metadata` configuration (manifest directory, package name, source file) and refuses what it
cannot attribute; no crate-name heuristics. The linker guard accepts only allow-listed
library/framework names (`gates.toml` `link_libraries`/`link_frameworks`) and Rust's own
artifacts, and refuses response files, scripts, plugins and driver redirection. The gate
builds default, all, no-default and each single feature, and runs `cargo test --doc`; every
non-adapter library root needs `#![doc(test(attr(forbid(unsafe_code))))]`. The build-gate
step must be its job's last step (file-command guarantee); scripts may not name or
indirectly reach file-command files. Provenance path rules ignore case. Attribution: generic
AI/LLM/vendor-AI names, collaboration joiners, possessives, `from:` trailers, Roo Code,
Continue, Supermaven. Residual risks 8–10 in the ADR 0001 revision-7 addendum. Finding→test
table in the PR body. Needs exact-head GPT re-review, Gemini adversarial re-check and triage
before bootstrap attestation.

**PR #23 revision-8 review (2026-11-14 UTC):** exact head
`bcd4b2c7aa933662ad952f3656b1a1f408057fc3` re-reviewed; verdict **must-fix**.
All 176 submitted tests pass; the round-7 reproduction inputs are rejected,
including the original native linker-script fixture. One STRUCTURAL rule conflict
remains: the attribution gate rejects D-61-permitted automatic bot summaries and
has no provenance-qualified block exception. The round-7 recommendation to remove
the integration is withdrawn. Standard OS library roots are reasonable M0 location
trust under the documented job-tampering limit; hosted passwordless sudo means they
are not process isolation. Do not attest this head as the one-time bootstrap seed.
Evidence, regression matrix and required correction:
[revision-8 report](reviews/gpt-2026-11-14-pr23-r8.md). PR #23 was not modified.

**PR #23 revision-7 review (2026-11-14 UTC):** exact head
`f986bbc48b23390ca50f4b22065313bcb08d94ea` re-reviewed; verdict **must-fix**.
All 163 submitted tests pass, and all fourteen round-6 reported inputs are fixed.
Two new STRUCTURAL probes remain: an allowlisted library name resolves through
an OUT_DIR linker script to a Python-named archive, and dev-dependency feature
unification hides a production single-feature native link. A CodeRabbit credit
also escapes the shared attribution patterns (ATTRIBUTION-SPELLING). The first
two gaps are outside the ADR's stated tampering/multi-feature limitations.
Do not attest this head as the one-time bootstrap seed. Reproductions, controls,
the finding/test matrix and M0 assessment:
[revision-7 report](reviews/gpt-2026-11-14-pr23-r7.md). PR #23 was not modified.

**PR #23 revision-6 review (2026-11-14 UTC):** exact head
`68a583d88276cd4339c0d08cebf6f0c8367ca76a` re-reviewed; verdict **must-fix**.
The four GPT round-5 inputs and Gemini's actionable underlying gaps are fixed;
two literal Gemini reproduction layouts need the qualifications in the report.
Four new STRUCTURAL counterexamples remain: a real crate named `___` skips the
wrapper, response-file quoting hides a native library name, all-features builds
omit default-feature link code, and doctests do not inherit the claimed root
unsafe forbid. Do not attest this head as the one-time bootstrap seed.
Reproductions, controls, regression matrix and M0 limitation assessment:
[revision-6 report](reviews/gpt-2026-11-13-pr23-r6.md). PR #23 was not modified.

**PR #23 revision 6 (implementation, 2026-11-13):** R5-01..04 (GPT) and ADV23R5-01..10
(Gemini) addressed by a change of approach (coordinator decision): the static Rust
reachability walker is deleted. What Cargo builds now comes from `cargo metadata`
(resolved members, targets, path deps, patches, build-deps); the cheap static rules stay
(crate-root `forbid`, manifests, alias-only Cargo config, workflows). New
`cargo xtask build-gate`, a step of the unprivileged `policy` CI job, builds the PR
workspace (all targets, all features) with main's xtask as `RUSTC_WRAPPER`: `-F unsafe_code`
for every repository crate outside `crates/solver-*` (lint caps stripped), plus a linker
guard and Cargo `build-script-executed` output for the Python/Julia ban. Workflows may set
no `CARGO_*`/`RUSTC*`/`RUSTDOC*`/`RUSTUP_*`/`*RUSTFLAGS` variable unless `allowed_env` in
the trusted `gates.toml` lists it. Attribution: generic verb/joiner/"help of" footer
phrases, any `<Words> by:` trailer with a model-shaped name, and Cody/Sourcegraph,
CodeWhisperer, GLM, Command R in the single name list. Residual risks in the ADR 0001
revision-6 addendum. Finding→test table in the PR body. Needs exact-head GPT re-review,
Gemini adversarial re-check and triage before bootstrap attestation.

**PR #23 revision-5 review:** exact head `b21d83de896ba09b59259fa687e427271bbc06ed`
re-reviewed; verdict **must-fix**. All 143 submitted tests pass, and all fourteen
round-4 concrete inputs are fixed. Four new STRUCTURAL probes, each verified with
real offline Cargo compilation and a rejection control, expose a substring-based
OUT_DIR include exemption, an aliased include macro, Cargo member-glob mismatch
and lost build context through a local patch. Do not attest this head as the
one-time trusted bootstrap seed. Evidence and finding-by-finding regression
assessment: [revision-5 report](reviews/gpt-2026-11-13-pr23-r5.md).
PR #23 was not modified.

**PR #23 revision-4 review:** exact head `dc3093ea100ace554ad3a7c11a9f77927f2776a7`
re-reviewed; verdict **must-fix**. All 124 submitted tests pass, and all eleven
round-3 concrete inputs are fixed. Four new STRUCTURAL probes demonstrate a
production target escaping through `spikes/`, incorrect `**/` CODEOWNERS coverage,
nested-array provenance loss and ignored extensionless manifests. Do not attest
this head as the one-time trusted bootstrap seed. Reproductions, regression
assessments and activation limits: [revision-4 report](reviews/gpt-2026-11-13-pr23-r4.md).
PR #23 was not modified.

**PR #23 revision 4 (implementation, 2026-11-13):** R3-01..04 (GPT) and ADV23R3-01..07
(Gemini) addressed with general mechanisms and mapped regression tests (finding→test
table in the PR body): structural parse of the `forbid` lint list; every Cargo-selected
target scanned regardless of suffix; no currentness exemption for the reviews directory;
`?` wildcard with a differential test against `git check-ignore`; NFKD plus removal of
all combining marks; vendor-prefixed model names and vendor domains; generic comment
delimiter stripping; shell allow-list including inherited `defaults.run.shell`; a
schema rule for manifest file keys; one canonical manifest name. Needs exact-head GPT
re-review, Gemini adversarial re-check and triage before bootstrap attestation.

**PR #23 revision-3 review:** exact head `b2173e28f8f232366ffe6a27d5e787fd2adfc39c`
re-reviewed; verdict **must-fix**. All 111 submitted tests pass. The original
F1–F9 / ADV23R-01..09 inputs are addressed (D-74 account/approval activation
remains operational), but four independent probes expose remaining unsafe-lint,
Cargo-target, review-currentness and CODEOWNERS gaps. Do not attest this head as
the trusted policy seed. Evidence and finding-by-finding regression assessment:
[revision-3 report](reviews/gpt-2026-11-13-pr23-r3.md). PR #23 was not modified.

**PR #23 revision-2 review:** exact head `b38db0675fa3b2eb6a05600fc6fb22d85e0dd8cd`
re-reviewed; verdict **must-fix**. All 81 submitted tests pass, but nine remaining
enforcement findings require correction and exact-head re-review. Evidence and
R1–R8 / ADV23-01..18 dispositions:
[revision-2 report](reviews/gpt-2026-11-13-pr23-r2.md). PR #23 was not modified.

**PR #23 revision 3 (implementation, 2026-11-13):** F1–F9 and ADV23R-01..09 addressed with
mapped regression tests (finding→test table in the PR body). Trusted gates now run from
`main` (`trusted-gates.yml`); PR jobs fail closed until `main` holds policy. **Coordinator
action before CI can pass:** commit `docs/process/ci-bootstrap.toml` with
`seed_sha = "<final PR #23 head>"` to `main` and re-run CI; then GPT re-review, Gemini
adversarial re-check and a triage record for every adversarial finding (format in
REVIEW-BRIEF.md). The D-74 machine account (`egs-bot` placeholder) is still to be created.

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
