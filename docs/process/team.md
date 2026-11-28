# Team and review

## Roles

- **GPT writes** each PR, from the `automated-machines` account. Keep at most two
  GPT threads active. GPT never spawns sub-agents.
- **While GPT is unavailable**, Claude writes, and a different, fresh Claude thread
  reviews adversarially. When GPT returns, GPT reviews those heads as well.
- **A fresh Claude thread reviews** every tier 1 and tier 2 PR and posts its
  findings as a PR comment from `automated-machines`.
- **The managing session**, running as the owner's account `Tobechukwu-Njoku`,
  checks that the Claude review passed and CI is green, then approves at the exact
  head. Auto-merge then fires.
- Gemini has no role.

## Review tiers

The trusted `review-tier` job classifies the net changes against current main.
The highest tier across changed paths wins, deleted paths included.

| Tier | Changes | Merge requirements |
| --- | --- | --- |
| 0 | Markdown and text under `docs/` or `notes/`, and root Markdown that code does not include | Green CI |
| 1 | Product code, spikes, tests and everything else | Green CI plus the owner's approval on the current head |
| 2 | Workflows, xtask, hooks, policy and process, Cargo manifests and lockfiles, toolchain and lint config, build scripts, security, provenance and schema migrations | Tier 1, plus an adversarial review comment from a different vendor than the author |

The owner decided on 2026-11-27 that only the owner can approve. The gate
counts an APPROVED review on the current head from the
`review_approver` in `xtask/policy/repository.toml`. A later request for changes
or a dismissal revokes it; the author never counts. The tier 2 cross-vendor
comment is checked by people, not by the gate. Historical reports are in the
[archive](../../archive/reviews/).

After approving, re-run the latest `review-tier` job for the PR. If the owner dismisses
an approval or requests changes after the job passed, the owner must re-run
`review-tier` and confirm it fails before any merge can proceed. Pause auto-merge
while the check still shows the stale approval.

If `trusted-gates.yml` changed on main, push an empty commit or close and reopen
the PR to trigger a fresh run. A manual re-run uses the earlier workflow.

## Ruleset

Main requires `merge-gates` and `review-tier`. Where supported, use the
repository-targeting “Require workflows to pass before merging” ruleset rule to
pin `.github/workflows/trusted-gates.yml` to `refs/heads/main`. GitHub runs the
required workflow from that protected ref, binding the check to the trusted
workflow file. Keep the workflow name-spoof scan.
[GitHub ruleset workflow settings](https://docs.github.com/en/rest/orgs/rules).

## Focus: M0

Build M0 features from the [requirements backlog](../requirements/04-requirements.md),
each with tests. A `feat` or `fix` title requires an added or modified test
source file. Pause docs and decision PRs unless they unblock a feature. Keep the
decisions log for product decisions.

A reviewer finding seen twice becomes a lint rule or a test.
