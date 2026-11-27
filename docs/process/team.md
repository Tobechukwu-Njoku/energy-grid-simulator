# Team & Roles

_Source: D-45, D-49, D-51, D-53, D-62, D-28, D-22, D-76, D-86. Owner checks milestones; agents plan, build, review and merge._

| Model | Role | Use for | Do not use for |
|---|---|---|---|
| **GPT 6.2 Sol** (medium effort; D-76) | Default author and coordinator (D-86) | Bulk of the work, including Rust engine, backend, frontend, tests, ingest pipelines, research, ADRs, specs, requirements, verification and validation; **verifying all Gemini output** | Reviewing its own work as the independent reviewer |
| **Claude Opus 5.5** | Focused cross-provider reviewer (D-28/D-54/D-86) | Independent review of GPT-authored PRs, including code and documents | Implementation, research, data gathering or other work |
| **Gemini Flash 3.8** (high) | Data gathering and adversarial review (D-53/D-62/D-86) | Bulk downloads, scraping, dataset collation and inventories; adversarial reviews, always verified and triaged by GPT | Production code, architecture decisions, independent approval of its own output or merge decisions |

## Rules

1. **Cross-provider review (D-28):** every PR is reviewed by a model from a different provider than its author.
   Claude-authored code → GPT 6.2 Sol reviews. GPT-authored documents and code → Claude Opus reviews (D-54).
2. **Gemini output is never trusted on its own.** Anything Gemini produces (data, licence findings, summaries)
   is checked by GPT 6.2 Sol (D-86) before it is used or merged. Gemini supplies adversarial reviews, never merge decisions.
3. Merge gates in D-28 apply to everyone; reviewer approval does not override a failing gate.
4. GPU work: SSH for development, self-hosted GitHub Actions runner for GPU CI gates (D-43).
5. **Commit and push frequently (D-53):** small commits, pushed after every meaningful step.
6. **Adversarial review by Gemini (D-53):** every substantive PR or document also gets a Gemini review whose
   job is to break it: contradictions, wrong facts, missing edge cases, failure modes. Use the two Gemini
   accounts alternately. Gemini's findings are claims, not facts: GPT or the coordinator triages them
   (accept / reject with reason) before anything changes.
7. **At most two delegates at once (D-86).** Bias effort towards GPT; reserve Claude for the focused cross-provider reviews above.
