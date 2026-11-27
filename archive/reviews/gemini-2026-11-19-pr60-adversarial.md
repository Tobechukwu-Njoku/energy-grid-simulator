# Adversarial Review of PR #60 (M1 Open-Data Acquisition Plan)

- Author role/provider: OpenAI
- Reviewer role/provider: Google (adversarial)
- Reviewed head: `f4ecfa2ebf17e364c4cc3071198c00c54def1256`
- Verdict: **request changes**

## Scope

Adversarial review and robustness analysis of PR #60 (`docs/m1-acquisition-plan`, head `f4ecfa2ebf17e364c4cc3071198c00c54def1256`), specifically examining:
- `docs/research/m1-acquisition-plan.md`: M1 acquisition plan for external datasets without credentials across `SRC-01` through `SRC-50`.
- `docs/requirements/STATUS.md`: Milestone and PR status updates.

The review evaluated licence classifications (open-commercial vs. research-only), share-alike (ODbL/OSM) exposures, risks of raw research-only data commitment, correctness of URLs and API endpoints, and temporal/publication controls for the 2024 historical holdout.

## Findings

### Finding 1: Confirmed licence misclassification of Market Index Data (MID) within SRC-43
- **Status:** Confirmed
- **Category:** Source licence misclassification / Raw data commitment risk
- **Details:** In `SRC-43 — Elexon 2024 market/operational streams`, the overall stream bundle is classified as `Rights / raw commit: open-commercial, BMRS licence; sample-only`. This bundle includes Market Index Data (`/datasets/MID/stream`). However, Market Index Data is provided to Elexon by commercial power exchanges (such as EPEX SPOT and Nord Pool) under exchange licence agreements that restrict commercial redistribution. In this repository's established clearance registry (`spikes/data/sources.tsv`), `market-index-day.json` is explicitly audited and designated as `research-only`. Misclassifying MID as `open-commercial` permits raw sample extracts to be committed under `spikes/data/samples/*` pursuant to the sample-only contract, violating third-party exchange copyright and repository policy D-56.

### Finding 2: Confirmed URL and publication-vintage mismatch for SEMOpx price archive (SRC-44)
- **Status:** Confirmed
- **Category:** URL / API route error
- **Details:** In `SRC-44 — SEMO/SEMOpx historical prices`, the plan identifies the target SEMOpx price archive as `https://www.semopx.com/sites/semopx/files/2026-11/Lookback_Market%20October2026.zip`. The stated objective for M1 is the historical 2024 baseline holdout. A monthly lookback archive from October 2026 (`Lookback_Market October2026.zip`) contains only late-2026 price data and does not contain 2024 Day-Ahead Market (DAM) or Intraday (IDA1/IDA2/IDA3) prices. Executing a fetch using this target URL will fail to retrieve the 2024 all-island validation dataset, or will corrupt the 2024 backtest holdout with 2026 prices.

### Finding 3: Confirmed missing publication-time / as-of control on Elexon operational streams (SRC-43: BOALF, BOD, PN, MELS, MILS)
- **Status:** Confirmed
- **Category:** Missing as-of / publication-time control
- **Details:** For operational streams in `SRC-43` (BOALF acceptances, BOD bid/offer pairs, PN physical notifications, MELS/MILS limits), the plan specifies queries of the form `/datasets/{dataset}/stream?from={start}&to={end}&settlementPeriodFrom=1&settlementPeriodTo=50`. Per Elexon's OpenAPI specification (`/datasets/BOALF/stream`), `from` and `to` on these routes filter exclusively by the physical delivery timestamp (`TimeFrom`), not publication or submission timestamp (`publishDateTimeFrom`/`publishDateTimeTo`). These endpoints provide no publication-time filter. Ingesting them directly retrieves ex-post outturns and settlement revisions. Relying on this fetch contract for forecast backtesting without establishing an independent publication-time archive or acknowledging that publication-time filtering cannot be enforced via this API violates D-70 and ADR 0002.

### Finding 4: Confirmed temporal contamination and missing 2024 holdout controls in P1 and P2 fetch sequences (SRC-05, SRC-17, SRC-18, SRC-19)
- **Status:** Confirmed
- **Category:** Missing as-of / publication-time control
- **Details:** The operational fetch sequences in P1 and P2 prescribe immediate acquisition of key fleet and network sources:
  - `SRC-17`: P1 directs fetching `SRC-17`, whose URL points to the 13 November 2026 TEC register (`tec-register-13-november-2026.csv`).
  - `SRC-18`: P1 directs fetching `SRC-18`, whose URL points to the Q3 2026 REPD publication (`REPD_Publication_Q3_2026.csv`).
  - `SRC-19`: P1 directs fetching `SRC-19`, whose URL points to the May 2026 DUKES 5.11 power stations workbook (`DUKES_5.11.xlsx`).
  - `SRC-05`: P2 directs fetching `SRC-05`, whose advertised URL points to May 2026 SEPD tables (`southern-electric-power-distribution-long-term-development-statement-tables_may-2026.xlsx`).
  While the narrative notes that historical 2024 editions should be preferred, the concrete execution sequence prescribes fetching these sources immediately without pinning verified 2024 URLs, resource IDs, or hashes. Following this sequence as written will acquire 2026 fleet registries and network topology, introducing future commissioned generators into the 2024 model.

### Finding 5: Speculative third-party proprietary data redistribution risk in World Bank Pink Sheet (SRC-37b)
- **Status:** Speculative
- **Category:** Source licence misclassification / Raw data commitment risk
- **Details:** In `SRC-37b — World Bank Pink Sheet`, the dataset is classified as `open-commercial` under World Bank dataset terms (CC BY 4.0), allowing sample commits (`sample-only`). However, the Pink Sheet compiles proprietary benchmark price quotes directly from commercial reporting agencies (including S&P Global Platts for crude and European natural gas, ICE, and Argus McCloskey for coal). While the World Bank licenses its own publication under CC BY 4.0, its Terms of Use explicitly disclaim third-party intellectual property and require users to obtain separate permissions from underlying data providers. Classifying SRC-37b as `open-commercial` risks committing proprietary price series to the repository under the assumption of CC BY 4.0 clearance.
