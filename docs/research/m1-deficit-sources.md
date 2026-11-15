# M1 Deficit Sources and Mitigation Analysis

This document addresses the 9-point deficit backlog identified in milestone M1 ([docs/research/m1-data-inventory.md](file:///home/njotob/.t3/projects/Dev%20-%20GitHub%20-%20Energy/t3-a292309c-b05197e0fb92a052/docs/research/m1-data-inventory.md)). For each deficit, up to 3 candidate public sources (from TSOs, DNOs, regulators, government agencies, and academic repositories) are evaluated for licensing compliance (D-56), access requirements, and coverage. A single primary recommendation or proxy/assumption is provided per deficit.

Per [AGENTS.md](file:///home/njotob/.t3/projects/Dev%20-%20GitHub%20-%20Energy/t3-a292309c-b05197e0fb92a052/AGENTS.md) Rule 10 and D-56, Open Database License (ODbL) and OpenStreetMap (OSM) datasets are strictly excluded as unusable due to viral share-alike provisions. Any term, schema attribute, or availability detail not confirmed directly on the publisher's published portal or documentation page is explicitly designated as **UNVERIFIED**.

---

## 1. Commissioned 2024 Network Fleet & Asset Denominators

### Deficit Description
ETYS 2025/26 and DNO 2025/26 releases represent forward-looking planning snapshots rather than historical 2024 commissioned asset baselines. GSP and BMU crosswalks, rating definitions (post-fault vs continuous / seasonal pre-fault), unit bases (% on 100 MVA vs pu on 100 MVA vs ohms), and 132-kV circuit denominators across all 14 DNO licence areas remain incomplete or unharmonised.

### Candidate Sources

#### Candidate 1.1: NESO ETYS 2024 Appendix B (System Technical Data Archive)
- **URL:** [https://www.neso.energy/etys-documents-and-appendices](https://www.neso.energy/etys-documents-and-appendices)
- **Publisher:** National Energy System Operator (NESO)
- **Licence Text / Link:** Governed by general [NESO Website Terms and Conditions](https://www.neso.energy/terms-and-conditions) (all rights reserved; commercial exploitation restricted without written consent).
- **Likely D-56 Class:** `research-only` (PC-1)
- **Access Method:** Open direct download (Excel `.xlsx` workbook)
- **Coverage:** Great Britain (400 kV, 275 kV, Scottish 132 kV); Historical 2023/24 baseline with forward planning horizons.
- **Evaluation:** Provides transmission line parameters (resistance $R$, reactance $X$, susceptance $B$), continuous/post-fault seasonal ratings, and transformer impedance data as configured for the 2023/24 operational year. Does not include England & Wales 132 kV distribution circuits or geographic bus coordinates.

#### Candidate 1.2: Elexon BMRS BM Unit Reference & GSP Group Register APIs
- **URL:** [https://data.elexon.co.uk/bmrs/api/v1/reference/bmunits/all](https://data.elexon.co.uk/bmrs/api/v1/reference/bmunits/all) and [https://data.elexon.co.uk/bmrs/api/v1/reference/gspgroups](https://data.elexon.co.uk/bmrs/api/v1/reference/gspgroups)
- **Publisher:** Elexon Limited
- **Licence Text / Link:** [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/) (permits commercial and non-commercial reproduction with attribution).
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open REST API (JSON format; no authentication credentials required)
- **Coverage:** Great Britain; operational 2024 Balancing Mechanism units mapped to Lead Parties, National Grid BMU identifiers, fuel types, transmission capacities, and GSP Group affiliations (`_A` through `_P`).
- **Evaluation:** Authoritative crosswalk bridging Balancing Mechanism generation and demand units to Grid Supply Points and regional DNO boundaries. Does not provide physical electrical line branch models or substation busbar topologies.

#### Candidate 1.3: DNO Long Term Development Statements (LTDS November 2024 Editions)
- **URL:** Energy Networks Association (ENA) and individual DNO portals:
  - NGED: [https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model](https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model)
  - SSEN: [https://data.ssen.co.uk/@ssen-distribution/sepd_long_term_development_statement](https://data.ssen.co.uk/@ssen-distribution/sepd_long_term_development_statement)
  - UKPN: [https://ukpowernetworks.opendatasoft.com/explore/assets/ukpn-ltds-table-1-circuit-data/](https://ukpowernetworks.opendatasoft.com/explore/assets/ukpn-ltds-table-1-circuit-data/)
  - SPEN: [https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-appendix-1-circuit-data-table-1/](https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-appendix-1-circuit-data-table-1/)
  - NPg: [https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-appendix-3/information/](https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-appendix-3/information/)
  - ENWL: [https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/login-to-ltds-secure-area/](https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/login-to-ltds-secure-area/)
- **Publisher:** 6 UK DNO Group Licensees (covering all 14 DNO licence areas: 12 England/Wales 132 kV distribution areas and 2 Scottish distribution areas)
- **Licence Text / Link:** Mixed: NGED Open Data Licence (`open-commercial`), SSEN CC BY 4.0 (`open-commercial`), UKPN/SPEN/NPg portal registration terms (`open-commercial` metadata, login prerequisite), ENWL Shared Data Licence (`research-only`, PC-5).
- **Likely D-56 Class:** Mixed (`open-commercial` for NGED and SSEN; `research-only` for ENWL and CIM models; UNVERIFIED for gated portal row extractions).
- **Access Method:** Open direct download (NGED, SSEN) / Free registered user login (UKPN, SPEN, NPg) / Formal registration request (ENWL).
- **Coverage:** Great Britain; 132 kV circuits (Table 1 / Appendix 3) and 132 kV transformers (Table 2 / Appendix 4) as reported under standard licence condition SLC 25B in November 2024.
- **Evaluation:** Necessary for 132 kV distribution impedances and equipment ratings across England & Wales. However, units differ across DNOs (SPM reports % on 100 MVA, others report ohms or pu), and individual switchgear segments lack impedances.

### Recommendation & Proxy Formulation
- **Recommended Candidate:** Combine **NESO ETYS 2024 Appendix B** (`Candidate 1.1`) for GB transmission lines with **Elexon BMRS BM Unit API** (`Candidate 1.2`) for 2024 GSP/BMU asset crosswalks, supplemented by **NGED and SSEN LTDS 2024 Table 1/2** (`Candidate 1.3`) for open-commercial 132 kV distribution.
- **Standard Engineering Proxy / Assumptions:**
  1. *Unit Base Harmonisation:* Convert all branch impedances to standard per-unit on a 100 MVA system base ($S_b = 100\text{ MVA}$). For branches given in physical ohms at nominal voltage $V_b = 132\text{ kV}$, base impedance is $Z_{\text{base}} = V_b^2 / S_b = (132^2) / 100 = 174.24\ \Omega$. For branches given in percentage on 100 MVA, divide by 100.
  2. *Continuous vs Post-Fault Rating Proxy:* Where DNO tables report only a single post-fault or emergency rating (e.g. 1-hour or 3-minute overload rating), assume the continuous operational thermal rating is $0.88 \times \text{Post-Fault Rating}$ (based on standard ENA Engineering Recommendation P27 overhead line thermal envelopes).

---

## 2. Proxy Rule Parameters and Cut-Sets

### Deficit Description
NESO boundary constraint groups lack explicit signed electrical branch cut-sets in machine-readable format. Furthermore, regional inertia floors, unit stability groups, minimum-unit requirements, and Irish System Non-Synchronous Penetration (SNSP) operational limits are often cited from realized outturn statistics rather than documented operational rules.

### Candidate Sources

#### Candidate 2.1: EirGrid & SONI Operational Constraints Update (Monthly / Weekly Releases)
- **URL:** [https://www.sem-o.com/publications/](https://www.sem-o.com/publications/) and [https://www.eirgrid.ie/customer-and-community/operational-constraints](https://www.eirgrid.ie/customer-and-community/operational-constraints)
- **Publisher:** EirGrid Group & SONI Ltd
- **Licence Text / Link:** SEMO and EirGrid General Website Disclaimers ([EirGrid Disclaimer](https://www.eirgrid.ie/disclaimer); copyright all rights reserved).
- **Likely D-56 Class:** `research-only` (PC-2 / PC-3)
- **Access Method:** Open direct download (PDF reports published regularly)
- **Coverage:** All-island Ireland (ROI and NI); calendar year 2024 and ongoing operational horizon.
- **Evaluation:** Highly authoritative operational rulebook. Formally codifies:
  - All-island SNSP operational ceiling limit: fixed at **75%** throughout 2024.
  - Rate of Change of Frequency (RoCoF) limit: **1.0 Hz/s** (over a 500 ms rolling window).
  - System Inertia Floor: minimum **23,000 MW·s** all-island floor.
  - Jurisdictional Minimum Units: minimum 5 large synchronous units in Ireland, minimum 3 synchronous units in Northern Ireland.
  - Area Constraint Groups (TCGs): explicit unit groups and export/import limits for Dublin, North-South tie-line, and North-West stability constraints.

#### Candidate 2.2: NESO Day-Ahead Constraints Limits & 24-Months-Ahead Limits
- **URL:** [https://api.neso.energy/api/3/action/package_show?id=1e2be3b2-e64e-4e42-b062-841f39f8df5b](https://api.neso.energy/api/3/action/package_show?id=1e2be3b2-e64e-4e42-b062-841f39f8df5b) (Day-Ahead Constraints) and [https://api.neso.energy/api/3/action/package_show?id=8610ea09-90d7-4638-b7ba-bdfb8e7df009](https://api.neso.energy/api/3/action/package_show?id=8610ea09-90d7-4638-b7ba-bdfb8e7df009) (24-Months-Ahead)
- **Publisher:** National Energy System Operator (NESO)
- **Licence Text / Link:** [National Energy SO Open Data Licence v1.0](https://www.neso.energy/data-portal/neso-open-licence)
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open CKAN REST API / direct CSV download
- **Coverage:** Great Britain; half-hourly transfer capability limits (MW) for major transmission boundaries (e.g., B6 Anglo-Scottish, B7 Upper North, B8 North to Midlands, B14 London) for 2024.
- **Evaluation:** Provides numerical boundary transfer capacities applied in daily scheduling and redispatch. However, it specifies the aggregate group MW limit, omitting the explicit member branch cut-set definitions.

#### Candidate 2.3: NESO Frequency Risk and Control Report (FRCR) & System Operability Framework
- **URL:** [https://www.neso.energy/industry-information/codes/grid-code](https://www.neso.energy/industry-information/codes/grid-code) and [https://www.neso.energy/document/280926/download](https://www.neso.energy/document/280926/download)
- **Publisher:** National Energy System Operator (NESO)
- **Licence Text / Link:** [NESO Terms and Conditions](https://www.neso.energy/terms-and-conditions) (statutory Grid Code documentation).
- **Likely D-56 Class:** `research-only` (PC-5)
- **Access Method:** Open direct download (PDF publications)
- **Coverage:** Great Britain; national frequency regulation, ROCOF risk requirements, and regional stability constraints.
- **Evaluation:** Formally defines the national inertia floor (baseline target ~130–140 GVA·s to limit RoCoF to 0.5–1.0 Hz/s under single-infeed loss) and regional stability unit requirements (e.g. minimum units in South Wales, Scotland, and London for dynamic voltage support).

### Recommendation & Proxy Formulation
- **Recommended Candidate:**
  - For Ireland: **EirGrid & SONI Operational Constraints Update** (`Candidate 2.1`) provides complete, verified operational ceilings (75% SNSP, 23,000 MW·s inertia, jurisdictional minimum unit counts).
  - For GB: **NESO Day-Ahead Constraints Limits** (`Candidate 2.2`) for numerical MW transfer limits, paired with FRCR (`Candidate 2.3`) for inertia floors.
- **Topological Branch Cut-Set Proxy:**
  - To derive explicit signed branch cut-sets for GB boundaries: intersect the physical network transmission branch graph (from ETYS Appendix B) with the boundary GIS geometry polygons from `SRC-02` (NESO ETYS Boundary GIS Data, `open-commercial`).
  - *Cut-Set Sign Convention:* For each intersecting circuit connecting bus $i$ (coordinates $(\text{lat}_i, \text{lon}_i)$) and bus $j$ (coordinates $(\text{lat}_j, \text{lon}_j)$), assign a positive flow direction if flowing from the northern/exporting side to the southern/importing side ($+1$ if $\text{lat}_i > \text{lat}_j$, $-1$ if $\text{lat}_i < \text{lat}_j$). The boundary flow is:
    $$F_{\text{boundary}} = \sum_{b \in \mathcal{C}_{\text{boundary}}} \text{sign}(b) \cdot P_{b}$$

---

## 3. Embedded Generation Decomposition & Demand Equation

### Deficit Description
Elexon FUELHH outturn records report transmission-connected generation only and do not provide embedded (distribution-connected) solar or wind production. Embedded generation profiles must be combined with transmission demand to establish gross underlying demand. In NESO Historic Demand records, National Demand ($ND$) already excludes power station load, pumped storage pumping, and interconnector exports (which Transmission System Demand, $TSD$, adds back). Double-counting checks, station-load reconciliation, and Irish embedded solar/wind decomposition remain missing.

### Candidate Sources

#### Candidate 3.1: Sheffield Solar PV_Live API
- **URL:** [https://www.solar.sheffield.ac.uk/pvlive/](https://www.solar.sheffield.ac.uk/pvlive/) and API Portal: [https://api.solar.sheffield.ac.uk/pvlive/api/v4/](https://api.solar.sheffield.ac.uk/pvlive/api/v4/)
- **Publisher:** Sheffield Solar (University of Sheffield, funded by NESO)
- **Licence Text / Link:** [Creative Commons Attribution 4.0 International (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/) per official terms ([Sheffield Solar Terms](https://api.solar.sheffield.ac.uk/)).
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open REST API (JSON format; no authentication key required for standard queries)
- **Coverage:** Great Britain; half-hourly embedded solar generation outturns and capacity estimates (MW) national aggregate, DNO region, and GSP Group levels for 2024.
- **Evaluation:** The industry-standard reference for unmetered embedded solar generation in GB. Direct input into NESO control room forecasting. Provides audited half-hourly estimates with clear attribution terms. Does not cover Northern Ireland or the Republic of Ireland.

#### Candidate 3.2: NESO Data Portal Historic Demand Data 2024
- **URL:** [https://api.neso.energy/api/3/action/package_show?id=888229c4-810e-477d-93cb-39f38f72a910](https://api.neso.energy/api/3/action/package_show?id=888229c4-810e-477d-93cb-39f38f72a910)
- **Publisher:** National Energy System Operator (NESO)
- **Licence Text / Link:** [National Energy SO Open Data Licence v1.0](https://www.neso.energy/data-portal/neso-open-licence)
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open CKAN API / direct CSV download
- **Coverage:** Great Britain; continuous 17,568 half-hourly settlement periods for 2024.
- **Evaluation:** Authoritatively reports `ND` (National Demand), `TSD` (Transmission System Demand), `ENGLAND_WALES_DEMAND`, `EMBEDDED_SOLAR_GENERATION`, `EMBEDDED_WIND_GENERATION`, and `PUMP_STORAGE_PUMPING`. Confirms official operational demand accounting boundaries.

#### Candidate 3.3: ESB Networks & SEAI Microgeneration Registers (All-Island Embedded Capacity)
- **URL:** [https://www.esbnetworks.ie/](https://www.esbnetworks.ie/) and [https://www.seai.ie/data-and-insights/seai-statistics/key-statistics/renewables/](https://www.seai.ie/data-and-insights/seai-statistics/key-statistics/renewables/)
- **Publisher:** ESB Networks & Sustainable Energy Authority of Ireland (SEAI)
- **Licence Text / Link:** SEAI Open Data / Government of Ireland Open Data Licence (OGL Ireland / CC BY 4.0); ESB Networks copyright disclaimers.
- **Likely D-56 Class:** `open-commercial` (SEAI statistical reports) / `research-only` (ESB corporate updates)
- **Access Method:** Open direct download (annual reports and periodic statistical CSV extracts)
- **Coverage:** Republic of Ireland; cumulative registered microgeneration capacity (reaching 498 MW across 121,800 domestic/commercial installations by year-end 2024).
- **Evaluation:** Verifies installed capacity baselines for behind-the-meter embedded solar in Ireland. However, neither ESB Networks nor EirGrid publishes half-hourly metered or estimated outturn time series for embedded microgeneration.

### Recommendation & Proxy Formulation
- **Recommended Candidate:**
  - For GB: Use **NESO Historic Demand Data 2024** (`Candidate 3.2`) as primary demand identity inputs, verified against **Sheffield Solar PV_Live** (`Candidate 3.1`) for embedded solar profiles.
  - For Ireland: Use **SEAI / ESB Networks** (`Candidate 3.3`) capacity baselines scaled by solar radiation proxies.
- **Demand Accounting Identity Formulation:**
  As verified from NESO settlement documentation, National Demand ($ND$) already excludes station load, interconnector exports, and pumped storage pumping (which Transmission System Demand, $TSD$, adds back). Thus:
  $$ND = TSD - \text{Interconnector Exports} - \text{Pumped Storage Pumping} - \text{Station Load}$$
  The starting gross underlying demand identity for GB is:
  $$\text{Gross Underlying Demand} = ND + \text{Embedded Solar} + \text{Embedded Wind}$$
  Pumped storage pumping is tracked as a distinct flexible storage demand asset, preventing double-subtraction.
- **Irish Embedded Generation Proxy:**
  Because EirGrid Smart Grid Dashboard reports only grid-connected utility-scale wind and solar, synthesize half-hourly Irish embedded solar via physical capacity scaling:
  $$P_{\text{embedded\_solar, IE}}(t) = C_{\text{micro\_PV, IE}}(2024) \times \frac{P_{\text{utility\_solar, IE}}(t)}{C_{\text{utility\_solar, IE}}(2024)}$$
  where $C_{\text{micro\_PV, IE}}(2024) \approx 498\text{ MW}$ and $C_{\text{utility\_solar, IE}}(2024) \approx 1,000\text{ MW}$ (from ESB Networks and EirGrid 2024 milestone reports).
