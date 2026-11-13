---
name: record-decision
description: Record an owner decision as the next D-<n> in the append-only decisions log and update STATUS.md. Use whenever the owner answers a question or approves a recommendation.
---

# Record an owner decision

1. Only record what the owner actually decided. Anything inferred is an `ASSUMPTION:` plus a new
   question in `docs/requirements/02-questionnaire.md`, never a D-entry.
2. Find the next ID: `grep -oE '^\| D-[0-9]+' docs/requirements/03-decisions-log.md | tail -1`.
3. Append exactly one table row (never edit or reorder earlier rows; supersede instead):
   `| D-<n> | <question ID or source> | **<decision in one bold clause>**; detail | <YYYY-MM-DD> | Owner |`
   If it replaces an earlier decision, say "supersedes D-<m>" in the detail.
4. Mark the source question answered in `02-questionnaire.md` if it has a Q-ID.
5. Update `docs/requirements/STATUS.md`: decision range `D-1 … D-<n>` and any affected rows.
6. If it changes requirements, routing or rules, update `04-requirements.md` (cite D-<n>),
   `docs/process/team.md` or `AGENTS.md` in the same commit.
7. Commit `docs: D-<n> <short summary>` and push. No AI attribution (D-52).
