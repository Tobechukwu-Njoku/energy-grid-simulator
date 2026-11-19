# M1 acquisition plan: sources without credentials

Publisher pages and catalogue/API documentation checked **2026-11-15 UTC**.
Reconciled with main `b13145f` and PR #58 on **2026-11-19 UTC**; candidate
checks inherited from the linked analysis are not new payload verification.
Based on the 54 external resource rows (`SRC-01`–`SRC-50`, including split IDs)
in [m1-data-inventory.md](m1-data-inventory.md). This is an acquisition plan,
not evidence of new acquisition, complete network coverage or M1 acceptance.
No bulk or operational datasets were fetched for this document; public HTML
pages, catalogue metadata and API specifications were read. The sole release
payload check in this revision was the small IANA tzdata archive in temporary
storage to verify the canonical hash (see candidate 8.3 below); it is not committed.
Previously recorded sizes come from
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
works, has the expected contents or reproduces an earlier capture. No operational dataset
payload endpoint was called to make such a claim.

The governing sources are [D-30 (per-dataset rights/share-alike ban), D-56
(research classification), D-57 (free historical prices), D-60/D-71 (all-GB
132 kV and missing-parameter acquisition), D-62 (bulk acquisition roles), D-65
(target-only heavy work), D-70 (reconstruction/as-of) and D-72 (baseline)](../requirements/03-decisions-log.md),
[FR-1–5, FR-19, FR-21 and NFR-1/2](../requirements/04-requirements.md), and
accepted [ADR 0002](../architecture/adr/0002-network-and-scenario-data-model.md)
and [ADR 0004](../architecture/adr/0004-storage-topology.md). Requirements links
below mean the input serves that requirement; acquisition alone does not pass it.
Other requirement IDs identify downstream uses of M1 inputs.

For `research-only`, raw data and restricted derived extracts may **never** be
committed or redistributed (FR-19.AC2). The raw store must be on the separate
D-65 target, or a declared private filesystem/object store under ADR 0004,
never the coding container. Record storage host/system identity, absolute private
root or object-store namespace, immutable relative key and access scope in C.
No concrete target host/path is provisioned by this plan; P0 blocks bulk fetching
until one is declared. D-62 acquisition runs may write only to their authorised
raw staging namespace there; verification precedes admission/promotion.

