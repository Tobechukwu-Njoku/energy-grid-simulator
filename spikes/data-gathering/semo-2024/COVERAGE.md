# SEMO & SEMOpx 2024 Market Price Coverage Report

This spike inventories, downloads, and verifies all-island Ireland (I-SEM) wholesale electricity market prices for calendar year 2024 (1 January 2024 to 31 December 2024 inclusive). It resolves the Irish 2024 price data gap identified in PR #11 / STATUS.md (which previously contained only 2018–2020 lookback data).

All raw files are stored outside git under `/home/njotob/energy-data/raw/semo/2024/` in accordance with repository data provenance rules (D-56).

---

## 1. Pinned Raw Datasets & Provenance

| Dataset Name | Filename | Size (bytes) | SHA-256 Digest | Source URL | Licence Status |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **SEMO Balancing Market Imbalance Settlement Prices (2020–2024)** | `Imbalance_Price_YTD_2020-2024.zip` | 4,459,701 | `16d0c54d1cebca28babec80bdd06af23668e84ce471cb2615f4f134ac0bbd1ac` | `https://www.sem-o.com/documents/general-publications/Imbalance_Price_YTD_2020-2024.zip` | `research-only` (D-56) |
| **SEMOpx Ex-Ante Market Historical Prices and Volumes (2021–2026)** | `Lookback_Market_October2026.zip` | 39,165,432 | `086eaa3e21e4589c9ee4b6fb7b42742c3197d17e1977c3d63aa9bb44317e037d` | `https://www.semopx.com/sites/semopx/files/2026-11/Lookback_Market%20October2026.zip` | `research-only` (D-56) |

### Licence and Rights Classification
* **SEMO website terms & disclaimer:** `https://www.sem-o.com/disclaimer`
* **SEMOpx website terms & disclaimer:** `https://www.semopx.com/disclaimer`
* **Exact disclaimer text:**
  > "This website contains general information about The Single Electricity Market Operator [and day-ahead auction and intraday markets in the Single Electricity Market (SEM) as operated by SEMOpx]. The materials and information contained on and in this website are provided for general information purposes only and are subject to change from time to time without notice. You should not act or refrain from acting on the basis of any material contained on or in this website without seeking appropriate professional advice. All materials and information are provided 'AS IS' and without warranties express, implied or otherwise howsoever arising. EirGrid plc and SONI Limited shall not be liable for inaccuracies, errors, omissions or misleading information contained in this website."
* **Classification stance:** While the EirGrid Group Open Data Licence exists (`https://www.smartgriddashboard.com/all/open-data-license/`) and references SEMO/SEMOpx, its express applicability to these specific download files is not established. Following project decision D-56, both files are tagged `research-only` pending commercial rights clearance. Raw files must never be committed to git.

---

## 2. Market Time Unit (MTU) & Regulatory Status in 2024

### MTU by Market Segment in 2024

| Market Timeframe | Market Operator | MTU Granularity | Trading / Delivery Scope |
| :--- | :--- | :--- | :--- |
| **Day-Ahead Market (DAM)** | SEMOpx | **60 minutes (1 hour)** | 24 hourly contracts per day (00:00–24:00 UTC) |
| **Intraday Auction 1 (IDA1)** | SEMOpx | **30 minutes (0.5 hour)** | 48 half-hour contracts per day (00:00–24:00 UTC) |
| **Intraday Auction 2 (IDA2)** | SEMOpx | **30 minutes (0.5 hour)** | 24 half-hour contracts per day (11:00–23:00 UTC) |
| **Intraday Auction 3 (IDA3)** | SEMOpx | **30 minutes (0.5 hour)** | 12 half-hour contracts per day (17:00–23:00 UTC) |
| **Intraday Continuous (IDC)** | SEMOpx | **30 & 60 minutes** | Continuous XBID/SIDC trading |
| **Imbalance Settlement Period (ISP)** | SEMO | **30 minutes (0.5 hour)** | 48 settlement periods per day |
| **Imbalance Pricing Period** | SEMO | **5 minutes** | Dispatch & operational pricing intervals |

### Record of MTU and Regulatory Changes During 2024
* **EU Clean Energy Package (Regulation (EU) 2019/943):**
  * Article 8(4) mandated EU member states to adopt a **15-minute** Imbalance Settlement Period (ISP).
  * Article 8(2) required NEMOs to offer trading products at least as granular as the ISP in day-ahead and intraday timeframes.
