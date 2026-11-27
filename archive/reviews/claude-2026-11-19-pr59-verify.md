- Author role/provider: specification / OpenAI
- Reviewer role/provider: document review / Anthropic
- Reviewed head: `aba7687ef43caa1fd92d0d0faf34f959c62d1822`
- Verdict: **request changes**

# PR #59 verification review: M1 acceptance-test specification (2026-11-19)

Scope: [PR #59](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/59)
at the exact head above. This review checks the author's response to
`claude-2026-11-18-pr59.md`, which reviewed head `c15fbe1`. Since that head, the
branch merged main at `b13145f` (`ad42c1c`, `235f7b2`) and added two revisions
(`1724312`, `aba7687`). At the reviewed head, the PR changes
`docs/requirements/m1-acceptance-tests.md` (772 lines),
`docs/requirements/02-questionnaire.md` and `docs/requirements/STATUS.md`.

I also checked the PR against the owner decisions D-78 to D-85 (Q-P-01 to Q-P-08,
accepted 2026-11-19). These decisions are recorded in open
[PR #87](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/87) at
`49c9189` and are not yet on `main`.

Line references are to `m1-acceptance-tests.md` at the reviewed head. This is a
document review. I did not build or execute anything and did not modify PR #59.
I downloaded the IANA `tzdata2026f.tar.gz` archive into temporary storage only to
check its hash.

## CI at the reviewed head

From `gh pr checks 59` and the run list for the head:

- `ci` run 44451112945: **success**. An earlier run, 44451018058, was cancelled
  by the concurrency group, which is why `gh pr checks` briefly showed failed jobs.
- `trusted-gates` run 44451113433: `policy-approval` passed. `independent-review`
  failed, as expected before this report. It found no approving review of this head
  and no Google adversarial report citing a commit of the PR. This report does not
  supply the adversarial review.
- The branch merges cleanly into `main`.

## Verification of the earlier findings

| # | Earlier finding | Result |
|---|---|---|
| 1 | Questionnaire ID collision with Q-P-01–08 | **Fixed.** The new items are section Q, Q-Q-1 to Q-Q-4. Section P is untouched. Every spec reference and the anchor `#q-m1-acceptance-oracles-and-staged-evidence` use section Q. |
| 2 | Not reconciled with PR #58 | **Fixed.** The branch contains `main` at `b13145f`. The standby-loss variant (line 196) attributes 1 MW / 0.5 MWh to a declared GB endpoint, and a duplicate at the SEM endpoint fails. The 0.88 inferred-rating variant (line 142) and the ohms-to-pu oracle (line 610: 132²/100 = 174.24 Ω, 17.424 Ω = 0.1 pu) are present. GAP-M1-4 to GAP-M1-7 cite the PR #58 candidates. The Irish rule figures in GAP-M1-7 match candidate 2.1. **However, see new finding N1:** two of these additions now conflict with D-78 and D-80. |
| 3 | Fixture placement vs the data gate | **Fixed.** Line 48: committed payloads only as registered samples under `spikes/data/samples/*`, or values built in memory or in temporary directories. Line 526 names `cargo xtask provenance --base --head` (range mode) as the AT-M1-24 scanner. |
| 4 | Binary64 exact-threshold boundary | **Fixed.** Line 635: the inclusive boundary uses the comparator's own binary64 threshold and its negation. The adjacent representable values are checked: the inner one passes, the outer one fails. The 0.99× and 1.01× cases keep the decimal oracle, and the `(a + tol) − a` construction is explicitly forbidden. |
| 5 | FX quote notation | **Fixed.** Line 208 says "0.80 and 0.90 GBP per EUR". |
| 6 | NI 275/110 kV not labelled | **Fixed.** There is an `ASSUMPTION:` at line 56, and Q-Q-4 is in the questionnaire. Synthetic NI cells cannot widen the approved denominator. |
| 7 | Calendar case vs fixture | **Fixed.** Line 282 covers all four 2023/2024 transitions; 26 March 2023 and 29 October 2023 are the correct last Sundays. Line 291 discriminates by UTC offset and fold, never by `is_dst`, and asserts +00:00 in winter and +01:00 in summer in both zones. |
| 8 | D-53/D-54/D-77 not cited | **D-53/D-54 fixed; the D-77 rebuttal is accepted.** D-77 is not in `03-decisions-log.md` on `main`, and PR #87 states that D-77 is reserved for PR #81. Under AGENTS.md rule 1, the author was right not to cite it. |
| 9 | Stale snapshot and date | **Fixed.** Line 4 gives 2026-11-19 against `b13145f`. STATUS keeps both histories. The rebuttal on the D-77 header is accepted for the same reason as finding 8. |

The canonical tzdata lock at line 59 is correct. The SHA-256 of
`https://data.iana.org/time-zones/releases/tzdata2026f.tar.gz` (483,310 bytes) is
`01b4a91cec5167ed48c11e8ab78f2048cead78ce94b0cafcad0ae8c5a3044d3d`, which matches.

## New findings

### N1. [Medium, blocking] Three oracles and the gap table contradict D-78, D-80 and D-82

These were written before the owner decided, so they are not authoring errors. As
they stand, though, an implementation that follows the decisions would fail these
acceptance cases.

- **AT-M1-2, inferred-rating variant (line 142), vs D-78.** The variant admits a
  universal 0.88 × post-fault proxy as *the* inferred method. D-78 chose option (b)
  over (a): asset-class proxies differentiated by cable or overhead line and by
  voltage class. The universal 0.88 multiplier is retained only as the
  sensitivity-sweep comparator.

  Required change: key the admitted proxy on construction type and voltage class.
  Use synthetic class multipliers, labelled as synthetic, because D-78 fixes no
  values. Record the D-78 method identifier. Make a missing class reject the
  record or leave it unready; it must never fall back silently to 0.88. Keep the
  88 MVA computation as an explicitly labelled sensitivity run. The AT-M1-7 rule
  (inferred coverage, never the verified numerator) remains correct.

- **AT-M1-5 (line 194) and GAP-M1-6 (line 760), vs D-80.** Line 194 says invalid
  variants, including a missing directional coefficient, "never receive an
  invented zero/default physical loss". GAP-M1-6 says "without inventing
  percentages". D-80 now authorises a default for exactly that case: where the
  coefficients are not known, a constant proxy of 0.75% per converter plus
  0.30% per 100 km of cable, labelled `ASSUMPTION: [Q-P-03]`.

  Required change: a missing coefficient either rejects, or receives exactly the
  D-80 fallback, labelled as a proxy. It never receives a silent zero or any other
  default. Add an oracle for the fallback. For example, 100 MW sent over two
  converters and 100 km of cable gives a loss of 100 × (2 × 0.0075 + 0.003) =
  1.8 MW and a receipt of 98.2 MW, or 50 / 49.1 / 0.9 MWh per half-hour.

  D-80 does not say what the percentages apply to (sending-end MW, additive or
  compounded per stage). The spec must state its reading as an `ASSUMPTION:`, and
  a questionnaire item may be needed if the reading matters for real links. The
  standby variant (line 196) is consistent with D-80's three-term primary path and
  can stay as written.

- **AT-M1-15 (lines 366 and 371) vs D-82.** "Omit heat rate" must reject ready
  status. D-82 supplies technology proxies for exploratory dispatch: CCGT 52%,
  £4/MWh VOM; OCGT 35%, £6.50/MWh VOM.

  Required change: split the variant. If the heat rate is missing and no proxy is
  declared, the record is rejected. If the D-82 proxy is applied, the record is
  admitted, labelled as an inferred `ASSUMPTION: [Q-P-05]` value, and never
  reported as observed. For the existing fixture, a CCGT proxy gives
  (30 + 50 × 0.2) / 0.52 + 4 = 1052/13 ≈ 80.923 GBP/MWh-electric.

- **GAP-M1-5 (line 759).** It says Q-P-02, Q-P-05 and Q-P-08 "remain unapproved
  candidate paths". They are now approved as D-79, D-82 and D-85. The line-647
  note says the decisions log "ends at D-76"; after PR #87 it ends at D-85, with
  D-77 still pending.

No contradiction found with D-81 (the MILP baseline is consistent with the NFR-2
certificates staged later), D-83 or D-84 (not exercised), or D-85. AT-M1-14 already
forbids later-published points in as-of interpolation. Optionally, AT-M1-15's
monthly fuel resampling could assert D-85 directly: two-sided interpolation in
reconstruction, release-time step-holding in as-of runs.

**Sequencing.** By the author's own valid rule (finding 8), D-78 to D-85 cannot be
cited until PR #87 merges. Merge PR #87 first. Then revise this PR, citing
D-78/D-80/D-82 in the affected cases and D-79/D-82/D-85 in GAP-M1-5. Citing the
decisions is required here only because these contradictions exist.

### N2. [Low] Questionnaire merge conflict with PR #87

`git merge-tree` of this head against PR #87 conflicts in
`02-questionnaire.md`. This PR deletes the trailing blank line after Q-P-08, and
PR #87 appends its `Answered` lines at the same place. Resolve it when rebasing
after PR #87. Separately, section Q is inserted *before* section P. That is
harmless, but appending it after P would keep the sections in letter order.

### N3. [Low] STATUS wording is out of date within the PR

In `STATUS.md`, the block at line 10 is headed "revised 2026-11-18 UTC", and line 20 lists
"New open Q-Q-1–Q-Q-3", but Q-Q-4 now exists. Update both in the same revision.

## Verdict

**Request changes.** All nine earlier findings are fixed or validly rebutted.
N1 is a real conflict with owner decisions D-78, D-80 and D-82 in the acceptance
oracles. Resolve it after PR #87 merges, together with N2 and N3. I expect to
approve once those changes are made.
