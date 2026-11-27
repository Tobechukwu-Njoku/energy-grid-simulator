- Author role/provider: research / OpenAI
- Reviewer role/provider: document review / Anthropic
- Reviewed head: `f4ecfa2ebf17e364c4cc3071198c00c54def1256`
- Verdict: **approve**

# PR #60 verification review: M1 open-data acquisition plan (2026-11-19)

Scope: [PR #60](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/60)
at the exact head above. This review checks the author's response to
`claude-2026-11-18-pr60.md`, which reviewed head `ea75ecb`. Since then the branch
has merged main at `b13145f` (`b3b2288`) and added two revisions (`be4eb00` and
`f4ecfa2`). At the reviewed head, the PR changes
`docs/research/m1-acquisition-plan.md` (592 lines) and
`docs/requirements/STATUS.md`.

I also checked the PR against the owner decisions D-78 to D-85 (Q-P-01 to Q-P-08,
accepted 2026-11-19). They are recorded in open
[PR #87](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/87) at
`49c9189` and are not yet on `main`.

Line references are to the plan at the reviewed head. This is a document review. I
built nothing and did not modify PR #60. I checked new URLs with HEAD requests, or
1 KiB ranged GETs where HEAD was refused. The only payload I fetched was the
483,310-byte IANA tzdata archive, into temporary storage, to check its hash.

## CI at the reviewed head

From `gh pr checks 60`:

- Every `ci` job passed (run 44451130137), as did `merge-gates`, `policy-approval`,
  attribution, documentation, provenance and repository policy.
- `independent-review` failed, as expected before this report: no approving review
  covered this head, and no Google adversarial report cites a commit of the PR.
  This report does not supply the adversarial review.
- An earlier run (44451019349) was cancelled by the concurrency group.
- The branch merges cleanly into `main` and, by `git merge-tree`, into PR #87.

## Verification of earlier findings

| # | Earlier finding | Result |
|---|---|---|
| 1 | PR #58 sources ignored | **Fixed.** A new section, "PR #58 candidate reconciliation" (line 465), gives every #58 candidate a disposition (added or deferred), a D-56 tag and a raw-commit rule, and keeps register counts separate from candidate counts. Details are below the table. |
| 2 | Raw-commit permission contradicts the data gate | **Fixed.** Lines 50–58 define **sample-only** as registered payloads under `spikes/data/samples/*` with the provenance record and sample-source mapping. Metadata paths never hold payloads, and any broader path needs an owner policy change. All 23 open-commercial register entries say "sample-only". All 20 research-only entries say "never", and no "yes, conditionally" remains. The register still has 43 entries. |
| 3 | Raw-store host unspecified | **Fixed.** Line 43: the store is on the D-65 target or a declared private ADR 0004 store, never the coding container. Host identity, root or namespace and access scope go in profile C. P0 blocks bulk fetching until a store is declared. D-62 runs may write only to their authorised staging namespace. |
| 4 | Decision traceability | **Fixed.** The governing list cites D-30, D-56, D-57, D-60/D-71, D-62, D-65, D-70 and D-72, each with its role. |
| 5 | SRC-36b admission | **Fixed.** Line 312: SRC-36b cannot be admitted even as research-only until project research use is confirmed. SRC-37a/37b and DESNZ are the default cost candidates. |
| 6 | tzdata pin | **Fixed.** Candidate 8.3 makes 2026f canonical for this plan and PR #59 F-TIME, superseding 2024b. I checked the SHA-256 `01b4a91c…5a3044d3d`, and it matches the archive. PR #59 line 59 carries the same lock. |
| 7 | Elexon as-of filters | **Fixed.** Lines 361 and 363 add `publishDateTimeFrom/To` windows and publication vintages for FUELHH and the interconnector flows. |
| 8 | STATUS conflict and stale base | **Fixed; the D-77 rebuttal is accepted.** The header records `b13145f`, and STATUS keeps both histories. D-77 is absent from `03-decisions-log.md` on `main`, and PR #87 confirms it is reserved for PR #81. |

Details for finding 1:

- ETYS 2024 Appendix B is the primary historical edition (research-only, never),
  and the 2025 edition is labelled as forward.
- The November 2024 LTDS editions are the holdout targets. Their exact archived
  identities remain an admission blocker, and the plan does not invent them.
- The EirGrid/SONI 2024 report is cited with 75% SNSP, 23,000 MW·s, and the
  3 NI + 4 ROI rule from 7 April 2024, as research-only R evidence.
- PV_Live, SEAI/ESB, DESNZ costs 2023, Ofgem/SEM loss material, Met Éireann
  stations, the WRC holidays page and the exact Ofgem 2024/25 RO buy-out notice
  are all added.
- ERA5 multi-level and MIDAS are deferred because they need credentials. CGMES,
  Frictionless and the internal schema are deferred because they are not payload
  sources.

URL spot-check at review time:

| Source | Result |
|---|---|
| EirGrid 2024 PDF, ETYS `document/351916`, PV_Live, Ofgem BritNed 2024 decision, SEM Committee publications, DESNZ costs 2023, Met Éireann historical data, WRC holidays, Ofgem RO 2024/25 notice | HTTP 200 |
| SEAI renewables statistics, LCCC CfD register | HTTP 403 to an automated client |

The plan claims no status for the two 403 pages and keeps rights checks as a
fetch-time gate, so this does not change the result.

## Consistency with D-78 to D-85

The acquisition contracts agree with the decisions:

- Loss coefficients are sought as directional standby, linear and quadratic terms
  (D-80 primary path).
- ERA5 at 100 m plus lower levels is sought, with the fallback shear policy
  retained (D-83).
- Original cost cadence and publication vintages are kept before resampling
  (D-85).
- PV_Live is acquired for profile scaling (D-79).
- The merit order cannot replace baseline unit commitment (D-81).
- Original rating types are kept for the inferred method (D-78).

### N1. [Low, required follow-up] Three status statements contradict D-78 to D-85

The following text says the proposals are not approved. The owner has now approved
them.

- Lines 507–508: "These existing questionnaire assumptions remain unapproved; this
  reconciliation introduces no owner decision or new proxy default." This is
  contradicted by D-78, D-80, D-82 and D-85, which the same paragraph discusses.
- Candidate 3.3 (line 485): "Q-P-02 interpolation/scaling is an open assumption".
  D-79 approves it. The output remains imputed rather than metered, which the
  line correctly keeps.
- Candidate 6.2 (line 491): "Q-P-05 defaults remain unapproved operating-fleet
  proxies". D-82 approves them as technology proxies for exploratory dispatch,
  migrating to unit-specific calibration.

D-80's fallback (0.75% per converter plus 0.30% per 100 km of cable) also needs, for each
link, the converter count and the route length of the cable. Add both to the
loss-acquisition contract (lines 501–503), so that the fallback can be computed
from evidenced values rather than assumed ones.

These are status-wording and contract-completeness fixes. They do not change any
source, tag or raw-commit rule. By the author's own valid rule, D-78 to D-85 cannot
be cited before PR #87 merges. So do one of the following:

- rebase onto `main` after PR #87 merges and make the change before merging this
  PR, which needs a fresh exact-head review; or
- merge this head and make the change in an immediate follow-up PR that cites the
  decisions.

Either way, the change is required, because these lines contradict the
decisions.

## Verdict

**Approve.** All eight earlier findings are fixed or validly rebutted. CI is green
apart from the review-evidence gate that this report addresses. N1 is a required
follow-up that depends on PR #87 merging. It does not block this head, because it
does not change any acquisition, rights or raw-commit rule.
