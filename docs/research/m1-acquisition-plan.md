# M1 acquisition plan: sources without credentials

Publisher pages and catalogue/API documentation checked **2026-11-15 UTC**.
Based on the 54 external resource rows (`SRC-01`–`SRC-50`, including split IDs)
in [m1-data-inventory.md](m1-data-inventory.md). This is an acquisition plan,
not evidence of new acquisition, complete network coverage or M1 acceptance.
No datasets were fetched for this document; only public HTML pages, catalogue
metadata and API specifications were read. Previously recorded sizes come from
committed manifests, not new dataset requests.

## Scope and interpretation

The register covers 42 whole resource rows plus the public portion of `SRC-49`:
**43 entries, 23 open-commercial and 20 research-only**, preserving the inventory's
classifications. Eleven whole resource rows are excluded for credentials; the
gated portion of `SRC-49` is also excluded. `INT-01`/`INT-02` are internal
specifications, not downloads. The appendix accounts for every excluded row.

Unresolved public candidates remain in the register so that a broken link or an
HTTP 403 is not silently treated as an account requirement. **UNVERIFIED** means
the stated location, access, content, cadence, size or rights could not be
confirmed by reading publisher material. A verified catalogue link establishes
that a publisher advertises a resource; it does not establish that the download
works, has the expected contents or reproduces an earlier capture. No dataset
payload endpoint was called to make such a claim.

The governing sources are [D-56 and D-72](../requirements/03-decisions-log.md),
[FR-1–5, FR-19, FR-21 and NFR-1/2](../requirements/04-requirements.md), and
accepted [ADR 0002](../architecture/adr/0002-network-and-scenario-data-model.md)
and [ADR 0004](../architecture/adr/0004-storage-topology.md). Requirements links
below mean the input serves that requirement; acquisition alone does not pass it.
Other requirement IDs identify downstream uses of M1 inputs.

For `research-only`, raw data and restricted derived extracts may **never** be
committed or redistributed (FR-19.AC2). Use private storage outside the checkout,
such as `~/energy-data/raw/`, and commit only legally publishable provenance and
scripts. For `open-commercial`, raw files **may** be committed only within the
resource's grant, with attribution, third-party/personal-data exclusions and
provenance satisfied; the plan recommends keeping bulk raw files outside git too.
All rows remain subject to the absolute OSM/ODbL/share-alike exclusion, including
basemaps and upstream derivatives. Research classification is not a waiver of
access terms or retention restrictions. PC references point to
[TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md).

## Provenance manifest contract

Every resource, archive member and dated API batch needs **C** below plus its row's
profiles. These are required information fields for the M1 manifest design,
not a claim that an implemented schema already exists. Preserve compatibility
with existing `licence_status`, `source_url`, `bytes` and `sha256` fields. Do not
invent acquisition timestamps, hashes, sizes or publication dates before fetching.

| Profile | Fields to record |
|---|---|
| **C — common, every row** | `source_id`, sub-resource ID, publisher, title, publisher-page URL, catalogue/resource ID, requested URL and resolved URL without secrets, query parameters, access-check date/status, source release/version, acquisition timestamp UTC, source publication/revision timestamp and evidence (or explicitly unknown), effective coverage `[from,to)`, format/media type, raw relative path, bytes, SHA-256, archive-member names/hashes, licence name/status (exactly `open-commercial` or `research-only`), terms/evidence URLs and date, attribution text, exclusions/retention/redistribution conditions, raw-commit permission, PC clearance IDs, parent release hashes, transform/mapping/schema versions, derived hashes, missing/imputed coverage, knowledge mode and requirement IDs. Never publish tokens or signed URL query secrets. |
| **N — electrical network/fleet** | Operator/licence area, voltage and asset class, source asset aliases and permanent UUID crosswalk version, snapshot/study year, commissioned/planned/removed status and effective dates, node/GSP/BMU joins, units and electrical bases (kV/MVA, percent/per-unit/ohms), ratings meaning, topology/switching/transformer conventions, coverage numerator/denominator, inferred or absent parameters. |
| **G — geographic** | Coordinate reference system, geometry type, source layer/feature IDs, coordinates' own rights lineage and attribution; separate basemap provenance. Geometry does not establish electrical connectivity. |
| **T — interval observations** | Product/quantity, units, native cadence, source timezone, settlement date/period, UTC interval boundaries, local offset/fold, publication/revision/run identifier, direction/metering convention, expected/observed intervals and dimensions, duplicates/revisions, missing mask, resampling and aggregation policy. |
| **R — rules/proxies** | Document version and clause/table/page, publication and effective dates, jurisdiction/market/product/border, modifications, numerical parameter units, signed branch cut-set or unit-group crosswalk, distinction between operational rule, forecast and outturn. |
| **E — costs/FX** | Commodity/product, units and currency, quote/auction/reference timestamp, delivery/obligation period, price basis (nominal/real and base year), FX direction/numeraire, conversion/resampling/fill version, technology mapping and third-party price rights. |
| **K — calendars** | Jurisdiction, holiday date/type and observance, calendar and tzdata release/hash, `Europe/London`/`Europe/Dublin` mapping version, civil-to-settlement rules, source interval ID/offset/fold, trading-calendar distinction. |
| **W — weather** | Model/product/version, variables/units, grid/CRS/bounds/height, issue/valid time, accumulation convention, native cadence, coverage and missingness, spatial/resampling/power-curve versions and benchmark-only designation. |

