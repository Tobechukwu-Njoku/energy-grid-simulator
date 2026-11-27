# Triage of PR #60 adversarial review

- Triager role/provider: triage / OpenAI
- Triaged report: `docs/requirements/reviews/gemini-2026-11-19-pr60-adversarial.md`
- Reviewed head: `f4ecfa2ebf17e364c4cc3071198c00c54def1256`

Checked all five findings against the plan at the reviewed head,
`spikes/data/sources.tsv`, the existing acquisition manifests and coverage
reports, accepted D-56/D-70 and ADR 0002, and publisher documentation on
2026-11-19 UTC. Plan line references below refer to that head. `ADV60-01` through
`ADV60-05` are local aliases for the report's numbered Findings 1 through 5;
the original report does not assign finding IDs.

No builds or tests were run. Network reads were publisher pages, terms and the
Elexon OpenAPI, not new operational dataset acquisitions. For Finding 2, I
independently hashed and inspected the already-acquired archive outside git;
no raw research-only data is added to this review. Publisher links establish
only the facts described below, not blanket commercial clearance or as-of
availability. This publication changes only this file; the end-of-session
STATUS update belongs with the corrective change on PR #60.

## Dispositions

| Finding | Disposition | Classification | Reason |
|---|---|---|---|
| ADV60-01 | deferred | VALID-BLOCKING | SRC-43 applies an open-commercial/sample-only label to MID despite its existing research-only classification and unverified exchange rights. Mandatory correction on PR #60 before merge; not waived or deferred beyond that PR. |
| ADV60-02 | rejected | INVALID | The October 2026 archive is a cumulative 2021–2026 release containing 2024 DAM and IDA1/2/3, not an October-only delivery extract. Publication vintage and delivery coverage must not be conflated. |
| ADV60-03 | deferred | VALID-FOLLOW-UP | The operational routes have delivery/settlement filters, not publication filters. Make that explicit per route; existing reconstruction-only and freeze controls already prohibit the alleged as-of claim. |
| ADV60-04 | deferred | VALID-FOLLOW-UP | P1's preference for historical registers merits an explicit 2024 fleet-admission gate. The broader claim of automatic contamination is unsupported: current captures are labelled and SRC-05 already forbids substitution for November 2024. |
| ADV60-05 | rejected | INVALID | SRC-37b already conditions sample admission on resource/series-specific third-party checks. The report identifies no excluded series that bypasses that condition; general third-party risk is not evidence of misclassification. |

Here `deferred` for ADV60-01 records outstanding mandatory remediation at the
reviewed head, not permission to merge it. The two VALID-FOLLOW-UP items are
non-blocking contract clarifications under existing decisions, not new defaults.
They can be addressed with the blocking correction without waiting for
D-78 through D-85.

## Finding verification

### ADV60-01 — VALID-BLOCKING: MID inside SRC-43

The plan's SRC-43 rights line at 354 says `open-commercial`, BMRS licence and
`sample-only`; its MID row at 364 contains no overriding restriction. That
conflicts with `spikes/data/sources.tsv:46`, where `market-index-day.json` is
`research-only`. The endpoint differs (the older market-index route versus
`/datasets/MID/stream`), but both serve the exchange-derived Market Index Data
product; a different transport is not additional rights clearance.
`spikes/data/REPORT.md:354,581` already records unconfirmed exchange rights and
no committed price excerpt. Conversely, the registry separately marks the BMU,
demand, fuel, bid/offer and acceptance samples open-commercial; there is no
basis to extend MID's restriction to all other BMRS streams indiscriminately.

