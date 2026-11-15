# M1 Deficit Sources and Mitigation Analysis

This document addresses the 9-point deficit backlog identified in milestone M1 ([m1-data-inventory.md](m1-data-inventory.md)). For each deficit, up to 3 candidate public sources (from TSOs, DNOs, regulators, government agencies, and academic repositories) are evaluated for licensing compliance (D-56), access requirements, and coverage. A single primary recommendation or proxy/assumption is provided per deficit.

Per [AGENTS.md](../../AGENTS.md) Rule 10 and D-56, Open Database License (ODbL) and OpenStreetMap (OSM) datasets are strictly excluded as unusable due to viral share-alike provisions. Any term, schema attribute, or availability detail not confirmed directly on the publisher's published portal or documentation page is explicitly designated as **UNVERIFIED**.

---

## 1. Commissioned 2024 Network Fleet & Asset Denominators

### Deficit Description
ETYS 2025/26 and DNO 2025/26 releases represent forward-looking planning snapshots rather than historical 2024 commissioned asset baselines. GSP and BMU crosswalks, rating definitions (post-fault vs continuous / seasonal pre-fault), unit bases (% on 100 MVA vs pu on 100 MVA vs ohms), and 132-kV circuit denominators across all 14 DNO licence areas remain incomplete or unharmonised.

### Candidate Sources

