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
- **Evaluation:** Metadata inspected. Provides transmission line parameters (resistance $R$, reactance $X$, susceptance $B$), continuous/post-fault seasonal ratings, and transformer impedance data as configured for the 2023/24 operational year. Does not include England & Wales 132 kV distribution circuits or geographic bus coordinates, nor does it establish an uninterrupted 17,568-row panel or a fully reconciled 2023/24 commissioned fleet.

#### Candidate 1.2: Elexon BMRS BM Unit Reference, GSP Group APIs & Market Domain Data (MDD)
- **URL:** [https://data.elexon.co.uk/bmrs/api/v1/reference/bmunits/all](https://data.elexon.co.uk/bmrs/api/v1/reference/bmunits/all), [OpenAPI Specification](https://data.elexon.co.uk/swagger/v1/swagger.json), and [https://www.elexon.co.uk/operations-settlement/market-domain-data-mdd/](https://www.elexon.co.uk/operations-settlement/market-domain-data-mdd/)
- **Publisher:** Elexon Limited
- **Licence Text / Link:** [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/) (permits commercial and non-commercial reproduction with attribution).
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open REST API (JSON format; no authentication credentials required) / Open MDD release files
- **Coverage:** Great Britain; operational 2024 Balancing Mechanism units mapped to Lead Parties, National Grid BMU identifiers, fuel types, transmission capacities, and GSP Group affiliations (`_A` through `_P`).
- **Evaluation:** The `/reference/bmunits/all` endpoint provides primary BM Unit registration details. Note that a GSP Group (`_A` through `_P`) is not a physical GSP substation. Combining the endpoint with BSC Market Domain Data (MDD) and Elexon Settlement P114 files provides a candidate crosswalk bridging BM generation and demand units to Grid Supply Points and regional DNO boundaries, but row-level completeness across all embedded units remains `UNVERIFIED` until verified during pipeline ingestion.

#### Candidate 1.3: DNO Long Term Development Statements (LTDS November 2024 Editions)
- **URL:** Energy Networks Association (ENA) and individual DNO portals:
  - NGED: [https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model](https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model)
  - SSEN: [https://data.ssen.co.uk/@ssen-distribution/sepd_long_term_development_statement](https://data.ssen.co.uk/@ssen-distribution/sepd_long_term_development_statement)
  - UKPN: [https://ukpowernetworks.opendatasoft.com/explore/assets/ukpn-ltds-table-1-circuit-data/](https://ukpowernetworks.opendatasoft.com/explore/assets/ukpn-ltds-table-1-circuit-data/)
  - SPEN: [https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-appendix-1-circuit-data-table-1/](https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-appendix-1-circuit-data-table-1/)
  - NPg: [https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-appendix-3/information/](https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-appendix-3/information/)
  - ENWL: [https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/login-to-ltds-secure-area/](https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/login-to-ltds-secure-area/)
- **Publisher:** 6 UK DNO Group Licensees (covering all 14 DNO licence areas: 12 England/Wales 132 kV distribution areas and 2 Scottish distribution areas)
- **Licence Text / Link:** Mixed: NGED Open Data Licence (`open-commercial`), SSEN CC BY 4.0 (`open-commercial`), UKPN/SPEN/NPg portal registration terms (`open-commercial` metadata with free login prerequisite), ENWL Shared Data Licence (`research-only`, PC-5).
- **Likely D-56 Class:** Mixed (`open-commercial` for NGED and SSEN; `research-only` for ENWL and CIM models; row extractions from gated portals are subject to account registration terms).
- **Access Method:** Open direct download (NGED, SSEN) / Free registered user login (UKPN, SPEN, NPg) / Formal registration request (ENWL).
- **Coverage:** Great Britain; 132 kV circuits (Table 1 / Appendix 3) and 132 kV transformers (Table 2 / Appendix 4) as reported under standard licence condition SLC 25B in November 2024.
- **Evaluation:** Necessary for 132 kV distribution impedances and equipment ratings across England & Wales. However, archived November 2024 editions for all DNOs are not fully established without record-level inspection; units differ across DNOs (SPM reports % on 100 MVA, others report ohms or pu), and individual switchgear segments lack impedances.

### Recommendation & Proxy Formulation
- **Recommended Candidate:** Combine **NESO ETYS 2024 Appendix B** (`Candidate 1.1`, metadata inspected) for GB transmission lines with **Elexon BMRS BM Unit API + MDD** (`Candidate 1.2`) for GSP/BMU asset crosswalks, supplemented by **NGED and SSEN LTDS 2024 Table 1/2** (`Candidate 1.3`) for open-commercial 132 kV distribution.
- **Standard Engineering Proxy / Assumptions:**
  1. *Unit Base Harmonisation:* Convert all branch impedances to standard per-unit on a 100 MVA system base ($S_b = 100\text{ MVA}$). For branches given in physical ohms at nominal voltage $V_b = 132\text{ kV}$, base impedance is $Z_{\text{base}} = V_b^2 / S_b = (132^2) / 100 = 174.24\ \Omega$. For branches given in percentage on 100 MVA, divide by 100.
  2. *Continuous vs Post-Fault Rating Proxy:* Continuous operational thermal ratings remain missing where DNO tables report only a single post-fault or emergency rating (e.g. 1-hour or 3-minute overload rating). ENA Engineering Recommendation P27 does not prescribe a universal 0.88 conversion factor across overhead lines, cables, and transformers; overload duration, conductor types, and ambient conditions govern actual thermal envelopes.
     - **ASSUMPTION:** Continuous rating is estimated as $0.88 \times \text{Post-Fault Rating}$ as an unverified author-proposed proxy (logged as open question in questionnaire). This proxy is not a commissioned-asset rating and requires sensitivity testing and replacement with asset-specific manufacturer or DNO ratings before baseline acceptance.

---

## 2. Proxy Rule Parameters and Cut-Sets

### Deficit Description
NESO boundary constraint groups lack explicit signed electrical branch cut-sets in machine-readable format. Furthermore, regional inertia floors, unit stability groups, minimum-unit requirements, and Irish System Non-Synchronous Penetration (SNSP) operational limits are often cited from realized outturn statistics rather than documented operational rules.

### Candidate Sources

#### Candidate 2.1: EirGrid & SONI Operational Constraints Update & 2024 Renewable Constraint Report
- **URL:** [CMS EirGrid 2024 Report](https://cms.eirgrid.ie/sites/default/files/publications/Annual-Renewable-Constraint-and-Curtailment-Report-2024-V1.0.pdf), [SEMO Publications](https://www.sem-o.com/publications/), and [EirGrid Operational Constraints](https://www.eirgrid.ie/customer-and-community/operational-constraints)
- **Publisher:** EirGrid Group & SONI Ltd
- **Licence Text / Link:** SEMO and EirGrid General Website Disclaimers ([EirGrid Disclaimer](https://www.eirgrid.ie/disclaimer); copyright all rights reserved).
- **Likely D-56 Class:** `research-only` (PC-2 / PC-3)
- **Access Method:** Open direct download (PDF reports published regularly)
- **Coverage:** All-island Ireland (ROI and NI); calendar year 2024.
- **Evaluation:** Formally codifies headline operational rules per the joint *Annual Renewable Constraint and Curtailment Report 2024* (V1.0), §3.3 & §5.1:
  - *All-island SNSP operational ceiling limit:* fixed at **75%** throughout 2024 ("75% limit applied for the full year").
  - *Rate of Change of Frequency (RoCoF) limit:* **1.0 Hz/s** (over a 500 ms rolling window).
  - *System Inertia Floor:* minimum **23,000 MW·s** all-island floor ("23,000 MWs floor applied for the full year").
  - *Jurisdictional Minimum Synchronous Units:*
    - **7-set enduring policy:** §3.3 confirms: *"On April 7th, 2024, the 7-set trial (3 in N. Ireland and 4 in Ireland) was completed and became enduring policy."* Enduring policy of **4 large synchronous units in ROI** and **3 in NI** took effect on **7 April 2024** (not 17 July).
    - *Pre-7 April 2024 operational schedule:* Prior to 7 April 2024, operational trials and temporary exceptions occurred; the exact interval schedule prior to 7 April remains **`UNVERIFIED`** pending dated operational logs.
  - *Area Constraint Groups (TCGs):* Local stability constraints (e.g. Dublin and North-West minimum generation rules) apply in addition to headline floors; headline limits alone do not constitute a complete rule set.

#### Candidate 2.2: NESO Day-Ahead Constraints Limits & 24-Months-Ahead Limits
- **URL:** [Day Ahead Constraint Flows and Limits](https://api.neso.energy/api/3/action/package_show?id=day-ahead-constraint-flows-and-limits) and [24 Months Ahead Constraint Limits](https://api.neso.energy/api/3/action/package_show?id=24-months-ahead-constraint-limits)
- **Publisher:** National Energy System Operator (NESO)
- **Licence Text / Link:** [National Energy SO Open Data Licence v1.0](https://www.neso.energy/data-portal/neso-open-licence)
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open CKAN REST API / direct CSV download
- **Coverage:** Great Britain; transmission boundary transfer capability limits (MW) for major boundaries (e.g., B6 Anglo-Scottish, B7 Upper North, B8 North to Midlands, B14 London) for 2024.
- **Evaluation:** Provides numerical boundary transfer capacities. However:
  - The 24 Months Ahead series is a **weekly** forecast schedule, not a half-hourly historical series.
  - The Day Ahead series is a publication-time snapshot, not necessarily the final realised operating limit.
  - Limits specify aggregate group MW transfers, omitting explicit signed member branch cut-sets.

#### Candidate 2.3: NESO Operability Strategy, Grid Code & Frequency Risk Reports
- **URL:** [NESO Operability Strategy Reports Archive](https://www.neso.energy/publications/operability-strategy-report-and-electricity-markets-roadmap), [2024 Report](https://www.neso.energy/document/299926/download), [2025 Report](https://www.neso.energy/document/358116/download), and [NESO Grid Code](https://www.neso.energy/industry-information/codes/grid-code)
- **Publisher:** National Energy System Operator (NESO)
- **Licence Text / Link:** [NESO Terms and Conditions](https://www.neso.energy/terms-and-conditions) (statutory Grid Code documentation).
- **Likely D-56 Class:** `research-only` (PC-5)
- **Access Method:** Open direct download (PDF publications)
- **Coverage:** Great Britain; national frequency regulation and stability constraints for 2024.
- **Evaluation:** The 2025 Operability Report (pp. 7, 24) supports that the national inertia floor was reduced **from 140 GVA·s to 120 GVA·s during 2024** (enabled by Stability Pathfinder synchronous compensators). However:
  - The intermediate floor of **130 GVA·s** and the exact transition dates (**28 February 2024** and **19 June 2024**) are **`UNVERIFIED`** as they are not established by publisher text and lack dated operational notices.
  - Regional inertia floors, regional inertia contribution factors, and local minimum units are not provided in this national publication and remain open deficits.

### Recommendation & Proxy Formulation
- **Recommended Candidate:**
  - For Ireland: **EirGrid & SONI Joint 2024 Report (§3.3) / Operational Constraints Updates** (`Candidate 2.1`) for operational ceilings (75% SNSP, 23,000 MW·s inertia, 4 ROI / 3 NI enduring policy from 7 April 2024; pre-April trial intervals marked `UNVERIFIED`).
  - For GB: **NESO Day-Ahead Constraints Limits** (`Candidate 2.2`) for numerical MW transfer limits (noting publication snapshot semantics), paired with the broad 140 $\to$ 120 GVA·s reduction during 2024 from NESO Operability reports (`Candidate 2.3`), with specific intermediate dates marked `UNVERIFIED`.
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

#### Candidate 3.1: Sheffield Solar PV_Live
- **URL:** [https://www.solar.sheffield.ac.uk/pvlive/](https://www.solar.sheffield.ac.uk/pvlive/)
- **Publisher:** Sheffield Solar (University of Sheffield)
- **Licence Text / Link:** [Creative Commons Attribution 4.0 International (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/) per publisher page ("PV_Live by Sheffield Solar is licensed under CC BY 4.0").
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open REST API / download (JSON / CSV; standard queries do not require credentials).
- **Coverage:** Great Britain; half-hourly embedded solar generation outturns and capacity estimates (MW) at national aggregate, DNO region, and GSP Group levels from 2013 through 2024.
- **Evaluation:** Industry reference for unmetered embedded solar generation in GB. Metadata inspected; CC BY 4.0 licence is confirmed on the publisher page. However, endpoint schema, regional field completeness, and uninterrupted 2024 series availability remain to be verified at record level during ingestion. Does not cover Northern Ireland or the Republic of Ireland.

#### Candidate 3.2: NESO Historic Demand Data 2024
- **URL:** [https://api.neso.energy/api/3/action/package_show?id=historic-demand-data](https://api.neso.energy/api/3/action/package_show?id=historic-demand-data)
- **Publisher:** National Energy System Operator (NESO)
- **Licence Text / Link:** [National Energy SO Open Data Licence v1.0](https://www.neso.energy/data-portal/neso-open-licence)
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open CKAN REST API / direct CSV download
- **Coverage:** Great Britain; 2024 operational demand records across settlement periods.
- **Evaluation:** Metadata inspected. Authoritatively reports `ND` (National Demand), `TSD` (Transmission System Demand), `ENGLAND_WALES_DEMAND`, `EMBEDDED_SOLAR_GENERATION`, `EMBEDDED_WIND_GENERATION`, and `PUMP_STORAGE_PUMPING`. Confirms official operational demand accounting boundaries. However, metadata inspection does not prove an uninterrupted 17,568-row panel; record-level completeness and station-load / metering reconciliation must be verified during pipeline ingestion.

#### Candidate 3.3: ESB Networks & SEAI Microgeneration Reports (All-Island Embedded Capacity)
- **URL:** ESB Networks Annual Performance Reports: [2023 Report](https://media.esbnetworks.ie/media/docs/default-source/publications/distribution-annual-performance-report-2023-accessibled434eedca532460d959ccfba328306eb.pdf), [2024 Report](https://media.esbnetworks.ie/media/docs/default-source/publications/distribution-annual-performance-report-2024.pdf), and [SEAI Renewables Statistics](https://www.seai.ie/data-and-insights/seai-statistics/key-statistics/renewables/)
- **Publisher:** ESB Networks & Sustainable Energy Authority of Ireland (SEAI)
- **Licence Text / Link:** SEAI Open Data (OGL Ireland / CC BY 4.0); ESB Networks corporate publications (copyright all rights reserved).
- **Likely D-56 Class:** `open-commercial` (SEAI extracts) / `research-only` (ESB annual performance reports).
- **Access Method:** Open direct download (PDF annual reports and periodic statistical CSV extracts).
- **Coverage:** Republic of Ireland (ROI only); operational microgeneration fleet:
  - *End-2023:* **78,400 connections and approximately 300 MW** (ESB 2023 Report, p. 5).
  - *End-2024:* **121,700 connections and approximately 500 MW** (ESB 2024 Report, pp. 5, 64).
- **Evaluation:** Primary publisher figures establish approximately 300 MW (end-2023) to 500 MW (end-2024) microgeneration capacity in ROI. However:
  - Precise intra-year trajectories (such as 350 $\to$ 498 MW) are not published in these reports and constitute an unverified interpolation.
  - Connection and application counts do not automatically equal metered commissioned generation.
  - ESB covers the Republic of Ireland only; Northern Ireland embedded microgeneration is not reported here and remains an open data gap.
  - Neither ESB nor EirGrid publishes half-hourly metered microgeneration outturns.

### Recommendation & Proxy Formulation
- **Recommended Candidate:**
  - For GB: Use **NESO Historic Demand Data 2024** (`Candidate 3.2`) as primary demand identity inputs, verified against **Sheffield Solar PV_Live** (`Candidate 3.1`) for embedded solar profiles.
  - For Ireland: Use **ESB Networks reported annual endpoints** (~300 MW to ~500 MW, `Candidate 3.3`), with Northern Ireland microgeneration tracked as an open deficit.
- **Demand Accounting Identity Formulation:**
  As verified from NESO settlement documentation, National Demand ($ND$) already excludes station load, interconnector exports, and pumped storage pumping (which Transmission System Demand, $TSD$, adds back):
  $$ND = TSD - \text{Interconnector Exports} - \text{Pumped Storage Pumping} - \text{Station Load}$$
  The starting gross underlying demand identity for GB is:
  $$\text{Gross Underlying Demand} = ND + \text{Embedded Solar} + \text{Embedded Wind}$$
  Because $ND$ already excludes pumped storage pumping, pumping must **not** be subtracted from $ND$ again. Pumping is tracked as a separate flexible storage demand asset. Station load and metering adjustments must be reconciled during data ingestion.
- **Irish Embedded Generation Proxy / Assumptions:**
  Because half-hourly metered microgeneration is absent in Ireland, synthesize hourly Irish embedded solar profiles using an explicit assumption:
  - **ASSUMPTION:** [Q-P-02] Linear interpolation between end-2023 (~300 MW) and end-2024 (~500 MW) is assumed for ROI micro-PV capacity:
    $$C_{\text{micro\_PV, ROI}}(t) \approx 300\text{ MW} + (500\text{ MW} - 300\text{ MW}) \times \frac{t - t_0}{t_{\text{end}} - t_0}$$
    Scaled generation is estimated by applying utility solar capacity factors:
    $$P_{\text{embedded\_solar, ROI}}(t) = C_{\text{micro\_PV, ROI}}(t) \times \frac{P_{\text{utility\_solar, IE}}(t)}{C_{\text{utility\_solar, IE}}(t)}$$
    This scaling carries significant uncertainty (differing tilt, orientation, inverter clipping, and geographic distribution between rooftop and utility solar) and is marked as an unvalidated working proxy. Northern Ireland embedded solar remains an open gap.

---

## 4. Directional Interconnector Losses & TLM Source

### Deficit Description
Directional physical loss parameters across the 10 cross-border and island interconnectors are missing from standard TSO datasets, resulting in unverified numeric defaults or placeholders. Converter and cable loss separations, asymmetric forward/reverse losses, and sending/receiving metering boundaries require primary regulatory grounding. Furthermore, Elexon Transmission Loss Multipliers (TLMs) are missing from the previously gathered settlement streams.

### Candidate Sources

#### Candidate 4.1: Regulated Interconnector Access Rules & Loss Methodology Filings
- **URL:** Ofgem & National Regulatory Authority decision libraries:
  - BritNed Access Rules: [BritNed 2025 Approved Rules](https://www.britned.com/wp-content/uploads/2025/09/BritNed-Access-Rules-2025-Approved.pdf) and [Ofgem 2024 Approval Decision](https://www.ofgem.gov.uk/decision/approval-modified-access-rules-britned-interconnector-pursuant-standard-licence-condition-11a-electricity-interconnector-licence-2024)
  - Nemo Link Access Rules: [Ofgem Nemo Link Modified Access Rules](https://www.ofgem.gov.uk/sites/default/files/2022-11/The%20modified%20Access%20Rules%20-%20Nemo%20Link.pdf)
  - ElecLink Loss Factor: [ElecLink Loss Factor Publisher Notice (20 May 2022)](https://cdn.prod.website-files.com/67b2dbc715938e08fe43fee1/67d9d09d90ce7e920a93ee31_ElecLink%20Loss%20Factor%20v2.pdf) and [ElecLink Loss Factor Page](https://www.eleclink.co.uk/publications/loss-factor)
  - Moyle & EWIC Joint Regulatory Decisions: [SEM Committee](https://www.semcommittee.com/) and [Utility Regulator](https://www.uregni.gov.uk/)
- **Publisher:** Interconnector Licensees (BritNed Development Ltd, Nemo Link Ltd, ElecLink Ltd, Mutual Energy, EirGrid Interconnector DAC) approved by National Regulatory Authorities (Ofgem, CREG, CRE, CRU, UR).
- **Licence Text / Link:** Regulatory decisions published under Open Government Licence (OGL v3.0); operator schedules under website terms (proprietary, `research-only`).
- **Likely D-56 Class:** Mixed (`open-commercial` for Ofgem statutory decisions; `research-only` for operator manuals and proprietary schedules).
- **Access Method:** Open direct download (PDF filings and schedules).
- **Coverage:** GB and connected European / Irish markets (France, Netherlands, Belgium, Northern Ireland, Republic of Ireland); 2024 operational baseline.
- **Evaluation:** Important distinction: these filings establish **commercial deemed-loss factors** for nomination accounting, NOT physical converter/cable loss models.
  - *BritNed:* Schedule 3 of published Access Rules defines a **3.0% commercial loss factor** applied symmetrically around the Mid North Sea reference point. (The 2024 Ofgem decision approved specific modifications, but does not prove a historical 2024 factor panel).
  - *ElecLink:* Publisher notice dated 20 May 2022 confirms an end-to-end loss factor of **2.9%** (mid-interconnector factor of 1.471%) applied to midpoint nominations to determine Deemed Metered Volumes.
  - *Nemo Link:* Ofgem-hosted rules (Annex C) define $\text{BMUMV} = (1 \pm LF/2) \times \text{DMV}$, deferring the numerical loss factor to the operator's website. The operator website was blocked (HTTP 403) during review; the precise numbers (2.372% end-to-end, 1.186% mid-factor, 2.344% market coupling) remain **`UNVERIFIED`** pending direct operator access, though algebraically consistent.
  - *Mathematical Note on Midpoint Factors:* A midpoint nomination denominator differs from a sending-end denominator. With $LF = 3.0\%$, the ratio of receiving to sending energy is $\frac{1 - LF/2}{1 + LF/2} = \frac{0.985}{1.015} \approx 0.9704$, not $0.9700$. Commercial deemed factors must not be conflated with physical loss curves.
  - *Moyle & EWIC:* Regulated under SEM Committee decisions; market settlement incorporates Transmission Loss Adjustment Factors (TLAFs) at connection substations (Woodland, Ballycronan More).

#### Candidate 4.2: Elexon Insights Solution REST API & Balancing Settlement Stack (`SRC-50`)
- **URL:** [OpenAPI Specification](https://data.elexon.co.uk/swagger/v1/swagger.json), [Elexon Insights API](https://data.elexon.co.uk/), and [BSC Modification P278 Decision](https://assets.elexon.co.uk/wp-content/uploads/sites/11/2012/01/28170102/P278D.pdf)
- **Publisher:** Elexon Limited
- **Licence Text / Link:** [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/)
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open REST API (JSON; no authentication key required).
- **Coverage:** Great Britain; 2024 settlement periods.
- **Evaluation:** 
  - The official OpenAPI specification defines the balancing settlement acceptance stack endpoint: `/balancing/settlement/stack/all/{bidOffer}/{settlementDate}/{settlementPeriod}`, which exposes a nullable `transmissionLossMultiplier` field. Annual coverage across all BMUs and 17,568 periods remains `UNVERIFIED` pending programmatic pipeline testing.
  - **Interconnector TLM Rule:** Per BSC Modification P278 (implemented 29 November 2012 per [P278 Proposal](https://assets.elexon.co.uk/wp-content/uploads/sites/11/2012/01/04142822/P278.pdf) and [Authority Decision](https://assets.elexon.co.uk/wp-content/uploads/sites/11/2012/01/28170102/P278D.pdf)), **$TLM = 1.0$ exactly for all interconnector BM units** in GB imbalance settlement. This is an administrative transmission-loss allocation rule under the BSC, not a claim of zero physical losses.

#### Candidate 4.3: SEM Committee / EirGrid & SONI Annual TLAF Decision Tables
- **URL:** [SEM Committee Publications](https://www.semcommittee.com/publications/) and [EirGrid Operational Constraints](https://www.eirgrid.ie/customer-and-community/operational-constraints)
- **Publisher:** Single Electricity Market Committee (CRU & Utility Regulator) and System Operators (EirGrid & SONI)
- **Licence Text / Link:** SEM Committee public notices / statutory decision disclaimers ([EirGrid Disclaimer](https://www.eirgrid.ie/disclaimer)).
- **Likely D-56 Class:** `research-only` (PC-2 / PC-3)
- **Access Method:** Open direct download (annual decision papers and Excel tables).
- **Coverage:** All-island Ireland (ROI and NI); 2023/24 and 2024/25 tariff years (half-hourly nodal loss multipliers).
- **Evaluation:** Authoritative source for nodal loss multipliers in the SEM. The dated 2023/24 and 2024/25 TLAF tables must be inspected during ingestion to verify EWIC (Woodland) and Moyle (Ballycronan More) factors across each market clearing and settlement stage.

### Recommendation & Proxy Formulation
- **Recommended Candidate:**
  - For GB settlement accounting: Apply **BSC Modification P278** ($TLM = 1.0$ for interconnector BMUs), with balancing stack data tested via **Elexon OpenAPI** (`Candidate 4.2`).
  - For commercial nomination accounting: Use certified operator notices for **BritNed** (3.0% midpoint) and **ElecLink** (2.9% end-to-end / 1.471% midpoint), noting Nemo Link values remain `UNVERIFIED` pending direct operator access.
- **Separation of Commercial vs Physical Losses & HVDC Physical Loss Proxy:**
  Commercial deemed-loss factors must strictly be kept separate from physical electrical loss models:
  - Commercial factors dictate billing and nomination reconciliation at the border.
  - Physical losses represent thermal dissipation across AC/DC converters and subsea/underground cables.
  - **ASSUMPTION:** [Q-P-03] For interconnectors lacking publisher-certified physical loss curves (e.g. IFA, IFA2, NSL, Viking Link, Greenlink), the following author-proposed heuristic proxy is adopted as an unvalidated working assumption:
    1. *Converter Losses:* Fixed plus linear converter loss approximated as $0.75\%$ per converter station ($1.50\%$ for both terminals combined).
    2. *Cable Ohmic Losses:* Linear cable loss approximated as $0.30\%$ per 100 km cable length: $L_{\text{cable}}(d) = 0.003 \times (d / 100\text{ km})$.
    3. *Physical Formulation:* For real power $P_{\text{send}}$ injected at the sending converter bus:
       $$P_{\text{receive}} = P_{\text{send}} \times (1 - L_{\text{converter, send}}) \times (1 - L_{\text{cable}}(d)) \times (1 - L_{\text{converter, receive}})$$
    - *Disclosures:* This formulation is an author-proposed proxy and is **NOT** an established or certified CIGRE standard. CIGRE Study Committee B4 publishes general technical literature but does not prescribe this universal coefficient pair. True physical cable losses follow quadratic resistive heating ($I^2 R$), depending on operating voltage, conductor cross-section, and temperature; converter stations have no-load and standby losses. This proxy is labeled `ASSUMPTION` and requires sensitivity evaluation before baseline reliance.

---

## 5. Dated Market Rules

### Deficit Description
Wholesale market settlement and trading procedures require dated, version-pinned releases reflecting the 2024 calendar baseline, rather than mutable live website hubs. Specifically, BSC Section T cashout calculations and SEMOpx auction rules (DAM, IDA1–3, IDC) must be locked to historical regulatory instruments in force during 2024.

### Candidate Sources

#### Candidate 5.1: Elexon Digital BSC Baseline Releases (Section T)
- **URL:** [Elexon 2024 Change Releases Register](https://www.elexon.co.uk/bsc/release/list-of-change-releases-for-2024/) and [BSC Sections Index](https://www.elexon.co.uk/bsc-and-codes/balancing-settlement-code/bsc-sections/)
- **Publisher:** Elexon Limited
- **Licence Text / Link:** [Licence to use BMRS open data](https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/) / BSC statutory documentation.
- **Likely D-56 Class:** `research-only` (PC-5; statutory code legal text).
- **Access Method:** Open direct download / web access (`bsc.elexon.co.uk` failed DNS; `bscdocs.elexon.co.uk` returned 403 during review).
- **Coverage:** Great Britain; 2024 calendar baseline.
- **Evaluation:** Elexon's 2024 release register confirms approved releases on 2 April 2024 (including Modification P451) and 7 November 2024 (including Modifications P415 and P473 regarding Virtual Lead Party deviation volumes and Supplier Compensation Cashflow $SCC_p$), alongside other 2024 releases.
  - *Version Mapping Caveat:* The specific assignment of Section T to **v38.0** (for Jan–Oct) and **v41.0** (for Nov–Dec) is **`UNVERIFIED`** as neither Section T change tables nor impacted-document matrices were inspected to establish exact section versions. A full BSC release date does not establish that Section T was re-versioned on that exact day.
  - Section T provides the algebraic formulation for system buy/sell prices, net imbalance volume ($NIV$), Continuous Acceptance Duration Limit ($CADL$), Replacement Price, and cashflow hierarchies.

#### Candidate 5.2: SEMOpx Operating Procedures & Market Rules (2024 Baseline)
- **URL:** [SEMOpx Intraday Market](https://www.semopx.com/markets/intraday-market) and [SEMOpx Day-Ahead Market](https://www.semopx.com/markets/day-ahead-market)
- **Publisher:** SEMOpx (EirGrid / SONI / EPEX SPOT joint exchange operator)
- **Licence Text / Link:** SEMOpx legal disclaimers (all rights reserved).
- **Likely D-56 Class:** `research-only` (PC-5).
- **Access Method:** Open direct download (PDF operating procedures).
- **Coverage:** All-island Ireland (I-SEM); calendar year 2024.
- **Evaluation:** Primary publisher schedules define trading milestones:
  - *Day-Ahead Market (DAM):* Order book gate closure at **11:00** (auction run daily). Clock convention and DST handling must be verified from dated operating procedures before hardcoding UTC vs local time.
  - *Intraday Auctions (IDA):*
    - **IDA1:** Order book gate closure at **17:30 D-1** (publisher-verified; correcting the previously cited 17:45).
    - **IDA2:** Order book gate closure at **08:00 D**.
    - **IDA3:** Order book gate closure at **14:00 D**.
  - *Intraday Continuous (IDC):* Continuous trading up to 30 minutes prior to delivery.
  - *Price Boundaries:* Technical price limits (e.g. -€500/MWh to +€4,000/MWh) must not be uniformly assigned across all DAM, IDA, and continuous products without inspecting product-specific 2024 rules; historical parameter limits remain `UNVERIFIED`.

#### Candidate 5.3: SEM Committee I-SEM Trading and Settlement Code (Part B Dated Baseline)
- **URL:** [SEM Committee Publications](https://www.semcommittee.com/publications/) and [SEMO Market Rules](https://www.sem-o.com/rules-and-governance/rules/)
- **Publisher:** Single Electricity Market Committee (CRU / UR) & SEMO
- **Licence Text / Link:** Statutory market code disclaimers (all rights reserved).
- **Likely D-56 Class:** `research-only` (PC-5).
- **Access Method:** Open direct download.
- **Coverage:** All-island Ireland; balancing market rules, imbalance settlement, capacity market charges, and currency conversion mechanics in force in 2024.
- **Evaluation:** Governs ex-post imbalance settlement and dispatch rules. Provides the legal baseline for Part B settlement, but the specific dated 2024 baseline document version remains to be verified during pipeline ingestion.

### Recommendation & Proxy Formulation
- **Recommended Candidate:**
  - For GB: Retain confirmed 2024 BSC release milestones (2 April and 7 November 2024, Candidate 5.1), leaving specific Section T version numbers marked `UNVERIFIED`.
  - For Ireland: Pin SEMOpx 2024 operating schedules with corrected IDA1 gate closure (17:30 D-1, Candidate 5.2) and SEM Committee Part B baseline (`Candidate 5.3`).
- **Standard Market Proxy / Assumptions:**
  - **ASSUMPTION:** [Q-P-04] Where commercial market clearing software (e.g. Euphemia MILP solver) cannot be run due to proprietary licensing, an author-proposed continuous merit-order clearing proxy (linear dispatch sorted by generator SRMC) is adopted as an exploratory proxy.
  - *Unit Commitment Caveat:* This continuous dispatch simplification is **NOT** an accepted replacement for D-26 / FR-7 unit-commitment requirements (which mandate integer on/off decisions, minimum up/down times, start-up costs, and ramp limits). It is strictly an exploratory proxy until full unit-commitment dispatch is solved.

---

## 6. Fuel, Carbon, and Subsidy Vectors

### Deficit Description
Unit dispatch modelling requires daily/half-hourly Short-Run Marginal Cost (SRMC) vectors. UK ETS auction clearing data has unverified third-party redistribution rights; EU ETS auction series are unacquired; World Bank commodity price indices are monthly; DESNZ fossil fuel assumptions are annual long-term projections; LCCC CfD portal requires authentication; Ofgem Renewables Obligation parameters lack an identified machine-readable API; generator heat rates and variable O&M parameters remain unacquired.

### Candidate Sources

#### Candidate 6.1: ICE End-of-Day UKA/EUA Auction Results & GOV.UK Public Reports
- **URL:** [ICE UKA Auctions](https://www.theice.com/emissions/auctions/uk-emission-allowances) and [GOV.UK UK ETS Markets Guidance](https://www.gov.uk/government/publications/taking-part-in-the-uk-emissions-trading-scheme-markets)
- **Publisher:** Intercontinental Exchange (ICE Futures Europe) / UK ETS Authority (DESNZ)
- **Licence Text / Link:** ICE Market Data Terms (proprietary; commercial redistribution restricted); GOV.UK guidance under Open Government Licence (OGL v3.0).
- **Likely D-56 Class:** `research-only / unverified` (PC-5; exchange redistribution rights unverified).
- **Access Method:** Open web inspection (clearing price published after each fortnightly auction).
- **Coverage:** United Kingdom (UKA) and European Union (EUA; note that EU ETS auctions are hosted by EEX, not ICE); calendar year 2024.
- **Evaluation:** Primary pricing instrument for UK Allowances (£/tCO2e). Governed by statutory auction reserve price (£22/tCO2e for UK ETS). Redistribution of raw exchange price series for commercial purposes is restricted without an exchange data licence.

#### Candidate 6.2: DESNZ Electricity Generation Costs 2023 / 2024 (Assumptions Report)
- **URL:** [DESNZ Electricity Generation Costs 2023](https://www.gov.uk/government/publications/electricity-generation-costs-2023) and [Main Report PDF](https://assets.publishing.service.gov.uk/media/6556027d046ed400148b99fe/electricity-generation-costs-2023.pdf)
- **Publisher:** Department for Energy Security and Net Zero (DESNZ)
- **Licence Text / Link:** [Open Government Licence v3.0 (OGL v3.0)](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/)
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open direct download (report PDF and Annex A assumptions table).
- **Coverage:** United Kingdom; technology-level cost and operational assumptions for future commissioning vintages (2025–2040).
- **Evaluation:** DESNZ publication is a valid OGL source for generic technology cost benchmarks. However:
  - It provides broad generation-cost assumptions for future build, NOT observed heat rates or efficiencies for the existing 2024 operating generator fleet.
  - No specific sheet, row, or cell was cited for existing CCGT 52% or OCGT 35% efficiencies or exact variable O&M ranges.
  - The exact numerical defaults for existing thermal plant efficiencies must be treated as modeling assumptions rather than verified measured attributes.

#### Candidate 6.3: Ofgem Renewables Obligation Annual Determinations & LCCC CfD Register
- **URL:** [Ofgem 2024/25 RO Buyout Notice](https://www.ofgem.gov.uk/publications/renewables-obligation-ro-buy-out-price-mutualisation-threshold-and-mutualisation-ceilings-2024-2025) and [LCCC CfD Register](https://www.lowcarboncontracts.uk/cfd-register/)
- **Publisher:** Office of Gas and Electricity Markets (Ofgem) & Low Carbon Contracts Company (LCCC)
- **Licence Text / Link:** Ofgem notices published under Crown copyright / Open Government Licence (OGL v3.0, excluding third-party content); LCCC register terms unverified (returned HTTP 403 in review).
- **Likely D-56 Class:** Split: `open-commercial` for Ofgem RO determinations; `research-only / unverified` for LCCC CfD register.
- **Access Method:** Open direct download (Ofgem notice PDF) / Gated portal (LCCC register).
- **Coverage:** UK; calendar year 2024 across two distinct obligation years:
  - *2023/24 Obligation Year (1 January 2024 – 31 March 2024):* Buyout price was **£59.01 per ROC**; obligation level was 0.469 ROCs/MWh in GB and 0.184 ROCs/MWh in NI.
  - *2024/25 Obligation Year (1 April 2024 – 31 December 2024):* Buyout price is **£64.73 per ROC**; obligation level was announced as 0.491 ROCs/MWh in GB (revised from 0.487 on 26 March 2024) and 0.192 ROCs/MWh in NI.
- **Evaluation:** Ofgem notice establishes statutory buyout benchmarks. However:
  - A calendar 2024 simulation must implement the obligation-year transition on 1 April 2024 (£59.01 $\to$ £64.73).
  - The supplier buyout price is an administrative compliance benchmark, not the net generator ROC subsidy (which depends on ROC market recycle values and technology banding from 0.5 to 2.0 ROCs/MWh).
  - LCCC register access was blocked in review, so CfD strike price availability remains unverified.

### Recommendation & Proxy Formulation
- **Recommended Candidate:**
  - For renewable subsidy benchmarks: **Ofgem RO Determinations** (`Candidate 6.3`, `open-commercial`), incorporating the 1 April 2024 obligation-year transition (£59.01 $\to$ £64.73).
  - For commodity vectors: Use **World Bank Commodity Price Data** (`SRC-37b`, `open-commercial`, CC BY 4.0 for Bank-produced Pink Sheet data) with daily interpolation, paired with fortnightly **UK ETS auction results** (`Candidate 6.1`, `research-only / unverified`, PC-5).
  - For existing plant operating parameters: Treat technology parameters as explicit assumptions (`Candidate 6.2`).
- **Generator Short-Run Marginal Cost (SRMC) Proxy Formulation:**
  For each thermal generator unit $g$ with fuel type $f \in \{\text{Gas}, \text{Coal}, \text{Oil}, \text{Biomass}\}$:
  $$\text{SRMC}_g(t) = \frac{\text{FuelPrice}_f(t) + \text{EmissionFactor}_f \times \text{CarbonPrice}(t)}{\eta_g} + \text{VOM}_g$$
  where:
  - **ASSUMPTION:** [Q-P-05] Net thermal efficiency defaults $\eta_g$ (e.g. 0.52 for CCGT, 0.35 for OCGT) and variable O&M defaults ($\text{VOM}_g \approx \text{£3.50–£5.00/MWh}$ for CCGT, $\text{£5.00–£8.00/MWh}$ for OCGT) are author-proposed technology proxies, NOT observed existing-plant heat rates.
  - Fuel units (e.g. $/MMBtu or $/therm to £/MWh_th), carbon emissions basis ($t\text{CO}_2/\text{MWh}_{\text{th}}$), and GBP/EUR exchange rates must be explicitly converted before evaluating the SRMC equation.

---

## 7. Weather Reanalysis Acquisition

### Deficit Description
Physical renewable generation profiles require gridded meteorological variables (wind speed at hub height, solar irradiance, ambient temperature). Zero bytes of Copernicus ERA5 reanalysis data have been downloaded in the repository due to owner API credential prerequisites; Met Éireann MÉRA reanalysis terminated in August 2019 and does not cover the 2024 baseline; automated power-curve conversion software remains unintegrated into the data pipeline.

### Candidate Sources

#### Candidate 7.1: Copernicus Climate Data Store (CDS) ERA5 Hourly Single Levels
- **URL:** [Copernicus CDS ERA5 Reanalysis](https://cds.climate.copernicus.eu/datasets/reanalysis-era5-single-levels)
- **Publisher:** European Centre for Medium-Range Weather Forecasts (ECMWF) / Copernicus Climate Change Service (C3S)
- **Licence Text / Link:** [Licence to use Copernicus Products](https://cds.climate.copernicus.eu/api/v2/terms/static/licence-to-use-copernicus-products.pdf) (incorporates Creative Commons Attribution 4.0 International, CC BY 4.0).
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Programmatic API via `cdsapi` client library (requires creating a free user account, accepting licence terms via the web interface on first access, and configuring user API credentials in `~/.cdsapirc`).
- **Coverage:** Global regular lat/lon grid at $0.25^\circ \times 0.25^\circ$ resolution (~31 km), covering GB, Northern Ireland, and the Republic of Ireland. All 8,784 hours of calendar year 2024.
- **Variables:** $100\text{ m}$ $u$- and $v$-components of wind speed, Surface Solar Radiation Downwards (`ssrd`, $\text{J}/\text{m}^2$), $2\text{ m}$ air temperature (`t2m`, $\text{K}$), surface pressure (`sp`, $\text{Pa}$).
- **Evaluation:** Industry benchmark meteorological reanalysis for energy systems modeling. Open-commercial under CC BY 4.0, but requires initial interactive terms acceptance before automated headless retrieval.

#### Candidate 7.2: UK Met Office / CEDA MIDAS Open Hourly Surface Weather Observations
- **URL:** [CEDA MIDAS Open Catalogue Record](https://catalogue.ceda.ac.uk/uuid/220a6561521f4bd42e93099438613f31)
- **Publisher:** UK Met Office / Centre for Environmental Data Analysis (CEDA)
- **Licence Text / Link:** [Open Government Licence v3.0 (OGL v3.0)](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/)
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Free CEDA user registration required for automated bulk HTTP/FTP downloads.
- **Coverage:** Great Britain and Northern Ireland; hourly observations across ~250 terrestrial synoptic weather stations throughout 2024.
- **Evaluation:** Provides empirical ground-truth weather station observations (measured wind speed, direction, ambient dry-bulb temperature, global horizontal irradiance where equipped). Essential for statistical validation and empirical bias correction of downscaled ERA5 grid cells. Record-level completeness across all stations requires empirical verification during ingestion.

#### Candidate 7.3: Met Éireann Historical Weather Station Hourly Records
- **URL:** [Met Éireann Historical Data](https://www.met.ie/climate/available-data/historical-data) and [Data.gov.ie Met Éireann Portal](https://data.gov.ie/organization/met-eireann)
- **Publisher:** Met Éireann (National Meteorological Service of Ireland)
- **Licence Text / Link:** Open Data Licence Ireland (CC BY 4.0) via data.gov.ie extracts; general website terms apply to direct `met.ie` bulk scraping.
- **Likely D-56 Class:** Mixed (`open-commercial` via data.gov.ie extracts; `research-only / unverified` via direct scraping).
- **Access Method:** Open direct download (CSV files per synoptic station; no credentials required).
- **Coverage:** Republic of Ireland; hourly records across 25 primary synoptic stations (e.g., Dublin Airport, Cork Airport, Shannon Airport, Athenry, Valentia Observatory, Malin Head) for 2024.
- **Evaluation:** Directly circumvents the limitation that MÉRA reanalysis ended in 2019. Delivers 2024 hourly wind speed, temperature, relative humidity, and solar radiation observations across Ireland. Uninterrupted completeness for all 8,784 hours requires verification per station.

### Recommendation & Proxy Formulation
- **Recommended Candidate:** Primary spatial weather driver: **Copernicus CDS ERA5** (`Candidate 7.1`, `open-commercial`), paired with **Met Office MIDAS Open** (`Candidate 7.2`) and **Met Éireann Hourly Station Records** (`Candidate 7.3`) for ground-truth validation.
- **Power Curve Conversion & Fallback Proxy:**
  1. *Wind Turbine Conversion:* Use standard IEC 61400-1 Class I/II/III references. Note that IEC classes specify design envelopes, NOT a unique manufacturer power curve; dated manufacturer/library power curves (e.g. from windpowerlib) must be calibrated.
     - **ASSUMPTION:** [Q-P-06] Scaling wind speed to hub height using the power-law profile:
       $$v(t) = v_{\text{ref}}(t) \cdot \left(\frac{h_{\text{hub}}}{h_{\text{ref}}}\right)^\alpha$$
       with assumed shear exponents $\alpha \approx 0.143$ onshore and $\alpha \approx 0.11$ offshore is an unvalidated author-proposed default.
  2. *Solar PV Conversion:* Convert Surface Solar Radiation Downwards ($G_{\text{horiz}}$ in $\text{W}/\text{m}^2$) to cell temperature and power output:
     $$CF_{\text{solar}}(t) = \frac{G_{\text{POA}}(t)}{1000\text{ W}/\text{m}^2} \times \left[1 - \gamma \cdot (T_{\text{cell}}(t) - 25^\circ\text{C})\right]$$
     - **ASSUMPTION:** [Q-P-07] Temperature derating coefficient $\gamma \approx 0.004 / ^\circ\text{C}$ is an unvalidated modeling default.
  3. *Zero-Download Synthetic Fallback Limitations:* If CDS credentials are not configured, synthesizing hourly renewable series by scaling national aggregate outturns from Elexon FUELHH (`SRC-31`) and SEMO Smart Grid Dashboard (`SRC-24`) is strictly an emergency reconstruction fallback.
     - **Honest Spatial Assessment:** Inverse-distance weighting of national outturns **CANNOT recover lost spatial detail**, local wind speed gradients, site-specific capacity factors, or microclimate weather variation without geographically dispersed donor observations.
     - **Circularity Disclosure:** Reusing target outturn generation to reconstruct input series creates circular validation unless an explicit independent holdout partition is strictly enforced.

---

## 8. Statutory and Market Calendars

### Deficit Description
Civil bank holidays, market trading calendars, clock transitions (DST), and interval indexes must be strictly pinned to immutable repository artifacts. Access to official Irish public holiday data returned HTTP 403 in initial queries; divergences between UK bank holidays and Irish statutory holidays remain unharmonised; repeated local clock hours during autumn transitions must not be erroneously deduplicated.

### Candidate Sources

#### Candidate 8.1: Workplace Relations Commission & GOV.IE Statutory Public Holiday Register
- **URL:** [WRC Public Holidays Guide](https://www.workplacerelations.ie/en/what_you_should_know/public-holidays/) and [GOV.IE](https://www.gov.ie/)
- **Publisher:** Workplace Relations Commission & Government of Ireland
- **Licence Text / Link:** [Irish Public Sector Information / Open Data Licence](https://data.gov.ie/licence) (CC BY 4.0 / OGL equivalent).
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open web access / tabular documentation.
- **Coverage:** Republic of Ireland; all 10 statutory public holidays for 2024 enacted under the *Organisation of Working Time Act 1997* (as amended):
  1. *1 January 2024:* New Year's Day
  2. *5 February 2024:* St. Brigid's Day (first Monday in February)
  3. *17 March 2024:* St. Patrick's Day (Sunday)
  4. *1 April 2024:* Easter Monday (**Note: Good Friday 29 March is NOT a statutory public holiday in Ireland**)
  5. *6 May 2024:* May Day (first Monday in May)
  6. *3 June 2024:* June Bank Holiday (first Monday in June)
  7. *5 August 2024:* August Bank Holiday (first Monday in August)
  8. *28 October 2024:* October Bank Holiday (last Monday in October)
  9. *25 December 2024:* Christmas Day
  10. *26 December 2024:* St. Stephen's Day
- **Evaluation:** Provides verified legal baseline for Irish public holidays. Note the legal distinction: the WRC specifies employee statutory benefit entitlements (e.g. paid day off within a month, additional annual leave, or an extra day's pay when a holiday falls on a Sunday); it does not automatically close all commercial operations or shift the statutory holiday date. Actual transmission demand profiles must account for this civil vs commercial distinction. Resolves jurisdiction splits (e.g. Good Friday is observed in GB/NI but not statutory in ROI; St. Brigid's Day is observed in ROI only).

#### Candidate 8.2: UK Government Cabinet Office Bank Holidays API (`SRC-33`)
- **URL:** [https://www.gov.uk/bank-holidays.json](https://www.gov.uk/bank-holidays.json)
- **Publisher:** UK Cabinet Office
- **Licence Text / Link:** [Open Government Licence v3.0 (OGL v3.0)](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/)
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open REST API (JSON format; no authentication required).
- **Coverage:** United Kingdom; official statutory bank holidays partitioned by jurisdiction: `england-and-wales`, `scotland`, and `northern-ireland` for 2024.
- **Evaluation:** Authoritative programmatic JSON endpoint for UK jurisdictions. Includes regional holidays such as St. Andrew's Day (Scotland), 12th July Battle of the Boyne (Northern Ireland), and Late August Holiday (England and Wales).

#### Candidate 8.3: IANA Time Zone Database (`tzdata` Release `2024b`)
- **URL:** [IANA Time Zone Database](https://www.iana.org/time-zones), [tzdb-2024b NEWS](https://data.iana.org/time-zones/tzdb-2024b/NEWS), and [tzdb LICENSE](https://data.iana.org/time-zones/tzdb-2024b/LICENSE)
- **Publisher:** Internet Assigned Numbers Authority (IANA)
- **Licence Text / Link:** Public Domain ([IANA Time Zone Database Licence](https://data.iana.org/time-zones/tzdb-2024b/LICENSE); code portions BSD-3-Clause).
- **Likely D-56 Class:** `open-commercial`
- **Access Method:** Open direct download (tarball source; incorporated in standard system packages and crates).
- **Coverage:** Global timezones, specifically `Europe/London` and `Europe/Dublin`.
- **Evaluation:** Release `2024b` was issued on **4 September 2024**. It codifies DST transitions for 2024:
  - *Spring Forward (23-hour day, 46 half-hour settlement periods):* Sunday, 31 March 2024 at 01:00 UTC ($\to 02:00\text{ local}$).
  - *Fall Back (25-hour day, 50 half-hour settlement periods):* Sunday, 27 October 2024 at 02:00 local ($\to 01:00\text{ local}$).
  - *As-of Qualification:* Because release 2024b was published on 4 September 2024, proposing it as a runtime dependency pin for 2024 historical reconstructions is valid, but it cannot be assumed to have been available at earlier 2024 forecast origins without an explicit as-of reconstruction argument.
  - Statutory nuance: In GB, standard time is GMT and daylight saving is BST (UTC+1). In Ireland, under the *Standard Time (Amendment) Act 1971*, standard time is Irish Standard Time (IST, UTC+1 in summer) with winter time adjustment (UTC in winter).

### Recommendation & Proxy Formulation
- **Recommended Candidate:** Bundle **GOV.UK Bank Holidays JSON** (`Candidate 8.2`) and **WRC Irish Public Holidays Register** (`Candidate 8.1`) into a single version-pinned repository calendar JSON fixture (`fixtures/calendars/2024-statutory-calendars.json`), driven by **IANA tzdata 2024b** (`Candidate 8.3`).
- **Clock Transition Handling Rule:**
  All simulator engine intervals operate natively in continuous monotonic UTC timestamps ($[t_{\text{start}}, t_{\text{end}})$ with 1800-second step duration).
  For local settlement reporting, map UTC to local settlement periods:
  - *Spring Forward transition:* 46 settlement periods ($SP \in [1, 46]$).
  - *Autumn Fall Back transition:* 50 settlement periods ($SP \in [1, 50]$).
  The repeated local wall-clock hours ($01:00–02:00$) during the autumn fall-back transition are distinguished via explicit fold/offset flags ($\text{offset} = +01:00$ vs $\text{offset} = +00:00$) and settlement period indices ($SP = 3$ vs $SP = 5$), strictly preventing interval deduplication.

---

## 9. Internal Schema, Identity Inputs & Fixtures (Implementation Deficit INT-01)

> [!NOTE]
> **Internal Engineering Note (Not an Acquired Data Source):**
> Deficit 9 is internal schema, identity, and fixture engineering implementation work (`INT-01`), as originally categorized in the M1 data inventory. It represents internal repository software deliverables, not an external dataset to be acquired. Accordingly, D-56 dataset licensing classification is **N/A**, and implementation status is **PENDING**. External standards (CGMES, Frictionless) cited below serve strictly as engineering design references.

### Deficit Description
The canonical scenario data model currently exists only as an architectural specification in ADR 0002. A complete, auditable scenario input engine requires: independent hashing of scenario input identity and execution identity; distinction between as-of forecast backtests and historical reconstructions; typed domain records with permanent opaque UUIDs; three distinct timestamps (effective interval $[t_{\text{from}}, t_{\text{to}})$, publication/revision time, acquisition time); version-pinned mappings for calendars, transformations, cost models, FX vectors, and proxy constraints; and rights lineage tracking with PC-1–PC-7 clearance tags.

### Design References & Internal Implementation

#### Candidate 9.1: ENTSO-E Common Grid Model Exchange Standard (CGMES v3.0 / IEC 61970) [Design Reference]
- **URL:** [ENTSO-E CIM for Grid Models](https://www.entsoe.eu/data/cim/cim-for-grid-models/) and [IEC Standards](https://iec.ch/)
- **Publisher:** European Network of Transmission System Operators for Electricity (ENTSO-E) / International Electrotechnical Commission (IEC)
- **Licence Text / Link:** IEC standards documentation copyright (statutory purchase/membership required; publicly accessible profiles do not grant open commercial redistribution of standard text).
- **Likely D-56 Class:** N/A (External Standard / Design Reference)
- **Access Method:** Open profile download (UML profiles and XSD schemas).
- **Evaluation:** Serves strictly as an architectural reference profile for entity naming and topological typing. CGMES uses persistent Master Resource Identifiers (mRIDs). ADR 0002 adopts this principle by specifying opaque UUIDs for domain entities, while preserving CIM mRIDs as external aliases.

#### Candidate 9.2: Frictionless Data Package Specification [Design Reference]
- **URL:** [Frictionless Data Package](https://specs.frictionlessdata.io/data-package/) and [JSON Schema Draft 2020-12](https://json-schema.org/draft/2020-12/schema)
- **Publisher:** Open Knowledge Foundation (OKF) & IETF
- **Licence Text / Link:** [Creative Commons Attribution 4.0 International (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/)
- **Likely D-56 Class:** N/A (Specification / Design Reference)
- **Access Method:** Open web standard.
- **Evaluation:** Architectural pattern for content-addressed dataset release manifests, column-level typing, and SHA-256 integrity verification.

#### Candidate 9.3: Internal Repository Implementation (`INT-01` / ADR 0002 Data Model)
- **URL:** [ADR 0002: Network and Scenario Data Model](../../architecture/adr/0002-network-and-scenario-data-model.md)
- **Publisher:** Internal Repository Implementation
- **Licence Text / Link:** Repository license (MIT / Apache-2.0 dual license).
- **Likely D-56 Class:** **N/A** (Internal Code / Schema Deliverable; Status: **PENDING**)
- **Access Method:** Proposed internal Rust crate (e.g. `crates/energy-scenario-schema/`, not yet created in the repository tree).
- **Evaluation:** Implements the requirements of ADR 0002, ADR 0004, and ADR 0005.

### Implementation Checklist Specification
In accordance with ADR 0002, the pending internal implementation (`INT-01`) must instantiate:
1. *Opaque UUIDs:* Every substation, bus, AC branch, DC link, transformer, generator, storage asset, and demand connection is assigned an immutable opaque UUID. Mutable human strings, DNO names, and bus numbers are stored strictly as external aliases.
2. *Tri-Temporal Interval Tracking:* Every entity revision and observed parameter records three timestamps:
   - `effective_range`: Half-open interval $[t_{\text{from}}, t_{\text{to}})$ in UTC seconds.
   - `publication_time`: Verified source publication or notice timestamp (labeled as `reconstruction` where unavailable).
   - `acquisition_time`: SHA-256 manifest ingestion timestamp.
3. *Independent Content Hashing:*
   - `input_identity_hash`: Deterministic SHA-256 hash computed over canonicalized scenario TOML, entity parameters, time series arrays, and referenced dataset manifest hashes.
   - `execution_identity_hash`: SHA-256 hash computed over `input_identity_hash` combined with solver binary version, compiler flags, hardware runner ID, tolerance parameters, and seed.
4. *Rights Lineage Tags:* Every input dataset carries an auditable classification tag (`open-commercial` or `research-only`) mapped to canonical pre-commercial tracking IDs (`PC-1` through `PC-7`), preventing commingling in commercial SaaS profiles.
5. *Test Fixtures:* Minimal synthetic test scenarios with verifiable SHA-256 release locks to validate deserialization, schema validation, and constraint resolution.

---

## 10. Summary Deficit Resolution Matrix

| Deficit # | Deficit Category | Primary Recommended Candidate Source | D-56 Licence Class | Access Requirement | Primary Recommended Proxy / Mitigation |
|:---:|---|---|:---:|:---:|---|
| **1** | Commissioned 2024 Network Fleet | NESO ETYS 2024 Appendix B (metadata inspected) + Elexon BMRS BMU / MDD API | `research-only` (PC-1) / `open-commercial` | Open download / Open REST API | Convert branch impedances to 100 MVA base; continuous ratings proxy marked as `ASSUMPTION: 0.88 × post-fault rating (author proxy, unverified)`. |
| **2** | Proxy Rule Parameters & Cut-Sets | EirGrid/SONI Joint 2024 Report (§3.3) & NESO Day-Ahead Constraints | `research-only` (PC-2) / `open-commercial` | Open PDF / Open CKAN API | Model verified SNSP (75%), enduring 4 ROI / 3 NI min units effective **7 April 2024** (pre-April trial marked `UNVERIFIED`), and GB inertia 140 $\to$ 120 GVA·s (intermediate dates marked `UNVERIFIED`); signed cut-sets from bus partition sets. |
| **3** | Embedded Generation Decomposition | NESO Historic Demand 2024 + Sheffield Solar PV_Live | `open-commercial` | Open CKAN API / Open REST API | Starting identity: $ND + \text{Embedded Solar} + \text{Embedded Wind}$ (pumping not subtracted from $ND$); Irish micro-PV capacity from ESB reported endpoints (~300 MW to ~500 MW), intra-year linear interpolation marked `ASSUMPTION: [Q-P-02]`. |
| **4** | Directional Interconnector Losses & TLM | Elexon Insights OpenAPI (`SRC-50`) + Regulatory Access Rules | `open-commercial` / Mixed | Open REST API / Open PDF filings | Apply $TLM = 1.0$ for interconnector BMUs in GB settlement (BSC P278 legal rule); apply certified commercial loss factors (BritNed 3.0%, ElecLink 2.9%, Nemo unverified operator factor); HVDC physical losses marked `ASSUMPTION: 0.75% per converter + 0.3%/100 km cable (author proxy, not CIGRE standard)`. |
| **5** | Dated Market Rules | Elexon BSC 2024 Releases & SEMOpx Procedures 2024 | `research-only` (PC-5) | Open web / Open PDF download | Pin confirmed 2024 BSC release dates (Section T versions marked `UNVERIFIED`); pin SEMOpx operating procedures with corrected IDA1 gate closure (**17:30 D-1**); continuous SRMC dispatch marked `ASSUMPTION: exploratory proxy (not an accepted replacement for D-26 / FR-7 unit commitment)`. |
| **6** | Fuel, Carbon & Subsidy Vectors | Ofgem RO Determinations + DESNZ Assumptions Report + World Bank Pink Sheet | `Mixed / UNVERIFIED` | Open direct download / Web inspection | Apply RO buyout with 1 April 2024 transition (£59.01 $\to$ £64.73/ROC, `open-commercial`); DESNZ CCGT 52% and OCGT 35% efficiencies marked `ASSUMPTION: technology proxies (not measured heat rates)`; carbon auction series `research-only / unverified`. |
| **7** | Weather Reanalysis Acquisition | Copernicus CDS ERA5 Hourly Single Levels + MIDAS / Met Éireann Stations | `open-commercial` | Free CDS account (web terms acceptance) / Free CEDA account | Downscale ERA5 wind/solar/temp; validate against Met Office & Met Éireann synoptic stations; fallback outturn scaling explicitly marked as a synthetic reconstruction fallback that **cannot recover lost spatial detail** and requires holdout validation. |
| **8** | Statutory & Market Calendars | WRC Irish Public Holidays Register + GOV.UK Bank Holidays JSON + IANA tzdata 2024b | `open-commercial` | Open web / Open JSON API / Public domain | Harmonise ROI (10 holidays; statutory entitlement vs market calendar) and UK calendars in JSON fixture; IANA tzdata 2024b (4 Sept 2024); **46 half-hour periods (spring forward) / 50 periods (autumn fold)**. |
| **9** | Internal Schema, Identity & Fixtures (`INT-01`) | Internal ADR 0002 Implementation + Frictionless / CGMES Design References | **N/A** (Internal Code / Schema) | Internal repository implementation (Status: **PENDING**) | Implement tri-temporal intervals $[t_{\text{from}}, t_{\text{to}})$, opaque entity UUIDs, separate input vs execution SHA-256 hashes, PC-1–PC-7 clearance tags, and TOML test fixtures in proposed scenario crate. |

---

## 11. Changes in Response to Review

This revision directly resolves all 16 findings raised in the GPT review ([gpt-2026-11-15-pr58-deficit-sources.md](../../requirements/reviews/gpt-2026-11-15-pr58-deficit-sources.md)):

| Finding ID | Severity | Finding Summary | Action Taken / Fix in This Revision |
|:---:|:---:|---|---|
| **F-01** | High | Incorrect effective date for Irish minimum units (cited 17 July instead of 7 April) | Corrected enduring policy date to **7 April 2024** citing §3.3 of the joint *Annual Renewable Constraint and Curtailment Report 2024* (V1.0). Marked pre-7 April trial operational schedule as `UNVERIFIED`. Updated Section 2 and Summary Row 2. |
| **F-02** | High | Universal 0.88 thermal-rating factor had no engineering derivation from P27 | Removed claim that P27 establishes a universal 0.88 factor across asset classes. Reclassified 0.88 continuous-to-emergency conversion as an unverified author-proposed `ASSUMPTION: [Q-P-01]` requiring sensitivity testing and asset-specific ratings. |
| **F-03** | High | HVDC converter/cable proxy attributed to unidentified "CIGRE B4" standard | Removed attribution to CIGRE B4. Reclassified 0.75% per converter and 0.30%/100 km cable as author-proposed `ASSUMPTION: [Q-P-03]`. Disclosed physics of quadratic $I^2 R$ cable losses and converter standby losses. |
| **F-04** | High | Commercial nomination loss factors conflated with physical loss models | Separated commercial deemed-volume nomination rules (BritNed 3.0%, ElecLink 2.9%, Nemo unverified operator factor) from physical converter/cable losses. Added mathematical note on midpoint vs sending-end denominators ($\frac{1 - LF/2}{1 + LF/2} \approx 0.9704 \neq 0.97$). Marked Nemo Link factors `UNVERIFIED` pending direct operator access. |
| **F-05** | High | Elexon TLM endpoints returned 404; acceptance stack coverage unverified | Replaced dead URLs with official Elexon OpenAPI specification (`/swagger/v1/swagger.json`). Marked acceptance stack annual coverage `UNVERIFIED`. Cited primary statutory legal sources for BSC Modification P278 establishing administrative $TLM = 1.0$ for interconnector BMUs. |
| **F-06** | High | BSC Section T version mapping (v38.0 / v41.0) unevidenced | Marked specific Section T version numbers as `UNVERIFIED`. Retained confirmed 2024 BSC release milestones (2 April and 7 November 2024) and approved modification names (P415, P473, P451). |
| **F-07** | High | SEMOpx schedule contained incorrect IDA1 gate closure (17:45) and lacked dated rules | Corrected IDA1 order book gate closure to **17:30 D-1** per publisher page. Repaired SEMOpx URLs. Explicitly clarified that continuous SRMC dispatch simplification is an exploratory `ASSUMPTION: [Q-P-04]`, not an accepted replacement for D-26 / FR-7 unit commitment. |
| **F-08** | High | Irish microgeneration baseline was overprecise (350 $\to$ 498 MW) and incorrectly sourced | Replaced false precision with ESB Networks published annual performance report figures: ~300 MW (78,400 connections) at end-2023 and ~500 MW (121,700 connections) at end-2024. Marked intra-year linear interpolation as `ASSUMPTION: [Q-P-02]`. Noted separate Northern Ireland gap and rooftop vs utility scaling uncertainty. |
| **F-09** | High | Plant efficiencies (CCGT 52%, OCGT 35%) presented as DESNZ-verified | Removed claim that exact unit efficiencies were extracted from DESNZ Annex A. Clarified that DESNZ report provides technology assumptions for future builds, not existing 2024 operating fleet heat rates. Marked 52% and 35% as author-proposed `ASSUMPTION: [Q-P-05]` proxies. |
| **F-10** | High | RO buyout recommendation lacked calendar-year transition; mixed rights collapsed | Added statutory obligation-year transition on 1 April 2024 (£59.01/ROC for 2023/24 $\to$ £64.73/ROC for 2024/25 per Ofgem notice). Disaggregated Candidate 6 licences into Ofgem (OGL, `open-commercial`), Pink Sheet (CC BY 4.0, `open-commercial`), DESNZ (OGL, `open-commercial`), and ICE/LCCC (`research-only / unverified`). Updated Summary Row 6 to `Mixed / UNVERIFIED`. |
| **F-11** | High | Precise GB inertia transition dates remained unverified | Repaired NESO operability archive URL. Retained publisher-supported broad reduction from 140 to 120 GVA·s during 2024. Marked intermediate 130 GVA·s floor and exact dates (28 Feb, 19 June) `UNVERIFIED`. Kept regional floors and minimum units open. |
| **F-12** | Medium | Broken URLs and metadata promoted to verified data coverage | Repaired NESO CKAN package IDs (historic demand, day-ahead limits, 24-month limits), Sheffield Solar publisher URL, and CEDA MIDAS catalog UUID. Clarified 24-month limits are weekly forecast snapshots. Marked row-level and weather station annual completeness `UNVERIFIED`. |
| **F-13** | Medium | Item 9 misclassified as acquired dataset instead of internal implementation | Reclassified Deficit 9 as internal software/schema engineering (`INT-01`, D-56 class **N/A**, status **PENDING**). Moved into dedicated internal engineering note. Noted crate does not yet exist. Maintained CGMES and Frictionless strictly as external design references. |
| **F-14** | High | Weather fallback claimed to recover spatial detail from national outturns | Clarified that aggregate-outturn inverse-distance weighting is an emergency reconstruction fallback that **cannot recover lost spatial detail** or local weather patterns. Required located donor stations and explicit holdout validation split to prevent circularity. Marked power-law shear and PV temperature derating defaults as `ASSUMPTION: [Q-P-06, Q-P-07]`. |
| **F-15** | Medium | Summary interval count had erroneous period count; tzdata pin needed date | Corrected summary interval count to **46 periods** for spring forward and **50 periods** for autumn fall back. Documented IANA tzdata 2024b release date (4 September 2024, public domain). Clarified WRC employee statutory entitlements vs transmission demand calendar. |
| **F-16** | Medium | Licence/access conclusions lacked resource-specific evidence and consistent summaries | Provided resource-specific licence classifications across all candidates. Noted interactive terms acceptance required for Copernicus CDS. Reflected mixed and unverified rights accurately in Summary Rows 1, 4, 6, and 7. Corrected Row 9 to D-56 N/A. |
