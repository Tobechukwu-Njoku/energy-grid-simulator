# M1 Data Ingest and Network Model: Data Inventory & Ingest Preparation

Revised and verified on **2026-11-14** following independent verification report [docs/requirements/reviews/gpt-2026-11-14-pr52-m1-inventory.md](../requirements/reviews/gpt-2026-11-14-pr52-m1-inventory.md). Prepares data gathering and ingestion for milestone **M1** (data ingest and network model) of the GB + all-island Ireland grid simulator per [06-roadmap.md](../requirements/06-roadmap.md) and [04-requirements.md](../requirements/04-requirements.md) (FR-1–5, FR-19, FR-21, NFR-1/2). Follows [AGENTS.md](../../AGENTS.md) Rule 10 and owner decision D-56: every candidate source is classified strictly as `open-commercial` or `research-only`. Share-alike (ODbL / OSM) data is strictly excluded across all layers (including PyPSA-GB coordinates or reduced networks, per FR-19.AC3). Raw restricted research data is never committed to the repository; download scripts, manifests, and verification reports only.

---

## 1. Executive Summary & Inventory Coverage

### Scope and Governing Decisions
- **Geography & Topology (FR-1, FR-2, D-11, D-32, D-60):** Great Britain (400 kV, 275 kV, and Scottish 132 kV transmission; plus full 132 kV across all 12 England & Wales DNO licence areas, with coverage dispositions tracked for both Scottish distribution areas under D-60: 14 total DNO areas) plus all-island Ireland (400/380 kV, 220 kV, 110 kV in Ireland and 275 kV, 110 kV in Northern Ireland), with 10 interconnector boundaries.
- **Interconnector Geography (FR-1.AC3):** The 10 interconnector boundaries comprise:
  - **7 GB-to-external-system links:** France (IFA, IFA2, ElecLink), Netherlands (BritNed), Belgium (Nemo Link), Norway (North Sea Link / NSL), Denmark (Viking Link).
  - **3 GB-to-Ireland internal links:** Moyle (Scotland $\leftrightarrow$ Northern Ireland), East–West / EWIC (Wales $\leftrightarrow$ Republic of Ireland), Greenlink (Wales $\leftrightarrow$ Republic of Ireland). These three internal links couple the two modelled transmission geographies.
  - *Greenlink baseline qualification:* Greenlink (`INTGRNL`) recorded 7,102 sparse observations in 2024; cause unverified; not a fully commissioned 2024 baseline. Commercial commissioning was announced in early 2025; count and sparsity are verified from Elexon BMRS flow records, but operational cause is unverified from flow counts alone. They must not be treated as a fully commissioned operational baseline for the 2024 historical holdout.
- **Physics Formulation (FR-8, FR-21, D-55):** DC network model augmented by published operational security and stability constraint proxies (boundary transfer limits, regional inertia floors, minimum units, Irish SNSP limits). Full AC power flow and dynamic simulation stay in P2/P4.
- **Market & Operational Rules (FR-7, D-15, D-59):** GB national market scheduling and I-SEM ex-ante trading, followed by cost-based redispatch against physical constraints. Historical bid/offer replay is not used; redispatch is driven by estimated generator operating costs.
- **Validation Holdout (FR-12, D-66, D-70):** Pinned historical baseline year 2024 (leap year: 366 days, 17,568 half-hour settlement periods). Reconstructions are distinguished from as-of forecast backtests. Known gaps in historical validation series (such as missing intervals in SEMO imbalance archives) are explicitly tracked rather than assumed complete.
- **Provenance & Pre-commercial Clearance (FR-19, D-56):** Data with affirmatively verified open commercial licences is classified as `open-commercial`. Publicly downloadable data lacking an explicit open commercial grant, or governed by statutory code text, general website disclaimers, or restrictive terms, is classified as `research-only` (with raw files stored outside git in `~/energy-data/raw/` and tracked in [TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md)). Unverified licences are marked as `research-only / unverified`.
- **Internally Produced Artifacts:** Simulator schemas, timezone conversions, and interval engines are internally authored simulator specifications, not acquired external datasets, and are not assigned an external license.

### Auditable Candidate Source Register & Classification Breakdown
To ensure full auditability (Findings 18, R2-05), candidate sources are tracked at resource level with stable IDs (`SRC-01` through `SRC-50`, splitting mixed-licence or multi-stream families into distinct sub-resource rows `SRC-16a/b`, `SRC-36a/b`, `SRC-37a/b`, and `SRC-38a/b` for 54 total external candidate resources; plus `INT-01` and `INT-02` for internal specifications):

| Stable ID | Candidate Source Name | Primary Publisher | D-56 Classification | Access Requirement |
|---|---|---|---|---|
| `SRC-01` | ETYS 2025 Appendix B (System Technical Data) | NESO | `research-only` (PC-1) | Open direct download |
| `SRC-02` | ETYS Boundary GIS Data | NESO | `open-commercial` | Open CKAN API |
| `SRC-03` | NGED LTDS Tabular Model (4 areas) | NGED | `open-commercial` | Open direct download |
| `SRC-04` | NGED LTDS CIM XML Models | NGED | `open-commercial` | Open direct download |
| `SRC-05` | SSEN SEPD LTDS Tables & CIM EQ | SSEN Distribution | `open-commercial` | Open direct download (signed S3) |
| `SRC-06` | UKPN LTDS Tabular Circuit Data (3 areas) | UK Power Networks | `open-commercial` | Registered user login prerequisite (HTTP 403 unauthenticated) |
| `SRC-07` | UKPN LTDS CIM Network Models | UK Power Networks | `research-only` (PC-5) | User registration & formal CIM request prerequisite |
| `SRC-08` | SPEN SPM LTDS Tabular Data (1 area) | SP Energy Networks | `open-commercial` | Registered user login prerequisite (HTTP 403 unauthenticated) |
| `SRC-09` | SPEN SPM CIM Network Models | SP Energy Networks | `research-only` (PC-5) | User registration & Data Request prerequisite |
| `SRC-10` | NPg LTDS Tabular & CIM Models (2 areas) | Northern Powergrid | `open-commercial` | Registered user login prerequisite (HTTP 403 unauthenticated) |
| `SRC-11` | ENWL LTDS Tables & CIM Models (1 area) | Electricity North West | `research-only` (PC-5) | Secure Area user registration & approval prerequisite |
| `SRC-12` | TYTFS 2024 Study Files (WP/SV RAW/SEQ) | EirGrid & SONI | `research-only` (PC-2) | Open direct download |
| `SRC-13` | NIE Networks Open Data Portal | NIE Networks | `open-commercial` | Open CKAN API |
| `SRC-14` | ESB Networks Network Maps & Publications | ESB Networks | `research-only` (PC-5) | Web access only (no machine-readable model) |
| `SRC-15` | Elexon BMRS Reference Interconnectors | Elexon Limited | `open-commercial` | Open REST API |
| `SRC-16a` | ENTSO-E Physical Flows (A11, non-IFA/Nemo) | ENTSO-E | `open-commercial` | Personal API Security Token prerequisite |
| `SRC-16b` | ENTSO-E Scheduled Exchanges (A09) & IFA/Nemo Flows (A11) | ENTSO-E | `research-only` (PC-5) | Personal API Security Token prerequisite |
| `SRC-17` | NESO Transmission Entry Capacity (TEC) Register | NESO | `open-commercial` | Open CKAN API (cached Nov 10 hash pinned; live URL mutable) |
| `SRC-18` | DESNZ Renewable Energy Planning Database (REPD) | DESNZ | `open-commercial` | Open direct download |
| `SRC-19` | DESNZ DUKES Table 5.11 (Power Stations in the UK) | DESNZ | `open-commercial` | Open direct download |
| `SRC-20` | Elexon BM Unit Reference Register (`bmunits`) | Elexon Limited | `open-commercial` | Open REST API |
| `SRC-21` | SEMO Registered Capacity & Daily Registered Units | SEMO | `research-only` (PC-3) | Open web/reports portal (live daily returns empty error) |
| `SRC-22` | NESO Day-Ahead Constraints Limits & Flows | NESO | `open-commercial` | Open CKAN API |
| `SRC-23` | NESO 24-Months-Ahead Constraint Limits | NESO | `open-commercial` | Open CKAN API |
| `SRC-24` | EirGrid / SONI System Data Quarter-Hourly (SNSP) | EirGrid & SONI | `research-only` (PC-2) | Open direct download |
| `SRC-25` | NESO Operability Strategy (Inertia & Voltage) | NESO | `research-only` (PC-5) | Web access (URL unresolved / 404) |
| `SRC-26` | Balancing and Settlement Code (BSC) Baseline Text | Elexon Limited | `research-only` (PC-5) | Open web access (statutory code hub) |
| `SRC-27` | I-SEM Trading & Settlement Code (Part B) | SEMO | `research-only` (PC-5) | Web access (URL unresolved / 404) |
| `SRC-28` | JAO Capacity Allocation Rules | JAO S.A. | `research-only` (PC-5) | Open web access |
| `SRC-29` | NESO Historic Demand Data (2024) | NESO | `open-commercial` | Open CKAN API |
| `SRC-30` | Elexon Initial Demand Outturn (`demand_outturn`) | Elexon Limited | `open-commercial` | Open REST API |
| `SRC-31` | Elexon Half-Hourly Fuel Outturn (`fuelhh`) | Elexon Limited | `open-commercial` | Open REST API |
| `SRC-32` | Interconnector Technical Loss Specifications | Interconnector Operators | `research-only` (PC-5) | Public filings / operator sites (parameters MISSING) |
| `SRC-33` | GOV.UK Bank Holidays JSON | UK Cabinet Office | `open-commercial` | Open REST API |
| `SRC-34` | Irish Public Holiday Text | Government of Ireland | `research-only / unverified` (PC-5) | Web access (HTTP 403 unverified) |
| `SRC-35` | IANA Time Zone Database (`tzdata`) | IANA | `open-commercial` | Standard system library / open download |
| `SRC-36a` | UK ETS Auction Clearing Prices | ICE Futures Europe / UK ETS Authority | `research-only / unverified` (PC-5) | Web download (URL unresolved / 404; rights unverified) |
| `SRC-36b` | EU ETS Auction & Spot Prices | EEX / European Commission | `research-only / unverified` (PC-5) | Web download / unacquired |
| `SRC-37a` | DESNZ Fossil Fuel Price Assumptions | DESNZ | `open-commercial` | Open direct download (OGL v3.0) |
| `SRC-37b` | World Bank Commodity Price Data (Pink Sheet) | World Bank | `open-commercial` | Open direct download (CC BY 4.0; monthly) |
| `SRC-38a` | CfD Strike Prices | Low Carbon Contracts Company | `research-only / unverified` (PC-5) | Web download (portal HTTP 403 unverified) |
| `SRC-38b` | Renewables Obligation (RO) Buyout Price & Parameters | Ofgem | `research-only / unverified` (PC-5) | Administrative notices (dataset location unidentified) |
| `SRC-39` | Copernicus CDS ERA5 Reanalysis (Single Levels) | ECMWF / Copernicus | `open-commercial` | Registered CDS user account & API key prerequisite |
| `SRC-40` | Met Office MIDAS Open Surface Observations | UK Met Office / CEDA | `open-commercial` | Free CEDA user registration prerequisite |
| `SRC-41` | Met Éireann MÉRA Reanalysis & Open Data | Met Éireann | `research-only / unverified` (PC-5) | Web download (MÉRA ends 2019; GRIB 1) |
| `SRC-42` | Renewables.ninja Simulated Benchmark Series | Renewables.ninja | `research-only` (PC-5) | Free user registration & API token prerequisite |
| `SRC-43` | Elexon BMRS 2024 Market & Operational Streams | Elexon Limited | `open-commercial` | Open REST API |
| `SRC-44` | SEMO / SEMOpx 2024 Pinned Price Archives | SEMO & SEMOpx | `research-only` (PC-3) | Open direct download |
| `SRC-45` | NESO Outturn Thermal Constraint Costs | NESO | `open-commercial` | Open CKAN API |
| `SRC-46` | ECB Euro Foreign Exchange Reference Rates | European Central Bank | `open-commercial` | Open direct download |
| `SRC-47` | Bank of England Database Series | Bank of England | `research-only` (PC-5) | Open direct download |
| `SRC-48` | NESO ETYS Appendix G (GSP Peak Demand Snapshots) | NESO | `research-only` (PC-1) | Open direct download |
| `SRC-49` | DNO LTDS Table 7 (GSP Substation Demands) | DNOs (14 licence areas) | `research-only / unverified` (PC-5) | Portal logins / data requests (varies by DNO) |
| `SRC-50` | Elexon BMRS Settlement Stack API (Candidate TLM) | Elexon Limited | `open-commercial` | Open REST API candidate |
| `INT-01` | Canonical Scenario Schema & Checks Specification | Internal Simulator | N/A (Internal Code/Schema) | Internal repository specification |
| `INT-02` | ISO 8601 Continuous Interval Engine | Internal Simulator | N/A (Mathematical Algorithm) | Internal repository code |

### Classification Totals (External Candidate Resources `SRC-01`–`SRC-50`, 54 Resource Rows):
- **Total External Candidate Resources:** 54 distinct resource entries mapped to 31 normalised publishers.
- **Classification Breakdown:**
  - `open-commercial`: 29 resources (53.7%)
  - `research-only` (including unverified grants): 25 resources (46.3%)
- **Internal Simulator Specifications:** 2 internal artifacts (`INT-01`, `INT-02`), not classified as acquired external datasets.

