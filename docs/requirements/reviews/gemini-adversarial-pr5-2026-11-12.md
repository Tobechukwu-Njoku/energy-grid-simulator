# Adversarial Review of PR #5: D-31 Data Spike & Transmission Feasibility

**Reviewer:** Gemini Flash 3.8 (Adversarial Review Sub-Agent)  
**Date:** 2026-11-12  
**Target:** PR #5 (`Tobechukwu-Njoku/energy-grid-simulator#5` — `spikes/data/`)  
**Scope:** `spikes/data/REPORT.md`, `spikes/data/sources.tsv`, `spikes/data/fetch.sh`, `spikes/data/inspect_networks.py`, `spikes/data/inspect_tabular.py`, `spikes/data/era5-request-2024-01.json`, and `spikes/data/samples/*`  
**Governing Context:** `AGENTS.md`, `docs/requirements/03-decisions-log.md` (specifically D-17, D-18, D-30, D-32, D-44, D-52, D-55)  

---

## Executive Summary

This review is an independent adversarial review of PR #5. Its purpose is to stress-test and break the conclusions, legal assertions, data methodologies, and proposed fallbacks in the D-31 data spike (`spikes/data/`).

PR #5 concludes that building a full GB + all-island Ireland electrical network is only **"Partially"** achievable. It asserts that while raw electrical parameters exist, the network cannot be cleared for commercial SaaS (D-17) because:
1. ETYS Appendix B lacks an affirmative commercial open licence and is blocked by NESO website terms.
2. EirGrid/SONI PSS/E cases lack commercial redistribution rights.
3. Only NGED provides an open 132-kV partial path, leaving the remaining GB DNOs unverified.
4. I-SEM 2024 Day-Ahead Market (DAM) prices could not be retrieved due to disabled frontend elements on SEMOpx.

PR #5 offers Fallbacks A–E, ranging from clearing published models (A) to reduced models (B), synthetic models (C), a two-zone boundary model (D), or an NGED-only regional pilot (E).

### Primary Findings of the Attack

1. **Regulatory False-Negative on ETYS Appendix B (ADV5-1):** The spike conflates generic promotional website browsing terms (`neso.energy/terms-and-conditions`) with statutory regulatory data licensing. ETYS is a regulatory publication mandated under Ofgem Transmission Licence Standard Condition C11. Under Ofgem’s binding **Data Best Practice (DBP)** Guidance, energy system data is legally **"Presumed Open"**. NESO's Data Portal licenses energy system data under the **NESO Open Data Licence v1.0** (derived from OGL v3.0, allowing commercial exploitation). Treating Appendix B as legally unusable based on a website copyright footer is a legal error.
2. **Grossly Incomplete Distribution Network Census (ADV5-2):** The spike treats NGED as a solitary "permissive exception" for 132-kV networks and dismisses all other DNOs as unverified. In reality, **all five remaining GB DNO groups** (UKPN, SSEN, SPEN, NPg, ENWL) and SSEN Transmission operate open data portals mandated by Ofgem, publishing LTDS, CIM network models, and GIS assets under **CC-BY 4.0** or **OGL v3.0**. **100% of GB DNO licence areas** have open 132-kV data, directly undermining the spike's coverage pessimism.
3. **Missed ENTSO-E Transparency Platform (ADV5-3):** The spike spent effort scraping SEMOpx dynamic web pages and finding disabled JavaScript selectors, concluding that I-SEM DAM prices are unacquired and interconnector flow rights unverified. It completely overlooked the primary European energy market API: the **ENTSO-E Transparency Platform**, which since 2022 is published under **CC-BY 4.0**. Complete official hourly Day-Ahead Prices (`documentType=A44`) for Ireland SEM (`10Y1001A1001A59C`) and all GB interconnector scheduled and physical flows are available via an open REST API.
4. **Distorted Fallback Evaluations & Disregard of D-55 (ADV5-5):** Option A (full model assembly) is severely understated at 8–16 person-weeks (industry precedent is multiple person-years); Option C (synthetic grid) is impossible without spatial data because D-30 bans OpenStreetMap; Option E (NGED regional pilot) fails all MVP objectives; and Option D (two-zone/boundary model) was prematurely dismissed despite the owner explicitly approving boundary proxy constraints for MVP in **D-55**.
5. **Critical Data Quality Traps (ADV5-6, ADV5-7, ADV5-8):** The spike ignores Elexon settlement run revisions (II vs. SF vs. RF), piecewise-linear BOA trajectory de-confliction algorithms, three-winding transformer star equivalent physics, shifting seasonal time-zone offsets between CET-based SDAC / I-SEM and GB clock time, and an ERA5 bounding box that cuts off four major continental interconnectors.

