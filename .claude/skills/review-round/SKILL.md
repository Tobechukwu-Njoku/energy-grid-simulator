---
name: review-round
description: Review the current head of a Tier 1 or Tier 2 pull request.
---

# Review a pull request

Follow [Team and review](../../../docs/process/team.md). Review the stated PR
and head in a fresh Claude thread. Check the diff, tests and applicable
requirements or ADRs. Submit a GitHub Approve or Request changes review on
that head, with concrete findings in the PR.

On Tier 2, post the additional adversarial review as a PR comment. Look for
failure modes and missing coverage. Keep evidence and follow-up in the PR.
A finding seen twice becomes a lint rule or a test. Work in this thread
without sub-agents.