### Normalised Publisher Mapping (31 Publishers):
To ensure full auditability and eliminate ambiguous multi-owner labels (Finding R2-05), the 54 external candidate resources are mapped to 31 distinct normalised publisher entities:
1. **National Energy System Operator (NESO):** 9 resources (`SRC-01`, `SRC-02`, `SRC-17`, `SRC-22`, `SRC-23`, `SRC-25`, `SRC-29`, `SRC-45`, `SRC-48`)
2. **National Grid Electricity Distribution (NGED):** 2 resources (`SRC-03`, `SRC-04`)
3. **Scottish and Southern Electricity Networks (SSEN Distribution):** 1 resource (`SRC-05`)
4. **UK Power Networks (UKPN):** 2 resources (`SRC-06`, `SRC-07`)
5. **SP Energy Networks (SPEN):** 2 resources (`SRC-08`, `SRC-09`)
6. **Northern Powergrid (NPg):** 1 resource (`SRC-10`)
7. **Electricity North West Limited (ENWL):** 1 resource (`SRC-11`)
8. **EirGrid & SONI:** 2 resources (`SRC-12`, `SRC-24`)
9. **Northern Ireland Electricity Networks (NIE Networks):** 1 resource (`SRC-13`)
10. **ESB Networks:** 1 resource (`SRC-14`)
11. **Elexon Limited:** 7 resources (`SRC-15`, `SRC-20`, `SRC-26`, `SRC-30`, `SRC-31`, `SRC-43`, `SRC-50`)
12. **ENTSO-E:** 2 resources (`SRC-16a`, `SRC-16b`)
13. **Department for Energy Security and Net Zero (DESNZ):** 3 resources (`SRC-18`, `SRC-19`, `SRC-37a`)
14. **Single Electricity Market Operator (SEMO):** 2 resources (`SRC-21`, `SRC-27`)
15. **SEMO & SEMOpx:** 1 resource (`SRC-44`)
16. **Joint Allocation Office (JAO S.A.):** 1 resource (`SRC-28`)
17. **Interconnector Operators (BritNed, ElecLink, Greenlink, Moyle, etc.):** 1 resource (`SRC-32`)
18. **UK Cabinet Office:** 1 resource (`SRC-33`)
19. **Government of Ireland:** 1 resource (`SRC-34`)
20. **Internet Assigned Numbers Authority (IANA):** 1 resource (`SRC-35`)
21. **ICE Futures Europe / UK ETS Authority:** 1 resource (`SRC-36a`)
22. **European Energy Exchange (EEX) / European Commission:** 1 resource (`SRC-36b`)
23. **World Bank:** 1 resource (`SRC-37b`)
24. **Low Carbon Contracts Company (LCCC):** 1 resource (`SRC-38a`)
25. **Office of Gas and Electricity Markets (Ofgem):** 1 resource (`SRC-38b`)
26. **ECMWF / Copernicus Climate Change Service:** 1 resource (`SRC-39`)
27. **UK Met Office / CEDA Archive:** 1 resource (`SRC-40`)
28. **Met Éireann:** 1 resource (`SRC-41`)
29. **Renewables.ninja (Imperial / ETH):** 1 resource (`SRC-42`)
30. **European Central Bank (ECB):** 1 resource (`SRC-46`)
31. **Bank of England (BoE):** 1 resource (`SRC-47`)
32. *(DNO licence areas collective candidate):* 1 resource (`SRC-49`)

### Field-Level Deficit Backlog
The claim of "zero unsourced inputs" is retracted (Finding 18). While candidate source families have been identified for high-level categories, an explicit field-level deficit backlog remains missing or unverified before M1 ingestion can be considered complete:
1. **Commissioned 2024 Network Fleet & Asset Denominators (Finding 7):** ETYS 2025/26 and DNO 2025/26 releases are forward snapshots, not 2024 historical commissioned baselines. GSP and BMU crosswalks, rating definitions (post-fault vs continuous), unit bases (% vs pu on 100 MVA vs ohms), and 132-kV circuit denominators across all 14 DNO areas remain incomplete.
2. **Proxy Rule Parameters and Cut-Sets (Finding 6):** Boundary constraint groups lack explicit signed electrical branch cut-sets. Regional inertia floors, unit groups, minimum-unit requirements, and Irish SNSP operational limits are unverified narrative figures or realised outturns rather than verified operational rules.
3. **Embedded Generation Decomposition & Demand Equation (Findings 3, R2-02):** Elexon FUELHH does not provide embedded solar or wind output. Embedded estimates must be sourced from NESO historic demand columns (`EMBEDDED_SOLAR_GENERATION`, `EMBEDDED_WIND_GENERATION`); Irish embedded components remain unacquired. In NESO Historic Demand data, National Demand (`ND`) already excludes station load, pumped storage pumping, and interconnector exports (which Transmission System Demand, `TSD`, adds back). Therefore, pumping must not be subtracted from `ND`. The starting accounting identity is $\text{Gross Underlying Demand} = \text{National Demand} (\text{ND}) + \text{Embedded Solar} + \text{Embedded Wind}$. Double-counting checks, station-load reconciliation, and MWh integration fixtures remain pending.
4. **Directional Interconnector Losses & TLM Source (Findings 4, 5, R2-03):** Directional physical loss parameters for all 10 interconnector links are missing from operator sources; unsupported numeric default percentages have been replaced with MISSING across all links. Converter and cable loss separation, distinct forward/reverse flow parameters, and sending/receiving metering conventions remain unverified pending primary regulatory filings. The claimed Elexon TLM dataset is not present in the 12 gathered settlement streams and was not acquired; Elexon's settlement stack API (`SRC-50`) remains an unacquired candidate.
5. **Dated Market Rules (Finding 11):** BSC Section T settlement cashout provisions and SEMOpx auction rules (DAM, IDA1–3, IDC) need dated, version-pinned 2024 releases rather than current website legal hubs.
6. **Fuel, Carbon, and Subsidy Vectors (Findings 10, R2-01, R2-05):** UK ETS auction URL is dead (404), licence is unverified (PC-5), and third-party auction rights are not established; EU ETS source is unacquired; World Bank series are monthly (resampling required); DESNZ assumptions are forward projections; LCCC portal is gated (403); Ofgem RO buyout parameters lack an identified machine-readable dataset; technology heat rates and variable O&M are unacquired.
7. **Weather Reanalysis Acquisition (Findings 14, 16):** Zero bytes of ERA5 weather data have been downloaded (requires owner CDS credentials); MÉRA does not cover 2024; power-curve conversion software is unintegrated.
8. **Statutory and Market Calendars (Finding 13):** Irish public holiday dataset access is unverified (403); trading settlement calendars and tzdata versions must be pinned to immutable repository artifacts.
9. **Scenario Schema, Identity Inputs & Fixtures (Findings 15, R2-07):** The canonical scenario schema exists only as a conceptual architecture in ADR 0002. A complete scenario input inventory requires: separate hashing of scenario input identity and execution identity; knowledge mode distinction (as-of forecast backtest vs historical reconstruction); typed domain entities with permanent opaque UUIDs; three distinct timestamps (effective time `[from, to)`, publication/revision time, and acquisition time); versioned mappings for calendar/tzdata, transformations, cost models, FX vectors, and constraint proxies; rights lineage tracking with PC-1–PC-7 clearance tags; and schema migration vectors with deterministic manifest hashes. The Rust serde deserializer, JSON-schema validation, immutable locks, and test fixtures remain to be implemented in M1.

---

## 2. Detailed Data Inventory by M1 Input Category

### Category 1: Dated Topology and Network Assets