#### Candidate 1.1: NESO ETYS 2024 Appendix B (System Technical Data Archive)
- **URL:** [https://www.neso.energy/etys-documents-and-appendices](https://www.neso.energy/etys-documents-and-appendices)
- **Publisher:** National Energy System Operator (NESO)
- **Licence Text / Link:** Governed by general [NESO Terms and Conditions](https://www.neso.energy/terms-and-conditions) (all rights reserved; commercial exploitation restricted without written consent).
- **Likely D-56 Class:** `research-only` (PC-1)
- **Access Method:** Open direct download (Excel `.xlsx` workbook)
- **Coverage:** Great Britain (400 kV, 275 kV, Scottish 132 kV); Historical 2023/24 baseline with forward planning horizons.
- **Evaluation:** Provides transmission line parameters (resistance $R$, reactance $X$, susceptance $B$), continuous/post-fault seasonal ratings, and transformer impedance data as configured for the 2023/24 operational year. Does not include England & Wales 132 kV distribution circuits or geographic bus coordinates.

#### Candidate 1.2: Elexon BMRS BM Unit Reference, GSP Group APIs & Market Domain Data (MDD)
- **URL:** [https://data.elexon.co.uk/bmrs/api/v1/reference/bmunits/all](https://data.elexon.co.uk/bmrs/api/v1/reference/bmunits/all) and [https://www.elexon.co.uk/operations-settlement/market-domain-data-mdd/](https://www.elexon.co.uk/operations-settlement/market-domain-data-mdd/)
- **Publisher:** Elexon Limited
- **Licence Text / Link:** [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/) (permits commercial and non-commercial reproduction with attribution).
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open REST API (JSON format; no authentication credentials required) / Open MDD release files
- **Coverage:** Great Britain; operational 2024 Balancing Mechanism units mapped to Lead Parties, National Grid BMU identifiers, fuel types, transmission capacities, and GSP Group affiliations (`_A` through `_P`).
- **Evaluation:** The `/reference/bmunits/all` endpoint provides primary BM Unit registration details, but does not identify explicit Grid Supply Point (GSP) substation membership for all embedded units. Combining the endpoint with BSC Market Domain Data (MDD) and Elexon Settlement P114 files provides the necessary crosswalk bridging Balancing Mechanism generation and demand units to Grid Supply Points and regional DNO boundaries.

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
- **Recommended Candidate:** Combine **NESO ETYS 2024 Appendix B** (`Candidate 1.1`) for GB transmission lines with **Elexon BMRS BM Unit API + MDD** (`Candidate 1.2`) for 2024 GSP/BMU asset crosswalks, supplemented by **NGED and SSEN LTDS 2024 Table 1/2** (`Candidate 1.3`) for open-commercial 132 kV distribution.
- **Standard Engineering Proxy / Assumptions:**
  1. *Unit Base Harmonisation:* Convert all branch impedances to standard per-unit on a 100 MVA system base ($S_b = 100\text{ MVA}$). For branches given in physical ohms at nominal voltage $V_b = 132\text{ kV}$, base impedance is $Z_{\text{base}} = V_b^2 / S_b = (132^2) / 100 = 174.24\ \Omega$. For branches given in percentage on 100 MVA, divide by 100.
  2. *Continuous vs Post-Fault Rating Proxy:* Where DNO tables report only a single post-fault or emergency rating (e.g. 1-hour or 3-minute overload rating), assume the continuous operational thermal rating is $0.88 \times \text{Post-Fault Rating}$ (based on standard ENA Engineering Recommendation P27 overhead line thermal envelopes).

---

## 2. Proxy Rule Parameters and Cut-Sets

### Deficit Description
NESO boundary constraint groups lack explicit signed electrical branch cut-sets in machine-readable format. Furthermore, regional inertia floors, unit stability groups, minimum-unit requirements, and Irish System Non-Synchronous Penetration (SNSP) operational limits are often cited from realized outturn statistics rather than documented operational rules.

### Candidate Sources

#### Candidate 2.1: EirGrid & SONI Operational Constraints Update (2024 Releases)
- **URL:** [https://www.sem-o.com/publications/](https://www.sem-o.com/publications/) and [https://www.eirgrid.ie/customer-and-community/operational-constraints](https://www.eirgrid.ie/customer-and-community/operational-constraints)
- **Publisher:** EirGrid Group & SONI Ltd
- **Licence Text / Link:** SEMO and EirGrid General Website Disclaimers ([EirGrid Disclaimer](https://www.eirgrid.ie/disclaimer); copyright all rights reserved).
- **Likely D-56 Class:** `research-only` (PC-2 / PC-3)
- **Access Method:** Open direct download (PDF reports published regularly)
- **Coverage:** All-island Ireland (ROI and NI); calendar year 2024 and ongoing operational horizon.
- **Evaluation:** Highly authoritative operational rulebook. Formally codifies:
  - *All-island SNSP operational ceiling limit:* fixed at **75%** throughout 2024.
  - *Rate of Change of Frequency (RoCoF) limit:* **1.0 Hz/s** (over a 500 ms rolling window).
  - *System Inertia Floor:* minimum **23,000 MW·s** all-island floor.
  - *Jurisdictional Minimum Synchronous Units:*
    - **Ireland (ROI):** minimum **5 large synchronous units** from 1 January 2024 until 16 July 2024; reduced to minimum **4 large synchronous units** effective **17 July 2024** per the joint EirGrid/SONI Operational Constraints Update.
    - **Northern Ireland (NI):** minimum **3 synchronous units** maintained throughout all of 2024.
  - *Area Constraint Groups (TCGs):* explicit unit groups and export/import limits for Dublin, North-South tie-line, and North-West stability constraints.

#### Candidate 2.2: NESO Day-Ahead Constraints Limits & 24-Months-Ahead Limits
- **URL:** [https://api.neso.energy/api/3/action/package_show?id=1e2be3b2-e64e-4e42-b062-841f39f8df5b](https://api.neso.energy/api/3/action/package_show?id=1e2be3b2-e64e-4e42-b062-841f39f8df5b) (Day-Ahead Constraints) and [https://api.neso.energy/api/3/action/package_show?id=8610ea09-90d7-4638-b7ba-bdfb8e7df009](https://api.neso.energy/api/3/action/package_show?id=8610ea09-90d7-4638-b7ba-bdfb8e7df009) (24-Months-Ahead)
- **Publisher:** National Energy System Operator (NESO)
- **Licence Text / Link:** [National Energy SO Open Data Licence v1.0](https://www.neso.energy/data-portal/neso-open-licence)
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open CKAN REST API / direct CSV download
- **Coverage:** Great Britain; half-hourly transfer capability limits (MW) for major transmission boundaries (e.g., B6 Anglo-Scottish, B7 Upper North, B8 North to Midlands, B14 London) for 2024.
- **Evaluation:** Provides numerical boundary transfer capacities applied in daily scheduling and redispatch. However, it specifies the aggregate group MW limit, omitting the explicit member branch cut-set definitions.

#### Candidate 2.3: NESO Operability Strategy, Grid Code & Frequency Risk Reports
- **URL:** [https://www.neso.energy/industry-information/codes/grid-code](https://www.neso.energy/industry-information/codes/grid-code) and [https://www.neso.energy/document/280926/download](https://www.neso.energy/document/280926/download)
- **Publisher:** National Energy System Operator (NESO)
- **Licence Text / Link:** [NESO Terms and Conditions](https://www.neso.energy/terms-and-conditions) (statutory Grid Code documentation).
- **Likely D-56 Class:** `research-only` (PC-5)
- **Access Method:** Open direct download (PDF publications)
- **Coverage:** Great Britain; national frequency regulation, ROCOF risk requirements, and regional stability constraints.
- **Evaluation:** Formally defines the national inertia floor across dated operational stages during 2024:
  - *Before 28 February 2024:* baseline inertia floor maintained at **140 GVA·s**.
  - *From 28 February 2024 to 18 June 2024:* operational floor lowered to **130 GVA·s**.
  - *From 19 June 2024 onward:* operational floor lowered to **120 GVA·s** (enabled by Stability Pathfinder synchronous compensators).

### Recommendation & Proxy Formulation
- **Recommended Candidate:**
  - For Ireland: **EirGrid & SONI Operational Constraints Update** (`Candidate 2.1`) provides complete, verified operational ceilings (75% SNSP, 23,000 MW·s inertia, dated 5-to-4 ROI minimum unit rules).
  - For GB: **NESO Day-Ahead Constraints Limits** (`Candidate 2.2`) for numerical MW transfer limits, paired with dated inertia floor stages from NESO Operability updates (`Candidate 2.3`).
- **Topological Branch Cut-Set Proxy Formulation:**
  - Let a boundary partition the network's electrical buses into two disjoint sets: an Exporting bus set $S_{\text{export}}$ and an Importing bus set $S_{\text{import}}$.
  - The electrical branch cut-set $\mathcal{C}_{\text{boundary}}$ comprises all circuits connecting a bus in $S_{\text{export}}$ to a bus in $S_{\text{import}}$.
  - *Sign Convention:* For each branch $b = (u, v)$ with predefined model orientation from bus $u$ to bus $v$:
    $$\text{sign}(b) = \begin{cases} +1 & \text{if } u \in S_{\text{export}} \text{ and } v \in S_{\text{import}} \\ -1 & \text{if } u \in S_{\text{import}} \text{ and } v \in S_{\text{export}} \end{cases}$$
    Net transfer across the boundary is:
    $$F_{\text{boundary}} = \sum_{b \in \mathcal{C}_{\text{boundary}}} \text{sign}(b) \cdot P_b$$
  - *Heuristic Caveat:* Geometric line-crossing of NESO ETYS Boundary GIS shapefile polygons (`SRC-02`) is an unverified heuristic unless verified against the electrical substation busbar memberships of the terminal endpoints.

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
- **Coverage:** Republic of Ireland; cumulative registered microgeneration capacity across 2024 (progressing from ~350 MW across ~79,000 installations at year-start to 498 MW across 121,800 installations at year-end, averaging ~800 additions per week).
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
  Because EirGrid Smart Grid Dashboard reports only grid-connected utility-scale wind and solar, synthesize half-hourly Irish embedded solar via physical capacity scaling with dated fleet progression:
  $$C_{\text{micro\_PV, IE}}(t) = 350\text{ MW} + (498\text{ MW} - 350\text{ MW}) \times \frac{t - t_0}{t_{\text{end}} - t_0}$$
  $$P_{\text{embedded\_solar, IE}}(t) = C_{\text{micro\_PV, IE}}(t) \times \frac{P_{\text{utility\_solar, IE}}(t)}{C_{\text{utility\_solar, IE}}(t)}$$
  (Applying uniform 498 MW across all of 2024 is acceptable only under an explicitly labeled fixed-fleet scenario, not as an historical outturn reconstruction).

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

---

## 7. Weather Reanalysis Acquisition

### Deficit Description
Physical renewable generation profiles require gridded meteorological variables (wind speed at hub height, solar irradiance, ambient temperature). Zero bytes of Copernicus ERA5 reanalysis data have been downloaded in the repository due to owner API credential prerequisites; Met Éireann MÉRA reanalysis terminated in August 2019 and does not cover the 2024 baseline; automated power-curve conversion software remains unintegrated into the data pipeline.

### Candidate Sources

#### Candidate 7.1: Copernicus Climate Data Store (CDS) ERA5 Hourly Single Levels
- **URL:** [https://cds.climate.copernicus.eu/datasets/reanalysis-era5-single-levels](https://cds.climate.copernicus.eu/datasets/reanalysis-era5-single-levels)
- **Publisher:** European Centre for Medium-Range Weather Forecasts (ECMWF) / Copernicus Climate Change Service (C3S)
- **Licence Text / Link:** [Licence to use Copernicus Products](https://cds.climate.copernicus.eu/api/v2/terms/static/licence-to-use-copernicus-products.pdf) (incorporates Creative Commons Attribution 4.0 International, CC BY 4.0).
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Programmatic API via `cdsapi` client library (requires free registered user account and API key configured in `~/.cdsapirc`).
- **Coverage:** Global regular lat/lon grid at $0.25^\circ \times 0.25^\circ$ resolution (~31 km), covering GB, Northern Ireland, and the Republic of Ireland. All 8,784 hours of calendar year 2024.
- **Variables:** $100\text{ m}$ $u$- and $v$-components of wind speed, Surface Solar Radiation Downwards (`ssrd`, $\text{J}/\text{m}^2$), $2\text{ m}$ air temperature (`t2m`, $\text{K}$), surface pressure (`sp`, $\text{Pa}$).
- **Evaluation:** Gold standard meteorological reanalysis for energy systems modeling. Automated retrieval scripts can be executed via headless CLI without interactive GUI steps.

#### Candidate 7.2: UK Met Office / CEDA MIDAS Open Hourly Surface Weather Observations
- **URL:** [https://catalogue.ceda.ac.uk/uuid/dbd451271ad046668776d2ec05690d64](https://catalogue.ceda.ac.uk/uuid/dbd451271ad046668776d2ec05690d64)
- **Publisher:** UK Met Office / Centre for Environmental Data Analysis (CEDA)
- **Licence Text / Link:** [Open Government Licence v3.0 (OGL v3.0)](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/)
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Free CEDA user registration required for automated bulk HTTP/FTP downloads.
- **Coverage:** Great Britain and Northern Ireland; hourly observations across ~250 terrestrial synoptic weather stations throughout 2024.
- **Evaluation:** Provides empirical ground-truth weather station observations (measured wind speed, direction, ambient dry-bulb temperature, global horizontal irradiance where equipped). Essential for statistical validation and empirical bias correction of downscaled ERA5 grid cells.

#### Candidate 7.3: Met Éireann Historical Weather Station Hourly Records
- **URL:** [https://www.met.ie/climate/available-data/historical-data](https://www.met.ie/climate/available-data/historical-data) and [https://data.gov.ie/organization/met-eireann](https://data.gov.ie/organization/met-eireann)
- **Publisher:** Met Éireann (National Meteorological Service of Ireland)
- **Licence Text / Link:** Open Data Licence Ireland (CC BY 4.0) via data.gov.ie; general website terms apply to direct met.ie downloads.
- **Likely D-56 Class:** `open-commercial` (via data.gov.ie extracts) / `research-only / unverified` (via direct met.ie scraping).
- **Access Method:** Open direct download (CSV files per synoptic station; no credentials required).
- **Coverage:** Republic of Ireland; all 8,784 hours of 2024 across 25 primary synoptic stations (e.g., Dublin Airport, Cork Airport, Shannon Airport, Athenry, Valentia Observatory, Malin Head).
- **Evaluation:** Directly circumvents the limitation that MÉRA reanalysis ended in 2019. Delivers verified 2024 hourly wind speed ($\text{knots} \to \text{m/s}$), temperature, relative humidity, and solar radiation observations across Ireland.

### Recommendation & Proxy Formulation
- **Recommended Candidate:** Primary spatial weather driver: **Copernicus CDS ERA5** (`Candidate 7.1`, `open-commercial`), paired with **Met Office MIDAS Open** (`Candidate 7.2`) and **Met Éireann Hourly Station Records** (`Candidate 7.3`) for ground-truth validation.
- **Power Curve Conversion & Fallback Proxy:**
  1. *Wind Turbine Conversion:* Use standard IEC 61400-1 Class I/II/III power curves. For hub-height wind speed $v(t)$, scale 100m ERA5 wind speed or extrapolate 10m station wind speed using the power-law profile:
     $$v(t) = v_{\text{ref}}(t) \cdot \left(\frac{h_{\text{hub}}}{h_{\text{ref}}}\right)^\alpha$$
     where $\alpha \approx 0.143$ onshore and $0.11$ offshore. Apply normalized power curves to generate capacity factors $CF_{\text{wind}}(t) \in [0, 1]$.
  2. *Solar PV Conversion:* Convert Surface Solar Radiation Downwards ($G_{\text{horiz}}$ in $\text{W}/\text{m}^2$) to cell temperature and power output:
     $$CF_{\text{solar}}(t) = \frac{G_{\text{POA}}(t)}{1000\text{ W}/\text{m}^2} \times \left[1 - \gamma \cdot (T_{\text{cell}}(t) - 25^\circ\text{C})\right]$$
     where temperature coefficient $\gamma \approx 0.004 / ^\circ\text{C}$.
  3. *Zero-Download Synthetic Proxy:* If CDS credentials are not yet configured by the repository owner, synthesize hourly renewable series by scaling actual outturn capacity factors from Elexon FUELHH (`SRC-31`) and SEMO Smart Grid Dashboard (`SRC-24`) across individual generator sites using inverse-distance spatial weighting.

---

## 8. Statutory and Market Calendars

### Deficit Description
Civil bank holidays, market trading calendars, clock transitions (DST), and interval indexes must be strictly pinned to immutable repository artifacts. Access to official Irish public holiday data returned HTTP 403 in initial queries; divergences between UK bank holidays and Irish statutory holidays remain unharmonised; repeated local clock hours during autumn transitions must not be erroneously deduplicated.

### Candidate Sources

#### Candidate 8.1: Workplace Relations Commission & GOV.IE Statutory Public Holiday Register
- **URL:** [https://www.workplacerelations.ie/en/what_you_should_know/public-holidays/](https://www.workplacerelations.ie/en/what_you_should_know/public-holidays/) and [https://www.gov.ie/](https://www.gov.ie/)
- **Publisher:** Workplace Relations Commission & Government of Ireland
- **Licence Text / Link:** [Irish Public Sector Information / Open Data Licence](https://data.gov.ie/licence) (CC BY 4.0 / OGL equivalent).
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open web access / direct tabular download (replaces the gated 403 URL).
- **Coverage:** Republic of Ireland; all 10 statutory public holidays for 2024 enacted under the *Organisation of Working Time Act 1997* (as amended):
  1. *1 January 2024:* New Year's Day
  2. *5 February 2024:* St. Brigid's Day (first Monday in February)
  3. *17 March 2024:* St. Patrick's Day (Sunday; statutory substitute entitlement rules apply)
  4. *1 April 2024:* Easter Monday (**Note: Good Friday 29 March is NOT a statutory public holiday in Ireland**)
  5. *6 May 2024:* May Day (first Monday in May)
  6. *3 June 2024:* June Bank Holiday (first Monday in June)
  7. *5 August 2024:* August Bank Holiday (first Monday in August)
  8. *28 October 2024:* October Bank Holiday (last Monday in October)
  9. *25 December 2024:* Christmas Day
  10. *26 December 2024:* St. Stephen's Day
- **Evaluation:** Provides verified legal baseline for Irish non-working days. Resolves jurisdiction-specific calendar splits against the UK (e.g., St. Brigid's Day is observed in ROI only; Good Friday is a bank holiday in GB/NI but not in ROI; August Bank Holiday falls on the first Monday of August in ROI/Scotland, but the last Monday in England & Wales).

#### Candidate 8.2: UK Government Cabinet Office Bank Holidays API (`SRC-33`)
- **URL:** [https://www.gov.uk/bank-holidays.json](https://www.gov.uk/bank-holidays.json)
- **Publisher:** UK Cabinet Office
- **Licence Text / Link:** [Open Government Licence v3.0 (OGL v3.0)](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/)
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open REST API (JSON format; no authentication required).
- **Coverage:** United Kingdom; official statutory bank holidays partitioned by jurisdiction: `england-and-wales`, `scotland`, and `northern-ireland` for 2024.
- **Evaluation:** Authoritative programmatic JSON endpoint for UK jurisdictions. Includes regional holidays such as St. Andrew's Day (Scotland), 12th July Battle of the Boyne (Northern Ireland), and Late August Holiday (England and Wales).

#### Candidate 8.3: IANA Time Zone Database (`tzdata` Release `2024b`)
- **URL:** [https://www.iana.org/time-zones](https://www.iana.org/time-zones) and [https://data.iana.org/time-zones/tz-link.html](https://data.iana.org/time-zones/tz-link.html)
- **Publisher:** Internet Assigned Numbers Authority (IANA)
- **Licence Text / Link:** Public Domain ([IANA Time Zone Database Licence](https://www.iana.org/time-zones/repository/tz-link.html)).
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open direct download (standard POSIX/tzfile distribution; embedded in standard Rust `chrono-tz`).
- **Coverage:** Global timezones; covers `Europe/London` and `Europe/Dublin`.
- **Evaluation:** Codifies exact DST transitions for 2024:
  - *Spring Forward (23-hour day, 46 settlement periods):* Sunday, 31 March 2024 at 01:00 UTC ($\to 02:00\text{ local}$).
  - *Fall Back (25-hour day, 50 settlement periods):* Sunday, 27 October 2024 at 02:00 local ($\to 01:00\text{ local}$).
  - Formally encodes the statutory nuance that in the UK standard time is GMT and daylight saving is BST (UTC+1), whereas under Ireland's *Standard Time (Amendment) Act 1971*, standard time is Irish Standard Time (IST, UTC+1 in summer) with a winter time adjustment (UTC in winter).

### Recommendation & Proxy Formulation
- **Recommended Candidate:** Bundle **GOV.UK Bank Holidays JSON** (`Candidate 8.2`) and **WRC Irish Public Holidays Register** (`Candidate 8.1`) into a single version-pinned repository calendar JSON fixture (`fixtures/calendars/2024-statutory-calendars.json`), driven by **IANA tzdata 2024b** (`Candidate 8.3`).
- **Clock Transition Handling Rule:**
  All simulator engine intervals operate natively in continuous monotonic UTC timestamps ($[t_{\text{start}}, t_{\text{end}})$ with 1800-second step duration).
  For local settlement reporting, map UTC to local settlement periods ($SP \in [1, 46]$ for spring transition, $SP \in [1, 50]$ for autumn transition). The repeated local wall-clock hours ($01:00–02:00$) during the autumn fall-back transition are distinguished via explicit fold/offset flags ($\text{offset} = +01:00$ vs $\text{offset} = +00:00$) and settlement period indices ($SP = 3$ vs $SP = 5$), strictly preventing interval deduplication.

---

## 9. Scenario Schema, Identity Inputs & Fixtures

### Deficit Description
The canonical scenario data model currently exists only as an architectural specification in ADR 0002. A complete, auditable scenario input engine requires: independent hashing of scenario input identity and execution identity; distinction between as-of forecast backtests and historical reconstructions; typed domain records with permanent opaque UUIDs; three distinct timestamps (effective interval $[t_{\text{from}}, t_{\text{to}})$, publication/revision time, acquisition time); version-pinned mappings for calendars, transformations, cost models, FX vectors, and proxy constraints; and rights lineage tracking with PC-1–PC-7 clearance tags.

### Candidate Sources

#### Candidate 9.1: ENTSO-E Common Grid Model Exchange Standard (CGMES v3.0 / IEC 61970)
- **URL:** [https://www.entsoe.eu/data/cim/cim-for-grid-models/](https://www.entsoe.eu/data/cim/cim-for-grid-models/) and [https://iec.ch/](https://iec.ch/)
- **Publisher:** European Network of Transmission System Operators for Electricity (ENTSO-E) / International Electrotechnical Commission (IEC)
- **Licence Text / Link:** Open international standard; ENTSO-E profile documentation freely accessible for implementation.
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open direct download (XSD schemas, UML models, and profile specifications).
- **Coverage:** Pan-European transmission system standard; covers Equipment (EQ), Topology (TP), State Variables (SV), and Dynamics (DY) profiles.
- **Evaluation:** Industry benchmark for electric network object modelling. Uses globally unique persistent identifiers (mRIDs / UUIDs) for buses, branches, transformers, and terminals. Serves as reference profile for canonical schema naming and entity typing.

#### Candidate 9.2: Frictionless Data Package Specification & JSON Schema Core (Draft 2020-12)
- **URL:** [https://specs.frictionlessdata.io/data-package/](https://specs.frictionlessdata.io/data-package/) and [https://json-schema.org/draft/2020-12/schema](https://json-schema.org/draft/2020-12/schema)
- **Publisher:** Open Knowledge Foundation (OKF) & IETF JSON Schema Working Group
- **Licence Text / Link:** [Creative Commons Attribution 4.0 International (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/) / Open Web Foundation Agreement.
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open web standard (W3C/IETF RFC specifications).
- **Coverage:** Universal tabular and multi-file data container packaging.
- **Evaluation:** Ideal specification for content-addressed dataset release manifests. Supports SHA-256 integrity checksums, foreign-key relationships between relational CSV/Parquet tables, typed column schemas, and embedded metadata licenses.

#### Candidate 9.3: Internal Repository Specification (`INT-01` / ADR 0002 Rust Serde Implementation)
- **URL:** [docs/architecture/adr/0002-network-and-scenario-data-model.md](../architecture/adr/0002-network-and-scenario-data-model.md)
- **Publisher:** Internal Repository Specification
- **Licence Text / Link:** Repository license (MIT / Apache 2.0 dual license; `open-commercial`).
- **Likely D-56 Class:** N/A (Internal Repository Code / Schema)
- **Access Method:** Direct repository implementation (`crates/energy-scenario-schema/`).
- **Coverage:** Full GB + all-island Ireland transmission and distribution topology, scenario parameter overrides, and execution locks.
- **Evaluation:** Tailored to the exact requirements of ADR 0002, ADR 0004, and ADR 0005.

### Recommendation & Proxy Formulation
- **Recommended Candidate:** Implement **Internal ADR 0002 Specification** (`Candidate 9.3`) using Rust `serde` and `schemars` aligned with **Frictionless Data Package** (`Candidate 9.2`) manifest conventions and **IEC 61970 CIM UUIDs** (`Candidate 9.1`).
- **Scenario Identity Checklist Specification:**
  In accordance with ADR 0002 and Finding R2-07, every scenario execution artifact must instantiate the following schema components:
  1. *Opaque UUIDs:* Every substation, bus, AC line, DC link, transformer, generator, storage, and demand attachment is assigned an immutable version 4 UUID (`uuid::Uuid`). Mutable human strings, DNO names, and bus numbers are stored strictly as external aliases.
  2. *Tri-Temporal Interval Tracking:* Every entity revision and observed parameter records three timestamps:
     - `effective_range`: Half-open interval $[t_{\text{from}}, t_{\text{to}})$ in UTC seconds.
     - `publication_time`: Verified source publication or notice timestamp (where unavailable, labeled as `reconstruction`).
     - `acquisition_time`: SHA-256 manifest ingestion timestamp.
  3. *Independent Content Hashing:*
     - `input_identity_hash`: Deterministic SHA-256 hash computed over canonicalized scenario TOML, entity parameters, time series arrays, and referenced dataset manifest hashes.
     - `execution_identity_hash`: SHA-256 hash computed over `input_identity_hash` combined with solver binary version, compiler flags, hardware runner ID, tolerance parameters, and seed.
  4. *Rights Lineage Tags:* Every input dataset carries an auditable classification tag (`open-commercial` or `research-only`) mapped to canonical pre-commercial tracking IDs (`PC-1` through `PC-7`), preventing commingling in commercial SaaS profiles.
  5. *Test Fixtures:* Provide minimal synthetic test scenarios (`tests/fixtures/scenarios/gb_ie_2024_baseline.toml`) with verifiable SHA-256 release locks to validate deserialization, schema validation, and constraint resolution before running full-scale historical simulations.

---

## 10. Summary Deficit Resolution Matrix

| Deficit # | Deficit Category | Primary Recommended Candidate Source | D-56 Licence Class | Access Requirement | Primary Recommended Proxy / Mitigation |
|:---:|---|---|:---:|:---:|---|
| **1** | Commissioned 2024 Network Fleet | NESO ETYS 2024 Appendix B + Elexon BMRS BMU / MDD API | `research-only` (PC-1) / `open-commercial` | Open download / Open REST API | Convert branch impedances to 100 MVA base; proxy continuous rating as 0.88 × post-fault rating. |
| **2** | Proxy Rule Parameters & Cut-Sets | EirGrid/SONI Operational Constraints Update (2024) & NESO Day-Ahead Constraints | `research-only` (PC-2) / `open-commercial` | Open PDF / Open CKAN API | Model dated SNSP (75%), ROI min units (5 to 4 on 17 July), NI min units (3), and GB inertia floors (140 $\to$ 130 $\to$ 120 GVA·s); derive signed cut-sets from bus partition sets. |
| **3** | Embedded Generation Decomposition | NESO Historic Demand 2024 + Sheffield Solar PV_Live API | `open-commercial` | Open CKAN API / Open REST API | Starting identity: $ND + \text{Embedded Solar} + \text{Embedded Wind}$ (pumping not subtracted from $ND$); scale Irish embedded solar using dated micro-PV capacity (350 $\to$ 498 MW). |
| **4** | Directional Interconnector Losses & TLM | Elexon Insights SAA API (`SRC-50`) + Regulatory Access Rules | `open-commercial` / Mixed | Open REST API / Open PDF filings | Apply $TLM = 1.0$ for interconnector BMUs in GB settlement (BSC P278); apply approved loss factors (BritNed 3.0%, Nemo 2.372%, ElecLink 2.9%); CIGRE proxy (0.75% per converter + 0.3%/100 km cable) for missing links. |
| **5** | Dated Market Rules | Elexon Digital BSC Section T (v38.0/v41.0) & SEMOpx Procedures 2024 | `research-only` (PC-5) | Open web / Open PDF download | Pin Spring 2024 (v38.0) and November 2024 (v41.0) BSC baselines; pin 2024 SEMOpx DAM/IDA/IDC operating procedures; linear SRMC stack dispatch proxy. |
| **6** | Fuel, Carbon & Subsidy Vectors | DESNZ Electricity Generation Costs Annex A + Ofgem RO Determinations | `open-commercial` | Open direct download (OGL v3.0) | Sourced DESNZ CCGT/OCGT net efficiencies (52% / 35%) and variable O&M; Ofgem £64.73/ROC buyout baseline; daily interpolated World Bank & UK ETS auction carbon prices. |
| **7** | Weather Reanalysis Acquisition | Copernicus CDS ERA5 Hourly Single Levels + MIDAS / Met Éireann Stations | `open-commercial` | Free CDS account (`cdsapi`) / Free CEDA account | Downscale ERA5 wind/solar/temp; validate against 2024 Met Office & Met Éireann synoptic stations; fallback to inverse-distance weighted actual outturns. |
| **8** | Statutory & Market Calendars | WRC Irish Public Holidays Register + GOV.UK Bank Holidays JSON + IANA tzdata | `open-commercial` | Open web / Open JSON API / Public domain | Harmonise ROI (10 holidays) and UK calendars in JSON fixture; distinguish Good Friday / St. Brigid / August bank holidays; preserve 49/50 settlement periods across autumn DST fold. |
| **9** | Scenario Schema, Identity & Fixtures | ADR 0002 Rust Serde Implementation + Frictionless Data Package Specs | `open-commercial` (MIT / Apache-2.0) | Repository internal implementation | Enforce tri-temporal intervals $[t_{\text{from}}, t_{\text{to}})$, opaque entity UUIDs, separate input vs execution SHA-256 hashes, PC-1–PC-7 clearance tags, and TOML test fixtures. |