---

## Findings Matrix

| ID | Severity | Category | Target Location | Title |
|---|---|---|---|---|
| **ADV5-1** | High | Licensing / False Negative | `REPORT.md` §1, §2, §6; `sources.tsv` | ETYS Appendix B False Negative: Conflating Website Terms with Ofgem "Presumed Open" Mandate |
| **ADV5-2** | High | Network Sources / Coverage | `REPORT.md` §1, §3, §6, §7; `sources.tsv` | Missed Open Data Portals for All 5 Other GB DNOs (100% GB 132-kV Networks Open Under CC-BY/OGL) |
| **ADV5-3** | High | Market Data / Missed Source | `REPORT.md` §1, §2, §4, §6; `sources.tsv` | Missed ENTSO-E Transparency Platform for Ireland (I-SEM) DAM Prices and Interconnectors Under CC-BY 4.0 |
| **ADV5-4** | Med | Licensing / Inaccuracy | `REPORT.md` §1, §2, §5, §6; `sources.tsv` | ERA5 Copernicus Contract vs. CC-BY 4.0 Confusion & Unaddressed Elexon Sublicensing Pass-Through |
| **ADV5-5** | High | Fallback Architecture | `REPORT.md` §7, §8 | Severe Understatement of Option A, Invalidity of C & E, and Failure to Align with Decision D-55 |
| **ADV5-6** | High | Data Quality Traps | `REPORT.md` §3, §4, §5; `samples/*` | Unaddressed Traps: Settlement Run Vintages, BOA De-confliction, Unit Incompatibilities, and DST Shift |
| **ADV5-7** | Med | Network Physics Blunder | `inspect_networks.py` L72–83; `samples/network-statistics.json` | Electrically Invalid Three-Winding Transformer Representation in Network Diagnostic Script |
| **ADV5-8** | Med | Spatial Truncation | `era5-request-2024-01.json` L25; `REPORT.md` §5 | ERA5 Bounding Box Truncation at 3° E Excludes Critical Offshore Wind Zones and Interconnectors |
| **ADV5-9** | Low | Compliance / Hygiene | `samples/*.json`, `samples/*.csv` | Committing BMRS and Market Snapshots Without License-Mandated Redistribution Notices |

---

## Detailed Findings

### ADV5-1: ETYS Appendix B False Negative: Conflating Website Terms with Ofgem "Presumed Open" Mandate

- **Severity:** High
- **Location:** `spikes/data/REPORT.md` (§1 Executive finding, §2 Licence evidence, §6 Dataset register); `spikes/data/sources.tsv`
- **The Attack:**  
  The spike rates ETYS Appendix B as **"UNVERIFIED, not approved"** and blocks its use for the network model, asserting:
  > *"Its publication page provides the workbook, but no affirmative open-licence association was found there or in the ETYS/transmission CKAN searches... The general terms, Ownership/use/IP section, permit personal print copies and restrict commercial exploitation except as permitted by law."*

  This legal conclusion is defective and represents a false negative. The reviewer conflated generic promotional website browsing terms (`neso.energy/terms-and-conditions`) with the statutory and regulatory framework governing electricity transmission system data in Great Britain:
  1. **Statutory Licence Condition:** ETYS is produced pursuant to **Standard Licence Condition C11** (Production of information about the National Electricity Transmission System) of the Transmission Licence granted under the Electricity Act 1989. SLC C11 explicitly requires the licensee to publish system data to facilitate competition in the generation and supply of electricity.
  2. **Ofgem Data Best Practice (DBP) Guidance:** Under Standard Special Condition A30 (and equivalent distribution/transmission licence conditions enforceable under RIIO-2), NESO and Transmission Owners (NGET, SPT, SSEN Transmission) are legally bound by Ofgem's Data Best Practice Guidance. The foundational principle of DBP is that all Energy System Data is **"Presumed Open"**. Data can only be restricted if a documented Open Data Triage proves demonstrable harm regarding Critical National Infrastructure, commercial sensitivity, or personal privacy.
  3. **No Triage Restriction Exists:** Appendix B contains no customer data, no dynamic control software, and no commercially sensitive bid curves; it contains steady-state impedances and seasonal ratings that have been published annually for more than a decade without restriction.
  4. **NESO Open Data Portal Terms:** Datasets published on the NESO Open Data Portal are licensed under the **NESO Open Data Licence v1.0** (derived from and identical in scope to the UK Open Government Licence v3.0), which expressly permits commercial exploitation. Because the Drupal CMS download link (`/document/383936/download`) did not display the CKAN badge on its static landing page, the spike assumed corporate website copyright applied.

  By falsely treating ETYS Appendix B as commercially prohibited, PR #5 incorrectly declared D-32 (full transmission network) unviable and triggered an unnecessary fallback search.
