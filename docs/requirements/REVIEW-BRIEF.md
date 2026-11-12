# Review Brief — for an independent reviewer model

_Prepared 2026-11-12._

You are reviewing the **requirements-engineering work so far** for a GB + Ireland energy grid
and market simulator. No code exists yet. Read in this order:

1. `/AGENTS.md`: repo rules
2. `STATUS.md`: summary and next steps
3. `03-decisions-log.md`: the owner's decisions (source of truth; D-5 superseded by D-23)
4. `02-questionnaire.md`: answered `[x]` and open `[ ]` questions
5. `05-risks.md`: identified risks
6. `../architecture/stack-candidates.md`: tentative stack (not decided)
7. `../research/prior-art.md`: reusable tools and data

## What to look for

- **Contradictions** between decisions (e.g. scope vs MVP, SaaS vs licences, Rust-only vs reused tools).
- **Missing questions** a requirements engineer should ask for a power-system + market simulator.
- **Missing or underrated risks**, especially scope, data availability, validation, licensing.
- **Stack concerns:** wrong or outdated tool choices, maturity of Rust crates, GPU solver fit.
- **Factual errors** in prior-art.md (tools, licences, data sources).

## Output format

Write findings to `docs/requirements/reviews/<model>-<date>.md` as a numbered list, each with
severity (high/med/low), the file/ID it concerns, and a suggested change. **Do not edit the
decisions log**; only the owner makes decisions.
