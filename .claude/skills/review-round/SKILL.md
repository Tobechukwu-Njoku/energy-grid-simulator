---
name: review-round
description: Review a GPT change locally in fresh Claude threads before it goes upstream, then collect and triage review evidence under D-53, D-54 and D-93.
---

# Review round for a PR

The Claude Co-ordinator routes every GPT change, code or docs, to a fresh Claude
thread for independent review before it goes upstream. Use a separate fresh
Claude thread for adversarial review (`adversarial review / Anthropic`).
Repeat on revisions. For existing Claude-authored code PRs, retain the
cross-provider GPT review routing and the two-active-GPT-thread limit.

Start every GPT prompt with: "Do not spawn sub-agents or use spawn_agent; do all the work yourself in this thread."

Steps:
1. Pin the exact head SHA. For GPT changes awaiting publication, use the local head. Every brief names it; reviews of any other SHA don't count.
2. Start separate fresh review threads using the pinned head. Use local worktrees for unpublished changes.
   Each brief must include: PR number (if assigned) and head SHA, the previous round's report paths and
   finding IDs to re-verify, what changed, "reproduce each finding with a concrete failing
   input", the report destination and REVIEW-BRIEF.md header format, "keep the review
   evidence local until publication is authorised", "do not edit the reviewed change",
   and the D-52 no-attribution rule. If the PR number is pending, keep a local draft
   keyed by head SHA and apply `docs/requirements/reviews/<model>-<date>-pr<N>-r<k>.md`
   and the PR-specific report metadata once the number is assigned.
3. Adversarial wording: phrase it as a *robustness review of our own CI checks / false negatives*.
   Ask the reviewer to describe key formats instead of writing realistic secrets into the report.
4. Finding IDs: independent `F<n>` / `R<n>`; adversarial `ADV<PR>R<k>-<n>`. Never renumber.
5. The Claude Co-ordinator verifies and triages the findings. Send one combined
   revision brief to Codex (see `delegate-code`) and review the resulting head again.
6. After the required local review and once publication is authorised, publish
   and merge the review reports against main for the corresponding PR head.
7. Merge the reviewed PR only when both reviewers' latest verdicts on the exact
   head are clean, the coordinator has checked that head, and required checks and
   exact-head owner approval are in place.
