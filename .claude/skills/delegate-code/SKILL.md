---
name: delegate-code
description: Delegate a coding task or PR revision to Claude Opus 5.5 (medium effort, ultracode on, D-64) with a complete self-contained brief. Use for any code generation in this repo.
---

# Delegate code to Opus

Target: `providerInstanceId: claudeAgent`, `model: claude-opus-5-5`,
`options: {effort: "medium", ultracode: true}`, `isolation: worktree`, `role: implementation`.

The child has no chat history; the brief must contain:
- Repo, branch, current head SHA; "merge origin/main first"; "if the branch is checked out in
  another worktree, work on a local branch and push to it".
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
- "Final line: PR URL, head SHA, checks state."

After it finishes, start a `review-round`.
