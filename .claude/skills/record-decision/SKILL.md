---
name: record-decision
description: Record an owner product decision as the next D-<n> in the append-only decisions log and update STATUS.md.
---

# Record an owner product decision

Keep review and process rules in [Team and review](../../../docs/process/team.md).

1. Only record what the owner actually decided. Anything inferred is an `ASSUMPTION:` plus a new
   question in `docs/requirements/02-questionnaire.md`, never a D-entry.
2. Find the next ID: `rg -o '^\| D-[0-9]+' docs/requirements/03-decisions-log.md | tail -1`.
3. Append exactly one table row (never edit or reorder earlier rows; supersede instead):
   `| D-<n> | <question ID or source> | **<decision in one bold clause>**; detail | <YYYY-MM-DD> | Owner |`
   If it replaces an earlier decision, say "supersedes D-<m>" in the detail.
4. Mark the source question answered in `02-questionnaire.md` if it has a Q-ID.
5. Update `docs/requirements/STATUS.md`: decision range `D-1 … D-<n>` and any affected rows.
6. If the decision changes requirements, update `04-requirements.md` and cite D-<n>.
7. Commit `docs: D-<n> <short summary>` and push. No AI attribution (D-52).