#### 1.1 GB Transmission Network (400 kV, 275 kV, Scottish 132 kV)
- **Candidate Source (`SRC-01`):** NESO Electricity Ten Year Statement (ETYS) 2025 Appendix B (System Technical Data)
  - **Publisher:** National Energy System Operator (NESO)
  - **URL:** [https://www.neso.energy/document/383936/download](https://www.neso.energy/document/383936/download)
  - **Format:** Excel Workbook (`.xlsx`), 31 worksheets
  - **Coverage:** GB transmission network: Scottish Hydro Electric Transmission (SHE Transmission), SP Transmission (SPT), National Grid Electricity Transmission (NGET), and Offshore Transmission Owners (OFTOs). Voltages: 400 kV, 275 kV, 132 kV (Scotland and selected English/Welsh assets). Base year: 2025/26 with future change sheets 2026/27–2033/34.
  - **Update Cadence:** Annual (typically published Q4)
  - **Licence Name & Link:** No dataset-specific open licence found; governed by [NESO Terms and Conditions](https://www.neso.energy/terms-and-conditions)
  - **D-56 Classification:** `research-only` ([PC-1 in TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md))
  - **Access Requirement:** Open direct download (no credentials required)
  - **Evidence State:** Raw acquired / hash-verified in cache (`spikes/data-gathering/network-ie/manifest.toml`); content audited in [spikes/data/REPORT.md](../../spikes/data/REPORT.md) (1,952 endpoint IDs, 1,392 circuit rows, 1,472 transformer rows).
  - **Remaining Gaps:** Unverified commercial grant ([PC-1](../requirements/TODO-pre-commercial.md)); ratings are labelled post-fault without continuous/emergency distinction; represents 2025/26 base rather than historical 2024 commissioned state; lacks geographic coordinates and tap models; does not cover England/Wales DNO 132 kV.

- **Candidate Source (`SRC-02`):** NESO ETYS Boundary GIS Data
  - **Publisher:** National Energy System Operator (NESO)
  - **URL:** Package: [https://api.neso.energy/api/3/action/package_show?id=997f4820-1ad4-499b-b1fe-4b8d3d7fbc72](https://api.neso.energy/api/3/action/package_show?id=997f4820-1ad4-499b-b1fe-4b8d3d7fbc72); Direct Resource: [https://api.neso.energy/dataset/997f4820-1ad4-499b-b1fe-4b8d3d7fbc72/resource/e914fcec-1dc9-4f1f-97e7-59c0d9521bea/download/etys-boundary-gis-data-mar25.zip](https://api.neso.energy/dataset/997f4820-1ad4-499b-b1fe-4b8d3d7fbc72/resource/e914fcec-1dc9-4f1f-97e7-59c0d9521bea/download/etys-boundary-gis-data-mar25.zip)
  - **Format:** Shapefile bundle (`.shp`, `.shx`, `.dbf`, `.prj`) inside ZIP
  - **Coverage:** 34 transmission system boundary features across GB
  - **Update Cadence:** Annual
  - **Licence Name & Link:** [National Energy SO Open Data Licence v1.0](https://www.neso.energy/data-portal/neso-open-licence)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open CKAN REST API / direct download
  - **Evidence State:** Raw acquired / hash-verified in cache (`spikes/data-gathering/network-ie/manifest.toml`).
  - **Remaining Gaps:** Provides boundary line geometries only; does not provide electrical branch cut-sets, individual circuit ratings, or substation coordinates.

#### 1.2 GB Distribution 132 kV Network (All 14 DNO Licence Areas, D-60)
Per D-60, all 14 DNO areas must be addressed. Note that there are **12 England/Wales DNO licence areas** (where 132 kV is distribution) and **2 Scottish licence areas** (where 132 kV is transmission under SHET/SPT and distribution operates at 33 kV and below, requiring coverage tracking under D-60). Full inventory verified in [dno-132kv-inventory.md](dno-132kv-inventory.md):

- **Candidate Source (`SRC-03`):** National Grid Electricity Distribution (NGED) LTDS Tabular Model
  - **Publisher:** NGED (East Midlands, West Midlands, South Wales, South West)
  - **URL:** [https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model](https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model)
  - **Format:** CSV (Table 1 Circuits, Table 2a 2W Transformers, Table 2b 3W Transformers, Attachment 8 Nodes)
  - **Coverage:** 4 England/Wales licence areas (EMIDS, WMIDS, SWALES, SWEST). Table 1 has 4,183 132-kV rows (1,138 circuits: 907 unchanged baseline, 53 updated, 115 new, 63 removed; plus switchgear).
  - **Update Cadence:** Annual / semi-annual (edition dates December 2025 / July 2026)
  - **Licence Name & Link:** [NGED Open Data Licence](https://www.nationalgrid.co.uk/open-data-licence)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download (no authentication required)
  - **Evidence State:** Raw acquired / hash-verified in cache (`spikes/data-gathering/network-ie/manifest.toml`); sample in `spikes/data/samples/`.
  - **Remaining Gaps:** 3,045 rows lack impedance/ratings (switchgear rows); unit bases for R/X/B (% on 100 MVA vs ohms) and ratings (MVA vs A) require explicit LTDS specification resolution; node IDs must be mapped to ETYS GSPs.

- **Candidate Source (`SRC-04`):** NGED LTDS Common Information Model (CIM)
  - **Publisher:** NGED
  - **URL:** [https://connecteddata.nationalgrid.co.uk/dataset/ltds-common-information-model](https://connecteddata.nationalgrid.co.uk/dataset/ltds-common-information-model)
  - **Format:** CIM XML / RDF archives in ZIP (EQ, SC, SCR, SYSCAP profiles)
  - **Coverage:** 4 NGED areas; detailed electrical network topologies
  - **Update Cadence:** Annual
  - **Licence Name & Link:** [NGED Open Data Licence](https://www.nationalgrid.co.uk/open-data-licence)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download
  - **Evidence State:** Raw acquired / hash-verified in cache (`spikes/data-gathering/network-ie/manifest.toml`).
  - **Remaining Gaps:** Complex nested CIM profiles require parsing into the canonical network model; operational switching states must be reconciled.

- **Candidate Source (`SRC-05`):** SSEN Distribution Long Term Development Statements (SEPD & SHEPD)
  - **Publisher:** Scottish and Southern Electricity Networks (SSEN)
  - **URL:** [https://data.ssen.co.uk/@ssen-distribution/sepd_long_term_development_statement](https://data.ssen.co.uk/@ssen-distribution/sepd_long_term_development_statement)
  - **Format:** Excel Workbook (`.xlsx`) Tables 1–7 and CIM XML ZIP
  - **Coverage:** Southern Electric Power Distribution (SEPD, 1 England/Wales licence area, 132 kV circuits and transformers) and Scottish Hydro Electric Power Distribution (SHEPD, 1 Scottish licence area; distribution 33/11 kV, with Scottish 132 kV under SHET transmission). Table 1 has 435 132-kV circuit rows and 218 132-kV transformers for SEPD.
  - **Update Cadence:** Annual (May 2026 edition)
  - **Licence Name & Link:** [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download (transient signed S3 URLs generated without login)
  - **Evidence State:** Raw acquired / hash-verified in cache (`spikes/data-gathering/network-ie/manifest.toml`).
  - **Remaining Gaps:** Substation geographic coordinates unverified in tables; joining SEPD GSPs with NGET transmission buses.

- **Candidate Source (`SRC-06`):** UK Power Networks (UKPN) LTDS Table 1–4 Tabular Data
  - **Publisher:** UK Power Networks (EPN, LPN, SPN — 3 England/Wales licence areas)
  - **URL:** Table 1 Circuit: [https://ukpowernetworks.opendatasoft.com/explore/assets/ukpn-ltds-table-1-circuit-data/](https://ukpowernetworks.opendatasoft.com/explore/assets/ukpn-ltds-table-1-circuit-data/)
  - **Format:** Tabular (CSV/Parquet via Opendatasoft API)
  - **Coverage:** Eastern Power Networks (EPN), London Power Networks (LPN), South Eastern Power Networks (SPN). 132 kV circuits, transformers, and grid sites (catalogue lists 5,076 records across all voltage levels; 132 kV circuit subset unmeasured).
  - **Update Cadence:** Annual (May 2026 release)
  - **Licence Name & Link:** Catalogue metadata identifies [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/)
  - **D-56 Classification:** `open-commercial` (per catalogue grant; login gating does not alter copyright licence)
  - **Access Requirement:** **Registered user login required**. Unauthenticated row queries return HTTP 403 ForbiddenAccess.
  - **Evidence State:** Metadata inspected and verified via Opendatasoft API; zero row payloads downloaded.
  - **Remaining Gaps:** Owner registration required to extract records; impedance unit bases (% on 100 MVA vs ohms) unverified until row access.

- **Candidate Source (`SRC-07`):** UKPN LTDS CIM Network Models
  - **Publisher:** UK Power Networks
  - **URL:** [https://ukpowernetworks.opendatasoft.com/explore/assets/ukpn-ltds-cim/](https://ukpowernetworks.opendatasoft.com/explore/assets/ukpn-ltds-cim/)
  - **Format:** CIM XML
  - **Coverage:** EPN, LPN, SPN CIM network model deliverables
  - **Licence Name & Link:** UK Power Networks Shared Data Licence - Common Information Model
  - **D-56 Classification:** `research-only` (restrictive terms, PC-5)
  - **Access Requirement:** Registered user login and formal CIM Access Request Form approval. Form redirects to login.
  - **Evidence State:** Metadata inspected; access unverified.
  - **Remaining Gaps:** Restricted commercial reuse terms require clearance under PC-5; SharePoint distribution gated.

- **Candidate Source (`SRC-08`):** SP Energy Networks (SPEN) LTDS Tabular Data (SP Manweb)
  - **Publisher:** SP Energy Networks (SPM — 1 England/Wales licence area)
  - **URL:** SPM Table 1: [https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-appendix-1-circuit-data-table-1/](https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-appendix-1-circuit-data-table-1/)
  - **Format:** Tabular (CSV/JSON/Parquet)
  - **Coverage:** SP Manweb (SPM, England/Wales 132 kV network, 1,798 catalogue records across all voltage classes). Note: SP Distribution (SPD) is Scottish distribution; its 132 kV is transmission under SPT.
  - **Update Cadence:** Annual (November 2025 release)
  - **Licence Name & Link:** Catalogue metadata identifies [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/)
  - **D-56 Classification:** `open-commercial` (per catalogue grant)
  - **Access Requirement:** **Registered user login required**. Unauthenticated record requests return HTTP 403 ForbiddenAccess.
  - **Evidence State:** Metadata inspected; zero row payloads downloaded.
  - **Remaining Gaps:** Row extraction gated behind account; SPM R/X/B units labelled as % on 100 MVA while SPD labels as pu on 100 MVA (not interchangeable).

- **Candidate Source (`SRC-09`):** SPEN SPM CIM Network Models
  - **Publisher:** SP Energy Networks
  - **URL:** SPM CIM: [https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-cim-eq-profile/](https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-cim-eq-profile/)
  - **Format:** CIM XML ZIP
  - **Licence Name & Link:** ENA Shared Data Licence hosted on [SPEN terms](https://spenergynetworks.opendatasoft.com/p/sp-energy-networks-shared-data-licence/)
  - **D-56 Classification:** `research-only` (restrictive terms, PC-5)
  - **Access Requirement:** Registered login & formal Data Request form. Cited form redirects to asset catalogue.
  - **Evidence State:** Metadata inspected; access unverified.
  - **Remaining Gaps:** Restricted terms require PC-5 clearance; actual Data Request form URL must be resolved.

- **Candidate Source (`SRC-10`):** Northern Powergrid (NPg) LTDS Tabular & CIM Models
  - **Publisher:** Northern Powergrid (Northeast & Yorkshire — 2 England/Wales licence areas)
  - **URL:** Appendix 3 Circuit: [https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-appendix-3/information/](https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-appendix-3/information/); CIM: [https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-cim-files/information/](https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-cim-files/information/)
  - **Format:** Tabular (CSV/Parquet) and CIM XML
  - **Coverage:** NPg Northeast and Yorkshire (1,670 catalogue records across all voltage classes).
  - **Update Cadence:** Annual (November 2025 release)
  - **Licence Name & Link:** [Northern Powergrid Open Data Licence v1.0](https://northernpowergrid.opendatasoft.com/p/opendatalicence/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** **Registered user login required**. Unauthenticated record requests return HTTP 403 ForbiddenAccess.
  - **Evidence State:** Metadata inspected; zero row payloads downloaded.
  - **Remaining Gaps:** Requires owner account to export records; 132-kV subset denominator unverified until rows downloaded.

- **Candidate Source (`SRC-11`):** Electricity North West (ENWL) LTDS
  - **Publisher:** Electricity North West Limited (1 England/Wales licence area)
  - **URL:** [https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/login-to-ltds-secure-area/](https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/login-to-ltds-secure-area/)
  - **Format:** Excel Workbook, CIM XML, and PDF diagrams
  - **Coverage:** North West England 132 kV circuits and transformers.
  - **Update Cadence:** Annual (November 2025 revision)
  - **Licence Name & Link:** Electricity North West Shared Data Licence ([SP ENW Shared Licence](https://electricitynorthwest.huwise.com/p/sp-enw-shared-licence/))
  - **D-56 Classification:** `research-only` (restrictive terms, PC-5)
  - **Access Requirement:** **Secure Area login required**. Mandatory registration form.
  - **Evidence State:** Portal structure inspected; zero payloads downloaded.
  - **Remaining Gaps:** Restrictive clause 1.1 (internal business use) and 3.1(b) (no modification/distribution without consent) require pre-commercial clearance ([PC-5 in TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md)).

#### 1.3 All-Island Ireland Transmission Network (Ireland 400/220/110 kV, NI 275/110 kV)
- **Candidate Source (`SRC-12`):** EirGrid / SONI All-Island Ten Year Transmission Forecast Statement (TYTFS) 2024 Study Files
  - **Publisher:** EirGrid Group & SONI Ltd
  - **URL:** [https://cms.eirgrid.ie/sites/default/files/publications/TYTFS2024_studyfiles.zip](https://cms.eirgrid.ie/sites/default/files/publications/TYTFS2024_studyfiles.zip)
  - **Format:** PSS/E RAW (`.raw`) and Sequence (`.seq`) cases (v33 and v35), 17 files in ZIP
  - **Coverage:** All-island transmission grid: Ireland (400/380 kV, 220 kV, 110 kV) and Northern Ireland (275 kV, 110 kV). Models Summer Valley (SV) and Winter Peak (WP) cases for 2024 and 2033. Inspected WP2024 case has 2,025 buses (547 transmission-voltage buses), 1,129 AC branch records, 1,310 transformer records, and 597 generator attachments.
  - **Update Cadence:** Annual (published November 2025)
  - **Licence Name & Link:** No dataset-specific open licence found; governed by [EirGrid Website Disclaimer](https://www.eirgrid.ie/disclaimer) and TYTFS PDF copyright notice (all rights reserved)
  - **D-56 Classification:** `research-only` ([PC-2 in TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md))
  - **Access Requirement:** Open direct download (no credentials required)
  - **Evidence State:** Raw acquired / hash-verified in cache (`spikes/data-gathering/network-ie/manifest.toml`); topology parsed and audited in [spikes/data/REPORT.md](../../spikes/data/REPORT.md).
  - **Remaining Gaps:** Commercial clearance required ([PC-2](../requirements/TODO-pre-commercial.md)); cases represent forecast study snapshots, not operational operating-history evidence; 132 zero-rating branches require operator definition; 3-winding transformers need star conversion; nominal 400 kV assets are modelled on a 380 kV base; generator IDs are model records, not commercial market participant IDs.

- **Candidate Source (`SRC-13`):** NIE Networks Open Data Portal (Northern Ireland)
  - **Publisher:** Northern Ireland Electricity Networks
  - **URL:** [https://nienetworks.opendatasoft.com/pages/home/](https://nienetworks.opendatasoft.com/pages/home/)
  - **Format:** GeoJSON, Shapefile, CSV
  - **Coverage:** Northern Ireland substations, transformers, switchgear, distribution feeders (20 listed datasets).
  - **Update Cadence:** Periodic
  - **Licence Name & Link:** Catalogue metadata identifies [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/); OSNI basemap features carry OGL notices.
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open CKAN/Opendatasoft API
  - **Evidence State:** Metadata inspected; documented in [data-sources-ie-weather.md](data-sources-ie-weather.md).
  - **Remaining Gaps:** Full branch impedance model completeness for all feeders unverified.

- **Candidate Source (`SRC-14`):** ESB Networks Network Maps & Publications (Republic of Ireland)
  - **Publisher:** ESB Networks
  - **URL:** [https://www.esbnetworks.ie/](https://www.esbnetworks.ie/)
  - **Format:** PDF schematics, interactive GIS viewers
  - **Coverage:** Republic of Ireland distribution substations and 110/38 kV interfaces
  - **Update Cadence:** Irregular
  - **Licence Name & Link:** All rights reserved ([ESB Copyright Policy](https://www.esbnetworks.ie/data-legal/copyright))
  - **D-56 Classification:** `research-only` (PC-5; no machine-readable open dataset verified)
  - **Access Requirement:** Web access; no open machine-readable parameter feed verified (data.gov.ie query returned zero datasets)
  - **Evidence State:** Source identified; no payload acquired.
  - **Remaining Gaps:** Lack of machine-readable distribution network model; fallback is D-37 synthetic feeders for distribution interfaces.

#### 1.4 Cross-Border Interconnectors & External Boundaries (FR-1, FR-1.AC3)
- **Candidate Source (`SRC-15`):** Elexon BMRS Reference Interconnector Endpoints
  - **Publisher:** Elexon Limited
  - **URL:** [https://data.elexon.co.uk/bmrs/api/v1/reference/interconnectors/all](https://data.elexon.co.uk/bmrs/api/v1/reference/interconnectors/all)
  - **Format:** JSON REST API
  - **Coverage:** Reference IDs for all 10 active GB cross-border interconnector endpoints:
    - *7 external links:* France: IFA (`INTFR`), IFA2 (`INTIFA2`), ElecLink (`INTELEC`); Netherlands: BritNed (`INTNED`); Belgium: Nemo Link (`INTNEM`); Norway: North Sea Link (`INTNSL`); Denmark: Viking Link (`INTVKL`).
    - *3 internal GB-to-Ireland links:* Northern Ireland: Moyle (`INTIRL`); Republic of Ireland: East–West / EWIC (`INTEW`), Greenlink (`INTGRNL`).
  - **Update Cadence:** Static reference register
  - **Licence Name & Link:** [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open REST API (no credentials required)
  - **Evidence State:** Source identified / reference endpoint verified (flow time-series acquired in `spikes/data-gathering/elexon/COVERAGE.md`, but static reference snapshot payload is not hash-verified in cache).
  - **Remaining Gaps:** Reference list does not provide historical commissioning dates; Greenlink recorded 7,102 sparse observations in 2024 (cause unverified; not a fully commissioned 2024 baseline).

- **Candidate Sources (`SRC-16a`, `SRC-16b`):** ENTSO-E Transparency Platform (Scheduled Exchanges & Physical Flows)
  - **Publisher:** ENTSO-E
  - **URL:** Platform: [https://transparency.entsoe.eu/](https://transparency.entsoe.eu/); Official Terms API: [https://transparencyplatform.zendesk.com/api/v2/help_center/en-us/articles/40921911218961.json](https://transparencyplatform.zendesk.com/api/v2/help_center/en-us/articles/40921911218961.json); Reuse List PDF: [https://transparencyplatform.zendesk.com/hc/article_attachments/40921869379729](https://transparencyplatform.zendesk.com/hc/article_attachments/40921869379729)
  - **Format:** XML REST API (Document types `A09` scheduled exchanges, `A11` physical flows)
  - **Coverage:** Cross-border exchanges across all European borders, including GB↔France, GB↔Belgium, GB↔Netherlands, GB↔Norway, GB↔Denmark, GB↔IE/NI.
  - **Update Cadence:** Hourly / Half-hourly
  - **Licence Name & Link:** [ENTSO-E Free Re-use List (18 Oct 2023)](https://transparencyplatform.zendesk.com/hc/article_attachments/40921869379729) incorporates CC BY 4.0 for physical flows (`A11`), but **explicitly excludes IFA and Nemo Link**, and does not list scheduled commercial exchanges (`A09`).
  - **D-56 Classification:** `SRC-16a` (`A11` physical flows on non-excluded borders): `open-commercial`; `SRC-16b` (`A11` on IFA and Nemo Link, and all `A09` scheduled commercial exchanges): `research-only` ([PC-5 in TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md)).
  - **Access Requirement:** **Owner credentials / security token required**. REST API requires personal token obtained via account registration and email request to `transparency@entsoe.eu` (prerequisite; zero authenticated payloads retrieved).
  - **Evidence State:** Terms and API structure verified; zero authenticated payloads retrieved (unauthenticated requests return HTTP 401).
  - **Remaining Gaps:** Owner token must be provisioned to unlock automated validation downloads.

#### 1.5 Generation, Storage, and Resource Fleets
- **Candidate Source (`SRC-17`):** NESO Transmission Entry Capacity (TEC) Register
  - **Publisher:** NESO
  - **URL:** Package: [https://api.neso.energy/api/3/action/package_show?id=cbd45e54-e6e2-4a38-99f1-8de6fd96d7c1](https://api.neso.energy/api/3/action/package_show?id=cbd45e54-e6e2-4a38-99f1-8de6fd96d7c1); Direct Resource: [https://api.neso.energy/dataset/cbd45e54-e6e2-4a38-99f1-8de6fd96d7c1/resource/17becbab-e3e8-473f-b303-3806f43a6a10/download/tec-register-10-november-2026.csv](https://api.neso.energy/dataset/cbd45e54-e6e2-4a38-99f1-8de6fd96d7c1/resource/17becbab-e3e8-473f-b303-3806f43a6a10/download/tec-register-10-november-2026.csv) (Note: live resource URL is mutable and redirects to subsequent signed snapshots, e.g. 13 November 2026; only the cached sample is pinned by hash).
  - **Format:** CSV
  - **Coverage:** GB transmission-connected generation and storage projects (snapshot 10 November 2026: 2,220 project rows, 1,759 direct agreements, 461 embedded agreements, 389 marked Built/Operational).
  - **Update Cadence:** Monthly
  - **Licence Name & Link:** [National Energy SO Open Data Licence v1.0](https://www.neso.energy/data-portal/neso-open-licence)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download via CKAN API
  - **Evidence State:** Cached snapshot (10 November 2026) acquired and hash-verified in `spikes/data-gathering/network-ie/manifest.toml`. The live CKAN resource URL is mutable and does not constitute an immutable November 10 release lock or 2024 fleet baseline.
  - **Remaining Gaps:** Live resource is mutable and redirects to subsequent snapshots. Current snapshot includes future connection queues; filtering to historical 2024 operational fleet requires effective date cross-referencing and cannot be established by the register alone.

- **Candidate Source (`SRC-18`):** DESNZ Renewable Energy Planning Database (REPD)
  - **Publisher:** Department for Energy Security and Net Zero (DESNZ)
  - **URL:** [https://www.gov.uk/government/publications/renewable-energy-planning-database-monthly-extract](https://www.gov.uk/government/publications/renewable-energy-planning-database-monthly-extract)
  - **Format:** CSV / Excel
  - **Coverage:** UK renewable electricity and battery storage projects >150 kW (over 14,000 projects; 3,132 operational).
  - **Update Cadence:** Quarterly (with monthly publication extract)
  - **Licence Name & Link:** [Open Government Licence v3.0](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download
  - **Evidence State:** Content audited in [data-sources-gb.md](data-sources-gb.md) and [spikes/data/REPORT.md](../../spikes/data/REPORT.md).
  - **Remaining Gaps:** Planning snapshot, not electrical bus assignments; coordinates are British National Grid Easting/Northing; historical threshold changes (<1 MW before 2021).

- **Candidate Source (`SRC-19`):** DESNZ DUKES Table 5.11 (Power Stations in the United Kingdom)
  - **Publisher:** DESNZ
  - **URL:** [https://www.gov.uk/government/statistics/electricity-chapter-5-digest-of-united-kingdom-energy-statistics-dukes](https://www.gov.uk/government/statistics/electricity-chapter-5-digest-of-united-kingdom-energy-statistics-dukes)
  - **Format:** Excel Workbook (`.xlsx`)
  - **Coverage:** Major Power Producers operating in the UK (1,364 rows with site name, technology, fuel, capacity, commissioning year, connection type, coordinates).
  - **Update Cadence:** Annual (July release, end-May baseline)
  - **Licence Name & Link:** [Open Government Licence v3.0](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download
  - **Evidence State:** Content audited in [data-sources-gb.md](data-sources-gb.md).
  - **Remaining Gaps:** Covers Major Power Producers only; geographical details are self-reported; requires filtering Northern Ireland records for GB fleet.

- **Candidate Source (`SRC-20`):** Elexon BM Unit Reference Register (`bmunits`)
  - **Publisher:** Elexon Limited
  - **URL:** [https://data.elexon.co.uk/bmrs/api/v1/reference/bmunits/all?format=json](https://data.elexon.co.uk/bmrs/api/v1/reference/bmunits/all?format=json)
  - **Format:** JSON REST API
  - **Coverage:** All 3,134 registered GB Balancing Mechanism units (National Grid BMU ID, Elexon BMU ID, fuel type, generation/demand capacity, lead party, GSP group).
  - **Update Cadence:** Live snapshot
  - **Licence Name & Link:** [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download
  - **Evidence State:** Raw acquired / hash-verified in cache (`spikes/data-gathering/elexon/COVERAGE.md`).
  - **Remaining Gaps:** Endpoint provides a current reference snapshot, not a historical 2024 register; requires date-effective joining with historical settlement files to account for decommissioned or renamed units.

- **Candidate Source (`SRC-21`):** SEMO Registered Capacity & Daily Registered Units Reports
  - **Publisher:** Single Electricity Market Operator (SEMO)
  - **URL:** Monthly Capacity: [https://reports.sem-o.com/documents/PUB_MnlyRegisteredCapacity_202412011000.xml](https://reports.sem-o.com/documents/PUB_MnlyRegisteredCapacity_202412011000.xml); Daily Register: [https://reports.sem-o.com/documents/PUB_DailyRegisteredUnits_202609140734.xml](https://reports.sem-o.com/documents/PUB_DailyRegisteredUnits_202609140734.xml) (live request returns empty error envelope `{"errorMessage":"","code":0}`; cached capture exists); Reports directory: [https://reports.sem-o.com/documents/](https://reports.sem-o.com/documents/) (returns empty error envelope live).
  - **Format:** XML reports
  - **Coverage:** All-island SEM generating and demand-side units (702 distinct resources in monthly capacity report, 769 distinct units in daily register).
  - **Update Cadence:** Monthly / Daily
  - **Licence Name & Link:** [EirGrid Group Open Data Licence](https://www.smartgriddashboard.com/all/open-data-license/) names SEMO, but reports carry standard website disclaimers; express dataset designation unverified.
  - **D-56 Classification:** `research-only` ([PC-3 in TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md))
  - **Access Requirement:** Open direct download via public reports portal (live daily URL returns empty error envelope; local cached capture is hash-verified)
  - **Evidence State:** Raw acquired / hash-verified in cache (`spikes/data-gathering/network-ie/manifest.toml`).
  - **Remaining Gaps:** Commercial reuse permission unverified; participant ResourceNames must be mapped to electrical buses in the TYTFS network model.

---

### Category 2: Network Rules and Operational Constraints / Proxies

#### 2.1 Published Security, Voltage, and Stability Proxies (FR-21, D-55)
- **Candidate Source (`SRC-22`):** NESO Day-Ahead Constraints Limits and Flow Output
  - **Publisher:** NESO
  - **URL:** Package: [https://api.neso.energy/api/3/action/package_show?id=cf3cbc92-2d5d-4c2b-bd29-e11a21070b26](https://api.neso.energy/api/3/action/package_show?id=cf3cbc92-2d5d-4c2b-bd29-e11a21070b26); Direct Resource: [https://api.neso.energy/dataset/cf3cbc92-2d5d-4c2b-bd29-e11a21070b26/resource/38a18ec1-9e40-465d-93fb-301e80fd1352/download/day-ahead-constraints-limits-and-flow-output-v1.5.csv](https://api.neso.energy/dataset/cf3cbc92-2d5d-4c2b-bd29-e11a21070b26/resource/38a18ec1-9e40-465d-93fb-301e80fd1352/download/day-ahead-constraints-limits-and-flow-output-v1.5.csv)
  - **Format:** CSV (764,304 rows)
  - **Coverage:** Day-ahead constraint-group forecast limits (`Limit_MW`) and flows (`Flow_MW`) across 32 key transmission boundaries (2023-01-01 through 2026-11-13, half-hourly resolution).
  - **Update Cadence:** Daily
  - **Licence Name & Link:** [National Energy SO Open Data Licence v1.0](https://www.neso.energy/data-portal/neso-open-licence)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open CKAN API download
  - **Evidence State:** Raw acquired / hash-verified in cache (`spikes/data-gathering/network-ie/manifest.toml`, 29.27 MB).
  - **Remaining Gaps (Finding 6, 12):** 
    - *Clock handling:* Timestamps are recorded in local `GMT_BST`. Source interval identifiers, offset/fold, and revisions must be preserved when mapping to UTC. Repeated local times at the autumn fall-back transition represent distinct physical intervals and **must not be discarded or deduplicated** (Finding 12).
    - *Missing cut-sets:* The file provides limits/flows for named constraint groups, but does **not** provide the electrical branch cut-sets required to apply these constraints to the network model.

- **Candidate Source (`SRC-23`):** NESO 24-Months-Ahead Constraint Limits
  - **Publisher:** NESO
  - **URL:** Package: [https://api.neso.energy/api/3/action/package_show?id=d515b4a9-60a1-489c-a126-004efc04f121](https://api.neso.energy/api/3/action/package_show?id=d515b4a9-60a1-489c-a126-004efc04f121); Direct Resource: [https://api.neso.energy/dataset/d515b4a9-60a1-489c-a126-004efc04f121/resource/3c359e33-3dac-4bdd-87d1-efbf4cbc2f07/download/24-months-ahead-constraint-limit_nov26.csv](https://api.neso.energy/dataset/d515b4a9-60a1-489c-a126-004efc04f121/resource/3c359e33-3dac-4bdd-87d1-efbf4cbc2f07/download/24-months-ahead-constraint-limit_nov26.csv)
  - **Format:** CSV (109 rows)
  - **Coverage:** Weekly constraint boundary limit forecasts for 12 major transmission boundary groups spanning 2026 week 49 through 2028 week 52.
  - **Update Cadence:** Monthly
  - **Licence Name & Link:** [National Energy SO Open Data Licence v1.0](https://www.neso.energy/data-portal/neso-open-licence)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open CKAN API download
  - **Evidence State:** Raw acquired / hash-verified in cache (`spikes/data-gathering/network-ie/manifest.toml`).
  - **Remaining Gaps:** Useful for forward scenario bounds; not a historical 2024 input.

- **Candidate Source (`SRC-24`):** EirGrid / SONI System Data Quarter-Hourly (SNSP Outturns)
  - **Publisher:** EirGrid & SONI
  - **URL:** [https://cms.eirgrid.ie/sites/default/files/publications/System-Data-Qtr-Hourly-2024.xlsx](https://cms.eirgrid.ie/sites/default/files/publications/System-Data-Qtr-Hourly-2024.xlsx)
  - **Format:** Excel Workbook (`.xlsx`)
  - **Coverage:** 35,136 quarter-hour records for all 366 days of 2024. Includes outturn System Non-Synchronous Penetration (`SNSP`), all-island wind/solar generation, net imports, and conventional generation.
  - **Update Cadence:** Annual retrospective publication
  - **Licence Name & Link:** [EirGrid Group Open Data Licence](https://www.smartgriddashboard.com/all/open-data-license/) (file-specific open designation unverified)
  - **D-56 Classification:** `research-only` ([PC-2 in TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md))
  - **Access Requirement:** Open direct download
  - **Evidence State:** Raw acquired / hash-verified in cache (`spikes/data-gathering/network-ie/manifest.toml`).
  - **Remaining Gaps (Finding 6):** Contains realised SNSP observations, **not** operational ceiling rules or minimum conventional unit rules. Operational policy limits (e.g. 75% SNSP ceiling, minimum 7–8 synchronous units) are unverified narrative assumptions and must be sourced from explicit operational publications before being treated as verified rules.

- **Candidate Source (`SRC-25`):** NESO Operability Strategy & System Defence Framework (Inertia & Voltage Rules)
  - **Publisher:** NESO
  - **URL:** [https://www.neso.energy/publications/operability-strategy-report](https://www.neso.energy/publications/operability-strategy-report) (returns HTTP 404; location unresolved)
  - **Format:** Narrative reports / technical appendices
  - **Coverage:** GB national and regional inertia floors and regional minimum-unit requirements.
  - **Update Cadence:** Annual
  - **Licence Name & Link:** No dataset-specific open licence found; NESO general terms apply.
  - **D-56 Classification:** `research-only` (PC-5)
  - **Access Requirement:** Web access (URL returns 404; location unresolved)
  - **Evidence State:** Source identified; dead link; parameters unacquired.
  - **Remaining Gaps (Finding 6):** National inertia floor "~140 GVA·s, declining towards 100" has no supporting document or effective date in 2024; regional unit groups, inertia contribution factors, and minimum-unit rules are **UNVERIFIED / missing**.

#### 2.2 Market Operation and Redispatch Rules (FR-7, D-15, D-59)
- **Candidate Source (`SRC-26`):** Balancing and Settlement Code (BSC) Baseline Statements & Section T/Q
  - **Publisher:** Elexon Limited
  - **URL:** [https://www.elexon.co.uk/bsc-and-codes/balancing-settlement-code/bsc-sections/](https://www.elexon.co.uk/bsc-and-codes/balancing-settlement-code/bsc-sections/) (redirects to digital code hub)
  - **Format:** PDF / HTML legal code documents
  - **Coverage:** GB Balancing Mechanism rules: Balancing Services (Section Q), Settlement and Cashout Pricing (Section T: Net Imbalance Volume / NIV, Single Imbalance Price, System Buy/Sell Prices, Reverse Price).
  - **Update Cadence:** Maintained via code modifications
  - **Licence Name & Link:** BSC legal code is published for code administration; statutory documentation does **not** confer an open commercial database redistribution licence (Finding 1).
  - **D-56 Classification:** `research-only` ([PC-5 in TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md))
  - **Access Requirement:** Open web access
  - **Evidence State:** Source identified / reference-only (statutory digital code hub identified; per [ADR 0006's evidence limits](../architecture/adr/0006-market-design-plugin-model.md#primary-references-and-evidence-limits), specific BSC clauses and historical 2024 rules archive are not verified or acquired).
  - **Remaining Gaps (Finding 11):** Current legal code hub does not pin dated 2024 baseline provisions; Section T cashout rules must be version-pinned for 2024.

- **Candidate Source (`SRC-27`):** I-SEM Trading & Settlement Code (Part B) & Industry Guide
  - **Publisher:** SEM Committee / SEMO
  - **URL:** [https://www.sem-o.com/rules-and-governance/rules/trading-and-settlement-code/](https://www.sem-o.com/rules-and-governance/rules/trading-and-settlement-code/) (returns HTTP 404; location unresolved)
  - **Format:** PDF legal text and guides
  - **Coverage:** I-SEM market arrangements: Balancing Market Imbalance Settlement; Part B rules.
  - **Update Cadence:** Maintained via code modifications
  - **Licence Name & Link:** Published for market participants; standard website disclaimers.
  - **D-56 Classification:** `research-only` (PC-5)
  - **Access Requirement:** Web access (URL returns 404; location unresolved)
  - **Evidence State:** Source identified / reference-only (URL returns 404; [spikes/data-gathering/semo-2024/COVERAGE.md](../../spikes/data-gathering/semo-2024/COVERAGE.md) analysed market archive payloads and settlement cadence, not the statutory Part B legal text).
  - **Remaining Gaps (Finding 11):** Part B alone does not specify SEMOpx auction rules (DAM, IDA1–3, IDC); working URL and dated 2024 auction rule documents must be identified.

- **Candidate Source (`SRC-28`):** Joint Allocation Office (JAO) Explicit Capacity Allocation Rules
  - **Publisher:** JAO S.A.
  - **URL:** [https://www.jao.eu/terms-conditions](https://www.jao.eu/terms-conditions)
  - **Format:** Auction rules PDF, capacity platform terms
  - **Coverage:** Post-Brexit explicit capacity allocation rules on GB↔EU borders.
  - **Update Cadence:** Annual rules updates
  - **Licence Name & Link:** JAO Terms of Use (permits internal analysis; commercial service redistribution restricted)
  - **D-56 Classification:** `research-only` (PC-5)
  - **Access Requirement:** Open web access
  - **Evidence State:** Terms inspected; documented in [data-sources-ie-weather.md](data-sources-ie-weather.md).
  - **Remaining Gaps (Finding 11):** Terms page establishes website conditions, not border-specific capacity-allocation rules or historical market coupling regimes (FR-1.AC2, FR-7.AC4).

---

### Category 3: Underlying and Embedded Demand

#### 3.1 National and Transmission-System Demand Outturn
- **Candidate Source (`SRC-29`):** NESO Historic Demand Data (2024)
  - **Publisher:** NESO
  - **URL:** Package: [https://api.neso.energy/api/3/action/package_show?id=historic-demand-data](https://api.neso.energy/api/3/action/package_show?id=historic-demand-data); Direct Resource: [https://api.neso.energy/dataset/8f2fe0af-871c-488d-8bad-960426f24601/resource/f6d02c0f-957b-48cb-82ee-09003f2ba759/download/demanddata_2024.csv](https://api.neso.energy/dataset/8f2fe0af-871c-488d-8bad-960426f24601/resource/f6d02c0f-957b-48cb-82ee-09003f2ba759/download/demanddata_2024.csv)
  - **Format:** CSV
  - **Coverage:** Complete GB half-hourly demand outturn for 2024 (17,568 rows across 366 dates). Fields include National Demand (`ND`), Transmission System Demand (`TSD`), embedded solar generation (`EMBEDDED_SOLAR_GENERATION`), embedded wind generation (`EMBEDDED_WIND_GENERATION`), pumped storage pumping (`PUMP_STORAGE_PUMPING`), and interconnector flows.
  - **Update Cadence:** Published 21 days in arrears, with retrospective settlement reconciliations
  - **Licence Name & Link:** [National Energy SO Open Data Licence v1.0](https://www.neso.energy/data-portal/neso-open-licence)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open CKAN API download
  - **Evidence State:** Raw acquired / hash-verified in cache; sample in `spikes/data/samples/demand-2024.csv`; audited in [spikes/data/REPORT.md](../../spikes/data/REPORT.md).
  - **Remaining Gaps:** Initial demand outturn vs final settlement run reconciliation.

- **Candidate Source (`SRC-30`):** Elexon Initial Demand Outturn (`demand_outturn`)
  - **Publisher:** Elexon Limited
  - **URL:** [https://data.elexon.co.uk/bmrs/api/v1/demand/outturn](https://data.elexon.co.uk/bmrs/api/v1/demand/outturn) (requires settlement date range, e.g. `?settlementDateFrom=2024-01-01&settlementDateTo=2024-12-31`)
  - **Format:** JSON REST API / daily files
  - **Coverage:** Initial Demand Outturn (`INDO`) and Initial Transmission System Demand Outturn (`ITSDO`) for all 17,568 settlement periods of 2024 (366 daily files).
  - **Update Cadence:** Daily near-real-time publication
  - **Licence Name & Link:** [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open REST API download
  - **Evidence State:** Raw acquired / hash-verified in cache (`spikes/data-gathering/elexon/COVERAGE.md`, 366 files, 3.40 MiB).
  - **Remaining Gaps:** Initial outturn lacks settlement reconciliation.

#### 3.2 Embedded Generation & Demand Decomposition (FR-3.AC3)
- **Candidate Source (`SRC-31`):** Elexon Half-Hourly Generation by Fuel Type (`fuelhh`)
  - **Publisher:** Elexon Limited
  - **URL:** [https://data.elexon.co.uk/bmrs/api/v1/datasets/FUELHH/stream](https://data.elexon.co.uk/bmrs/api/v1/datasets/FUELHH/stream) (requires dated query)
  - **Format:** JSON stream (366 daily files)
  - **Coverage:** Generation by fuel category for every settlement period of 2024 (340,894 records). Categories present: `BIOMASS`, `CCGT`, `COAL`, interconnectors, `NPSHYD`, `NUCLEAR`, `OCGT`, `OIL`, `OTHER`, `PS`, and `WIND`.
  - **Update Cadence:** Half-hourly
  - **Licence Name & Link:** [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open REST API stream
  - **Evidence State:** Raw acquired / hash-verified in cache (`spikes/data-gathering/elexon/COVERAGE.md`, 366 files, 58.91 MiB).
  - **Correction (Findings 3, R2-02):** FUELHH does **not** provide embedded solar or embedded wind decomposition; there is **no SOLAR category** in FUELHH, and its `WIND` series is an aggregate. GB embedded estimates must be taken from the explicit NESO Historic Demand Data (`SRC-29`) columns `EMBEDDED_SOLAR_GENERATION` and `EMBEDDED_WIND_GENERATION`. The Irish equivalent embedded series is currently unacquired.
  - **Demand Accounting & Boundaries (Finding R2-02):** In the NESO Historic Demand dataset, National Demand (`ND`) is defined as transmission-supplied demand excluding power station transmission-connected load (station load), pumped storage pumping, and interconnector exports; Transmission System Demand (`TSD`) adds station load and pumping back. Consequently, subtracting pumping from `ND` subtracts pumping twice. The starting accounting identity for gross underlying demand is:
    $$\text{Gross Underlying Demand (starting accounting identity)} = \text{National Demand} (\text{ND}) + \text{Embedded Solar} + \text{Embedded Wind}$$
    *Boundary qualifications:* This formulation is a starting accounting identity, not proof that all gross-demand, station-load, transmission loss, import/export, and battery storage charging/discharging boundaries are resolved. Pumping and storage withdrawals must be recorded once as distinct demand components rather than deducted from `ND`. GSP allocation factors, station load reconciliation, energy integration (MW power to MWh per half-hour interval), and double-counting verification test fixtures remain explicitly pending.

#### 3.3 Nodal and Grid Supply Point (GSP) Demand
- **Candidate Source (`SRC-48`):** NESO ETYS Appendix G (GSP Peak Demand Snapshots)
  - **Publisher:** NESO
  - **Format:** Tabular Excel / CSV
  - **Coverage:** Peak winter/summer transmission-system GSP demand snapshots across Great Britain.
  - **D-56 Classification:** `research-only` (PC-1)
  - **Access Requirement:** Open direct download
  - **Evidence State:** Source identified; parameters unextracted.
- **Candidate Source (`SRC-49`):** DNO Long Term Development Statement Table 7 (GSP Substation Demands)
  - **Publisher:** Distribution Network Operators (14 licence areas)
  - **Format:** Tabular CSV / Excel
  - **Coverage:** Distribution-level GSP capacities, transformer ratings, and peak demand data.
  - **D-56 Classification:** `research-only / unverified` (PC-5)
  - **Access Requirement:** Portal logins / data requests (varies by DNO)
  - **Evidence State:** Source identified; parameters unextracted.
  - **Remaining Gaps:** Peak snapshots must be synthesized with half-hourly national and zonal profiles to create nodal time-series; GSP naming crosswalks, bus attachments, and synthetic distribution models remain to be implemented.

---

### Category 4: Network Losses

#### 4.1 Interconnector Losses (FR-1.AC3)
- **Candidate Source (`SRC-32`):** Interconnector Technical Specifications & Loss Factor Filings
  - **Publisher:** Interconnector Operators (BritNed, National Grid Interconnectors, EirGrid, Moyle Interconnector Ltd, Greenlink Interconnector Ltd)
  - **URL:** BritNed: [https://www.britned.com/](https://www.britned.com/); ElecLink: [https://www.eleclink.co.uk/](https://www.eleclink.co.uk/); Greenlink: [https://greenlink.ie/operations/](https://greenlink.ie/operations/)
  - **Licence Name & Link:** Public regulatory filings / statutory rules (does not confer open commercial reuse rights, Finding 1)
  - **D-56 Classification:** `research-only` ([PC-5 in TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md))
  - **Access Requirement:** Public web access
  - **Evidence State:** Source identified; parameters incomplete.
  - **Per-Interconnector Loss Model Status Ledger (Findings 5, R2-03, R2-06):**
    
    | Interconnector Link | Directional Flow & Metering Conventions | Converter Loss (Fwd / Rev) | Cable Loss (Fwd / Rev) | Total Loss Factor / Function | Source Status & Derivation |
    |---|---|---|---|---|---|
    | **IFA (GB $\leftrightarrow$ France)** | Forward: GB $\rightarrow$ FR; Reverse: FR $\rightarrow$ GB; Metering boundary unverified | **MISSING** | **MISSING** | **MISSING** | **MISSING** (No dated 2024 coefficient table; uncited defaults removed) |
    | **IFA2 (GB $\leftrightarrow$ France)** | Forward: GB $\rightarrow$ FR; Reverse: FR $\rightarrow$ GB; Metering boundary unverified | **MISSING** | **MISSING** | **MISSING** | **MISSING** (No dated 2024 coefficient table; uncited defaults removed) |
    | **ElecLink (GB $\leftrightarrow$ France)** | Forward: GB $\rightarrow$ FR; Reverse: FR $\rightarrow$ GB; Metering boundary unverified | **MISSING** | **MISSING** | **MISSING** | **MISSING** (No dated 2024 coefficient table; uncited defaults removed) |
    | **BritNed (GB $\leftrightarrow$ Netherlands)** | Forward: GB $\rightarrow$ NL; Reverse: NL $\rightarrow$ GB; Metering boundary unverified | **MISSING** | **MISSING** | **MISSING** | **MISSING** (No dated 2024 coefficient table; uncited defaults removed) |
    | **Nemo Link (GB $\leftrightarrow$ Belgium)** | Forward: GB $\rightarrow$ BE; Reverse: BE $\rightarrow$ GB; Metering boundary unverified | **MISSING** | **MISSING** | **MISSING** | **MISSING** (No dated 2024 coefficient table; uncited defaults removed) |
    | **North Sea Link (GB $\leftrightarrow$ Norway)** | Forward: GB $\rightarrow$ NO; Reverse: NO $\rightarrow$ GB; Metering boundary unverified | **MISSING** | **MISSING** | **MISSING** | **MISSING** (No dated 2024 coefficient table; uncited defaults removed) |
    | **Viking Link (GB $\leftrightarrow$ Denmark)** | Forward: GB $\rightarrow$ DK; Reverse: DK $\rightarrow$ GB; Metering boundary unverified | **MISSING** | **MISSING** | **MISSING** | **MISSING** (No dated 2024 coefficient table; uncited defaults removed) |
    | **Moyle (Scotland $\leftrightarrow$ NI)** | Forward: SCO $\rightarrow$ NI; Reverse: NI $\rightarrow$ SCO; Metering boundary unverified | **MISSING** | **MISSING** | **MISSING** | **MISSING** (No dated 2024 coefficient table; uncited defaults removed) |
    | **EWIC (Wales $\leftrightarrow$ Ireland)** | Forward: WAL $\rightarrow$ IE; Reverse: IE $\rightarrow$ WAL; Metering boundary unverified | **MISSING** | **MISSING** | **MISSING** | **MISSING** (No dated 2024 coefficient table; uncited defaults removed) |
    | **Greenlink (Wales $\leftrightarrow$ Ireland)** | Forward: WAL $\rightarrow$ IE; Reverse: IE $\rightarrow$ WAL; Metering boundary unverified | **MISSING** | **MISSING** | **MISSING** | **MISSING** (7,102 sparse observations in 2024; cause unverified; not a fully commissioned 2024 baseline) |

  - **Remaining Gaps (Finding R2-03):** All directional physical loss parameters are currently **MISSING** as explicit, dated 2024 coefficient tables. Unsupported numeric percentage defaults have been removed across all 10 links. Metering conventions (whether losses are attributed at the sending bus, receiving bus, or split mid-interconnector) and distinct directional parameters (forward vs reverse flow) are not documented on public landing pages and must be extracted from formal operator regulatory filings. Converter and cable losses must be separated, and interconnector losses must not be double-counted against onshore transmission branch losses.

#### 4.2 Transmission & Distribution Branch Losses (FR-8, FR-10, ADR 0003)
- Resistances from ETYS Appendix B (`SRC-01`) and DNO Table 1 (`SRC-03`, `SRC-05`) provide positive sequence resistance $R_1$. Affine piecewise loss approximations must conserve active power.

#### 4.3 System Loss Accounting and Outturns (TLM)
- **Candidate Source (`SRC-50`):** Elexon BMRS Settlement Stack API (`transmissionLossMultiplier` Candidate)
  - **Publisher:** Elexon Limited
  - **URL:** [https://data.elexon.co.uk/bmrs/api/v1/balancing/settlement/stack/all/](https://data.elexon.co.uk/bmrs/api/v1/balancing/settlement/stack/all/) (per `{bidOffer}/{settlementDate}/{settlementPeriod}`)
  - **Format:** JSON REST API
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open REST API
  - **Evidence State:** Source identified (candidate only; not acquired in gathered streams).
- **Candidate Source Status & Corrections (Findings 4, R2-04):**
  - The previous URL (`https://data.elexon.co.uk/bmrs/api/v1/balancing/settlement/system-prices/`) returns HTTP 404; when dated, it returns prices and NIV, **not** a delivering/offtaking BMU TLM panel.
  - None of the 12 gathered Elexon settlement streams contains a TLM panel.
  - Elexon's OpenAPI does expose a `transmissionLossMultiplier` in the settlement stack response at `/balancing/settlement/stack/all/{bidOffer}/{settlementDate}/{settlementPeriod}` (`SRC-50`). This is an unacquired candidate to investigate.
  - **Correction:** Prior claims of "included within settlement streams" and "downloaded for 2024" are **retracted**. Transmission loss multipliers are currently **UNVERIFIED / missing**.

---

### Category 5: Calendar, Holidays, and Clock Changes

#### 5.1 Market Settlement Calendars & Bank Holidays
- **Candidate Source (`SRC-33`):** GOV.UK Bank Holidays API
  - **Publisher:** UK Cabinet Office / Government Digital Service
  - **URL:** [https://www.gov.uk/bank-holidays.json](https://www.gov.uk/bank-holidays.json)
  - **Format:** JSON
  - **Coverage:** Official bank holidays for England and Wales, Scotland, and Northern Ireland (observed range 2019–2028, including all 2024 holidays).
  - **Licence Name & Link:** [Open Government Licence v3.0](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/)
  - **D-56 Classification:** `open-commercial`
  - **Evidence State:** Live endpoint verified (HTTP 200).
  - **Remaining Gaps:** Pinning capture to versioned repository artifact.

- **Candidate Source (`SRC-34`):** Irish Public Holiday Schedule
  - **Publisher:** Government of Ireland (Citizens Information / Workplace Relations)
  - **URL:** [https://www.citizensinformation.ie/en/employment/employment-rights-and-conditions/leave-and-holidays/public-holidays/](https://www.citizensinformation.ie/en/employment/employment-rights-and-conditions/leave-and-holidays/public-holidays/)
  - **Format:** Web page (HTTP 403 Forbidden on automated probe)
  - **Licence Name & Link:** No machine-readable CC BY dataset identified on data.gov.ie for statutory holidays (Finding 1, 13).
  - **D-56 Classification:** `research-only / unverified` ([PC-5 in TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md))
  - **Evidence State:** Access unverified (HTTP 403); statutory dates known from legislation (10 public holidays in 2024).
  - **Remaining Gaps:** Codify in an immutable versioned repository calendar.

#### 5.2 Daylight Saving Clock Transitions (FR-3.AC1, FR-5)
- **Candidate Source (`SRC-35`):** IANA Time Zone Database (`tzdata`)
  - **Publisher:** Internet Assigned Numbers Authority (IANA)
  - **URL:** [https://www.iana.org/time-zones](https://www.iana.org/time-zones)
  - **Format:** Standard Olson timezone compilation (`Europe/London`, `Europe/Dublin`)
  - **Coverage:** Complete daylight saving transition rules. In 2024:
    - Spring transition (2024-03-31): clocks advance 01:00 GMT $\rightarrow$ 02:00 BST/IST (short day: exactly 46 settlement periods).
    - Autumn transition (2024-10-27): clocks fall back 02:00 BST/IST $\rightarrow$ 01:00 GMT (long day: exactly 50 settlement periods).
  - **Licence Name & Link:** Public Domain
  - **D-56 Classification:** `open-commercial`
  - **Evidence State:** Verified; transition behavior tested in prior reviews.
  - **Handling Rule (Finding 12):** Source rows in local time during the autumn fall-back transition must **retain source interval IDs and fold/offset**. Do **not** discard duplicate civil hours; map each interval to its distinct UTC timestamp.

#### 5.3 Continuous Delivery Interval Engine (FR-3.AC1)
- **Internal Artifact (`INT-02`):** ISO 8601 UTC Continuous Delivery Interval Engine
  - **Publisher:** Internal simulator specification / standard time arithmetic
  - **Coverage:** Exactly 17,568 half-hours in 2024 (366 days $\times$ 48 settlement periods).
  - **Classification:** Internal code / arithmetic (not an acquired external dataset; no external licence invented).

---

### Category 6: Scenario Identity Inputs (FR-4, NFR-1, FR-3)

#### 6.1 Canonical Scenario Definition & Execution Settings (FR-4, NFR-1)
- **Internal Artifact (`INT-01`):** Canonical Scenario Schema & Checksum Specification
  - **Publisher:** Internal simulator specification (ADR 0002, [04-requirements.md](../requirements/04-requirements.md) FR-4)
  - **Classification:** Internal simulator specification (not an acquired dataset; no external licence invented, Finding 1, 15).
  - **Scenario Identity & Specification Inventory Checklist (Findings 15, R2-07):**
    Per ADR 0002, a complete canonical scenario definition requires the following identity and specification artifacts:
    1. **Independent Identity Hashing:** Distinct content hashes for scenario input identity and execution identity; separate tracking of operational run ID and attempt ID (equal inputs do not collapse separate execution attempts).
    2. **Knowledge Modes:** Explicit declaration of run mode: *as-of forecast backtest* (selecting only entity revisions provably published before a knowledge cut timestamp) versus *historical reconstruction* (used when publication history is unknown or retrospective data is selected).
    3. **Semantic Canonicalisation:** Typed domain entities (substations, buses, circuits, transformers, generators, storage, demand attachments, interconnectors) with permanent opaque UUIDs independent of mutable names or row positions, plus external alias mappings.
    4. **Three Distinct Timestamps:** Explicit half-open UTC effective intervals `[from, to)`, source publication/revision time (where known), and system acquisition time.
    5. **Versioned Dependency Artifacts:** Version-pinned mappings for:
       - Civil-to-market calendars and tzdata (settlement date/period, DST fold, leap year);
       - Transformation, resampling, and spatial aggregation policies;
       - Generator operating cost models (heat rates, fuel price mappings, variable O&M, carbon intensities);
       - Foreign exchange rate vectors;
       - Published operational security and stability constraint proxies (boundary limits, SNSP ceilings, inertia floors).
    6. **Rights Lineage & Clearances:** Provenance manifests capturing dataset licence status (`open-commercial` vs `research-only`) and canonical pre-commercial clearance IDs (`PC-1` through `PC-7`).
    7. **Migration & Hash Vectors:** Schema migration framework and deterministic manifest hashes over all tabular and time-series inputs.
  - **Remaining Backlog:** The scenario schema exists as an architectural design in ADR 0002. The Rust serde deserializer, JSON-schema validation, immutable release locks, and round-trip test fixtures remain to be implemented in M1.

#### 6.2 Fuel, Carbon, and Variable Operating Costs (FR-7, D-59)
- **Candidate Sources (`SRC-36a`–`SRC-38b`):**
  - **UK ETS Auction Clearing Prices (`SRC-36a`):**
    - *Publisher / Host:* ICE Futures Europe / UK ETS Authority (DESNZ)
    - *URL:* [https://www.gov.uk/government/publications/uk-ets-auction-results](https://www.gov.uk/government/publications/uk-ets-auction-results) (returns HTTP 404; location unresolved)
    - *Release / Product:* UK Allowance (UKA) primary auction clearing results
    - *Units:* £/tCO2e (Pounds sterling per metric tonne of CO2 equivalent)
    - *Currency:* GBP (£)
    - *Frequency:* Bi-weekly / periodic auctions (not daily spot prices)
    - *Licence Name & Link:* Unverified. Third-party auction data rights held by ICE Futures Europe are not cleared by GOV.UK portal hosting or general OGL website footers (Findings 1, 10, R2-01).
    - *D-56 Classification:* `research-only / unverified` ([PC-5 in TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md))
    - *Access Requirement:* Web download (URL unresolved / 404)
    - *Evidence State:* Source identified; location unresolved; parameters unacquired.
  - **EU ETS Auction / Spot Prices (`SRC-36b`):**
    - *Publisher:* European Energy Exchange (EEX) / European Commission
    - *URL:* Platform: [https://www.eex.com/](https://www.eex.com/)
    - *Format:* Web / CSV download
    - *Release / Product:* EU Allowance (EUA) primary auction and secondary spot prices
    - *Units:* €/tCO2e
    - *Currency:* EUR (€)
    - *Frequency:* Daily / auction periods
    - *Licence Name & Link:* EEX Market Data terms (exchange market data requires commercial licence; third-party reuse unverified)
    - *D-56 Classification:* `research-only / unverified` (PC-5)
    - *Access Requirement:* Unacquired
    - *Evidence State:* Source identified; unacquired.
  - **DESNZ Fossil Fuel Price Assumptions (`SRC-37a`):**
    - *Publisher:* DESNZ
    - *URL:* [https://www.gov.uk/government/collections/fossil-fuel-price-assumptions](https://www.gov.uk/government/collections/fossil-fuel-price-assumptions)
    - *Format:* Spreadsheet / PDF
    - *Coverage:* Long-term oil, gas, and coal price scenario projections (not historical 2024 spot outturns).
    - *Licence Name & Link:* [Open Government Licence v3.0](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/)
    - *D-56 Classification:* `open-commercial`
    - *Access Requirement:* Open direct download
    - *Evidence State:* Source identified; scenario assumptions only.
  - **World Bank Commodity Price Data (`SRC-37b`):**
    - *Publisher:* World Bank
    - *URL:* [https://www.worldbank.org/en/research/commodity-markets](https://www.worldbank.org/en/research/commodity-markets)
    - *Format:* Excel / CSV (Pink Sheet)
    - *Coverage:* Monthly and annual historical global commodity prices (natural gas, coal, crude oil).
    - *Licence Name & Link:* [World Bank Dataset Terms of Use](https://data.worldbank.org/summary-terms-of-use) (CC BY 4.0 with additional condition terms)
    - *D-56 Classification:* `open-commercial`
    - *Access Requirement:* Open direct download
    - *Evidence State:* Metadata inspected (CC BY 4.0 confirmed). Note: Series are monthly; any expansion to daily or half-hourly resolution must be explicitly labelled as resampled.
  - **CfD Strike Prices (`SRC-38a`):**
    - *Publisher:* Low Carbon Contracts Company (LCCC)
    - *URL:* [https://www.lowcarboncontracts.uk/data-portal](https://www.lowcarboncontracts.uk/data-portal) (returns HTTP 403 on automated probe)
    - *Format:* Web / CSV data portal
    - *Coverage:* Contracts for Difference strike prices, market reference prices, and generation payments.
    - *Licence Name & Link:* Portal terms unverified (HTTP 403)
    - *D-56 Classification:* `research-only / unverified` (PC-5)
    - *Access Requirement:* Web download (unverified access)
    - *Evidence State:* Source identified; portal access unverified.
  - **Renewables Obligation (RO) Buyout Price & Parameters (`SRC-38b`):**
    - *Publisher:* Office of Gas and Electricity Markets (Ofgem)
    - *URL:* Scheme publications: [https://www.ofgem.gov.uk/environmental-and-social-schemes/renewables-obligation-ro](https://www.ofgem.gov.uk/environmental-and-social-schemes/renewables-obligation-ro)
    - *Format:* Annual administrative guidance documents / circulars
    - *Coverage:* Annual RO buyout price (£/ROC) and obligation levels for 2024.
    - *Licence Name & Link:* Crown copyright / OGL v3.0 on Ofgem portal, but exact machine-readable dataset location is unidentified (parameters published in policy circulars).
    - *D-56 Classification:* `research-only / unverified` (PC-5)
    - *Access Requirement:* Web access; exact machine-readable dataset location unidentified
    - *Evidence State:* Source identified; statutory parameters published in circulars; machine-readable dataset location unidentified.
  - **Technology SRMC Parameters (Finding 10):** Generator heat rates, emissions factors, and variable O&M are unacquired / assumed. No complete daily SRMC vector exists in the repository.

#### 6.3 Weather-Derived Renewable Resource Profiles (FR-3, FR-17)
- **Candidate Source (`SRC-39`):** Copernicus Climate Data Store (CDS) ERA5 Reanalysis (Single Levels)
  - **Publisher:** ECMWF / Copernicus Climate Change Service
  - **URL:** [https://cds.climate.copernicus.eu/datasets/reanalysis-era5-single-levels](https://cds.climate.copernicus.eu/datasets/reanalysis-era5-single-levels)
  - **Format:** GRIB / NetCDF via CDS API
  - **Coverage:** Global regular 0.25° grid. Consolidated ERA5 available for 2024 (latency 2–3 months). Hourly series: 100m wind (`u100`, `v100`), 10m wind (`u10`, `v10`), 2m temperature (`2t`), solar radiation (`ssrd`, `fdir`). Bounding box for GB+IE: 48°N–61°N, 12°W–10°E (4,717 grid points).
  - **Licence Name & Link:** Catalogue API confirms [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** **Owner credentials required**. Registered CDS account, terms agreement on web portal, and `~/.cdsapirc` containing personal API key.
  - **Evidence State:** Request template drafted in `spikes/data/era5-request-2024-01.json`; volume budget (~1.33 GB uncompressed float32 per year) calculated; **0 weather bytes currently downloaded**.
  - **Remaining Gaps:** Owner credentials required; power-curve conversion software unintegrated.

- **Candidate Source (`SRC-40`):** Met Office MIDAS Open Surface Observations
  - **Publisher:** UK Met Office / CEDA Archive
  - **URL:** [https://catalogue.ceda.ac.uk/uuid/dbd451271eb04662beade68da43546e1/](https://catalogue.ceda.ac.uk/uuid/dbd451271eb04662beade68da43546e1/)
  - **Format:** CSV via CEDA
  - **Coverage:** Hourly surface observations from UK weather stations.
  - **Licence Name & Link:** [Open Government Licence v3.0](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/) (distinct from restricted full MIDAS)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Free CEDA user registration required.

- **Candidate Source (`SRC-41`):** Met Éireann MÉRA Reanalysis & Open Data
  - **Publisher:** Met Éireann
  - **URL:** [https://www.met.ie/climate/available-data/mera](https://www.met.ie/climate/available-data/mera)
  - **Format:** GRIB 1 sample (not NetCDF/CSV)
  - **Coverage:** Historical archive covers 1981–August 2019 only (Finding 16); **does not cover 2024**. Operational station data covers 2024 as a separate product.
  - **Licence Name & Link:** Met Éireann general website copyright restricts reuse; specific open data grant unverified.
  - **D-56 Classification:** `research-only / unverified` ([PC-5 in TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md))
  - **Remaining Gaps:** Cannot be used as a 2024 reanalysis source.

- **Candidate Source (`SRC-42`):** Renewables.ninja (Simulated Wind & Solar Benchmark)
  - **Publisher:** Renewables.ninja (Imperial College London / ETH Zürich)
  - **URL:** [https://www.renewables.ninja/](https://www.renewables.ninja/); Terms: [https://www.renewables.ninja/about](https://www.renewables.ninja/about)
  - **Format:** CSV / JSON API
  - **Licence Name & Link:** Publisher terms confirm **Creative Commons Attribution-NonCommercial 4.0 International (CC BY-NC 4.0)** (not share-alike BY-NC-SA, Finding 14)
  - **D-56 Classification:** `research-only` (PC-5; strictly non-commercial)
  - **Access Requirement:** Free user account and API token required.
  - **Purpose:** Offline research-only benchmark to validate internal ERA5 power-curve conversions.

#### 6.4 Historical Validation Holdout Series (2024 Holdout Year, FR-12)
- **Candidate Source (`SRC-43`):** Elexon BMRS 2024 Operational and Market Datasets
  - **Publisher:** Elexon Limited
  - **URL:** [https://data.elexon.co.uk/](https://data.elexon.co.uk/) (API routes from manifest)
  - **Format:** JSON / CSV streams (12 streams, 3,673 files, 4.16 GiB total)
  - **Coverage:** All 17,568 settlement periods of 2024 across 12 streams: System Prices, Market Index Data (`mid`), Physical Notifications (`pn`), Bid-Offer Data (`bod`), Bid-Offer Acceptances (`boalf`), Maximum Limits (`mels`, `mils`), Fuel Outturn (`fuelhh`), Interconnector Flows (`interconnectors`), REMIT Messages (`remit`).
  - **Licence Name & Link:** [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/)
  - **D-56 Classification:** `open-commercial`
  - **Evidence State:** Raw acquired / hash-verified in cache (`spikes/data-gathering/elexon/COVERAGE.md`).
  - **Correction:** Market Index Data (`mid`) is a volume-weighted average price across exchanges, **not** a day-ahead auction price (Finding 17).

- **Candidate Source (`SRC-44`):** SEMO / SEMOpx 2024 I-SEM Price Archives
  - **Publisher:** SEMO & SEMOpx
  - **URL:** Pinned archives from manifest:
    - SEMOpx Lookback Archive: [https://www.semopx.com/sites/semopx/files/2026-11/Lookback_Market%20October2026.zip](https://www.semopx.com/sites/semopx/files/2026-11/Lookback_Market%20October2026.zip) (39.17 MB; replaces `lookback_mkt.xlsx` which had 2018–2020 data, Finding 8)
    - SEMO Imbalance Archive: [https://www.sem-o.com/documents/general-publications/Imbalance_Price_YTD_2020-2024.zip](https://www.sem-o.com/documents/general-publications/Imbalance_Price_YTD_2020-2024.zip) (4.46 MB)
  - **Format:** Excel Workbook & XML/CSV archives (43.6 MB total)
  - **Licence Name & Link:** [EirGrid Group Open Data Licence](https://www.smartgriddashboard.com/all/open-data-license/) names SEMO, but archives carry general website disclaimers; express open grant unverified.
  - **D-56 Classification:** `research-only` ([PC-3 in TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md))
  - **Evidence State:** Raw acquired / hash-verified in cache (`spikes/data-gathering/semo-2024/COVERAGE.md`).
  - **Measured Limitations & Gaps (Finding 8):**
    - Day-Ahead Market (`DAM`): 8,784 hourly prices (mean €108.91/MWh UTC-year).
    - Intraday Auctions (`IDA1`): 17,568 half-hour prices.
    - Intraday Auctions (`IDA2`, `IDA3`): 8,784 / 4,392 half-hour prices (cover auction windows only, not every settlement period).
    - Imbalance Settlement 30-min (`30MIN`): 17,507 rows (**61 missing intervals**).
    - Imbalance Settlement 5-min (`5MIN`): 96,147 rows (**333 missing 5-min intervals in Jan–Nov; 8,928 missing intervals in December**).
    - Intraday Continuous (`IDC`): interval identities not recoverable from aggregate sheet alone.

- **Candidate Source (`SRC-45`):** NESO Outturn Thermal Constraint Costs (2024–2025)
  - **Publisher:** NESO
  - **URL:** Package: [https://api.neso.energy/api/3/action/package_show?id=f0055054-c55c-4068-a01c-61da4334e58f](https://api.neso.energy/api/3/action/package_show?id=f0055054-c55c-4068-a01c-61da4334e58f); Direct Resource: [https://api.neso.energy/dataset/f0055054-c55c-4068-a01c-61da4334e58f/resource/27df75fd-6233-466c-beed-3e6e2261e6b1/download/outturn-system-costs-2024-2025.csv](https://api.neso.energy/dataset/f0055054-c55c-4068-a01c-61da4334e58f/resource/27df75fd-6233-466c-beed-3e6e2261e6b1/download/outturn-system-costs-2024-2025.csv)
  - **Format:** CSV (2,190 rows)
  - **Coverage:** Daily thermal constraint costs (`Daily Cost GBP`) for six major constraint groups from 2024-04-01 through 2025-03-31.
  - **Licence Name & Link:** [National Energy SO Open Data Licence v1.0](https://www.neso.energy/data-portal/neso-open-licence)
  - **D-56 Classification:** `open-commercial`
  - **Evidence State:** Raw acquired / hash-verified in cache (`spikes/data-gathering/network-ie/manifest.toml`).
  - **Remaining Gaps:** Covers financial year 2024/25; calendar Q1 2024 (Jan–Mar 2024) is missing from this file.

#### 6.5 Exogenous Foreign Exchange Rates (FR-1.AC4, D-38)
- **Candidate Source (`SRC-46`):** European Central Bank (ECB) Reference Rates
  - **Publisher:** European Central Bank
  - **URL:** Series: [https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/html/index.en.html](https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/html/index.en.html); Terms: [https://www.ecb.europa.eu/services/disclaimer/html/index.en.html](https://www.ecb.europa.eu/services/disclaimer/html/index.en.html)
  - **Format:** CSV / XML / JSON
  - **Coverage:** Daily EUR/GBP exchange rates published around 16:00 CET on working days.
  - **Licence Name & Link:** [ECB Reuse Disclaimer](https://www.ecb.europa.eu/services/disclaimer/html/index.en.html) (requires source acknowledgement; disclosure required if sold)
  - **D-56 Classification:** `open-commercial`
  - **Access Requirement:** Open direct download (no credentials required)
  - **Evidence State:** Terms and data structure verified; primary numeraire source.
  - **Remaining Gaps:** Non-working days (weekends, TARGET holidays) require forward-filling the last valid reference rate into a continuous 366-day UTC series.

- **Candidate Source (`SRC-47`):** Bank of England Database (`XUDLSER`)
  - **Publisher:** Bank of England
  - **URL:** [https://www.bankofengland.co.uk/boeapps/database/](https://www.bankofengland.co.uk/boeapps/database/); Terms: [https://www.bankofengland.co.uk/legal](https://www.bankofengland.co.uk/legal)
  - **Format:** CSV
  - **Licence Name & Link:** [Bank of England Legal Terms](https://www.bankofengland.co.uk/legal) (OGL v3.0, but selected third-party series are excluded; selected series clearance is unverified, Finding 1)
  - **D-56 Classification:** `research-only / unverified` ([PC-5 in TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md))
  - **Access Requirement:** Open direct download
  - **Remaining Gaps:** Retained as secondary check; ECB reference rate is preferred primary numeraire.

---

## 3. Owner Credentials Summary & Capability Unlock Matrix

Several candidate sources require owner actions (account creation, API key generation, or formal data request approval). In accordance with Findings 14 and R2-04, portal registrations and application forms are listed as **prerequisites only**; actual access permissions, request approval processes, request form availability, and whether requested payloads (such as CIM models or full row tables) are provided to research users remain **unverified**. Credentials are never committed to git. In accordance with D-65 and NFR-12.AC1/AC2, isolation and resource tests on dedicated runner environments must be completed prior to executing any heavy data processing jobs:

| Required Credential / Account | Target Service / Portal | Access Type | What It Unlocks for Milestone M1 | Action Required by Owner |
|---|---|---|---|---|
| **ENTSO-E API Security Token** | ENTSO-E Transparency Platform ([transparency.entsoe.eu](https://transparency.entsoe.eu/)) | Personal API token (`securityToken`) | Prerequisite for automated REST API retrieval of scheduled commercial interconnector exchanges (`A09`) and cross-border physical flows (`A11`) for independent validation. | 1. Register free account on [transparency.entsoe.eu](https://transparency.entsoe.eu/).<br>2. Email `transparency@entsoe.eu` with subject `RESTful API access`.<br>3. Generate token under "My Account" and set environment variable. |
| **Copernicus CDS Account & API Key** | Copernicus Climate Data Store ([cds.climate.copernicus.eu](https://cds.climate.copernicus.eu/)) | API Key via `~/.cdsapirc` | Prerequisite for automated submission and downloading of ERA5 gridded hourly weather data (wind, solar radiation, temperature) for 2024 to generate renewable generator profiles. | 1. Create free account at [cds.climate.copernicus.eu](https://cds.climate.copernicus.eu/).<br>2. Accept online dataset terms for ERA5 single levels.<br>3. Copy API key from user profile into `~/.cdsapirc`. |
| **CEDA User Registration** | CEDA Archive ([catalogue.ceda.ac.uk](https://catalogue.ceda.ac.uk/)) | Registered user account | Prerequisite for automated download of Met Office MIDAS Open hourly weather station observations for validating ERA5 ground accuracy. | 1. Register free account on [services.ceda.ac.uk](https://services.ceda.ac.uk/).<br>2. Agree to OGL terms for MIDAS Open dataset. |
| **Renewables.ninja User Account & Token** | Renewables.ninja ([renewables.ninja](https://www.renewables.ninja/)) | Registered user API token | Prerequisite for automated API access to simulated regional wind and solar capacity factors for offline benchmark comparison. | 1. Register free account on [renewables.ninja](https://www.renewables.ninja/).<br>2. Obtain API token from user profile. |
| **UK Power Networks Open Data Login** | UK Power Networks Portal ([ukpowernetworks.opendatasoft.com](https://ukpowernetworks.opendatasoft.com/)) | Registered user account prerequisite | Prerequisite for authenticated access; whether user registration alone enables bulk row exports without organizational role approval is unverified. | 1. Register free account on UKPN portal.<br>2. Test whether login permits row export queries without HTTP 403. |
| **UK Power Networks CIM Request** | UK Power Networks SharePoint | Formal access application prerequisite | Prerequisite application form for CIM access; approval criteria, review turnaround, and whether requested CIM XML models are released for non-licensee research remain unverified. | 1. Submit [CIM Access Request Form](https://ukpowernetworks.opendatasoft.com/explore/forms/cim-access-request-form/).<br>2. Await determination and verify deliverable availability. |
| **SP Energy Networks Open Data Login** | SP Energy Networks Portal ([spenergynetworks.opendatasoft.com](https://spenergynetworks.opendatasoft.com/)) | Registered user account prerequisite | Prerequisite for portal access; whether user registration alone enables bulk row-level downloads without additional permissions is unverified. | 1. Register free account on SPEN portal.<br>2. Test whether login permits row queries. |
| **SP Energy Networks Data Request** | SP Energy Networks Shared Data Portal | Formal Data Request prerequisite | Prerequisite application for ENA Shared Data Licence; request form location, approval requirements, and availability of SPM CIM EQ profiles remain unverified. | 1. Submit Data Request form under ENA Shared Data Licence.<br>2. Await review and verify payload availability. |
| **Northern Powergrid Open Data Portal Login** | Northern Powergrid Portal ([northernpowergrid.opendatasoft.com](https://northernpowergrid.opendatasoft.com/)) | Registered user account prerequisite | Prerequisite for portal access; row-level download permissions and table export access remain unverified. | 1. Register free account on Northern Powergrid portal.<br>2. Test export permissions for Appendix 3 and Appendix 4. |
| **Electricity North West Secure Area Login** | ENWL Secure Portal ([enwl.co.uk](https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/login-to-ltds-secure-area/)) | Secure area registration prerequisite | Prerequisite registration for secure portal; approval criteria, account verification, and availability of full CIM models remain unverified. | 1. Register on ENWL website for LTDS Secure Area access.<br>2. Await account verification and inspect available models. |
| **D-65 Target Runner Provisioning** | Dedicated Linux Benchmark/Run Machine (D-48, D-65) | Host SSH access / self-hosted GitHub Actions runner | Unlocks execution of heavy M1 data processing jobs, PostgreSQL database population, and large-scale validation runs outside the lightweight coding container. | 1. Confirm target runner hardware specifications (Q-N-7).<br>2. Configure and attach GitHub self-hosted runner with NFR-12 isolation tests before running heavy jobs. |

---

## 4. Pre-Commercial Clearance & Data Provenance Status

Per D-56, all research-only datasets are admitted for non-commercial research and development, but must be cleared, replaced, or removed prior to commercial release. In accordance with Finding 17, the canonical ledger IDs from [TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md) are strictly preserved:

| Canonical Item ID | Asset / Dataset Scope | Primary Rights Holder | Current Legal Status & Strategy |
|---|---|---|---|
| **PC-1** | NESO ETYS 2025 Appendix B | NESO (with TO contributions from NGET, SPT, SHET, OFTOs) | NESO general website terms restrict commercial exploitation. Formal written permission request required to NESO Data Portal team for commercial SaaS use of extracted electrical network parameters. |
| **PC-2** | EirGrid / SONI TYTFS 2024 Study Files | EirGrid Group & SONI Ltd | Website disclaimer forbids public/commercial use without authorization. Formal written request required to EirGrid/SONI Open Data and Regulation teams for commercial derivative modelling rights. |
| **PC-3** | SEMO & SEMOpx Market Price Datasets | SEMO & SEMOpx | Standard market report disclaimers; express open licence grant unverified on archive download pages. Seek formal confirmation from SEMO/SEMOpx that published historical settlement and auction reports are covered by group open data terms. |
| **PC-4** | GB Day-Ahead Power Exchange Prices | Nord Pool N2EX & EPEX SPOT | Commercial price-feed and redistribution rights are unverified and may be paid. Under D-57, default to free historical proxies (Elexon Market Index Data and imbalance prices); procure paid commercial exchange feed only if customer funding is secured. |
| **PC-5** | Review of every dataset in provenance manifest marked `research-only` | All applicable rights holders (ENWL, BSC legal text, operator loss specs, Met Éireann, BoE series, CfD/RO, UK ETS auction results, etc.) | Comprehensive review of all research-only datasets: clear with publisher, replace with open alternative, or remove from commercial distribution prior to launch. |
| **PC-6** | Dependency licences for closed SaaS | Software dependency authors (Valkey default under D-67, TimescaleDB TSL, solvers) | Confirm permissiveness of runtime dependencies and solver terms. |
| **PC-7** | Legal review of all of the above | Independent legal counsel | Formal legal review of provenance and licensing clearances prior to commercial launch. |

---

## 5. Input Completeness Assessment & Triage

To replace unsupported readiness conclusions (Findings 9, 18), input categories are assessed using strict, auditable evidence states:
- **Source Identified:** Candidate publisher and product URL documented.
- **Metadata Inspected:** Catalog API, schema, or licensing terms verified.
- **Raw Acquired / Hash-Verified:** Files downloaded and SHA-256 fingerprint verified against manifest outside git.
- **Content Audited:** Internal structure, records, and columns inspected and reconciled.
- **Parameters Extracted:** Model parameters transformed into canonical internal representations.
- **Ingest Validated:** Automated end-to-end ingest and test fixtures verified against acceptance criteria.

| M1 Input Requirement | Candidate Sources | Current Evidence State | Remaining Deficit Backlog |
|---|---|---|---|
| **Dated GB 400/275 kV Network** | `SRC-01` (NESO ETYS App B) | Raw Acquired / Content Audited | Post-fault vs continuous ratings; 2025 forecast vs 2024 baseline; coordinates missing; PC-1 clearance. |
| **Dated Scottish 132 kV Network** | `SRC-01` (ETYS SHE / SPT) | Raw Acquired / Content Audited | Post-fault vs continuous ratings; 2024 baseline reconciliation; PC-1 clearance. |
| **Dated E&W DNO 132 kV Networks (12 areas)** | `SRC-03`–`SRC-11` (DNO LTDS) | Mixed (NGED/SSEN Raw Acquired; UKPN/SPEN/NPg/ENWL Metadata Inspected) | 4 DNO groups require owner logins; unit bases (% vs pu vs ohms) unverified; 132-kV circuit denominators incomplete. |
| **Dated All-Island Ireland Network (400/220/275/110 kV)** | `SRC-12` (TYTFS 2024 RAW/SEQ) | Raw Acquired / Content Audited | Forecast cases (WP/SV) vs operational baseline; 132 zero-rating branches; 3W star conversion; PC-2 clearance. |
| **Interconnector Boundaries & Flows (10 links)** | `SRC-15` (Elexon), `SRC-16a/b` (ENTSO-E) | Raw Acquired (Elexon flows); Metadata Inspected (ENTSO-E) | Greenlink 2024 recorded 7,102 sparse observations (cause unverified; not a fully commissioned 2024 baseline); ENTSO-E personal API token prerequisite; A09/IFA/Nemo PC-5 clearance. |
| **Generation & Storage Fleet Registers** | `SRC-17`–`SRC-21` (TEC, REPD, DUKES, BMU, SEMO) | Raw Acquired / Content Audited | Filtering forward queues to 2024 operational baseline; asset-to-BMU-to-bus crosswalk. |
| **Security & Stability Proxies (Boundary Limits)** | `SRC-22`, `SRC-23` (NESO Constraints) | Raw Acquired / Hash-Verified | Signed electrical branch cut-sets missing; fall-back autumn hours must retain interval IDs. |
| **Stability Proxies (SNSP & Inertia)** | `SRC-24` (EirGrid), `SRC-25` (NESO) | Raw Acquired (SNSP outturns); Identified (Inertia dead link) | SNSP outturns are not ceiling rules; numeric inertia floors and unit groups are UNVERIFIED / missing. |
| **Market Trading & Redispatch Rules** | `SRC-26`–`SRC-28` (BSC, I-SEM, JAO) | Source Identified / Reference-Only | Per ADR 0006 evidence limits, specific BSC clauses and historical 2024 rules archive are not verified; SEMO Part B URL 404; SEMOpx auction rules and JAO border regimes need dated 2024 version pinning; PC-5 clearance. |
| **National & Transmission Demand Outturn** | `SRC-29` (NESO Demand), `SRC-30` (Elexon) | Raw Acquired / Content Audited | Reconciling initial demand outturns against final settlement runs. |
| **Embedded Generation Decomposition** | `SRC-29` (NESO Demand), `SRC-31` (Elexon) | Raw Acquired (NESO Demand & FUELHH) | FUELHH lacks solar/embedded decomposition; gross demand starting identity (ND + embedded solar + embedded wind) pending double-counting and station load verification; Irish embedded data missing. |
| **Nodal / GSP Peak Demand** | `SRC-48` (NESO ETYS App G), `SRC-49` (DNO Table 7) | Source Identified | Peak snapshots must be combined with half-hourly profiles; GSP naming crosswalk and bus attachments missing. |
| **Interconnector Loss Models** | `SRC-32` (Operator specifications) | Source Identified | Directional physical loss parameters for all 10 links are MISSING; uncited percentage defaults removed; converter/cable separation unverified; PC-5 clearance. |
| **Branch Resistances / Loss Segments** | `SRC-01`, `SRC-03`, `SRC-05`, `SRC-12` | Raw Acquired / Content Audited | DC OPF piecewise-affine loss segments must conserve power (ADR 0003). |
| **Settlement Loss Outturns (TLM)** | `SRC-50` (Elexon Settlement Stack API) | Source Identified (Candidate only) | Not in gathered streams; acquisition is UNVERIFIED / missing. |
| **Calendar, Bank Holidays & Clock Changes** | `SRC-33` (GOV.UK), `SRC-34` (IE), `SRC-35` (tzdata) | Metadata Inspected / Arithmetic Verified | Irish holiday access unverified (403); statutory and market settlement calendars must be pinned. |
| **Canonical Scenario Identity Schema** | `INT-01` (Internal specification) | Conceptual Architecture (ADR 0002) | Scenario/input/execution identities, knowledge mode, canonicalisation, versions, and hash vectors checklist added; Rust serde schema, JSON-schema validation, immutable locks, and test fixtures missing. |
| **Fuel & Carbon Commodity Costs** | `SRC-36a/b`–`SRC-38a/b` (ETS, DESNZ, World Bank, LCCC, Ofgem) | Source Identified / Metadata Inspected | UK ETS URL dead (404), licence unverified (PC-5), third-party auction rights unestablished; EU ETS unacquired; WB monthly resampled; LCCC 403; Ofgem RO dataset unidentified; daily SRMC vector missing. |
| **Weather & Renewable Resource Profiles** | `SRC-39`–`SRC-42` (ERA5, MIDAS, MÉRA, Ninja) | Template in Repo; 0 bytes downloaded | Owner CDS credentials required; MÉRA ends 2019; power-curve conversion module unintegrated. |
| **Historical Validation Holdouts (2024)** | `SRC-43` (Elexon 12 streams), `SRC-44` (SEMO) | Raw Acquired / Hash-Verified in Cache | 61 missing 30-min intervals and 9,261 missing 5-min intervals in SEMO; MID is not day-ahead auction price. |
| **Foreign Exchange Rates (EUR/GBP)** | `SRC-46` (ECB), `SRC-47` (BoE) | Metadata Inspected | Forward-fill non-trading days into continuous 366-day UTC series; BoE series unverified. |

---

## 6. Response to Independent Review

### 6.1 Round 1 Verification Disposition Summary (PR #52 Round 1 Report)

The independent verification report for round 1 ([docs/requirements/reviews/gpt-2026-11-14-pr52-m1-inventory.md](../requirements/reviews/gpt-2026-11-14-pr52-m1-inventory.md)) identified 18 findings. As evaluated in the round 2 re-check ([docs/requirements/reviews/gpt-2026-11-14-pr52-m1-inventory-r2.md](../requirements/reviews/gpt-2026-11-14-pr52-m1-inventory-r2.md)), 9 findings were applied at the inventory/planning level in the first revision, and 9 were partially applied. The round 1 disposition table below truthfully reflects those outcomes, with each partial finding fully resolved in revision 2:

| Finding ID | Title / Topic | Round 1 Disposition | Final Resolution & Cross-Reference |
|---|---|---|---|
| **F-01** | Unverified commercial permission labelled cleared | **partial** | Downgraded BSC text, operator loss specs, Irish holiday text, BoE, CfD/RO, and Met Éireann. UK ETS remained partially applied in round 1; fully applied in round 2 under **R2-01** by retracting OGL/open-commercial and assigning `research-only / unverified` (PC-5). |
| **F-02** | Several source URLs do not yield claimed dataset | **applied** | Replaced dead NESO `/dataset/...` redirect URLs with public `package_show` APIs and direct resource URLs from committed manifests. Marked 404 URLs as unresolved; documented SEMO daily register live empty error. |
| **F-03** | FUELHH does not supply embedded solar/wind decomposition | **partial** | Removed solar/embedded-wind claims from FUELHH and pointed GB estimates to NESO columns. Demand formula in round 1 subtracted pumping twice; fully applied in round 2 under **R2-02** by removing double-subtraction and establishing the starting accounting identity. |
| **F-04** | TLM acquisition falsely reported complete | **applied** | Retracted claims of "included within settlement streams" and "downloaded for 2024". Identified Elexon OpenAPI settlement stack (`SRC-50`) as an unacquired candidate to investigate; marked TLM acquisition as UNVERIFIED / missing. |
| **F-05** | Directional physical loss models not sourced per link | **partial** | Added 10-link ledger, but assigned uncited default percentages. Fully applied in round 2 under **R2-03** by replacing all numeric percentages with MISSING, adding directional flow and metering conventions, and separating converter and cable losses. |
| **F-06** | Published proxy rules confused with observations | **applied** | Distinguished realised SNSP outturns from operational ceiling rules; retracted national inertia "~140 GVA·s" narrative approximation; marked boundary cut-sets, regional unit groups, and inertia rules as UNVERIFIED / missing. |
| **F-07** | Dated full-network/asset input not established | **partial** | Corrected 12 E&W + 2 Scottish DNO areas and separated 7 external from 3 internal links. Greenlink test interpretations persisted; fully applied in round 2 under **R2-06** by standardising to 7,102 sparse observations with unverified cause. |
| **F-08** | Irish price source is wrong lookback file; gaps hidden | **applied** | Pinned the two SEMO archives from manifest (`Lookback_Market October2026.zip` and `Imbalance_Price_YTD_2020-2024.zip`). Documented exact measured gaps (61 30-min intervals; 9,261 5-min intervals) and preserved €108.91/MWh UTC-year DAM mean statistic. |
| **F-09** | "Already in repo / downloaded / verified" overclaimed | **partial** | Introduced 6-tier evidence vocabulary. Evidence claims for BMRS reference interconnectors, BSC text, and I-SEM Part B remained overclaimed; fully applied in round 2 under **R2-04** by downgrading to reference-only and distinguishing TEC cached hash from mutable live URL. |
| **F-10** | Fuel/carbon/subsidy coverage claims mismatched | **partial** | Separated projections, commodities, and auctions. UK ETS rights and combined CfD/RO remained partially applied; fully applied in round 2 under **R2-01** and **R2-05** by downgrading UK ETS, splitting products, and adding Ofgem reference. |
| **F-11** | Rule documents not version-pinned | **applied** | Identified Section T for BSC cashout rules; required dated 2024 versions for BSC and SEMO/SEMOpx trading procedures; noted that JAO terms establish platform use, not border-specific capacity-allocation regimes. |
| **F-12** | Deduplicating local fall-back hour discards intervals | **applied** | Corrected clock-transition handling: source interval identifiers, offset/fold, and revisions must be preserved when mapping to UTC. Repeated local timestamps during autumn transition represent distinct physical intervals and must NOT be deduplicated. |
| **F-13** | Calendar sourcing and versioning incomplete | **applied** | Separated civil bank holidays from market trading and settlement calendars. Documented Irish holiday URL as HTTP 403 / unverified access; required pinning tzdata and holiday captures to versioned repository artifacts. |
| **F-14** | Credentials matrix omits sources; overstates outcomes | **partial** | Added CEDA and Renewables.ninja; separated D-65 runner provisioning. Matrix still guaranteed row exports and CIM receipt; fully applied in round 2 under **R2-04** by describing accounts and forms as prerequisites with approvals and payloads unverified. |
| **F-15** | Scenario identity is architectural design, not schema | **partial** | Reclassified scenario definition as internal specification (`INT-01`) and acknowledged missing implementation. Checklist of identity inputs remained incomplete; fully applied in round 2 under **R2-07** by detailing the complete ADR 0002 scenario identity checklist. |
| **F-16** | Weather format and cadence need narrower descriptions | **applied** | Clarified consolidated ERA5 vs near-real-time ERA5T; verified CC BY 4.0 via CDS API; confirmed MÉRA reanalysis covers 1981–Aug 2019 only (GRIB 1 sample) and does not cover 2024; confirmed Renewables.ninja licence is CC BY-NC 4.0. |
| **F-17** | Stable clearance IDs and procurement misrepresented | **applied** | Restored canonical pre-commercial IDs (`PC-1` through `PC-7`) from [TODO-pre-commercial.md](../requirements/TODO-pre-commercial.md). Restored PC-5 as review of all research-only datasets. Clarified that commercial exchange power feeds are unverified and may be paid. |
| **F-18** | Summary statistics and zero-unsourced claim not auditable | **partial** | Replaced zero-unsourced claim with 9-point backlog. Register row counting and publisher aggregation remained unauditable; fully applied in round 2 under **R2-05** with 54 resource rows, 31 normalised publishers, and exact 29/25 totals. |

### 6.2 Round 2 Verification Response Table (PR #52 Round 2 Report, R2-01 through R2-07)

The independent re-check report ([docs/requirements/reviews/gpt-2026-11-14-pr52-m1-inventory-r2.md](../requirements/reviews/gpt-2026-11-14-pr52-m1-inventory-r2.md)) delivered a verdict of **must-fix** with 7 remaining findings (`R2-01` through `R2-07`). Every remaining finding has been evaluated and applied in this second revision:

| Finding ID | Topic / Key Blocker | Disposition | Action Applied in Revision 2 |
|---|---|---|---|
| **R2-01** | UK ETS still has an invented clearance label (`SRC-36a`) | **applied** | Retained `research-only / unverified` (PC-5) for UK ETS Auction Clearing Prices (`SRC-36a`). Retracted OGL v3.0 and `open-commercial` assertions. Clarified that UK Allowance auction data rights held by ICE Futures Europe are not cleared by GOV.UK portal hosting or general website footers. Separated publisher/host (ICE Futures Europe / UK ETS Authority), release (UKA auction clearing), units (£/tCO2e), currency (GBP), and frequency (bi-weekly auctions, not daily spot prices) from licence determination. Updated candidate register row and category details safely without asserting third-party clearance. |
| **R2-02** | New demand equation double-subtracts pumping | **applied** | Removed the incorrect demand equation `ND + embedded solar + embedded wind - pumping`. Sourced official NESO definitions confirming that National Demand (`ND`) already excludes power station transmission-connected load (station load), pumped storage pumping, and interconnector exports (which Transmission System Demand, `TSD`, adds back). Established the starting accounting identity $\text{Gross Underlying Demand} = \text{National Demand} (\text{ND}) + \text{Embedded Solar} + \text{Embedded Wind}$. Qualified that this is a starting accounting identity, not proof that all gross-demand, station load, and loss boundaries are resolved; recorded pumping/storage withdrawals as distinct components; left reconciliation, MWh integration, and double-counting verification test fixtures explicitly pending. |
| **R2-03** | Loss ledger introduces ten unsupported numeric defaults | **applied** | Removed all 10 uncited numeric percentage defaults (e.g. IFA ~2.5%, IFA2 ~2.3%, Greenlink ~2.8%) from the interconnector loss ledger, replacing every total loss factor with **MISSING**. Added explicit directional flow and metering conventions (forward vs reverse flow, sending vs receiving bus boundary) for each link. Separated converter loss and cable loss stages, marking both as MISSING. Preserved the requirement to prevent double-counting against onshore transmission branch losses pending primary operator regulatory filings. |
| **R2-04** | Evidence/access claims still exceed their cited artifacts | **applied** | Downgraded evidence assertions across four areas to match verified artifacts: (1) `SRC-15` downgraded from "raw acquired / hash-verified in cache" to "source identified / reference endpoint verified", since BMRS gathering acquired flow time-series rather than a cached static `/reference/interconnectors/all` payload; (2) `SRC-17` TEC register explicitly distinguishes the cached 10 November 2026 hash from the mutable live CKAN resource URL which redirects to later signed snapshots; (3) `SRC-26` BSC and `SRC-27` I-SEM Part B legal texts downgraded from "rules documented / analysed" to "source identified / reference-only", explicitly acknowledging ADR 0006 evidence limits and the 404 URL for Part B; (4) Section 3 credentials matrix rewritten to treat portal registrations and data requests as prerequisites only, with access permissions, approval turnaround, and actual payload delivery explicitly marked unverified. Retained D-65/NFR-12 isolation tests before heavy jobs. |
| **R2-05** | Source register totals remain unauditable | **applied** | Established an auditable resource-level candidate register (`SRC-01` through `SRC-50`, splitting mixed families into `SRC-16a/b`, `SRC-36a/b`, `SRC-37a/b`, and `SRC-38a/b` for 54 total external candidate resources). Added previously unlisted candidate and backlog sources: `SRC-48` (NESO ETYS Appendix G GSP peak snapshots, `research-only` PC-1), `SRC-49` (DNO Table 7 GSP substation demands, `research-only / unverified` PC-5), and `SRC-50` (Elexon BMRS Settlement Stack API candidate TLM, `open-commercial`). Recomputed exact totals: 29 `open-commercial` (53.7%) and 25 `research-only` (46.3%), plus 2 internal specifications (`INT-01`, `INT-02`). Added an explicit normalised publisher mapping defining all 31 distinct legal/operational publisher entities and their mapped resources. Separated CfD (`SRC-38a`) from RO (`SRC-38b`), adding an official Ofgem scheme reference and noting that the machine-readable dataset location is unidentified. |
| **R2-06** | Greenlink's unverified causal interpretation survives | **applied** | Standardised Greenlink 2024 description across the entire inventory (Executive Summary, `SRC-15` remaining gaps, Interconnector Loss Ledger, Completeness Assessment table, and review response tables) to: `"7,102 sparse observations in 2024; cause unverified; not a fully commissioned 2024 baseline"`. Removed all unverified causal assertions of "commissioning tests", "test operations", and "test records", noting that while observation count and sparsity are verified, operational cause is unverified from flow data alone. |
| **R2-07** | Scenario-input inventory and closure summary need completion | **applied** | Expanded Section 6.1 with an explicit scenario identity and specification checklist cross-referencing ADR 0002 requirements: (1) independent content hashing of scenario input identity and execution identity; (2) knowledge mode distinction between as-of forecast backtests and historical reconstructions; (3) semantic canonicalisation of typed domain entities with permanent opaque UUIDs; (4) three distinct timestamps (effective range `[from, to)`, publication/revision time, acquisition time); (5) version-pinned mappings for calendars/tzdata, transformations, generator cost models, FX vectors, and proxy constraints; (6) rights lineage with PC-1–PC-7 clearance tags; and (7) schema migration vectors with deterministic manifest hashes. Updated Section 6 with a truthful round-1 status table (9 applied, 9 partial) and this dedicated round-2 response table with all 7 remaining findings applied. |
