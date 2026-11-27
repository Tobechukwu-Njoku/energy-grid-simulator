# Team and review

GPT 6.2 Sol writes changes. A fresh Claude thread reviews each Tier 1 or Tier 2 PR
using GitHub's Approve or Request changes review. Gemini has no role. Keep at
most two GPT threads active. Each thread does its own work without sub-agents.
Open PRs from `automated-machines` so the owner can approve them.

## Review tiers

The trusted `review-tier` check classifies the net changes against current main.
The highest tier across changed paths wins, including deleted paths.

| Tier | Changes | Merge requirements |
| --- | --- | --- |
| 0 | Docs, notes, status files and Markdown outside policy and process | Green CI |
| 1 | Product code, spikes, tests and other files | Green CI plus a non-author approval on the current head |
| 2 | Workflows, xtask, hooks, policy and process, Cargo manifests and lockfiles, security, provenance and schema migrations | Tier 1 plus a separate current-head approval by `Tobechukwu-Njoku`. Post an additional cross-vendor adversarial review as a PR comment. |

GitHub's review API does not identify the reviewer's model provider, so the
check accepts any non-author approval for the reviewer role. Tier 2 requires
two distinct approvals. The adversarial comment is a process step. Keep review
findings and responses in the PR conversation. Historical reports are in the
[archive](../../archive/reviews/).

Re-run the trusted workflow after approvals, requests for changes or dismissals.
Use auto-merge once the required reviews are in place. Main requires
`merge-gates` and `review-tier`.

## M0

Build M0 features from the [requirements backlog](../requirements/04-requirements.md),
with tests in each feature PR. A conventional `feat` or `fix` title requires
an added or modified test file. Pause new docs and decision PRs unless they
unblock a feature. Keep the decisions log for product decisions.

A reviewer finding seen twice becomes a lint rule or a test.
