# D-31 data spike: GB + all-island Ireland inputs and R-12 verdict

**Checked:** 12 November 2026. **Historic backtest year:** 2024 (leap year).
**Verdict on R-12: PARTIALLY.** A substantial electrical model is technically
obtainable, but a **complete, commercially cleared, present-day GB + all-island
network has not been demonstrated**. Do not close R-12 or treat D-32 as delivered.

This is an independent primary-source inspection, not a synthesis of the parallel
Gemini catalogues. Work started from `main` at `8b70c67`; no earlier data-spike
branch or artefacts were reused. Only this directory and `.gitignore` change.
Requirements remain unbaselined; no application code or owner decision is added.

## 1. Executive finding

Three different problems must not be conflated:

1. **Electrical data exists.** ETYS 2025 Appendix B yields **1,952 endpoint IDs,
   1,392 circuit rows and 1,472 transformer rows** for its 2025/26 base tables.
   EirGrid/SONI's downloadable 2024 study archive contains actual PSS/E RAW and
   sequence cases, not just diagrams. One winter case has **2,025 bus records,
   1,129 AC branches and 1,310 transformer devices**. Both include electrical
   parameters and ratings; it would be wrong to say public impedances are absent.
2. **Downloadable does not mean permissively licensed.** ETYS Appendix B was not
   found explicitly associated with NESO's dataset-specific open licence. NESO's
   general website terms restrict commercial exploitation. EirGrid's disclaimer
   restricts public/commercial use without express permission, and the joint
   TYTFS PDF reserves reproduction rights. Neither network is cleared for the
   proposed commercial database. No restricted network records are committed.
3. **Full and current coverage is unproven.** ETYS is principally a transmission
   owner dataset: England/Wales DNO-owned 132-kV circuits require additional LTDS
   models. **NGED provides an important permissive exception:** its LTDS tables
   contain 1,138 132-kV circuit rows with R/X/B and winter/summer ratings, plus
   switchgear and node IDs, across its four licence areas. This is not all GB.
   Neither the ETYS 2025/26 baseline nor Ireland's 2024 snapshot establishes the
   exact operational network on 12 November 2026.

The cleared part of the time-series problem is much better: **all 17,568 half-hours
of 2024 GB demand and fuel-mix data were downloaded and inspected**. Elexon BM
bids/offers/acceptances are publicly accessible and commercially reusable under
its BMRS licence. Genuine GB day-ahead auction prices and I-SEM day-ahead prices
remain separate access/licensing tasks. ERA5 is commercially reusable under the
**CC-BY-4.0 licence currently attached to the CDS dataset**, but no authenticated
  weather-year retrieval was completed in this spike.

**Combined permissively cleared full-network model achieved: none.** Candidate
network statistics below are feasibility measurements, not a reusable licensed
GB+IE network, an operational topology, or a power-flow validation result.

## 2. Licence evidence and admissibility

These are licence-screening findings, not legal advice. `UNVERIFIED` means no
affirmative dataset-specific grant was established; it is **not** permission.
All raw downloads are ignored. No OSM/ODbL or other share-alike dataset was used,
including for connectivity checks. A portal-wide licence was never silently
applied to unrelated publication downloads.

### Positive commercial-use clauses, checked on official pages

