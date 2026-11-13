# Network and Irish/European Data Gathering Coverage Report

## Overview
This spike gathers network models, boundary capabilities, generator registers, 132 kV DNO parameters, all-island Irish network study cases, and 2024 operational and market results for the GB + Ireland electricity grid simulator (`Tobechukwu-Njoku/energy-grid-simulator`).

In accordance with project decisions:
- **D-52:** Clean provenance with zero AI attribution watermarks.
- **D-56:** Publicly downloadable data without an open commercial licence is gathered for research use and strictly tagged `research-only`. Raw data is stored entirely outside the git working tree in `/Users/njotob/energy-data/raw/<source>/`.
- **D-57:** Market results and pricing use public historical data.
- **D-60:** 132 kV scope covers all 132 kV circuits in GB, including distribution network operators.

### Gathering Statistics
- **Total Datasets Downloaded:** 30
- **Total Raw Data Volume:** 126,027,753 bytes (~120.19 MB)
- **Local Storage Path:** `/Users/njotob/energy-data/raw/` (outside git repository)
- **Manifest:** `spikes/data-gathering/network-ie/manifest.toml`

---

## 1. NESO (National Energy System Operator - GB Transmission)
**Licence:** Open Government Licence v3.0 (`open-commercial`)  
**Storage Directory:** `/Users/njotob/energy-data/raw/neso/`

| Name | File | Size | SHA-256 | Description |
| :--- | :--- | :--- | :--- | :--- |
| **ETYS 2025 Appendix B** | `etys-2025-appendix-b-system-technical-data.xlsx` | 1,237,005 B | `d5cacf238292...` | Complete network workbook: busbars, 400/275/132 kV transmission circuits, transformers, and parameters. |
| **ETYS 2025 Boundary Chart Data** | `etys-2025-boundary-chart-data.xlsx` | 71,901 B | `5199c86cc403...` | Boundary capability, transfer limits, and required capabilities across GB transmission boundaries (B0 to B17). |
| **ETYS 2025 Appendix A** | `etys-2025-appendix-a-system-schematics.pdf` | 4,829,641 B | `417dcbb99254...` | System single-line schematics and geographic topology diagrams. |
| **ETYS 2025 Appendix C** | `etys-2025-appendix-c-power-flows.pdf` | 3,249,024 B | `086236f8220e...` | Peak and minimum demand power flow study results and voltage profiles. |
| **ETYS 2025 Appendix D (Peak)** | `etys-2025-appendix-d-fault-peak.xlsx` | 718,863 B | `ad8b54fa0b05...` | Maximum/peak fault level study calculations and break/make duties. |
| **ETYS 2025 Appendix D (Min)** | `etys-2025-appendix-d-fault-min.xlsx` | 161,789 B | `5f44bdb16504...` | Minimum fault level study calculations. |
| **ETYS 2025 Appendix F** | `etys-2025-appendix-f-contracted-generation.xlsx` | 188,189 B | `1a4d16701ac8...` | Contracted transmission generation background and capacities. |
| **ETYS 2025 Appendix G** | `etys-2025-appendix-g-nodal-demand.xlsx` | 74,494 B | `9f7e66eef563...` | FES nodal demand breakdown across transmission Grid Supply Points. |
| **ETYS Boundary GIS** | `etys-boundary-gis-data-mar25.zip` | 87,714 B | `0e6dbd110a5d...` | GIS shapefiles/boundaries for GB transmission boundaries. |
| **TEC Register** | `tec-register-10-november-2026.csv` | 445,749 B | `0f19d9b411c9...` | Transmission Entry Capacity register of all operational and contracted transmission-connected generators. |
| **Embedded Capacity Register** | `embedded-register-10-november-2026.csv` | 92,230 B | `90bbca6c91c6...` | GB-wide embedded generation register connected to distribution networks. |
| **24-Months Ahead Constraint Limits** | `24-months-ahead-constraint-limit_nov26.csv` | 7,688 B | `3bbb83f6b0f1...` | Boundary capability limits forecast for 24 months ahead. |
| **Day-Ahead Constraint Limits & Flows** | `day-ahead-constraints-limits-and-flow-output-v1.5.csv` | 29,271,088 B | `6406ca6f7e6a...` | Day-ahead transmission boundary constraint flows, limits, and outturns. |
| **Outturn System Costs** | `outturn-system-costs-2024-2025.csv` | 48,346 B | `9a0b431c475e...` | Outturn balancing and constraint costs for 2024-2025. |