- **Evidence URLs:**
  - Ofgem Data Best Practice Guidance: https://www.ofgem.gov.uk/publications/data-best-practice-guidance
  - Ofgem Electricity Transmission Licence Standard Conditions (SLC C11): https://www.ofgem.gov.uk/energy-policy-and-regulation/industry-licensing/licences-and-licence-conditions
  - NESO Open Data Licence v1.0: https://www.neso.energy/data-portal/neso-open-licence

---

### ADV5-2: Missed Open Data Portals for All 5 Other GB DNOs (100% GB 132-kV Networks Open Under CC-BY/OGL)

- **Severity:** High
- **Location:** `spikes/data/REPORT.md` (§1, §3 England/Wales 132 kV, §6 Dataset register, §7 Option E); `spikes/data/sources.tsv`
- **The Attack:**  
  In §1 and §3, the report presents National Grid Electricity Distribution (NGED) as an isolated "permissive exception":
  > *"NGED provides an important permissive exception: its LTDS tables contain 1,138 132-kV circuit rows... Other GB DNO areas, SSEN/SPT publication rights, and TO geographic layers remain UNVERIFIED rather than inferred covered."*
  
  The spike stopped researching after NGED and assumed the other 10 GB licence areas were unverified or restricted. This is factually erroneous. Under Ofgem RIIO-ED2 licence conditions mandating Data Best Practice, **every single Distribution Network Operator (DNO) in Great Britain operates an active open data portal** publishing LTDS data, CIM network models, and geospatial asset registers under permissive, commercial-friendly open licenses:
  1. **UK Power Networks (UKPN)** (3 licence areas: London / LPN, South Eastern / SPN, Eastern / EPN): Operates `ukpowernetworks.opendatasoft.com`. Releases all open datasets, including LTDS Section 2 tables, CIM models, and 33/132-kV asset geometries, under **Creative Commons Attribution 4.0 International (CC-BY 4.0)** and **Open Government Licence (OGL)**.
  2. **Scottish and Southern Electricity Networks (SSEN Distribution & SSEN Transmission)** (SHEPD in Northern Scotland, SEPD in Southern England, and Scottish Hydro Transmission): Operates `data.ssen.co.uk` and `ssentransmission.opendatasoft.com`. Explicitly licenses open datasets under **CC-BY 4.0**.
  3. **SP Energy Networks (SPEN)** (2 licence areas: SP Distribution in Central/Southern Scotland, SP Manweb in Merseyside/N. Wales): Operates `spenergynetworks.opendatasoft.com`. Datasets are released under the SP Energy Networks Open Data Licence, which explicitly states: *"These terms are based on the Creative Commons Attribution 4.0 International Public Licence."*
  4. **Northern Powergrid (NPg)** (2 licence areas: Northeast / NEDL, Yorkshire / YEDL): Operates `northernpowergrid.opendatasoft.com`. Releases datasets under the Northern Powergrid Open Data Licence v1.0, directly mirroring **OGL v3.0 / CC-BY 4.0** with explicit commercial exploitation rights.
  5. **Electricity North West (ENWL)** (1 licence area: North West): Operates `electricitynorthwest.opendatasoft.com`, publishing network asset and capacity datasets under **OGL / CC-BY 4.0**.

  Across Great Britain, **14 out of 14 DNO licence areas (100%)** provide permissively licensed, machine-readable 132-kV network models under non-share-alike terms. The report's claim that 132-kV data coverage is unverified outside NGED is flatly wrong and reflects an incomplete audit.
