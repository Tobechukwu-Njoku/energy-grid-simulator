---
name: review-round
description: Run a review round on a PR per D-53/D-54 — GPT reviews Claude's code, Claude reviews GPT's docs, an adversarial reviewer from the other vendor (D-91) tries to break it — then merge the reports and triage. Use after any substantive PR revision is pushed.
---

# Review round for a PR

Routing (docs/process/team.md):
- Code by Claude → GPT reviewer (`gpt-6.2-sol`, reasoningEffort high).
- Docs by GPT → Claude reviewer.
- Every PR → also an adversarial review (D-53, D-91): GPT 6.2 Sol (`adversarial review / OpenAI`),
  or Claude in a fresh thread (`adversarial review / Anthropic`) when GPT wrote the PR. Never Gemini.
- Every GPT brief starts: "Do not spawn sub-agents or use spawn_agent; do all the work yourself in
  this thread." One task per GPT thread; start a fresh thread after a failure.

Steps:
1. Pin the exact head SHA. Every brief names it; reviews of any other SHA don't count.
2. Launch both reviews in parallel with `delegate_task`, `isolation: worktree`, `role: review`.
   Each brief must include: PR number and head SHA, the previous round's report paths and
   finding IDs to re-verify, what changed, "reproduce each finding with a concrete failing
   input", output path `docs/requirements/reviews/<model>-<date>-pr<N>-r<k>.md` in the
   REVIEW-BRIEF.md header format, "open as its own PR against main", "do not modify PR <N>",
   and the D-52 no-attribution rule.
3. Adversarial wording: phrase it as a *robustness review of our own CI checks / false negatives*.
   Ask the reviewer to describe key formats instead of writing realistic secrets into the report.
4. Finding IDs: independent `F<n>` / `R<n>`; adversarial `ADV<PR>R<k>-<n>`. Never renumber.
5. If a reviewer can't push (e.g. missing machine-account credentials, D-74), push its
   branch from its worktree and open the PR yourself.
6. Merge the report PRs (`gh pr merge --squash --delete-branch`), then send ONE combined
   revision brief to the coder (see `delegate-code` skill). Prefer general fixes over
   per-case patches so rounds converge.
7. Merge the reviewed PR only when both reviewers' latest verdicts on the exact head are clean
   and the coordinator has checked that head.
