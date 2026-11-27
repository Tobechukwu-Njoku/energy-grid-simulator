# Team & Roles

_Source: D-45, D-49, D-51, D-53, D-62, D-28, D-22, D-76, D-86, D-91. Owner checks milestones; agents plan, build, review and merge._

| Model | Role | Use for | Do not use for |
|---|---|---|---|
| **GPT 6.2 Sol** (medium effort; D-76) | Default author and coordinator (D-86) | Bulk of the work, including Rust engine, backend, frontend, tests, ingest pipelines, research, ADRs, specs, requirements, verification and validation, data gathering and drafting (D-62, D-91); **adversarial review of Claude-authored PRs (D-91)** | Reviewing its own work as the independent reviewer |
| **Claude Opus 5.5** | Focused cross-provider reviewer (D-28/D-54/D-86) | Independent review of GPT-authored PRs, including code and documents; review of GPT data gathering; adversarial review of GPT-authored PRs in a fresh thread (D-91) | Implementation, research, data gathering or other work |

## Rules

1. **Cross-provider review (D-28):** every PR is reviewed by a model from a different provider than its author.
   Claude-authored code → GPT 6.2 Sol reviews. GPT-authored documents and code → Claude Opus reviews (D-54).
2. **No Gemini (D-91).** Gemini has no role: no reviews, no data gathering, no drafts. GPT gathers data
   and drafts; Claude reviews it. Gemini output already merged stays as the historical record.
3. Merge gates in D-28 apply to everyone; reviewer approval does not override a failing gate.
4. GPU work: SSH for development, self-hosted GitHub Actions runner for GPU CI gates (D-43).
5. **Commit and push frequently (D-53):** small commits, pushed after every meaningful step.
6. **Adversarial review (D-53, D-91):** every PR, docs or code, also gets an adversarial review whose job
   is to break it: contradictions, wrong facts, missing edge cases, failure modes. GPT 6.2 Sol does it
   (`adversarial review / OpenAI`); when GPT wrote the PR, Claude does it in a fresh thread
   (`adversarial review / Anthropic`). The adversarial reviewer is never from the author's vendor (the gate
   enforces this). Findings are claims, not facts: the coordinator triages them before anything changes.
8. **GPT threads (t3code#39101):** every GPT prompt says "Do not spawn sub-agents or use spawn_agent; do all
   the work yourself in this thread." One task per thread; start a fresh thread after a failure.
7. **At most two delegates at once (D-86).** Bias effort towards GPT; reserve Claude for the focused cross-provider reviews above.
