@AGENTS.md

Under D-93, review GPT changes in a fresh Claude thread before they go upstream.
Verify independent and adversarial findings against the exact head. The Claude
Co-ordinator manages work and triages findings. Codex writes plans, implementation,
tests and PR bodies.

# Compact instructions
When compacting, keep:
1. The thread's PR or issue number, goal, acceptance criteria and changed files (branch, worktree, head commit).
2. Current build/test/CI status and any unresolved failures, review findings or bot comments.
3. Constraints and edge cases discovered, plus decisions or "go"s still pending from the user.
Drop full command output, file dumps and passing test logs.
