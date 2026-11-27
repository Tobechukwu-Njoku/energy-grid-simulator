# PR #58 review — M1 deficit sources

Review date: **2026-11-15 UTC**. Reviewed [PR #58](https://github.com/Tobechukwu-Njoku/energy-grid-simulator/pull/58), branch `docs/m1-deficit-sources`, exact head **`db59b5b44adec4835fdd1d82d9ba8b94ebfec861`**, file `docs/research/m1-deficit-sources.md` (483 lines). Line references below refer to that head. Governing sources: D-56, D-70, the accepted ADRs and the nine deficits in [the inventory](../../docs/research/m1-data-inventory.md#field-level-deficit-backlog).

This was a documentation and publisher-source check. Publisher pages, catalogue metadata, API specifications and reference reports were inspected; **no datasets were downloaded**, no authenticated acquisition was attempted, and no annual completeness or payload-field audit is claimed. Reference-document text was inspected outside the repository. PR #58 was not changed. A failed URL or blocked request is evidence of an access problem, not proof that a dataset does not exist or has restrictive rights.

Severity: **high** means a correction is required before the document can guide M1 implementation or provenance; **medium** means a material sourcing, classification or reproducibility correction. “Unverified” means insufficient evidence, not necessarily a false number. Findings do not create owner decisions or approve new modelling defaults.

## Publisher licence and access checks

| Publisher/resource | Independent result and D-56 treatment |
|---|---|
| ERA5 / Copernicus CDS | The [publisher's single-levels catalogue API](https://cds.climate.copernicus.eu/api/catalogue/v1/collections/reanalysis-era5-single-levels) explicitly returns `license: CC-BY-4.0`, supporting **`open-commercial`**. The [API setup instructions](https://cds.climate.copernicus.eu/how-to-api) require registration/login, a personal access token and **manual acceptance of dataset terms before downloading**. The PR's old `/api/v2/terms/static/licence-to-use-copernicus-products.pdf` URL returns 404. Cite the current dataset-attached licence rather than claiming that the obsolete document incorporates CC BY. |
| NESO ETYS appendices | The [actual ETYS archive](https://www.neso.energy/publications/electricity-ten-year-statement-etys/etys-documents-and-appendices) lists ETYS24 Appendix B (`/document/351916/download`) and Appendix G (`/document/351986/download`). The PR's shortened archive URL returns 404. [NESO website terms](https://www.neso.energy/terms-and-conditions) do not establish an open commercial grant for these appendices. **`research-only`**, PC-1, is appropriate pending resource-specific clearance. Do not extend the separate Data Portal open licence to all NESO publications. |
| EirGrid / SONI publications | The joint [2024 renewable constraint and curtailment report](https://cms.eirgrid.ie/sites/default/files/publications/Annual-Renewable-Constraint-and-Curtailment-Report-2024-V1.0.pdf) is publicly readable; [EirGrid's disclaimer](https://www.eirgrid.ie/disclaimer) reserves copying/public/commercial-use rights absent express authorisation. **`research-only`**, PC-2, remains appropriate for these documents. Public access does not confer redistribution rights. SONI's [operational-constraints page](https://www.soni.ltd.uk/industry/transmission-system/operational-constraints) provides joint reference documents but no affirmative open licence was established here. |
| SEMOpx | The [current rules hub](https://www.semopx.com/rules-and-monitoring/market-rules) is public; the PR uses a different, broken path. The [disclaimer](https://www.semopx.com/disclaimer) supplies no affirmative open commercial licence. **`research-only`** is the conservative D-56 classification. Its wording is a general disclaimer, so do not quote an unlocated “all rights reserved” licence as if inspected. Current linked rules are not automatically 2024 rules. |
| MIDAS **Open** | The [correct CEDA collection](https://catalogue.ceda.ac.uk/uuid/dbd451271eb04662beade68da43546e1/) explicitly says **all these data are provided under an Open Government Licence**. **`open-commercial` is correct**, distinct from the fuller restricted MIDAS collection. Registration/access is separate from licensing. The PR's different UUID returns HTTP 200 with “No records were found matching this identifier.” Pin a release containing 2024: the catalogue says v202407 ends in 2023, and each annual release ends in the preceding complete year. |
| Met Éireann | The [publisher's Open Data page](https://www.met.ie/about-us/specialised-services/open-data) says **most**, not all, datasets use CC BY 4.0, with attribution conditions; some, principally live forecasts, have a custom licence. The [general website notice](https://www.met.ie/about-us/copyright-notice) restricts commercial reuse and scraping. The PR's distinction between identified open extracts and general website content is prudent, but an organisation listing on data.gov.ie does not establish the licence of every station file. **`open-commercial` only for a specifically identified CC BY resource; otherwise `research-only / unverified`.** |
| World Bank Pink Sheet | The [Commodity Markets page](https://www.worldbank.org/en/research/commodity-markets) explicitly links its “Using this Data” section to [Data Access and Licensing](https://datacatalog.worldbank.org/public-licenses#cc-by), which permits commercial reuse under CC BY 4.0 for Bank-produced open data, with additional terms and third-party exceptions. **`open-commercial` is supported for the Bank's Pink Sheet**, subject to the attached terms and source exceptions. Do not generalise that to every World Bank-hosted dataset: the licensing page also describes ODbL and other grants. |
| Sheffield Solar PV_Live | The [PV_Live publisher page](https://www.solar.sheffield.ac.uk/pvlive/) explicitly states **“PV_Live by Sheffield Solar is licensed under CC BY 4.0”**. **`open-commercial` is supported.** That page offers historical downloads from 2013; it does not establish all regional fields, publication vintages or 2024 completeness. The PR's `/pvlive/api/v4/` URL returns 404. |
| Ofgem versus LCCC / operator attachments | [Ofgem's copyright policy](https://www.ofgem.gov.uk/copyright) permits reuse of its Crown copyright content under OGL, **excluding identified third-party material**. Ofgem's own RO notice can be `open-commercial`; an operator's attached access rules are not automatically OGL. LCCC's PR-linked register returned 403 and its resource licence was not independently established: retain `research-only / unverified` for that separate candidate. |

**ODbL/OSM check:** no proposed source in PR #58 affirmatively uses ODbL or OSM; the document expressly excludes them. World Bank hosting and CGMES compatibility must not be treated as blanket licence clearance. This review confirms no identified share-alike candidate was admitted, not the unseen upstream lineage of unacquired files. D-56 still requires resource-level manifests and rejects share-alike ancestors.

## Requested numerical checks

| Claim | Result |
|---|---|
| All-island SNSP 75% | **Confirmed.** Joint 2024 report, §3.3: “75% limit applied for the full year.” Cite this report, not realised SNSP observations. |
| Inertia floor 23,000 MW·s | **Confirmed.** Same section: “23,000 MWs floor applied for the full year.” It is an all-island operational floor. |
| ROI unit-count change on 17 July 2024 | **Contradicted.** The same report says the seven-set trial, four Ireland plus three NI, became enduring policy on **7 April 2024**. See F-01 for trial qualifications. |
| Micro-PV 350→498 MW; ~79,000→121,800 installations | **Not established as stated.** ESB's annual reports instead give about **300 MW / 78,400 at end-2023** and **500 MW / 121,700 at end-2024**. The exact 350/498 MW series needs a different dated source, not false precision attributed to these reports. F-08. |
| BritNed 3.0% | **Supported as a commercial loss factor** by current operator rules; historical applicability and physical-model interpretation need qualification. F-04. |
| Nemo 2.372%, half 1.186%, coupling 2.344% | **Numeric notice unverified in this review.** The operator site returned 403; regulator-hosted rules confirm the midpoint adjustment mechanism but defer the numeric factor to the operator. The numbers are algebraically consistent if LF = 2.372%: `1 − (1 − LF/2)/(1 + LF/2) ≈ 2.3442%`. Arithmetic is not publisher verification. F-04. |
| ElecLink 2.9% / midpoint 1.471% | **Confirmed.** Operator [loss-factor page](https://www.eleclink.co.uk/publications/loss-factor) and its linked notice dated **20 May 2022** support these values and midpoint nomination treatment. F-04. |
| Interconnector BMU TLM = 1.0 under P278 | **Confirmed rule.** Elexon's [P278 page](https://www.elexon.co.uk/bsc/mod-proposal/p278/), [proposal](https://assets.elexon.co.uk/wp-content/uploads/sites/11/2012/01/04142822/P278.pdf) and [Authority decision](https://assets.elexon.co.uk/wp-content/uploads/sites/11/2012/01/28170102/P278D.pdf) support it; implementation was **29 November 2012**. This concerns GB transmission-loss allocation, not zero physical interconnector losses. |
| BSC Section T v38/v41 on 2 April / 7 November 2024 | **Exact section versions unverified.** Elexon's release page confirms those release dates and P415/P473 in November, but not the claimed section-version mapping or Jan–Oct / Nov–Dec selection. F-06. |
| DESNZ CCGT 52% / OCGT 35% | **Unverified as attributed.** The publication page and main report establish the source's purpose, not these exact Annex A cells, technology variants, vintage and fuel basis. No assumptions workbook was downloaded. F-09. |
| £64.73/ROC | **Confirmed for 2024/25**, not all calendar 2024. Ofgem also confirms 0.491 GB / 0.192 NI, noting the GB revision from 0.487 announced on 26 March 2024. F-10. |
| tzdata 2024b | **Real release, dated 4 September 2024**, permissive/public-domain data; proposed pin is reasonable. Neither a repository lock nor `chrono-tz`'s use of that release was demonstrated. F-15. |
| 0.88 × post-fault rating | **Unsourced proposed assumption**, not a verified P27 prescription. F-02. |
| CIGRE B4 0.75% per converter + 0.30%/100 km cable | **Unsourced proposed assumption**, not an established CIGRE standard in this submission. F-03. |

## Findings and suggested corrections

### F-01 — High: incorrect effective date for Irish minimum units

**Location:** lines 82–89, 114 and summary row 2.

The joint [2024 report](https://cms.eirgrid.ie/sites/default/files/publications/Annual-Renewable-Constraint-and-Curtailment-Report-2024-V1.0.pdf), §3.3, expressly says: “On April 7th, 2024, the 7-set trial (3 in N. Ireland and 4 in Ireland) was completed and became enduring policy.” §5.1 also refers to 7 April. It supports the 75% SNSP and 23,000 MW·s figures, but contradicts the claimed 17 July enduring-policy change. Moreover, a trial before April means that simply replacing July with April does not prove five ROI units were required during every earlier interval. The report itself has inconsistent trial/start wording between these sections, so exact trial operation needs the dated notices.

**Correction:** cite §3.3, replace the enduring-policy date with 7 April, and separately source the trial start, permitted intervals/exceptions and unit memberships. Mark the earlier interval schedule `UNVERIFIED` until established. Preserve NI's three-unit component while checking any additional local must-run requirements; do not call the overall rule set “complete, verified” from these three headline limits alone.

### F-02 — High: 0.88 thermal-rating factor has no supplied engineering derivation

**Location:** lines 51–54 and summary row 1.

No P27 edition, clause, table, conductor, weather assumptions or overload duration is cited for the asserted universal 0.88 factor. An emergency-to-continuous conversion depends on asset type, initial temperature/loading, duration and environmental assumptions. A single factor applied to overhead lines, cables and transformers is not justified by naming an overhead-line recommendation.

**Correction:** remove the claim that P27 establishes this factor. Keep continuous ratings missing unless supplied or derived with an asset-specific method and evidence. If proposing 0.88 for exploratory scenarios, label it **`ASSUMPTION:`**, create the required questionnaire entry, version it, disclose uncertainty and validate its effect before accepting an equivalent. Do not represent it as a commissioned-asset rating or a conservative bound established by this review.

### F-03 — High: CIGRE B4 proxy is attributed to an unidentified standard

**Location:** lines 212–218 and summary row 4.

“CIGRE B4” identifies a study committee, not a technical brochure, equation or universal pair of coefficients. The submission supplies no document number, date, page or operating point for 0.75% and 0.30%/100 km. Its constant fractional cable loss gives loss power proportional to transmitted power; resistive loss instead depends on current squared, resistance and operating voltage. Calling the converter treatment “fixed + linear” is also inconsistent with the supplied constant fractional formula, which has no fixed power-loss term.

**Correction:** treat both coefficients as **unsourced author-proposed assumptions**, not verified CIGRE prescriptions. This review cannot prove who invented the numbers; it can establish that their claimed provenance is absent. Supply an applicable reference and per-link calibration, including converter technology, voltage, resistance, standby terms, loading and direction, or leave the physical-loss fields missing. Any exploratory proxy needs the explicit assumption/questionnaire/provenance process.

### F-04 — High: commercial nomination losses are presented as certified physical models

**Location:** lines 175–190, 207–218 and summary row 4.

BritNed's [published rules](https://www.britned.com/wp-content/uploads/2025/09/BritNed-Access-Rules-2025-Approved.pdf) define a **“3% commercial loss factor”** and Schedule 3 applies it around Mid North Sea. These are 2025 rules, so they do not alone prove all 2024 vintages. The [2024 approval](https://www.ofgem.gov.uk/decision/approval-modified-access-rules-britned-interconnector-pursuant-standard-licence-condition-11a-electricity-interconnector-licence-2024) establishes a dated amendment, not a complete historical factor panel. ElecLink's [publisher notice](https://cdn.prod.website-files.com/67b2dbc715938e08fe43fee1/67d9d09d90ce7e920a93ee31_ElecLink%20Loss%20Factor%20v2.pdf) describes deemed metered volumes from midpoint nominations; its numbers are supported.

Nemo's [regulator-hosted rules](https://www.ofgem.gov.uk/sites/default/files/2022-11/The%20modified%20Access%20Rules%20-%20Nemo%20Link.pdf), Annex C, give `BMUMV = (1 ± LF/2) × DMV`, and defer the actual factor to the operator website. The operator blocked inspection. The PR's precise Nemo numbers therefore remain **unverified**, although the stated relationship is mathematically plausible. A midpoint denominator differs from a sending-end denominator: e.g. with LF = 3%, the implied receiving/sending ratio is `0.985/1.015`, not `0.97`.

**Correction:** cite dated operator notices and retain the exact nomination, sending/receiving and settlement definitions. Separate commercial deemed-volume factors, GB TLM, Irish nodal TLAF and estimated physical losses. Do not infer asymmetric physical curves or all ten links' parameters from these schedules, and do not call unsourced Nemo values certified. Identify the actual dated Irish TLAF table before asserting EWIC/Moyle application in each market stage.

### F-05 — High: TLM API coverage is asserted without a working cited endpoint

**Location:** lines 192–199 and 211.

Both cited endpoints return 404: `/saa/api/v1/i014/party-level/` and the undated `/bmrs/api/v1/balancing/settlement/system-prices`. The [official OpenAPI specification](https://data.elexon.co.uk/swagger/v1/swagger.json) identifies dated system-price routes and the distinct `/balancing/settlement/stack/all/{bidOffer}/{settlementDate}/{settlementPeriod}` route, with a nullable `transmissionLossMultiplier`. A nullable field in an acceptance stack is not proof of a panel covering all BMUs, GSP groups, settlement runs and 17,568 periods. This repeats a completeness assertion explicitly retracted in the inventory.

**Correction:** retain the stack as a candidate, cite its correct specification, and mark annual coverage, delivering/offtaking coverage and TLF availability unverified. Cite P278's legal sources for interconnector TLM = 1.0, which is independently supported; do not attribute legal proof to an uninspected data payload.

### F-06 — High: the claimed two-version BSC calendar is not evidenced

**Location:** lines 229–238, 260 and summary row 5.

Elexon's [2024 release register](https://www.elexon.co.uk/bsc/release/list-of-change-releases-for-2024/) confirms 2 April (P451) and 7 November (including P415/P473), but also February, June and other releases. The PR supplies no historical Section T files or change table establishing v38.0/v41.0. Its recommendation to use v38 for January–October even contradicts its own claimed 2 April start; likewise a 7 November release cannot apply from 1 November. The linked `bsc.elexon.co.uk` host failed DNS in this review; Elexon now directs readers to `bscdocs.elexon.co.uk`, which returned 403 here.

**Correction:** mark **both exact version assignments `UNVERIFIED`** until dated Section T documents and impacted-document matrices establish them. Build an effective-interval table covering the start of 2024 and every relevant change. A full BSC release date is not proof that Section T changed or had a particular version. Keep the confirmed November modification names, without inventing replacement section-version numbers.

### F-07 — High: SEMOpx schedule contains an incorrect gate closure and lacks dated rules

**Location:** lines 240–263.

The [publisher's Intraday Market page](https://www.semopx.com/markets/intraday-market) gives IDA1 order-book closure **17:30 D−1**, not 17:45; it gives IDA2 08:00 and IDA3 14:00. The [Day-Ahead page](https://www.semopx.com/markets/day-ahead-market) supports an 11:00 auction, but the PR must establish the applicable clock convention and DST handling from dated procedures before hardcoding UTC/GMT throughout the year. The PR's rules/modifications paths are broken, and the working hub currently links 2026 documents. A single −€500/+€4,000 pair must not be silently assigned to all DAM, IDA and continuous products, or to the BSC cashout price.

**Correction:** fix IDA1 to the publisher-supported value, cite a 2024 procedure/version with timezone and product-specific gate/price limits, and leave those historical details unverified until sourced. Identify the actual dated Part B baseline; linking a publications hub does not itself resolve the inventory's missing baseline. Check the proposed continuous SRMC dispatch simplification against D-26/FR-7's unit-commitment requirement rather than presenting it as an accepted replacement.

### F-08 — High: Irish microgeneration baseline is overprecise and incorrectly sourced

**Location:** lines 145–163 and summary row 3.

ESB's [2023 distribution annual performance report](https://media.esbnetworks.ie/media/docs/default-source/publications/distribution-annual-performance-report-2023-accessibled434eedca532460d959ccfba328306eb.pdf), p.5, reports **78,400 connections and approximately 300 MW** at end-2023. Its [2024 report](https://media.esbnetworks.ie/media/docs/default-source/publications/distribution-annual-performance-report-2024.pdf), pp.5 and 64, reports **121,700 connections and approximately 500 MW** at end-2024. Those reports do not establish the document's exact 350→498 MW, 121,800 endpoint or an intra-year linear progression. Registration/application counts are also not automatically commissioned PV capacity observations.

**Correction:** use the report's rounded figures with their dates and definitions, or supply another primary source for the exact values and reconcile the differences. Mark interpolation as an assumption, distinguish registrations from commissioning, and retain a separate NI source/gap: ESB's Republic-of-Ireland fleet does not resolve all-island embedded generation. Utility-scale capacity factors scaled to rooftops also need an explicit uncertainty/representativeness assessment.

### F-09 — High: exact plant efficiencies and O&M are presented as DESNZ-verified

**Location:** lines 283–292, 307, 313 and summary row 6.

The [DESNZ publication](https://www.gov.uk/government/publications/electricity-generation-costs-2023) is a real OGL source with a main report and Annex A, updated November 2023. The [main report](https://assets.publishing.service.gov.uk/media/6556027d046ed400148b99fe/electricity-generation-costs-2023.pdf) concerns technology-level generation-cost assumptions, not an observed heat-rate inventory for every existing 2024 plant. The PR cites no sheet/cell/technology row for CCGT 52%, OCGT 35% or its O&M ranges. These numbers were **not independently verified** under the no-dataset-download scope, and the document supplies no auditable extraction evidence.

**Correction:** remove “sourced/from DESNZ Annex A” from the exact defaults until sheet/row, variant, commissioning vintage, price year and gross/net plus HHV/LHV basis are established. Treat technology proxies as assumptions rather than measured existing-unit characteristics. Preserve the valid publication and OGL classification. Record fuel units, carbon-emission basis and GBP/EUR conversion before evaluating the SRMC equation.

### F-10 — High: RO recommendation lacks a calendar-year transition; mixed rights collapse

**Location:** lines 294–313 and summary row 6.

Ofgem's [actual 2024/25 notice](https://www.ofgem.gov.uk/publications/renewables-obligation-ro-buy-out-price-mutualisation-threshold-and-mutualisation-ceilings-2024-2025) confirms **£64.73/ROC**, **0.491 GB** and **0.192 NI**. The PR correctly names 2024/25, but its recommended baseline does not explain the calendar-2024 vector's transition: Ofgem's historical table gives **£59.01/ROC for 2023/24**, and January–March 2024 falls in that preceding obligation year. The notice says the GB 2024/25 obligation was revised from 0.487 to 0.491 on 26 March. A supplier buyout price is not itself a complete generator ROC subsidy, nor one ROC/MWh for every eligible plant.

The PR also combines Ofgem with a blocked LCCC register under a single OGL clearance. Its summary labels the fuel/carbon/subsidy row `open-commercial` despite recommending ICE auction results classified `research-only / unverified` in the body. The EU auction source is not actually identified separately, and interpolating monthly/auction prices does not create observed daily prices or as-of evidence.

**Correction:** cite the notice, distinguish obligation-year effective dates and publication vintages, and source ROC banding/recycle assumptions separately if used for revenue. Split Ofgem, LCCC, UKA, EUA and Pink Sheet into independently licensed resources; keep unresolved rights research-only and correct the summary's mixed classification. Label interpolation and its information timing explicitly under D-70.

### F-11 — High: precise GB inertia transition dates remain unverified

**Location:** lines 99–109, 115 and summary row 2.

The PR's `/document/280926/download` returns 404. The [working operability archive](https://www.neso.energy/publications/operability-strategy-report-and-electricity-markets-roadmap) links the [2024 report](https://www.neso.energy/document/299926/download) and [2025 report](https://www.neso.energy/document/358116/download). The latter, pp.7 and 24, supports reduction **from 140 to 120 GVA·s during 2024**, but does not establish the PR's **28 February →130 / 19 June →120** effective-date sequence.

**Correction:** retain the supported broad reduction with its source; mark the intermediate 130 and both exact transition dates `UNVERIFIED` pending dated implementation notices/FRCR material. Do not derive regional inertia floors, contribution factors or local minimum units from a national narrative. Keep these inventory deficits open.

### F-12 — Medium: broken links and metadata descriptions are promoted to verified coverage

**Location:** candidates 1.1–1.3, 2.2, 3.1–3.2, 7.2–7.3 and 9.1.

Besides the broken links above, all three supplied NESO CKAN IDs for demand/day-ahead/24-month limits return 404. Working publisher metadata is available as [Historic Demand Data](https://api.neso.energy/api/3/action/package_show?id=historic-demand-data), [Day Ahead Constraint Flows and Limits](https://api.neso.energy/api/3/action/package_show?id=day-ahead-constraint-flows-and-limits), and [24 Months Ahead Constraint Limits](https://api.neso.energy/api/3/action/package_show?id=24-months-ahead-constraint-limits). The last is **weekly**, not a half-hourly historical schedule; the day-ahead series is a publication-time snapshot, not necessarily the final realised operating limit.

The metadata supports the ND/TSD distinction and the `ND + embedded solar + embedded wind` starting identity: pumping must not be subtracted from ND again. It does not prove an uninterrupted 17,568-row panel. Nor do current archive titles establish ETYS24 as a fully reconciled 2023/24 commissioned fleet, archived November 2024 editions for every DNO, or the claimed complete MDD/P114-to-physical-GSP crosswalk. A GSP Group is not a physical GSP. “All 8,784 hours” at 25 Met Éireann stations and ~250 MIDAS stations also need record-level evidence.

**Correction:** repair the URLs, pin edition/resource identifiers and distinguish metadata-inspected from data/fields/completeness-verified. Preserve source-specific gaps and missing intervals without claiming closure. Cite the corrected MIDAS UUID. Correct the forward-limit cadence and version/publication semantics. Keep the valid demand identity while verifying its station-load and metering reconciliation during ingestion.

### F-13 — Medium: item 9 is internal implementation work, not an acquired data source

**Location:** lines 420–473 and summary row 9.

The inventory already identifies `INT-01` as internal. Candidate 9.3 correctly says N/A in its body but the summary counts it as `open-commercial`; it also describes direct access to `crates/energy-scenario-schema/`, which does not exist in the reviewed tree. [ADR 0002](../../docs/architecture/adr/0002-network-and-scenario-data-model.md) specifies opaque UUIDs, not necessarily version 4, and keeps CIM mRIDs as external aliases. It does not establish that the proposed crate, fixtures, hashing implementation or Frictionless integration exists. A standard being publicly accessible does not establish commercial redistribution rights for IEC text or a particular CGMES artifact; the submitted ENTSO-E URL returns 404.

**Correction:** reclassify deficit 9 as **internal schema/identity/fixture implementation**, with D-56 **N/A** for the internal artifact and explicit pending status. Keep external standards as design references with their own artifact/licence checks, not acquired dataset counts. Identify new engineering choices as proposals; preserve ADR 0002's canonicalisation, migrations, knowledge-mode and separate input/execution identity requirements. Do not assert SHA-256/package capabilities merely by citing a generic standards hub.

### F-14 — High: weather fallback cannot supply the claimed spatial information

**Location:** lines 349–361 and summary row 7.

The proposed inverse-distance weighting starts from aggregate Elexon fuel outturns and dashboard totals, without geographically located donor observations. National generation totals do not identify per-site weather or site-specific capacity factors; weighting them by distance cannot recover that missing information. Reusing target outturns to generate inputs also needs an explicit holdout policy to avoid circular validation. Separately, IEC 61400-1 turbine classes do not define a unique manufacturer's power curve; the stated shear and temperature coefficients are unvalidated modelling defaults.

**Correction:** describe aggregate-outturn scaling as an explicitly labelled reconstruction/synthetic fallback with its lost spatial information; do not call it weather acquisition or validated downscaling. Require located donors for spatial interpolation, dated manufacturer/library curves and a validation split independent of the reconstruction target. Mark coefficients `ASSUMPTION:` with the required questionnaire/provenance entries, and leave actual weather acquisition and conversion integration open.

### F-15 — Medium: tzdata exists, but the claimed pin and summary interval count need correction

**Location:** lines 400–416 and summary row 8.

IANA's [2024b NEWS](https://data.iana.org/time-zones/tzdb-2024b/NEWS) dates the release to **4 September 2024**; its [LICENSE](https://data.iana.org/time-zones/tzdb-2024b/LICENSE) puts the data in the public domain with specified BSD exceptions for some code. The stated GB/Irish 31 March and 27 October transitions and **46/50** half-hour counts are consistent. However, the summary says **49/50** periods at the autumn fold, and naming `chrono-tz` does not prove its resolved database release is 2024b. This September release also cannot be presumed available at earlier 2024 forecast origins without an equivalence/as-of argument.

**Correction:** fix the summary to 46/50 for spring/autumn and 50 for autumn alone. Present 2024b as a proposed reconstruction pin until an artifact hash and runtime dependency mapping exist. Keep the version in scenario provenance and distinguish civil holiday calendars from exchange/settlement calendars. The [WRC page](https://www.workplacerelations.ie/en/what_you_should_know/public-holidays/) supports ten holidays and Good Friday's exclusion but does not grant every employee the next working day off after a Sunday holiday; preserve entitlement versus actual demand-calendar distinctions.

### F-16 — Medium: licence/access conclusions need resource-specific evidence and consistent summaries

**Location:** lines 5, 139–150, 294–299, 324–350, 372–377, 427–450 and summary.

The licence table above establishes the requested principal classifications. It also shows why a generic Creative Commons/OGL link, an organisation portal, regulator publication or publicly accessible standard is not sufficient evidence for every listed artifact. ERA5's open licence and account requirement are correct, but its licence URL is obsolete and first-time manual terms acceptance contradicts the unqualified “without interactive GUI steps” wording. MIDAS's OGL is correct despite the broken identifier. Met Éireann's mixed treatment should remain resource-specific. PV_Live's licence is supported, while its claimed API/regional field coverage is a separate question.

**Correction:** attach licence evidence to each exact resource, preserve account/access prerequisites separately, split mixed candidates and reflect mixed rights in the matrix (including row 6). Cite the actual current ERA5 licence and acceptance instructions. Do not relabel internal item 9 as an external open-commercial dataset. Keep unresolved WRC/IEC/LCCC artifact grants `research-only / unverified` where an external artifact is actually acquired; no blanket licence is inferred from another portal's policy. Apply PC tracking without treating public visibility as permission to redistribute raw restricted data.

## Disposition

The document identifies useful publishers and contains several correct facts: the Irish SNSP/inertia values, P278's interconnector TLM rule, ElecLink's published factors, the RO buyout for its proper obligation year, ERA5/PV_Live CC BY, MIDAS Open OGL, Pink Sheet's permissive licensing route and a real tzdata release. None establishes the document's blanket claim that unconfirmed details are all labelled `UNVERIFIED`.

Correct F-01–F-16, retain the nine original deficit IDs, and distinguish externally sourced facts, unverified candidates, internally proposed assumptions and pending implementation. Do not close a deficit or permit commercial reuse from this candidate list alone. The rating and CIGRE proxies in particular have no established provenance in the submission; the unit-count date is contradicted by the publisher. A corrected revision needs a source-by-source re-check before reliance.

**Verdict: reject.**