- **Evidence URLs:**
  - UK Power Networks Open Data Portal FAQs & CC-BY 4.0 Terms: https://ukpowernetworks.opendatasoft.com/pages/faq/
  - SSEN Transmission Open Data Portal (CC-BY 4.0): https://ssentransmission.opendatasoft.com/
  - SP Energy Networks Open Data Licence: https://spenergynetworks.opendatasoft.com/
  - Northern Powergrid Open Data Licence: https://northernpowergrid.opendatasoft.com/pages/license/

---

### ADV5-3: Missed ENTSO-E Transparency Platform for Ireland (I-SEM) DAM Prices and Interconnectors Under CC-BY 4.0

- **Severity:** High
- **Location:** `spikes/data/REPORT.md` (§1, §2 Blocked/unconfirmed reuse, §4 Backtest observations, §6 Dataset register); `spikes/data/sources.tsv`
- **The Attack:**  
  The spike claims that retrieving 2024 Day-Ahead Market (DAM) prices for Ireland failed and remains unachieved:
  > *"I-SEM day-ahead price: SEMOpx public reporting and day-ahead description checked; 2024 DAM/static/dynamic probes did not return a usable price series. The current results page comments out the DAM selector and EA-001 is disabled in its app configuration... Ask SEMOpx for historical DAM endpoint/archive and commercial rights. Do not replace DAM with imbalance price."*
  
  The report also lists SEMOpx XML interconnector flows as "UNVERIFIED, not cleared".

  This investigative failure stems from attempting to screen-scrape dynamic Single Page Applications (SPAs) on `semopx.com` while missing the central European electricity market data repository: the **ENTSO-E Transparency Platform**.
  1. **Universal Open License:** Effective 18 February 2022, the entire ENTSO-E Transparency Platform transitioned its open data policy to **Creative Commons Attribution 4.0 International (CC-BY 4.0)**. Users are expressly permitted to use, adapt, and exploit the data commercially without prior written consent.
  2. **I-SEM Day-Ahead Prices Available via REST API:** Under European Commission Regulation (EU) No 543/2013 (Transparency Regulation), Article 12.1.d, Day-Ahead Prices (`documentType=A44`) are mandated for every bidding zone. The All-Island Single Electricity Market of Ireland is cataloged under Energy Identification Code (EIC) **`10Y1001A1001A59C`** (`IE_SEM` / `BZN|IE(SEM)`). The complete 2024 hourly day-ahead auction series is directly accessible via a standard REST API call in EUR/MWh:
     ```http
     GET https://web-api.tp.entsoe.eu/api?documentType=A44&in_Domain=10Y1001A1001A59C&out_Domain=10Y1001A1001A59C&periodStart=202401010000&periodEnd=202412312300&securityToken=<TOKEN>
     ```
  3. **Interconnector Commercial & Physical Flows:** ENTSO-E publishes both scheduled commercial exchanges (`A09`) and cross-border physical flows (`A11`) for all GB/IE interconnectors (Moyle, EWIC, Greenlink, IFA, IFA2, BritNed, Nemo Link, NSL, Viking Link) under the same CC-BY 4.0 license.

  Reverse-engineering disabled DOM elements on SEMOpx created an artificial blocker for the 2024 backtest, when an authoritative, CC-BY-4.0 REST endpoint was available.
- **Evidence URLs:**
  - ENTSO-E Transparency Platform Open Data Policy (CC-BY 4.0 Announcement): https://transparency.zendesk.com/hc/en-us/articles/4417387799569-Open-data-policy-on-Transparency-Platform-CC-BY-4-0
  - ENTSO-E Transparency Platform Terms of Use: https://transparency.entsoe.eu/content/static_content/download?path=/Static%20Content/terms%20and%20conditions/220218_Transparency%20Platform%20terms%20of%20use.pdf
  - ENTSO-E REST API Implementation Guide (Document Type A44): https://transparency.entsoe.eu/content/static_content/Static%20content/web%20api/Guide.html

---

### ADV5-4: ERA5 Copernicus Contract vs. CC-BY 4.0 Confusion & Unaddressed Elexon Sublicensing Pass-Through