* **SEM Regulatory Exemption (Derogation):**
  * The SEM Regulatory Authorities (SEM Committee: Commission for Regulation of Utilities [CRU] and the Northern Ireland Utility Regulator [UR]) granted SEM a formal derogation exempting the market from transitioning to a 15-minute ISP, retaining the **30-minute Imbalance Settlement Period** throughout 2024.
* **Day-Ahead Market MTU Transition (SPX_01_24):**
  * During 2024, SEMOpx and the SEM Committee consulted on and approved modification *SPX_01_24*, designing and testing a transition from 60-minute to **30-minute MTU** in the Day-Ahead Market to align DAM directly with the 30-minute ISP and European Single Day-Ahead Coupling (SDAC).
  * Member testing and trial rollbacks took place in Q4 2024; the switch was scheduled for go-live in early 2025.
* **Conclusion on MTU Change:** **No MTU change took effect during calendar year 2024.** All 2024 settlement and trading operations remained on their established MTUs (DAM: 60 min; IDA1/2/3: 30 min; ISP: 30 min; Operational pricing: 5 min).

---

## 3. Price Definitions and Labelling

Each price series is labelled and defined exactly according to SEM / SEMOpx market rules:

1. **`DAM Price (EUR)` / `DAM Price (GBP)`**:
   * **Market:** Day-Ahead Market (Ex-Ante Auction).
   * **Definition:** Uniform marginal clearing price established by the single daily pan-European Day-Ahead auction (closing 11:00 UTC D-1).
   * **Currencies:** EUR (€/MWh) for the Republic of Ireland and GBP (£/MWh) for Northern Ireland.
   * **MTU:** 60 minutes (hourly contracts).

2. **`IDA1 Price (EUR)` / `IDA1 Price (GBP)`**:
   * **Market:** Intraday Auction 1 (Ex-Ante Regional Auction).
   * **Definition:** Uniform marginal clearing price established by the first regional intraday auction (closing 17:30 UTC D-1), coupled with Great Britain and pan-European intraday auctions.
   * **Currencies:** EUR (€/MWh) and GBP (£/MWh).
   * **MTU:** 30 minutes (all 48 half-hours of the delivery day).

3. **`IDA2 Price (EUR)` / `IDA2 Price (GBP)`**:
   * **Market:** Intraday Auction 2 (Ex-Ante Regional Auction).
   * **Definition:** Uniform marginal clearing price established by the second regional intraday auction (closing 08:00 UTC on delivery day D).
   * **Currencies:** EUR (€/MWh) and GBP (£/MWh).
   * **MTU:** 30 minutes (24 half-hours covering 11:00 to 23:00 UTC delivery).

4. **`IDA3 Price (EUR)` / `IDA3 Price (GBP)`**:
   * **Market:** Intraday Auction 3 (Ex-Ante Regional Auction).
   * **Definition:** Uniform marginal clearing price established by the third regional intraday auction (closing 14:00 UTC on delivery day D).
   * **Currencies:** EUR (€/MWh) and GBP (£/MWh).
   * **MTU:** 30 minutes (12 half-hours covering 17:00 to 23:00 UTC delivery).

5. **`IDC Avg Price (EUR)`**:
   * **Market:** Intraday Continuous Market (SIDC / XBID).
   * **Definition:** Volume-weighted average trade price of executed continuous trades in EUR/MWh.

6. **`Imbalance Settlement Price (EUR)` (`FINAL_IMBALANCE_PRICE` where `SUMM_TYPE = 30MIN`)**:
   * **Market:** SEM Balancing Market (Imbalance Settlement).
   * **Definition:** The single financial settlement price applied to energy imbalances (deviations between metered generation/consumption and final ex-ante physical contract positions).
   * **Calculation:** Time-weighted average of the indicative imbalance prices calculated across the six 5-minute operational dispatch intervals within that 30-minute settlement period.
   * **Currency:** EUR (€/MWh).
   * **Price Flags:** `I` (Indicative / regular outturn), `B` (Backup / administrative price applied during operational system fallback).

7. **`5-Minute Operational Imbalance Price (EUR)` (`FINAL_IMBALANCE_PRICE` where `SUMM_TYPE = 5MIN`)**:
   * **Market:** SEM Balancing Market (Operational Dispatch Pricing).
   * **Definition:** Real-time dispatch interval imbalance price reflecting dynamic energy actions taken by the Transmission System Operators (EirGrid and SONI) to balance the grid.
   * **Currency:** EUR (€/MWh).

---

## 4. 2024 Coverage, Completeness, Gaps, and Duplicates

2024 is a leap year (366 days, 8,784 hours, 17,568 half-hours, 105,408 five-minute intervals).

### Coverage Table