| Licence and primary page | Commercial-use sentence/clause | Scope and conditions |
|---|---|---|
| [NESO Open Data Licence v1.0](https://www.neso.energy/data-portal/neso-open-licence) | “exploit the Information commercially and non-commercially for example, by combining it with other Information, or by including it in your own product or application.” | Only datasets explicitly associated with it. No share-alike. Attribution required; excludes unlicensed third-party rights. CKAN metadata confirms demand, generation mix, TEC and boundary GIS. **Not proof for Appendix B.** |
| [NGED Open Data Licence, December 2020](https://www.nationalgrid.co.uk/open-data-licence) | “exploit the Information commercially and non-commercially for example, by combining it with other Information, or by including it in your own product or application.” | Applies to Connected Data datasets marked Open. The [LTDS tabular dataset page](https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model) explicitly says Rights: NGED Open Data Licence and Data Classification: Open. API retains the historical `wpd-open-data` label; this is not a share-alike licence. |
| [Open Government Licence v3.0](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/) | “exploit the Information commercially and non-commercially for example, by combining it with other Information, or by including it in your own product or application.” | GOV.UK REPD and DUKES pages carry the OGL release notice, subject to stated exceptions/third-party rights. No share-alike. Do not infer that proprietary basemap/geocoder rights are included. |
| [Elexon licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/) | “exploit the BMRS Data, including commercially, or by including it in your own product or application” | Explicitly defines BMRS Data to include Insights Solution data. Applies to non-BSC parties; BSC parties have code terms. Attribution and attribution propagation through sublicences, **not** share-alike of an entire derived database. Third-party rights excluded. |
| [CC-BY-4.0 official deed](https://creativecommons.org/licenses/by/4.0/), linked by the [CDS ERA5 licence page](https://cds.climate.copernicus.eu/licences/cc-by) | “Share — copy and redistribute the material in any medium or format for any purpose, even commercially.” | The [ERA5 dataset page](https://cds.climate.copernicus.eu/datasets/reanalysis-era5-single-levels?tab=download) links this licence. Attribution/link/change notice required; no share-alike. This check uses the current dataset association, not a generic historical Copernicus licence. |

Attribution text for the committed excerpts is in [README.md](README.md).
NESO's CKAN `license_url` still uses an older `ngeso-open-licence` path; the live
NESO licence page was checked independently. CDS's general licence catalogue
contains many different licences: its listing of ODbL for other datasets does
**not** mean ERA5 is ODbL, and those other data were not fetched.

### Blocked or unconfirmed reuse

- **ETYS Appendix B: UNVERIFIED, not approved.** Its [publication page](https://www.neso.energy/publications/electricity-ten-year-statement-etys/etys-documents-and-appendices)
  provides the workbook, but no affirmative open-licence association was found
  there or in the ETYS/transmission CKAN searches. Those searches found the
  explicitly licensed **boundary GIS**, not the technical workbook. Absence from
  a search is not proof that permission cannot be obtained. The [general terms](https://www.neso.energy/terms-and-conditions),
  Ownership/use/IP section, permit personal print copies and restrict commercial
  exploitation except as permitted by law. Obtain a written grant covering the
  TO-contributed data, derived databases and commercial service use.
- **Ireland network: permission required / UNVERIFIED for commercial reuse.**
  The [EirGrid disclaimer](https://www.eirgrid.ie/disclaimer) includes the restriction
  “used for any public or commercial purposes” and requires express provision or
  written authorisation. The joint TYTFS PDF's copyright notice (second PDF page)
  requires “the prior written permission of EirGrid and SONI” for reproduction.
  The ZIP's own 608-byte disclaimer discusses simulation/reference limitations;
  it contains **no open commercial licence**. Secure permission for the RAW/SEQ
  files specifically, from the relevant rights holders, including SONI/NIE/ESB
  contributions where applicable. Do not redistribute a parsed model meanwhile.
- **SEMO/SEMOpx: UNVERIFIED.** The [SEMO disclaimer](https://www.sem-o.com/disclaimer)
  and [SEMOpx disclaimer](https://www.semopx.com/disclaimer) disclaim accuracy but
  supply no affirmative commercial redistribution/database grant. Public XML
  availability is not sufficient. No commercial-use sentence could be verified.
- **GB exchange auction histories: not established as free commercial open data.**
  [Nord Pool's data service](https://www.nordpoolgroup.com/en/services/power-market-data-services/)
  offers UK day-ahead history, while its [website terms](https://www.nordpoolgroup.com/en/About-us/terms-and-conditions-for-useofwebsite/)
  distinguish internal firm research/trading from republication, which requires
  consent. This is not a SaaS redistribution grant. The EPEX market-data page
  returned an access-denied response in this run; pricing and usable terms there
  are **UNVERIFIED**. Request a quote/rights statement rather than assume free use.

## 3. Network inspections and counts

Reproduction: `fetch.sh --include-uncleared`, then the one-off
`inspect_networks.py`. Exact inputs, bytes and SHA-256 values are preserved in
`samples/download-manifest.json`; complete aggregate results are in
`samples/network-statistics.json`. Network records remain in ignored `raw/`.

### GB — ETYS 2025 Appendix B

The live appendices page identified ETYS **2025**, not a verified 2026 base case.
Only **B-2-1a…d** (2025/26 circuits) and **B-3-1a…d** (2025/26 transformers)
were counted; future change tables B-2-2/B-3-2 were not merged in.

| Published owner group | Circuit rows | Transformer rows |
|---|---:|---:|
| SHE Transmission | 310 | 254 |
| SPT | 352 | 310 |
| NGET | 614 | 768 |
| OFTO | 116 | 140 |
| **Total** | **1,392** | **1,472** |

- **1,952 distinct literal endpoint IDs**, **2,864 branch rows**. These include
  lower-voltage transformer terminals and offshore voltages, not 1,952 substations
  or exclusively 400/275/132-kV buses. B-1 is a site/voltage index, not a complete
  busbar/switch-state table. NGET's index has 231 rows at 400 kV, 128 at 275 kV
  and 186 at 132 kV; these are index rows, not a completeness census.
- Undirected literal-ID AC graph: **20 components**, largest **1,861 nodes**.
  All nodes are edge endpoints, so its zero isolated-node count says nothing
  about unpublished isolated buses. Collapsing IDs to their first four site
  characters gives **658 sites**, **12 components**, largest **636**. This merges
  bus sections **and voltage levels** and is a diagnostic only; it must never
  be used as an electrical network reduction without an approved bus mapping.
- **491 repeated unordered endpoint pairs** occur. They are not automatically
  errors: parallel circuits and transformers must retain separate identities.
- All counted R/X/B/rating cells were numeric. This is a schema check, **not**
  evidence of physical correctness, completeness or non-zero sensible values.
  R/X/B are labelled **percent on 100 MVA**; ratings are MVA, season-dependent
  for onshore circuits. Preserve base/units, sign conventions and seasonal limits.
  Transformer tables do not by themselves provide a full tap-control model.
- B-4 contains reactive equipment; B-5 explicitly distinguishes existing/planned
  intra-GB HVDC. These were inspected at header/sample level but excluded from
  the AC graph. Bus-coupler equivalence, normally-open switching, DC links and
  true isolated systems must be reconciled before interpreting every component
  as a missing line. GB and Ireland are separate synchronous systems linked by
  HVDC; a single connected **AC** graph spanning both is not the acceptance test.
- No geographic coordinates or universal generator-to-bus mapping were supplied
  by these base branch tables. [Appendix A schematics](https://www.neso.energy/document/383931/download)
  and [Appendix G nodal-demand inputs](https://www.neso.energy/document/379136/download)
  are follow-up candidates only, not parsed or separately licensed in this run.
  The separately cleared boundary GIS is not bus topology, a circuit cut-set, or
  a complete set of time-varying electrical transfer limits.

### All-island Ireland — TYTFS 2024 study archive

The official [report page](https://www.eirgrid.ie/all-island-ten-year-transmission-forecast-statement-2024)
is dated **3 November 2025**, and the [study-files page](https://www.eirgrid.ie/all-island-ten-year-transmission-statement-2024-study-files-0)
is dated **10 November 2025**. The archive contains summer-valley and winter-peak
cases for **2024 and 2033**, in PSS/E v33 and v35, with sequence files. The
inspected `TYTFS2024_WP2024_V33.raw` header describes a 2024/25 winter case and
was generated in September 2025. It is a modelled historical baseline, not a
measurement of today's topology. The 2033 case is not a current-network substitute.

| Measurement of this one winter case | Result |
|---|---:|
| Bus records, all voltage levels | 2,025 |
| AC non-transformer branch records / in service | 1,129 / 1,079 |
| Transformer devices / three-winding devices | 1,310 / 109 |
| Transformers with status 1 / partially disconnected status 4 | 1,309 / 1 |
| Generator records already attached to buses | 597 |
| Missing branch/transformer endpoint bus references | 0 |
| Graph with all declared branches/transformers | 2 components: 2,024 + 1 |
| In-service AC graph, excluding bus type 4 | 2 components: 2,022 + 1 |
| Declared 110/220/275/380/400-kV bus records | 547 |
| Active buses at those voltage bases; induced AC graph | 546; components 545 + 1 |
| AC branch records with nonnumeric R/X/B/rating-A fields | 0 |
| AC branch records with zero rating A | 132 |

Important interpretation limits:

- The voltage breakdown is **456 at 110 kV, 69 at 220 kV, 15 at 275 kV and
  7 at a 380-kV base**. There are no literal 400-kV base entries in this case.
  Reconcile the nominal 400-kV assets with the model's 380-kV base, not an
  arbitrary omission or voltage relabel. **Northern Ireland's 275-kV system is
  present**; the owner's Ireland 400/220/110 list needs explicit clarification.
- Many of the remaining buses are generation terminals, 38/33-kV network detail,
  or equivalents. Neither 2,025 nor 547 is a count of physical substations.
  A voltage-induced subgraph is a diagnostic, not a Kron-reduced equivalent.
- The one-off parser preserves the 4-/5-line distinction between two-/three-winding
  transformer records. Three-winding devices become two graph edges, not three
  independent transformers. Status 4 disconnects winding 1; its other two
  windings remain connected. Two bus-type-4 records are excluded from the live
  graph. DC data and electrical admittance calculations are not evaluated.
- RAW includes R/X/B, thermal ratings, transformer data, load and generator
  records, so technical assembly is credible. **132 zero rating-A entries are
  not zero capacity**: determine unlimited/unspecified/equivalent meanings and
  alternative ratings with the operator. No AC solution, N-1 validation,
  impedance-base conversion or reconciliation of the remaining island was run.
- Generator RAW IDs are model records, not a commercially cleared authoritative
  market-unit register. Geocoordinates and SEM participant/resource aliases still
  need independent, licensed mapping. Restricted sample records are not exported.

### England/Wales 132 kV — a concrete permissive partial path

The [NGED LTDS tabular model](https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model)
explicitly covers its four licence areas from 132 kV to primary substations.
Downloaded files contain **22,516 circuit/switch records**, **22,590 node records**
and **2,705 two-winding-transformer records**. Of the circuit table's **4,183
132-kV records**, **1,138 are circuits**, **1,183 circuit breakers** and **1,862
disconnectors**. All 1,138 circuit rows have populated R1/X1/B1 and winter/summer
rating fields. Their unit interpretation still needs the LTDS specification.

This is real evidence against a blanket claim that England/Wales 132-kV electrical
data is unavailable. It is **not** a validated present-day model: tables include
Unchanged/New/Updated/Removed statuses; the dataset's case/date semantics must
be resolved before selecting a baseline. Node IDs must be joined through
switchgear and matched to ETYS GSPs. No current-topology connectivity result is
claimed for this mixed-change table. Its successor [CIM publication](https://connecteddata.nationalgrid.co.uk/dataset/ltds-common-information-model)
advertises EQ/SC/SCR/SYSCAP cases and says GL/SSH/TP/SV/DL profiles are future
releases; ZIP model contents were **not** downloaded or counted here.

NGET's route-map page and NIE Networks' network page were blocked by Cloudflare
in direct HTTP checks. Guessed NGED dataset slugs returned 404; CKAN discovery
found the actual LTDS dataset. Other GB DNO areas, SSEN/SPT publication rights,
and TO geographic layers remain **UNVERIFIED** rather than inferred covered.

## 4. Generator fleet and historic-year inputs

### Fleet assembly

- **Elexon BM-unit reference:** 3,134 records in the retrieved current register,
  containing nationalGridBmUnit/elexonBmUnit identifiers, capacities, fuel field,
  party and GSP group. Fuel and EIC can be null. A GSP **group** is not an
  electrical bus; BM units can aggregate assets or be demand/storage. Current
  registry contents cannot be applied unchanged to a 2024 backtest.
- **NESO TEC register, 10 November 2026:** 2,220 rows, only **389 marked Built**;
  much of the remainder is the connection queue. Connection Site, plant type,
  effective date and capacity help aliases, but TEC rights are not necessarily
  turbine/nameplate capacity or a market-unit identifier. Preserve effective dates.
- **REPD Q2 2026:** 14,657 project records, **3,132 Operational** after trimming
  status whitespace. Technologies, capacity, addresses and X/Y coordinates help
  site mapping; planning-stage projects are not today's fleet. Not a complete
  thermal-unit catalogue, and no automatic substation/BMU key. Do not assume a
  single coordinate reference system without checking country-specific metadata.
- **DUKES 5.11, May 2026 fleet:** 1,364 rows with site name and numeric installed
  capacity in the main list. Technology/fuels, connection type, postcode, OS
  reference/X/Y and commissioning year are available. Notes say the list covers
  major power producers and that supplied geography/connection information is
  not verified by DESNZ. Historic sheets exist, including 2024, but a historical
  unit-by-unit mapping was not assembled here. Not every small generator is listed.
- **Ireland:** the inspected RAW has 597 generator records attached to model
  buses. The [2026–2035 resource-adequacy workbook publication](https://www.eirgrid.ie/all-island-resource-adequacy-assessment-2026-2035-data-workbook)
  was discovered in EirGrid's official sitemap, but not downloaded/cleared here.
  Secure EirGrid/SONI permission and unit/resource aliases before relying on either
  as a commercial fleet database. A generator count in RAW is not proof of a
  complete market-unit fleet; equivalents and control devices may be present.

Plant name matching alone is insufficient for UC: derive a versioned
asset → unit → BM/SEM resource → network-node crosswalk, retaining confidence,
capacity/date checks and manual exceptions. Public fuel/capacity data does not
establish startup cost, minimum stable generation, ramp limits, minimum up/down
times, outages or heat-rate curves. Those are additional M1/M2 inputs, not
properties demonstrated by this spike.

### 2024 backtest observations

| Input | Actual inspection | Consequence for M1 |
|---|---|---|
| GB demand | Full NESO file: **17,568 rows, 366 dates, no duplicate date/period keys or empty cells**. 31 March has 46 periods; 27 October has 50. ND/TSD/embedded wind/solar/storage and named interconnector flows supplied. | Good licensed baseline; specify ND vs TSD and treatment of embedded generation/pumping to avoid double counting. Local settlement dates are not fixed 48-row UTC days. |
| GB generation mix | Full 63,066,409-byte multi-year CSV downloaded; **17,568 unique 2024 timestamps**, 1 January 00:00 through 31 December 23:30, no empty cells. Gas/coal/nuclear/wind/embedded wind/hydro/imports/biomass/other/solar/storage fields. | Suitable aggregate backtest target; timestamps lack an offset in the CSV, so confirm publisher time convention before joining UTC observations. Embedded outputs are estimates, not complete site metering. |
| Elexon corroborating samples | 1 January: **48 demand rows**, **288 five-minute generation snapshots**. | Confirms an independent API route, not a full-year API download. Elexon initial-demand values differ from NESO historical/revised values; keep vintage/definition provenance. |
| BM bids/offers | **1,206 rows** for 1 January, settlement period 1; level endpoints, bid/offer prices, pair ID and BMU IDs parsed. | Historic access works without an API key. Year pagination/rate limits, amendments and missing periods are not yet measured. |
| BM acceptances | **65 rows** for that queried period; acceptance number/time, MW trajectories and SO/STOR flags available. Some records overlap midnight and retain the previous settlement date. | Deduplicate/version acceptances and integrate trajectories against FPN/pair volumes. Do not multiply every row by 0.5 h or treat offer acceptance as an invoice. |
| GB price | Elexon market-index query returns **98 records**, two providers, because time endpoints are inclusive. | This is short-term **Market Index Data**, **not a verified day-ahead auction series**. Some provider records have zero volume. Third-party exchange rights are unconfirmed. It is not an approved DAM substitute. |
| I-SEM day-ahead price | SEMOpx public reporting and day-ahead description checked; 2024 DAM/static/dynamic probes did **not** return a usable price series. The current results page comments out the DAM selector and EA-001 is disabled in its app configuration. | No finding that the prices do not exist; the attempted route did not establish retrieval. Ask SEMOpx for historical DAM endpoint/archive and commercial rights. Do not replace DAM with imbalance price. |
| Interconnector flows | Full GB named flows in demand CSV. Irish public `IDA1_InterconnectorFlows_20240101.xml` parsed: **4 time series, 192 half-hour interval records**. | Irish XML is **scheduled auction/cumulative flow**, not physical metered flow, and has a 23:00Z-to-23:00Z interval. Rights UNVERIFIED; no committed XML excerpt. Align market day, direction, units and losses; validate physical flows separately. |

The general Elexon [OpenAPI document](https://data.elexon.co.uk/swagger/v1/swagger.json)
was used to verify parameter names. An initial demand query with `from/to` silently
returned a recent default window instead of 2024; only the corrected
`settlementDateFrom/settlementDateTo` result is counted. Early SEMOpx queries with
`DateFrom/DateTo` similarly returned unrelated dates; the site's own JavaScript
established `date_from/date_to`. HTTP 200 is **not** sufficient validation.

The successful URLs are in `sources.tsv`. Failed probes are not data samples:
404/error JSON, empty listings and blocked HTML were not counted as coverage.
This spike **does not claim a complete 2024 BM/price/Irish-flow archive**.

## 5. One weather year — ERA5

The official [ERA5 single-level dataset](https://cds.climate.copernicus.eu/datasets/reanalysis-era5-single-levels?tab=download)
supports the required wind, radiation and temperature variables and currently
attaches CC-BY, commercially compatible with D-30. The committed
`era5-request-2024-01.json` is an **unexecuted monthly request template** for
2024, bbox **north 61°, west −12°, south 48°, east 3°**, hourly 100-m and 10-m
wind components, 2-m temperature and surface solar radiation downwards.

- Complete 2024 has **8,784 hours**. A 0.25° regular grid over this inclusive
  box would have 53 × 61 = **3,233 points**. Six variables as float32 would be
  `8784 × 3233 × 6 × 4 = 681,568,128` bytes before encoding and metadata;
  compression and NetCDF overhead are not fixed by this arithmetic.
- Registration, dataset licence acceptance and authenticated CDS job submission
  are needed for this retrieval route. No `~/.cdsapirc` was present; no credentials
  were read/requested and **zero weather bytes were downloaded**. Public metadata
  and attached terms were checked; neither a request JSON nor licence permission
  is proof that a weather year has been retrieved. A CDS job/request schema
  should be verified again when credentials are provisioned.
- The public [CDS process description](https://cds.climate.copernicus.eu/api/retrieve/v1/processes/reanalysis-era5-single-levels)
  was retrieved and the template checked against its per-input JSON schemas,
  including variable names and the 2024 year value. The [STAC catalogue](https://cds.climate.copernicus.eu/api/catalogue/v1/collections/reanalysis-era5-single-levels)
  independently reports `license: CC-BY-4.0`. Schema validation is not job execution.
- Convert Kelvin and accumulated radiation consistently (J/m² versus W/m² and
  timestamp/accumulation conventions). Calculate wind speed from components;
  aggregate irradiance through an explicit PV model; interpolate hourly weather
  to half-hour dispatch without inventing new information. Hub-height/power-curve,
  wake/availability/curtailment and installed-capacity history require validation.
  ERA5 is reanalysis, not a day-ahead forecast, so it cannot alone establish
  forecasting skill without avoiding look-ahead leakage.

**ASSUMPTION:** a 0.25° grid and six fields are a useful initial data-volume
budget, not an owner decision or measured download size. Exact arithmetic is
recorded during final validation; allow roughly **0.68 GB uncompressed per year**
before encoding/metadata, and submit monthly jobs rather than one giant request.
The bounding box is a candidate, not a guarantee of all offshore/future assets.

## 6. Dataset register — everything used or investigated

Sizes are **actual downloaded payload bytes** unless explicitly marked otherwise.
`Page only` and `not retrieved` are not successful dataset acquisitions. Licence
URLs/clauses are in §2; exact successful resource URLs are also in `sources.tsv`.
No = no share-alike requirement identified; **?** = no usable licence established.

| Dataset / primary URL | Licence | Commercial OK? | Share-alike? | Coverage / size | Quality or limit |
|---|---|---|---|---|---|
| [ETYS Appendix B](https://www.neso.energy/document/383936/download) | **UNVERIFIED**; general NESO website restrictions | Not cleared | ? | GB 2025/26 + future changes; **1,237,005 B** | Electrical data parsed, not complete all-DNO 132 kV or current 2026 operation. |
| [TYTFS 2024 study ZIP](https://cms.eirgrid.ie/sites/default/files/publications/TYTFS2024_studyfiles.zip) | No open grant found; EirGrid/SONI permission needed | Not cleared | ? | All-island 2024/2033 PSS/E; **11,954,215 B** | Strong electrical detail; only one case audited. |
| [TYTFS 2024 report PDF](https://cms.eirgrid.ie/sites/default/files/publications/All-Island-Ten-Year-Transmission-Forecast-Statement-2024_1.pdf) | All rights reserved / written permission | No open grant | ? | Joint report; **40,408,830 B** | Copyright and scope checked; not a permissive model licence. |
| [ETYS boundary GIS](https://api.neso.energy/api/3/action/package_show?id=etys-gb-transmission-system-boundaries) | NESO Open | Yes | No | ETYS 2024/Mar25 shapefile ZIP; **87,714 B** | Archive inspected, not converted to electrical cut-sets. |
| [NGED LTDS circuits](https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model) | NGED Open; explicit dataset association | Yes | No | Four DNO areas, mixed changes; **2,663,962 B** | 22,516 circuit/switch rows; 1,138 132-kV parameterised circuits. |
| [NGED LTDS nodes](https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/a92da9e4-2010-4406-b86a-825cd65761d6/download/ltds-attachment-8-node.csv) | NGED Open | Yes | No | Same model; **1,826,097 B** | 22,590 node rows, not all 132 kV/current. |
| [NGED LTDS two-winding transformers](https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/8c964d61-a7fb-41e7-99b4-6032d063a8c6/download/ltds-table-2a-two-winding-transformer.csv) | NGED Open | Yes | No | Same model; **308,177 B** | 2,705 rows, statuses and unit/tap semantics need resolution. |
| [NGED LTDS CIM](https://connecteddata.nationalgrid.co.uk/dataset/ltds-common-information-model) | Portal metadata: NGED/WPD Open | Yes for associated open data | No | Four areas; **page/metadata only**, no ZIP size measured | Successor case/profile coverage checked; not parsed. |
| [Elexon BM-unit reference](https://data.elexon.co.uk/bmrs/api/v1/reference/bmunits/all?format=json) | BMRS open-data licence | Yes, subject to exemptions | No | Current GB registry; **2,171,607 B** | 3,134 records, no electrical-node key. |
| [NESO TEC](https://api.neso.energy/api/3/action/package_show?id=transmission-entry-capacity-tec-register) | NESO Open | Yes | No | 10 Nov 2026; **445,749 B** | 2,220 rows include queue, not just operating plant. |
| [REPD](https://www.gov.uk/government/publications/renewable-energy-planning-database-monthly-extract) | OGL v3, stated exceptions | Yes for released rights | No | Q2 2026 CSV; **5,087,389 B** | 14,657 project rows; renewables/planning, not complete fleet. |
| [DUKES 5.11](https://www.gov.uk/government/statistics/electricity-chapter-5-digest-of-united-kingdom-energy-statistics-dukes) | OGL v3, stated exceptions | Yes for released rights | No | May 2026 + historical sheets; **1,247,443 B** | Main-list major producers and supplied geography, not verified grid nodes. |
| [EirGrid resource-adequacy workbook](https://www.eirgrid.ie/all-island-resource-adequacy-assessment-2026-2035-data-workbook) | **UNVERIFIED** | Not cleared | ? | Publication discovered; **not retrieved**, size unknown | Candidate fleet source, no unit-level contents claimed. |
| [NESO demand 2024](https://api.neso.energy/dataset/8f2fe0af-871c-488d-8bad-960426f24601/resource/f6d02c0f-957b-48cb-82ee-09003f2ba759/download/demanddata_2024.csv) | NESO Open | Yes | No | Full 2024; **1,809,707 B** | 17,568 periods, DST checked; includes GB named flows. |
| [NESO historic generation mix](https://api.neso.energy/dataset/88313ae5-94e4-4ddc-a790-593554d8c6b9/resource/f93d1835-75bc-43e5-84ad-12472b180a98/download/df_fuel_ckan.csv) | NESO Open | Yes | No | Multi-year, 2024 filtered; **63,066,409 B** | 17,568 2024 timestamps; aggregate/estimated components. |
| [Elexon demand outturn](https://data.elexon.co.uk/bmrs/api/v1/demand/outturn?settlementDateFrom=2024-01-01&settlementDateTo=2024-01-01&format=json) | BMRS open-data licence | Yes | No | One day 2024; **9,738 B** | 48 initial values; revisions/definitions differ from NESO. |
| [Elexon fuel snapshots](https://data.elexon.co.uk/bmrs/api/v1/generation/outturn/summary?startTime=2024-01-01T00:00:00Z&endTime=2024-01-02T00:00:00Z&format=json) | BMRS open-data licence | Yes | No | One day; **180,374 B** | 288 five-minute entries, not half-hour energy records. |
| [BM bid/offer](https://data.elexon.co.uk/bmrs/api/v1/balancing/bid-offer/all?settlementDate=2024-01-01&settlementPeriod=1&format=json) | BMRS open-data licence | Yes | No | One settlement period; **289,923 B** | 1,206 rows; full year not fetched. |
| [BM acceptances](https://data.elexon.co.uk/bmrs/api/v1/balancing/acceptances/all?settlementDate=2024-01-01&settlementPeriod=1&format=json) | BMRS open-data licence | Yes | No | One period query; **23,383 B** | 65 trajectory rows; cross-midnight overlaps. |
| [Elexon market index](https://data.elexon.co.uk/bmrs/api/v1/balancing/pricing/market-index?from=2024-01-01T00:00Z&to=2024-01-02T00:00Z&format=json) | BMRS terms exclude unauthorised third-party rights | **UNVERIFIED** for exchange-derived reuse | ? | One-day inclusive window; **14,172 B** | 98 provider records; not a verified DAM series. No committed price excerpt. |
| [SEMOpx scheduled interconnector flow](https://reports.semopx.com/documents/IDA1_InterconnectorFlows_20240101.xml) | **UNVERIFIED** | Not cleared | ? | 1 Jan 2024; **27,819 B** | 192 intervals, 4 directional series; scheduled, not metered. |
| [SEMOpx auction results](https://www.semopx.com/market-data/market-results) | **UNVERIFIED** | Not cleared | ? | **No DAM price data retrieved**, size unknown | Reporting probes and page/config checked; historical endpoint unresolved. |
| [Nord Pool UK histories](https://www.nordpoolgroup.com/en/services/power-market-data-services/) / [EPEX](https://www.epexspot.com/en/market-data) | Nord Pool restrictive website/data-contract terms; EPEX **UNVERIFIED** | No free commercial grant established | ? | **Page only / blocked**, no price dataset, size unknown | Obtain product/redistribution terms, not scrape-and-assume. |
| [ERA5 single levels](https://cds.climate.copernicus.eu/datasets/reanalysis-era5-single-levels?tab=download) | Attached CC-BY-4.0 | Yes | No | Proposed 2024 GB+IE; **0 weather B retrieved** | Metadata/licence checked; request template only. |

Other inspected pages/APIs were discovery or legal evidence, not extra datasets:
EirGrid sitemaps; NESO/NGED CKAN search and licence metadata; Elexon OpenAPI,
WordPress search and static app bundle; SEMOpx static-report listings/app bundle;
general Copernicus/CDS licence indexes; blocked NGET/NIE pages and a failed SONI
map URL. No successful independent TO geographic dataset is claimed. URLs which
404 or return empty/error responses have no assumed coverage, size or licence.

## 7. Concrete fallbacks for an owner decision

**ASSUMPTION:** estimates below are engineering person-weeks for an experienced
data/network modeller after access is available, excluding external response
delays, procurement and engine/UI work. These are estimates, not commitments or
decisions. The user prohibited edits to the questionnaire/decisions/STATUS in
this task, so approval questions are recorded here for the coordinator.

| Option | Prerequisite and effort estimate | Fidelity / accuracy impact |
|---|---|---|
| **A. Clear and assemble the published detailed models** | Obtain written ETYS and EirGrid/SONI commercial/derived-database grants; inventory every relevant DNO LTDS, use explicit present cases and match GSPs. **8–16 person-weeks** after access; rights responses may take weeks/months. | Best route towards D-32. Keeps real impedances/ratings, but not a verified full model until coverage/current topology, ties and missing parameters are reconciled. Do not promise today's grid just by importing planning files. |
| **B. Reduced ~300 main-substation GB network + ~100–250 Irish equivalents** | **Conditional on cleared source rights**; select major transmission nodes, preserve interfaces/boundaries and reduce detailed models, validate transfer factors. **4–8 person-weeks** after access. No right to derive from restricted models is assumed. | Loses some 132/110-kV/local congestion, radial connections and busbar security. Can support national dispatch and major-boundary BM studies; cannot claim all-node fidelity, reliable local LMPs, voltage or stability. Reduced sizes are targets, not achieved counts. |
| **C. Licence-clean synthetic electrical model** | Only independently licensed connectivity/geography/line types, plus independently sourced or newly estimated R/X/B/ratings from length/conductor assumptions; no copying restricted topology/parameters. **6–12 person-weeks** for a transparent prototype if connectivity is available. | Approximate PTDFs, loop flows, losses and constraint costs; underground cable charging and transformer behaviour particularly uncertain. Invented topology/line types must be labelled synthetic. Does **not** satisfy an accurate full-current-grid requirement. Source topology rights are still necessary. |
| **D. National GB + all-island two-zone UC with explicit HVDC links and calibrated major-boundary constraints** | Cleared aggregate demand/generation/weather; procure/clear prices and link data. **2–4 person-weeks** for data preparation once access is resolved. | Fastest honest dispatch/backtest fallback; cannot reproduce nodal congestion or full network BM costs. Boundary-only representation remains a scope reduction requiring owner approval, not a silent interpretation of D-32. |
| **E. Licence-clean regional pilot while rights proceed** | Use NGED's explicitly Open LTDS and current-case CIM if adequate, plus permitted time series; resolve switch/status semantics and GSP injection assumptions. **2–4 person-weeks**. | Demonstrates ingest/topology/electrical-unit handling without restricted ETYS/Irish data. Limited to four DNO areas, not the GB+IE MVP; cannot validate national transmission dispatch. |

Synthetic parameters alone cannot solve an unlicensed/missing topology. Buying
data is compatible with D-30 only if the actual commercial terms allow the
intended uses and do not impose share-alike. PyPSA-GB/PyPSA-Eur/OSM-derived network
datasets were **not** used as a shortcut or test reference in this spike.

## 8. Recommended M1 gates and next actions

1. **Rights gate before network ingest:** ask NESO to attach a licence explicitly
   to the Appendix B workbook and TO contributions; ask EirGrid/SONI for RAW/SEQ
   use, alteration, derived-database and commercial SaaS permissions. Preserve
   terms/version/hash and identify redistribution restrictions. Keep uncleared
   sources quarantined. Owner chooses A/B/C/D/E if grants are unavailable.
2. **Coverage and date gate:** create an owner/voltage/region inventory including
   all England/Wales 132-kV DNO areas, Scottish 132 kV, offshore connections,
   Ireland 400/220/110 **and NI 275 kV**. Agree whether other connecting voltage
   levels/equivalents are retained. Separate 2024 historical and 2026 present-day
   cases; obtain commissioning/outage/topology updates. An asset list and effective
   dates must explain what “full” and “today” mean.
3. **Electrical QA gate:** account for every node/terminal, switch, circuit,
   transformer winding/tap, shunt and converter. Resolve ETYS bus equivalences,
   remaining islands and zero/unspecified limits; preserve parallel circuits.
   Verify unit/base/voltage conventions and physical connectivity per synchronous
   system, then run independent power-flow/reference cases once engine work is
   authorised. No arbitrary component-joining or zero-rating substitution.
4. **Fleet/load gate:** version plant/BMU/resource/node crosswalks; reconcile
   installed/available/TEC capacities and commissioning dates; obtain permitted
   nodal demand allocation and UC technical parameters. Demand disaggregation and
   generator mapping can dominate congestion errors even with correct branches.
5. **Historic archive gate:** ingest all 2024 BM periods with backoff/retries,
   deduplication, revision tracking, FPN and accepted-volume reconciliation;
   require correct UTC/local settlement alignment and tests for both DST days.
   Preserve negative prices, direction/loss conventions, currencies GBP/EUR and
   data vintages. Secure **actual** GB and I-SEM DAM histories and document price
   rights; do not relabel MID or imbalance prices.
6. **Weather gate:** provision CDS access/accept terms, verify and submit monthly
   2024 requests, capture job IDs/checksums, inspect NetCDF coordinates/units and
   prove 8,784 unique hours with expected spatial/variable coverage. Weather-year
   assembly is **unfinished**, not hidden behind a request template.

**Owner questions for the coordinator:** Is an explicitly reduced/synthetic or
two-zone fallback acceptable if network rights cannot be cleared? Does D-32
intend NI 275 kV and all E&W distribution-owned 132 kV, with operational status
as of a named date? What budget is acceptable for licensed auction histories and
operator data access? No answer or scope change is inferred here.

## 9. Validation performed

The default cleared-source `fetch.sh` completed, including explicit skips for
uncleared sources. Both inspections completed with `openpyxl==3.1.5`; the
2024 row-count assertions passed. `unzip -t` passed for the Irish archive.
Shell/Python syntax, every committed JSON document, ERA5 template input schemas,
the catalogue's CC-BY association, sample files below 1 MB, ignored raw paths and
`git diff --check` were checked. These are data-spike checks, not application CI,
legal clearance or numerical model validation. No Cargo project exists to test.
