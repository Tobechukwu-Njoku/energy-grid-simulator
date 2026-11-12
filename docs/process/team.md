# Team & Roles

_Source: D-45, D-49, D-51, D-53, D-28, D-22. Owner checks milestones; agents plan, build, review and merge._

| Model | Role | Use for | Do not use for |
|---|---|---|---|
| **Claude Opus 5.5** (medium effort) | Code generation only (D-51) | Writing code: Rust engine, backend, frontend, tests, data pipelines | Research, reviews, specs, verification (→ GPT) |
| **GPT 6.1 Sol** | Default for everything except code generation (D-51) | Research, ADRs, specs, requirements, task breakdown, code review, **verifying all Gemini output**, validation reports | Bulk routine work (→ Gemini) |
| **Gemini Flash 3.8** (high) | Assistant | Data retrieval and collation (datasets, licences, docs), formatting, boilerplate, first-pass summaries | Anything complex or needing scrutiny: engine/solver code, architecture, merge decisions, reviews |

## Rules

1. **Cross-provider review (D-28):** every PR is reviewed by a model from a different provider than its author.
   Claude-authored → GPT 6.1 Sol reviews. GPT-authored → Claude reviews.
2. **Gemini output is never trusted on its own.** Anything Gemini produces (data, licence findings, summaries)
   is checked by GPT 6.1 Sol (D-51) before it is used or merged. Gemini never reviews or merges.
3. Merge gates in D-28 apply to everyone; reviewer approval does not override a failing gate.
4. GPU work: SSH for development, self-hosted GitHub Actions runner for GPU CI gates (D-43).
5. **Commit and push frequently (D-53):** small commits, pushed after every meaningful step.
6. **Adversarial review by Gemini (D-53):** every substantive PR or document also gets a Gemini review whose
   job is to break it: contradictions, wrong facts, missing edge cases, failure modes. Use the two Gemini
   accounts alternately. Gemini's findings are claims, not facts: GPT or the coordinator triages them
   (accept / reject with reason) before anything changes.
