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

### Machine-checked header for code-review reports

The `independent-review` CI gate (`cargo xtask review`, ADR 0001, NFR-3.AC3) reads review
reports from `main`, never from the pull request. A code review counts only if its first
40 lines contain these fields, one per line:

    - Author role/provider: <role> / <provider>
    - Reviewer role/provider: <role> / <provider>
    - Reviewed head: `<full 40-character commit SHA>`
    - Verdict: **approve**   (or approved, merge-ready; anything else blocks)

The providers must differ, the reviewed SHA must be a commit of the pull request, and no
path the pull request changes (other than review reports) may differ between that commit
and the current head. Adversarial reports (file name containing `adversarial`) must cite a
full commit SHA of the pull request. After a report lands on `main`, re-run the gate by
pushing or editing the pull request. Triage of adversarial findings is still confirmed by
the coordinator.
