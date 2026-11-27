---
name: delegate-code
description: Prepare an implementation brief for the M0 backlog.
---

# Prepare an implementation brief

Follow [Team and review](../../../docs/process/team.md). The coordinator assigns
implementation to a GPT 6.2 Sol thread. Keep at most two GPT threads active.
Each thread does its own work without sub-agents.

Give the author the repository, base or current PR head, the M0 requirement
and relevant ADRs, the expected behaviour and test cases, and any PR review
findings to address. Keep builds light here and run heavy workloads on the
D-65 target. Use the machine account for PRs and follow the attribution policy
in [Contributing](../../../CONTRIBUTING.md).
