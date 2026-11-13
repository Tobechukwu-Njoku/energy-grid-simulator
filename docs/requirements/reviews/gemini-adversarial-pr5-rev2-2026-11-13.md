# Adversarial Review (Round 2) of PR #5: Revised D-31 Data Spike at Head d214b73b

**Reviewer:** Gemini Flash 3.8 (Adversarial Review Sub-Agent)  
**Date:** 2026-11-13  
**Target:** PR #5 (`Tobechukwu-Njoku/energy-grid-simulator#5`), head commit `d214b73b204a5302d1a3a45341684a21e4c3e099`  
**Target Files:**
- [`spikes/data/REPORT.md`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/REPORT.md)
- [`spikes/data/sources.tsv`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/sources.tsv)
- [`spikes/data/fetch.sh`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/fetch.sh)
- [`spikes/data/verify_hashes.py`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/verify_hashes.py)
- [`spikes/data/inspect_tabular.py`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_tabular.py)
- [`spikes/data/inspect_networks.py`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_networks.py)
- [`spikes/data/README.md`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/README.md)
- [`spikes/data/era5-request-2024-01.json`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/era5-request-2024-01.json)
- [`spikes/data/samples/download-manifest.json`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/samples/download-manifest.json)
- [`spikes/data/samples/README.md`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/samples/README.md)
- [`docs/requirements/02-questionnaire.md`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/requirements/02-questionnaire.md)
- [`docs/requirements/05-risks.md`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/requirements/05-risks.md)
- [`docs/requirements/reviews/response-pr5-2026-11-13.md`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/requirements/reviews/response-pr5-2026-11-13.md)  
**Governing Documents:** [`AGENTS.md`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/AGENTS.md), [`docs/requirements/03-decisions-log.md`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/requirements/03-decisions-log.md) (specifically D-11, D-17, D-26, D-28, D-30, D-31, D-32, D-52, D-53, D-55, D-56, D-57, D-58, D-59, D-60, D-65), merged PRs #10 and #11, and open PR #12.

---

## 1. Executive Summary & Verdict

This adversarial review assesses the second revision of PR #5 at head commit `d214b73b204a5302d1a3a45341684a21e4c3e099`.

Revision 2 addresses several Round 1 findings: it incorporates decision D-56 (allowing research-only use of publicly downloadable datasets), introduces a provenance manifest schema, expands the ERA5 bounding box eastwards to 10° E to avoid North Sea interconnector truncation, adds finite-numeric validation to NGED circuit parameters, and reframes the one-off Irish network script as a physical-bus connectivity diagnostic rather than an electrical equivalent.

However, an exhaustive adversarial audit reveals **critical defects, incorrect licence classifications, non-reproducible scripts, and severe data-quality traps** that remain or were newly introduced:

1. **False Claim of Free Re-Use for ENTSO-E Day-Ahead Energy Prices (ADV5-R2-1):** [`REPORT.md:383-390`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/REPORT.md:383) and [`response-pr5-2026-11-13.md:52`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/requirements/reviews/response-pr5-2026-11-13.md:52) assert that the ENTSO-E 18 October 2023 free re-use list incorporates CC-BY 4.0 for Day-Ahead Energy Prices (TR 12.1.d) and Scheduled Commercial Exchanges (TR 12.1.f), and claim that `open-commercial` is appropriate for retrieved I-SEM day-ahead series. This claim is demonstrably false: direct inspection of the official PDF confirms that Article 12.1.d and 12.1.f are **completely absent** from the free re-use list. This directly contradicts the verified research catalogue in [`docs/research/data-sources-ie-weather.md:72`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/research/data-sources-ie-weather.md:72) and [`docs/research/data-sources-ie-weather.md:158`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/research/data-sources-ie-weather.md:158) (merged in PR #2), violates D-56 and D-57, and ignores CodeRabbit's explicit review finding on PR #5.
2. **Open Licences Classified as `research-only` (ADV5-R2-2):** In [`spikes/data/samples/download-manifest.json:48-55, 88-95`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/samples/download-manifest.json:48), the UK Open Government Licence v3.0 (`ogl.html`) and the Creative Commons Attribution 4.0 deed (`cc-by.html`) are formally classified with `"licence_status": "research-only"`, corrupting the repository's provenance metrics.
3. **Temporal Desynchronization Trap Between Demand and Generation (ADV5-R2-3):** In [`spikes/data/inspect_tabular.py:48`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_tabular.py:48), the script enforces `assert len(demand) == len(fuel) == 17568`. This creates a lethal trap for downstream developers: `generation-mix.csv` is published in UTC (exactly 48 half-hours every day, including 31 March and 27 October), whereas `demand-2024.csv` is published in GB Settlement Dates (Europe/London clock time, with 46 periods on 31 March and 50 periods on 27 October). While both total 17,568 rows over leap year 2024, their row-by-row alignment suffers a **1-hour (2-period) phase shift** for seven continuous months (April to October).
4. **Fragile and Mutating Inspection Scripts (ADV5-R2-4, ADV5-R2-5):** [`inspect_tabular.py:76`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_tabular.py:76) crashes with `FileNotFoundError` if `dukes-5.11.xlsx` is missing; [`inspect_tabular.py:87-101`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_tabular.py:87) overwrites `download-manifest.json` in the working tree and erases all research-only provenance entries during a cleared-only run; [`inspect_networks.py:35, 71`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_networks.py:35) crashes unconditionally if uncleared files are absent; and [`verify_hashes.py:12-15`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/verify_hashes.py:12) only prints a warning and never fails, rendering integrity verification completely toothless.
5. **Committed Raw Data Excerpts (ADV5-R2-6):** PR #5 tracks 11 CSV and JSON sample files in git under [`spikes/data/samples/`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/samples/), which are mutated by local script executions. This conflicts with the clean separation established by PR #11 and PR #12 (which place all raw data strictly outside git at `~/energy-data/raw/`) and creates repository hygiene risks under D-58.
6. **Re-opening Decided Scope in Questionnaire (ADV5-R2-7):** [`02-questionnaire.md:71-77`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/requirements/02-questionnaire.md:71) re-questions whether Northern Ireland 275 kV should be included, directly re-opening established owner decisions D-11 and D-32, and omits the mandatory `★` marker for recommended defaults.

**Verdict: REVISE BEFORE MERGE.** PR #5 must not be merged until these eight findings are resolved.

---

## 2. Findings Matrix

| ID | Severity | Category | Target Location | Summary |
|---|---|---|---|---|
| **ADV5-R2-1** | **CRITICAL** | Licensing / Factual Error | [`REPORT.md:383-390`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/REPORT.md:383); [`response-pr5:52`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/requirements/reviews/response-pr5-2026-11-13.md:52) | False claim that ENTSO-E Day-Ahead Energy Prices (TR 12.1.d) are covered by the CC-BY 4.0 Free Re-Use List; contradicts PR #2 and violates D-56 / D-57 |
| **ADV5-R2-2** | **HIGH** | Governance / Manifest Defect | [`download-manifest.json:48-55, 88-95`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/samples/download-manifest.json:48); [`inspect_tabular.py:98`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_tabular.py:98) | Open commercial licence legal deeds (OGL v3.0, CC-BY 4.0) classified as `"licence_status": "research-only"` in manifest |
| **ADV5-R2-3** | **HIGH** | Data Quality Trap / Time Sync | [`inspect_tabular.py:48`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_tabular.py:48); [`REPORT.md:301-310`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/REPORT.md:301) | Unchecked 1-hour temporal phase shift between UTC generation (48 periods/day) and BST settlement demand (46/50 transition days) enforced by length assertion |
| **ADV5-R2-4** | **HIGH** | Script Breakage / Data Mutation | [`inspect_tabular.py:76, 87-101`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_tabular.py:76) | Unchecked `openpyxl` crash on missing `dukes-5.11.xlsx`; script mutates git working tree and destroys research-only provenance in manifest during cleared runs |
| **ADV5-R2-5** | **HIGH** | Script Breakage / Verification | [`inspect_networks.py:35, 71`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_networks.py:35); [`verify_hashes.py:12-15`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/verify_hashes.py:12); [`fetch.sh:22`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/fetch.sh:22) | Network script crashes unconditionally in cleared-only mode; hash verification script is warning-only and never fails on checksum mismatch |
| **ADV5-R2-6** | **MED** | Repo Hygiene / D-56 / D-58 | [`spikes/data/samples/*`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/samples/); [`fetch.sh:10-20`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/fetch.sh:10) | Committing 11 raw/tabular sample files to git; storing downloads inside repo instead of external directory used by PRs #11 and #12 |
| **ADV5-R2-7** | **MED** | Decision Drift / Formatting | [`02-questionnaire.md:71-77, 209-216`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/requirements/02-questionnaire.md:71) | Q-C-11 re-opens settled geography decisions D-11 and D-32 (NI 275 kV); omits mandatory `★` recommended default marker |
| **ADV5-R2-8** | **MED** | Analysis Defect / Filtering | [`REPORT.md:214, 655`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/REPORT.md:214); [`inspect_tabular.py:61-66`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_tabular.py:61) | NGED 1,138 132-kV circuit count conflates in-service, new, and removed circuits, and lacks PR #10 four-area regional disaggregation |

---

## 3. Detailed Technical Findings

### ADV5-R2-1: False Claim of CC-BY 4.0 Free Re-Use for ENTSO-E Day-Ahead Energy Prices (TR 12.1.d)

- **Severity:** **CRITICAL**
- **Location:** [`spikes/data/REPORT.md:383-390`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/REPORT.md:383); [`docs/requirements/reviews/response-pr5-2026-11-13.md:52`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/requirements/reviews/response-pr5-2026-11-13.md:52)
- **The Attack:**  
  [`spikes/data/REPORT.md:383-386`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/REPORT.md:383) asserts:
  > *"That list incorporates CC-BY-4.0: listed data may be copied, redistributed and adapted for any purpose with source credit, licence reference and changes indicated. It includes energy prices (TR 12.1.d), scheduled commercial exchanges (TR 12.1.f) and physical flows (TR 12.1.g)."*
  
  And line 390 claims:
  > *"I-SEM energy prices have no named exception in this list; `open-commercial` is appropriate when the retrieved series matches that scope."*

  This claim is **factually false** and legally dangerous.
  1. **Direct Audit of Primary Evidence:** Decompression and parsing of the official ENTSO-E attachment [`40921869379729`](https://transparencyplatform.zendesk.com/hc/article_attachments/40921869379729) (*"List of Data Available for Free Re-Use"*, dated 18 October 2023) proves that the document contains 35 enumerated data items.
     - Items 1–4 cover Load Forecasts (Articles 6.1.a–e).
     - Items 5–7 cover Outages and Unavailability.
     - Items 8–11 cover Cross-Zonal Capacity (Articles 11.1.a, 11.1.b).
     - Items 12–17 cover Explicit Allocations (Articles 12.1.a, 12.1.b, 12.1.c, 12.1.e).
     - Item 18 covers Physical Flows (Article 12.1.g).
     - Items 19–22 cover Redispatching and Countertrading (Articles 13.1.a, 13.1.b).
     - Items 23–35 cover Balancing Reserves, Activated Balancing Energy, and Imbalance Prices (Articles 17.1.b–h).
     - **Article 12.1.d (Day-Ahead Energy Prices) is completely absent.**
     - **Article 12.1.f (Scheduled Commercial Exchanges) is completely absent.**
  2. **Direct Contradiction with Merged Research:** The verified repository research in [`docs/research/data-sources-ie-weather.md:72`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/research/data-sources-ie-weather.md:72) (merged in commit `736d9ed`) explicitly warns:
     > *"Day-ahead prices under article 12.1.d are absent from the free-reuse list; distinguish them from listed imbalance/balancing prices."*  
     And line 158:
     > *"Confirmed that day-ahead prices are not on the list without inventing a blanket commercial-use prohibition for every owner."*
  3. **Violation of Decisions D-56 & D-57:** Under D-56, non-cleared data may be ingested for research but **must be tagged `research-only`**. Under D-57, price data must be labelled with what it actually is. Labeling I-SEM day-ahead prices retrieved from ENTSO-E as `open-commercial` based on a non-existent free re-use listing breaches both decisions.
  4. **Ignored CodeRabbit Finding:** CodeRabbit specifically flagged this exact error in its PR #5 review:
     > *"Revise the REPORT discussion of the 2023 list so it does not treat energy prices as commercially cleared... Update the 'I-SEM prices in scope' register entry to classify energy prices and scheduled exchanges as research-only pending confirmation of scope under D-56."*  
     PR #5 rev 2 failed to apply this fix.
- **Remediation:**  
  1. Revise [`spikes/data/REPORT.md:383-391`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/REPORT.md:383) to state that Article 12.1.d (Day-ahead Energy Prices) and 12.1.f (Scheduled Commercial Exchanges) are excluded from the 18 October 2023 free re-use list.
  2. Classify I-SEM Day-Ahead Prices and scheduled interconnector exchanges as `research-only` in all registers and manifest entries, tracking commercial clearance under `PC-5`.
  3. Update [`docs/requirements/reviews/response-pr5-2026-11-13.md:52`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/requirements/reviews/response-pr5-2026-11-13.md:52) to remove the claim that energy prices are on the CC-BY list.

---

### ADV5-R2-2: Open Commercial Licence Legal Deeds Classified as `research-only` in Manifest

- **Severity:** **HIGH**
- **Location:** [`spikes/data/samples/download-manifest.json:48-55, 88-95`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/samples/download-manifest.json:48); [`spikes/data/inspect_tabular.py:98`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_tabular.py:98)
- **The Attack:**  
  In [`spikes/data/samples/download-manifest.json`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/samples/download-manifest.json), the legal instruments defining open commercial use are stamped `research-only`:
  ```json
  {
    "file": "ogl.html",
    "source_url": "https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/",
    "licence_status": "research-only",
    "source_role": "metadata"
  },
  {
    "file": "cc-by.html",
    "source_url": "https://creativecommons.org/licenses/by/4.0/",
    "licence_status": "research-only",
    "source_role": "metadata"
  }
  ```
  The logic causing this is in [`spikes/data/inspect_tabular.py:98`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_tabular.py:98):
  ```python
  "licence_status": "open-commercial" if clearance == "open-commercial" else "research-only"
  ```
  Because [`sources.tsv:7, 12`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/sources.tsv:7) lists `clearance` as `metadata` for licence files, `inspect_tabular.py` defaults their `licence_status` to `"research-only"`.

  [`REPORT.md:76-78`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/REPORT.md:76) rationalizes this as:
  > *"Legal and discovery metadata is conservatively research-only, not silently relicensed."*

  This is a serious category error. A public licence deed (such as OGL v3.0 or Creative Commons Attribution 4.0) is not proprietary data requiring non-commercial research exemption under D-56. Tagging OGL v3.0 as `research-only` pollutes automated audit pipelines, falsely indicating that the repository’s core legal deeds carry restricted research-only rights.
- **Remediation:**  
  Update `inspect_tabular.py` and `download-manifest.json` so that `metadata` sources carry `licence_status: "public-metadata"` or inherit the clearance of the licence they describe, rather than collapsing all metadata into `research-only`.

---

### ADV5-R2-3: Temporal Desynchronization Trap Between UTC Generation and BST Settlement Demand

- **Severity:** **HIGH**
- **Location:** [`spikes/data/inspect_tabular.py:48`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_tabular.py:48); [`spikes/data/REPORT.md:301-310`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/REPORT.md:301)
- **The Attack:**  
  In [`spikes/data/inspect_tabular.py:48`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_tabular.py:48), the tabular audit script executes:
  ```python
  assert len(demand) == len(fuel) == 17568
  ```
  This creates a catastrophic temporal alignment trap.
  1. **Empirical Fact:**
     - In [`demand-2024.csv`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/samples/demand-2024.csv), rows represent GB Settlement Dates and Settlement Periods (clock time):
       - 31 March 2024 (short day, British Summer Time starts) has **46 settlement periods**.
       - 27 October 2024 (long day, British Summer Time ends) has **50 settlement periods**.
       - Total: $364 \times 48 + 46 + 50 = 17,568$ rows.
     - In `generation-mix.csv` (`df_fuel_ckan.csv`), timestamps are published in pure **UTC**:
       - 31 March 2024 has **48 timestamps** (`2024-03-31T00:00:00` to `2024-03-31T23:30:00`).
       - 27 October 2024 has **48 timestamps** (`2024-10-27T00:00:00` to `2024-10-27T23:30:00`).
       - Every day of 2024 has exactly 48 timestamps. Total: $366 \times 48 = 17,568$ rows.
  2. **The Failure Mode:**
     Because both datasets happen to have exactly 17,568 rows in a leap year, the assert passes. But if any downstream engineer joins the two series row-by-row or by index offset:
     - On 31 March 2024, `demand` advances to 1 April after 46 rows (at 23:00 GMT / 00:00 BST).
     - `generation-mix` continues for 2 more rows to complete the 48-period UTC day.
     - **Result:** Starting from 31 March 2024 at 23:00 UTC through 27 October 2024, every demand observation is paired with generation data from **1 hour (2 settlement periods) earlier**!
     - The phase shift persists for nearly seven full months of the simulation year.
  3. While [`REPORT.md:301-310`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/REPORT.md:301) mentions that local settlement dates are not 48-row UTC days, it **fails to state** that `generation-mix.csv` is published in 48-row UTC days, and `inspect_tabular.py` reinforces the false impression of row equivalence by asserting `assert len(demand) == len(fuel)`.
- **Remediation:**  
  1. Add an explicit test in `inspect_tabular.py` demonstrating timestamp-to-settlement-period alignment and rejecting row-index joins.
  2. Explicitly document in [`spikes/data/REPORT.md:401`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/REPORT.md:401) that `df_fuel_ckan.csv` has 48 rows on clock-change days while `demanddata_2024.csv` has 46/50, and specify the mandatory conversion function from Europe/London settlement periods to UTC.

---

### ADV5-R2-4: `inspect_tabular.py` Crashes on Missing Files, Overwrites Git Samples, and Wipes Provenance

- **Severity:** **HIGH**
- **Location:** [`spikes/data/inspect_tabular.py:76, 87-101`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_tabular.py:76); [`spikes/data/README.md:19-20`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/README.md:19)
- **The Attack:**  
  [`response-pr5-2026-11-13.md:27`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/requirements/reviews/response-pr5-2026-11-13.md:27) claims:
  > *"5. Cleared-only reproduction: inspect_tabular.py skips optional missing MID/XML and missing manifest sources; isolated cleared-only execution passes without ETYS/Irish restricted datasets."*

  This claim does not withstand adversarial execution:
  1. **Unconditional Workbook Load Crash:**  
     Line 76:
     ```python
     book = openpyxl.load_workbook(RAW / "dukes-5.11.xlsx", read_only=True, data_only=True)
     ```
     Unlike `repd.csv` (line 53) or `bmunits.json` (line 68), `dukes-5.11.xlsx` **has no existence check**. If `dukes-5.11.xlsx` is missing or fails to download, the script crashes immediately with `FileNotFoundError`.
  2. **Destructive Manifest Overwrite:**  
     Lines 87–101:
     ```python
     manifest = []
     with (ROOT / "sources.tsv").open() as stream:
         for source in csv.reader(stream, delimiter="\t"):
             ...
             path = RAW / name
             if not path.exists():
                 continue
             manifest.append(...)
     (SAMPLES / "download-manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
     ```
     If an operator runs `sh spikes/data/fetch.sh` (which skips research-only data by default per [`fetch.sh:13-16`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/fetch.sh:13)) and then runs `inspect_tabular.py` as instructed in [`README.md:20`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/README.md:20):
     - `path.exists()` is false for `etys-2025-appendix-b.xlsx`, `tytfs-2024.zip`, `tytfs-2024.pdf`, etc.
     - `inspect_tabular.py` writes a new `download-manifest.json` directly to the working directory, **permanently deleting the research-only provenance records and hashes** from git tracking!
  3. **In-Tree Mutation:**  
     Running `inspect_tabular.py` unconditionally rewrites git-tracked sample files (`demand-2024.csv`, `generation-2024.csv`, `nged-circuits.csv`, `tabular-statistics.json`, `download-manifest.json`). Verification scripts must never mutate the git repository as a side effect of inspection.
- **Remediation:**  
  1. Wrap line 76 in `if (RAW / "dukes-5.11.xlsx").exists(): ... else: result["dukes_5_11"] = {"status": "not downloaded"}`.
  2. Stop overwriting `download-manifest.json` during partial reproduction runs, or write regenerated manifest files to an untracked output directory (e.g. `raw/` or `output/`).

---

### ADV5-R2-5: `inspect_networks.py` Crashes in Cleared-Only Mode; Hash Verification is Toothless

- **Severity:** **HIGH**
- **Location:** [`spikes/data/inspect_networks.py:35, 71`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_networks.py:35); [`spikes/data/verify_hashes.py:12-15`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/verify_hashes.py:12); [`spikes/data/fetch.sh:22`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/fetch.sh:22)
- **The Attack:**  
  1. **Unconditional Network Script Crash:**  
     [`inspect_networks.py:35`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_networks.py:35) calls `openpyxl.load_workbook(RAW / "etys-2025-appendix-b.xlsx")` and line 71 calls `zipfile.ZipFile(RAW / "tytfs-2024.zip")`.
     Neither call checks for file existence. If an operator runs `fetch.sh` without flags and runs the network inspection, Python crashes with unhandled exceptions.
  2. **Toothless Integrity Verification:**  
     [`verify_hashes.py`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/verify_hashes.py):
     ```python
     if digest != entry["sha256"]:
         print(f"WARNING source drift: {entry['file']}: expected {entry['sha256']}, got {digest}")
     else:
         print(f"SHA-256 OK: {entry['file']}")
     ```
     The script never raises an error or calls `sys.exit(1)`.
     In [`fetch.sh:22`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/fetch.sh:22):
     ```sh
     python3 verify_hashes.py
     ```
     Because `verify_hashes.py` unconditionally exits 0, `set -eu` in `fetch.sh` has zero effect. Corrupted, truncated, or MITM-tampered downloads pass as successful. A verification script that cannot fail is not a verification gate.
- **Remediation:**  
  1. Add existence checks in `inspect_networks.py` and print a clean error/skip message when research inputs are absent.
  2. Update `verify_hashes.py` to count mismatches and exit with code 1 if any existing downloaded file fails its SHA-256 pin, with an explicit `--allow-drift` flag if live re-pinning is intended.

---

### ADV5-R2-6: Committing Raw Tabular Data into Git and Storing Downloads In-Tree

- **Severity:** **MED**
- **Location:** [`spikes/data/samples/*`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/samples/); [`spikes/data/fetch.sh:10-20`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/fetch.sh:10)
- **The Attack:**  
  1. **Committed Raw Samples in Git:**  
     Branch `d214b73b` commits 11 CSV and JSON data excerpts from live platforms into [`spikes/data/samples/`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/samples/): `demand-2024.csv`, `generation-2024.csv`, `tec-register.csv`, `nged-circuits.csv`, `nged-nodes.csv`, `nged-transformers.csv`, `bmunits.json`, `boa-period.json`, `bod-period.json`, `demand-day-correct.json`, `fuel-day.json`.
     Even though these are small excerpts of open-commercial sources, committing real tabular market data into git creates repository pollution and conflicts with:
     - **Decision D-58:** Precondition before opening the repository: *"a scan confirming no restricted/raw data or secrets anywhere in history or branches"*.
     - **CodeRabbit Path Filters:** CodeRabbit automatically ignored these samples in PR #5 (`spikes/data/samples/*.csv is excluded by !**/*.csv`).
  2. **In-Tree Download Architecture:**  
     [`fetch.sh:10`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/fetch.sh:10) downloads 135 MB of raw data into `spikes/data/raw/` inside the git working copy.
     In contrast, subsequent merged gathering PRs #11 and #12 strictly placed raw data outside git in `~/energy-data/raw/` (e.g. [`spikes/data-gathering/network-ie/fetch.py`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data-gathering/network-ie/fetch.py) and [`spikes/data-gathering/elexon/fetch.sh`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data-gathering/elexon/fetch.sh)), per **Decision D-65** (the container is for coding, not data storage).
- **Remediation:**  
  1. Standardize `fetch.sh` on `DATA_DIR="${DATA_DIR:-$HOME/energy-data/raw/spike}"` outside the repository tree, matching PR #11 and PR #12.
  2. Replace committed tabular data samples with synthetic test fixtures or schema assertions.

---

### ADV5-R2-7: Q-C-11 Re-opens Settled Decision D-11 / D-32 (NI 275 kV) and Omits `★` Marker

- **Severity:** **MED**
- **Location:** [`docs/requirements/02-questionnaire.md:71-77, 209-216`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/requirements/02-questionnaire.md:71)
- **The Attack:**  
  1. **Re-questioning Settled Decisions:**  
     [`02-questionnaire.md:71-74`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/requirements/02-questionnaire.md:71) states:
     ```markdown
     - [ ] **Q-C-11 Network snapshot date and connecting voltages (spike PR #5):**
       Which named operational date defines the present-day model, separately from
       the 2024 backtest? Include NI 275 kV and required connecting-voltage equivalents?
     ```
     Asking whether to include Northern Ireland 275 kV contradicts already established decisions:
     - **D-11 (Round 2, 2026-11-12):** *"Geography: GB + Northern Ireland + Republic of Ireland (all-island SEM)..."*
     - **D-32 (Round C, 2026-11-12):** *"MVP network: full transmission (all 400/275/132 kV nodes in GB; 400/220/110 kV in Ireland)..."*
     In Northern Ireland, the transmission backbone is 275 kV and 110 kV. D-11 and D-32 already settled this; asking whether to include NI 275 kV generates decision churn and violates AGENTS.md rule 1 (*"Source of truth: the owner's answers in 03-decisions-log.md. Never invent a decision"*).
  2. **Formatting Inconsistency Flagged by CodeRabbit:**  
     Line 75 uses `**Recommended default:**` without the documented `★` marker defined on line 5 of `02-questionnaire.md` (*"Each question has a stable ID, why it matters, options, and a recommended default (★)"*). Lines 79, 83, and 211 share this formatting omission.
- **Remediation:**  
  1. Remove the query about NI 275 kV from Q-C-11, citing D-11 and D-32 as already deciding NI 275 kV transmission inclusion. Focus Q-C-11 solely on the named topology baseline date.
  2. Add the `★` marker to the recommended defaults in Q-C-11, Q-C-12, and Q-M-4.

---

### ADV5-R2-8: NGED 132-kV Equipment Filtering Conflates In-Service, Future, and Decommissioned Assets

- **Severity:** **MED**
- **Location:** [`spikes/data/REPORT.md:214, 655`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/REPORT.md:214); [`spikes/data/inspect_tabular.py:61-66`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_tabular.py:61)
- **The Attack:**  
  1. **Status Conflation in 1,138 Circuit Count:**  
     In [`inspect_tabular.py:61-66`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/inspect_tabular.py:61):
     ```python
     if name == "nged-circuits.csv":
         target = [row for row in rows if row["Operating Voltage (kV)"] == "132"]
         csv_sample(name, [row for row in target if row["Equipment"] == "Circuit"])
         result[name]["132kv_rows_including_changes"] = len(target)
         result[name]["132kv_equipment_counts"] = dict(collections.Counter(row["Equipment"] for row in target))
         result[name]["132kv_numeric_r_x_b_and_winter_summer_ratings"] = sum(all(finite_number(row.get(key)) for key in ["R1", "X1", "B1", "Winter", "Summer"]) for row in target)
     ```
     `target` filters only on `Operating Voltage (kV) == "132"`.
     In [`spikes/data/samples/tabular-statistics.json:52-57`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/samples/tabular-statistics.json:52), `status_counts` for `nged-circuits.csv` shows:
     `{"Unchanged": 17596, "Updated": 202, "New": 3281, "Removed": 1437}`.
     The 1,138 circuits counted by line 66 include circuits with status `New` (schemes not yet built) and `Removed` (circuits already decommissioned)!
     Presenting 1,138 as the number of verified operational 132-kV circuits in NGED is incorrect.
  2. **Lack of Geographic Breakdown:**  
     [`docs/research/dno-132kv-inventory.md:32-35`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/research/dno-132kv-inventory.md:32) (merged PR #10) provides the exact breakdown across all four NGED licence areas:
     - East Midlands: 1,311 132-kV rows
     - West Midlands: 1,411 132-kV rows
     - South Wales: 857 132-kV rows
     - South West: 604 132-kV rows (Total Table 1 132-kV rows = 4,183).
     [`REPORT.md:214`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/REPORT.md:214) provides only an aggregated 1,138 figure and obscures the distinct licence area boundaries required for D-60.
- **Remediation:**  
  Filter `nged-circuits.csv` by `Circuit Status == "Unchanged"` (or explicitly separate in-service from planned/retired assets) in `inspect_tabular.py`, and cite the area-by-area breakdown established in PR #10.

---

## 4. Compliance and Process Verification

1. **AI Attribution Prohibition (D-52):**  
   - Commit messages contain no `Co-Authored-By` trailers or AI watermarking.
   - Files contain no `Generated with` footers, model attribution badges, or 🤖 emojis.
   - Model names appear strictly in review process reports, conforming with D-52.
2. **Document Protection:**  
   - [`docs/requirements/STATUS.md`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/requirements/STATUS.md) is untouched.
   - [`docs/requirements/03-decisions-log.md`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/requirements/03-decisions-log.md) is untouched.
   - Target files of PR #5 are untouched.

---

## 5. Required Actions for PR #5 Author

To achieve merge readiness, the PR author must:
1. **Fix ENTSO-E Licence Claims:** Strike the claim that Day-Ahead Energy Prices (TR 12.1.d) are CC-BY 4.0 free re-use data in [`REPORT.md:383-390`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/spikes/data/REPORT.md:383) and [`response-pr5-2026-11-13.md:52`](/home/njotob/.t3/projects/Dev - GitHub - Energy/t3-088e279a-319c327894ac6700/docs/requirements/reviews/response-pr5-2026-11-13.md:52). Reclassify all I-SEM day-ahead series as `research-only` under D-56, tracked under `PC-5`.
2. **Correct Manifest Licence Tags:** Fix `inspect_tabular.py:98` and `download-manifest.json` so that open licence deeds (`ogl.html`, `cc-by.html`) are not stamped `research-only`.
3. **Resolve UTC / Settlement Period Desynchronization:** Expose the temporal difference between UTC 48-period generation and BST 46/50-period demand in `inspect_tabular.py` and `REPORT.md:401`, eliminating the misleading length assertion.
4. **Make Scripts Resilient and Non-Destructive:** Add missing file checks for `dukes-5.11.xlsx` and uncleared network files; prevent `inspect_tabular.py` from mutating the git working tree or erasing manifest entries.
5. **Make Hash Verification Enforceable:** Update `verify_hashes.py` to exit with non-zero status on checksum failure.
6. **Clean Up Questionnaire:** Update Q-C-11 to respect D-11 / D-32 (NI 275 kV), and format all recommended defaults with the standard `★` marker.
