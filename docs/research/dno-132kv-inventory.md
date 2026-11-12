# GB DNO 132 kV Network Data Inventory

This inventory documents where each Great Britain Distribution Network Operator (DNO) publishes 132 kV network data, covering all 14 licence areas across the 6 DNO operator groups. It supports Decision D-60 (GB MVP includes all 132 kV networks, including England & Wales 132 kV distribution owned by DNOs).

---

## Summary

* **Total GB DNO Licence Areas:** 14 licence areas across 6 operator groups.
* **Licence Areas with Usable 132 kV Electrical Data (Open / Commercial Licence):** **11 of 14 licence areas** publish complete 132 kV circuit parameters (resistance $R$, reactance $X$, susceptance $B$, and seasonal ratings) under open licenses permitting commercial use:
  * **UK Power Networks (3):** Eastern (EPN), London (LPN), South Eastern (SPN) — CC BY 4.0 (free portal registration required).
  * **National Grid Electricity Distribution (4):** East Midlands, West Midlands, South Wales, South West — NGED Open Data Licence (OGL v3.0 equivalent, direct download, no login).
  * **SSEN Distribution (1):** Southern Electric (SEPD) — CC BY 4.0 (direct download, no login).
  * **SP Energy Networks (1):** SP Manweb (SPM) — CC BY 4.0 (free portal registration required).
  * **Northern Powergrid (2):** Northeast, Yorkshire — Northern Powergrid Open Data Licence v1.0 (free portal registration required).
* **Licence Areas Without 132 kV Distribution Data (Scottish Transmission Boundary):** **2 of 14 licence areas**:
  * **SSEN Distribution (SHEPD - North of Scotland)** and **SP Energy Networks (SPD - Central & Southern Scotland)** do **not** publish 132 kV circuit data in their DNO LTDS. In Scotland, 132 kV is defined by statute (Electricity Act 1989) as transmission, owned and operated by Scottish Hydro Electric Transmission plc (SHET / SSEN Transmission) and SP Transmission plc (SPT), respectively. Their 132 kV circuits are reported at the transmission level in the NESO Electricity Ten Year Statement (ETYS) Appendix B, rather than in DNO distribution LTDS Table 1.
* **Licence Areas with Restricted / Non-Open Licensing (Data Gap):** **1 of 14 licence areas**:
  * **Electricity North West (ENWL):** Publishes 132 kV and 33 kV circuit data in LTDS Table 1 and CIM models, but restricts access behind a secure customer portal login under the *SP ENW Shared Licence*, which limits use to *"internal business use only"* and prohibits public commercial distribution or re-use without prior written consent.
* **Access Friction Breakdown:**
  * **Direct Unauthenticated Download (5 areas):** NGED (4 areas: East Midlands, West Midlands, South Wales, South West) via CKAN; SSEN Distribution (SEPD) via API/R2 storage.
  * **Free Portal Registration / Login Required (6 areas):** UKPN (3 areas), SPEN (SPM), Northern Powergrid (2 areas) host their data on OpenDataSoft platforms where catalog metadata is public, but raw table record downloads return HTTP 403 Forbidden without an authenticated user account.
  * **Gated / Restricted Customer Portal (1 area):** ENWL requires explicit application/login for secure area access.

---

## 14-Licence Area Inventory Table

