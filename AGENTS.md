# AGENTS.md — Read this first

This repository is an **energy grid simulation platform**, built from the ground up by
multiple AI models/threads working together with one human product owner.

## Current phase: REQUIREMENTS ENGINEERING (no application code yet)

Do **not** start writing application code until `docs/requirements/STATUS.md` says the
requirements are baselined. Sole exception: the data spike in `spikes/` (D-31).

## Where things live

| Path | Purpose | Who edits |
|---|---|---|
| `docs/requirements/STATUS.md` | Phase, progress, what's blocking | Any agent, keep current |
| `docs/requirements/01-vision.md` | Product vision & confirmed facts | Only from owner answers |
| `docs/requirements/02-questionnaire.md` | Every open question with ID, options, recommended default | Add questions freely; mark answered |
| `docs/requirements/03-decisions-log.md` | Append-only log of owner decisions (ID → answer → date) | Append only |
| `docs/requirements/04-requirements.md` | Formal requirements (FR-/NFR- IDs) derived from decisions | Derive from decisions log only |
| `docs/requirements/05-risks.md` | Risk register (R-<n>) | Any agent |
| `docs/requirements/06-roadmap.md` | Phases P1–P4 and MVP milestones | From decisions only |
| `docs/requirements/REVIEW-BRIEF.md` | Instructions for reviewer models | Any agent |
| `docs/requirements/reviews/` | Review reports (`<model>-<date>.md`) | Reviewers |
| `spikes/` | Throwaway exploratory code (only the data spike, D-31) | Spike agents |
| `docs/process/team.md` | Which model does what; review rules | Owner decisions only |
| `docs/research/prior-art.md` | Existing tools/libraries/datasets we may reuse | Any agent |
| `docs/architecture/stack-candidates.md` | Candidate tech stack (tentative until ADRs accepted) | Any agent |
| `docs/architecture/adr/` | Architecture Decision Records (`NNNN-title.md`) | Propose; owner accepts |

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
9. **Commit and push often (D-53).** Small commits; push after every meaningful step.
10. **Data provenance (D-56).** Every dataset is tagged in the provenance manifest as
    `open-commercial` or `research-only`. Never commit raw `research-only` data; commit only the
    download scripts. Share-alike (ODbL/OSM) data is never used.
