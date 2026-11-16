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

The gate runs from `main` (`trusted-gates.yml`) and checks, against the trusted matrix
`xtask/policy/review.toml`:

- **Independent review:** the author and reviewer vendors differ (vendors, not model
  names: two Anthropic models do not count) and the role pair is allowed, for example
  `code generation / Anthropic` reviewed by `independent code review / OpenAI`.
  Gemini never supplies independent review.
- **Adversarial review:** at least one report with `Reviewer role/provider: adversarial
  review / Google` cites a commit of the pull request. Every adversarial report on the
  pull request (by that role or a file name containing `adversarial`) must be triaged.
  Finding IDs are the first token of `###` (or deeper) headings, such as `ADV23R-01`.
- **Triage record** (GPT or coordinator, D-53), first 40 lines:

      - Triager role/provider: triage / OpenAI
      - Triaged report: `docs/requirements/reviews/<adversarial report>.md`
      - Reviewed head: `<full 40-character commit SHA>`

  and a table with one row per finding: `| ADV23R-01 | fixed | reason |`. Closing
  dispositions are fixed, rejected, deferred, duplicate and not-applicable; anything
  else (for example `accepted`) leaves the finding open.
- **Currentness:** the reviewed SHA (and the triage SHA) must be a commit of the pull
  request, and no tree entry the pull request owns (contents or file mode) may differ
  between it and the current head. No path is exempt, including the reviews directory
  (a file there can be compiled like any other, R3-03): reports land on `main`, never
  on the pull-request branch. Merging `main` into the branch keeps a review current.

After a report lands on `main`, re-run the trusted-gates run of the pull request.