The publisher's [BMRS licence](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/)
permits commercial exploitation of licensed BMRS data but expressly excludes
third-party rights Elexon cannot license. Its [MID definition](https://www.elexon.co.uk/bsc/settlement/market-index-data/)
describes information received from appointed Market Index Data Providers.
Neither page supplies the missing exchange-rights grant. The report overstates
the evidence when it asserts that specific exchange agreements definitively
prohibit this reuse: those agreements were not established here. The narrower
verified defect is sufficient under D-56: absent clearance, the existing
research-only designation cannot be promoted by a bundle label.

Required fix: override SRC-43 at stream level. MID and any inseparable mixed
raw extract remain research-only/never; separately evidenced BMRS streams may
retain open-commercial/sample-only. Make manifest inheritance and the register
totals consistent, and prevent P5 from treating MID as an ordinary open sample.
Do not alter the already-conservative `sources.tsv` entry or claim that the
existing bulk manifest's umbrella tag establishes MID rights.

### ADV60-02 — INVALID: SEMOpx archive vintage

The source registry has an I-SEM interconnector-flow sample, not this price
archive; `sources.tsv` is not evidence for its delivery coverage. The relevant
record is `spikes/data-gathering/semo-2024/manifest.toml`, dataset
`semopx-ex-ante-market-lookback-2021-2026`. It pins the exact publisher URL in plan line
385, 39,165,432 bytes, and SHA-256
`086eaa3e21e4589c9ee4b6fb7b42742c3197d17e1977c3d63aa9bb44317e037d`.
It explicitly describes delivery coverage from 1 January 2021 through
31 October 2026. The companion `COVERAGE.md` reports 2024 counts of 8,784 DAM,
17,568 IDA1, 8,784 IDA2 and 4,392 IDA3 observations, with auction-window and
imbalance-gap qualifications.

I independently matched that SHA-256 against the private cached ZIP. Its sole
member is `Lookback_Market October2026.xlsx`; the `auctions_to` worksheet
contains 1 January 2024 UTC delivery rows for DAM, IDA1, IDA2 and IDA3.
This directly falsifies the claim that it contains only late-2026 prices.
The [publisher library](https://www.semopx.com/market-data/document-library)
does not expose the exact archived link in the HTML read, just as the plan
already warns. The [SEMOpx disclaimer](https://www.semopx.com/disclaimer)
does not establish open-commercial rights, so research-only/never remains
appropriate.

A November 2026 publication can contain 2024 deliveries and support a labelled
reconstruction; it cannot by itself prove the data were available in 2024.
The plan's three-clock rule at 86–89 and ADR 0002 require exactly this
distinction. Keep the pinned archive and rights/access caveat; do not replace
it with an invented 2024 archive URL. No fresh full-year recount or current
downloadability claim is made by this triage.

### ADV60-03 — VALID-FOLLOW-UP: Elexon delivery versus publication filters

The [live publisher OpenAPI](https://data.elexon.co.uk/swagger/v1/swagger.json),
`GET /datasets/{BOALF,BOD,PN,MELS,MILS}/stream`, lists `from`, `to` and
settlement-period parameters but no `publishDateTimeFrom/To` parameters.
BOALF/BOD document inclusive `TimeFrom` filtering, with settlement-date/period
interpretation when period parameters are supplied. MELS/MILS document
effective-time or settlement-date/period filtering; PN resolves the supplied
datetime to a settlement period. Thus the report is right about the missing
publication-time query, but its description of every route as exclusively
`TimeFrom` filtering is too broad. The plan itself passes period parameters.

Response schemas also distinguish evidence: MELS/MILS expose
`notificationTime` and `notificationSequence`, BOALF exposes `acceptanceTime`,
and PN/BOD expose no equivalent publication timestamp. A notification or
acceptance timestamp is not automatically proof of public availability or a
complete historical revision archive. The registry's open-commercial BOD/BOA
samples describe rights, not availability at a forecast origin; they cannot
supply the missing time evidence.

Plan lines 366–378 should state these limitations directly. However, the
governing D-70 rule at 86–89 already requires reconstruction when historical
publication evidence is unknown, and P7 at 590–592 expressly denies a forecast
validation claim from this subset. ADR 0002 at 88–95 permits such reconstructed
research; it does not require every historic API to provide publication filters.
The report therefore does not demonstrate its alleged D-70 violation.

Follow-up: use delivery-day batches for these five streams, retain available
notification/acceptance metadata without overclaiming its meaning, and require
independent archived publication/revision evidence before as-of admission.
Do not fabricate publication query parameters or describe these five streams
as publication-window captures in P5. This is safe to clarify now under D-70.

### ADV60-04 — VALID-FOLLOW-UP: P1/P2 vintage-admission wording

The cited dates are real: plan lines 170,177,184 advertise 2026 TEC, REPD and
DUKES releases, while line 135 advertises May 2026 SEPD tables. The existing
registry likewise pins 2026 TEC/REPD/DUKES captures, not 2024 archive locks.
Current publisher [REPD](https://www.gov.uk/government/publications/renewable-energy-planning-database-quarterly-extract)
and [DUKES](https://www.gov.uk/government/statistics/electricity-chapter-5-digest-of-united-kingdom-energy-statistics-dukes)
pages still advertise the quoted resources. The [TEC catalogue](https://api.neso.energy/api/3/action/package_show?id=cbd45e54-e6e2-4a38-99f1-8de6fd96d7c1)
has already advanced its mutable resource to 16 November 2026, demonstrating
why a catalogue URL alone cannot be a historical lock. These sources' open
licences govern reuse, not historic fleet validity. SSEN's metadata endpoint
returned HTTP 403 rather than catalogue JSON during this check; no new archived
resource identity is asserted.

The report omits stronger contrary text. SRC-05 line 136 explicitly requires
November 2024 tables and says not to substitute current URLs. Lines 173,180,187
require commissioned/effective dates and warn that contracted capacity or
planning status is not an operating fleet. Profile C requires coverage,
publication evidence and knowledge mode; profile N requires asset status and
effective dates. P0 separates the holdout from current/forward releases, and
P1 labels current captures as current. Acquisition is not automatic admission.
Consequently the assertion that following P1/P2 necessarily inserts future
plant or that SRC-05 lacks a historical gate is not established.

There is nevertheless a useful local gap: P1's wording merely prefers
historical registers. Strengthen it to require evidenced 2024 asset-effective
coverage, release identity and hash before fleet admission; isolate current
captures, and leave absent historic status unresolved. A later-corrected table
may serve a declared reconstruction only if its relevant 2024 status is proven;
it must not contribute 2026 planned/commissioned state to 2024. Repeat the
existing November 2024 network gate in P2. Exact archive discovery belongs to
acquisition; do not invent URLs or prohibit the reconstructions D-70 allows.

### ADV60-05 — INVALID: Pink Sheet third-party rights

`sources.tsv` contains no Pink Sheet series or payload, so it supplies neither
series clearance nor evidence of an improperly committed sample. Plan lines
324–327 already condition the open-commercial candidate on additional and
third-party terms: sample-only applies only after resource/series exclusions
are checked, and quoted third-party prices are not presumed redistributable.
Profile E also requires third-party price rights, and the global sample
contract at 51–61 requires rights checks before admission.

The [Commodity Markets page](https://www.worldbank.org/en/research/commodity-markets)
advertises the historical monthly/annual workbooks and links its dataset terms.
The short data.worldbank.org terms URL returned an access challenge in this
check. The publisher's accessible [current dataset terms](https://www.worldbank.org/ext/en/legal/terms-conditions/datasets)
confirm CC BY 4.0 unless labelled otherwise, with additional conditions and a
section on third-party exceptions: relevant restrictions are carried in dataset
or indicator metadata. The [public-licence catalogue](https://datacatalog.worldbank.org/public-licenses)
also confirms the default and differently licensed exceptions. The older legal
URL redirected to general site terms, which are not a replacement for dataset
terms.

The report correctly identifies a risk but names no selected series or
restrictive metadata that the plan ignores. General terms do not prove every
series clear; equally, third-party contribution alone does not prove a
restriction. Retain the existing series-specific admission gate. If a selected
series fails clearance, withhold its sample and preserve a research-only/never
disposition pending confirmation; the current conditional candidate label is
not an authorisation to redistribute it. No new rights determination is made.

## Blocking fixes

- Correct ADV60-01 on PR #60 before merge: MID research-only/never, explicit
  stream-level rights, conservative mixed-bundle treatment and matching totals.
- ADV60-03 and ADV60-04 are non-blocking clarifications that may accompany that
  fix. ADV60-02 and ADV60-05 require no replacement source or blanket re-tagging.
- From `claude-2026-11-19-pr60-verify.md` N1, acquire converter counts and cable
  route lengths as evidenced link metadata now, without adopting D-80's pending
  fallback coefficients. Defer the three approval-status rewrites and new
  D-78–D-85 citations until those decisions merge on main.
- Merge main into the plan branch, update STATUS there, and obtain fresh
  exact-head review after the corrections. This triage is not approval of a
  future head and does not waive any owner, rights or validation gate.
