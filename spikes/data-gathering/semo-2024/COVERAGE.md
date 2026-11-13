# SEMO and SEMOpx 2024 market price coverage

Independent verification on 2026-11-13 of the two pinned official archives. This is a
research inventory with documented limitations, not a complete calibration dataset.
Raw files remain outside git at `/home/njotob/energy-data/raw/semo/2024/` (D-56).
Reproduce with `python3 -I spikes/data-gathering/semo-2024/recount.py`; it uses only
the standard library, streams XML/CSV, and retains timestamp keys and aggregates.

## Integrity and provenance

| Archive | Bytes | SHA-256 |
| --- | ---: | --- |
| `Imbalance_Price_YTD_2020-2024.zip` | 4,459,701 | `16d0c54d1cebca28babec80bdd06af23668e84ce471cb2615f4f134ac0bbd1ac` |
| `Lookback_Market_October2026.zip` | 39,165,432 | `086eaa3e21e4589c9ee4b6fb7b42742c3197d17e1977c3d63aa9bb44317e037d` |

The total is 43,625,133 bytes. URLs, retrieval dates, and pinned hashes are in
`manifest.toml` and `sha256.txt`. Both entries remain `research-only`.

The [SEMO disclaimer](https://www.sem-o.com/disclaimer) and
[SEMOpx disclaimer](https://www.semopx.com/disclaimer) are warranty disclaimers,
not commercial reuse grants. Dataset-specific verbatim excerpts are in the manifest.
The [EirGrid Group Open Data Licence](https://www.smartgriddashboard.com/all/open-data-license/)
explicitly names SEMO/SEMOpx and permits commercial exploitation with attribution
“Supported by EirGrid Group Data”, but also limits the grant to information
“expressly made available under this licence”. Neither archive's publication page
established that express designation. No stronger file-specific terms were found;
retain the conservative D-56 tag pending clarification. This tag is a project
classification, not an assertion that the publisher prohibits commercial use.

## 2024 market units and primary-source checks

| Series | Measured 2024 cadence | Delivery coverage |
| --- | --- | --- |
| DAM | 60 minutes | Full UTC year |
| IDA1 | 30 minutes | Full UTC year |
| IDA2 | 30 minutes | 11:00–23:00 Irish civil time; starts through 22:30 |
| IDA3 | 30 minutes | 17:00–23:00 Irish civil time; starts through 22:30 |
| IDC | Aggregated price rows with date and hour only | Interval duration and distinct interval identities not recoverable from this sheet alone |
| Imbalance `30MIN` | 30 minutes | 61 absent intervals |
| Imbalance `5MIN` | 5 minutes | 333 absent intervals Jan–Nov; all December absent |

The IDA2/3 windows are 11:00–23:00 and 17:00–23:00 UTC in winter, and
10:00–22:00 and 16:00–22:00 UTC in summer. A fixed winter UTC grid falsely
produces 420 missing and 420 unexpected timestamps in each series.
The [SEMOpx intraday market page](https://www.semopx.com/markets/intraday-market)
confirms half-hour contracts and the delivery windows; the
[Industry Guide](https://www.sem-o.com/sites/semo/files/2024-12/I-SEM-Industry-Guide.pdf)
explains that market clock times observe GMT/IST and daylight saving. IDA1/2 are
coupled to GB; IDA3 is a local SEM auction. The previous pan-European IDA and
SIDC/XBID descriptions are removed: those affiliations were not established for
these 2024 records. IDC rows are aggregated weighted average prices, not a trade ledger.

[SPX_01_24 decision, 21 May 2024](https://www.semopx.com/sites/semopx/files/documents/market-modifications/SPX_01_24/SPX_01_24DecisionLetter.pdf),
page 1, approves the DAM change from 60 to 30 minutes and originally schedules
January 2025. It is a SEMOpx decision, not evidence of a SEM Committee approval.
The [implementation announcement](https://www.semopx.com/market-messages/15min-mtu-sdac-30min-mtu-sem-implemented)
confirms trading day **30 September 2025**, first delivery **1 October 2025**.
The archive supports hourly DAM throughout 2024. Claims of Q4 2024 member tests
and rollbacks were not substantiated and are removed.

The [SEM-21-015 publication](https://www.semcommittee.com/publications/sem-21-015-15-minute-imbalance-settlement-period-isp-exemption-information-paper)
includes the [signed 12 April 2021 letter](https://www.semcommittee.com/files/semcommittee/media-files/SEM-21-015_Letter_15-Min_ISP_Exemption.pdf).
Page 2 grants a three-year exemption **until 1 January 2024** under the EBGL /
Electricity Regulation framework. This verifies an earlier exemption, not a grant
covering calendar 2024. No renewal covering 2024 was established in this review.
The previous assertion of a formal derogation throughout 2024 is withdrawn.
Measured 30-minute settlement data does not itself establish the legal basis.

The Industry Guide, section 4.5.7.1, describes six five-minute imbalance prices
averaged into the half-hour settlement price. `FINAL_IMBALANCE_PRICE` is in EUR/MWh;
DAM/IDA prices are in EUR/MWh and GBP/MWh. Preserve the CSV flags verbatim:
the archive contains no codebook establishing the meanings of `I`, `B`, or `Y`.
The previous interpretations as indicative/backup/fallback are not verified here.

## Independent counts and price quality

Counts use the half-open UTC year `[2024-01-01T00:00Z, 2025-01-01T00:00Z)`;
IDC uses its supplied delivery date. Expected grids are independently generated,
including the summer/winter IDA2/3 shift. “Duplicates” means excess rows on a full
timestamp key for the six time series; IDC requires separate treatment below.

| Series | Expected | Rows | Missing | Duplicate timestamp rows | Null/nonfinite prices |
| --- | ---: | ---: | ---: | ---: | ---: |
| DAM | 8,784 | 8,784 | 0 | 0 | 0 |
| IDA1 | 17,568 | 17,568 | 0 | 0 | 0 |
| IDA2 | 8,784 | 8,784 | 0 | 0 | 0 |
| IDA3 | 4,392 | 4,392 | 0 | 0 | 0 |
| 30MIN | 17,568 | 17,507 | 61 | 0 | 0 |
| 5MIN | 105,408 | 96,147 | 9,261 | 0 | 0 |
| IDC | Unknown | 1,516 | Unknown | Not identifiable | 0 |

`30MIN` completeness is 99.6528%; `5MIN` is 91.2141%. The 30-minute series
ends at 31 December 23:00; its final 23:30 interval is absent. The five-minute
series ends at **30 November 23:55**: 8,928 of its 9,261 missing intervals are
December, not scattered operational gaps. Neither series should be imputed as
zero or described as complete.

IDC has 996 distinct `(deliverydate, deliveryhour)` keys: 520 occur twice,
producing 520 excess rows on that coarse key. There are 206 repeated full-field
rows across all 14 source columns. Those are not proof of duplicate trades:
the sheet omits the half-hour/offset key needed to distinguish two legitimate
intervals with equal values from duplicated observations. Do not deduplicate or
assign interval timestamps without an additional source. No IDC rows occur on
either clock-change date; that absence does not establish trading inactivity.

The `30MIN` flags are `I`: 17,493 and `B`: 14. Volumes are populated in 17,354
rows and absent in 153. The `5MIN` flags are `I`: 95,169 and `Y`: 978;
978 volumes are absent. Missing records and records with missing volumes are
separate quality dimensions.

## Actual 30-minute missing intervals

All times below are raw `START_TIME` labels, provisionally treated as UTC.
The earlier day-by-day list was incorrect despite matching monthly totals.

| Date in 2024 | Missing start times | Count |
| --- | --- | ---: |
| 2024-03-13 | 12:00, 12:30, 13:00, 14:00, 14:30, 15:00 | 6 |
| 2024-03-20 | 20:00, 20:30, 21:30, 22:00, 22:30 | 5 |
| 2024-04-30 | 12:00, 12:30, 13:00, 14:00, 14:30 | 5 |
| 2024-05-22 | 12:00, 12:30, 14:00, 14:30 | 4 |
| 2024-06-17 | 18:30, 19:00, 19:30, 20:00, 20:30, 21:00 | 6 |
| 2024-07-24 | 13:00 | 1 |
| 2024-07-30 | 18:30, 19:00, 19:30, 20:00, 20:30, 21:00, 21:30, 22:00, 22:30, 23:00 | 10 |
| 2024-08-20 | 14:00, 14:30, 15:30 | 3 |
| 2024-09-26 | 13:00, 13:30, 15:00 | 3 |
| 2024-10-09 | 18:30, 19:00, 19:30, 20:00, 20:30, 21:00, 21:30, 22:00, 22:30 | 9 |
| 2024-10-17 | 14:30 | 1 |
| 2024-11-27 | 14:00, 14:30, 15:30, 16:00 | 4 |
| 2024-12-18 | 14:30, 15:00, 16:00 | 3 |
| 2024-12-31 | 23:30 | 1 |

**Cause unverified.** These gaps span Mondays, Tuesdays, Wednesdays and Thursdays,
including evening blocks and the year-end boundary. No source-backed incident
mapping was established. The prior “weekly maintenance windows” explanation is
withdrawn. Preserve gaps explicitly; do not label their cause from a weekday pattern.

## Timezones and clock changes

SEMOpx auction timestamps have explicit `Z` offsets in the 2024 records. Parse them
as aware UTC instants, then use `ZoneInfo('Europe/Dublin')` for civil dates. Do not
add a fixed one-hour offset or remove repeated civil times.

ASSUMPTION: The imbalance archive's offsetless `DD/MM/YYYY HH:MM` `START_TIME`
labels represent UTC. Its uninterrupted 48/288-record transition UTC dates and
absence of repeated labels support this interpretation but do not prove it. The
archive has no timezone metadata; authoritative file-specific confirmation remains
open in Q-C-11. The imbalance Dublin counts below are conditional on this assumption.

| Series | 31 Mar UTC date | 31 Mar Dublin date | 27 Oct UTC date | 27 Oct Dublin date |
| --- | ---: | ---: | ---: | ---: |
| DAM | 24 | 23 | 24 | 25 |
| IDA1 | 48 | 46 | 48 | 50 |
| IDA2 | 24 | 24 | 24 | 24 |
| IDA3 | 12 | 12 | 12 | 12 |
| 30MIN | 48 | 46 | 48 | 50 |
| 5MIN | 288 | 276 | 288 | 300 |

At 01:00 UTC on 31 March, Dublin jumps from 00:59 GMT to 02:00 IST. At 01:00 UTC
on 27 October, Dublin returns from 01:59 IST to 01:00 GMT. A complete Dublin date
therefore has 23/25 hours, 46/50 half-hours, or 276/300 five-minute intervals.
Convert instants before grouping by local date. Keep UTC/offset or `fold` when
representing the repeated autumn hour; no observations need expansion or collapse.

## Price statistics

Arithmetic means over present rows only; no imputation, volume weighting, or
resampling. Values are rounded to two decimals. Missing five-minute December
records mean that statistic is **not a full-year observed mean**.

| Series / field | Currency or unit | Valid rows | Minimum | Maximum | Mean |
| --- | --- | ---: | ---: | ---: | ---: |
| DAM / `price_eur` | EUR/MWh | 8,784 | -5.02 | 499.99 | 108.91 |
| DAM / `price_gbp` | GBP/MWh | 8,784 | -4.29 | 411.95 | 92.07 |
| IDA1 / `price_eur` | EUR/MWh | 17,568 | -30.05 | 536.91 | 107.99 |
| IDA1 / `price_gbp` | GBP/MWh | 17,568 | -24.93 | 442.37 | 91.29 |
| IDA2 / `price_eur` | EUR/MWh | 8,784 | -17.62 | 563.16 | 118.87 |
| IDA2 / `price_gbp` | GBP/MWh | 8,784 | -14.92 | 464.00 | 100.45 |
| IDA3 / `price_eur` | EUR/MWh | 4,392 | -17.00 | 510.00 | 130.76 |
| IDA3 / `price_gbp` | GBP/MWh | 4,392 | -14.50 | 420.20 | 110.54 |
| 30MIN / `FINAL_IMBALANCE_PRICE` | EUR/MWh | 17,507 | -144.09 | 874.25 | 110.17 |
| 30MIN / `NET_IMBALANCE_VOLUME` | volume (source units) | 17,354 | -161.49 | 193.16 | 1.51 |
| 5MIN / `FINAL_IMBALANCE_PRICE` | EUR/MWh | 96,147 | -1000.00 | 498.11 | 107.01 |
| 5MIN / `NET_IMBALANCE_VOLUME` | volume (source units) | 95,169 | -164.34 | 199.20 | 1.44 |

The UTC-year DAM EUR mean is **108.914071038251…**, which rounds to **€108.91/MWh**.
The [official December 2024 Monthly Market Report](https://www.semopx.com/sites/semopx/files/2025-03/MONTHLY-mkt-report-Dec-2024.pdf),
PDF page 1, “Year To Date”, row 2024, “Avg. DAM (€/MWh)”, prints **€108.91**.
The reported figure therefore matches **at published two-decimal precision**;
this is not evidence that the publisher used an identical time boundary or method.
In particular, filtering the workbook's derived delivery `year = 2024` yields
108.916806693989… (€108.92), so those calendar definitions must not be interchanged.

## Downloader verification and remaining limits

The downloader streams downloads/hashes and checks both pinned size and digest.
Repeated runs validate existing files without network requests or raw-file changes.
Verification-only and dry-run do not create absent target directories. Destinations
inside any git checkout, including a different worktree, are rejected. Downloads
are staged in `.part` files and renamed only after integrity passes. No credentials
or secrets were found, and no raw archives, extracted tables, or raw samples are
tracked or added by this PR.

Outstanding source issues are the 2024 ISP exemption basis, explicit commercial
reuse designation, the offsetless imbalance timezone, IDC interval identities,
and missing imbalance observations. The spike records these limitations rather
than declaring the data complete or production-ready.