| Operator & Licence Area | LTDS Dataset URL & Format | Latest Edition Date | 132 kV Circuits & Electrical Parameters ($R, X, B$, Ratings, Units) | Licence Name, Link & Exact Commercial Use Clause | Share-Alike? | Access Method | Notes |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **UK Power Networks**<br>Eastern Power Networks (EPN) | [UKPN LTDS Table 1 Circuit Data](https://ukpowernetworks.opendatasoft.com/explore/dataset/ukpn-ltds-table-1-circuit-data/)<br>Formats: CSV, XLSX, JSON, Parquet, API; CIM models in [LTDS CIM Packages](https://ukpowernetworks.opendatasoft.com/explore/dataset/long-term-development-statement-ltds-common-information-model-cim/) | May 2026 (dataset modified 2026-06-29) | **Yes.** Contains 132 kV circuits.<br>• Impedance: `positive_sequence_impedance_r_percent`, `positive_sequence_impedance_x_percent` in **% on 100 MVA base**.<br>• Susceptance: `susceptance_b_percent` in **% on 100 MVA base**.<br>• Ratings: `rating_amps_summer`, `rating_amps_winter` in **Amps (A)**.<br>• Length: `circuit_length` in **km**. | [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/)<br><br>*"Adapt — remix, transform, and build upon the material for any purpose, even commercially."* | No | Portal login (API returns 403 ForbiddenAccess without login) | Single dataset covers EPN, LPN, and SPN. Filter by field `dno` = `EPN`. |
| **UK Power Networks**<br>London Power Networks (LPN) | [UKPN LTDS Table 1 Circuit Data](https://ukpowernetworks.opendatasoft.com/explore/dataset/ukpn-ltds-table-1-circuit-data/)<br>Formats: CSV, XLSX, JSON, Parquet, API; CIM models in [LTDS CIM Packages](https://ukpowernetworks.opendatasoft.com/explore/dataset/long-term-development-statement-ltds-common-information-model-cim/) | May 2026 (dataset modified 2026-06-29) | **Yes.** Contains 132 kV circuits.<br>• Impedance: `positive_sequence_impedance_r_percent`, `positive_sequence_impedance_x_percent` in **% on 100 MVA base**.<br>• Susceptance: `susceptance_b_percent` in **% on 100 MVA base**.<br>• Ratings: `rating_amps_summer`, `rating_amps_winter` in **Amps (A)**.<br>• Length: `circuit_length` in **km**. | [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/)<br><br>*"Adapt — remix, transform, and build upon the material for any purpose, even commercially."* | No | Portal login (API returns 403 ForbiddenAccess without login) | Single dataset covers EPN, LPN, and SPN. Filter by field `dno` = `LPN`. Predominantly cable circuits. |
| **UK Power Networks**<br>South Eastern Power Networks (SPN) | [UKPN LTDS Table 1 Circuit Data](https://ukpowernetworks.opendatasoft.com/explore/dataset/ukpn-ltds-table-1-circuit-data/)<br>Formats: CSV, XLSX, JSON, Parquet, API; CIM models in [LTDS CIM Packages](https://ukpowernetworks.opendatasoft.com/explore/dataset/long-term-development-statement-ltds-common-information-model-cim/) | May 2026 (dataset modified 2026-06-29) | **Yes.** Contains 132 kV circuits.<br>• Impedance: `positive_sequence_impedance_r_percent`, `positive_sequence_impedance_x_percent` in **% on 100 MVA base**.<br>• Susceptance: `susceptance_b_percent` in **% on 100 MVA base**.<br>• Ratings: `rating_amps_summer`, `rating_amps_winter` in **Amps (A)**.<br>• Length: `circuit_length` in **km**. | [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/)<br><br>*"Adapt — remix, transform, and build upon the material for any purpose, even commercially."* | No | Portal login (API returns 403 ForbiddenAccess without login) | Single dataset covers EPN, LPN, and SPN. Filter by field `dno` = `SPN`. |
| **National Grid Electricity Distribution**<br>East Midlands | [NGED LTDS Tabular Model](https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model) ([Table 1 CSV Resource](https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/7892e22f-e489-49fa-a582-b1384006e4e2/download/ltds-table-1-circuit.csv))<br>CIM: [NGED LTDS CIM](https://connecteddata.nationalgrid.co.uk/dataset/ltds-common-information-model) (`emids.zip`)<br>Formats: CSV, CIM XML ZIP | Tabular: Dec 2025 (updated July 2026); CIM: May 2026 (modified 2026-05-28) | **Yes.** Contains 132 kV circuits.<br>• Impedance: `R1`, `X1` (and zero-seq `R0`, `X0`) in **% on 100 MVA base**.<br>• Susceptance: `B1` in **% on 100 MVA base**.<br>• Ratings: `Winter`, `Intermediate Cool`, `Intermediate Warm`, `Summer` in **Amps (A)**.<br>• Length: `Circuit Length (km)` in **km**. | [NGED Open Data Licence](https://commercial.nationalgrid.co.uk/open-data-licence) (based on OGL v3.0)<br><br>*"3.3 exploit the Information commercially and non-commercially for example, by combining it with other Information, or by including it in your own product or application."* | No | Direct download (unauthenticated HTTP 200) | Single CSV covers all 4 NGED areas. Filter by `Licence` = `EMIDS`. Formerly Western Power Distribution. |
| **National Grid Electricity Distribution**<br>West Midlands | [NGED LTDS Tabular Model](https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model) ([Table 1 CSV Resource](https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/7892e22f-e489-49fa-a582-b1384006e4e2/download/ltds-table-1-circuit.csv))<br>CIM: [NGED LTDS CIM](https://connecteddata.nationalgrid.co.uk/dataset/ltds-common-information-model) (`wmids.zip`)<br>Formats: CSV, CIM XML ZIP | Tabular: Dec 2025 (updated July 2026); CIM: May 2026 (modified 2026-05-28) | **Yes.** Contains 132 kV circuits.<br>• Impedance: `R1`, `X1` (and zero-seq `R0`, `X0`) in **% on 100 MVA base**.<br>• Susceptance: `B1` in **% on 100 MVA base**.<br>• Ratings: `Winter`, `Intermediate Cool`, `Intermediate Warm`, `Summer` in **Amps (A)**.<br>• Length: `Circuit Length (km)` in **km**. | [NGED Open Data Licence](https://commercial.nationalgrid.co.uk/open-data-licence) (based on OGL v3.0)<br><br>*"3.3 exploit the Information commercially and non-commercially for example, by combining it with other Information, or by including it in your own product or application."* | No | Direct download (unauthenticated HTTP 200) | Filter by `Licence` = `WMIDS`. |
| **National Grid Electricity Distribution**<br>South Wales | [NGED LTDS Tabular Model](https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model) ([Table 1 CSV Resource](https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/7892e22f-e489-49fa-a582-b1384006e4e2/download/ltds-table-1-circuit.csv))<br>CIM: [NGED LTDS CIM](https://connecteddata.nationalgrid.co.uk/dataset/ltds-common-information-model) (`swales.zip`)<br>Formats: CSV, CIM XML ZIP | Tabular: Dec 2025 (updated July 2026); CIM: May 2026 (modified 2026-05-28) | **Yes.** Contains 132 kV circuits.<br>• Impedance: `R1`, `X1` (and zero-seq `R0`, `X0`) in **% on 100 MVA base**.<br>• Susceptance: `B1` in **% on 100 MVA base**.<br>• Ratings: `Winter`, `Intermediate Cool`, `Intermediate Warm`, `Summer` in **Amps (A)**.<br>• Length: `Circuit Length (km)` in **km**. | [NGED Open Data Licence](https://commercial.nationalgrid.co.uk/open-data-licence) (based on OGL v3.0)<br><br>*"3.3 exploit the Information commercially and non-commercially for example, by combining it with other Information, or by including it in your own product or application."* | No | Direct download (unauthenticated HTTP 200) | Filter by `Licence` = `SWALES`. |
| **National Grid Electricity Distribution**<br>South West | [NGED LTDS Tabular Model](https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model) ([Table 1 CSV Resource](https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/7892e22f-e489-49fa-a582-b1384006e4e2/download/ltds-table-1-circuit.csv))<br>CIM: [NGED LTDS CIM](https://connecteddata.nationalgrid.co.uk/dataset/ltds-common-information-model) (`swest.zip`)<br>Formats: CSV, CIM XML ZIP | Tabular: Dec 2025 (updated July 2026); CIM: May 2026 (modified 2026-05-28) | **Yes.** Contains 132 kV circuits.<br>• Impedance: `R1`, `X1` (and zero-seq `R0`, `X0`) in **% on 100 MVA base**.<br>• Susceptance: `B1` in **% on 100 MVA base**.<br>• Ratings: `Winter`, `Intermediate Cool`, `Intermediate Warm`, `Summer` in **Amps (A)**.<br>• Length: `Circuit Length (km)` in **km**. | [NGED Open Data Licence](https://commercial.nationalgrid.co.uk/open-data-licence) (based on OGL v3.0)<br><br>*"3.3 exploit the Information commercially and non-commercially for example, by combining it with other Information, or by including it in your own product or application."* | No | Direct download (unauthenticated HTTP 200) | Filter by `Licence` = `SWEST`. |
| **SSEN Distribution**<br>Southern Electric Power Distribution (SEPD) | [SEPD LTDS Dataset Page](https://data.ssen.co.uk/@ssen-distribution/sepd_long_term_development_statement)<br>Download: [SEPD May 2026 Tables XLSX](https://data-api.ssen.co.uk/dataset/695078a0-bbd8-420b-8383-ee6bf7fc0045/resource/02c9431f-6126-4384-9f46-9079dbcd7057/download/southern-electric-power-distribution-long-term-development-statement-tables_may-2026.xlsx)<br>CIM: `ltds_sepd_2026-01_faultlevel_existing_v1.zip`, `ltds_sepd_2026-01_systemcapacity_existing_v1.zip`<br>Formats: XLSX, CIM XML ZIP, PDF | May 2026 (XLSX published May 2026) | **Yes.** Sheet `Table 1 - Circuit Data` contains 132 kV circuits (verified from downloaded workbook).<br>• Impedance: `R`, `X` in **% on 100 MVA base**.<br>• Susceptance: `B` in **% on 100 MVA base**.<br>• Ratings: `Winter`, `Spring-Autumn`, `Summer` in **MVA**.<br>• Length: `Circuit Length` in **km**. | [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/)<br><br>*"Adapt — remix, transform, and build upon the material for any purpose, even commercially."* | No | Direct download (unauthenticated HTTP 302 to R2 storage) | Covers central southern England. 132 kV circuits connect GSPs to BSPs (e.g. Amersham, Loudwater, Axminster, Botley Wood). |
| **SSEN Distribution**<br>Scottish Hydro Electric Power Distribution (SHEPD) | [SHEPD LTDS Dataset Page](https://data.ssen.co.uk/@ssen-distribution/shepd_long_term_development_statement)<br>Download: [SHEPD May 2026 Tables XLSX](https://data-api.ssen.co.uk/dataset/17d2aee2-5248-45d7-a4ce-d09bdfa77b3c/resource/2bd2c3f3-6692-4e0c-8a0e-5232be6ad682/download/scottish-hydro-electric-power-distribution-long-term-development-statement-tables_may-2026.xlsx)<br>Formats: XLSX, CIM XML ZIP, PDF | May 2026 (XLSX published May 2026) | **No 132 kV circuits.**<br>Verified by inspecting all strings in `Table 1-Circuit Data`: exactly zero occurrences of 132 kV circuits; covers 33 kV circuits only.<br>• Units for 33 kV circuits: $R, X, B$ in % on 100 MVA base; ratings in MVA. | [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/)<br><br>*"Adapt — remix, transform, and build upon the material for any purpose, even commercially."* | No | Direct download (unauthenticated HTTP 302 to R2 storage) | **Scottish transmission area:** 132 kV is transmission, owned by Scottish Hydro Electric Transmission plc (SHET / SSEN Transmission), not the DNO (SHEPD). SHET 132 kV circuits are in NESO ETYS Appendix B. |
| **SP Energy Networks**<br>SP Manweb (SPM) | [SPM LTDS Table 1 Dataset](https://spenergynetworks.opendatasoft.com/explore/dataset/spm-ltds-appendix-1-circuit-data-table-1/information/)<br>CIM: [SPM LTDS CIM](https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-cim-eq-profile/)<br>Landing: [SPEN LTDS Page](https://www.spenergynetworks.co.uk/pages/long_term_development_statement.aspx)<br>Formats: CSV, JSON, XLSX via portal export; CIM XML | November 2025 (dataset modified 2025-11-28) | **Yes.** Contains 132 kV circuits.<br>• Impedance: `positive_sequence_impedance_r_on_100_mva_base`, `positive_sequence_impedance_x_on_100_mva_base` in **% on 100 MVA Base**.<br>• Susceptance: `susceptance_b_on_100_mva_base` in **% on 100 MVA Base**.<br>• Ratings: `winter_rating_mva`, `spring_autumn_rating_mva`, `summer_rating_mva` in **MVA**.<br>• Length: `circuit_length_km` in **km**. | [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/deed.en)<br><br>*"Adapt — remix, transform, and build upon the material for any purpose, even commercially."* | No | Portal login (API returns 403 ForbiddenAccess without login) | Covers Merseyside, Cheshire, North Wales, North Shropshire (England & Wales). 132 kV is distribution owned by SPM. |
| **SP Energy Networks**<br>SP Distribution (SPD) | [SPD LTDS Table 1 Dataset](https://spenergynetworks.opendatasoft.com/explore/dataset/spd-ltds-appendix-1-circuit-data-table-1/information/)<br>CIM: [SPD LTDS CIM](https://spenergynetworks.opendatasoft.com/explore/assets/spd-ltds-cim-eq-profile/)<br>Landing: [SPEN LTDS Page](https://www.spenergynetworks.co.uk/pages/long_term_development_statement.aspx)<br>Formats: CSV, JSON, XLSX via portal export; CIM XML | November 2025 (dataset modified 2025-11-28) | **No 132 kV circuits.**<br>Table 1 covers 33 kV distribution circuits only.<br>• Impedance: `resistance_r_pu_on_100mva`, `reactance_x_pu_on_100mva` in **pu on 100MVA**.<br>• Susceptance: `susceptance_b_pu_on_100mva` in **pu on 100MVA**.<br>• Ratings: `winter_maximum_continuous_rating_mva`, etc., in **MVA**.<br>• Length: `length_km` in **km**. | [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/deed.en)<br><br>*"Adapt — remix, transform, and build upon the material for any purpose, even commercially."* | No | Portal login (API returns 403 ForbiddenAccess without login) | **Scottish transmission area:** Central & Southern Scotland. 132 kV is transmission, owned by SP Transmission plc (SPT), not the DNO (SPD). SPT 132 kV circuits are in NESO ETYS Appendix B. |
| **Northern Powergrid**<br>Northern Powergrid (Northeast) | [NPg LTDS Appendix 3 Circuit Data](https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-appendix-3/information/)<br>CIM: [NPg LTDS Stage 2 CIM Files](https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-cim-files/information/)<br>Landing: [NPg LTDS Documents Page](https://northernpowergrid.opendatasoft.com/pages/ltds-documents/)<br>Formats: CSV, JSON, XLSX via portal export; CIM XML | November 2025 (dataset modified 2025-11-29) | **Yes.** Covers EHV circuits (132 kV and 66 kV).<br>• Impedance: `positive_sequence_impedance_r_on_100_mva_base`, `positive_sequence_impedance_x_on_100_mva_base` in **% on 100 MVA Base**.<br>• Susceptance: `susceptance_b_on_100_mva_base` in **% on 100 MVA Base**.<br>• Ratings: `rating_information_mva` in **MVA** (continuous rating for cables, spring/autumn rating for overhead lines).<br>• Length: `circuit_length_km` in **km**. | [Northern Powergrid Open Data Licence v1.0](https://northernpowergrid.opendatasoft.com/p/opendatalicence/) (based on OGL v3.0)<br><br>*"3.3 Exploit the Information commercially and non-commercially for example, by combining it with other Information, or by including it in your own product or application."* | No | Portal login ("This dataset DOES require a login to view") | Single dataset covers Northeast and Yorkshire licence areas. Filter by field `license` = `Northeast`. |
| **Northern Powergrid**<br>Northern Powergrid (Yorkshire) | [NPg LTDS Appendix 3 Circuit Data](https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-appendix-3/information/)<br>CIM: [NPg LTDS Stage 2 CIM Files](https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-cim-files/information/)<br>Landing: [NPg LTDS Documents Page](https://northernpowergrid.opendatasoft.com/pages/ltds-documents/)<br>Formats: CSV, JSON, XLSX via portal export; CIM XML | November 2025 (dataset modified 2025-11-29) | **Yes.** Covers EHV circuits (132 kV and 66 kV).<br>• Impedance: `positive_sequence_impedance_r_on_100_mva_base`, `positive_sequence_impedance_x_on_100_mva_base` in **% on 100 MVA Base**.<br>• Susceptance: `susceptance_b_on_100_mva_base` in **% on 100 MVA Base**.<br>• Ratings: `rating_information_mva` in **MVA**.<br>• Length: `circuit_length_km` in **km**. | [Northern Powergrid Open Data Licence v1.0](https://northernpowergrid.opendatasoft.com/p/opendatalicence/) (based on OGL v3.0)<br><br>*"3.3 Exploit the Information commercially and non-commercially for example, by combining it with other Information, or by including it in your own product or application."* | No | Portal login ("This dataset DOES require a login to view") | Filter by field `license` = `Yorkshire`. |
| **Electricity North West**<br>Electricity North West (ENWL) | [ENWL LTDS Landing Page](https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/) & [Introduction to LTDS](https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/introduction-to-ltds/)<br>Secure Area Login: [ENWL Secure Area](https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/login-to-LTDS-secure-area/)<br>Formats: XLSX, CSV, CIM XML, PDF (in secure area) | November 2025 | **Yes (contains 132 kV and 33 kV circuits in Table 1; file access restricted).**<br>• Circuit parameters confirmed in public introduction: schematic diagrams of 33 kV and 132 kV systems, circuit data, transformer data, fault levels, and CIM steady-state models.<br>• Exact column units in file: `UNVERIFIED` (file locked behind portal secure area login; Ofgem SLC 25 standard specifies % on 100 MVA base and MVA/Amps). | [SP ENW Shared Licence](https://electricitynorthwest.huwise.com/p/sp-enw-shared-licence/)<br><br>*"Permitted Use: internal business use only (which shall not include the use of the Data, Manipulated Data or the Materials by, or for the benefit of, any person other than You)."*<br>and Clause 3.1(b): *"will not modify adapt publish disclose or otherwise distribute any Licensed Information without ENWL prior written consent."* | No (Restricted proprietary shared data licence) | Portal login / secure area registration request | **Commercial re-use gap:** Data is restricted to internal business use. Cannot be freely re-distributed or incorporated into an open simulator without ENWL prior written permission. |

---

## Operator Deep Dives and Verification Evidence

### 1. UK Power Networks (UKPN)
* **Licence Areas:** Eastern Power Networks (EPN), London Power Networks (LPN), South Eastern Power Networks (SPN).
* **Portal:** OpenDataSoft portal at `ukpowernetworks.opendatasoft.com`.
* **Table 1 Circuit Dataset:** `ukpn-ltds-table-1-circuit-data`.
  * URL: `https://ukpowernetworks.opendatasoft.com/explore/dataset/ukpn-ltds-table-1-circuit-data/`
  * API endpoint: `https://ukpowernetworks.opendatasoft.com/api/explore/v2.1/catalog/datasets/ukpn-ltds-table-1-circuit-data/`
* **Verified Schema & Fields:**
  * `dno`: Licence area identifier (`EPN`, `LPN`, `SPN`).
  * `operating_voltage`: Operating voltage in kV (includes `132`, `66`, `33`).
  * `positive_sequence_impedance_r_percent`: Positive sequence resistance in `% on 100 MVA base`.
  * `positive_sequence_impedance_x_percent`: Positive sequence reactance in `% on 100 MVA base`.
  * `susceptance_b_percent`: Susceptance in `% on 100 MVA base`.
  * `rating_amps_summer` / `rating_amps_winter`: Thermal ratings in **Amps (A)**.
  * `circuit_length`: Length in **km**.
* **CIM Grid Models:** Dataset `long-term-development-statement-ltds-common-information-model-cim`, published May 2026, modified 2026-06-29.
* **Licence & Terms:** CC BY 4.0 (`https://creativecommons.org/licenses/by/4.0/`).
  * Commercial use: *"Adapt — remix, transform, and build upon the material for any purpose, even commercially."*
  * Share-alike: No.
* **Access Method:** Requires free user account registration and login on the OpenDataSoft portal. Unauthenticated calls to `/records` return `403 ForbiddenAccess`.

---

### 2. National Grid Electricity Distribution (NGED)
* **Licence Areas:** East Midlands (EMIDS), West Midlands (WMIDS), South Wales (SWALES), South West (SWEST). Formerly Western Power Distribution (WPD).
* **Portal:** CKAN portal at `connecteddata.nationalgrid.co.uk`.
* **Datasets Verified:**
  1. `ltds-tabular-model`: `https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model`
     * Direct CSV Download: `https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/7892e22f-e489-49fa-a582-b1384006e4e2/download/ltds-table-1-circuit.csv`
     * Latest modification: 2026-07-28; publication edition: Dec 2025 with 2026 mid-year updates.
  2. `ltds-common-information-model`: `https://connecteddata.nationalgrid.co.uk/dataset/ltds-common-information-model`
     * Contains separate CIM ZIP packages: `emids.zip`, `wmids.zip`, `swales.zip`, `swest.zip`. Modified 2026-05-28.
* **Verified Schema & Fields (Inspected directly from downloaded CSV):**
  * Columns: `Licence`, `GSP Group`, `BSP Group`, `Node 1`, `Node 2`, `Circuit Status`, `Operating Voltage (kV)`, `Circuit Code`, `R1`, `X1`, `B1`, `R0`, `X0`, `Winter`, `Intermediate Cool`, `Intermediate Warm`, `Summer`, `Circuit Length (km)`, `Equipment`.
  * Contains 132 kV circuits across all 4 licence identifiers (`EMIDS`, `WMIDS`, `SWALES`, `SWEST`).
  * Impedances: `R1`, `X1`, `B1`, `R0`, `X0` are in **% on 100 MVA base** (governed by Ofgem SLC 25 LTDS specification).
  * Ratings: `Winter`, `Intermediate Cool`, `Intermediate Warm`, `Summer` in **Amps (A)**.
* **Licence & Terms:** NGED Open Data Licence (`https://commercial.nationalgrid.co.uk/open-data-licence`), dated Dec 2020, modelled on OGL v3.0.
  * Commercial use clause: Clause 3.3: *"exploit the Information commercially and non-commercially for example, by combining it with other Information, or by including it in your own product or application."*
  * Share-alike: No.
* **Access Method:** Direct download via public HTTP URL without login or API key.

---

### 3. SSEN Distribution
* **Licence Areas:** Southern Electric Power Distribution plc (SEPD), Scottish Hydro Electric Power Distribution plc (SHEPD).
* **Portal:** Next.js / CKAN portal at `data.ssen.co.uk`, backed by `data-api.ssen.co.uk` and Cloudflare R2 storage.
* **SEPD (Southern Electric):**
  * Dataset URL: `https://data.ssen.co.uk/@ssen-distribution/sepd_long_term_development_statement`
  * Direct XLSX Resource: `https://data-api.ssen.co.uk/dataset/695078a0-bbd8-420b-8383-ee6bf7fc0045/resource/02c9431f-6126-4384-9f46-9079dbcd7057/download/southern-electric-power-distribution-long-term-development-statement-tables_may-2026.xlsx`
  * CIM ZIP Resource: `ltds_sepd_2026-01_faultlevel_existing_v1.zip`, `ltds_sepd_2026-01_systemcapacity_existing_v1.zip`.
  * Verified Contents: Inspected XML and shared strings of the downloaded May 2026 workbook. Sheet `Table 1 - Circuit Data` explicitly contains 132 kV circuits (connecting Amersham, Loudwater, Axminster, Botley Wood, Bramley, etc.).
  * Verified Units:
    * Positive Sequence Impedance: `R`, `X` in **% on 100 MVA base**.
    * Susceptance: `B` in **% on 100 MVA base**.
    * Ratings: `Winter`, `Spring-Autumn`, `Summer` in **MVA**.
    * Length: `Circuit Length` in **km**.
* **SHEPD (North of Scotland):**
  * Dataset URL: `https://data.ssen.co.uk/@ssen-distribution/shepd_long_term_development_statement`
  * Direct XLSX Resource: `https://data-api.ssen.co.uk/dataset/17d2aee2-5248-45d7-a4ce-d09bdfa77b3c/resource/2bd2c3f3-6692-4e0c-8a0e-5232be6ad682/download/scottish-hydro-electric-power-distribution-long-term-development-statement-tables_may-2026.xlsx`
  * Verified Contents: Inspected all 1,091 shared strings in the May 2026 workbook. Exactly zero entries contain "132" or 132 kV circuits. Table 1 contains only 33 kV distribution circuits.
  * Governance Distinction: In Northern Scotland, 132 kV is transmission under the Electricity Act 1989, owned by Scottish Hydro Electric Transmission plc (SHET). DNO LTDS Table 1 excludes transmission assets.
* **Licence & Terms:** CC BY 4.0 (`https://creativecommons.org/licenses/by/4.0/`).
  * Commercial use: *"Adapt — remix, transform, and build upon the material for any purpose, even commercially."*
  * Share-alike: No.
* **Access Method:** Direct download without login.

---

### 4. SP Energy Networks (SPEN)
* **Licence Areas:** SP Manweb plc (SPM) in England & Wales; SP Distribution plc (SPD) in Central & Southern Scotland.
* **Portal:** OpenDataSoft portal at `spenergynetworks.opendatasoft.com` and main site at `spenergynetworks.co.uk`.
* **SP Manweb (SPM - England & Wales):**
  * Dataset: `spm-ltds-appendix-1-circuit-data-table-1`
  * Dataset URL: `https://spenergynetworks.opendatasoft.com/explore/dataset/spm-ltds-appendix-1-circuit-data-table-1/information/`
  * CIM asset URL: `https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-cim-eq-profile/`
  * Latest edition: November 2025 (modified 2025-11-28).
  * Verified Schema & Units:
    * `operating_voltage_kv`: 132 kV circuits included.
    * `positive_sequence_impedance_r_on_100_mva_base`: **% on 100 MVA Base**.
    * `positive_sequence_impedance_x_on_100_mva_base`: **% on 100 MVA Base**.
    * `susceptance_b_on_100_mva_base`: **% on 100 MVA Base**.
    * `winter_rating_mva`, `spring_autumn_rating_mva`, `summer_rating_mva`: **MVA**.
    * `circuit_length_km`: **km**.
* **SP Distribution (SPD - Central & Southern Scotland):**
  * Dataset: `spd-ltds-appendix-1-circuit-data-table-1`
  * Dataset URL: `https://spenergynetworks.opendatasoft.com/explore/dataset/spd-ltds-appendix-1-circuit-data-table-1/information/`
  * Latest edition: November 2025 (modified 2025-11-28).
  * Scope: Covers 33 kV distribution circuits only. In Southern Scotland, 132 kV is transmission owned and operated by SP Transmission plc (SPT), reported in NESO ETYS Appendix B.
  * Verified Units (33 kV): `resistance_r_pu_on_100mva`, `reactance_x_pu_on_100mva`, `susceptance_b_pu_on_100mva` in **pu on 100MVA**; ratings in **MVA**.
* **Licence & Terms:** CC BY 4.0 (`https://creativecommons.org/licenses/by/4.0/deed.en`).
  * Commercial use: *"Adapt — remix, transform, and build upon the material for any purpose, even commercially."*
  * Share-alike: No.
* **Access Method:** Portal registration/login required. API record endpoints return `403 ForbiddenAccess` without an authenticated session.

---

### 5. Northern Powergrid (NPg)
* **Licence Areas:** Northern Powergrid (Northeast) plc, Northern Powergrid (Yorkshire) plc.
* **Portal:** OpenDataSoft portal at `northernpowergrid.opendatasoft.com`.
* **Datasets Verified:**
  1. `ltds-appendix-3`: "LTDS Appendix 3 - Circuit Data"
     * URL: `https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-appendix-3/information/`
     * Latest modification: November 2025 (2025-11-29).
  2. `ltds-cim-files`: "LTDS Stage 2 CIM Files"
     * URL: `https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-cim-files/information/`
* **Verified Schema & Fields:**
  * Official metadata description: *"All R, X and B impedance data detailed in this table are given as a percentage on 100 MVA base. The rating of each section is defined as either the continuous rating for cables or spring/autumn rating for overhead lines."*
  * Fields:
    * `license`: Licence area (`Northeast` or `Yorkshire`).
    * `operating_voltage_kv`: Extra High Voltage (includes 132 kV and 66 kV).
    * `positive_sequence_impedance_r_on_100_mva_base`: **% on 100 MVA Base**.
    * `positive_sequence_impedance_x_on_100_mva_base`: **% on 100 MVA Base**.
    * `susceptance_b_on_100_mva_base`: **% on 100 MVA Base**.
    * `rating_information_mva`: **MVA**.
    * `circuit_length_km`: **km**.
* **Licence & Terms:** Northern Powergrid Open Data Licence v1.0 (`https://northernpowergrid.opendatasoft.com/p/opendatalicence/`), based on OGL v3.0.
  * Commercial use clause: Clause 3.3: *"Exploit the Information commercially and non-commercially for example, by combining it with other Information, or by including it in your own product or application."*
  * Share-alike: No.
* **Access Method:** Requires free user account registration/login ("This dataset DOES require a login to view").

---

### 6. Electricity North West (ENWL)
* **Licence Area:** Electricity North West Limited (North West England: Cumbria, Lancashire, Greater Manchester).
* **Portal & Landing Pages:**
  * Landing Page: `https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/`
  * Introduction Page: `https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/introduction-to-ltds/`
  * Secure Area Login: `https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/login-to-LTDS-secure-area/`
* **Verified Scope:**
  * Introduction confirms detailed information covers the distribution network from 132 kV down to primary substation busbars, including schematic diagrams of the 33 kV and 132 kV systems, circuit data (Table 1), transformer data (Table 2), and CIM steady-state power flow models.
  * Latest revision: November 2025.
  * Column units inside file: `UNVERIFIED` (file download is locked inside the secure area; standard Ofgem SLC 25 structure uses % on 100 MVA base and MVA/Amps).
* **Licence & Commercial Restrictions:**
  * Governed by the *SP ENW Shared Licence* (`https://electricitynorthwest.huwise.com/p/sp-enw-shared-licence/`).
  * Commercial restriction clause:
    * Clause 1.1: *"Permitted Use: internal business use only (which shall not include the use of the Data, Manipulated Data or the Materials by, or for the benefit of, any person other than You)."*
    * Clause 3.1(b): *"You will treat any Licensed Information made available to You under this Licence as private and confidential and will not modify adapt publish disclose or otherwise distribute any Licensed Information without ENWL prior written consent."*
  * Share-alike: No (restrictive proprietary licence).
* **Access Method:** Portal login / secure area registration request.
* **Impact on Grid Simulator:** Because ENWL shared licence forbids external sharing, redistribution, and non-internal use, ENWL 132 kV circuit parameters cannot be incorporated directly into an open-source public repository without bilateral written consent from ENWL.

---

## Sources Checked

All sources checked and verified online on 2026-11-12:

1. **Ofgem:**
   * Standard Licence Condition 25 (Long Term Development Statement) Form of Statement: `https://www.ofgem.gov.uk/sites/default/files/docs/2006/07/14681-12506_ltds.pdf`
2. **UK Power Networks (EPN, LPN, SPN):**
   * LTDS Table 1 Circuit Data: `https://ukpowernetworks.opendatasoft.com/explore/dataset/ukpn-ltds-table-1-circuit-data/`
   * OpenDataSoft API endpoint: `https://ukpowernetworks.opendatasoft.com/api/explore/v2.1/catalog/datasets/ukpn-ltds-table-1-circuit-data`
   * LTDS CIM Dataset: `https://ukpowernetworks.opendatasoft.com/explore/dataset/long-term-development-statement-ltds-common-information-model-cim/`
   * Creative Commons Attribution 4.0 International: `https://creativecommons.org/licenses/by/4.0/`
3. **National Grid Electricity Distribution (East Midlands, West Midlands, South Wales, South West):**
   * LTDS Tabular Model: `https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model`
   * Table 1 Circuit CSV Resource: `https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/7892e22f-e489-49fa-a582-b1384006e4e2/download/ltds-table-1-circuit.csv`
   * LTDS Common Information Model: `https://connecteddata.nationalgrid.co.uk/dataset/ltds-common-information-model`
   * NGED Open Data Licence: `https://commercial.nationalgrid.co.uk/open-data-licence`
4. **SSEN Distribution (SEPD, SHEPD):**
   * SEPD LTDS Dataset Page: `https://data.ssen.co.uk/@ssen-distribution/sepd_long_term_development_statement`
   * SEPD May 2026 Tables XLSX Resource: `https://data-api.ssen.co.uk/dataset/695078a0-bbd8-420b-8383-ee6bf7fc0045/resource/02c9431f-6126-4384-9f46-9079dbcd7057/download/southern-electric-power-distribution-long-term-development-statement-tables_may-2026.xlsx`
   * SHEPD LTDS Dataset Page: `https://data.ssen.co.uk/@ssen-distribution/shepd_long_term_development_statement`
   * SHEPD May 2026 Tables XLSX Resource: `https://data-api.ssen.co.uk/dataset/17d2aee2-5248-45d7-a4ce-d09bdfa77b3c/resource/2bd2c3f3-6692-4e0c-8a0e-5232be6ad682/download/scottish-hydro-electric-power-distribution-long-term-development-statement-tables_may-2026.xlsx`
5. **SP Energy Networks (SP Manweb, SP Distribution):**
   * SPEN LTDS Landing Page: `https://www.spenergynetworks.co.uk/pages/long_term_development_statement.aspx`
   * SPM LTDS Appendix 1 Circuit Data: `https://spenergynetworks.opendatasoft.com/explore/dataset/spm-ltds-appendix-1-circuit-data-table-1/information/`
   * SPD LTDS Appendix 1 Circuit Data: `https://spenergynetworks.opendatasoft.com/explore/dataset/spd-ltds-appendix-1-circuit-data-table-1/information/`
   * SPM CIM Asset Page: `https://spenergynetworks.opendatasoft.com/explore/assets/spm-ltds-cim-eq-profile/`
   * SPD CIM Asset Page: `https://spenergynetworks.opendatasoft.com/explore/assets/spd-ltds-cim-eq-profile/`
   * Creative Commons Attribution 4.0 International: `https://creativecommons.org/licenses/by/4.0/deed.en`
6. **Northern Powergrid (Northeast, Yorkshire):**
   * NPg LTDS Documents Landing Page: `https://northernpowergrid.opendatasoft.com/pages/ltds-documents/`
   * NPg LTDS Appendix 3 Circuit Data: `https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-appendix-3/information/`
   * NPg LTDS CIM Files: `https://northernpowergrid.opendatasoft.com/explore/dataset/ltds-cim-files/information/`
   * Northern Powergrid Open Data Licence v1.0: `https://northernpowergrid.opendatasoft.com/p/opendatalicence/`
7. **Electricity North West (ENWL):**
   * ENWL LTDS Landing Page: `https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/`
   * ENWL Introduction to LTDS: `https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/introduction-to-ltds/`
   * ENWL Secure Area Login: `https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/login-to-LTDS-secure-area/`
   * SP ENW Shared Licence: `https://electricitynorthwest.huwise.com/p/sp-enw-shared-licence/`