- **Severity:** Med
- **Location:** `spikes/data/REPORT.md` (§1, §2 Positive commercial-use clauses, §5 One weather year, §6 Dataset register); `spikes/data/sources.tsv`
- **The Attack:**  
  The spike contains legal characterization errors in both directions (too permissive and misclassified):
  1. **ERA5 Reanalysis (Copernicus Product Licence):** The report asserts that ERA5 is licensed under *"CC-BY-4.0 currently attached to the CDS dataset"*, referencing STAC catalogue metadata. This is legally imprecise. When registering an account and submitting API retrieval scripts to the Copernicus Climate Data Store (CDS), the binding legal agreement accepted by the user is the **Licence to Use Copernicus Products** (`licence-to-use-copernicus-products.pdf`), issued under the EU Copernicus Regulation (EU) No 377/2014. While this licence permits commercial exploitation without share-alike, it establishes specific administrative clauses:
     - Mandatory non-standard attribution: *"Contains modified Copernicus Climate Change Service information [Year]"*.
     - Explicit exclusion of EU / ECMWF liability and warranty under European administrative law.
     - Specific rules regarding redistribution of unaltered raw reanalysis rasters.  
     Treating ERA5 as standard CC-BY 4.0 based on a JSON metadata tag rather than the governing Copernicus contract creates compliance risk for a commercial SaaS product (D-17).
  2. **Elexon BMRS Viral Sublicensing Condition:** The spike marks Elexon BMRS data as "Commercial OK: Yes" and "Share-alike: No". While it is not copyleft, the spike ignores Section 3 of the **Elexon Licence to use BMRS open data**:
     > *"The same attribution statements must be contained in any sub-licences of the BMRS Data that You grant, together with a requirement that any further sub-licences do the same."*  
     Under decisions D-8 (API and code access) and D-17 (commercial SaaS), the simulator will expose endpoints and export datasets containing or derived from BMRS BM data. The SaaS Terms of Service must enforce this mandatory downstream sub-licensing attribution clause. Treating BMRS data as unrestricted commercial open data without designing a pass-through attribution architecture risks automatic licence termination under Section 4 of the Elexon agreement.
- **Evidence URLs:**
  - Copernicus Products Licence Text: https://cds.climate.copernicus.eu/api/v2/terms/static/licence-to-use-copernicus-products.pdf
  - Elexon Licence to use BMRS open data: https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/

---

### ADV5-5: Severe Understatement of Option A, Invalidity of C & E, and Failure to Align with Decision D-55