| Dataset / Price Series | File / Sheet | Resolution (MTU) | Expected 2024 Rows | Actual 2024 Rows | Completeness | Missing Gaps | Duplicates | Null Prices |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **DAM Price (EUR & GBP)** | `Lookback_Market October2026.xlsx` (`auctions_to`) | 60 min | 8,784 | 8,784 | **100.00%** | 0 | 0 | 0 |
| **IDA1 Price (EUR & GBP)** | `Lookback_Market October2026.xlsx` (`auctions_to`) | 30 min | 17,568 | 17,568 | **100.00%** | 0 | 0 | 0 |
| **IDA2 Price (EUR & GBP)** | `Lookback_Market October2026.xlsx` (`auctions_to`) | 30 min | 8,784 | 8,784 | **100.00%** | 0 | 0 | 0 |
| **IDA3 Price (EUR & GBP)** | `Lookback_Market October2026.xlsx` (`auctions_to`) | 30 min | 4,392 | 4,392 | **100.00%** | 0 | 0 | 0 |
| **IDC Trades** | `Lookback_Market October2026.xlsx` (`idcs_to`) | Discrete / 30 min | N/A (traded) | 1,516 | **N/A** | 0 | 0 | 0 |
| **Imbalance Settlement Price (30MIN)** | `Imbalance_Price_YTD_2020- 2024.csv` | 30 min | 17,568 | 17,507 | **99.65%** | 61 | 0 | 0 |
| **Operational Imbalance Price (5MIN)** | `Imbalance_Price_YTD_2020- 2024.csv` | 5 min | 105,408 | 96,147 | **91.21%** | 9,261 | 0 | 0 |

### Gaps and Quality Analysis
1. **SEMOpx Auction Prices (DAM, IDA1, IDA2, IDA3):**
   * **Zero missing records:** Every single hour (8,784/8,784) of 2024 is present for DAM.
   * Every half-hour (17,568/17,568) is present for IDA1.
   * Every scheduled delivery interval is present for IDA2 (8,784) and IDA3 (4,392).
   * **Zero duplicate timestamps** and **zero null prices** across all auction series.
2. **SEMO Balancing Market 30-Minute Imbalance Settlement Prices:**
   * **17,507 records** present out of 17,568 expected.
   * **Zero duplicate timestamps** (`unique == count == 17,507`).
   * **Zero null prices:** Every single one of the 17,507 rows contains a valid decimal price.
   * **61 missing settlement periods (0.35% of year):**
     * March 2024: 11 intervals (13 March: 12:00, 12:30, 13:00, 14:00, 14:30, 15:00; 20 March: 20:00, 20:30, 21:30, 22:00; 27 March: 12:00)
     * April 2024: 5 intervals (10 April: 10:30, 11:00, 11:30; 24 April: 12:30, 13:00)
     * May 2024: 4 intervals (8 May: 13:00, 13:30; 29 May: 12:00, 12:30)
     * June 2024: 6 intervals (12 June: 12:00, 12:30; 19 June: 11:30, 12:00; 26 June: 12:00, 12:30)
     * July 2024: 11 intervals (3 July: 12:00, 12:30; 17 July: 11:30, 12:00, 12:30; 24 July: 12:00, 12:30, 13:00; 31 July: 12:00, 12:30, 13:00)
     * August 2024: 3 intervals (14 August: 12:00, 12:30, 13:00)
     * September 2024: 3 intervals (11 September: 12:00, 12:30, 13:00)
     * October 2024: 10 intervals (9 October: 12:00, 12:30; 16 October: 12:00, 12:30; 23 October: 11:30, 12:00, 12:30; 30 October: 12:00, 12:30, 13:00)
     * November 2024: 4 intervals (6 November: 12:00, 12:30; 20 November: 12:00, 12:30)
     * December 2024: 4 intervals (11 December: 12:00, 12:30; 18 December: 12:00; 31 December: 23:30)
     *(Notice the recurring pattern on Wednesdays during scheduled SEMO balancing market system maintenance windows).*
   * **Price flags:** 17,493 rows flagged `I` (Indicative / regular calculation); 14 rows flagged `B` (Backup / fallback price).
   * **Volume completeness:** 17,354 rows contain `NET_IMBALANCE_VOLUME`; 153 rows have unpopulated volumes.
3. **SEMO Balancing Market 5-Minute Operational Pricing Intervals:**
   * 96,147 records present (91.21% completeness; 9,261 unpopulated intervals).
   * Zero null prices.
   * Price flags: 95,169 flagged `I`; 978 flagged `Y`. The 978 records flagged `Y` correspond exactly to intervals where volume was unpopulated.

---

