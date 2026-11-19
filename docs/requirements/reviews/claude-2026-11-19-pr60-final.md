# PR #60 final verification review: M1 open-data acquisition plan (2026-11-19)

- Author role/provider: OpenAI
- Reviewer role/provider: Anthropic
- Reviewed head: `5f21b9d05bbe5e70612e8416561c3e260ccba0ab`
- Previous approval: `f4ecfa2` (`claude-2026-11-19-pr60-verify.md`)
- Verdict: **request changes**

## Scope

[PR #60](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/60),
reviewing only `git diff f4ecfa2 5f21b9d`. That diff covers:

- merge `8511433`, which brings main up to `a266a9b` (three review reports from main);
- `5f21b9d`, which changes `docs/research/m1-acquisition-plan.md` and
  `docs/requirements/STATUS.md`.

The reports that came in through the merge are already on main and were not
re-reviewed. I checked the plan changes against
`triage-gemini-2026-11-19-pr60.md`, the Gemini report and committed evidence:
`spikes/data/sources.tsv`, `spikes/data-gathering/semo-2024/manifest.toml`,
`COVERAGE.md` and `m1-data-inventory.md`. I also checked the live Elexon OpenAPI
(`data.elexon.co.uk/swagger/v1/swagger.json`), reading only the specification and
no market data.

I ran no builds. `git diff --check f4ecfa2 5f21b9d` is clean. CI results are from
`gh pr checks 60` and the job log at this head.

## Triage classifications

| Finding | Triage | My assessment |
|---|---|---|
| ADV60-01 MID in SRC-43 | VALID-BLOCKING | **Agree; fixed.** See below. |
| ADV60-02 SEMOpx vintage | INVALID | **Agree, INVALID.** |
| ADV60-03 Elexon filters | VALID-FOLLOW-UP | **Agree; addressed.** |
| ADV60-04 2024 register gates | VALID-FOLLOW-UP | **Agree; addressed.** |
| ADV60-05 Pink Sheet rights | INVALID | **Agree, INVALID.** |

### ADV60-02: genuinely INVALID

Gemini claimed that `Lookback_Market October2026.zip` holds only late-2026 prices.
That mistakes the publication vintage for delivery coverage. The committed evidence
contradicts the claim:

- `semo-2024/manifest.toml`, dataset `semopx-ex-ante-market-lookback-2021-2026`,
  pins the same URL, 39,165,432 bytes and SHA-256 `086eaa3e…317e037d`. Its
  description gives coverage from 1 January 2021 to 31 October 2026.
- `COVERAGE.md` reports 2024 counts for the same hash, for example 8,784 DAM
  hours, with IDA1/2/3 auction windows.

I did not re-inspect the private ZIP; the triager reports that they did. The
committed manifest and coverage report are enough to refute the finding.

The finding's other worry, contaminating the holdout with 2026 prices, does not
hold. Rows are selected by delivery time. Using a November 2026 publication for
2024 is a reconstruction, and the D-70 three-clock rule (plan lines 90–92) already
says so. Replacing the pinned archive with an unevidenced "2024" URL would be
worse. N2 below is an optional wording improvement, not a reclassification.

### ADV60-05: genuinely INVALID

The finding claims SRC-37b is misclassified, but the plan already contains the
protection Gemini asks for:

- Line 330 tags the source open-commercial "subject to additional/third-party
  terms". It is sample-only "only after resource/series exclusions are checked",
  and the line says: "Do not presume every quoted third-party price is
  redistributable."
- Profile E (line 83) requires third-party price rights.

Gemini named no selected series or restrictive metadata that the plan ignores, and
`sources.tsv` holds no Pink Sheet payload. The report itself rates the finding
"Speculative".

MID is different, and the triage handles that difference correctly:

- **MID:** there was already a research-only designation (`sources.tsv:46`). The
  BMRS licence explicitly excludes third-party rights, and MID is by definition
  data from appointed exchange providers.
- **Pink Sheet:** the World Bank affirmatively designates the dataset CC BY 4.0,
  and its terms say that exceptions are flagged in dataset or indicator metadata.

General third-party contribution alone does not overturn a publisher designation.
The series gate is the right control. N3 below is optional hardening.

### ADV60-01: fixed

SRC-43's rights are now set per stream (line 358). MID is research-only, PC-5,
never. Any inseparable raw bundle containing MID is research-only and never. The
existing capture's blanket BMRS manifest label is explicitly not MID clearance.
The MID table row (line 368) and P5 (lines 602–603) repeat the restriction.

The register totals were recounted: there are 43 rights lines, with 22 wholly
open-commercial, 20 wholly research-only and 1 mixed (SRC-43). This matches
lines 21–23. The other BMRS streams keep open-commercial and sample-only, which is
correct: `sources.tsv` separately clears those samples.

### ADV60-03: addressed

I checked the new paragraph (lines 383–394) against the live OpenAPI:

- BOALF, BOD, PN, MELS and MILS take only `from`, `to`, `settlementPeriodFrom`,
  `settlementPeriodTo` and a BMU filter. None of them has `publishDateTimeFrom/To`.
  FUELHH, REMIT and interconnector outturn do have it, so the earlier rows remain
  correct.
- The response schemas match the plan. MELS and MILS carry `notificationTime` and
  `notificationSequence`, BOALF carries `acceptanceTime`, and PN and BOD have no
  equivalent field.

Delivery-day batching, reconstruction-only admission and the warning not to read
notification times as publication evidence are all correct.

### ADV60-04: addressed

P1 (lines 567–575) now does the following:

- isolates current SRC-17/18/19 captures;
- requires release identity, hash and evidenced 2024 status and effective dates
  before a record is admitted to the holdout;
- allows a later release only as a declared reconstruction of proven 2024 state;
- keeps the D-70 requirement of publication evidence for as-of use.

P2 (lines 579–582) repeats the November 2024 LTDS gate, which is consistent with
SRC-05 line 136. No URLs are invented.

### Verification follow-up N1 (converter count and cable length)

SRC-32 (line 282) and the Q-P-03 paragraph (lines 519–522) add evidenced
converter counts and cable route lengths in km, with route definitions and
effective dates. Unknowns are preserved and straight-line distance is not used as
a substitute. The text explicitly does not adopt a fallback formula. The plan
correctly does not cite D-80, because D-78 to D-85 are not yet on main (the
decisions log ends at D-76). The approval-status rewrites therefore remain
deferred, as my earlier review allowed.

## Findings

### B1. [Blocking, regression] STATUS.md cites decisions that do not exist on main, so the documentation gate fails

At `docs/requirements/STATUS.md:21` the new STATUS entry says: "D-78–D-85 are
still absent from main; their approval-status rewrites and citations remain
deferred."

The documentation-gates job (run 44462237066, step "Links and traceability")
fails at this head:

```
FAIL: references
docs/requirements/STATUS.md:21: cites unknown decision D-78
docs/requirements/STATUS.md:21: cites unknown decision D-85
```

At `f4ecfa2` every `ci` job passed, so this is new in the reviewed diff. The
sentence that explains why the plan does not cite D-78 to D-85 itself cites them.

**Fix:** remove the D-IDs. For example: "The Q-P-01–Q-P-08 owner decisions recorded
in PR #87 are not yet on main; their approval-status rewrites and citations remain
deferred." Then re-run CI. This is a one-line wording change, and it does not need
a further content review beyond confirming that the diff touches only that line.

### N1. [Non-blocking] PN filter wording is slightly imprecise

Lines 385–386 say that with the period parameters, BOALF, BOD, MELS and MILS use
settlement dates, and that "PN resolves the datetime to a settlement period".
According to the PN route's own description, a supplied settlement period "will
take precedence over the time portion of the datetime". With the query shown, PN
therefore also filters by settlement date and period. The batching conclusion is
unaffected. Optionally fold PN into the same clause.

### N2. [Non-blocking, optional] Make SRC-44's archive vintage explicit

The plan's introduction (lines 12–14) mentions the SEMOpx hash and 2024-row check,
but SRC-44 (line 401) still names only the "October 2026 lookback ZIP". A short
clause in SRC-44 would stop the filename from misleading a future reader, as it
misled the adversarial reviewer. It could say that the file is a cumulative
2021–2026 lookback published in November 2026, and that its 2024 rows are
reconstruction-only under D-70.

### N3. [Non-blocking, optional] State the disposition when a Pink Sheet series fails clearance

The triage says that a selected series which fails clearance keeps a
research-only/never disposition. Line 330 says to withhold such series, but it
does not name that disposition. Adding the four words would close the gap.

### N4. [Non-blocking, follow-up] The inventory still carries the umbrella label

`m1-data-inventory.md:72` still tags all of SRC-43 as `open-commercial`. The plan
documents its override (lines 21–23), so nothing in this PR is wrong. A later
inventory edit should annotate MID as research-only so that the two documents do
not drift apart.

## CI at the reviewed head

From `gh pr checks 60`:

- **Failing:**
  - documentation gates (B1);
  - `independent-review`, expected before this report;
  - `merge-gates`, which depends on the other two.
- **Passing:** attribution, repository policy, licences and policy-approval.
- **Still pending when checked:** provenance and the Rust jobs.

## Verdict

**Request changes.** The substantive corrections are sound:

- The triage classifications are correct, and findings 2 and 5 are genuinely
  INVALID.
- MID is correctly downgraded per stream, and the register totals are consistent.
- The Elexon filter statements match the publisher OpenAPI.
- The 2024 admission gates and the interconnector metadata fields have no
  regressions.

The only blocker is B1, the STATUS citation that breaks the documentation gate.
Once that one line is fixed and CI is green, this review's content findings would
support approval of the corrected head.