---

## 2. DNO 132 kV Long Term Development Statements (LTDS)
**Licence:** Open Government Licence v3.0 / Open Data (`open-commercial`)  
**Storage Directory:** `/Users/njotob/energy-data/raw/dno/`

Under D-60, 132 kV network coverage includes all 14 DNO licence areas in GB.

### 2.1 Obtained via Direct Unauthenticated Download (5 Licence Areas)
- **National Grid Electricity Distribution (NGED - 4 licence areas):**
  - Areas: East Midlands (`EMIDS`), West Midlands (`WMIDS`), South Wales (`SWALES`), South West (`SWEST`).
  - **Tabular Data:**
    - `ltds-table-1-circuit.csv` (2,663,962 B): Circuit electrical parameters (resistance, reactance, susceptance, ratings) for 132 kV, 66 kV, and 33 kV across all 4 NGED areas.
    - `ltds-table-2a-two-winding-transformer.csv` (308,177 B): Two-winding transformer parameters.
    - `ltds-table-2b-three-winding-transformer.csv` (11,279 B): Three-winding transformer parameters.
    - `ltds-attachment-8-node.csv` (1,826,097 B): Substation and node electrical bus mappings.
  - **CIM Models (IEC 61970 / CIM XML):**
    - `nged-cim-emids.zip` (11,166,470 B): Full CIM network model for East Midlands.
    - `nged-cim-wmids.zip` (7,414,763 B): Full CIM network model for West Midlands.
    - `nged-cim-swales.zip` (5,305,393 B): Full CIM network model for South Wales.
    - `nged-cim-swest.zip` (8,297,269 B): Full CIM network model for South West.
- **SSEN Distribution - Southern Electric (SEPD - 1 licence area):**
  - Area: Southern England (`SEPD`).
  - **Tabular Data:**
    - `southern-electric-power-distribution-long-term-development-statement-tables_may-2026.xlsx` (730,943 B): Complete LTDS Tables 1–7, including Table 1 132 kV circuits and Table 2 transformers.
  - **CIM Model:**
    - `ltds_sepd_2025-02_eq_2025-10-20_v1.0.zip` (5,599,633 B): CIM Equipment profile (EQ) for Southern Electric.

### 2.2 Scottish Boundary Note (2 Licence Areas)
- **SSEN SHEPD (Scottish Hydro Electric Power Distribution):** In northern Scotland, 132 kV is transmission infrastructure owned by Scottish Hydro Electric Transmission (SHET) and operated under NESO. Consequently, SHEPD distribution LTDS does not contain 132 kV; 132 kV network parameters for this region are fully captured in **NESO ETYS Appendix B**.
- **SPEN SPD (SP Distribution - Central and Southern Scotland):** In southern/central Scotland, 132 kV is transmission infrastructure owned by SP Transmission (SPT). SP Distribution does not hold 132 kV assets; all Scottish 132 kV network parameters are fully captured in **NESO ETYS Appendix B**.

### 2.3 Licences Requiring Portal Login ("Needs Owner Login") (7 Licence Areas)
In accordance with instructions, no accounts were created during research.
1. **UK Power Networks (UKPN - 3 licence areas):**
   - Areas: Eastern Power Networks (`EPN`), London Power Networks (`LPN`), South Eastern Power Networks (`SPN`).
   - Portal: OpenDataSoft (`https://ukpowernetworks.opendatasoft.com/`).
   - Status: **Needs owner login**. Unauthenticated API record requests return `403 ForbiddenAccess`; unauthenticated CSV export downloads return only the header line (0 data records). Full LTDS Table 1 circuit records require an authenticated portal account.
2. **SP Energy Networks - SP Manweb (SPEN SPM - 1 licence area):**
   - Area: Merseyside and North Wales (`SPM`).
   - Portal: OpenDataSoft (`https://spenergynetworks.opendatasoft.com/`).
   - Status: **Needs owner login**. Unauthenticated calls return 1-line header files without data rows. Full LTDS Appendix 1 / Table 1 data requires portal login.
3. **Northern Powergrid (NPg - 2 licence areas):**
   - Areas: Northeast (`NEDL`) and Yorkshire (`YEDL`).
   - Portal: OpenDataSoft (`https://northernpowergrid.opendatasoft.com/`).
   - Status: **Needs owner login**. Unauthenticated CSV exports return only headers. Full LTDS Appendix 3 data requires portal login.