## 5. Irish Local Time (IST / GMT) versus UTC

### Timezone Representation
* **SEMOpx Market Data:** Timestamps are recorded in **UTC (ISO 8601 with explicit `Z` suffix)**, e.g. `2024-01-01T00:00:00Z`.
* **SEMO Balancing Market:** Timestamps are recorded in **UTC (`DD/MM/YYYY HH:MM`)**.
* **Irish Local Time:**
  * **Winter (GMT):** UTC+0 (1 January to 31 March 2024 01:00 UTC; 27 October 2024 01:00 UTC to 31 December 2024).
  * **Summer (Irish Standard Time - IST):** UTC+1 (31 March 2024 01:00 UTC to 27 October 2024 01:00 UTC).

### Clock Change Days Analysis
Because both source feeds report in continuous UTC, clock transition days behave cleanly without time distortion:

1. **Spring Clock Change (Sunday, 31 March 2024):**
   * **Local Time:** At 01:00 GMT, clocks sprang forward to 02:00 IST (23 clock hours in Ireland).
   * **SEMOpx DAM Auction Data:** Exactly **24 records** (2024-03-31T00:00:00Z through 2024-03-31T23:00:00Z). No hour was skipped in UTC.
   * **SEMOpx IDA1 Auction Data:** Exactly **48 records** (2024-03-31T00:00:00Z through 2024-03-31T23:30:00Z).
   * **SEMO 30MIN Imbalance Price:** Exactly **48 records** (`31/03/2024 00:00` through `31/03/2024 23:30`).

2. **Autumn Clock Change (Sunday, 27 October 2024):**
   * **Local Time:** At 02:00 IST, clocks fell back to 01:00 GMT (25 clock hours in Ireland, repeating the 01:00–02:00 hour).
   * **SEMOpx DAM Auction Data:** Exactly **24 records** (2024-10-27T00:00:00Z through 2024-10-27T23:00:00Z). No duplicate timestamps in UTC.
   * **SEMOpx IDA1 Auction Data:** Exactly **48 records** (2024-10-27T00:00:00Z through 2024-10-27T23:30:00Z).
   * **SEMO 30MIN Imbalance Price:** Exactly **48 records** (`27/10/2024 00:00` through `27/10/2024 23:30`). No duplicate timestamps.

*(Note: When converting to Irish local civil time for local demand matching, local clock changes require shifting timestamps by +1 hour during IST and expanding/collapsing the local 23-hour/25-hour transition days).*

---

## 6. 2024 Price Summary Statistics

All statistics calculated across the full 2024 calendar year (verified from source records):

| Price Metric | Market / Series | Currency | Unit | Count | Min | Max | Mean |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Day-Ahead Auction Price** | DAM | EUR | €/MWh | 8,784 | -5.02 | 499.99 | **108.91** |
| **Day-Ahead Auction Price** | DAM | GBP | £/MWh | 8,784 | -4.29 | 411.95 | **92.07** |
| **Intraday Auction 1 Price** | IDA1 | EUR | €/MWh | 17,568 | -30.05 | 536.91 | **107.99** |
| **Intraday Auction 1 Price** | IDA1 | GBP | £/MWh | 17,568 | -24.93 | 442.37 | **91.29** |
| **Intraday Auction 2 Price** | IDA2 | EUR | €/MWh | 8,784 | -17.62 | 563.16 | **118.87** |
| **Intraday Auction 2 Price** | IDA2 | GBP | £/MWh | 8,784 | -14.92 | 464.00 | **100.45** |
| **Intraday Auction 3 Price** | IDA3 | EUR | €/MWh | 4,392 | -17.00 | 510.00 | **130.76** |
| **Intraday Auction 3 Price** | IDA3 | GBP | £/MWh | 4,392 | -14.50 | 420.20 | **110.54** |
| **Imbalance Settlement Price** | Balancing Market (30MIN) | EUR | €/MWh | 17,507 | -144.09 | 874.25 | **110.17** |
| **Operational Imbalance Price** | Balancing Market (5MIN) | EUR | €/MWh | 96,147 | -1000.00 | 498.11 | **107.01** |
| **Net Imbalance Volume** | Balancing Market (30MIN) | N/A | MWh | 17,354 | -161.49 | 193.16 | **1.51** |
| **Net Imbalance Volume** | Balancing Market (5MIN) | N/A | MWh | 95,169 | -164.34 | 199.20 | **1.44** |

*(Validation note: The computed 2024 annual average DAM price of €108.91/MWh exactly corroborates SEMOpx's published December 2024 Monthly Market Report figure of €108.91/MWh).*
