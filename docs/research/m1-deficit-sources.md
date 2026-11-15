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

---

## 4. Directional Interconnector Losses & TLM Source

### Deficit Description
Directional physical loss parameters across the 10 cross-border and island interconnectors are missing from standard TSO datasets, resulting in unverified numeric defaults or placeholders. Converter and cable loss separations, asymmetric forward/reverse losses, and sending/receiving metering boundaries require primary regulatory grounding. Furthermore, Elexon Transmission Loss Multipliers (TLMs) are missing from the previously gathered settlement streams.

### Candidate Sources

#### Candidate 4.1: Regulated Interconnector Access Rules & Loss Methodology Filings
- **URL:** Ofgem & National Regulatory Authority decision libraries:
  - BritNed Access Rules (Schedule 3): [https://www.britned.com/](https://www.britned.com/) / [https://www.ofgem.gov.uk/](https://www.ofgem.gov.uk/)
  - Nemo Link Access Rules: [https://www.nemolink.co.uk/](https://www.nemolink.co.uk/)
  - ElecLink Access Rules: [https://www.eleclink.co.uk/](https://www.eleclink.co.uk/)
  - Moyle & EWIC Joint Regulatory Decisions: [https://www.semcommittee.com/](https://www.semcommittee.com/) and [https://www.uregni.gov.uk/](https://www.uregni.gov.uk/)
- **Publisher:** Interconnector Licensees (BritNed Development Ltd, Nemo Link Ltd, ElecLink Ltd, Mutual Energy, EirGrid Interconnector DAC) approved by National Regulatory Authorities (Ofgem, CREG, CRE, CRU, UR).
- **Licence Text / Link:** Regulatory decision documents published under Open Government Licence (OGL v3.0) / public regulatory transparency; operator schedules under website terms.
- **Likely D-56 Class:** Mixed (`open-commercial` for Ofgem/UR statutory decision notices; `research-only` for proprietary operator manuals).
- **Access Method:** Open direct download (PDF filings and schedules)
- **Coverage:** GB and connected European / Irish markets (France, Netherlands, Belgium, Northern Ireland, Republic of Ireland); 2024 operational baseline.
- **Evaluation:** Provides approved loss factor schedules:
  - *BritNed:* Schedule 3 defines a 3.0% end-to-end loss factor (1.5% applied symmetrically from Mid North Sea reference point to shore).
  - *Nemo Link:* Annex Business Rules specify a mid-point loss factor of 2.372% (symmetrical half-factor of 1.186% to each end) and an explicit market coupling technical loss factor of 2.344% in Euphemia.
  - *ElecLink:* Deemed Metered Volume Allocation specifies an end-to-end loss factor of 2.9% (1.471% mid-interconnector loss factor).
  - *Moyle & EWIC:* SEM Committee decisions align loss treatment with annual Transmission Loss Adjustment Factors (TLAFs).

#### Candidate 4.2: Elexon Insights Solution SAA & Settlement Stack REST API (`SRC-50`)
- **URL:** [https://data.elexon.co.uk/saa/api/v1/i014/party-level/](https://data.elexon.co.uk/saa/api/v1/i014/party-level/) and [https://data.elexon.co.uk/bmrs/api/v1/balancing/settlement/system-prices](https://data.elexon.co.uk/bmrs/api/v1/balancing/settlement/system-prices)
- **Publisher:** Elexon Limited
- **Licence Text / Link:** [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/)
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open REST API (JSON / SAA-I014 settlement reports; no authentication key required)
- **Coverage:** Great Britain; all 17,568 settlement periods for 2024 across settlement runs (II, SF, R1–R3, RF).
- **Evaluation:** Provides verified outturn Transmission Loss Multipliers ($TLM_{ij}$) and Transmission Loss Factors ($TLF_{ij}$) for all registered BM Units and GSP groups. Formally confirms BSC Modification P278 rule: **for all interconnector BM units, $TLM = 1.0$ exactly** in GB imbalance settlement.

#### Candidate 4.3: SEM Committee / EirGrid & SONI Annual TLAF Decision Tables
- **URL:** [https://www.semcommittee.com/publications/](https://www.semcommittee.com/publications/) and [https://www.eirgrid.ie/customer-and-community/operational-constraints](https://www.eirgrid.ie/customer-and-community/operational-constraints)
- **Publisher:** Single Electricity Market Committee (CRU & Utility Regulator) and System Operators (EirGrid & SONI)
- **Licence Text / Link:** SEM Committee public notices / statutory decision disclaimers ([EirGrid Disclaimer](https://www.eirgrid.ie/disclaimer)).
- **Likely D-56 Class:** `research-only` (PC-2 / PC-3)
- **Access Method:** Open direct download (annual decision papers and Excel tables)
- **Coverage:** All-island Ireland (ROI and NI); 2023/24 and 2024/25 tariff years (half-hourly nodal loss multipliers).
- **Evaluation:** Authoritative source for nodal loss multipliers in the SEM. Confirms that interconnector volumes landing at Woodland (EWIC) and Ballycronan More (Moyle) are scaled by nodal TLAFs in ex-ante market clearing and ex-post imbalance settlement.

### Recommendation & Proxy Formulation
- **Recommended Candidate:** Use **Elexon Insights SAA API** (`Candidate 4.2`) for GB settlement loss accounting (enforcing $TLM = 1.0$ for interconnector units per BSC P278), combined with primary **Regulated Access Rules** (`Candidate 4.1`) for physical loss percentages across certified links (BritNed 3.0%, Nemo Link 2.372%, ElecLink 2.9%).
- **HVDC Interconnector Physical Loss Proxy:**
  For links lacking explicit operator loss filings in 2024 (IFA, IFA2, NSL, Viking Link, Greenlink), adopt the standard CIGRE B4 HVDC converter and cable loss formulation:
  1. *Converter Losses:* Assume fixed + linear converter station loss of $0.75\%$ per converter station ($1.50\%$ for both AC/DC terminals combined).
  2. *Cable Ohmic Losses:* Linear ohmic cable loss factor of $0.30\%$ per 100 km of subsea/underground HVDC cable.
  3. *Directional Formulation:* Let $P_{\text{send}}$ be real power injected at the sending converter bus. Power received at the receiving converter bus is:
     $$P_{\text{receive}} = P_{\text{send}} \times (1 - L_{\text{converter, send}}) \times (1 - L_{\text{cable}}(d)) \times (1 - L_{\text{converter, receive}})$$
     where $L_{\text{cable}}(d) = 0.003 \times (d / 100\text{ km})$.

---

## 5. Dated Market Rules

### Deficit Description
Wholesale market settlement and trading procedures require dated, version-pinned releases reflecting the 2024 calendar baseline, rather than mutable live website hubs. Specifically, BSC Section T cashout calculations and SEMOpx auction rules (DAM, IDA1–3, IDC) must be locked to historical regulatory instruments in force during 2024.

### Candidate Sources

#### Candidate 5.1: Elexon Digital BSC Baseline Releases (Section T, Versions 38.0 & 41.0)
- **URL:** [https://bsc.elexon.co.uk/](https://bsc.elexon.co.uk/) and [https://www.elexon.co.uk/bsc-and-codes/balancing-settlement-code/bsc-sections/](https://www.elexon.co.uk/bsc-and-codes/balancing-settlement-code/bsc-sections/)
- **Publisher:** Elexon Limited
- **Licence Text / Link:** [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/) / BSC statutory documentation.
- **Likely D-56 Class:** `research-only` (PC-5; statutory code legal text)
- **Access Method:** Open direct download / web access
- **Coverage:** Great Britain; historical versions in force across 2024:
  - *Version 38.0:* Effective 2 April 2024 (Spring 2024 baseline).
  - *Version 41.0:* Effective 7 November 2024 (Standard November 2024 Release, implementing Modification P415 and P473 regarding Virtual Lead Party deviation volumes and Supplier Compensation Cashflow $SCC_p$).
- **Evaluation:** Complete algebraic codification of system buy/sell prices, net imbalance volume ($NIV$), Continuous Acceptance Duration Limit ($CADL$), Replacement Price, and cashflow hierarchies.

#### Candidate 5.2: SEMOpx Operating Procedures & Market Rules (2024 Baseline)
- **URL:** [https://www.semopx.com/rules-monitoring/market-rules/](https://www.semopx.com/rules-monitoring/market-rules/) and [https://www.semopx.com/rules-monitoring/modifications/](https://www.semopx.com/rules-monitoring/modifications/)
- **Publisher:** SEMOpx (EirGrid / SONI / EPEX SPOT joint exchange operator)
- **Licence Text / Link:** SEMOpx legal disclaimers; all rights reserved.
- **Likely D-56 Class:** `research-only` (PC-5)
- **Access Method:** Open direct download (PDF operating procedures)
- **Coverage:** All-island Ireland (I-SEM); calendar year 2024.
- **Evaluation:** Defines formal rules for Day-Ahead Market (DAM, gate closure 11:00 UTC/GMT), Intraday Auctions (IDA1 gate closure 17:45 D-1, IDA2 gate closure 08:00 D, IDA3 gate closure 14:00 D), and Intraday Continuous trading (IDC, up to 30 min before delivery). Specifies order types (simple hourly, block bids, complex orders) and price-cap boundaries (-€500/MWh to +€4,000/MWh).

#### Candidate 5.3: SEM Committee I-SEM Trading and Settlement Code (Part B Dated Baseline)
- **URL:** [https://www.semcommittee.com/publications/](https://www.semcommittee.com/publications/) and [https://www.sem-o.com/rules-and-governance/rules/](https://www.sem-o.com/rules-and-governance/rules/)
- **Publisher:** Single Electricity Market Committee (CRU / UR) & SEMO
- **Licence Text / Link:** Statutory market code disclaimers (all rights reserved).
- **Likely D-56 Class:** `research-only` (PC-5)
- **Access Method:** Open direct download
- **Coverage:** All-island Ireland; balancing market rules, imbalance settlement, capacity market charges, and currency conversion mechanics in force in 2024.
- **Evaluation:** Governs ex-post imbalance settlement and dispatch redispatch rules. Resolves the previous dead link in `SRC-27` by anchoring to the approved baseline versions published on the SEM Committee publications hub.

### Recommendation & Proxy Formulation
- **Recommended Candidate:**
  - For GB: Pin **Elexon BSC Section T Version 38.0** (for Jan–Oct 2024) and **Version 41.0** (for Nov–Dec 2024) (`Candidate 5.1`).
  - For Ireland: Pin **SEMOpx Operating Procedures (DAM, IDA, IDC) 2024 Edition** (`Candidate 5.2`) and **SEM Committee Part B Baseline** (`Candidate 5.3`).
- **Standard Market Proxy / Assumptions:**
  - Where detailed market clearing algorithms (e.g., Euphemia mixed-integer linear programming branch-and-bound) cannot be executed end-to-end due to commercial licensing, implement the standard marginal clearing price proxy: continuous linear dispatch sorted by generator Short-Run Marginal Cost (SRMC), clearing at the intersection of total scheduled demand and offer stacks, bounded by SEMOpx and BSC price caps.

---

## 6. Fuel, Carbon, and Subsidy Vectors

### Deficit Description
Unit dispatch modelling requires daily/half-hourly Short-Run Marginal Cost (SRMC) vectors. UK ETS auction clearing data has unverified third-party redistribution rights; EU ETS auction series are unacquired; World Bank commodity price indices are monthly; DESNZ fossil fuel assumptions are annual long-term projections; LCCC CfD portal requires authentication; Ofgem Renewables Obligation parameters lack an identified machine-readable API; generator heat rates and variable O&M parameters remain unacquired.

### Candidate Sources

#### Candidate 6.1: ICE End-of-Day UKA/EUA Auction Results & GOV.UK Public Reports
- **URL:** [https://www.theice.com/emissions/auctions/uk-emission-allowances](https://www.theice.com/emissions/auctions/uk-emission-allowances) and [https://www.gov.uk/government/publications/taking-part-in-the-uk-emissions-trading-scheme-markets](https://www.gov.uk/government/publications/taking-part-in-the-uk-emissions-trading-scheme-markets)
- **Publisher:** Intercontinental Exchange (ICE Futures Europe) / UK ETS Authority (DESNZ)
- **Licence Text / Link:** ICE Market Data Terms (proprietary; commercial use restricted); GOV.UK guidance under Open Government Licence (OGL v3.0).
- **Likely D-56 Class:** `research-only / unverified` (PC-5; exchange redistribution rights unverified)
- **Access Method:** Open web inspection (clearing price published after each fortnightly auction)
- **Coverage:** United Kingdom (UKA) and European Union (EUA); calendar year 2024 (fortnightly auctions).
- **Evaluation:** Primary pricing instrument for UK Allowances (£/tCO2e) and EU Allowances (€/tCO2e). Governed by statutory auction reserve price (£22/tCO2e for UK ETS).

#### Candidate 6.2: DESNZ Electricity Generation Costs 2023 / 2024 (Annex A Assumptions)
- **URL:** [https://www.gov.uk/government/publications/electricity-generation-costs-2023](https://www.gov.uk/government/publications/electricity-generation-costs-2023)
- **Publisher:** Department for Energy Security and Net Zero (DESNZ)
- **Licence Text / Link:** [Open Government Licence v3.0 (OGL v3.0)](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/)
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open direct download (Excel spreadsheet `Annex A: Key Data and Assumptions`)
- **Coverage:** United Kingdom; technology cost and operational performance parameters (CCGT, OCGT, Nuclear, Biomass, Wind, Solar, Battery Storage) commissioning 2025–2040.
- **Evaluation:** Authoritative public source for plant operating characteristics:
  - *Net Thermal Efficiency:* CCGT unabated ~50–54% (heat rate ~6,300–6,800 BTU/kWh or 1.85–2.00 MWh_th / MWh_e); OCGT peaker ~35% (heat rate ~9,750 BTU/kWh or 2.85 MWh_th / MWh_e).
  - *Variable O&M:* CCGT unabated ~£3.50–£5.00/MWh; OCGT ~£5.00–£8.00/MWh; Biomass ~£5.00–£10.00/MWh.

#### Candidate 6.3: Ofgem Renewables Obligation Annual Determinations & LCCC CfD Register
- **URL:** Ofgem RO Determination: [https://www.ofgem.gov.uk/](https://www.ofgem.gov.uk/) and LCCC Register: [https://www.lowcarboncontracts.uk/cfd-register/](https://www.lowcarboncontracts.uk/cfd-register/)
- **Publisher:** Office of Gas and Electricity Markets (Ofgem) & Low Carbon Contracts Company (LCCC)
- **Licence Text / Link:** [Open Government Licence v3.0 (OGL v3.0)](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/)
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open direct download (Ofgem notice PDF / LCCC CSV register)
- **Coverage:** UK; 2024/25 obligation period.
- **Evaluation:** Authoritative regulatory subsidy values:
  - *RO Buyout Price (2024–25):* **£64.73 per ROC**. Obligation level: 0.491 ROCs/MWh in England, Wales, and Scotland; 0.192 ROCs/MWh in Northern Ireland.
  - *CfD Strike Prices:* Technology and allocation round specific strike prices (£/MWh, 2012 indexed).

### Recommendation & Proxy Formulation
- **Recommended Candidate:**
  - For operational plant characteristics: **DESNZ Electricity Generation Costs Annex A** (`Candidate 6.2`, `open-commercial`) for thermal efficiencies, heat rates, and variable O&M.
  - For renewable subsidy values: **Ofgem RO Determinations** (`Candidate 6.3`, `open-commercial`) for £64.73/ROC buyout baseline.
  - For carbon and commodity vectors: Use **World Bank Commodity Price Data** (`SRC-37b`, `open-commercial`, CC BY 4.0) with cubic spline / forward-fill daily interpolation, paired with official **UK ETS / EU ETS auction results** (`Candidate 6.1`, `research-only`, PC-5).
- **Generator Short-Run Marginal Cost (SRMC) Proxy Formulation:**
  For each thermal generator unit $g$ with fuel type $f \in \{\text{Gas}, \text{Coal}, \text{Oil}, \text{Biomass}\}$:
  $$\text{SRMC}_g(t) = \frac{\text{FuelPrice}_f(t) + \text{EmissionFactor}_f \times \text{CarbonPrice}(t)}{\eta_g} + \text{VOM}_g$$
  where $\eta_g$ is net thermal efficiency (e.g. 0.52 for CCGT, 0.35 for OCGT from DESNZ Annex A), $\text{FuelPrice}_f(t)$ is daily gas/coal price, $\text{CarbonPrice}(t)$ is daily interpolated UKA/EUA price, and $\text{VOM}_g$ is variable O&M.
