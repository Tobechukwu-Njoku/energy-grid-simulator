# PR #58 re-check — M1 deficit sources, round 2

Review date: **2026-11-15 UTC**. Reviewed [PR #58](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/58) at exact head **`67a9d3d84091dd329ae212f8f2333e59dca4cdbb`**, file `docs/research/m1-deficit-sources.md` (547 lines). All source-file line references below refer to that head. Compared with F-01–F-16 in the [first review](gpt-2026-11-15-pr58-deficit-sources.md), retained on `main` at `d84d37e`.

**Verdict: reject.** The revision makes substantial factual corrections, and all six requested spot-checks are supported by publisher evidence. However, the seven claimed assumption questions do not exist, several expressly requested link repairs were not made, and resource-specific rights and historical applicability remain overstated. The claim at line 528 that all sixteen findings are resolved is inaccurate. Correct the remaining findings below before treating this document as an implementation/provenance guide.

Scope: documentation, publisher pages, catalogue metadata, API specifications and reference-document text only. **No datasets were downloaded**, no authenticated acquisition was attempted, and no record-level completeness was tested. Public reference PDFs were inspected outside the repository; none is included in this PR. PR #58 was not changed. A proposed source may remain unacquired or historically unverified; that is acceptable when stated consistently. This review does not approve modelling defaults, close M1 deficits or confer commercial clearance.

## Disposition of all sixteen earlier findings

“Resolved” means the requested correction is made at candidate-document level, including an explicit admission of missing evidence where appropriate. It does not mean the underlying data deficit is closed. “Partial” identifies a remaining correction, including missing questionnaire registration.

| Earlier finding | Result | Evidence at reviewed head / remaining correction |
|---|---|---|
| F-01 — Irish minimum-unit date | **Resolved** | Lines 72–79 and 106 replace 17 July with 7 April, preserve 4 ROI / 3 NI, leave pre-April intervals unverified, and acknowledge additional local constraints. Joint report §3.3 independently rechecked. |
| F-02 — 0.88 rating proxy | **Partial** | Lines 53–54 remove the P27 derivation and disclose uncertainty and baseline restrictions. The question said to be logged is absent; even the inline Q-P-01 reference claimed in the response table is missing. R2-01. |
| F-03 — CIGRE loss proxy | **Partial** | Lines 232–237 remove the unsupported CIGRE attribution and disclose quadratic cable physics, but Q-P-03 is absent and “fixed plus linear” still misdescribes the formula. R2-01, R2-06. |
| F-04 — commercial versus physical losses | **Resolved** | Lines 197–237 separate deemed-volume factors, BSC allocation and physical losses, qualify BritNed's historical panel, and retain Nemo's numbers as unverified. Dated Irish TLAF application remains a candidate to inspect (line 222). This does not certify any ten-link physical model. |
| F-05 — Elexon TLM API | **Resolved** | Lines 205–213 cite the working swagger specification, correct stack route, nullable TLM and unverified annual coverage; P278's legal evidence is separate. Neither TLF nor complete delivering/offtaking panels are claimed acquired. |
| F-06 — Section T versions | **Resolved** | Lines 255–256 and 286 mark both version assignments unverified and acknowledge other releases; no replacement section-version calendar is invented. Historical Section T acquisition remains open. |
| F-07 — SEMOpx schedule / dated rules | **Partial** | IDA1 is corrected; timezone, price limits and Part B version have caveats, and continuous dispatch no longer replaces unit commitment. The schedule is still labelled a 2024 baseline without a dated procedure, and Q-P-04 is absent. R2-01, R2-05. |
| F-08 — microgeneration endpoints | **Partial** | Lines 150–175 use the publisher's rounded endpoints and counts, distinguish ROI/NI and disclose interpolation/scaling uncertainty. Q-P-02 is absent; summary row 3 omits ESB's restricted lineage. R2-01, R2-03. |
| F-09 — plant efficiencies / O&M | **Partial** | Lines 317–320 and 345 withdraw the unsupported DESNZ extraction and call exact defaults proposed technology proxies. Units are addressed at line 346. Q-P-05 is absent. R2-01. |
| F-10 — RO transition / mixed rights | **Partial** | Lines 328–338 correctly introduce the April transition and separate Ofgem/LCCC rights; row 6 is mixed. Monthly-price interpolation still lacks an assumption/as-of policy, and the EUA candidate lacks a specific EEX resource. R2-04. |
| F-11 — GB inertia dates | **Resolved** | Lines 100–107 support only the broad 140-to-120 reduction and explicitly leave 130 and both exact dates unverified; regional deficits remain open. |
| F-12 — links and coverage | **Partial** | Corrected NESO package IDs work; weekly/snapshot semantics and completeness caveats are present. ETYS and MIDAS links remain wrong, despite the claimed MIDAS repair; no MIDAS release containing 2024 is pinned. R2-02. |
| F-13 — internal implementation | **Partial** | Lines 460–506 and row 9 correctly use INT-01, N/A, pending status, opaque UUIDs, aliases and separate hashes. The crate is explicitly proposed. The ADR link is broken and the ENTSO-E reference still returns 404. R2-02, R2-07. The accepted ADR remains authoritative for canonicalisation, migration and knowledge-mode details beyond this abbreviated checklist. |
| F-14 — weather fallback | **Partial** | Lines 388–397 withdraw spatial recovery from national outturns, distinguish manufacturer curves from IEC classes, disclose synthetic reconstruction and require independent holdout treatment. Q-P-06 and Q-P-07 are absent. R2-01. |
| F-15 — tzdata / calendar | **Resolved** | Lines 436–456 and row 8 correct 46/50, cite the release date, qualify early forecast origins and explain statutory entitlement. Calendar bundling remains a recommendation, not an implemented lock. Artifact hashes and runtime database mapping are still implementation work. WRC licensing is separately unresolved under F-16. |
| F-16 — resource-specific rights | **Partial** | Ofgem/LCCC separation, CDS interactive acceptance and internal N/A are improved. The obsolete ERA5 licence link remains; WRC and Met Éireann grants are not tied to exact acquired artifacts, and summary rights are still inconsistent. R2-02, R2-03. |

## Requested publisher spot-checks

These are checks of the cited claims, not a historical data acquisition or completeness audit.

| Claim | Publisher evidence inspected on 2026-11-15 | Result |
|---|---|---|
| IDA1 closure at 17:30 D−1 | [SEMOpx Intraday Market](https://www.semopx.com/markets/intraday-market), “Order Book timelines”: IDA-1 closure **17:30 (D-1)**; IDA-2 **08:00 (D)**; IDA-3 **14:00 (D)**. | **Confirmed on the live page.** A dated 2024 procedure/time convention still needs establishing; R2-05. |
| ESB approximately 300 → 500 MW | [2023 distribution annual performance report](https://media.esbnetworks.ie/media/docs/default-source/publications/distribution-annual-performance-report-2023-accessibled434eedca532460d959ccfba328306eb.pdf), p.5: **78,400 connections**, approximately **300 MW** at end-2023. [2024 report](https://media.esbnetworks.ie/media/docs/default-source/publications/distribution-annual-performance-report-2024.pdf), pp.5, 64: **121,700**, approximately **500 MW** at end-2024. The detailed sections describe facilitated connection applications. | **Confirmed as rounded publisher microgeneration figures.** They do not independently establish a commissioned, PV-only intra-year fleet or an NI series. |
| £59.01 → £64.73/ROC | [Ofgem 2024/25 notice](https://www.ofgem.gov.uk/transparency-document/renewables-obligation-ro-buy-out-price-mutualisation-threshold-and-mutualisation-ceilings-2024-2025), historical table: **2023–24 £59.01**, **2024–25 £64.73**. It also confirms GB/NI obligation levels **0.469/0.184** and **0.491/0.192**, and the 26 March GB revision from 0.487. | **Confirmed.** January–March calendar 2024 uses the former obligation year; April–December uses the latter. These calendar slices are not the full obligation-year dates. Buyout remains a supplier compliance benchmark, not generator ROC revenue. |
| tzdata 2024b, 4 September 2024 | [IANA NEWS](https://data.iana.org/time-zones/tzdb-2024b/NEWS): `Release 2024b - 2024-09-04 12:27:47 -0700`. [LICENSE](https://data.iana.org/time-zones/tzdb-2024b/LICENSE): public domain unless specified, with BSD exceptions for named code files. | **Confirmed.** The source release exists; this does not establish a repository/runtime pin. |
| 140 → 120 GVA·s during 2024 | [NESO 2025 Operability Strategy Report](https://www.neso.energy/document/358116/download), p.7: “Last year” reduced minimum inertia from **140 to 120 GVAs**; p.24 repeats **140 GVA.s to 120 GVA.s**. | **Confirmed broadly.** These passages do not establish the 130 intermediate floor or exact implementation dates. |
| Elexon swagger URL | [Official specification](https://data.elexon.co.uk/swagger/v1/swagger.json) returned **HTTP 200**, valid JSON. It includes `/balancing/settlement/stack/all/{bidOffer}/{settlementDate}/{settlementPeriod}` and `SettlementStackResponse.transmissionLossMultiplier` with `nullable: true`. | **Confirmed.** No settlement payload was fetched. Nullable schema presence is not a complete 2024 loss-factor panel. |

Additional changed-source checks: the three [NESO](https://api.neso.energy/api/3/action/package_show?id=historic-demand-data) [CKAN](https://api.neso.energy/api/3/action/package_show?id=day-ahead-constraint-flows-and-limits) [records](https://api.neso.energy/api/3/action/package_show?id=24-months-ahead-constraint-limits) returned valid metadata, supporting retrospective demand corrections, day-ahead snapshot semantics and weekly forward limits. [PV_Live](https://www.solar.sheffield.ac.uk/pvlive/) confirms CC BY 4.0 and earliest available historical data of 2013-01-01. The [joint Irish report](https://cms.eirgrid.ie/sites/default/files/publications/Annual-Renewable-Constraint-and-Curtailment-Report-2024-V1.0.pdf), §3.3, confirms 7 April, 75% and 23,000 MW·s. The [2024 BSC release register](https://www.elexon.co.uk/bsc/release/list-of-change-releases-for-2024/) supports the release milestones and named modifications, not the unverified Section T version mapping. [P278's proposal](https://assets.elexon.co.uk/wp-content/uploads/sites/11/2012/01/04142822/P278.pdf), p.1, sets interconnector BMU TLM to 1.0; the [Authority decision](https://assets.elexon.co.uk/wp-content/uploads/sites/11/2012/01/28170102/P278D.pdf) gives 29 November 2012 implementation.

The newly linked [BritNed rules](https://www.britned.com/wp-content/uploads/2025/09/BritNed-Access-Rules-2025-Approved.pdf) define a “3% commercial loss factor”; the [ElecLink notice](https://cdn.prod.website-files.com/67b2dbc715938e08fe43fee1/67d9d09d90ce7e920a93ee31_ElecLink%20Loss%20Factor%20v2.pdf), linked from its [publisher page](https://www.eleclink.co.uk/publications/loss-factor), supports the nomination mechanism and 1.471% midpoint factor. The [Nemo rules](https://www.ofgem.gov.uk/sites/default/files/2022-11/The%20modified%20Access%20Rules%20-%20Nemo%20Link.pdf), Annex C, support the `1 ± LF/2` mechanism. No fresh verification of Nemo's operator numeric notice is claimed. These checks support separating commercial accounting from physical-loss modelling.

## Remaining findings and suggested fixes

### R2-01 — High: all seven claimed assumption questions are missing

**Location:** lines 54, 171, 232, 289, 345, 389, 394 and response rows F-02/F-03/F-07/F-08/F-09/F-14.

Exact-ID checks against `docs/requirements/02-questionnaire.md` at **both reviewed head `67a9d3d` and main `d84d37e`** find none of the following:

| Claimed ID | Proposed assumption | Questionnaire entry |
|---|---|---|
| Q-P-01 | 0.88 post-fault-to-continuous rating proxy | **Absent**; claimed in response row F-02, but line 54 only says it was logged. |
| Q-P-02 | ROI micro-PV interpolation and utility-to-rooftop scaling | **Absent** |
| Q-P-03 | HVDC converter/cable loss coefficients | **Absent** |
| Q-P-04 | Exploratory continuous SRMC clearing | **Absent** |
| Q-P-05 | Efficiency and variable O&M defaults | **Absent** |
| Q-P-06 | Wind shear exponents | **Absent** |
| Q-P-07 | PV temperature coefficient | **Absent** |

PR #58 changes only the source document, so these are not merely missing from this checkout. `ASSUMPTION:` labels and invented references do not satisfy AGENTS.md rule 1.

**Suggested fix:** add seven actual open questionnaire entries with these stable IDs, options, recommended defaults and links to the corresponding proposed assumptions; use working links from the source document. Do not mark them owner-approved or alter the decisions log. Replace the false “logged” assertion until the entries exist. Register any retained additional modelling assumptions as well, including the price interpolation in R2-04.

### R2-02 — Medium: three source links remain broken and ERA5's licence link is obsolete

**Location:** lines 17, 360, 368, 472 and response rows F-12/F-16.

| Submitted reference | Independent result | Suggested correction |
|---|---|---|
| NESO `/etys-documents-and-appendices` | **HTTP 404** | Use the working [ETYS archive](https://www.neso.energy/publications/electricity-ten-year-statement-etys/etys-documents-and-appendices), then identify the exact Appendix B edition/resource. |
| CEDA UUID `220a6561521f4bd42e93099438613f31` | HTTP 200, but body says **“No records were found matching this identifier.”** | Use the [actual MIDAS Open collection](https://catalogue.ceda.ac.uk/uuid/dbd451271eb04662beade68da43546e1/). Pin a release containing 2024 and the required station-variable resources; its catalogue states v202407 ends in 2023. Do not infer a 2024 station panel from the collection title. |
| CDS `/api/v2/terms/static/licence-to-use-copernicus-products.pdf` | **HTTP 404** | Cite the current dataset-attached licence via the [ERA5 catalogue API](https://cds.climate.copernicus.eu/api/catalogue/v1/collections/reanalysis-era5-single-levels), which explicitly says `CC-BY-4.0`. Retain account and interactive acceptance prerequisites separately. The open-commercial classification itself is supported. |
| ENTSO-E `/data/cim/cim-for-grid-models/` | **HTTP 404** | Replace with a working publisher reference identifying the applicable profile/version, or label the reference unresolved. No CGMES artifact acquisition or rights clearance follows from a standard name. |

The response table specifically claims the MIDAS UUID was repaired, but line 368 retains the original nonexistent identifier. Correct the document and its response table together. A broken link does not prove the underlying source is unavailable or restrictive.

### R2-03 — High: summary rights still omit restricted or unverified inputs

**Location:** lines 379–380, 411–412, summary rows 3/7/8 (516, 520–521), and response row F-16.

Row 3 labels its mitigation `open-commercial` while explicitly deriving the ROI capacity proxy from ESB reports that Candidate 3.3 classifies `research-only`. A derived restricted input does not lose its lineage through interpolation. Row 7 is still wholly `open-commercial`, contrary to Candidate 7.3's mixed/unverified treatment; the response table incorrectly says this row now reflects mixed rights. Met Éireann's [own policy](https://www.met.ie/about-us/specialised-services/open-data) says **most** datasets are CC BY 4.0 and some have custom licences. An organisation listing on data.gov.ie identifies neither the exact station resource nor its grant.

Candidate 8.1 and row 8 still infer WRC `open-commercial` status from the generic data.gov.ie licence page. The cited [WRC guide](https://www.workplacerelations.ie/en/what_you_should_know/public-holidays/) and [WRC disclaimer](https://www.workplacerelations.ie/en/disclaimer/) did not establish that resource-specific grant. This is insufficient evidence, not a finding that calendar facts are proprietary. Ofgem's [copyright policy](https://www.ofgem.gov.uk/copyright) likewise explicitly excludes identified third-party material, so keep operator schedules separate from regulator-authored notices.

**Suggested fix:** include ESB's research-only lineage in row 3; retain mixed/unverified status in row 7 until exact Met Éireann extracts and terms are identified; source a WRC-specific reuse grant or keep acquired WRC content `research-only / unverified`. Keep IANA and GOV.UK classifications separate from the unresolved WRC artifact. Apply D-56 to derived inputs and reflect the actual classifications in the response table. No commercial execution should inherit blanket clearance from these rows.

### R2-04 — Medium: interpolated price inputs still lack provenance and an as-of policy

**Location:** lines 307, 339–346; earlier F-10.

The recommendation still turns monthly Pink Sheet values into daily inputs by interpolation without labelling that step `ASSUMPTION:` or recording publication timing. Interpolation between monthly values can use information published after the simulated day; even a step-held monthly value requires its availability date. It is not observed daily fuel pricing. Fortnightly auction observations also require an explicit alignment/imputation policy before becoming daily/half-hourly carbon vectors. The document now correctly names EEX as the EU ETS auction publisher, but supplies no specific EUA auction resource or its terms.

**Suggested fix:** label and register the proposed resampling/imputation method, preserve original cadence and observed/imputed masks, and declare reconstruction where future monthly/auction information is used. For as-of backtests, select only prices demonstrably published by the knowledge cut (D-70 / ADR 0002). Identify a dated EEX EUA resource and independently check its rights, or leave EUA acquisition and licensing explicitly unresolved. Do not imply that the ICE link supplies both series.

### R2-05 — Medium: a live SEMOpx schedule is still presented as a pinned 2024 baseline

**Location:** lines 259–273, 287 and summary row 5 (518).

The corrected 17:30 D−1 value is real on the live publisher page. Nevertheless, the candidate heading and coverage say “2024 Baseline” / “calendar year 2024”, while no historical operating-procedure identifier is supplied. The [current rules hub](https://www.semopx.com/rules-and-monitoring/market-rules) contains 2026-dated material; the live intraday page links a procedure under a `2025-04` path. Neither establishes the gates, continuous-trading horizon, timezone/DST convention or effective intervals for every 2024 product. Qualifying the clock convention and price limits alone does not establish historical gate applicability.

**Suggested fix:** describe the verified schedule as a **current publisher reference**; mark its historical 2024 applicability, including IDC, `UNVERIFIED` until a dated procedure/effective-interval mapping is identified. Rewrite the recommendation and row 5 as pending historical pinning. Acknowledging the missing historical evidence is sufficient for this candidate list; no dataset download is needed to make the correction.

### R2-06 — Medium: the HVDC formula still has no fixed converter-loss term

**Location:** lines 233–237; earlier F-03.

“Fixed plus linear converter loss” remains inconsistent with the supplied constant-fraction formula. Each converter contributes loss power proportional to transmitted power, with no no-load/standby power term; at zero transfer the formula gives zero loss. The added disclosure of standby losses does not change that equation.

**Suggested fix:** call this a constant fractional exploratory proxy with omitted standby losses, or specify a separate calibrated power-loss formulation with fixed/linear/quadratic terms and units. Keep the coefficients unverified and subject to Q-P-03. Say that **no applicable CIGRE source has been identified** rather than asserting what all B4 literature does or does not prescribe. No claim of an engineering standard or accepted physical baseline is justified.

### R2-07 — Low: the ADR and earlier-review links resolve outside `docs/`

**Location:** lines 488 and 528.

From `docs/research/m1-deficit-sources.md`, `../../architecture/...` resolves to nonexistent root `architecture/...`, and `../../requirements/reviews/...` resolves to nonexistent root `requirements/reviews/...`. This breaks the two references needed to inspect implementation authority and the response-to-review mapping.

**Suggested fix:** use `../architecture/adr/0002-network-and-scenario-data-model.md` and `../requirements/reviews/gpt-2026-11-15-pr58-deficit-sources.md`. Replace “directly resolves all 16 findings” with an accurate disposition until all remaining corrections, especially R2-01 and R2-03, are made.

## Handoff

The corrected publisher numbers and removal of unsupported engineering attribution should be retained. Add the real questionnaire entries, repair source/reference links, propagate resource-level rights through the summaries and derived inputs, and qualify historical schedules and interpolated prices consistently. Preserve all nine deficit IDs and pending implementation/acquisition work. Re-check the resulting exact head; this report does not authorise merging PR #58.