4. **Electricity North West (ENWL - 1 licence area):**
   - Area: North West England (`ENWL`).
   - Portal: ENWL LTDS Secure Customer Portal (`https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/login-to-LTDS-secure-area/`).
   - Status: **Needs owner login**. Gated behind customer login and restricted SP ENW Shared Licence.

---

## 3. EirGrid / SONI (All-Island Transmission Network & Operational Data)
**Licence:** Public operational & study data (`research-only` under D-56)  
**Storage Directory:** `/Users/njotob/energy-data/raw/eirgrid/`

| Name | File | Size | SHA-256 | Description |
| :--- | :--- | :--- | :--- | :--- |
| **All-Island TYTFS 2024 Study Files** | `TYTFS2024_studyfiles.zip` | 11,954,215 B | `1dae320fa3b5...` | Full all-island network study cases in Siemens PTI PSS/E RAW format (buses, branches, transformers, generators, HVDC links) and SEQ sequence data archives for Summer Peak, Summer Valley, Winter Peak, and Autumn scenarios. |
| **All-Island 2024 Quarter-Hourly System Data** | `System-Data-Qtr-Hourly-2024.xlsx` | 12,101,550 B | `27b01a5b1fbc...` | Complete 2024 SCADA operational dataset (35,136 quarter-hourly intervals): Republic of Ireland demand (`IEDemand`), Northern Ireland demand (`NIDemand`), wind generation, solar PV generation, interconnector physical flows (EWIC, Moyle), and System Non-Synchronous Penetration (SNSP). |
| **System & Renewable Summary Report** | `System-and-Renewable-Data-Summary-Report-V30.xlsx` | 621,212 B | `5d3f7d1eb928...` | Monthly and annual operational summary metrics for renewable integration and system performance. |

---

## 4. SEMO / SEMOpx (All-Island Market Results)
**Licence:** Public market publications (`research-only` under D-56)  
**Storage Directory:** `/Users/njotob/energy-data/raw/semo/`

| Name | File | Size | SHA-256 | Description |
| :--- | :--- | :--- | :--- | :--- |
| **SEMOpx Historical Market Lookback** | `lookback_mkt.xlsx` | 16,788,754 B | `e650ff08de43...` | Comprehensive historical market outcomes: Day-Ahead Market (DAM) hourly clearing prices, trading volumes, Intraday Auction (IDA1, IDA2, IDA3) clearing prices, and harmonised reference prices across Ireland and Northern Ireland. |
| **SEMO Balancing Market Monthly Registered Capacity** | `PUB_MnlyRegisteredCapacity_202412011000.xml` | 464,148 B | `15b4a26f344d...` | All-island registered capacity for all participating generating units in the SEM balancing market (December 2024). |
| **SEMO Balancing Market Registered Units** | `PUB_DailyRegisteredUnits_202609140734.xml` | 290,167 B | `b344bb639cdc...` | Master list of all registered trading parties and balancing units in the Single Electricity Market. |

---

## 5. ENTSO-E Transparency Platform
**Status:** **Needs owner token**

- **Day-Ahead Prices (Ireland SEM) & Cross-Border Flows (GB/IE Interconnectors):**
  - **Platform URL:** `https://transparency.entsoe.eu/`
  - **RESTful API Endpoint:** `https://web-api.tp.entsoe.eu/api`
  - **Authentication Mechanism:** Every RESTful API request requires a valid `securityToken` query parameter. Requests without a security token return `401 Unauthorized`.
  - **Token Acquisition Procedure:** Requires registering a personal user account on `transparency.entsoe.eu`, then sending an email request to `transparency@entsoe.eu` with subject "RESTful API access". Once approved by the helpdesk, a token can be generated under user profile settings.
  - **Manual Web Export:** The interactive web portal allows viewing charts and tables, but exporting data (CSV/XLSX) requires an active authenticated session.
  - **Compliance with Directives:** Per user instruction ("The API needs a security token; if none is available, do NOT register an account. Note 'needs owner token'"), no account was created and no unauthorised token was generated.

---

## 6. How to Reproduce or Update Data Gathering
To re-run the automated gathering process:
```bash
python3 spikes/data-gathering/network-ie/fetch.py
```
To inspect target URLs without downloading:
```bash
python3 spikes/data-gathering/network-ie/fetch.py --dry-run
```
All downloaded files will be validated and re-hashed into `spikes/data-gathering/network-ie/manifest.toml`.
