# M1 Data Ingest and Network Model: Data Inventory & Ingest Preparation

Verified and compiled on **2026-11-14**. Prepares data gathering and ingestion for milestone **M1** (data ingest and network model) of the GB + all-island Ireland grid simulator per [06-roadmap.md](../requirements/06-roadmap.md) and [04-requirements.md](../requirements/04-requirements.md) (FR-1–5, FR-19, FR-21, NFR-1/2). Follows [AGENTS.md](../../AGENTS.md) Rule 10 and owner decision D-56: every candidate source is classified as `open-commercial` or `research-only`. Share-alike (ODbL / OSM) data is strictly excluded. Raw restricted research data is never committed to the repository; download scripts and metadata manifests only.

---

## 1. Executive Summary & Inventory Coverage

### Scope and Governing Decisions
- **Geography & Topology (FR-1, FR-2, D-11, D-32, D-60):** Great Britain (400 kV, 275 kV, and full 132 kV across all 14 DNO licence areas) plus all-island Ireland (400 kV, 220 kV, 110 kV in Ireland and 275 kV, 110 kV in Northern Ireland), with 10 external interconnector boundaries.
- **Physics Formulation (FR-8, FR-21, D-55):** DC network model augmented by published operational security and stability constraint proxies (boundary transfer limits, regional inertia floors, minimum units, Irish SNSP limits). Full AC power flow and dynamic simulation stay in P2/P4.
- **Market & Operational Rules (FR-7, D-15, D-59):** GB national market scheduling and I-SEM ex-ante trading, followed by cost-based redispatch against physical constraints. Historical bid/offer replay is not used; redispatch is driven by estimated generator operating costs.
- **Validation Holdout (FR-12, D-66, D-70):** Pinned historical baseline year 2024 (leap year: 366 days, 17,568 half-hour settlement periods). Reconstructions are distinguished from as-of forecast backtests.
- **Provenance & Pre-commercial Clearance (FR-19, D-56):** Data with open commercial licences is cleared for application use; publicly downloadable data lacking an explicit open commercial licence is permitted in the research phase under the tag `research-only` (with raw files stored outside git in `~/energy-data/raw/` and tracked in [TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md)).

### Source Classification & Deficit Summary
- **Total M1 Input Categories Assessed:** 6 primary categories spanning 18 distinct functional input groups.
- **Total Candidate Sources Inventoried:** 48 unique candidate datasets across 16 publishers.
- **Classification Breakdown:**
  - `open-commercial`: 25 datasets (52.1%)
  - `research-only`: 23 datasets (47.9%)
- **Inputs with No Known Source:** **0**. Every required M1 input has at least one identified candidate source (though several require owner credentials, permission requests, or derivation from related operational data).

---

## 2. Detailed Data Inventory by M1 Input Category

### Category 1: Dated Topology and Network Assets

