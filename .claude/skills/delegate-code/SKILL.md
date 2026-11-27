---
name: delegate-code
description: Route code plans, implementation, tests and PR bodies to Codex GPT 6.2 Sol under D-93.
---

# Route code to Codex

The Claude Co-ordinator starts a fresh top-level Codex thread with `gpt-6.2-sol`.
Check that starting it keeps the total at two active GPT threads or fewer.

Start every GPT prompt with: "Do not spawn sub-agents or use spawn_agent; do all the work yourself in this thread."

The brief must be self-contained and include:
- Repo, branch, current head SHA; "merge origin/main first"; "if the branch is checked out in
  another worktree, work on a local branch".
- Files to read first: AGENTS.md, relevant D-IDs in 03-decisions-log.md, ADRs, and every review
  report whose findings must be fixed (by path and finding ID).
- The design intent for anything non-obvious (what must be trusted, what must fail closed).
- "Every finding gets a regression test shown to fail on <old SHA> and pass after; map
  findings → tests in the PR body."
- Gates: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo nextest run`,
  `cargo xtask all`. Light builds only here; heavy builds/benchmarks belong on the D-65 target.
- Rust for all product code, tooling, CI and tests (D-73); Python only in `spikes/` and
  `tools/reference-validators/`.
- Rules: no AI attribution anywhere (D-52); never push `refs/t3/*`; don't merge; don't change
  repo settings.
- "Keep the resulting change local for Claude review. Report the branch, head SHA and checks state."

After it finishes, start a `review-round` on the local head before sending the change upstream.
