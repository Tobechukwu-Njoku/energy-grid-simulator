# Team & Roles

_Source: D-28, D-45, D-49, D-51, D-53, D-54, D-62, D-76, D-86, D-90, D-91, D-93._

| Who | Responsibilities |
|---|---|
| **Codex (GPT 6.2 Sol, `gpt-6.2-sol`)** | Writes plans, implementation, tests and PR bodies. Gathers data and drafts documents. |
| **Claude** | Reviews every GPT change in a fresh Claude thread before it goes upstream. The Claude Co-ordinator routes work and triages findings. |
| **Owner** | Makes decisions, checks milestones and approves changes at the exact head. |

## Rules

1. **Cross-provider review (D-28, D-54, D-93).** Claude independently reviews GPT-authored code
   and documentation. Pin the exact head, including for local review before pushing. Each
   revision needs fresh Claude review before it goes upstream. GPT independently reviews
   existing Claude-authored code PRs. (D-91 still covers Claude-authored PRs opened before D-93.)
2. **Gemini is retired (D-91, D-93).** Gemini has no role.
3. Merge gates in D-28 apply to everyone. Reviewer approval does not override a failing gate.
4. GPU work uses SSH for development and the self-hosted GitHub Actions runner for GPU CI gates (D-43).
5. **Commit and push frequently (D-53, D-93).** Make small commits and push meaningful steps after
   the required pre-upstream review.
6. **Adversarial review (D-53, D-91).** Every PR also gets an adversarial review for contradictions,
   wrong facts, missing edge cases and failure modes. For GPT-authored PRs, use a separate fresh
   Claude thread (`adversarial review / Anthropic`). For existing Claude-authored PRs, use GPT
   6.2 Sol (`adversarial review / OpenAI`). The Claude Co-ordinator verifies and triages findings.
7. **At most two GPT threads may be active at once (D-93).**
8. **GPT prompts (D-93).** Every GPT prompt says: "Do not spawn sub-agents or use spawn_agent; do all the work yourself in this thread."
   Use one task per thread and start a fresh thread after a failure (t3code#39101).