All entries also serve **FR-19.AC1–3, FR-4.AC1 and NFR-1.AC1** through C.
T/K entries serve **FR-3.AC1/4 and FR-5.AC3**; N/T/E entries must preserve
original units for **NFR-2.AC2/3**. Row mappings below add the substantive uses.
Three clocks remain separate: source effective time, publication/revision time,
and acquisition time. Unknown historical publication evidence permits a labelled
reconstruction, not an as-of forecast-validation claim (D-70).

## Register conventions

Sizes use decimal MB/kB unless GiB is explicit. **Recorded** means a historical
manifest size; **advertised** means publisher metadata; **UNVERIFIED** means no
verified byte figure. Cadence distinguishes publication updates from observation
resolution. Inventory-only cadence claims are explicitly UNVERIFIED here.
Order codes P0–P7 are defined in the recommended sequence at the end; dependencies
are ingestion/interpretation dependencies unless specifically called a fetch gate.

## Public-source register

### SRC-01 — NESO ETYS Appendix B

- **URL/evidence:** [ETYS appendices](https://www.neso.energy/publications/electricity-ten-year-statement-etys/etys-documents-and-appendices), HTTP 200, advertises [Appendix B](https://www.neso.energy/document/383936/download). Page labels the 2025 edition published 30 June 2026; preserve both edition and publication date.
- **Format / cadence / size:** XLSX; annual editions, with revisions; recorded **1.24 MB**, 31 worksheets. Current payload size/content UNVERIFIED.
- **Rights / raw commit:** `research-only`, PC-1; **never**. General NESO terms do not associate this workbook with the portal open-data grant.
- **Manifest / order / requirements:** C+N; **P2**, then join SRC-03/04/05/17/20/48/49. FR-1.AC1/2/5, FR-2.AC1–3. The 2025/26 planning case cannot establish a commissioned 2024 fleet; ratings/impedance bases need interpretation.

### SRC-02 — NESO ETYS boundary GIS

- **URL/evidence:** [CKAN package metadata](https://api.neso.energy/api/3/action/package_show?id=997f4820-1ad4-499b-b1fe-4b8d3d7fbc72), HTTP 200, names [March 2025 ZIP](https://api.neso.energy/dataset/997f4820-1ad4-499b-b1fe-4b8d3d7fbc72/resource/e914fcec-1dc9-4f1f-97e7-59c0d9521bea/download/etys-boundary-gis-data-mar25.zip).
- **Format / cadence / size:** Shapefile ZIP; **ad hoc** per metadata (supersedes inventory's annual cadence); recorded **87.7 kB**.
- **Rights / raw commit:** `open-commercial`, [NESO Open Data Licence](https://www.neso.energy/data-portal/neso-open-licence); **yes, conditionally** under the grant and attribution rules above.
- **Manifest / order / requirements:** C+G+R; **P2**, then SRC-01/22/23 for boundary mapping. FR-2.AC2/3, FR-21. GIS boundary paths do not define signed electrical cut-sets.

### SRC-03 — NGED LTDS tables, four areas

- **URL/evidence:** [Publisher page](https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model) and [package metadata](https://connecteddata.nationalgrid.co.uk/api/3/action/package_show?id=ltds-tabular-model), both HTTP 200, advertise these exact resources:
  [circuits](https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/7892e22f-e489-49fa-a582-b1384006e4e2/download/ltds-table-1-circuit.csv),
  [two-winding transformers](https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/8c964d61-a7fb-41e7-99b4-6032d063a8c6/download/ltds-table-2a-two-winding-transformer.csv),
  [three-winding transformers](https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/56033622-8ed7-4425-916c-82a030df4822/download/ltds-table-2b-three-winding-transformer.csv),
  [nodes](https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/a92da9e4-2010-4406-b86a-825cd65761d6/download/ltds-attachment-8-node.csv).
- **Format / cadence / size:** CSV; page says **12 months**, while individual files can be revised between editions; recorded **4.81 MB total** (2.66 MB, 308 kB, 11.3 kB, 1.83 MB respectively).
- **Rights / raw commit:** `open-commercial`, [NGED Open Data Licence](https://www.nationalgrid.co.uk/open-data-licence); **yes, conditionally**.
- **Manifest / order / requirements:** C+N; **P2**, nodes before circuit/transformer joins, then SRC-01/04/49. FR-1.AC1/2, FR-2.AC1–3. Keep all four licence areas distinct; switchgear rows are not missing-impedance circuits.

### SRC-04 — NGED LTDS CIM, four areas

- **URL/evidence:** [Page](https://connecteddata.nationalgrid.co.uk/dataset/ltds-common-information-model) and [metadata](https://connecteddata.nationalgrid.co.uk/api/3/action/package_show?id=ltds-common-information-model), HTTP 200, advertise [East Midlands](https://connecteddata.nationalgrid.co.uk/dataset/7367f8a5-7714-4647-84db-dc35119bea78/resource/2ef4b5ac-3e57-4f84-a376-9014c146c1de/download/emids.zip), [West Midlands](https://connecteddata.nationalgrid.co.uk/dataset/7367f8a5-7714-4647-84db-dc35119bea78/resource/ab83d544-7a2a-4801-9d2a-4dbec5c8efbf/download/wmids.zip), [South Wales](https://connecteddata.nationalgrid.co.uk/dataset/7367f8a5-7714-4647-84db-dc35119bea78/resource/46575bac-99cf-4edd-933e-9df58c2b95e0/download/swales.zip), [South West](https://connecteddata.nationalgrid.co.uk/dataset/7367f8a5-7714-4647-84db-dc35119bea78/resource/38dbec57-e1ff-48c3-b4dc-c9759ebed3c0/download/swest.zip).
- **Format / cadence / size:** CIM RDF/XML ZIP; **12 months** per page, revisions possible; recorded **32.2 MB compressed total**, 5.3–11.2 MB per area. Expanded size UNVERIFIED.
- **Rights / raw commit:** `open-commercial`, NGED Open Data Licence; **yes, conditionally**.
- **Manifest / order / requirements:** C+N; **P2 after SRC-03** for reconciliation; record CIM model/profile/version, mRID aliases and EQ/SC/SCR/SYSCAP relationships. FR-1.AC1/2, FR-2.AC1–3. Do not combine incompatible edition snapshots silently.

### SRC-05 — SSEN SEPD LTDS tables and CIM EQ

- **URL/evidence:** [Page](https://data.ssen.co.uk/@ssen-distribution/sepd_long_term_development_statement) and [metadata](https://data-api.ssen.co.uk/api/3/action/package_show?id=sepd_long_term_development_statement), HTTP 200, advertise [May 2026 tables](https://data-api.ssen.co.uk/dataset/695078a0-bbd8-420b-8383-ee6bf7fc0045/resource/02c9431f-6126-4384-9f46-9079dbcd7057/download/southern-electric-power-distribution-long-term-development-statement-tables_may-2026.xlsx) and [CIM EQ ZIP](https://data-api.ssen.co.uk/dataset/695078a0-bbd8-420b-8383-ee6bf7fc0045/resource/d56de5a2-4150-48e9-986a-7c315f570a7b/download/ltds_sepd_2025-02_eq_2025-10-20_v1.0.zip). Use durable publisher URLs; transient signed redirects are not release identities.
- **Format / cadence / size:** XLSX + CIM XML ZIP; dated May/November table releases are listed, no metadata frequency set (**annual/semiannual schedule UNVERIFIED**); advertised/recorded **0.731 MB + 5.60 MB**. May/November 2024 tables are also advertised; their historical suitability is UNVERIFIED.
- **Rights / raw commit:** `open-commercial`, dataset-designated [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/); **yes, conditionally**.
- **Manifest / order / requirements:** C+N; **P2 after SRC-01**, reconcile tables and EQ before SRC-49 demand attachment. FR-1.AC1/2, FR-2.AC1–3. This SEPD resource is not evidence of complete SHEPD coverage.

### SRC-12 — EirGrid/SONI TYTFS 2024 study cases

- **URL/evidence:** [Study-file page](https://www.eirgrid.ie/all-island-ten-year-transmission-statement-2024-study-files-0), HTTP 200, advertises [publisher ZIP](https://www.eirgrid.ie/sites/default/files/publications/TYTFS2024_studyfiles.zip). The inventory/cache used the [CMS-host URL](https://cms.eirgrid.ie/sites/default/files/publications/TYTFS2024_studyfiles.zip); byte equivalence of the two hosts is **UNVERIFIED** without a future fetch/hash comparison.
- **Format / cadence / size:** PSS/E RAW/SEQ in ZIP, v33/v35; annual statement family per inventory, regular file-release cadence **UNVERIFIED**; recorded **11.95 MB**, 17 members.
- **Rights / raw commit:** `research-only`, PC-2; **never**. [EirGrid disclaimer](https://www.eirgrid.ie/disclaimer), no verified file-specific commercial grant.
- **Manifest / order / requirements:** C+N; **P2**, then SRC-13/21/24 and generator/bus crosswalks. FR-1.AC1/2, FR-2.AC1–3. Record WP/SV and study year; forecast cases are not operating history. Resolve 380/400 kV bases, zero ratings and three-winding conversion.

### SRC-13 — NIE Networks assets catalogue

- **URL/evidence:** [Portal](https://nienetworks.opendatasoft.com/pages/home/) and [catalogue metadata endpoint](https://nienetworks.opendatasoft.com/api/explore/v2.1/catalog/datasets?limit=30), HTTP 200. Verified dataset IDs include `nie-networks-assets-sites-grid-supply-points` (10 records), `nie-networks-assets-transmission-transformers` (100), and `nie-networks-assets-transmission-circuit-breakers` (312). This is **Opendatasoft**, not CKAN.
- **Format / cadence / size:** Metadata JSON; inventory proposes CSV/GeoJSON/Shapefile exports. **Export endpoint and unauthenticated row access UNVERIFIED**: metadata includes `data_visible: false`, despite public descriptions/counts; do not infer account requirements or public payload access. Frequency metadata is null; periodic updates **UNVERIFIED**. Byte size **UNVERIFIED**; counts above describe catalogue metadata only.
- **Rights / raw commit:** `open-commercial`, each named catalogue entry designates CC BY 4.0; **yes, conditionally**, for covered features only. Exclude unrelated basemaps/upstream share-alike geometry.
- **Manifest / order / requirements:** C+N+G; **P2 discovery, fetch gate: confirm public export without credentials**; then join SRC-12, not a replacement for its branch model. FR-1.AC1, FR-2.AC2/3. Failure of public export remains a gap, not permission to log in.

### SRC-14 — ESB Networks public maps/publications

- **URL/evidence:** [Publisher home](https://www.esbnetworks.ie/), HTTP 200, exposes public network/project navigation and [copyright policy](https://www.esbnetworks.ie/data-legal/copyright). A specific reusable network-model or map-download endpoint is **UNVERIFIED**; the home page is a discovery URL only. Account services are excluded.
- **Format / cadence / size:** HTML/GIS viewers and PDF publications per inventory; exact resource format/size **UNVERIFIED**; irregular publication schedule **UNVERIFIED**.
- **Rights / raw commit:** `research-only`, PC-5, inventory records all rights reserved; **never**.
- **Manifest / order / requirements:** C+N+G; **P7 discovery**, after SRC-12; no automatic network fetch until a specific public asset is identified and its terms inspected. FR-1.AC1, FR-2.AC2. Does not resolve the electrical-model deficit.

### SRC-15 — Elexon reference interconnectors

- **URL/evidence:** [Publisher OpenAPI](https://data.elexon.co.uk/swagger/v1/swagger.json), HTTP 200, documents `GET` [reference/interconnectors/all](https://data.elexon.co.uk/bmrs/api/v1/reference/interconnectors/all). Endpoint schema verified; payload not requested.
- **Format / cadence / size:** JSON (also CSV/XML supported by API); reference updates as records change, formal schedule **UNVERIFIED**; byte size **UNVERIFIED** (small reference list; inventory expects 10 links, not independently re-counted here).
- **Rights / raw commit:** `open-commercial`, [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/); **yes, conditionally**, including mandated BMRS attribution and third-party exclusions.
- **Manifest / order / requirements:** C+N; **P1**, before SRC-43 flows/SRC-32 losses. FR-1.AC1–5. Record seven external and three GB–Ireland links separately; reference IDs alone do not prove commissioning dates, especially Greenlink in 2024.

### SRC-17 — NESO TEC register

- **URL/evidence:** [Metadata](https://api.neso.energy/api/3/action/package_show?id=cbd45e54-e6e2-4a38-99f1-8de6fd96d7c1), HTTP 200, currently advertises [13 November 2026 CSV](https://api.neso.energy/dataset/cbd45e54-e6e2-4a38-99f1-8de6fd96d7c1/resource/17becbab-e3e8-473f-b303-3806f43a6a10/download/tec-register-13-november-2026.csv). The inventory's 10 November URL/hash identifies a previous capture only.
- **Format / cadence / size:** CSV; metadata describes updates **twice weekly** (supersedes inventory's monthly cadence); recorded **0.446 MB** for the old capture, current size **UNVERIFIED**.
- **Rights / raw commit:** `open-commercial`, NESO Open Data Licence; **yes, conditionally**.
- **Manifest / order / requirements:** C+N; **P1**, then reconcile SRC-18/19/20 and dated network snapshots at P2. FR-1.AC1/2/5, FR-2.AC2. Contracted capacity and queue status do not establish a 2024 commissioned fleet.

### SRC-18 — DESNZ REPD

- **URL/evidence:** [Inventory URL](https://www.gov.uk/government/publications/renewable-energy-planning-database-monthly-extract) redirects to the [quarterly publisher page](https://www.gov.uk/government/publications/renewable-energy-planning-database-quarterly-extract), HTTP 200. It advertises [Q3 2026 CSV](https://assets.publishing.service.gov.uk/media/6af42ff0a4004bdbe54e54a5/REPD_Publication_Q3_2026.csv) and [XLSX](https://assets.publishing.service.gov.uk/media/6af42ffdb83b3cae0d0a21c9/REPD_Publication_Q3_2026.xlsx); select CSV, not both duplicate encodings by default.
- **Format / cadence / size:** CSV/XLSX; **quarterly** since 2018, latest page update 13 November 2026; advertised CSV **4.98 MB** (publisher units). Old Q2 capture was 5.09 MB; do not attach its hash to Q3.
- **Rights / raw commit:** `open-commercial`, [OGL v3.0](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/); **yes, conditionally**.
- **Manifest / order / requirements:** C+N+G; **P1**, then join SRC-17/19/20 and network nodes. FR-1.AC1/2, FR-2.AC2, FR-3.AC3. Preserve project status/date/technology and coordinate provenance; do not assume planning records equal operating plant.

### SRC-19 — DESNZ DUKES 5.11 power stations

- **URL/evidence:** [Electricity chapter page](https://www.gov.uk/government/statistics/electricity-chapter-5-digest-of-united-kingdom-energy-statistics-dukes), HTTP 200, advertises [DUKES 5.11 XLSX](https://assets.publishing.service.gov.uk/media/6a6a365b0ddb7e4831c629f3/DUKES_5.11.xlsx), titled power stations in the UK, May 2026.
- **Format / cadence / size:** XLSX; annual DUKES publication with revisions; advertised **1.19 MB**, previously recorded **1.247 MB** in bytes. Pin edition and hash rather than reconcile differently rounded publisher units as exact sizes.
- **Rights / raw commit:** `open-commercial`, OGL v3.0; **yes, conditionally**.
- **Manifest / order / requirements:** C+N; **P1**, reconcile SRC-17/18/20 at P2. FR-1.AC1/2, FR-2.AC2, FR-3. Preserve station/unit and UK/GB/NI distinctions and effective fleet dates.

### SRC-20 — Elexon BM unit register

- **URL/evidence:** Publisher OpenAPI documents [reference/bmunits/all?format=json](https://data.elexon.co.uk/bmrs/api/v1/reference/bmunits/all?format=json). Schema verified; payload not requested.
- **Format / cadence / size:** JSON; reference revisions as registration changes, schedule **UNVERIFIED**; recorded **2.17 MB**, 3,134 records in the earlier capture.
- **Rights / raw commit:** `open-commercial`, BMRS licence; **yes, conditionally**.
- **Manifest / order / requirements:** C+N; **P1**, before SRC-43/50 and fleet/network crosswalks. FR-1.AC1/2, FR-2.AC2, FR-3.AC2. Current register is not a point-in-time 2024 register; preserve settlement/NGC IDs and effective registration evidence.

### SRC-21 — SEMO registered capacity/units

- **URL/evidence:** [Market-data hub](https://www.sem-o.com/market-data) and [static reports page](https://www.sem-o.com/market-data/static-reports), HTTP 200. Inventory candidates are [monthly capacity XML](https://reports.sem-o.com/documents/PUB_MnlyRegisteredCapacity_202412011000.xml) and [daily units XML](https://reports.sem-o.com/documents/PUB_DailyRegisteredUnits_202609140734.xml). **UNVERIFIED:** these exact filenames were not exposed in the pages read; inventory reports an empty-error live daily endpoint. No report payload requested.
- **Format / cadence / size:** XML; monthly/daily report families advertised, exact delivery schedule **UNVERIFIED**; recorded **464 kB + 290 kB**. December 2024 capacity and September 2026 daily registry are different vintages.
- **Rights / raw commit:** `research-only`, PC-3; **never**. Generic group licence does not establish report-specific open designation.
- **Manifest / order / requirements:** C+N+T; **P1 discovery; P2 ingest after SRC-12**, with `ResourceName`/unit/bus crosswalks. Fetch gate: resolve a functioning public report link, reject empty/error envelopes. FR-1.AC1/2, FR-2.AC2, FR-3.AC2.

### SRC-22 — NESO day-ahead boundary limits/flows

- **URL/evidence:** [Metadata](https://api.neso.energy/api/3/action/package_show?id=cf3cbc92-2d5d-4c2b-bd29-e11a21070b26), HTTP 200, advertises [CSV v1.5](https://api.neso.energy/dataset/cf3cbc92-2d5d-4c2b-bd29-e11a21070b26/resource/38a18ec1-9e40-465d-93fb-301e80fd1352/download/day-ahead-constraints-limits-and-flow-output-v1.5.csv). Metadata also advertises England/Wales and Scotland boundary-direction diagrams.
- **Format / cadence / size:** CSV; **daily on weekdays** per metadata, half-hour delivery intervals; recorded **29.27 MB**, 764,304 rows. Metadata says the rolling file retains three previous years plus current year: preserve 2024 before it ages out.
- **Rights / raw commit:** `open-commercial`, NESO Open Data Licence; **yes, conditionally**.
- **Manifest / order / requirements:** C+T+R; **P3**, after SRC-35/01/02 for interpretation; capture raw early, derive cut-sets only with electrical evidence. FR-2.AC3, FR-3.AC2, FR-21. Preserve `GMT_BST` offset/fold and publication vintage; do not deduplicate repeated local times. Boundary diagrams/sign arrows still do not prove complete branch cut-sets.

### SRC-23 — NESO 24-month boundary limits

- **URL/evidence:** [Metadata](https://api.neso.energy/api/3/action/package_show?id=d515b4a9-60a1-489c-a126-004efc04f121), HTTP 200, advertises [November 2026 CSV](https://api.neso.energy/dataset/d515b4a9-60a1-489c-a126-004efc04f121/resource/3c359e33-3dac-4bdd-87d1-efbf4cbc2f07/download/24-months-ahead-constraint-limit_nov26.csv).
- **Format / cadence / size:** CSV; **monthly** publication of weekly horizon values; recorded **7.69 kB**, 109 rows.
- **Rights / raw commit:** `open-commercial`, NESO Open Data Licence; **yes, conditionally**.
- **Manifest / order / requirements:** C+T+R; **P3 after SRC-01/02/22** for common boundary aliases. FR-3.AC2, FR-21. Preserve schema changes in boundary column IDs; this 2026–28 horizon is a forward-scenario input, not 2024 limits.

### SRC-24 — EirGrid/SONI quarter-hour system data

- **URL/evidence:** [System-data page](https://www.eirgrid.ie/grid/system-and-renewable-data-reports), HTTP 200, advertises quarter-hour spreadsheets and historical data. Inventory target [2024 XLSX](https://cms.eirgrid.ie/sites/default/files/publications/System-Data-Qtr-Hourly-2024.xlsx) is **UNVERIFIED as a current page link**: the page read exposes the 2026 workbook, not this exact historical URL. Earlier acquisition is evidenced by the network manifest.
- **Format / cadence / size:** XLSX; quarter-hour observations, historical annual file/revision schedule **UNVERIFIED**; recorded **12.10 MB**, 35,136 quarter-hours for 2024.
- **Rights / raw commit:** `research-only`, PC-2; **never**. File-specific association with the group open licence remains UNVERIFIED.
- **Manifest / order / requirements:** C+T; **P3 after SRC-35/12/21** for interval/market interpretation. FR-3.AC1–4, FR-21 (observational check only), downstream FR-12. Realised SNSP is not an SNSP ceiling or a minimum-synchronous-units rule.

### SRC-25 — NESO operability publications

- **URL/evidence:** Inventory URL `/publications/operability-strategy-report` returns 404. Verified replacement [OSR and Electricity Markets Roadmap page](https://www.neso.energy/publications/operability-strategy-report-and-electricity-markets-roadmap), HTTP 200, advertises [OSR 2026](https://www.neso.energy/document/379601/download) and archived [OSR 2024](https://www.neso.energy/document/299926/download). The archive's displayed publication date is anomalous relative to the edition; effective dates must be checked inside the future acquired document.
- **Format / cadence / size:** Reports advertised for annual editions; PDF expected, exact response format/size **UNVERIFIED** (files not requested).
- **Rights / raw commit:** `research-only`, PC-5; **never**. General NESO website terms, no verified dataset-specific grant.
- **Manifest / order / requirements:** C+R; **P3 after SRC-01/22**, then map any verified unit groups/cut-sets. FR-3.AC2, FR-21. Specific 2024 inertia floors, contribution factors and minimum-unit rules remain **UNVERIFIED**; current strategy targets are not historical operational rules.

### SRC-26 — Elexon BSC legal text

- **URL/evidence:** [Inventory code-hub route](https://www.elexon.co.uk/bsc-and-codes/balancing-settlement-code/bsc-sections/) redirects to [digital BSC](https://bscdocs.elexon.co.uk/bsc), which returned **HTTP 403**. Access, exact Section Q/T download locations and 2024 versions are **UNVERIFIED**; no account requirement established by this response.
- **Format / cadence / size:** HTML/PDF per inventory, exact files/size **UNVERIFIED**; changes through code modifications, regular release schedule **UNVERIFIED**.
- **Rights / raw commit:** `research-only`, PC-5; **never**. The BMRS dataset licence must not be applied to statutory code text.
- **Manifest / order / requirements:** C+R; **P3 discovery**, before interpretation/freeze of SRC-43/50. FR-1.AC2, FR-3.AC2, downstream FR-7.AC2/3. Fetch gate: accessible public dated text; do not treat current code as a 2024 rules lock.

### SRC-27 — SEMO TSC and SEMOpx auction rules

- **URL/evidence:** Inventory Part B URL returns 404. Verified [Balancing Market Rules page](https://www.sem-o.com/rules-and-modifications/balancing-market-modifications/market-rules), HTTP 200, links [TSC DOCX](https://www.sem-o.com/sites/semo/files/2026-10/Trading%20and%20Settlement%20Code.docx), glossary/appendices/procedures. It explains Part B is the enduring code and that Part B naming is no longer used. Also read [SEMOpx Market Rules](https://www.semopx.com/rules-and-monitoring/market-rules), HTTP 200: [Rules](https://www.semopx.com/sites/semopx/files/2026-02/SEMOpx%20Rules.docx) and [Operating Procedures](https://www.semopx.com/sites/semopx/files/2026-02/SEMOpx%20Operating%20Procedures.docx).
- **Format / cadence / size:** DOCX links verified (corrects inventory's PDF-only expectation); modified as rules change; bytes and historical 2024 versions **UNVERIFIED**.
- **Rights / raw commit:** `research-only`, PC-5; **never**. No affirmative commercial redistribution grant verified.
- **Manifest / order / requirements:** C+R; **P3 before SRC-44 interpretation**, after market/interval definitions. FR-1.AC2, FR-3.AC2, downstream FR-7.AC3/4. TSC and auction procedures are separate products; 2026 documents cannot establish the 2024 baseline.

### SRC-28 — JAO capacity allocation rules

- **URL/evidence:** [Terms page](https://www.jao.eu/terms-conditions), HTTP 200, links border navigation including [IFA/IFA2](https://www.jao.eu/trading-ifa-ifa2) and [Nemo Link](https://www.jao.eu/trading-nemo-link). Those border pages/documents were not read in this check: **UNVERIFIED** as historical rules sources. The terms URL itself is verified, not a dated capacity-allocation document.
- **Format / cadence / size:** HTML terms and rule PDFs per inventory; exact rule format/bytes **UNVERIFIED**; annual/modification cadence **UNVERIFIED**.
- **Rights / raw commit:** `research-only`, PC-5; **never**. Website terms do not confer a reusable rules database grant.
- **Manifest / order / requirements:** C+R; **P3 after SRC-15**, before SRC-43 flow-policy interpretation. FR-1.AC2/5, downstream FR-7.AC4. Pin border, product, direction and Brexit-effective regime separately.

### SRC-29 — NESO historical demand, 2024

- **URL/evidence:** [Metadata](https://api.neso.energy/api/3/action/package_show?id=historic-demand-data), HTTP 200, advertises exact [2024 CSV](https://api.neso.energy/dataset/8f2fe0af-871c-488d-8bad-960426f24601/resource/f6d02c0f-957b-48cb-82ee-09003f2ba759/download/demanddata_2024.csv).
- **Format / cadence / size:** CSV, half-hour observations; published **21 days in arrears**, later corrections explicitly described; recorded **1.81 MB**, 17,568 rows.
- **Rights / raw commit:** `open-commercial`, NESO Open Data Licence; **yes, conditionally**.
- **Manifest / order / requirements:** C+T; **P3 after SRC-35**, then SRC-30/31/48/49 for reconciliation/allocation. FR-3.AC1/3/4, downstream FR-12. Starting gross-demand identity is `ND + embedded solar + embedded wind`; never subtract pumping from ND again. Version all further station-load/loss/storage boundary adjustments.

### SRC-30 — Elexon demand outturn

- **URL/evidence:** OpenAPI documents [demand/outturn](https://data.elexon.co.uk/bmrs/api/v1/demand/outturn). Use `?settlementDateFrom={d}&settlementDateTo={d}&format=json`, one settlement date per batch; date query names verified in schema.
- **Format / cadence / size:** JSON; half-hour initial demand observations, near-real-time/daily publication timing **UNVERIFIED** from the schema alone; recorded **3.56 MB/year**, 366 files.
- **Rights / raw commit:** `open-commercial`, BMRS licence; **yes, conditionally**.
- **Manifest / order / requirements:** C+T; **P3 after SRC-35/29**; SRC-43 reuses these files, never counts them twice. FR-3.AC1/3/4. Keep INDO/ITSDO and initial/revised demand distinct.

### SRC-31 — Elexon FUELHH

- **URL/evidence:** OpenAPI documents [datasets/FUELHH/stream](https://data.elexon.co.uk/bmrs/api/v1/datasets/FUELHH/stream), queried with `?settlementDateFrom={d}&settlementDateTo={d}`. Schema verified; no payload queried.
- **Format / cadence / size:** JSON stream; half-hour observations, revision/publication latency **UNVERIFIED**; recorded **61.77 MB/year**, 366 files, 340,894 records.
- **Rights / raw commit:** `open-commercial`, BMRS licence; **yes, conditionally**.
- **Manifest / order / requirements:** C+T; **P3 after SRC-35/29**, included once in SRC-43. FR-3.AC1/3/4, downstream FR-12 mix checks. No embedded solar series and no separate embedded wind series; use SRC-29 for GB embedded estimates.

### SRC-32 — Interconnector physical losses

- **URL/evidence:** [BritNed](https://www.britned.com/) and [ElecLink](https://www.eleclink.co.uk/), HTTP 200; ElecLink links its verified [loss-factor page](https://www.eleclink.co.uk/publications/loss-factor), HTTP 200, advertising a [loss guide PDF](https://cdn.prod.website-files.com/67b2dbc715938e08fe43fee1/67d9d09d90ce7e920a93ee31_ElecLink%20Loss%20Factor%20v2.pdf). Page states 2.9% end-to-end and 1.471% mid-interconnector; **2024 applicability, directional convention and converter/cable decomposition UNVERIFIED**. [Greenlink operations](https://greenlink.ie/operations/) returned 403. No complete 10-link parameter set verified.
- **Format / cadence / size:** HTML/PDF guidance/filings; updates on operator changes, schedule and bytes **UNVERIFIED**.
- **Rights / raw commit:** `research-only`, PC-5; **never**.
- **Manifest / order / requirements:** C+N+R+T; **P3 after SRC-15**, before lossy balance fixtures. FR-1.AC3/5, FR-3.AC2, NFR-2. Keep IFA, IFA2, ElecLink, BritNed, Nemo, NSL, Viking, Moyle, EWIC and Greenlink separate, with both directions and sending/receiving metering boundaries. Do not apply ElecLink's current figures to other links or invent missing coefficients.

### SRC-33 — GB/NI public holidays

- **URL/evidence:** [Government API catalogue](https://www.api.gov.uk/gds/bank-holidays/), HTTP 200, explicitly names [bank-holidays.json](https://www.gov.uk/bank-holidays.json). [Human-readable calendar](https://www.gov.uk/bank-holidays), HTTP 200, also read. JSON payload not fetched.
- **Format / cadence / size:** JSON; revised when dates announced/corrected, formal schedule **UNVERIFIED**; bytes and current 2024 retention **UNVERIFIED** (inventory previously observed 2019–28).
- **Rights / raw commit:** `open-commercial`, OGL v3.0; **yes, conditionally**.
- **Manifest / order / requirements:** C+K; **P1 after SRC-35**. FR-3.AC1/4, FR-5. England/Wales, Scotland and Northern Ireland calendars differ; bank holidays are not exchange trading or settlement-publication calendars.

### SRC-34 — Irish public holidays

- **URL/evidence:** [Citizens Information public-holidays page](https://www.citizensinformation.ie/en/employment/employment-rights-and-conditions/leave-and-holidays/public-holidays/) returned **403**. Public content/access, exact 2024 dates, machine-readable endpoint and commercial grant remain **UNVERIFIED**. This response does not prove a login requirement.
- **Format / cadence / size:** HTML per inventory; updates with statutory/observance changes, schedule/bytes **UNVERIFIED**.
- **Rights / raw commit:** `research-only`, PC-5; **never**.
- **Manifest / order / requirements:** C+K; **P7 discovery after SRC-35**; fetch only when an accessible public publisher source is located. FR-3.AC1/4, FR-5. Do not silently substitute Northern Ireland's calendar for the Republic.

### SRC-35 — IANA tzdata

- **URL/evidence:** [Time-zones page](https://www.iana.org/time-zones) and [release 2026f](https://www.iana.org/time-zones/releases/2026f), HTTP 200, advertise [tzdata2026f.tar.gz](https://data.iana.org/time-zones/releases/tzdata2026f.tar.gz). Use the explicit release, not a mutable `latest` URL or unrecorded system library.
- **Format / cadence / size:** Text timezone rules in tar.gz; **ad hoc** political/clock-rule releases; advertised **472.0 kB** compressed, released 27 October 2026.
- **Rights / raw commit:** `open-commercial`, public-domain data per inventory; **yes, conditionally**, retaining applicable notices. Release payload/its notices were not inspected here; any different rights discovered at acquisition require review.
- **Manifest / order / requirements:** C+K; **P1 first**; no source dependency after P0. FR-3.AC1, FR-4, FR-5, NFR-1. Pin both Europe/London and Europe/Dublin, leap-day and 46/50-period clock-change days; preserve repeated civil hours.
