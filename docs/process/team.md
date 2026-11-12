# Team & Roles

_Source: D-45, D-49, D-28, D-22. Owner checks milestones; agents plan, build, review and merge._

| Model | Role | Use for | Do not use for |
|---|---|---|---|
| **Claude Opus 5.5** (medium effort) | Primary engineer | Most coding: Rust engine, backend, frontend, tests, data pipelines | Reviewing its own PRs |
| **GPT 6.1 Sol** | Architect & reviewer | ADRs, specs, requirements, task breakdown, code review of Claude's PRs, validation reports | Bulk routine work |
| **Gemini Flash 3.8** (high) | Assistant | Data retrieval and collation (datasets, licences, docs), formatting, boilerplate, first-pass summaries | Anything complex or needing scrutiny: engine/solver code, architecture, merge decisions, reviews |

## Rules

1. **Cross-provider review (D-28):** every PR is reviewed by a model from a different provider than its author.
   Claude-authored → GPT 6.1 Sol reviews. GPT-authored → Claude reviews.
2. **Gemini output is never trusted on its own.** Anything Gemini produces (data, licence findings, summaries)
   is checked by Claude or GPT before it is used or merged. Gemini never reviews or merges.
3. Merge gates in D-28 apply to everyone; reviewer approval does not override a failing gate.
4. GPU work: SSH for development, self-hosted GitHub Actions runner for GPU CI gates (D-43).