For `open-commercial`, raw files may be committed **only as registered samples
under `spikes/data/samples/*`**, with the provenance record and sample-source
mapping required by `xtask/policy/repository.toml`, rights and attribution checks.
Exact `data_metadata_files` paths allow metadata, never arbitrary payloads.
All other raw files stay outside git; a broader path requires an owner policy
change first. Synthetic committed fixtures follow the same sample-path rule;
tests may instead construct values in memory/temporary directories. No CI rule
is loosened. Here **sample-only** always means this entire admission contract.
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
| **C — common, every row** | `source_id`, sub-resource ID, publisher, title, publisher-page URL, catalogue/resource ID, requested URL and resolved URL without secrets, query parameters, access-check date/status, source release/version, acquisition timestamp UTC, source publication/revision timestamp and evidence (or explicitly unknown), effective coverage `[from,to)`, format/media type, storage host/root or object-store namespace and authorised writer scope, raw relative path, bytes, SHA-256, archive-member names/hashes, licence name/status (exactly `open-commercial` or `research-only`), terms/evidence URLs and date, attribution text, exclusions/retention/redistribution conditions, raw-commit permission, PC clearance IDs, parent release hashes, transform/mapping/schema versions, derived hashes, missing/imputed coverage, knowledge mode and requirement IDs. Never publish tokens or signed URL query secrets. |
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
- **Manifest / order / requirements:** C+N; **P2**, then join SRC-03/04/05/17/20/48/49. FR-1.AC1/2/5, FR-2.AC1–3. The primary historical edition is [ETYS 2024 Appendix B](https://www.neso.energy/document/351916/download) (PR #58 candidate 1.1), research-only PC-1, raw commit never. Acquire it before the labelled 2025 forward snapshot; neither alone establishes a fully reconciled commissioned 2024 fleet; ratings/impedance bases need interpretation.

### SRC-02 — NESO ETYS boundary GIS

- **URL/evidence:** [CKAN package metadata](https://api.neso.energy/api/3/action/package_show?id=997f4820-1ad4-499b-b1fe-4b8d3d7fbc72), HTTP 200, names [March 2025 ZIP](https://api.neso.energy/dataset/997f4820-1ad4-499b-b1fe-4b8d3d7fbc72/resource/e914fcec-1dc9-4f1f-97e7-59c0d9521bea/download/etys-boundary-gis-data-mar25.zip).
- **Format / cadence / size:** Shapefile ZIP; **ad hoc** per metadata (supersedes inventory's annual cadence); recorded **87.7 kB**.
- **Rights / raw commit:** `open-commercial`, [NESO Open Data Licence](https://www.neso.energy/data-portal/neso-open-licence); **sample-only** under the grant and attribution rules above.
- **Manifest / order / requirements:** C+G+R; **P2**, then SRC-01/22/23 for boundary mapping. FR-2.AC2/3, FR-21. GIS boundary paths do not define signed electrical cut-sets.

### SRC-03 — NGED LTDS tables, four areas

- **URL/evidence:** [Publisher page](https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model) and [package metadata](https://connecteddata.nationalgrid.co.uk/api/3/action/package_show?id=ltds-tabular-model), both HTTP 200, advertise these exact resources:
  [circuits](https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/7892e22f-e489-49fa-a582-b1384006e4e2/download/ltds-table-1-circuit.csv),
  [two-winding transformers](https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/8c964d61-a7fb-41e7-99b4-6032d063a8c6/download/ltds-table-2a-two-winding-transformer.csv),
  [three-winding transformers](https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/56033622-8ed7-4425-916c-82a030df4822/download/ltds-table-2b-three-winding-transformer.csv),
  [nodes](https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/a92da9e4-2010-4406-b86a-825cd65761d6/download/ltds-attachment-8-node.csv).
- **Format / cadence / size:** CSV; page says **12 months**, while individual files can be revised between editions; recorded **4.81 MB total** (2.66 MB, 308 kB, 11.3 kB, 1.83 MB respectively).
- **Rights / raw commit:** `open-commercial`, [NGED Open Data Licence](https://www.nationalgrid.co.uk/open-data-licence); **sample-only**.
- **Manifest / order / requirements:** C+N; **P2**, nodes before circuit/transformer joins, then SRC-01/04/49. FR-1.AC1/2, FR-2.AC1–3. Pin the **November 2024 LTDS edition** for the holdout (PR #58 candidate 1.3), retaining edition/resource ID, effective dates and hash before admission; the mutable URLs above are discovery locators, not a dated lock. Exact archived file identities remain UNVERIFIED until acquisition; block substitution with current files. Keep all four licence areas distinct; switchgear rows are not missing-impedance circuits.

### SRC-04 — NGED LTDS CIM, four areas

- **URL/evidence:** [Page](https://connecteddata.nationalgrid.co.uk/dataset/ltds-common-information-model) and [metadata](https://connecteddata.nationalgrid.co.uk/api/3/action/package_show?id=ltds-common-information-model), HTTP 200, advertise [East Midlands](https://connecteddata.nationalgrid.co.uk/dataset/7367f8a5-7714-4647-84db-dc35119bea78/resource/2ef4b5ac-3e57-4f84-a376-9014c146c1de/download/emids.zip), [West Midlands](https://connecteddata.nationalgrid.co.uk/dataset/7367f8a5-7714-4647-84db-dc35119bea78/resource/ab83d544-7a2a-4801-9d2a-4dbec5c8efbf/download/wmids.zip), [South Wales](https://connecteddata.nationalgrid.co.uk/dataset/7367f8a5-7714-4647-84db-dc35119bea78/resource/46575bac-99cf-4edd-933e-9df58c2b95e0/download/swales.zip), [South West](https://connecteddata.nationalgrid.co.uk/dataset/7367f8a5-7714-4647-84db-dc35119bea78/resource/38dbec57-e1ff-48c3-b4dc-c9759ebed3c0/download/swest.zip).
- **Format / cadence / size:** CIM RDF/XML ZIP; **12 months** per page, revisions possible; recorded **32.2 MB compressed total**, 5.3–11.2 MB per area. Expanded size UNVERIFIED.
- **Rights / raw commit:** `open-commercial`, NGED Open Data Licence; **sample-only**.
- **Manifest / order / requirements:** C+N; **P2 after SRC-03** for reconciliation; record CIM model/profile/version, mRID aliases and EQ/SC/SCR/SYSCAP relationships. FR-1.AC1/2, FR-2.AC1–3. Do not combine incompatible edition snapshots silently.

### SRC-05 — SSEN SEPD LTDS tables and CIM EQ

- **URL/evidence:** [Page](https://data.ssen.co.uk/@ssen-distribution/sepd_long_term_development_statement) and [metadata](https://data-api.ssen.co.uk/api/3/action/package_show?id=sepd_long_term_development_statement), HTTP 200, advertise [May 2026 tables](https://data-api.ssen.co.uk/dataset/695078a0-bbd8-420b-8383-ee6bf7fc0045/resource/02c9431f-6126-4384-9f46-9079dbcd7057/download/southern-electric-power-distribution-long-term-development-statement-tables_may-2026.xlsx) and [CIM EQ ZIP](https://data-api.ssen.co.uk/dataset/695078a0-bbd8-420b-8383-ee6bf7fc0045/resource/d56de5a2-4150-48e9-986a-7c315f570a7b/download/ltds_sepd_2025-02_eq_2025-10-20_v1.0.zip). Use durable publisher URLs; transient signed redirects are not release identities.
- **Format / cadence / size:** XLSX + CIM XML ZIP; dated May/November table releases are listed, no metadata frequency set (**annual/semiannual schedule UNVERIFIED**); advertised/recorded **0.731 MB + 5.60 MB**. Use **November 2024 tables** as the holdout acquisition edition (PR #58 candidate 1.3); May 2026 tables/CIM remain separately labelled forward/current snapshots. Archived resource ID/hash and historical commissioned coverage must still be verified before admission; do not substitute current URLs.
- **Rights / raw commit:** `open-commercial`, dataset-designated [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/); **sample-only**.
- **Manifest / order / requirements:** C+N; **P2 after SRC-01**, reconcile tables and EQ before SRC-49 demand attachment. FR-1.AC1/2, FR-2.AC1–3. This SEPD resource is not evidence of complete SHEPD coverage.

### SRC-12 — EirGrid/SONI TYTFS 2024 study cases

- **URL/evidence:** [Study-file page](https://www.eirgrid.ie/all-island-ten-year-transmission-statement-2024-study-files-0), HTTP 200, advertises [publisher ZIP](https://www.eirgrid.ie/sites/default/files/publications/TYTFS2024_studyfiles.zip). The inventory/cache used the [CMS-host URL](https://cms.eirgrid.ie/sites/default/files/publications/TYTFS2024_studyfiles.zip); byte equivalence of the two hosts is **UNVERIFIED** without a future fetch/hash comparison.
- **Format / cadence / size:** PSS/E RAW/SEQ in ZIP, v33/v35; annual statement family per inventory, regular file-release cadence **UNVERIFIED**; recorded **11.95 MB**, 17 members.
- **Rights / raw commit:** `research-only`, PC-2; **never**. [EirGrid disclaimer](https://www.eirgrid.ie/disclaimer), no verified file-specific commercial grant.
- **Manifest / order / requirements:** C+N; **P2**, then SRC-13/21/24 and generator/bus crosswalks. FR-1.AC1/2, FR-2.AC1–3. Record WP/SV and study year; forecast cases are not operating history. Resolve 380/400 kV bases, zero ratings and three-winding conversion.

### SRC-13 — NIE Networks assets catalogue

- **URL/evidence:** [Portal](https://nienetworks.opendatasoft.com/pages/home/) and [catalogue metadata endpoint](https://nienetworks.opendatasoft.com/api/explore/v2.1/catalog/datasets?limit=30), HTTP 200. Verified dataset IDs include `nie-networks-assets-sites-grid-supply-points` (10 records), `nie-networks-assets-transmission-transformers` (100), and `nie-networks-assets-transmission-circuit-breakers` (312). This is **Opendatasoft**, not CKAN.
- **Format / cadence / size:** Metadata JSON; inventory proposes CSV/GeoJSON/Shapefile exports. **Export endpoint and unauthenticated row access UNVERIFIED**: metadata includes `data_visible: false`, despite public descriptions/counts; do not infer account requirements or public payload access. Frequency metadata is null; periodic updates **UNVERIFIED**. Byte size **UNVERIFIED**; counts above describe catalogue metadata only.
- **Rights / raw commit:** `open-commercial`, each named catalogue entry designates CC BY 4.0; **sample-only**, for covered features only. Exclude unrelated basemaps/upstream share-alike geometry.
- **Manifest / order / requirements:** C+N+G; **P2 discovery, fetch gate: confirm public export without credentials**; then join SRC-12, not a replacement for its branch model. FR-1.AC1, FR-2.AC2/3. Failure of public export remains a gap, not permission to log in.

### SRC-14 — ESB Networks public maps/publications

- **URL/evidence:** [Publisher home](https://www.esbnetworks.ie/), HTTP 200, exposes public network/project navigation and [copyright policy](https://www.esbnetworks.ie/data-legal/copyright). A specific reusable network-model or map-download endpoint is **UNVERIFIED**; the home page is a discovery URL only. Account services are excluded.
- **Format / cadence / size:** HTML/GIS viewers and PDF publications per inventory; exact resource format/size **UNVERIFIED**; irregular publication schedule **UNVERIFIED**.
- **Rights / raw commit:** `research-only`, PC-5, inventory records all rights reserved; **never**.
- **Manifest / order / requirements:** C+N+G; **P7 discovery**, after SRC-12; no automatic network fetch until a specific public asset is identified and its terms inspected. FR-1.AC1, FR-2.AC2. Does not resolve the electrical-model deficit.

### SRC-15 — Elexon reference interconnectors

- **URL/evidence:** [Publisher OpenAPI](https://data.elexon.co.uk/swagger/v1/swagger.json), HTTP 200, documents `GET` [reference/interconnectors/all](https://data.elexon.co.uk/bmrs/api/v1/reference/interconnectors/all). Endpoint schema verified; payload not requested.
- **Format / cadence / size:** JSON (also CSV/XML supported by API); reference updates as records change, formal schedule **UNVERIFIED**; byte size **UNVERIFIED** (small reference list; inventory expects 10 links, not independently re-counted here).
- **Rights / raw commit:** `open-commercial`, [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/); **sample-only**, including mandated BMRS attribution and third-party exclusions.
- **Manifest / order / requirements:** C+N; **P1**, before SRC-43 flows/SRC-32 losses. FR-1.AC1–5. Record seven external and three GB–Ireland links separately; reference IDs alone do not prove commissioning dates, especially Greenlink in 2024.

### SRC-17 — NESO TEC register

- **URL/evidence:** [Metadata](https://api.neso.energy/api/3/action/package_show?id=cbd45e54-e6e2-4a38-99f1-8de6fd96d7c1), HTTP 200, currently advertises [13 November 2026 CSV](https://api.neso.energy/dataset/cbd45e54-e6e2-4a38-99f1-8de6fd96d7c1/resource/17becbab-e3e8-473f-b303-3806f43a6a10/download/tec-register-13-november-2026.csv). The inventory's 10 November URL/hash identifies a previous capture only.
- **Format / cadence / size:** CSV; metadata describes updates **twice weekly** (supersedes inventory's monthly cadence); recorded **0.446 MB** for the old capture, current size **UNVERIFIED**.
- **Rights / raw commit:** `open-commercial`, NESO Open Data Licence; **sample-only**.
- **Manifest / order / requirements:** C+N; **P1**, then reconcile SRC-18/19/20 and dated network snapshots at P2. FR-1.AC1/2/5, FR-2.AC2. Contracted capacity and queue status do not establish a 2024 commissioned fleet.

### SRC-18 — DESNZ REPD

- **URL/evidence:** [Inventory URL](https://www.gov.uk/government/publications/renewable-energy-planning-database-monthly-extract) redirects to the [quarterly publisher page](https://www.gov.uk/government/publications/renewable-energy-planning-database-quarterly-extract), HTTP 200. It advertises [Q3 2026 CSV](https://assets.publishing.service.gov.uk/media/6af42ff0a4004bdbe54e54a5/REPD_Publication_Q3_2026.csv) and [XLSX](https://assets.publishing.service.gov.uk/media/6af42ffdb83b3cae0d0a21c9/REPD_Publication_Q3_2026.xlsx); select CSV, not both duplicate encodings by default.
- **Format / cadence / size:** CSV/XLSX; **quarterly** since 2018, latest page update 13 November 2026; advertised CSV **4.98 MB** (publisher units). Old Q2 capture was 5.09 MB; do not attach its hash to Q3.
- **Rights / raw commit:** `open-commercial`, [OGL v3.0](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/); **sample-only**.
- **Manifest / order / requirements:** C+N+G; **P1**, then join SRC-17/19/20 and network nodes. FR-1.AC1/2, FR-2.AC2, FR-3.AC3. Preserve project status/date/technology and coordinate provenance; do not assume planning records equal operating plant.

### SRC-19 — DESNZ DUKES 5.11 power stations

- **URL/evidence:** [Electricity chapter page](https://www.gov.uk/government/statistics/electricity-chapter-5-digest-of-united-kingdom-energy-statistics-dukes), HTTP 200, advertises [DUKES 5.11 XLSX](https://assets.publishing.service.gov.uk/media/6a6a365b0ddb7e4831c629f3/DUKES_5.11.xlsx), titled power stations in the UK, May 2026.
- **Format / cadence / size:** XLSX; annual DUKES publication with revisions; advertised **1.19 MB**, previously recorded **1.247 MB** in bytes. Pin edition and hash rather than reconcile differently rounded publisher units as exact sizes.
- **Rights / raw commit:** `open-commercial`, OGL v3.0; **sample-only**.
- **Manifest / order / requirements:** C+N; **P1**, reconcile SRC-17/18/20 at P2. FR-1.AC1/2, FR-2.AC2, FR-3. Preserve station/unit and UK/GB/NI distinctions and effective fleet dates.

### SRC-20 — Elexon BM unit register

- **URL/evidence:** Publisher OpenAPI documents [reference/bmunits/all?format=json](https://data.elexon.co.uk/bmrs/api/v1/reference/bmunits/all?format=json). Schema verified; payload not requested.
- **Format / cadence / size:** JSON; reference revisions as registration changes, schedule **UNVERIFIED**; recorded **2.17 MB**, 3,134 records in the earlier capture.
- **Rights / raw commit:** `open-commercial`, BMRS licence; **sample-only**.
- **Manifest / order / requirements:** C+N; **P1**, before SRC-43/50 and fleet/network crosswalks. FR-1.AC1/2, FR-2.AC2, FR-3.AC2. Current register is not a point-in-time 2024 register; preserve settlement/NGC IDs and effective registration evidence.

### SRC-21 — SEMO registered capacity/units

- **URL/evidence:** [Market-data hub](https://www.sem-o.com/market-data) and [static reports page](https://www.sem-o.com/market-data/static-reports), HTTP 200. Inventory candidates are [monthly capacity XML](https://reports.sem-o.com/documents/PUB_MnlyRegisteredCapacity_202412011000.xml) and [daily units XML](https://reports.sem-o.com/documents/PUB_DailyRegisteredUnits_202609140734.xml). **UNVERIFIED:** these exact filenames were not exposed in the pages read; inventory reports an empty-error live daily endpoint. No report payload requested.
- **Format / cadence / size:** XML; monthly/daily report families advertised, exact delivery schedule **UNVERIFIED**; recorded **464 kB + 290 kB**. December 2024 capacity and September 2026 daily registry are different vintages.
- **Rights / raw commit:** `research-only`, PC-3; **never**. Generic group licence does not establish report-specific open designation.
- **Manifest / order / requirements:** C+N+T; **P1 discovery; P2 ingest after SRC-12**, with `ResourceName`/unit/bus crosswalks. Fetch gate: resolve a functioning public report link, reject empty/error envelopes. FR-1.AC1/2, FR-2.AC2, FR-3.AC2.

### SRC-22 — NESO day-ahead boundary limits/flows

- **URL/evidence:** [Metadata](https://api.neso.energy/api/3/action/package_show?id=cf3cbc92-2d5d-4c2b-bd29-e11a21070b26), HTTP 200, advertises [CSV v1.5](https://api.neso.energy/dataset/cf3cbc92-2d5d-4c2b-bd29-e11a21070b26/resource/38a18ec1-9e40-465d-93fb-301e80fd1352/download/day-ahead-constraints-limits-and-flow-output-v1.5.csv). Metadata also advertises England/Wales and Scotland boundary-direction diagrams.
- **Format / cadence / size:** CSV; **daily on weekdays** per metadata, half-hour delivery intervals; recorded **29.27 MB**, 764,304 rows. Metadata says the rolling file retains three previous years plus current year: preserve 2024 before it ages out.
- **Rights / raw commit:** `open-commercial`, NESO Open Data Licence; **sample-only**.
- **Manifest / order / requirements:** C+T+R; **P3**, after SRC-35/01/02 for interpretation; capture raw early, derive cut-sets only with electrical evidence. FR-2.AC3, FR-3.AC2, FR-21. Preserve `GMT_BST` offset/fold and publication vintage; do not deduplicate repeated local times. Boundary diagrams/sign arrows still do not prove complete branch cut-sets.

### SRC-23 — NESO 24-month boundary limits

- **URL/evidence:** [Metadata](https://api.neso.energy/api/3/action/package_show?id=d515b4a9-60a1-489c-a126-004efc04f121), HTTP 200, advertises [November 2026 CSV](https://api.neso.energy/dataset/d515b4a9-60a1-489c-a126-004efc04f121/resource/3c359e33-3dac-4bdd-87d1-efbf4cbc2f07/download/24-months-ahead-constraint-limit_nov26.csv).
- **Format / cadence / size:** CSV; **monthly** publication of weekly horizon values; recorded **7.69 kB**, 109 rows.
- **Rights / raw commit:** `open-commercial`, NESO Open Data Licence; **sample-only**.
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

- **URL/evidence:** Read [terms](https://www.jao.eu/terms-conditions), [IFA/IFA2 trading](https://www.jao.eu/trading-ifa-ifa2) and [Nemo Link trading](https://www.jao.eu/trading-nemo-link), all HTTP 200. IFA page advertises [non-IEM access rules dated 24 September 2019](https://www.jao.eu/sites/default/files/2020-03/IFA%20%20IFA2%20Access%20Rules%20non-IEM%2024%20Sept%202019.pdf); Nemo page points to its operator for rules. **UNVERIFIED:** PDF contents, later modifications, 2024 applicability and complete border-specific rules. Terms alone are not a dated allocation regime.
- **Format / cadence / size:** HTML terms/trading pages and advertised PDF; PDF response/bytes **UNVERIFIED**; annual/modification cadence **UNVERIFIED**.
- **Rights / raw commit:** `research-only`, PC-5; **never**. Website terms do not confer a reusable rules database grant.
- **Manifest / order / requirements:** C+R; **P3 after SRC-15**, before SRC-43 flow-policy interpretation. FR-1.AC2/5, downstream FR-7.AC4. Pin border, product, direction and Brexit-effective regime separately.

### SRC-29 — NESO historical demand, 2024

- **URL/evidence:** [Metadata](https://api.neso.energy/api/3/action/package_show?id=historic-demand-data), HTTP 200, advertises exact [2024 CSV](https://api.neso.energy/dataset/8f2fe0af-871c-488d-8bad-960426f24601/resource/f6d02c0f-957b-48cb-82ee-09003f2ba759/download/demanddata_2024.csv).
- **Format / cadence / size:** CSV, half-hour observations; published **21 days in arrears**, later corrections explicitly described; recorded **1.81 MB**, 17,568 rows.
- **Rights / raw commit:** `open-commercial`, NESO Open Data Licence; **sample-only**.
- **Manifest / order / requirements:** C+T; **P3 after SRC-35**, then SRC-30/31/48/49 for reconciliation/allocation. FR-3.AC1/3/4, downstream FR-12. Starting gross-demand identity is `ND + embedded solar + embedded wind`; never subtract pumping from ND again. Version all further station-load/loss/storage boundary adjustments.

### SRC-30 — Elexon demand outturn

- **URL/evidence:** OpenAPI documents [demand/outturn](https://data.elexon.co.uk/bmrs/api/v1/demand/outturn). Use `?settlementDateFrom={d}&settlementDateTo={d}&format=json`, one settlement date per batch; date query names verified in schema.
- **Format / cadence / size:** JSON; half-hour initial demand observations, near-real-time/daily publication timing **UNVERIFIED** from the schema alone; recorded **3.56 MB/year**, 366 files.
- **Rights / raw commit:** `open-commercial`, BMRS licence; **sample-only**.
- **Manifest / order / requirements:** C+T; **P3 after SRC-35/29**; SRC-43 reuses these files, never counts them twice. FR-3.AC1/3/4. Keep INDO/ITSDO and initial/revised demand distinct.

### SRC-31 — Elexon FUELHH

- **URL/evidence:** OpenAPI documents [datasets/FUELHH/stream](https://data.elexon.co.uk/bmrs/api/v1/datasets/FUELHH/stream), queried with `?settlementDateFrom={d}&settlementDateTo={d}`. Schema verified; no payload queried.
- **Format / cadence / size:** JSON stream; half-hour observations, revision/publication latency **UNVERIFIED**; recorded **61.77 MB/year**, 366 files, 340,894 records.
- **Rights / raw commit:** `open-commercial`, BMRS licence; **sample-only**.
- **Manifest / order / requirements:** C+T; **P3 after SRC-35/29**, included once in SRC-43. FR-3.AC1/3/4, downstream FR-12 mix checks. No embedded solar series and no separate embedded wind series; use SRC-29 for GB embedded estimates.

### SRC-32 — Interconnector physical losses

- **URL/evidence:** [BritNed](https://www.britned.com/) and [ElecLink](https://www.eleclink.co.uk/), HTTP 200; ElecLink links its verified [loss-factor page](https://www.eleclink.co.uk/publications/loss-factor), HTTP 200, advertising a [loss guide PDF](https://cdn.prod.website-files.com/67b2dbc715938e08fe43fee1/67d9d09d90ce7e920a93ee31_ElecLink%20Loss%20Factor%20v2.pdf). Page states 2.9% end-to-end and 1.471% mid-interconnector; **2024 applicability, directional convention and converter/cable decomposition UNVERIFIED**. [Greenlink operations](https://greenlink.ie/operations/) returned 403. No complete 10-link parameter set verified.
- **Format / cadence / size:** HTML/PDF guidance/filings; updates on operator changes, schedule and bytes **UNVERIFIED**.
- **Rights / raw commit:** `research-only`, PC-5; **never**.
- **Manifest / order / requirements:** C+N+R+T; **P3 after SRC-15**, before lossy balance fixtures. FR-1.AC3/5, FR-3.AC2, NFR-2. Keep IFA, IFA2, ElecLink, BritNed, Nemo, NSL, Viking, Moyle, EWIC and Greenlink separate, with both directions and sending/receiving metering boundaries. Do not apply ElecLink's current figures to other links or invent missing coefficients.

### SRC-33 — GB/NI public holidays

- **URL/evidence:** [Government API catalogue](https://www.api.gov.uk/gds/bank-holidays/), HTTP 200, explicitly names [bank-holidays.json](https://www.gov.uk/bank-holidays.json). [Human-readable calendar](https://www.gov.uk/bank-holidays), HTTP 200, also read. JSON payload not fetched.
- **Format / cadence / size:** JSON; revised when dates announced/corrected, formal schedule **UNVERIFIED**; bytes and current 2024 retention **UNVERIFIED** (inventory previously observed 2019–28).
- **Rights / raw commit:** `open-commercial`, OGL v3.0; **sample-only**.
- **Manifest / order / requirements:** C+K; **P1 after SRC-35**. FR-3.AC1/4, FR-5. England/Wales, Scotland and Northern Ireland calendars differ; bank holidays are not exchange trading or settlement-publication calendars.

### SRC-34 — Irish public holidays

- **URL/evidence:** [Citizens Information public-holidays page](https://www.citizensinformation.ie/en/employment/employment-rights-and-conditions/leave-and-holidays/public-holidays/) returned **403**. Public content/access, exact 2024 dates, machine-readable endpoint and commercial grant remain **UNVERIFIED**. This response does not prove a login requirement.
- **Format / cadence / size:** HTML per inventory; updates with statutory/observance changes, schedule/bytes **UNVERIFIED**.
- **Rights / raw commit:** `research-only`, PC-5; **never**.
- **Manifest / order / requirements:** C+K; **P7 discovery after SRC-35**; fetch only when an accessible public publisher source is located. FR-3.AC1/4, FR-5. Do not silently substitute Northern Ireland's calendar for the Republic.

### SRC-35 — IANA tzdata

- **URL/evidence:** [Time-zones page](https://www.iana.org/time-zones) and [release 2026f](https://www.iana.org/time-zones/releases/2026f), HTTP 200, advertise [tzdata2026f.tar.gz](https://data.iana.org/time-zones/releases/tzdata2026f.tar.gz). Use the explicit release, not a mutable `latest` URL or unrecorded system library.
- **Format / cadence / size:** Text timezone rules in tar.gz; **ad hoc** political/clock-rule releases; advertised **472.0 kB** compressed, released 27 October 2026.
- **Rights / raw commit:** `open-commercial`, public-domain data per inventory; **sample-only**, retaining applicable notices. Release payload/its notices were not inspected here; any different rights discovered at acquisition require review.
- **Manifest / order / requirements:** C+K; **P1 first**; no source dependency after P0. FR-3.AC1, FR-4, FR-5, NFR-1. Pin both Europe/London and Europe/Dublin, leap-day and 46/50-period clock-change days; preserve repeated civil hours.

### SRC-36a — UK ETS auction clearing prices

- **URL/evidence:** Inventory [UK ETS auction-results route](https://www.gov.uk/government/publications/uk-ets-auction-results) returned **404**. [UK ETS participation guidance](https://www.gov.uk/government/publications/participating-in-the-uk-ets/participating-in-the-uk-ets), HTTP 200, does not identify the required historical auction-results file in the page read. Exact public results endpoint **UNVERIFIED**; do not substitute allowance-reporting templates for prices.
- **Format / cadence / size:** Auction tables/web downloads expected; precise format/bytes **UNVERIFIED**; periodic auctions (biweekly per inventory, exact calendar **UNVERIFIED**), not daily spot prices.
- **Rights / raw commit:** `research-only`, PC-5; **never**. ICE's third-party auction data is not cleared by a general GOV.UK OGL footer.
- **Manifest / order / requirements:** C+E+T; **P7 discovery**, then P4 after SRC-35/46 and verified dated price source. FR-3.AC2/4, downstream FR-7.AC2. Record UKA GBP/tCO2e, auction date and period; resampling cannot invent daily spot evidence.

### SRC-36b — EU ETS auction/spot prices

- **URL/evidence:** [EEX auction-download page](https://www.eex.com/en/market-data/market-data-hub/environmentals/eex-eua-primary-auction-spot-download), HTTP 200 after redirect from the older route, advertises [2012–2025 auction archive ZIP](https://www.eex.com/fileadmin/EEX/Downloads/Markets/Environmentals/EUA_Emission_Spot_Primary_Market_Auction_Report/Archive_Reports/emission-spot-primary-market-auction-report-2012-2025-data.zip) and [2026 XLSX](https://public.eex-group.com/eex/eua-auction-report/emission-spot-primary-market-auction-report-2026-data.xlsx). Separate historical secondary spot-series endpoint **UNVERIFIED**.
- **Format / cadence / size:** XLSX and archived ZIP; auction-day reports, precise refresh latency **UNVERIFIED**; bytes **UNVERIFIED**, archive content not inspected.
- **Rights / raw commit:** `research-only`, PC-5; **never**. [EEX disclaimer](https://www.eex.com/en/disclaimer), HTTP 200, permits a personal non-commercial copy and requires written approval for commercial use; no broader project redistribution grant established. SRC-36b cannot be admitted even as research-only until project research use is explicitly confirmed; D-56 does not expand a personal-use grant. Default to SRC-37a/37b and PR #58 candidate 6.2 DESNZ cost assumptions meanwhile; these are proxies, not a replacement verified daily carbon series.
- **Manifest / order / requirements:** C+E+T; **P4 after SRC-35/46**, fetch gate: document permitted research use under applicable product terms. FR-3.AC2/4, downstream FR-7.AC2. EUA EUR/tCO2e; auction results are not a verified daily spot vector.

### SRC-37a — DESNZ fossil-fuel assumptions

- **URL/evidence:** [Collection](https://www.gov.uk/government/collections/fossil-fuel-price-assumptions) links [2024 publication](https://www.gov.uk/government/publications/fossil-fuel-price-assumptions-2024), both HTTP 200; publication advertises [data XLSX](https://assets.publishing.service.gov.uk/media/66f679a230536cb92748287a/Fossil_Fuel_Price_Assumptions_2024.xlsx) and [method PDF](https://assets.publishing.service.gov.uk/media/66f3e1a8080bdf716392e855/2024-Fossil-Fuel-price-Assumptions-Publication.pdf).
- **Format / cadence / size:** XLSX/PDF; edition-based assumptions, fixed annual schedule **UNVERIFIED**; page advertises **45 kB + 504 kB**, 19-page PDF, last revised 27 September 2024.
- **Rights / raw commit:** `open-commercial`, OGL v3.0; **sample-only**.
- **Manifest / order / requirements:** C+E; **P4 after SRC-46** for common numeraire, alongside SRC-37b. FR-3.AC2, downstream FR-7.AC2. Projections through 2050 are scenario assumptions, not historical 2024 spot outturns; record real/nominal basis and units before conversions.

### SRC-37b — World Bank Pink Sheet

- **URL/evidence:** [Commodity Markets page](https://www.worldbank.org/en/research/commodity-markets), HTTP 200, advertises [historical monthly XLSX](https://thedocs.worldbank.org/en/doc/74e8be41ceb20fa0da750cda2f6b9e4e-0050012026/related/CMO-Historical-Data-Monthly.xlsx) and [annual XLSX](https://thedocs.worldbank.org/en/doc/74e8be41ceb20fa0da750cda2f6b9e4e-0050012026/related/CMO-Historical-Data-Annual.xlsx). Select monthly for historical cost candidates.
- **Format / cadence / size:** XLSX, **monthly** Pink Sheet updates with annual aggregates; bytes **UNVERIFIED** (page supplies no verified file size).
- **Rights / raw commit:** `open-commercial`, [World Bank dataset terms](https://data.worldbank.org/summary-terms-of-use), HTTP 200, CC BY 4.0 subject to additional/third-party terms; **sample-only**, only after resource/series exclusions are checked. Do not presume every quoted third-party price is redistributable.
- **Manifest / order / requirements:** C+E+T; **P4 after SRC-46/35**, map selected gas/coal/oil benchmarks and physical units to technologies. FR-3.AC2/4, downstream FR-7.AC2. USD prices need a pinned USD-to-objective conversion as applicable; monthly-to-half-hour expansion must be labelled resampled.

### SRC-38a — LCCC CfD strike prices

- **URL/evidence:** [Inventory data-portal URL](https://www.lowcarboncontracts.uk/data-portal) returned **403**. Actual public dataset endpoint, content and access conditions **UNVERIFIED**. HTTP 403 alone does not establish a required account; keep as public discovery candidate, not a credential-free fetch promise.
- **Format / cadence / size:** CSV/web portal per inventory; contract/amendment/indexation updates, precise cadence/bytes **UNVERIFIED**.
- **Rights / raw commit:** `research-only`, PC-5; **never**. Dataset terms inaccessible/unverified.
- **Manifest / order / requirements:** C+E+N+T; **P7 discovery**, then P4 after SRC-17/18/20/46 and dated contract matching. FR-3.AC2, downstream FR-7.AC2. Record base-year/indexed strike, delivery period, market reference price and signed CfD effects; do not equate every strike with a positive lost subsidy.

### SRC-38b — Ofgem RO parameters

- **URL/evidence:** [RO scheme page](https://www.ofgem.gov.uk/environmental-and-social-schemes/renewables-obligation-ro) and [supplier page](https://www.ofgem.gov.uk/environmental-and-social-schemes/renewables-obligation-ro/suppliers), HTTP 200. Supplier HTML contains a buy-out/obligation table through **2023/24**; PR #58 candidate 6.3 supplies the exact [2024/25 buy-out notice](https://www.ofgem.gov.uk/publications/renewables-obligation-ro-buy-out-price-mutualisation-threshold-and-mutualisation-ceilings-2024-2025), reachable at review time. Use that notice, not the previously guessed 404 slug; verify its period, values, rights and hash at acquisition. A machine-readable table remains UNVERIFIED.
- **Format / cadence / size:** HTML table/administrative notices, annual **1 April–31 March** obligation periods (verified); downloadable dataset bytes **UNVERIFIED**.
- **Rights / raw commit:** `research-only`, PC-5, preserving inventory classification until the exact resource grant is confirmed; **never**. A general Crown/OGL notice does not resolve unidentified resource rights.
- **Manifest / order / requirements:** C+E+R+N; **P4 discovery after SRC-18/46**, fetch gate: exact applicable annual notice. FR-3.AC2, downstream FR-7.AC2. Calendar 2024 spans 2023/24 and 2024/25; record GBP/ROC and ROCs/MWh separately. Buy-out is not a spot ROC price.

### SRC-41 — Met Éireann MÉRA public samples

- **URL/evidence:** [MÉRA page](https://www.met.ie/climate/available-data/mera), HTTP 200, explicitly says **1981–August 2019** and advertises one-month GRIB 1 samples: [mean sea-level pressure](https://www.met.ie/downloads/MERA_PRODYEAR_2015_06_1_103_0_0_FC3hr.grb), [2 m temperature](https://www.met.ie/downloads/MERA_PRODYEAR_2015_06_11_105_2_0_FC3hr.grb), [precipitation](https://www.met.ie/downloads/MERA_PRODYEAR_2015_06_61_105_0_4_FC3hr.grb). Bulk archive access without a request/account is **UNVERIFIED**; only these advertised public samples are proposed here.
- **Format / cadence / size:** GRIB 1; static historical reanalysis/sample, no 2024 refresh; publisher warns samples are **approximately 500 MB** (per-file versus combined size **UNVERIFIED**).
- **Rights / raw commit:** `research-only`, PC-5, specific open-data grant unverified; **never**.
- **Manifest / order / requirements:** C+W+T; **P7 optional**, after SRC-35 and weather-variable selection in Q-C-12; no dependency for 2024 acquisition. FR-3.AC1/4 and downstream FR-17 only for supported historical years. Not a replacement for excluded ERA5 2024 or a weather-to-power implementation.

### SRC-43 — Elexon 2024 market/operational streams

- **URL/evidence:** [Publisher OpenAPI](https://data.elexon.co.uk/swagger/v1/swagger.json), HTTP 200, documents all routes in the table below. Use exact API root `https://data.elexon.co.uk/bmrs/api/v1`; braces are caller-supplied parameters, not literal requests. No market data was requested.
- **Format / cadence / size:** JSON streams/daily batches; observations are settlement-period or event-driven (REMIT/notifications/acceptances), publication/revision timing varies and exact service latency is **UNVERIFIED**. Recorded **4.16 GiB (4,467,563,083 bytes)**, 3,673 files for the 12-stream capture, including SRC-20/30/31. It is not additional volume on top of those three rows.
- **Rights / raw commit:** `open-commercial`, BMRS licence; **sample-only**. Keep the required data-year and reference-year copyright/database attribution; exclude unlicensed third-party/personal content.
- **Manifest / order / requirements:** C+N+T+E, including stream/query/provider/BMU/run/revision dimensions and checksum indices; **P5 after SRC-35/15/20**, and SRC-26/27/28 before rule-dependent interpretation. FR-3.AC1/2/4, downstream FR-7.AC2 and FR-12. Acquire scarce rolling-publication histories early where possible, but freeze a comparable holdout only after coverage review.

| Stream (12 total) | Exact documented endpoint under the API root | Batch/filter and recorded size |
|---|---|---|
| BM units | `/reference/bmunits/all?format=json` | Reuse SRC-20; 2.17 MB |
| Demand | `/demand/outturn?settlementDateFrom={d}&settlementDateTo={d}&format=json` | Reuse SRC-30; 3.56 MB |
| Fuel | `/datasets/FUELHH/stream?settlementDateFrom={d}&settlementDateTo={d}` | Settlement-date or `publishDateTimeFrom/To` windows; preserve publication vintages. Reuse SRC-31; 61.77 MB |
| System prices | `/balancing/settlement/system-prices/{settlementDate}?format=json` | Settlement date/run; 15.64 MB |
| Interconnector flows | `/generation/outturn/interconnectors?settlementDateFrom={d}&settlementDateTo={d}&format=json` | Settlement-date or `publishDateTimeFrom/To` windows, link ID and publication vintages; 40.90 MB |
| Market Index Data | `/datasets/MID/stream?from={start}&to={end}&settlementPeriodFrom=1&settlementPeriodTo=50` | Provider/product; 5.63 MB |
| REMIT | `/datasets/REMIT/stream?publishDateTimeFrom={start}&publishDateTimeTo={end}` | **Publication-time** window, event/revision lineage; 90.17 MB |
| Acceptances | `/datasets/BOALF/stream?from={start}&to={end}&settlementPeriodFrom=1&settlementPeriodTo=50` | BMU/acceptance flags/revisions; 1,389.49 MB |
| MEL | `/datasets/MELS/stream?from={start}&to={end}&settlementPeriodFrom=1&settlementPeriodTo=50` | BMU/effective times; 795.83 MB |
| MIL | `/datasets/MILS/stream?from={start}&to={end}&settlementPeriodFrom=1&settlementPeriodTo=50` | BMU/effective times; 633.42 MB |
| Physical notifications | `/datasets/PN/stream?from={start}&to={end}&settlementPeriodFrom=1&settlementPeriodTo=50` | BMU/level/time profiles; 523.40 MB |
| Bid/offer pairs | `/datasets/BOD/stream?from={start}&to={end}&settlementPeriodFrom=1&settlementPeriodTo=50` | BMU/pair/direction/revision; 905.58 MB |

Use one source settlement day at a time initially; `{start}`/`{end}` are explicit
ISO 8601 boundaries according to the route's documented filter semantics. Confirm
inclusive/exclusive endpoints at acquisition, rather than assuming every route
uses the same convention. Preserve all 46/48/50 source periods. REMIT windows
must include the pre-year publications needed to establish outages active on
1 January 2024; an exclusively 2024 publication-window capture cannot prove that
coverage. Full 2024 interval unions do not prove every BMU/message is complete.
MID is a volume-weighted reference across exchanges, not a day-ahead auction
price. Bid/offer data serves historical comparison; it does not change D-59's
cost-based redispatch design.

### SRC-44 — SEMO/SEMOpx historical prices

- **URL/evidence:** Read [SEMO historical-data page](https://www.sem-o.com/market-data/static-reports/historic-market-data) and [SEMOpx document library](https://www.semopx.com/market-data/document-library), HTTP 200. Exact previously pinned targets are [SEMO imbalance ZIP](https://www.sem-o.com/documents/general-publications/Imbalance_Price_YTD_2020-2024.zip) and [SEMOpx October 2026 lookback ZIP](https://www.semopx.com/sites/semopx/files/2026-11/Lookback_Market%20October2026.zip). These exact archive URLs were **not exposed by the public pages read: current-link/access verification UNVERIFIED**. Use the existing manifest as historical identity evidence, not as a claim of fresh verification.
- **Format / cadence / size:** ZIP containing Excel/XML/CSV historical price products; monthly lookback/periodic archive revision schedule **UNVERIFIED**; recorded **4.46 MB + 39.17 MB = 43.63 MB**.
- **Rights / raw commit:** `research-only`, PC-3; **never**. File-specific open-commercial designation unverified.
- **Manifest / order / requirements:** C+T+E; **P5 after SRC-35/27/46** for product/interval/currency interpretation; fetch gate: resolve public archive availability without logging in. FR-3.AC1/4, downstream FR-12. Record DAM hourly, IDA1/2/3 auction-window coverage, IDC limitations and imbalance products separately. Earlier captures have **61 missing half-hours**, **333 missing five-minute intervals in Jan–Nov**, and **8,928 missing five-minute intervals in December**; never silently impute a validation pass.

### SRC-45 — NESO thermal constraint costs

- **URL/evidence:** [Metadata](https://api.neso.energy/api/3/action/package_show?id=f0055054-c55c-4068-a01c-61da4334e58f), HTTP 200, advertises [2024/25 CSV](https://api.neso.energy/dataset/f0055054-c55c-4068-a01c-61da4334e58f/resource/27df75fd-6233-466c-beed-3e6e2261e6b1/download/outturn-system-costs-2024-2025.csv) and adjacent [2023/24 CSV](https://api.neso.energy/dataset/f0055054-c55c-4068-a01c-61da4334e58f/resource/75c9c564-af38-4421-a461-a612a6921212/download/outturn-system-costs-2023-2024.csv).
- **Format / cadence / size:** CSV; **weekly** update per metadata, daily cost observations; recorded **48.3 kB**, 2,190 rows for 2024/25. Adjacent financial-year size/content **UNVERIFIED**.
- **Rights / raw commit:** `open-commercial`, NESO Open Data Licence; **sample-only**.
- **Manifest / order / requirements:** C+T+E+R; **P5 after SRC-22/35** for group/date mapping. FR-3.AC2, downstream FR-12.AC2/3. Need Jan–Mar from the 2023/24 resource plus Apr–Dec from 2024/25 for calendar 2024; keep constraint-only costs separate from total BM/settlement transfers.

### SRC-46 — ECB FX reference rates

- **URL/evidence:** [Reference-rates page](https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/html/index.en.html), HTTP 200, advertises [full-history CSV ZIP](https://www.ecb.europa.eu/stats/eurofxref/eurofxref-hist.zip) and [full-history XML](https://www.ecb.europa.eu/stats/eurofxref/eurofxref-hist.xml) (page appends cache query strings; stable paths retained here). Select one encoding.
- **Format / cadence / size:** CSV ZIP/XML; publisher says normally updated **around 16:00 CET each working day**, except TARGET closing days; bytes **UNVERIFIED**.
- **Rights / raw commit:** `open-commercial`, [ECB reuse disclaimer](https://www.ecb.europa.eu/services/disclaimer/html/index.en.html), HTTP 200; **sample-only**, acknowledge source, disclose modifications and follow sale/third-party conditions.
- **Manifest / order / requirements:** C+E+T; **P1**, before SRC-36/37/38/44 cost/price normalization. FR-1.AC3/4, FR-3.AC1/4, NFR-2. Record GBP per EUR (and USD per EUR if required), publication time and conversion direction. Carry the last valid pre-2024 working-day rate into the year's start/non-working days under a versioned fill policy; disclose effective-date/as-of differences.

### SRC-47 — Bank of England XUDLSER check

- **URL/evidence:** [Database hub](https://www.bankofengland.co.uk/boeapps/database/) returned **403**. Exact XUDLSER download query, series definition, access and series-specific reuse rights **UNVERIFIED**; do not invent a CSV export URL.
- **Format / cadence / size:** CSV per inventory; working-day FX publication expected, precise schedule/bytes **UNVERIFIED**.
- **Rights / raw commit:** `research-only`, PC-5; **never**. General OGL terms have third-party exceptions; selected-series clearance is unverified.
- **Manifest / order / requirements:** C+E+T; **P7 optional discovery after SRC-46**, not a blocker for the primary ECB route. FR-1.AC4, FR-3.AC1/4. Confirm quote direction before using as an independent FX check.

### SRC-48 — NESO ETYS Appendix G nodal demand

- **URL/evidence:** Verified [ETYS appendices page](https://www.neso.energy/publications/electricity-ten-year-statement-etys/etys-documents-and-appendices) advertises [2025 Appendix G](https://www.neso.energy/document/379136/download); corrects the inventory's missing URL.
- **Format / cadence / size:** XLSX in previous manifest; annual ETYS editions with revisions, current payload format **UNVERIFIED**; recorded **74.5 kB**. Publication page labels this edition 18 March 2026.
- **Rights / raw commit:** `research-only`, PC-1; **never**.
- **Manifest / order / requirements:** C+N+T; **P3 after SRC-01/29**, then SRC-49 for GSP reconciliation. FR-2.AC2, FR-3.AC3/4. Record season/scenario/year and whether figures are FES nodal projections or measured peaks; uninspected Appendix G cannot be presumed to supply historical 2024 demand shapes.

### SRC-49 — Public DNO demand material only (partial family)

- **URL/evidence:** Reuse verified SRC-05 [May 2026 tables](https://data-api.ssen.co.uk/dataset/695078a0-bbd8-420b-8383-ee6bf7fc0045/resource/02c9431f-6126-4384-9f46-9079dbcd7057/download/southern-electric-power-distribution-long-term-development-statement-tables_may-2026.xlsx) as an inspection candidate. NGED's verified [metadata](https://connecteddata.nationalgrid.co.uk/api/3/action/package_show?id=ltds-tabular-model) advertises [Table 3 Demand](https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/0ac103b3-5ee0-44e0-be31-51cea65aac7c/download/table-3-2025.csv). **Correction:** NGED Table 7 is labelled **Planned Work**, not GSP demand. A universal “Table 7 GSP demands” endpoint and complete 14-area coverage are **UNVERIFIED**.
- **Format / cadence / size:** CSV/XLSX; NGED 12-month metadata, SSEN dated editions; advertised NGED demand **331 kB**, SSEN **731 kB reused from SRC-05**. Per-area GSP subsets/denominators and any different demand payload sizes **UNVERIFIED**.
- **Rights / raw commit:** `research-only`, PC-5, retaining the unresolved aggregate candidate classification; **never under SRC-49 admission**. The exact reused NGED/SSEN releases retain their independently evidenced open-commercial grants under SRC-03/05; do not downgrade their original manifests or infer the same grant for other DNOs. Any new cleared demand sub-resource needs its own rights record before reclassification.
- **Manifest / order / requirements:** C+N+T; **P3 after SRC-01/03/05/29/48**, inspect table meaning and attach GSP peaks only with explicit source definitions. FR-2.AC1/2, FR-3.AC3/4. The excluded gated DNO portions are in the appendix; Scottish-area coverage dispositions remain required. Peak snapshots need an explicit temporal-allocation policy and national-energy reconciliation, not unlabelled half-hour observations.

### SRC-50 — Elexon settlement stack TLM candidate

- **URL/evidence:** Verified OpenAPI documents `GET /balancing/settlement/stack/all/{bidOffer}/{settlementDate}/{settlementPeriod}` under `https://data.elexon.co.uk/bmrs/api/v1`; `bidOffer` is `bid` or `offer`, optional `format=json`. Response schema `SettlementStackResponse` has nullable `transmissionLossMultiplier`. No stack payload requested.
- **Format / cadence / size:** JSON; settlement-period stacks with revisions, publication/run coverage **UNVERIFIED**; per-batch/full-year bytes **UNVERIFIED**. Exhaustive bid+offer scans would involve up to **35,136 period-direction queries** for a 17,568-period year before retries; start with schema/coverage checks, not a blind annual scan.
- **Rights / raw commit:** `open-commercial`, BMRS licence; **sample-only**.
- **Manifest / order / requirements:** C+N+T+R; **P6 after SRC-20/26/35**, compare physical-loss interpretation with SRC-32. FR-1.AC3, FR-3.AC2, NFR-2. Candidate accepted-action stacks do not establish a complete delivering/offtaking BMU TLM panel or physical cable losses. Sampling/full-panel suitability is **UNVERIFIED**; the 12 SRC-43 streams do not supply that missing panel.

## Evidence limits and M1 handoff

The public-page checks above establish advertised locations, not live payload
availability. Previously measured byte counts are traceable to the existing
[network manifest](../../spikes/data-gathering/network-ie/manifest.toml),
[Elexon manifest](../../spikes/data-gathering/elexon/manifest.toml),
[SEMO archive manifest](../../spikes/data-gathering/semo-2024/manifest.toml) and
[sample manifest](../../spikes/data/samples/download-manifest.json). Preserve those
captures and their original hashes; a future fetch is a new immutable release.
Publisher-advertised resource sizes and cadence overrides in this plan are
metadata evidence, not fresh file measurements.

Specific unresolved fetch/admission items: SRC-13 public row exports; SRC-14
electrical model; SRC-21 exact working reports; SRC-24/44 exact historical links;
SRC-26 BSC access; SRC-28 historical border documents; SRC-34 holidays;
SRC-36a UKA results; SRC-36b project-use terms/secondary spot data;
SRC-38a CfD access; SRC-38b 2024/25 notice; SRC-47 FX export. SRC-25/27 now have
working current publication pages, but dated 2024 rules/parameters remain
UNVERIFIED. SRC-32 still lacks verified dated loss functions for all ten links.
SRC-49 is a partial inspection candidate, not a resolved 14-area demand source.

Public-only acquisition cannot satisfy FR-2's full-network gate: UKPN/SPEN/NPg/ENWL
132-kV coverage remains gated, electrical cut-sets and commissioned 2024 asset
crosswalks remain unresolved, and no in-scope reanalysis supplies 2024 weather.
ERA5/MIDAS/Ninja are excluded below; MÉRA stops in 2019. Existing Q-C-12/Q-C-13
track weather selection and network snapshot decisions in the
[questionnaire](../requirements/02-questionnaire.md). Do not replace those gaps
with inferred scope decisions, current snapshots or fabricated weather profiles.
Calendar/rule locks and transformation/schema implementation remain M1 work.
Heat rates/VOM, Irish embedded generation and loss/rule candidates now have
explicit PR #58 dispositions below; candidate identification does not close
parameter verification or dated coverage gaps.

## PR #58 candidate reconciliation

These dispositions supplement the original 43-entry register; its counts remain
inventory-row counts, not counts of all candidates below. “Added” means added to
this acquisition plan, never acquired/cleared. Tags below govern each new resource;
research-only raw commit is **never**, open-commercial raw commit is **sample-only**.
All require profile C, repeat terms/access checks and record bytes/hashes at fetch.
Candidate numbers link to [the merged deficit analysis](m1-deficit-sources.md),
which preserves publisher URLs, rights evidence and verification limitations.

| #58 candidate | Disposition, acquisition contract and remaining limit |
|---|---|
| 1.1 ETYS 2024 | Added to SRC-01 as primary historical edition, research-only PC-1 / never; C+N, P2. 2025 remains forward. |
| 1.2 Elexon BMU/MDD crosswalk | Added alongside SRC-20, open-commercial / sample-only; C+N, P1/P2. BMU reference is not a physical GSP crosswalk; inspect dated MDD/P114 joins. |
| 1.3 November 2024 LTDS | Added to SRC-03/05 holdout edition selection, open-commercial / sample-only; C+N, P2. Exact archived resources/hash unresolved; other DNO gated areas remain excluded and tracked under D-60/D-71. |
| 2.1 EirGrid/SONI 2024 annual report | Added: [publisher PDF](https://cms.eirgrid.ie/sites/default/files/publications/Annual-Renewable-Constraint-and-Curtailment-Report-2024-V1.0.pdf), research-only PC-2/3 / never; C+R, P3, FR-21/FR-3.AC2. Candidate evidence: SNSP 75% and inertia 23,000 MW·s throughout 2024; 3 NI + 4 ROI units enduring from 7 April 2024. Pre-April trial schedule and local groups remain gaps. |
| 2.2 NESO limits | Added through SRC-22/23, open-commercial / sample-only; C+R+T, P3. Weekly forecasts/outturn are not signed electrical cut-sets. |
| 2.3 Operability/Grid Code | Added through SRC-25/26, research-only / never; C+R, P3. Dated clauses and applicability must be verified. |
| 3.1 Sheffield Solar PV_Live | Added: [PV_Live](https://www.solar.sheffield.ac.uk/pvlive/), open-commercial CC BY 4.0 / sample-only per publisher grant recorded in candidate 3.1; C+T, P3, FR-3.AC3/4. Capture regional/time definitions; no invented Irish observations. |
| 3.2 NESO historical demand | Added through SRC-29, open-commercial / sample-only; C+T, P3. Preserve ND/embedded/pumping definitions. |
| 3.3 SEAI/ESB microgeneration | Added via [SEAI renewables statistics](https://www.seai.ie/data-and-insights/seai-statistics/key-statistics/renewables/) and ESB 2023/2024 reports linked in candidate 3.3; SEAI extracts open-commercial OGL Ireland/CC BY 4.0 / sample-only, ESB reports research-only / never; verify each resource grant; C+N+T, P3, FR-3.AC3. Q-P-02 interpolation/scaling is an open assumption, not metered output. |
| 4.1 Ofgem/operator loss methodology | Added alongside SRC-32, research-only / never pending resource-specific rights; C+R+N, P3, FR-1.AC3. Use dated [2024 Ofgem BritNed approval](https://www.ofgem.gov.uk/decision/approval-modified-access-rules-britned-interconnector-pursuant-standard-licence-condition-11a-electricity-interconnector-licence-2024) and operator filings in candidate 4.1. Deemed factors are not physical loss curves. |
| 4.2 Elexon stack/P278 | Added through SRC-50 (open-commercial / sample-only); C+T+R, P6. P278 settlement TLM=1 for interconnector BMUs is not zero physical loss. |
| 4.3 SEM TLAF/methodology | Added: [SEM Committee publications](https://www.semcommittee.com/publications/), research-only PC-2/3 / never; C+R+T, P3, FR-1.AC3. Pin 2023/24 and 2024/25 tables; exact filenames remain UNVERIFIED. |
| 5.1–5.3 Market rules | Added through SRC-26/27/28, research-only / never; C+R, P3. Dated historical rules required; Q-P-04 merit order cannot replace baseline UC. |
| 6.1 UKA auctions | Added discovery path to SRC-36a; research-only / never until exact third-party rights confirmed; C+E+T, P4. Do not infer OGL from a GOV.UK page. |
| 6.2 DESNZ generation costs 2023 | Added: [publisher page](https://www.gov.uk/government/publications/electricity-generation-costs-2023), open-commercial OGL / sample-only; C+E+N, P4, FR-3.AC2 and downstream FR-7.AC2. Heat rates/VOM are future-build assumptions; Q-P-05 defaults remain unapproved operating-fleet proxies. |
| 6.3 RO/CfD | Added exact 2024/25 Ofgem notice to SRC-38b and [LCCC register](https://www.lowcarboncontracts.uk/cfd-register/) discovery to SRC-38a; research-only / never until resource-specific rights confirmed; C+E+R, P4/P7. |
| 7.1 ERA5 multi-level wind | Deferred from credential-free run: CDS account/token required (SRC-39). C+W+T; research-only / never pending exact product rights. Q-P-06 requires 100 m plus lower-level wind where available; retain heights and any fallback shear policy. |
| 7.2 MIDAS | Deferred: registration/terms gate (SRC-40); research-only / never pending exact dataset grant; C+W+T. |
| 7.3 Met Éireann hourly stations | Added: [historical station records](https://www.met.ie/climate/available-data/historical-data), research-only / never pending exact station-resource grant (some CC BY 4.0, some custom); C+W+T, P7, FR-3.AC1/4. Public observations complement weather candidates; they do not establish nationwide reanalysis coverage. |
| 8.1 Irish public holidays | Added [WRC guide](https://www.workplacerelations.ie/en/what_you_should_know/public-holidays/) as SRC-34 alternative to the 403 Citizens Information route; research-only / never under WRC disclaimer; C+K+R, P1/P7. Verify dated 2023/24 statutory/observance rules before calendar admission. |
| 8.2 UK holidays | Added through SRC-33, open-commercial / sample-only; C+K, P1. |
| 8.3 tzdata | Added through SRC-35, open-commercial / sample-only; C+K, P1. Canonical release for this plan and PR #59 F-TIME is **2026f**, superseding candidate 2024b. SHA-256 `01b4a91cec5167ed48c11e8ab78f2048cead78ce94b0cafcad0ae8c5a3044d3d` verified from the IANA archive on 2026-11-19 in temporary storage; no archive committed. |
| 9.1 CGMES / 9.2 Frictionless / 9.3 internal schema | Deferred as dataset acquisition: design references/internal implementation, not external payload sources. Consult with INT-01/ADR 0002; no raw-data admission asserted. |

Q-P-03 also changes the loss acquisition contract: seek directional dated standby
MW, linear and quadratic coefficients with physical units and metering endpoint,
not merely a percentage. Preserve missing terms as gaps, not invented zeros.
Q-P-01 rating proxy and the 132 kV/100 MVA `Z_base=174.24 ohms` conversion
need original ratings/types/bases plus inferred-method provenance. Q-P-05 and
Q-P-08 require original cost cadence, unit/technology basis and publication
vintages before any resampling. These existing questionnaire assumptions remain
unapproved; this reconciliation introduces no owner decision or new proxy default.

## Appendix — excluded credential-gated sources

Access requirements here retain the inventory's evidence; no accounts were
created and no tokens, approvals or payload access were requested. Public
catalogue descriptions do not remove the following prerequisites. Future
credentials/clearance do not belong in this acquisition run or public manifests.

| SRC IDs | Excluded source | What is needed |
|---|---|---|
| SRC-06, SRC-07 | UKPN LTDS tables/CIM | UKPN portal registration/login; CIM additionally needs the formal access request/approval and shared-data terms. Permissions and receipt remain unverified. |
| SRC-08, SRC-09 | SPEN LTDS tables/CIM | SPEN portal registration/login; CIM additionally requires a Data Request/approval under shared-data terms. The precise request form remains unresolved. |
| SRC-10 | Northern Powergrid LTDS tables/CIM | NPg portal account/login; export/CIM entitlement must be confirmed. |
| SRC-11 | ENWL LTDS secure area | Secure-area registration/login and approval; restrictive shared-data terms also need PC-5 clearance before commercial use. |
| SRC-16a, SRC-16b | ENTSO-E physical flows/scheduled exchanges | Transparency Platform account and personal REST API security token requested from ENTSO-E. Keep non-excluded A11 flows distinct from IFA/Nemo A11 and all A09 schedules; a token does not clear their different reuse rights. |
| SRC-39 | Copernicus CDS ERA5 | Registered CDS account, agreement to dataset terms and personal API key/CDS configuration. No weather bytes acquired in this task. |
| SRC-40 | CEDA MIDAS Open | Free CEDA registration/login and applicable access/licence acceptance. Open-data copyright status does not remove the account prerequisite. |
| SRC-42 | Renewables.ninja | Free user account and API token; CC BY-NC 4.0 remains research-only, with separate commercial permission needed. |
| SRC-49 (gated portions only) | DNO GSP-demand tables | UKPN/SPEN/NPg/ENWL portal accounts and any required data-request approvals as above. Public NGED/SSEN inspection candidates remain in the register; no promise of complete all-area demand coverage. |

## Recommended fetch sequence

This is a proposed operational order, not authorisation to download in this
docs-only task. Independent raw acquisitions need not wait for unrelated joins;
dependencies below govern admission and derivation. Every fetch first repeats
the public metadata/terms check and stops if credentials are required. A 403/404,
empty error envelope or rights mismatch creates an unresolved manifest entry,
not a substituted dataset. Keep retry/backoff and batching consistent with each
publisher's service terms; do not circumvent access controls.

1. **P0 — Prepare admission and immutable storage.** Before any dataset request,
   establish C/profile manifest records, private raw paths outside git, rights
   evidence and PC references; reject share-alike ancestors. Keep the 2024 holdout
   separate from current/forward releases. Do not run databases, heavy ingest or
   benchmarks in the coding container; M1 heavy work needs the D-65 target and
   NFR-12.AC1/2 isolation checks. No account setup is part of this sequence.
2. **P1 — Pin small clocks, currencies and identity inputs.** Fetch **SRC-35**
   first, then **SRC-33, SRC-46, SRC-15, SRC-20, SRC-17, SRC-18, SRC-19**.
   Discover **SRC-21** working public filenames in parallel; defer failed reports.
   Prefer pinned historical registers if available; label current captures as
   current and retain release/publication evidence. Clock/FX definitions precede
   all time/currency transformations.
3. **P2 — Acquire topology and fleet crosswalk inputs.** Fetch **SRC-01, SRC-12,
   SRC-02, SRC-03, SRC-04, SRC-05**. For NGED, ingest node aliases before circuits
   and transformers, then reconcile same-edition CIM; map SEPD GSPs and Irish
   ResourceNames to buses. Add **SRC-21** only if its public fetch gate passes.
   Inspect **SRC-13** public export metadata; fetch records only when no-account
   access is established. Record 14 DNO-area coverage dispositions and every
   absent denominator. A reduced model is not full-network completion.
4. **P3 — Pin demand, boundaries and dated rules.** Fetch **SRC-29, SRC-30,
   SRC-31, SRC-48**, and public **SRC-49** candidates without duplicating SRC-05;
   join GSP peaks only after table definitions are verified. Capture **SRC-22**
   before rolling history expires; add **SRC-23** as a separately labelled
   forward horizon and **SRC-24** when its historical URL is resolved. Acquire
   accessible **SRC-25/26/27/28/32** documents, pin historical effective versions,
   then derive supported security/loss proxies. Missing cut-sets/rules/losses
   remain gaps. Keep 2026 strategy/market rules out of a 2024 as-of lock.
5. **P4 — Build traceable cost candidates.** With FX/units pinned, fetch
   **SRC-37a/37b**, then **SRC-36b** only within verified permitted research use;
   resolve **SRC-36a, SRC-38a, SRC-38b** public endpoints before any price/contract
   batches. Convert units and currencies, preserve auction/monthly/annual
   resolution, and version resampling. This does not produce a complete SRMC
   vector without sourced heat rates, emissions, VOM and subsidy mappings.
6. **P5 — Acquire historical comparison streams in increasing volume.** Reuse
   SRC-20/30/31 captures, then SRC-43 **system prices, MID, interconnector flows,
   REMIT**, followed by **PN, MILS, MELS, BOD, BOALF** (daily/publication-window
   batches; include needed pre-year REMIT history). Acquire **SRC-44** after
   resolving its public archive links, and **SRC-45** across both financial-year
   files. Reconcile product/timezone/provider/unit masks, up/down volumes and
   constraint-only costs; archive gaps are not passes. Fetching observations can
   proceed before rule interpretation is finished, but validation freeze cannot.
7. **P6 — Resolve the TLM candidate before scaling.** After BMU/calendar/BSC
   mapping, inspect a small dated **SRC-50** stack sample in a later acquisition
   task and check nullability, settlement runs, directional units and whether
   inactive BMUs are absent. Scale only if it actually serves the required loss
   quantity; otherwise retain the missing-panel issue. TLM is not a substitute
   for the physical interconnector loss functions in SRC-32.
8. **P7 — Resolve public gaps and freeze an honest release.** Follow up
   **SRC-14/34/36a/38a/38b/47** and unresolved earlier fetch gates; **SRC-41** is
   optional historical-format research, never a 2024 dependency. Reconcile bytes,
   hashes, rights ancestry, dates, GSP/BMU aliases and interval/energy masks,
   explicitly retain every unresolved item above, and freeze input/holdout
   manifests before calibration (FR-12.AC1). Do not claim forecast validation,
   commercial clearance or full-network M1 acceptance from this public subset.