#### 1.1 GB Transmission Network (400 kV, 275 kV, Scottish 132 kV)
- **Candidate Source:** NESO Electricity Ten Year Statement (ETYS) 2025 Appendix B (System Technical Data)
  - **Publisher:** National Energy System Operator (NESO)
  - **URL:** [https://www.neso.energy/document/383936/download](https://www.neso.energy/document/383936/download)
  - **Format:** Excel Workbook (`.xlsx`), 31 worksheets
  - **Coverage:** GB transmission network: Scottish Hydro Electric Transmission (SHE Transmission), SP Transmission (SPT), National Grid Electricity Transmission (NGET), and Offshore Transmission Owners (OFTOs). Voltages: 400 kV, 275 kV, 132 kV (Scotland and selected English/Welsh assets). Base year: 2025/26 with future change sheets 2026/27–2033/34.
  - **Update Cadence:** Annual (typically published Q4)
  - **Licence Name & Link:** No dataset-specific open licence found; governed by [NESO Terms and Conditions](https://www.neso.energy/terms-and-conditions)
  - **D-56 Classification:** `research-only`
  - **Access Requirement:** Open direct download (no credentials required)
  - **Already in Repo / Spikes:** Download script and SHA-256 fingerprint in [spikes/data-gathering/network-ie/manifest.toml](../../spikes/data-gathering/network-ie/manifest.toml); audit in [spikes/data/REPORT.md](../../spikes/data/REPORT.md) (1,952 literal endpoint IDs, 1,392 circuit rows, 1,472 transformer rows). Raw file stored externally in `raw/neso/`.
  - **Remaining Gaps:** Unverified commercial grant ([PC-1 in TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md)); ratings are labelled post-fault; no continuous/emergency distinction; does not include England/Wales DNO 132 kV; lacks geographic coordinates and tap models.

- **Candidate Source:** NESO ETYS Boundary GIS Data
  - **Publisher:** National Energy System Operator (NESO)
  - **URL:** [https://api.neso.energy/dataset/997f4820-1ad4-499b-b1fe-4b8d3d7fbc72](https://api.neso.energy/dataset/997f4820-1ad4-499b-b1fe-4b8d3d7fbc72)
  - **Format:** Shapefile bundle (`.shp`, `.shx`, `.dbf`, `.prj`) inside ZIP
  - **Coverage:** 34 transmission system boundary features across GB
  - **Update Cadence:** Annual
  - **Licence Name & Link:** [National Energy SO Open Data Licence v1.0](https://www.neso.energy/data-portal/neso-open-licence)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open CKAN REST API / direct download
  - **Already in Repo / Spikes:** Hash and metadata in [spikes/data-gathering/network-ie/manifest.toml](../../spikes/data-gathering/network-ie/manifest.toml); file cached externally.
  - **Remaining Gaps:** Provides boundary line geometries only; does not provide electrical branch cut-sets, individual circuit ratings, or substation coordinates.

#### 1.2 GB Distribution 132 kV Network (All 14 DNO Licence Areas, D-60)
Per D-60, all 14 DNO areas must be incorporated. Full inventory verified in [dno-132kv-inventory.md](dno-132kv-inventory.md):

- **Candidate Source:** National Grid Electricity Distribution (NGED) LTDS Tabular Model (East Midlands, West Midlands, South Wales, South West)
  - **Publisher:** NGED
  - **URL:** [https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model](https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model)
  - **Format:** CSV (Table 1 Circuits, Table 2a 2W Transformers, Table 2b 3W Transformers, Attachment 8 Nodes)
  - **Coverage:** 4 licence areas: East Midlands (EMIDS), West Midlands (WMIDS), South Wales (SWALES), South West (SWEST). 132 kV, 66 kV, 33 kV. Table 1 has 4,183 132-kV rows (1,138 circuits: 907 unchanged baseline, 53 updated, 115 new, 63 removed; plus disconnectors and circuit breakers).
  - **Update Cadence:** Annual / semi-annual (edition dates December 2025 / July 2026)
  - **Licence Name & Link:** [NGED Open Data Licence](https://www.nationalgrid.co.uk/open-data-licence)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download (no authentication required)
  - **Already in Repo / Spikes:** Downloaded and verified in [spikes/data-gathering/network-ie/manifest.toml](../../spikes/data-gathering/network-ie/manifest.toml); samples in [spikes/data/samples/](../../spikes/data/samples/).
  - **Remaining Gaps:** 3,045 rows lack impedance/ratings (switchgear rows); unit bases for R/X/B (% on 100 MVA vs ohms) and ratings (MVA vs A) require explicit LTDS specification resolution; node IDs must be mapped to ETYS GSPs.

- **Candidate Source:** NGED LTDS Common Information Model (CIM)
  - **Publisher:** NGED
  - **URL:** [https://connecteddata.nationalgrid.co.uk/dataset/ltds-common-information-model](https://connecteddata.nationalgrid.co.uk/dataset/ltds-common-information-model)
  - **Format:** CIM XML / RDF archives in ZIP (EQ, SC, SCR, SYSCAP profiles)
  - **Coverage:** 4 NGED areas; detailed electrical network topologies
  - **Update Cadence:** Annual
  - **Licence Name & Link:** [NGED Open Data Licence](https://www.nationalgrid.co.uk/open-data-licence)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download
  - **Already in Repo / Spikes:** 4 regional ZIP archives downloaded and hashed in [spikes/data-gathering/network-ie/manifest.toml](../../spikes/data-gathering/network-ie/manifest.toml).
  - **Remaining Gaps:** Complex nested CIM profiles (EQ/SC/SCR/SYSCAP) require parsing into the canonical network model; operational switching states must be reconciled.

- **Candidate Source:** SSEN Distribution Long Term Development Statements (SEPD & SHEPD)
  - **Publisher:** Scottish and Southern Electricity Networks (SSEN)
  - **URL:** [https://data.ssen.co.uk/@ssen-distribution/sepd_long_term_development_statement](https://data.ssen.co.uk/@ssen-distribution/sepd_long_term_development_statement)
  - **Format:** Excel Workbook (`.xlsx`) Tables 1–7 and CIM XML ZIP
  - **Coverage:** Southern Electric Power Distribution (SEPD, 132 kV circuits and transformers) and Scottish Hydro Electric Power Distribution (SHEPD, distribution 33/11 kV; Scottish 132 kV is transmission under SHET). Table 1 has 435 132-kV circuit rows and 218 132-kV transformers for SEPD.
  - **Update Cadence:** Annual (May 2026 edition verified)
  - **Licence Name & Link:** [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download (transient signed S3 URLs generated without login)
  - **Already in Repo / Spikes:** SEPD workbook and CIM EQ profile downloaded in [spikes/data-gathering/network-ie/manifest.toml](../../spikes/data-gathering/network-ie/manifest.toml).
  - **Remaining Gaps:** Substation geographic coordinates are unverified in tables; joining SEPD GSPs with NGET transmission buses.

- **Candidate Source:** UK Power Networks (UKPN) LTDS Table 1–4 and CIM (EPN, LPN, SPN)
  - **Publisher:** UK Power Networks
  - **URL:** Table 1 Circuit: [https://ukpowernetworks.opendatasoft.com/explore/assets/ukpn-ltds-table-1-circuit-data/](https://ukpowernetworks.opendatasoft.com/explore/assets/ukpn-ltds-table-1-circuit-data/); CIM: [https://ukpowernetworks.opendatasoft.com/explore/assets/ukpn-ltds-cim/](https://ukpowernetworks.opendatasoft.com/explore/assets/ukpn-ltds-cim/)
  - **Format:** Tabular (CSV/Parquet via Opendatasoft API) and CIM XML
  - **Coverage:** Eastern Power Networks (EPN), London Power Networks (LPN), South Eastern Power Networks (SPN). 132 kV circuits, transformers, and grid sites (catalogue lists 5,076 records).
  - **Update Cadence:** Annual (May 2026 release)
  - **Licence Name & Link:** Tabular data: [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/); CIM data: UK Power Networks Shared Data Licence - Common Information Model
  - **D-56 Classification:** Tabular: `open-commercial`; CIM: `research-only` (unverified terms)
  - **Access Requirement:** **Owner credentials / portal login required**. Public API returned HTTP 403 ForbiddenAccess without authenticated session. CIM requires explicit access request form approval.
  - **Already in Repo / Spikes:** Schemas and URLs documented in [dno-132kv-inventory.md](dno-132kv-inventory.md); no record payload downloaded.
  - **Remaining Gaps:** Row-level record access blocked behind login; requires owner account creation; impedance base confirmation (unverified 100 MVA); CIM licence agreement unverified behind Microsoft sign-in.

- **Candidate Source:** SP Energy Networks (SPEN) LTDS Tables 1–7 and CIM (SP Manweb & SP Distribution)
  - **Publisher:** SP Energy Networks
  - **URL:** SPM Table 1: [https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-appendix-1-circuit-data-table-1/](https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-appendix-1-circuit-data-table-1/); SPM CIM: [https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-cim-eq-profile/](https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-cim-eq-profile/)
  - **Format:** Tabular (CSV/JSON/Parquet) and CIM XML ZIP
  - **Coverage:** SP Manweb (SPM, England/Wales 132 kV network, 1,798 catalogue records) and SP Distribution (SPD, Scottish distribution; 132 kV is transmission under SPT).
  - **Update Cadence:** Annual (November 2025 release)
  - **Licence Name & Link:** Tabular data: [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/); CIM data: ENA Shared Data Licence hosted on [SPEN terms](https://spenergynetworks.opendatasoft.com/p/sp-energy-networks-shared-data-licence/)
  - **D-56 Classification:** Tabular: `open-commercial`; CIM: `research-only` (use on hold)
  - **Access Requirement:** **Owner credentials / portal login required**. Record endpoints returned 403 ForbiddenAccess; CIM requires formal Data Request form.
  - **Already in Repo / Spikes:** Schemas documented in [dno-132kv-inventory.md](dno-132kv-inventory.md).
  - **Remaining Gaps:** Row-level data gated; SPM R/X/B units labelled as % on 100 MVA, while SPD labels as pu on 100 MVA (not interchangeable); CIM Licence Details/Permitted Uses must be obtained.

- **Candidate Source:** Northern Powergrid (NPg) LTDS Appendix 3–4 and CIM (Northeast & Yorkshire)
  - **Publisher:** Northern Powergrid
  - **URL:** Appendix 3 Circuit: [https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-appendix-3/information/](https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-appendix-3/information/); CIM: [https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-cim-files/information/](https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-cim-files/information/)
  - **Format:** Tabular (CSV/Parquet) and CIM XML
  - **Coverage:** Northern Powergrid Northeast and Yorkshire. 132 kV and EHV circuits, transformers, and substations (1,670 catalogue records).
  - **Update Cadence:** Annual (November 2025 release)
  - **Licence Name & Link:** [Northern Powergrid Open Data Licence v1.0](https://northernpowergrid.opendatasoft.com/p/opendatalicence/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** **Owner credentials / portal login required**. Catalogue discovery is public, but record requests return 403 without login.
  - **Already in Repo / Spikes:** Metadata documented in [dno-132kv-inventory.md](dno-132kv-inventory.md).
  - **Remaining Gaps:** Ingestion requires registered account; area-specific row filters and impedance units unverified until download.

- **Candidate Source:** Electricity North West (ENWL) LTDS
  - **Publisher:** Electricity North West Limited
  - **URL:** [https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/login-to-ltds-secure-area/](https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/login-to-ltds-secure-area/)
  - **Format:** Excel Workbook, CIM XML, and PDF diagrams
  - **Coverage:** North West England 132 kV circuits and transformers (November 2025 revision).
  - **Update Cadence:** Annual
  - **Licence Name & Link:** Electricity North West Shared Data Licence ([SP ENW Shared Licence](https://electricitynorthwest.huwise.com/p/sp-enw-shared-licence/))
  - **D-56 Classification:** `research-only` (restrictive terms)
  - **Access Requirement:** **Owner credentials / Secure Area login required**. Registration form mandatory.
  - **Already in Repo / Spikes:** Structure documented in [dno-132kv-inventory.md](dno-132kv-inventory.md).
  - **Remaining Gaps:** Unverified row-level data behind login; restrictive clause 1.1 (internal business use) and 3.1(b) (no modification/distribution without consent) require pre-commercial clearance ([PC-5](../requirements/TODO-pre-commercial.md)).

#### 1.3 All-Island Ireland Transmission Network (Ireland 400/220/110 kV, NI 275/110 kV)
- **Candidate Source:** EirGrid / SONI All-Island Ten Year Transmission Forecast Statement (TYTFS) 2024 Study Files
  - **Publisher:** EirGrid Group & SONI Ltd
  - **URL:** [https://cms.eirgrid.ie/sites/default/files/publications/TYTFS2024_studyfiles.zip](https://cms.eirgrid.ie/sites/default/files/publications/TYTFS2024_studyfiles.zip)
  - **Format:** PSS/E RAW (`.raw`) and Sequence (`.seq`) cases (v33 and v35), 17 files in ZIP
  - **Coverage:** All-island transmission grid: Ireland (400/380 kV, 220 kV, 110 kV) and Northern Ireland (275 kV, 110 kV). Models Summer Valley (SV) and Winter Peak (WP) cases for 2024 and 2033. Inspected WP2024 case has 2,025 buses (547 transmission-voltage buses), 1,129 AC branch records, 1,310 transformer records, and 597 generator attachments.
  - **Update Cadence:** Annual (published November 2025)
  - **Licence Name & Link:** No dataset-specific open licence found; governed by [EirGrid Website Disclaimer](https://www.eirgrid.ie/disclaimer) and TYTFS PDF copyright notice (all rights reserved)
  - **D-56 Classification:** `research-only`
  - **Access Requirement:** Open direct download (no credentials required)
  - **Already in Repo / Spikes:** Downloaded, verified, and SHA-256 hashed in [spikes/data-gathering/network-ie/manifest.toml](../../spikes/data-gathering/network-ie/manifest.toml); parsed topology audit in [spikes/data/REPORT.md](../../spikes/data/REPORT.md).
  - **Remaining Gaps:** Commercial clearance required ([PC-2 in TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md)); 132 zero-rating branches require operator definition; terminal projection for 3-winding transformers needs conversion to star bus equivalents; nominal 400 kV assets are modelled on a 380 kV base; generator IDs are model records, not commercial market participant IDs.

- **Candidate Source:** NIE Networks Open Data Portal (Northern Ireland)
  - **Publisher:** Northern Ireland Electricity Networks
  - **URL:** [https://nienetworks.opendatasoft.com/pages/home/](https://nienetworks.opendatasoft.com/pages/home/)
  - **Format:** GeoJSON, Shapefile, CSV
  - **Coverage:** Northern Ireland substations, transformers, transmission switchgear, distribution feeders
  - **Update Cadence:** Periodic
  - **Licence Name & Link:** [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open CKAN/Opendatasoft API
  - **Already in Repo / Spikes:** Documented in [data-sources-ie-weather.md](data-sources-ie-weather.md).
  - **Remaining Gaps:** Full branch impedance parameters for all feeders unverified; basemap layers use Ordnance Survey NI under OGL.

- **Candidate Source:** ESB Networks Network Maps & Publications (Republic of Ireland)
  - **Publisher:** ESB Networks
  - **URL:** [https://www.esbnetworks.ie/](https://www.esbnetworks.ie/)
  - **Format:** PDF schematics, interactive GIS viewers
  - **Coverage:** Republic of Ireland distribution substations and 110/38 kV interfaces
  - **Update Cadence:** Irregular
  - **Licence Name & Link:** All rights reserved ([ESB Copyright Policy](https://www.esbnetworks.ie/data-legal/copyright))
  - **D-56 Classification:** `research-only` (unverified open datasets)
  - **Access Requirement:** Web access; no open machine-readable parameter feed verified (data.gov.ie query returned zero datasets)
  - **Already in Repo / Spikes:** Documented in [data-sources-ie-weather.md](data-sources-ie-weather.md).
  - **Remaining Gaps:** No machine-readable distribution model; fallback is D-37 synthetic feeders for distribution interfaces.

#### 1.4 Cross-Border Interconnectors & External Boundaries (FR-1, FR-1.AC3)
- **Candidate Source:** Elexon BMRS / Insights Solution Reference Interconnector Endpoints
  - **Publisher:** Elexon Limited
  - **URL:** [https://data.elexon.co.uk/bmrs/api/v1/reference/interconnectors/all](https://data.elexon.co.uk/bmrs/api/v1/reference/interconnectors/all)
  - **Format:** JSON REST API
  - **Coverage:** All 10 active GB cross-border interconnectors:
    - France: IFA (`INTFR`), IFA2 (`INTIFA2`), ElecLink (`INTELEC`)
    - Netherlands: BritNed (`INTNED`)
    - Belgium: Nemo Link (`INTNEM`)
    - Norway: North Sea Link (`INTNSL`)
    - Denmark: Viking Link (`INTVKL`)
    - Northern Ireland: Moyle (`INTIRL`)
    - Republic of Ireland: East–West / EWIC (`INTEW`), Greenlink (`INTGRNL`)
  - **Update Cadence:** Static reference register / updated upon asset commissioning
  - **Licence Name & Link:** [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open REST API (no API key required)
  - **Already in Repo / Spikes:** Verified mapping and query in [data-sources-ie-weather.md](data-sources-ie-weather.md); 2024 half-hourly flow streams downloaded in [spikes/data-gathering/elexon/COVERAGE.md](../../spikes/data-gathering/elexon/COVERAGE.md).
  - **Remaining Gaps:** Greenlink (`INTGRNL`) commissioning occurred in early 2025; 2024 contains only 7,102 sparse test observations that must not be treated as a fully commissioned operational baseline for the 2024 historical holdout.

- **Candidate Source:** ENTSO-E Transparency Platform (Scheduled Commercial Exchanges & Physical Flows)
  - **Publisher:** ENTSO-E
  - **URL:** [https://transparency.entsoe.eu/](https://transparency.entsoe.eu/)
  - **Format:** XML REST API (Document types `A09` scheduled exchanges, `A11` physical flows)
  - **Coverage:** Cross-border exchanges across all European borders, including GB↔France, GB↔Belgium, GB↔Netherlands, GB↔Norway, GB↔Denmark, GB↔IE/NI.
  - **Update Cadence:** Hourly / Half-hourly
  - **Licence Name & Link:** [ENTSO-E Free Re-use List (18 Oct 2023)](https://transparencyplatform.zendesk.com/hc/en-us/articles/40921911218961) incorporates [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/) for physical flows (`A11`), but explicitly **excludes** IFA and Nemo Link, and does not list scheduled exchanges (`A09`).
  - **D-56 Classification:** `A11` (non-excluded): `open-commercial`; `A11` (IFA/Nemo Link) and `A09`: `research-only` ([PC-3/PC-5](../requirements/TODO-pre-commercial.md))
  - **Access Requirement:** **Owner credentials / security token required**. REST API requires personal token obtained via account registration and email approval to `transparency@entsoe.eu`.
  - **Already in Repo / Spikes:** Tested without token (HTTP 401 Unauthorized); documented in [spikes/data-gathering/network-ie/COVERAGE.md](../../spikes/data-gathering/network-ie/COVERAGE.md).
  - **Remaining Gaps:** Owner token must be provisioned to unlock automated validation downloads.

#### 1.5 Generation, Storage, and Resource Fleets
- **Candidate Source:** NESO Transmission Entry Capacity (TEC) Register
  - **Publisher:** NESO
  - **URL:** [https://api.neso.energy/dataset/cbd45e54-e6e2-4a38-99f1-8de6fd96d7c1](https://api.neso.energy/dataset/cbd45e54-e6e2-4a38-99f1-8de6fd96d7c1)
  - **Format:** CSV
  - **Coverage:** GB transmission-connected generation and storage projects (snapshot 10 November 2026: 2,220 project rows, 1,759 direct agreements, 461 embedded agreements, 389 marked Built/Operational).
  - **Update Cadence:** Monthly
  - **Licence Name & Link:** [National Energy SO Open Data Licence v1.0](https://www.neso.energy/data-portal/neso-open-licence)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download via CKAN API
  - **Already in Repo / Spikes:** Snapshot downloaded and hashed in [spikes/data-gathering/network-ie/manifest.toml](../../spikes/data-gathering/network-ie/manifest.toml).
  - **Remaining Gaps:** Contains future connection queues; filtering to historical 2024 operational fleet requires effective date cross-referencing.

- **Candidate Source:** DESNZ Renewable Energy Planning Database (REPD)
  - **Publisher:** Department for Energy Security and Net Zero (DESNZ)
  - **URL:** [https://www.gov.uk/government/publications/renewable-energy-planning-database-monthly-extract](https://www.gov.uk/government/publications/renewable-energy-planning-database-monthly-extract)
  - **Format:** CSV / Excel
  - **Coverage:** UK renewable electricity and battery storage projects >150 kW (over 14,000 projects; 3,132 operational).
  - **Update Cadence:** Quarterly (with monthly publication extract)
  - **Licence Name & Link:** [Open Government Licence v3.0](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download
  - **Already in Repo / Spikes:** Evaluated in [data-sources-gb.md](data-sources-gb.md) and [spikes/data/REPORT.md](../../spikes/data/REPORT.md).
  - **Remaining Gaps:** Planning snapshot, not electrical bus assignments; coordinates are British National Grid Easting/Northing; historical threshold changes (<1 MW before 2021).

- **Candidate Source:** DESNZ DUKES Table 5.11 (Power Stations in the United Kingdom)
  - **Publisher:** DESNZ
  - **URL:** [https://www.gov.uk/government/statistics/electricity-chapter-5-digest-of-united-kingdom-energy-statistics-dukes](https://www.gov.uk/government/statistics/electricity-chapter-5-digest-of-united-kingdom-energy-statistics-dukes)
  - **Format:** Excel Workbook (`.xlsx`)
  - **Coverage:** Major Power Producers operating in the UK (1,364 rows with site name, technology, fuel, capacity, commissioning year, connection type, coordinates).
  - **Update Cadence:** Annual (July release, end-May baseline)
  - **Licence Name & Link:** [Open Government Licence v3.0](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download
  - **Already in Repo / Spikes:** Audited in [data-sources-gb.md](data-sources-gb.md).
  - **Remaining Gaps:** Covers Major Power Producers only; geographical and connection details are self-reported and unverified by DESNZ; requires filtering Northern Ireland records for GB fleet.

- **Candidate Source:** Elexon BM Unit Reference Register (`bmunits`)
  - **Publisher:** Elexon Limited
  - **URL:** [https://data.elexon.co.uk/bmrs/api/v1/reference/bmunits/all?format=json](https://data.elexon.co.uk/bmrs/api/v1/reference/bmunits/all?format=json)
  - **Format:** JSON REST API
  - **Coverage:** All 3,134 registered GB Balancing Mechanism units (National Grid BMU ID, Elexon BMU ID, fuel type, generation/demand capacity, lead party, GSP group).
  - **Update Cadence:** Live snapshot
  - **Licence Name & Link:** [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download
  - **Already in Repo / Spikes:** Downloaded and verified in [spikes/data-gathering/elexon/COVERAGE.md](../../spikes/data-gathering/elexon/COVERAGE.md).
  - **Remaining Gaps:** The endpoint provides a current reference snapshot, not a historical 2024 register; requires date-effective joining with historical settlement files to account for decommissioned or renamed units.

- **Candidate Source:** SEMO Registered Capacity & Daily Registered Units Reports
  - **Publisher:** Single Electricity Market Operator (SEMO)
  - **URL:** Registered Capacity: [https://reports.sem-o.com/documents/PUB_MnlyRegisteredCapacity_202412011000.xml](https://reports.sem-o.com/documents/PUB_MnlyRegisteredCapacity_202412011000.xml); Daily Registered Units: [https://reports.sem-o.com/documents/PUB_DailyRegisteredUnits_202609140734.xml](https://reports.sem-o.com/documents/PUB_DailyRegisteredUnits_202609140734.xml)
  - **Format:** XML reports
  - **Coverage:** All-island SEM generating and demand-side units (702 distinct resources in monthly capacity report, 769 distinct units in daily register).
  - **Update Cadence:** Monthly / Daily
  - **Licence Name & Link:** [EirGrid Group Open Data Licence](https://www.smartgriddashboard.com/all/open-data-license/) names SEMO, but reports carry standard website disclaimers; express dataset designation unverified.
  - **D-56 Classification:** `research-only` ([PC-3 in TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md))
  - **Access Requirement:** Open direct download via public reports portal
  - **Already in Repo / Spikes:** Downloaded and SHA-256 hashed in [spikes/data-gathering/network-ie/manifest.toml](../../spikes/data-gathering/network-ie/manifest.toml).
  - **Remaining Gaps:** Commercial reuse permission unverified; participant ResourceNames must be mapped to electrical buses in the TYTFS network model.

---

### Category 2: Network Rules and Operational Constraints / Proxies

#### 2.1 Published Security, Voltage, and Stability Proxies (FR-21, D-55)
- **Candidate Source:** NESO Day-Ahead Constraints Limits and Flow Output
  - **Publisher:** NESO
  - **URL:** [https://api.neso.energy/dataset/cf3cbc92-2d5d-4c2b-bd29-e11a21070b26](https://api.neso.energy/dataset/cf3cbc92-2d5d-4c2b-bd29-e11a21070b26)
  - **Format:** CSV (764,304 rows)
  - **Coverage:** Day-ahead constraint-group forecast limits (`Limit_MW`) and flows (`Flow_MW`) across 32 key transmission boundaries (2023-01-01 through 2026-11-13, half-hourly resolution).
  - **Update Cadence:** Daily
  - **Licence Name & Link:** [National Energy SO Open Data Licence v1.0](https://www.neso.energy/data-portal/neso-open-licence)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open CKAN API download
  - **Already in Repo / Spikes:** Complete CSV downloaded (29.27 MB) and hashed in [spikes/data-gathering/network-ie/manifest.toml](../../spikes/data-gathering/network-ie/manifest.toml).
  - **Remaining Gaps:** Timestamps are recorded in local `GMT_BST`; clock transitions require deduplicating local duplicate hours into UTC; constraint limits are day-ahead forecasts rather than real-time telemetry outturns.

- **Candidate Source:** NESO 24-Months-Ahead Constraint Limits
  - **Publisher:** NESO
  - **URL:** [https://api.neso.energy/dataset/d515b4a9-60a1-489c-a126-004efc04f121](https://api.neso.energy/dataset/d515b4a9-60a1-489c-a126-004efc04f121)
  - **Format:** CSV (109 rows)
  - **Coverage:** Weekly constraint boundary limit forecasts for 12 major transmission boundary groups spanning 2026 week 49 through 2028 week 52.
  - **Update Cadence:** Monthly
  - **Licence Name & Link:** [National Energy SO Open Data Licence v1.0](https://www.neso.energy/data-portal/neso-open-licence)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open CKAN API download
  - **Already in Repo / Spikes:** Downloaded and hashed in [spikes/data-gathering/network-ie/manifest.toml](../../spikes/data-gathering/network-ie/manifest.toml).
  - **Remaining Gaps:** Useful for forward scenario bounds; not a replacement for historical 2024 day-ahead constraint limits.

- **Candidate Source:** EirGrid / SONI System Data Quarter-Hourly (SNSP & Minimum Conventional Generation)
  - **Publisher:** EirGrid & SONI
  - **URL:** [https://cms.eirgrid.ie/sites/default/files/publications/System-Data-Qtr-Hourly-2024.xlsx](https://cms.eirgrid.ie/sites/default/files/publications/System-Data-Qtr-Hourly-2024.xlsx)
  - **Format:** Excel Workbook (`.xlsx`)
  - **Coverage:** 35,136 quarter-hour records for all 366 days of 2024. Includes outturn System Non-Synchronous Penetration (`SNSP`), all-island wind/solar generation, net imports, and conventional generation.
  - **Update Cadence:** Annual retrospective publication / monthly operational reports
  - **Licence Name & Link:** [EirGrid Group Open Data Licence](https://www.smartgriddashboard.com/all/open-data-license/) (file-specific open designation unverified)
  - **D-56 Classification:** `research-only` ([PC-2](../requirements/TODO-pre-commercial.md))
  - **Access Requirement:** Open direct download
  - **Already in Repo / Spikes:** Downloaded, verified, and hashed in [spikes/data-gathering/network-ie/manifest.toml](../../spikes/data-gathering/network-ie/manifest.toml).
  - **Remaining Gaps:** Contains realised SNSP percentages; operational SNSP ceiling rules (historically 75% in 2024, moving towards 80%) and minimum conventional unit requirements (typically 7–8 large synchronous units across IE/NI) must be codified as explicit constraint rules in scenario definitions.

- **Candidate Source:** NESO Operability Strategy & System Defence Framework (Inertia Floors & Voltage Rules)
  - **Publisher:** NESO
  - **URL:** [https://www.neso.energy/publications/operability-strategy-report](https://www.neso.energy/publications/operability-strategy-report) (unverified exact technical parameter appendix URL)
  - **Format:** PDF reports and technical guidance appendices
  - **Coverage:** GB national and regional inertia floors (national floor of ~140 GVA·s, declining towards 100 GVA·s under Stability Pathfinder deployments) and regional minimum-unit requirements (e.g. South Coast, North Scotland).
  - **Update Cadence:** Annual
  - **Licence Name & Link:** No dataset-specific open licence found; NESO general terms apply.
  - **D-56 Classification:** `research-only`
  - **Access Requirement:** Open web access
  - **Already in Repo / Spikes:** Proxy requirements specified in [04-requirements.md](../requirements/04-requirements.md) (FR-21).
  - **Remaining Gaps:** Published as narrative reports; requires manual extraction and codification into structured scenario proxy rules.

#### 2.2 Market Operation and Redispatch Rules (FR-7, D-15, D-59)
- **Candidate Source:** Balancing and Settlement Code (BSC) Baseline Statements & Section Q
  - **Publisher:** Elexon Limited
  - **URL:** [https://www.elexon.co.uk/bsc-and-codes/balancing-settlement-code/bsc-sections/](https://www.elexon.co.uk/bsc-and-codes/balancing-settlement-code/bsc-sections/)
  - **Format:** PDF / HTML legal code documents
  - **Coverage:** GB Balancing Mechanism rules: Bid-Offer Acceptance (BOA) pricing, Net Imbalance Volume (NIV), Single Imbalance Price calculation, System Buy/Sell Prices.
  - **Update Cadence:** Maintained via BSC modifications
  - **Licence Name & Link:** BSC documentation is published for industry code governance; open text access, commercial database redistribution unverified.
  - **D-56 Classification:** `open-commercial` (statutory / code documents)
  - **Access Requirement:** Open web access
  - **Already in Repo / Spikes:** Rules referenced throughout ADR 0006 and FR-7.
  - **Remaining Gaps:** P1 uses cost-based redispatch (D-59) rather than exact bid/offer replay; requires compiling reference generator cost parameters (fuel, carbon, heat rates, variable O&M).

- **Candidate Source:** I-SEM Trading & Settlement Code (Part B) & Industry Guide
  - **Publisher:** SEM Committee / SEMO
  - **URL:** [https://www.sem-o.com/rules-and-governance/rules/trading-and-settlement-code/](https://www.sem-o.com/rules-and-governance/rules/trading-and-settlement-code/)
  - **Format:** PDF legal text and guides
  - **Coverage:** I-SEM market arrangements: Day-Ahead Market (DAM), Intraday Auctions (IDA1–3), Intraday Continuous (IDC), and Balancing Market Imbalance Settlement.
  - **Update Cadence:** Maintained via code modifications
  - **Licence Name & Link:** Published for market participants; standard disclaimers.
  - **D-56 Classification:** `research-only`
  - **Access Requirement:** Open web access
  - **Already in Repo / Spikes:** Analysed in [spikes/data-gathering/semo-2024/COVERAGE.md](../../spikes/data-gathering/semo-2024/COVERAGE.md).
  - **Remaining Gaps:** Legal code rules must be represented as solver constraints and market dispatch stages in the modular market engine (ADR 0006).

- **Candidate Source:** Joint Allocation Office (JAO) Explicit Capacity Allocation Rules (Brexit Decoupling)
  - **Publisher:** JAO S.A.
  - **URL:** [https://www.jao.eu/terms-conditions](https://www.jao.eu/terms-conditions)
  - **Format:** Auction rules PDF, capacity platform results
  - **Coverage:** Post-Brexit explicit capacity allocation rules on GB↔EU borders (IFA, IFA2, BritNed, Nemo Link, Viking Link, ElecLink).
  - **Update Cadence:** Annual rules updates
  - **Licence Name & Link:** JAO Terms of Use (permits internal analysis; commercial service redistribution restricted)
  - **D-56 Classification:** `research-only`
  - **Access Requirement:** Open web access
  - **Already in Repo / Spikes:** Analysed in [data-sources-ie-weather.md](data-sources-ie-weather.md).
  - **Remaining Gaps:** Date-effective coupling regimes must be enforced (FR-1.AC2, FR-7.AC4).

---

### Category 3: Underlying and Embedded Demand

#### 3.1 National and Transmission-System Demand Outturn
- **Candidate Source:** NESO Historic Demand Data (2001–2026)
  - **Publisher:** NESO
  - **URL:** [https://api.neso.energy/dataset/historic-demand-data-2024](https://api.neso.energy/dataset/historic-demand-data-2024) (and portal archive)
  - **Format:** CSV
  - **Coverage:** Complete GB half-hourly demand outturn for 2024 (17,568 rows across 366 dates). Fields include National Demand (`ND`), Transmission System Demand (`TSD`), embedded solar output, embedded wind output, pumped storage pumping, and interconnector flows.
  - **Update Cadence:** Published 21 days in arrears, with retrospective settlement reconciliations
  - **Licence Name & Link:** [National Energy SO Open Data Licence v1.0](https://www.neso.energy/data-portal/neso-open-licence)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open CKAN API download
  - **Already in Repo / Spikes:** Fully downloaded, inspected, and validated in [spikes/data/REPORT.md](../../spikes/data/REPORT.md). Sample in [spikes/data/samples/demand-2024.csv](../../spikes/data/samples/demand-2024.csv).
  - **Remaining Gaps:** Reconciling differences between initial demand outturn (`INDO`/`ITSDO`) and final settlement run demand.

- **Candidate Source:** Elexon Initial Demand Outturn (`demand_outturn`)
  - **Publisher:** Elexon Limited
  - **URL:** [https://data.elexon.co.uk/bmrs/api/v1/demand/outturn](https://data.elexon.co.uk/bmrs/api/v1/demand/outturn)
  - **Format:** JSON REST API / daily files
  - **Coverage:** Initial Demand Outturn (`INDO`) and Initial Transmission System Demand Outturn (`ITSDO`) for all 17,568 settlement periods of 2024 (366 daily files).
  - **Update Cadence:** Daily near-real-time publication
  - **Licence Name & Link:** [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open REST API download
  - **Already in Repo / Spikes:** 366 daily files downloaded (3.40 MiB) and verified in [spikes/data-gathering/elexon/COVERAGE.md](../../spikes/data-gathering/elexon/COVERAGE.md).
  - **Remaining Gaps:** None for initial outturn; historical revisions between initial and final settlement must be handled transparently.

- **Candidate Source:** EirGrid / SONI System Data Quarter-Hourly (Irish Demand)
  - **Publisher:** EirGrid Group
  - **URL:** [https://cms.eirgrid.ie/sites/default/files/publications/System-Data-Qtr-Hourly-2024.xlsx](https://cms.eirgrid.ie/sites/default/files/publications/System-Data-Qtr-Hourly-2024.xlsx)
  - **Format:** Excel Workbook (`.xlsx`)
  - **Coverage:** Quarter-hourly demand outturn for Republic of Ireland (`IE Demand`), Northern Ireland (`NI Demand`), and all-island total for all 35,136 quarter-hours of 2024.
  - **Update Cadence:** Annual
  - **Licence Name & Link:** [EirGrid Group Open Data Licence](https://www.smartgriddashboard.com/all/open-data-license/) (file-level open status unverified)
  - **D-56 Classification:** `research-only` ([PC-2](../requirements/TODO-pre-commercial.md))
  - **Access Requirement:** Open direct download
  - **Already in Repo / Spikes:** Downloaded and verified in [spikes/data-gathering/network-ie/manifest.toml](../../spikes/data-gathering/network-ie/manifest.toml).
  - **Remaining Gaps:** Aggregation from quarter-hour intervals to half-hour settlement periods must be performed consistently without loss of energy balance.

#### 3.2 Embedded Generation & Demand Decomposition (FR-3.AC3)
- **Candidate Source:** Elexon Half-Hourly Generation by Fuel Type (`fuelhh`)
  - **Publisher:** Elexon Limited
  - **URL:** [https://data.elexon.co.uk/bmrs/api/v1/datasets/FUELHH/stream](https://data.elexon.co.uk/bmrs/api/v1/datasets/FUELHH/stream)
  - **Format:** JSON stream (366 daily files)
  - **Coverage:** Outturn generation by fuel type for every settlement period of 2024 (340,894 records). Includes transmission-connected generation as well as embedded solar and embedded wind estimates.
  - **Update Cadence:** Half-hourly
  - **Licence Name & Link:** [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open REST API stream
  - **Already in Repo / Spikes:** 366 daily files downloaded (58.91 MiB) and verified in [spikes/data-gathering/elexon/COVERAGE.md](../../spikes/data-gathering/elexon/COVERAGE.md).
  - **Remaining Gaps:** Embedded solar and wind are model estimates rather than metered outturns; must be added back to National Demand to derive gross underlying demand before re-simulating dispatch.

- **Candidate Source:** DNO Embedded Capacity Registers (ECRs)
  - **Publisher:** All 6 GB DNO Operator Groups (NGED, SSEN, UKPN, SPEN, NPg, ENWL)
  - **URL:** Portal links listed in [data-sources-gb.md](data-sources-gb.md) §3.9
  - **Format:** CSV / XLSX
  - **Coverage:** All distribution-connected generation and storage resources ≥50 kW (Part 1: 50 kW–1 MW, Part 2: ≥1 MW).
  - **Update Cadence:** Monthly
  - **Licence Name & Link:** Varies by DNO: CC BY 4.0 (UKPN, SSEN, SPEN, ENWL) and custom open licences (NGED, Northern Powergrid).
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Mostly open download; UKPN/SPEN require registration for bulk API downloads.
  - **Already in Repo / Spikes:** Evaluated in [data-sources-gb.md](data-sources-gb.md); NESO Scottish embedded register snapshot in [spikes/data-gathering/network-ie/manifest.toml](../../spikes/data-gathering/network-ie/manifest.toml).
  - **Remaining Gaps:** Aggregating capacity across all 14 DNO areas to map distribution-connected renewables to specific transmission GSPs.

#### 3.3 Nodal and Grid Supply Point (GSP) Demand
- **Candidate Source:** NESO ETYS 2025 Appendix G (GSP Winter Peak Demand)
  - **Publisher:** NESO
  - **URL:** [https://www.neso.energy/document/379136/download](https://www.neso.energy/document/379136/download)
  - **Format:** Excel Workbook (`.xlsx`)
  - **Coverage:** 636 GSP node rows across GB; user-submitted winter peak demand forecasts spanning 2025/26 to 2034/35.
  - **Update Cadence:** Annual
  - **Licence Name & Link:** No dataset-specific open licence; NESO website terms apply.
  - **D-56 Classification:** `research-only` ([PC-1](../requirements/TODO-pre-commercial.md))
  - **Access Requirement:** Open direct download
  - **Already in Repo / Spikes:** Downloaded and hashed in [spikes/data-gathering/network-ie/manifest.toml](../../spikes/data-gathering/network-ie/manifest.toml).
  - **Remaining Gaps:** Provides peak demand snapshots, not hourly profiles; must be combined with national half-hourly demand shapes to synthesize dated nodal time-series demand.

- **Candidate Source:** DNO LTDS Table 7 / Attachment 8 Node Registers
  - **Publisher:** Individual DNOs (NGED, SSEN, UKPN, SPEN, NPg, ENWL)
  - **URL:** Respective DNO open data portals
  - **Format:** CSV / Excel
  - **Coverage:** Primary substation and GSP peak demands and power factor data.
  - **Update Cadence:** Annual
  - **Licence Name & Link:** See DNO 132 kV entries above.
  - **D-56 Classification:** Mixed (`open-commercial` for NGED/SSEN, `research-only` for gated DNOs)
  - **Access Requirement:** Open for NGED/SSEN; login required for others.
  - **Already in Repo / Spikes:** NGED Attachment 8 downloaded in [spikes/data-gathering/network-ie/manifest.toml](../../spikes/data-gathering/network-ie/manifest.toml).
  - **Remaining Gaps:** Reconciling local DNO node identifiers with NESO transmission substation naming codes.

---

### Category 4: Network Losses

#### 4.1 Interconnector Losses (FR-1.AC3)
- **Candidate Source:** Interconnector Technical Specifications & Loss Factor Filings
  - **Publisher:** Individual Interconnector Operators (BritNed, National Grid Interconnectors, EirGrid, Moyle Interconnector Ltd, Greenlink Interconnector Ltd)
  - **URL:** BritNed: [https://www.britned.com/](https://www.britned.com/); ElecLink: [https://www.eleclink.co.uk/](https://www.eleclink.co.uk/); Greenlink: [https://greenlink.ie/operations/](https://greenlink.ie/operations/)
  - **Format:** Technical publications, access rules PDF
  - **Coverage:** Loss factors as a function of flow (typically piecewise linear or quadratic converter + cable loss percentages, ranging between 2.0% and 4.5% depending on link and direction).
  - **Update Cadence:** Irregular / modified on regulatory approval
  - **Licence Name & Link:** Statutory regulatory filings / public access rules
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open web access
  - **Already in Repo / Spikes:** Documented in [data-sources-ie-weather.md](data-sources-ie-weather.md) and [spikes/data/REPORT.md](../../spikes/data/REPORT.md).
  - **Remaining Gaps:** Extracting parameters into a versioned TOML table specifying sending-MW, receiving-MW, and directional loss curves for each link.

#### 4.2 Transmission & Distribution Branch Losses (FR-8, FR-10, ADR 0003)
- **Candidate Source:** Circuit Resistance Data in ETYS Appendix B & DNO LTDS Table 1
  - **Publisher:** NESO and DNOs
  - **URL:** See Category 1 entries above
  - **Format:** Excel / CSV
  - **Coverage:** Positive sequence resistance $R_1$ for all modelled transmission lines, underground cables, and transformers.
  - **Update Cadence:** Annual
  - **Licence Name & Link:** See Category 1 entries above.
  - **D-56 Classification:** Mixed (`open-commercial` for NGED/SSEN; `research-only` for ETYS)
  - **Access Requirement:** See Category 1 entries above.
  - **Already in Repo / Spikes:** Parsed resistance columns audited in [spikes/data/REPORT.md](../../spikes/data/REPORT.md).
  - **Remaining Gaps:** In DC optimal power flow dispatch (ADR 0003), ohmic losses $I^2 R$ are represented via signed piecewise-affine linear loss segments around operating points; loss approximations must conserve active power without producing negative losses or double-counting.

#### 4.3 System Loss Accounting and Outturns
- **Candidate Source:** Elexon Transmission Loss Multipliers (TLM)
  - **Publisher:** Elexon Limited
  - **URL:** [https://data.elexon.co.uk/bmrs/api/v1/balancing/settlement/system-prices/](https://data.elexon.co.uk/bmrs/api/v1/balancing/settlement/system-prices/) (and BSC settlement streams)
  - **Format:** JSON REST API
  - **Coverage:** Half-hourly Transmission Loss Multipliers for delivering and off-taking BM units across GB.
  - **Update Cadence:** Half-hourly
  - **Licence Name & Link:** [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open REST API download
  - **Already in Repo / Spikes:** Included within BMRS settlement streams in [spikes/data-gathering/elexon/COVERAGE.md](../../spikes/data-gathering/elexon/COVERAGE.md).
  - **Remaining Gaps:** Reconciling physical network loss model outputs with commercial settlement TLM accounting.

---

### Category 5: Calendar, Holidays, and Clock Changes

#### 5.1 Market Settlement Calendars & Bank Holidays
- **Candidate Source:** GOV.UK Bank Holidays API
  - **Publisher:** UK Cabinet Office / Government Digital Service
  - **URL:** [https://www.gov.uk/bank-holidays.json](https://www.gov.uk/bank-holidays.json)
  - **Format:** JSON
  - **Coverage:** Official bank holidays for England and Wales, Scotland, and Northern Ireland (historical and forward schedules).
  - **Update Cadence:** Continuous
  - **Licence Name & Link:** [Open Government Licence v3.0](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download (no credentials required)
  - **Already in Repo / Spikes:** Specification and holiday logic verified in [spikes/data/REPORT.md](../../spikes/data/REPORT.md).
  - **Remaining Gaps:** Republic of Ireland public holidays must be added from Irish statutory schedules to complete the all-island calendar.

- **Candidate Source:** Irish Public Holiday Schedule
  - **Publisher:** Government of Ireland (Citizens Information / Department of Enterprise, Trade and Employment)
  - **URL:** [https://www.citizensinformation.ie/en/employment/employment-rights-and-conditions/leave-and-holidays/public-holidays/](https://www.citizensinformation.ie/en/employment/employment-rights-and-conditions/leave-and-holidays/public-holidays/)
  - **Format:** Web / Open Data
  - **Coverage:** 10 official annual public holidays in Ireland (including St Brigid's Day introduced in 2023).
  - **Update Cadence:** Annual
  - **Licence Name & Link:** [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/) (via data.gov.ie)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open web access
  - **Already in Repo / Spikes:** Referenced in SEMO coverage report [spikes/data-gathering/semo-2024/COVERAGE.md](../../spikes/data-gathering/semo-2024/COVERAGE.md).
  - **Remaining Gaps:** Codify in a static versioned JSON/TOML calendar within the repository.

#### 5.2 Daylight Saving Clock Transitions (FR-3.AC1, FR-5)
- **Candidate Source:** IANA Time Zone Database (`tzdata`)
  - **Publisher:** Internet Assigned Numbers Authority (IANA)
  - **URL:** [https://www.iana.org/time-zones](https://www.iana.org/time-zones)
  - **Format:** Standard Olson timezone compilation (`Europe/London`, `Europe/Dublin`)
  - **Coverage:** Complete historical and future daylight saving transition timestamps. In 2024:
    - Spring transition (2024-03-31): clocks advance 01:00 GMT $\rightarrow$ 02:00 BST/IST (short day: exactly 46 settlement periods).
    - Autumn transition (2024-10-27): clocks fall back 02:00 BST/IST $\rightarrow$ 01:00 GMT (long day: exactly 50 settlement periods).
  - **Update Cadence:** Semi-annual releases
  - **Licence Name & Link:** Public Domain
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Standard system library / open download
  - **Already in Repo / Spikes:** Full transition verification in [spikes/data-gathering/elexon/COVERAGE.md](../../spikes/data-gathering/elexon/COVERAGE.md) and [spikes/data-gathering/semo-2024/COVERAGE.md](../../spikes/data-gathering/semo-2024/COVERAGE.md).
  - **Remaining Gaps:** None. Canonical time representation in UTC prevents all timezone ambiguities; settlement mapping is strictly derived.

#### 5.3 Leap Year & Calendar Integrity (FR-3.AC1)
- **Candidate Source:** ISO 8601 UTC Continuous Delivery Interval Engine
  - **Publisher:** Internal simulator specification / standard time arithmetic
  - **Coverage:** Complete UTC timestamp sequences:
    - Non-leap years: exactly 365 days $\times$ 48 = 17,520 half-hours.
    - Leap years (2024): exactly 366 days $\times$ 48 = 17,568 half-hours.
  - **Format:** ISO 8601 UTC timestamps (`YYYY-MM-DDTHH:MM:SSZ`)
  - **Licence Name & Link:** N/A (mathematical standard)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Built-in
  - **Already in Repo / Spikes:** Verified across all 12 Elexon 2024 datasets and SEMO 2024 market datasets.
  - **Remaining Gaps:** None.

---

### Category 6: Scenario Identity Inputs (FR-4, NFR-1, FR-3)

#### 6.1 Canonical Scenario Definition & Execution Settings (FR-4, NFR-1)
- **Candidate Source:** Canonical TOML Scenario Schema & Checksum Registry
  - **Publisher:** Internal repository specification (ADR 0002, [04-requirements.md](../requirements/04-requirements.md) FR-4)
  - **Format:** TOML (`.toml`)
  - **Coverage:** Versioned specification defining: schema version, geography/asset snapshot, input-manifest SHA-256 hashes, time range, resolution, market rule version, solver configuration (gap tolerance, time limits), rolling horizon parameters (48-hour look-ahead, 24-hour advance), and deterministic seed policy.
  - **Licence Name & Link:** Project open source repository licence
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Internal repository file
  - **Already in Repo / Spikes:** Conceptual model accepted in ADR 0002; manifest schemas in spikes.
  - **Remaining Gaps:** Complete Rust deserializer and JSON-schema emission in M1 implementation.

#### 6.2 Fuel, Carbon, and Variable Operating Costs (FR-7, D-59)
- **Candidate Source:** UK ETS & EU ETS Carbon Auction Clearing Prices
  - **Publisher:** ICE Futures Europe (for UK ETS) / EEX (for EU ETS) / UK Department for Energy Security and Net Zero
  - **URL:** [https://www.gov.uk/government/publications/uk-ets-auction-results](https://www.gov.uk/government/publications/uk-ets-auction-results)
  - **Format:** CSV / HTML tables
  - **Coverage:** Daily and weekly clearing prices for UK Emissions Trading Scheme allowances (GBP/tCO$_2$) and EU Allowances (EUR/tCO$_2$) throughout 2024.
  - **Update Cadence:** Auction frequency (bi-weekly / daily)
  - **Licence Name & Link:** UK Government publications under [Open Government Licence v3.0](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download
  - **Already in Repo / Spikes:** Parameter requirement identified in [04-requirements.md](../requirements/04-requirements.md) FR-7.AC2.
  - **Remaining Gaps:** Collate daily historical time-series into immutable versioned input table.

- **Candidate Source:** DESNZ Fossil Fuel Price Assumptions & World Bank Commodity Price Data
  - **Publisher:** DESNZ & World Bank
  - **URL:** DESNZ: [https://www.gov.uk/government/collections/fossil-fuel-price-assumptions](https://www.gov.uk/government/collections/fossil-fuel-price-assumptions); World Bank: [https://www.worldbank.org/en/research/commodity-markets](https://www.worldbank.org/en/research/commodity-markets)
  - **Format:** Excel / CSV
  - **Coverage:** Daily/monthly wholesale natural gas (NBP, TTF), coal (ARA), and oil prices for 2024.
  - **Update Cadence:** Monthly / Annual
  - **Licence Name & Link:** World Bank Open Data (CC BY 4.0) / UK OGL v3.0
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download
  - **Already in Repo / Spikes:** Outlined in [data-sources-gb.md](data-sources-gb.md).
  - **Remaining Gaps:** Formulate standardized technology short-run marginal cost (SRMC) formulas combining fuel price, carbon price, heat rate, and variable O&M.

- **Candidate Source:** CfD & RO Renewable Subsidy Lost-Opportunity Cost Parameters
  - **Publisher:** Low Carbon Contracts Company (LCCC) & Ofgem
  - **URL:** [https://www.lowcarboncontracts.uk/data-portal](https://www.lowcarboncontracts.uk/data-portal)
  - **Format:** CSV
  - **Coverage:** Contract for Difference (CfD) strike prices and Renewables Obligation (RO) buyout prices and value per MWh.
  - **Update Cadence:** Annual / quarterly indexation
  - **Licence Name & Link:** [Open Government Licence v3.0](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download
  - **Already in Repo / Spikes:** Required under FR-7.AC2 to price renewable redispatch opportunity costs.
  - **Remaining Gaps:** Crosswalk CfD/RO plant registers to BM units and network buses.

#### 6.3 Weather-Derived Renewable Resource Profiles (FR-3, FR-17)
- **Candidate Source:** Copernicus Climate Data Store (CDS) ERA5 Reanalysis (Single Levels)
  - **Publisher:** European Centre for Medium-Range Weather Forecasts (ECMWF) / Copernicus Climate Change Service
  - **URL:** [https://cds.climate.copernicus.eu/datasets/reanalysis-era5-single-levels](https://cds.climate.copernicus.eu/datasets/reanalysis-era5-single-levels)
  - **Format:** GRIB / NetCDF via CDS API
  - **Coverage:** Global regular 0.25° grid (~31 km native resolution). Hourly series for 2024: 100-metre wind components (`u100`, `v100`), 10-metre wind components (`u10`, `v10`), 2-metre temperature (`2t`), surface solar radiation downwards (`ssrd`), total precipitation (`tp`), total-sky direct solar radiation (`fdir`). Bounding box for GB+IE: 48°N–61°N, 12°W–10°E (4,717 grid points).
  - **Update Cadence:** Monthly updates in arrears (complete historical 2024 available)
  - **Licence Name & Link:** [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/) (explicitly attached to current CDS dataset catalogue)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** **Owner credentials required**. Requires CDS user account, agreement to licence terms on web portal, and `~/.cdsapirc` containing personal UID and API key.
  - **Already in Repo / Spikes:** Monthly request template drafted in [spikes/data/era5-request-2024-01.json](../../spikes/data/era5-request-2024-01.json); volume budget (~1.33 GB uncompressed float32 per year) calculated in [spikes/data/REPORT.md](../../spikes/data/REPORT.md). Zero weather bytes currently downloaded.
  - **Remaining Gaps:** Requires owner CDS account credentials to execute download; requires open-source power-curve conversion software (e.g. `atlite` or internal Rust conversion module) to convert wind/radiation fields into GSP wind and solar capacity factors.

- **Candidate Source:** Met Office MIDAS Open Surface Observations (Validation Companion)
  - **Publisher:** UK Met Office / CEDA Archive
  - **URL:** [https://catalogue.ceda.ac.uk/uuid/dbd451271eb04662beade68da43546e1/](https://catalogue.ceda.ac.uk/uuid/dbd451271eb04662beade68da43546e1/)
  - **Format:** CSV via CEDA Archive
  - **Coverage:** Hourly weather observations from weather stations across the UK.
  - **Update Cadence:** Annual
  - **Licence Name & Link:** [Open Government Licence v3.0](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Free CEDA user registration required
  - **Already in Repo / Spikes:** Evaluated in [data-sources-ie-weather.md](data-sources-ie-weather.md).
  - **Remaining Gaps:** Point observations require spatial interpolation; primary purpose is validating ERA5 reanalysis ground accuracy.

- **Candidate Source:** Met Éireann MÉRA Reanalysis & Open Data (Ireland)
  - **Publisher:** Met Éireann
  - **URL:** [https://www.met.ie/climate/available-data/mera](https://www.met.ie/climate/available-data/mera)
  - **Format:** NetCDF / CSV
  - **Coverage:** 2.5 km regional atmospheric reanalysis for Ireland (historical archive covers 1981–August 2019; operational station data covers 2024).
  - **Update Cadence:** Archived reanalysis; station data updated daily
  - **Licence Name & Link:** [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open web access
  - **Already in Repo / Spikes:** Documented in [data-sources-ie-weather.md](data-sources-ie-weather.md).
  - **Remaining Gaps:** MÉRA reanalysis ends in 2019, so ERA5 is required for the common 2024 weather year.

- **Candidate Source:** Renewables.ninja (Simulated Wind & Solar Benchmark)
  - **Publisher:** Renewables.ninja (Imperial College London / ETH Zürich)
  - **URL:** [https://www.renewables.ninja/](https://www.renewables.ninja/)
  - **Format:** CSV / JSON API
  - **Coverage:** Pre-calculated hourly capacity factors by region
  - **Update Cadence:** Irregular
  - **Licence Name & Link:** [Creative Commons Attribution-NonCommercial 4.0 International (CC BY-NC 4.0)](https://creativecommons.org/licenses/by-nc/4.0/)
  - **D-56 Classification:** `research-only` (strictly non-commercial)
  - **Access Requirement:** Free account / API key
  - **Already in Repo / Spikes:** Audited in [data-sources-ie-weather.md](data-sources-ie-weather.md).
  - **Remaining Gaps:** Prohibited from commercial use; serves as an offline research-only benchmark to validate our own ERA5 wind/solar conversion pipelines.

#### 6.4 Historical Validation Holdout Series (2024 Holdout Year, FR-12)
- **Candidate Source:** Elexon BMRS 2024 Operational and Market Datasets
  - **Publisher:** Elexon Limited
  - **URL:** [https://data.elexon.co.uk/](https://data.elexon.co.uk/)
  - **Format:** JSON / CSV streams (12 distinct streams, 3,673 files, 4.16 GiB total)
  - **Coverage:** All 17,568 settlement periods of 2024 across 12 streams:
    - System Prices (`system_prices`): 17,568 periods (imbalance buy/sell prices)
    - Market Index Data (`mid`): 35,091 records (APXMIDP and N2EXMIDP prices/volumes)
    - Physical Notifications (`pn`): 36.6M records
    - Bid-Offer Data (`bod`): 55.1M records
    - Bid-Offer Acceptances (`boalf`): 3.47M records across 17,563 periods (5 sparse periods: `2024-03-14/21`, `2024-07-08/27`, `2024-09-22/29`, `2024-11-03/23`, `2024-11-03/24`)
    - Maximum Limits (`mels`, `mils`): 36.7M / 34.3M records
    - Outturn Generation by Fuel (`fuelhh`): 340,894 records
    - Interconnector Flows (`interconnectors`): 165,214 records
    - REMIT Messages (`remit`): 48,191 records
  - **Update Cadence:** Settlement period stream
  - **Licence Name & Link:** [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download (no credentials required)
  - **Already in Repo / Spikes:** 100% downloaded, verified, and SHA-256 hashed in [spikes/data-gathering/elexon/COVERAGE.md](../../spikes/data-gathering/elexon/COVERAGE.md); stored outside git in `/home/njotob/energy-data/raw/elexon/`.
  - **Remaining Gaps:** None for raw acquisition; Market Index Data must be labelled strictly as Market Index Price (not day-ahead auction price).

- **Candidate Source:** SEMO / SEMOpx 2024 I-SEM Price Archives
  - **Publisher:** SEMO & SEMOpx
  - **URL:** Lookback Market: [https://www.semopx.com/documents/general-publications/lookback_mkt.xlsx](https://www.semopx.com/documents/general-publications/lookback_mkt.xlsx) (and historical archives); Imbalance Price: [https://reports.sem-o.com/documents/](https://reports.sem-o.com/documents/)
  - **Format:** Excel Workbook & XML/CSV archives (43.6 MB)
  - **Coverage:** Full calendar year 2024:
    - Day-Ahead Market (`DAM`): 8,784 hourly prices (full UTC year; mean €108.91/MWh matches official December 2024 market report).
    - Intraday Auctions (`IDA1`): 17,568 half-hour prices (full UTC year).
    - Intraday Auctions (`IDA2`, `IDA3`): 8,784 / 4,392 half-hour prices.
    - Imbalance Settlement 30-min (`30MIN`): 17,507 rows (61 missing intervals).
    - Imbalance Settlement 5-min (`5MIN`): 96,147 rows (December absent).
  - **Update Cadence:** Continuous market operations / periodic archives
  - **Licence Name & Link:** [EirGrid Group Open Data Licence](https://www.smartgriddashboard.com/all/open-data-license/) (express archive coverage unverified; disclaimers apply)
  - **D-56 Classification:** `research-only` ([PC-3](../requirements/TODO-pre-commercial.md))
  - **Access Requirement:** Open direct download via public links
  - **Already in Repo / Spikes:** Downloaded, audited, and independently re-counted in [spikes/data-gathering/semo-2024/COVERAGE.md](../../spikes/data-gathering/semo-2024/COVERAGE.md); stored outside git in `/home/njotob/energy-data/raw/semo/2024/`.
  - **Remaining Gaps:** 61 missing 30-minute imbalance intervals and missing December 5-minute data; offsetless `START_TIME` timezone confirmation (Q-C-11); commercial reuse clearance ([PC-3](../requirements/TODO-pre-commercial.md)).

- **Candidate Source:** NESO Outturn Thermal Constraint Costs (2024–2025)
  - **Publisher:** NESO
  - **URL:** [https://api.neso.energy/dataset/f0055054-c55c-4068-a01c-61da4334e58f](https://api.neso.energy/dataset/f0055054-c55c-4068-a01c-61da4334e58f)
  - **Format:** CSV (2,190 rows)
  - **Coverage:** Daily thermal constraint costs (`Daily Cost GBP`) for six major constraint groups from 2024-04-01 through 2025-03-31 (full financial year).
  - **Update Cadence:** Monthly / annual updates
  - **Licence Name & Link:** [National Energy SO Open Data Licence v1.0](https://www.neso.energy/data-portal/neso-open-licence)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open CKAN API download
  - **Already in Repo / Spikes:** Downloaded and hashed in [spikes/data-gathering/network-ie/manifest.toml](../../spikes/data-gathering/network-ie/manifest.toml).
  - **Remaining Gaps:** Covers financial year 2024/25; calendar Q1 2024 (January–March 2024) requires the previous 2023/24 financial year outturn table to complete calendar year 2024.

#### 6.5 Exogenous Foreign Exchange Rates (FR-1.AC4, D-38)
- **Candidate Source:** European Central Bank (ECB) Reference Rates
  - **Publisher:** European Central Bank
  - **URL:** [https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/html/index.en.html](https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/html/index.en.html)
  - **Format:** CSV / XML / JSON
  - **Coverage:** Daily EUR/GBP exchange rates published around 16:00 CET on working days.
  - **Update Cadence:** Daily (working days)
  - **Licence Name & Link:** [ECB Reuse Disclaimer](https://www.ecb.europa.eu/services/disclaimer/html/index.en.html) (requires source acknowledgement; disclosure required if sold)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download (no credentials required)
  - **Already in Repo / Spikes:** Evaluated in [data-sources-ie-weather.md](data-sources-ie-weather.md).
  - **Remaining Gaps:** Non-working days (weekends, TARGET holidays) require forward-filling the last valid reference rate into a complete 366-day UTC time series.

- **Candidate Source:** Bank of England Database (`XUDLSER`)
  - **Publisher:** Bank of England
  - **URL:** [https://www.bankofengland.co.uk/boeapps/database/](https://www.bankofengland.co.uk/boeapps/database/)
  - **Format:** CSV
  - **Coverage:** Daily spot exchange rates (GBP to EUR).
  - **Update Cadence:** Daily
  - **Licence Name & Link:** [Bank of England Legal Terms](https://www.bankofengland.co.uk/legal) (OGL v3.0, but selected third-party series excluded; series clearance unverified)
  - **D-56 Classification:** `open-commercial` (subject to series check)
  - **Access Requirement:** Open direct download
  - **Already in Repo / Spikes:** Evaluated in [data-sources-ie-weather.md](data-sources-ie-weather.md).
  - **Remaining Gaps:** ECB reference rate is preferred primary numeraire source due to explicit EU/Irish statutory role.

---

## 3. Owner Credentials Summary & Capability Unlock Matrix

Several candidate sources require human owner actions (registering accounts, requesting API tokens, or signing data access agreements). The table below details every credential requirement and what it unlocks for M1:

| Required Credential / Account | Target Service / Portal | Access Type | What It Unlocks for Milestone M1 | Action Required by Owner |
|---|---|---|---|---|
| **ENTSO-E API Security Token** | ENTSO-E Transparency Platform ([transparency.entsoe.eu](https://transparency.entsoe.eu/)) | Personal API token (`securityToken`) | Unlocks automated REST API retrieval of scheduled commercial interconnector exchanges (`A09`), cross-border physical flows (`A11`), and pan-European day-ahead prices for independent cross-validation. | 1. Register free account on [transparency.entsoe.eu](https://transparency.entsoe.eu/).<br>2. Email `transparency@entsoe.eu` with subject `RESTful API access`.<br>3. Generate token under "My Account" and provide in environment variable. |
| **Copernicus CDS Account & API Key** | Copernicus Climate Data Store ([cds.climate.copernicus.eu](https://cds.climate.copernicus.eu/)) | User UID & API Key (`~/.cdsapirc`) | Unlocks automated submission and downloading of ERA5 and ERA5-Land gridded hourly weather data (wind, solar radiation, temperature) for 2024 to generate renewable generator profiles. | 1. Create free account at [cds.climate.copernicus.eu](https://cds.climate.copernicus.eu/).<br>2. Accept online dataset terms for ERA5 single levels.<br>3. Copy API key from user profile into `~/.cdsapirc`. |
| **UK Power Networks Open Data Login & CIM Request** | UK Power Networks Open Data Portal ([ukpowernetworks.opendatasoft.com](https://ukpowernetworks.opendatasoft.com/)) | Registered user account & CIM access approval | Unlocks row-level API and export access to LTDS Table 1 (Circuit data) and Table 2 (Transformers) for EPN, LPN, and SPN licence areas (5,076 records), plus SharePoint access to CIM network model deliverables. | 1. Register free account on UKPN portal.<br>2. Fill out and submit the [CIM Access Request Form](https://ukpowernetworks.opendatasoft.com/explore/forms/cim-access-request-form/).<br>3. Provide API key or download deliverables. |
| **SP Energy Networks Open Data Login & Data Request** | SP Energy Networks Open Data Portal ([spenergynetworks.opendatasoft.com](https://spenergynetworks.opendatasoft.com/)) | Registered user account & Shared Data Request | Unlocks row-level download of SP Manweb (SPM) 132 kV circuit and transformer tables (1,798 records) and CIM EQ profile cases. | 1. Register account on SPEN portal.<br>2. Submit the [Data Request Form](https://spenergynetworks.opendatasoft.com/explore/forms/data-request/) under ENA Shared Data Licence for SPM CIM/LTDS. |
| **Northern Powergrid Open Data Portal Login** | Northern Powergrid Portal ([northernpowergrid.opendatasoft.com](https://northernpowergrid.opendatasoft.com/)) | Registered user account | Unlocks row-level export and API queries for LTDS Appendix 3 (Circuits) and Appendix 4 (Transformers) for Northeast and Yorkshire licence areas (1,670 records). | 1. Register account on Northern Powergrid portal.<br>2. Export Appendix 3 and Appendix 4 CSV/Parquet files. |
| **Electricity North West Secure Area Login** | ENWL Secure Portal ([enwl.co.uk](https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/login-to-ltds-secure-area/)) | Secure area username & password | Unlocks access to download ENWL's Long Term Development Statement tables, schematics, and CIM models covering North West England 132 kV. | 1. Register on ENWL website for LTDS Secure Area access.<br>2. Download November 2025 statement tables and models. |
| **D-65 Target Runner Provisioning** | Dedicated Linux Benchmark/Run Machine (D-48, D-65) | Host SSH access / self-hosted GitHub Actions runner | Unlocks execution of heavy M1 data processing jobs, PostgreSQL database population, and large-scale validation runs outside the lightweight coding container. | 1. Confirm target runner hardware specifications (Q-N-7).<br>2. Configure and attach GitHub self-hosted runner. |

---

## 4. Pre-Commercial Clearance & Data Provenance Status

Per D-56, all research-only datasets are admitted for non-commercial research and development, but must be cleared, replaced, or removed prior to commercial release ([TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md)):

| Pre-Commercial Item | Asset / Dataset | Primary Rights Holder | Current Legal Barrier / Risk | Strategy for Resolution |
|---|---|---|---|---|
| **PC-1** | NESO ETYS 2025 Appendix B | NESO (with TO contributions from NGET, SPT, SHET, OFTOs) | NESO general website terms restrict commercial exploitation; workbook not explicitly covered by portal licence grant. | Submit formal written permission request to NESO Data Portal team for commercial SaaS use of extracted electrical network parameters. |
| **PC-2** | EirGrid / SONI TYTFS 2024 Study Files | EirGrid Group & SONI Ltd | Website disclaimer forbids public/commercial use without authorization; joint report reserves rights. | Formal written request to EirGrid/SONI Open Data and Regulation teams for commercial derivative modelling rights. |
| **PC-3** | SEMO & SEMOpx Market Price Datasets | SEMO & SEMOpx | Standard market report disclaimers; express open licence grant unverified on archive download pages. | Seek formal confirmation from SEMO/SEMOpx that published historical settlement and auction reports are covered by group open data terms. |
| **PC-4** | GB Day-Ahead Power Exchange Prices | Nord Pool N2EX & EPEX SPOT | Exchange power data services require paid redistribution licences (€1,040+/month). | Maintain D-57 stance: default to free historical proxies (Elexon Market Index Data and imbalance prices); procure paid commercial exchange feed only if customer funding is secured. |
| **PC-5** | Restrictive DNO Shared Data Licences | UKPN, SPEN, ENWL | ENA Shared Data Licence clauses restrict commercial derivation, aggregation, and redistribution without written consent. | Seek commercial use extensions under DSO open data consultations or derive topology from open tabular equivalents. |

---

## 5. Input Completeness Assessment & Triage

Every required M1 input identified in [06-roadmap.md](../requirements/06-roadmap.md), [04-requirements.md](../requirements/04-requirements.md) (FR-1–5, FR-19, FR-21, NFR-1/2), and [AGENTS.md](../../AGENTS.md) has been cross-referenced against available candidate sources:

| M1 Input Requirement | Known Candidate Sources Identified? | Current Ingestion Status | Identified Remaining Gaps |
|---|---|---|---|
| **Dated GB 400/275 kV Network** | Yes (NESO ETYS Appendix B) | Downloaded & parsed in spikes | Commercial clearance (PC-1); coordinates missing. |
| **Dated Scottish 132 kV Network** | Yes (ETYS SHE Transmission & SPT) | Downloaded & parsed in spikes | Commercial clearance (PC-1); busbar switching details. |
| **Dated E&W DNO 132 kV Networks (14 areas)** | Yes (NGED, SSEN, UKPN, SPEN, NPg, ENWL) | Partial (NGED & SSEN downloaded; others gated) | 4 DNO groups require owner portal logins (see §3). |
| **Dated All-Island Ireland Network (400/220/275/110 kV)** | Yes (EirGrid/SONI TYTFS 2024 RAW/SEQ) | Downloaded & parsed in spikes | Commercial clearance (PC-2); transformer 3W star conversion. |
| **Interconnector Boundaries & Flows (10 links)** | Yes (Elexon BMRS, ENTSO-E, Operator sites) | Complete 2024 Elexon flows downloaded | Greenlink commissioned 2025 (sparse 2024 test flow). |
| **Generation & Storage Fleet Registers** | Yes (TEC Register, REPD, DUKES 5.11, Elexon BMU, SEMO) | Snapshots downloaded in spikes | Historical crosswalk (asset $\rightarrow$ BMU $\rightarrow$ network node). |
| **Security & Stability Proxies (Boundary Limits)** | Yes (NESO Day-Ahead Constraints, 24-Mo Limits, ETYS) | Downloaded in spikes | Convert local GMT/BST timestamps to UTC. |
| **Stability Proxies (SNSP & Inertia)** | Yes (EirGrid System Data, NESO Operability Strategy) | Downloaded in spikes | Codify numeric constraint thresholds into scenario rules. |
| **Market Trading & Redispatch Rules** | Yes (BSC Code Section Q, I-SEM TSC Part B) | Rules documented & verified | Formulate technology SRMC cost curves (D-59). |
| **National & Transmission Demand Outturn** | Yes (NESO Demand Data, Elexon INDO/ITSDO, EirGrid) | 100% downloaded for 2024 | Reconcile initial outturn vs final settlement run. |
| **Embedded Generation Decomposition** | Yes (Elexon FUELHH embedded solar/wind, NESO, ECR) | 100% downloaded for 2024 | Add embedded output to net demand for gross load. |
| **Nodal / GSP Peak Demand** | Yes (ETYS Appendix G, DNO LTDS Table 7) | Downloaded in spikes | Reconcile DNO node names with transmission GSPs. |
| **Interconnector Loss Models** | Yes (Operator technical rules, Elexon parameters) | Parameters documented | Codify piecewise linear loss curves per direction. |
| **Branch Resistances / Loss Segments** | Yes (ETYS Appendix B, DNO Table 1, TYTFS RAW) | Downloaded & audited | Implement affine loss segment approximation (ADR 0003). |
| **Settlement Loss Outturns (TLM)** | Yes (Elexon BMRS TLM, EirGrid System Data) | Downloaded for 2024 | Validate simulated losses against settlement outturns. |
| **Calendar, Bank Holidays & Clock Changes** | Yes (GOV.UK API, IANA tzdata, Elexon calendars) | Rules verified; 46/50 SP verified | Compile into immutable versioned scenario calendar. |
| **Canonical Scenario Identity Schema** | Yes (Internal TOML schema, ADR 0002) | Architectural specification baselined | Implement Rust serde validation models in M1. |
| **Fuel & Carbon Commodity Costs** | Yes (UK ETS, EU ETS, DESNZ price series) | Sources identified | Compile historical 2024 daily price vector. |
| **Weather & Renewable Resource Profiles** | Yes (Copernicus CDS ERA5 single-levels, CEDA MIDAS) | Monthly request template in repo; 0 bytes downloaded | **Owner CDS account required** to download 2024 weather. |
| **Historical Validation Holdouts (2024)** | Yes (Elexon 12 streams, SEMO 2024 prices, NESO costs) | 100% downloaded & verified in spikes | Missing December 5-min Irish imbalance data. |
| **Foreign Exchange Rates (EUR/GBP)** | Yes (ECB Reference Rates, Bank of England) | Verified in data catalogue | Forward-fill weekend/holiday dates for continuous UTC series. |

**Count of M1 inputs with no known source: 0**.