- **Severity:** High
- **Location:** `spikes/data/REPORT.md` (§7 Concrete fallbacks, §8 Recommended M1 gates)
- **The Attack:**  
  Section 7 proposes Fallbacks A–E for an owner decision, but the effort and fidelity analyses are severely distorted:
  1. **Option A (8–16 person-weeks) is Drastically Understated:** The report estimates 8–16 person-weeks to *"Clear and assemble the published detailed models"*. In reality, taking 1,952 literal ETYS endpoints with 20 disconnected components, reconciling 658 sites, harmonizing 14 separate DNO LTDS formats, bridging 132-kV distribution substations to 400/275-kV transmission GSPs, and reconciling 2,025 Irish PSS/E buses (which contain 132 zero-rating branches and a 380-kV base) into a fully solved, non-divergent AC/DC power flow case represents an immense undertaking. Historical academic and industry precedents (such as PyPSA-GB, which still omits full distribution, and National Grid ESO's own CIM migration) required **multiple person-years**. Promising an operational full-network model in 8–16 weeks sets up an immediate schedule failure.
  2. **Option C (Synthetic model, 6–12 person-weeks) is Blocked by D-30:** The report claims a synthetic electrical model can be built in 6–12 weeks without copying restricted topology. However, D-30 strictly prohibits OpenStreetMap (ODbL). Standard synthetic grid generators (e.g. Texas A&M, GridKit) rely heavily on OSM for substation locations, transmission corridors, and line lengths. Without OSM, and without ETYS topology, there is no permissive geospatial source for GB transmission lines (DUKES and REPD only provide generator point coordinates). Synthesizing a physically realistic 2,000-bus power flow network from zero spatial geometry without violating D-30 is an unsolved problem, not a 6-week fallback.
  3. **Option E (Regional NGED pilot, 2–4 person-weeks) Fails MVP:** The report suggests using NGED's 4 licence areas as a pilot. But NGED is a Distribution Network Operator (DNO). It contains no 400/275-kV transmission lines, no direct interconnectors, and no major transmission-connected generators (Drax, Sizewell, offshore wind), and does not set the national wholesale price or run the Balancing Mechanism. An NGED-only pilot cannot execute a Dunkelflaute scenario (D-14, D-18) and provides zero validation for transmission dispatch.
  4. **Contradiction with Decision D-55 (Proxy Constraints):** The report dismisses Option D (boundary/two-zone model) as an unacceptable fidelity compromise. However, the owner explicitly decided in **D-55**:
     > *"MVP network physics: DC network plus proxy constraints — known voltage and stability limits enter as published constraint boundaries/limits and constraint costs (NESO publications)."*  
     NESO publishes explicit boundary transfer capabilities (B0–B17) in ETYS and the Monthly Balancing Services Summary (MBSS). A calibrated boundary model satisfies D-55 and captures the majority of GB constraint spending (e.g. B6 Scotland-England export limit). The spike's all-or-nothing insistence on a 2,000-node raw physical network ignores the owner's baselined proxy-constraint decision.
- **Evidence URLs:**
  - `docs/requirements/03-decisions-log.md` (D-18, D-30, D-32, D-55)
  - Lyden et al. (2024), *PyPSA-GB: An open-source model of Great Britain's power system*, Energy Strategy Reviews: https://doi.org/10.1016/j.esr.2024.101375

---

### ADV5-6: Unaddressed Traps: Settlement Run Vintages, BOA De-confliction, Unit Incompatibilities, and DST Shift

- **Severity:** High
- **Location:** `spikes/data/REPORT.md` (§3, §4, §5); `spikes/data/inspect_tabular.py`; `spikes/data/samples/*`
- **The Attack:**  
  While the spike validated basic file parsing and row counts (e.g. 17,568 half-hours), it omitted several fundamental data engineering and domain traps:
  1. **Settlement Run Vintages (II vs. SF vs. RF):** In GB market operation, initial settlement outturn (II, Interim Information) published post-event is subject to substantial revisions across formal settlement runs:
     - **SF (Settlement Final):** R + 21 days
     - **R1 (Reconciliation 1):** R + 40 days
     - **R2 (Reconciliation 2):** R + 84 days
     - **R3 (Reconciliation 3):** R + 14 months
     - **RF (Reconciliation Final):** R + 19 months  
     Imbalance prices (System Price), Net Imbalance Volumes (NIV), and accepted BM volumes change substantially across these runs as metered data is corrected and disputed transactions are resolved. Live BMRS API queries return dynamic data snapshots that diverge from frozen historical settlement sheets. The spike does not establish which settlement run vintage is authoritative for the 2024 backtest, introducing non-reproducible validation drifts.
  2. **Trapezoidal BOA Trajectory Overlaps:** The spike sampled 65 Bid-Offer Acceptance (BOA) records for a single half-hour and noted *"integrate trajectories against FPN/pair volumes. Do not multiply every row by 0.5 h"*. However, it omits the formal precedence algorithm specified in BSC Section Q: BOAs are linear ramp rate trajectories defined by $(T_1, P_1) \to (T_2, P_2)$. When multiple overlapping acceptances are issued to the same BM unit, later acceptances overwrite, truncate, or re-baseline earlier instructions. Simply integrating each row independently results in massive double counting of accepted energy.
  3. **Unit & Parameter Scale Discrepancies:**
     - **ETYS Appendix B:** Branch impedances are expressed in **"% on 100 MVA base"** ($R_{\text{pu}} = R / 100$).
     - **NGED LTDS Table 1:** Circuit parameters are in **Ohms ($\Omega$)** for $R_1, X_1$, **microSiemens ($\mu\text{S}$)** for $B_1$, and thermal ratings are in **Amperes (A)**, NOT MVA! For example, `Winter: 644` at 132 kV represents $1.732 \times 132 \times 644 / 1000 \approx 147.2\text{ MVA}$, not 644 MVA.
     - **PSS/E RAW (Ireland):** Impedances are per-unit, but transformer branch data interpretation depends on the `CZ` flag (`CZ=1` system base, `CZ=2` transformer MVA base, `CZ=3` load loss and impedance voltage).  
     Directly feeding these numbers into a unified power flow model without explicit conversion will distort circuit susceptance by a factor of $10^6$ and line ratings by up to $4\times$.
  4. **DST / Asymmetric Market Time Zones:** I-SEM and the European Day-Ahead Market (SDAC) operate on Central European Time (CET), with trading days running from **23:00 UTC (D-1) to 23:00 UTC (D)**. In contrast, GB electricity settlement operates on local clock time (00:00 to 24:00 GMT/BST), creating a shifting 46-, 48-, and 50-period structure. ERA5 operates strictly on UTC (00:00 to 23:00 UTC). During winter (GMT), GB settlement lags the Irish market day by one hour; during summer (BST, UTC+1), GB Settlement Period 1 begins at 23:00 UTC, aligning with SDAC. Joining these three asynchronous time-series without a strict UTC timestamp anchor will introduce a seasonal 1-hour time-skew error across all market and weather variables.
- **Evidence URLs:**
  - Elexon Settlement Calendar & Run Types: https://www.elexon.co.uk/operations-settlement/settlement-operations/settlement-calendar/
  - Elexon BSC Section Q (Balancing Mechanism Activities): https://www.elexon.co.uk/bsc-and-codes/balancing-and-settlement-code/bsc-sections/
  - Energy Networks Association (ENA) LTDS Standard Data Formats: https://www.energynetworks.org/industry-hub/resource-library/

---

### ADV5-7: Electrically Invalid Three-Winding Transformer Representation in Network Diagnostic Script

- **Severity:** Med
- **Location:** `spikes/data/inspect_networks.py` (lines 72–83); `spikes/data/samples/network-statistics.json`
- **The Attack:**  
  In `inspect_networks.py`, the network parser processes PSS/E three-winding transformers by creating direct edges between the primary winding and each secondary/tertiary terminal:
  ```python
  terminals = [int(value) for value in record[:3] if int(value) != 0]
  transformer_edges = [(terminals[0], other) for other in terminals[1:]]
  all_edges.extend(transformer_edges)
  ```
  Furthermore, when status is 4 (winding 1 disconnected), it creates a direct branch between terminal 1 and terminal 2:
  ```python
  elif status in {2, 3, 4} and len(terminals) == 3:
    disconnected_winding = {2: 1, 3: 2, 4: 0}[status]
    live_terminals = [
        node
        for index, node in enumerate(terminals)
        if index != disconnected_winding
    ]
    active_edges.append(tuple(live_terminals))
```

  This implementation contains a fundamental electrical error:
  1. In power system engineering (and PSS/E standards), a three-winding transformer does not connect physical terminal buses directly together in a delta or direct-branch topology. It is mathematically represented as an **equivalent star (Y) circuit with an internal, fictitious neutral bus**. Three separate winding branches connect the three physical terminal buses to this internal star point.
  2. Disconnecting winding 1 (status 4) removes the branch connecting terminal 0 to the internal star point. It does **not** create a direct branch between terminal 1 and terminal 2; rather, power flow between windings 2 and 3 must still pass through their respective winding impedances to the neutral point.
  3. By omitting the internal star bus and inserting direct fictitious branches, the script distorts graph topology, alters cut-sets, and produces invalid connectivity metrics. The resulting component counts (2 components, largest 2,022 buses) in `network-statistics.json` are topologically corrupted.
- **Evidence URLs:**
  - PSS/E Model Documentation: Transformer Modeling and 3-Winding Star Equivalent: UNSURE
  - Grainger, J.J. & Stevenson, W.D., *Power System Analysis*, McGraw-Hill (Section: Three-Winding Transformers and Star Equivalents): UNSURE

---

### ADV5-8: ERA5 Bounding Box Truncation at 3° E Excludes Critical Offshore Wind Zones and Interconnectors

- **Severity:** Med
- **Location:** `spikes/data/era5-request-2024-01.json` (line 25); `spikes/data/REPORT.md` (§5 One weather year)
- **The Attack:**  
  In `era5-request-2024-01.json`, the bounding box for the weather retrieval is specified as:
  ```json
  "area": [61, -12, 48, 3]
  ```
  representing North 61°, West −12°, South 48°, East 3°.

  Setting the eastern boundary to 3° E severely truncates the spatial scope mandated by project decisions:
  1. **Cross-Border Interconnectors (D-11):** Decision D-11 explicitly includes GB interconnectors to Norway, Denmark, Belgium, and the Netherlands. Continental connection points and offshore cable corridors lie substantially east of 3° E:
     - **North Sea Link (NSL):** Connects Blyth (GB) to Kvilldal (Norway) at **6.2° E**.
     - **Viking Link:** Connects Bicker Fen (GB) to Revsing (Denmark) at **9.3° E**.
     - **BritNed:** Connects Isle of Grain (GB) to Maasvlakte (Netherlands) at **4.0° E**.
     - **Nemo Link:** Connects Richborough (GB) to Zeebrugge (Belgium) at **3.2° E**.
  2. **North Sea Offshore Wind Corridors:** Round 3 and Round 4 offshore wind leasing zones in the Dogger Bank and East Anglia development areas extend to approximately **2.8°–3.5° E**.

  By cutting off the request grid at 3° E, the simulator cannot simulate weather-dependent transfer limits, ambient cable ratings, or correlated wind generation profiles for continental interconnector boundaries and deep North Sea wind farms. The bounding box must extend to at least 10° E.
- **Evidence URLs:**
  - NESO Interconnector Asset Profiles: https://www.neso.energy/electricity-explained/how-electricity-works/electricity-interconnectors
  - 4C Offshore North Sea Wind & Interconnector Spatial Database: https://www.4coffshore.com/

---

### ADV5-9: Committing BMRS and Market Snapshots Without License-Mandated Redistribution Notices

- **Severity:** Low
- **Location:** `spikes/data/samples/bmunits.json`, `spikes/data/samples/boa-period.json`, `spikes/data/samples/bod-period.json`; `spikes/data/README.md`
- **The Attack:**  
  The spike commits raw data excerpts into `spikes/data/samples/`, including actual operational BM unit parameters, bid/offer matrices, and acceptances from Elexon BMRS.
  While the spike author included general attributions in `spikes/data/README.md`, Section 3 of the Elexon BMRS Open Data Licence explicitly dictates:
  > *"When You Use BMRS Data You must include the following attribution statement acknowledging the source: Contains BMRS data © Elexon Limited copyright and database right [year] and where possible provide a link to this licence."*
  
  In `spikes/data/samples/`, raw JSON records extracted from Elexon APIs are committed as standalone JSON files (`boa-period.json`, `bod-period.json`, `bmunits.json`) without any embedded copyright notice, metadata wrapper, or license reference in the JSON payloads themselves. If these sample files are distributed or parsed by downstream services, they detach from `README.md` and breach the attribution terms of the Elexon licence.
- **Evidence URLs:**
  - Elexon Licence to use BMRS open data: https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/

---

## Strategic Recommendations for PR #5 Resolution

1. **Re-baseline ETYS Appendix B as Open Under Ofgem DBP:** Reverse the "UNVERIFIED" blocker for Appendix B. Formally cite Ofgem Transmission Licence Condition C11 and Data Best Practice Guidance ("Presumed Open"), confirming that technical network parameters published pursuant to regulatory mandates are commercially usable under the NESO Open Data Licence v1.0.
2. **Expand Distribution Network Scope Across All 6 DNOs:** Update the data catalogue to include the open data portals of UKPN, SSEN, SPEN, NPg, and ENWL. Recognize that 100% of GB 132-kV networks have permissively licensed models under CC-BY 4.0 / OGL v3.0.
3. **Ingest I-SEM Day-Ahead Prices and Interconnector Flows from ENTSO-E:** Abandon web scraping of SEMOpx SPAs. Query the ENTSO-E Transparency Platform REST API (`documentType=A44`, `10Y1001A1001A59C`) under CC-BY 4.0 for official 2024 I-SEM DAM prices and interconnector flows.
4. **Align MVP Fallback with Decision D-55:** Reframe the MVP network architecture around **D-55 (DC network plus proxy constraints)**. Use NESO ETYS boundary transfer capabilities (B0–B17) for MVP dispatch rather than attempting an unvalidated 2,000-bus nodal model or an invalid NGED-only regional pilot.
5. **Expand ERA5 Bounding Box to 10° E:** Amend `era5-request-2024-01.json` to extend from 3° E to 10° E to encompass Viking Link, North Sea Link, BritNed, Nemo Link, and eastern North Sea offshore wind assets.
6. **Correct Network Script Transformer Logic:** Fix `inspect_networks.py` to model three-winding transformers as star-equivalent networks with fictitious internal neutral buses rather than creating direct branches between physical terminal buses.
