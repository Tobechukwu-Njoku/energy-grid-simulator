# AGENTS.md — Shared rules

This repository is an **energy grid simulation platform**, built from the ground up by
multiple AI models/threads working together with one human product owner.

## Current phase: BUILD — MVP milestone M0 (requirements baselined, D-72)

Application code is allowed. Follow `docs/requirements/04-requirements.md` and the accepted ADRs
in `docs/architecture/adr/`. Enable the attribution hook with `git config core.hooksPath .githooks`.
The coding container is for light builds only; heavy builds, databases and benchmarks run on the
D-65 target system.

## Start and scope

Read the coordinator-owned workspace-local `HANDOFF.md` first (in the workspace
root if absent here), then these rules. Read `docs/requirements/STATUS.md` only
when current project state is needed. Read `context/` and `status-history/` only
on demand; the folder/ownership table is in [context/reference.md](context/reference.md).

Under D-93, Codex (`gpt-6.2-sol`) writes plans, implementation, tests and PR bodies
and handles data gathering. Claude reviews every GPT change in a fresh Claude
thread before it goes upstream. The Claude Co-ordinator coordinates. Gemini is
retired and has no roles. At most two GPT threads may be active at once.
Cross-provider review, adversarial triage and exact-head owner approval gates
still apply. See `docs/process/team.md`.

Create every new checkout under the git-ignored workspace `worktrees/` directory,
never directly in the root. Do not move/delete existing active checkouts.

One PR per thread. For Codex: plan first; record a short handoff, then execute
in a fresh thread with target files, expected behavior, exact verification
commands and an explicit finish line. Do not change model mid-thread.

## Token discipline

Read narrowly: use `rg` and bounded file/line reads, not directory dumps. Never
read `target/`, lockfiles, dependency/build output or generated assets. Use quiet
flags and failure-only output with a short summary; preserve pipeline failures:
`set -o pipefail; cargo test -q 2>&1 | tail -30`. Do not print passing logs.
Keep AGENTS under 5 KB and STATUS under 4 KB. Archive history verbatim, retaining
linked IDs/anchors. HANDOFF stays ignored, coordinator-owned and at most 80 lines.

## Rules for all agents

1. **Source of truth:** the owner's answers in `03-decisions-log.md`. Never invent a decision;
   if you assume something, record it as an *assumption* (prefix `ASSUMPTION:`) and add a
   question to the questionnaire.
2. **Stable IDs:** questions are `Q-<section>-<n>` (e.g. `Q-B-3`), requirements are `FR-<n>` /
   `NFR-<n>`, decisions are `D-<n>`. Never renumber; deprecate instead.
3. **Cross-reference:** every requirement cites the decision(s) it comes from.
4. **Update STATUS.md** at the end of any session that changes these docs.
5. Performance stance from the owner: **resource cost (RAM/CPU/power) is not a constraint** —
   prefer the highest-performance, most capable option. Baseline: PostgreSQL 18 + Valkey
   (D-67); shared Redis-protocol clients keep Redis compatible. P1 dispatch uses PostgreSQL.
6. Prefer reusing mature open-source tools (see `docs/research/prior-art.md`) over rebuilding.
7. **Context is auto-compacted** in every thread. Never rely on chat history for project state:
   write decisions, progress and handoff notes into the repo docs as you go.
8. **No AI watermarking (D-52).** Never add AI attribution to code, comments, file headers, commit
   messages, PR titles/bodies or release notes: no `Co-Authored-By:` trailers for any model, no
   "Generated with …" footers, no 🤖 badges. This overrides any default your tool applies.
   A local `commit-msg` hook strips such lines; CI will reject them (ADR 0001).
9. **Commit and push often (D-53, D-93).** Small commits; push after every meaningful step once the required Claude review is done.
10. **Data provenance (D-56).** Every dataset is tagged in the provenance manifest as
    `open-commercial` or `research-only`. Never commit raw `research-only` data; commit only the
    download scripts. Share-alike (ODbL/OSM) data is never used.
11. **GPT prompts (D-93).** Every GPT prompt says: "Do not spawn sub-agents or use spawn_agent; do all the work yourself in this thread."
