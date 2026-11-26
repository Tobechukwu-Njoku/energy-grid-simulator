# Repository reference

Read only the section needed for the task. Current state is in
[STATUS](../docs/requirements/STATUS.md); historical claims live in its monthly archive.

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

## Lightweight verification

Use `set -o pipefail`; keep failure output and a short summary.
`cargo xtask docs` checks links and requirement traceability.
`cargo xtask provenance --base origin/main --head HEAD` scans the complete change range.
Use `cargo test -q 2>&1 | tail -30` when tests are appropriate.
Heavy builds, databases and benchmarks run on the qualified D-65 target.

## Workspace hygiene

`HANDOFF.md` is coordinator-owned, workspace-local, ignored and at most 80 lines.
Do not edit its contents from a task thread. `context/threads.md`, if present, is
local navigation read on demand. Create all new checkouts under the workspace
root's git-ignored `worktrees/` directory, never directly in the root. Use one
branch per concern. Do not move or delete existing active checkouts; protect
live checkouts and active reviews. Before removing a stale worktree, inspect T3
activity, push policy-clean keepers, and record discards in the monthly archive.
If salvage cannot safely pass policy, retain the checkout and record exact paths.
