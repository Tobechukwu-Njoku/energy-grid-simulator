# Network and Irish Data Gathering: Verified Coverage

Verified **2026-11-13** against the files and live publisher sites. This is a
research inventory under D-31/D-56, not a completed network or proof that all
MVP inputs have been obtained. No continental-European dataset was downloaded.

- **30 files, 126,027,753 bytes** (120.19 MiB). All local sizes and SHA-256 hashes
  match the manifest. All 30 original source URLs returned HTTP 200 after
  redirects and their live response bodies produced the same hashes.
- Raw storage: `/Users/njotob/energy-data/raw/{neso,dno,eirgrid,semo}/`, outside git.
- **16 `open-commercial`, 14 `research-only`**; evidence URLs, terms,
  verification date and obligations are recorded per entry in `manifest.toml`.
  A research-only tag is not a licence grant or permission to redistribute.
- Review: `docs/requirements/reviews/verify-pr11-2026-11-13.md`. The PR branch
  contains no raw files; **a separate local checkpoint ref retains two ETYS
  raw originals and 31 worksheet CSV exports**, so repository-wide raw-data
  safety is not yet clear.

## 1. Licences and Sources

The original blanket OGL claims were wrong. The downloader now uses the
verified manifest rather than a conflicting second catalogue.

| Files | Live evidence | Classification / obligations |
| --- | --- | --- |
| Eight ETYS publication files: Appendices A/B/C/D peak/D minimum/F/G and boundary chart workbook | [Publication hub][etys], [NESO terms][neso-terms]; no explicit association with a dataset-level open grant found | `research-only`; retain the coordinator's correction. Commercial clearance remains outstanding. |
| Six NESO portal files: boundary GIS, TEC, embedded register, 24-month limits, day-ahead flows/limits, thermal constraint costs | All six CKAN packages explicitly identify licence ID `ESO`, title NESO Open Data Licence. Exact package API URLs are in the manifest. | `open-commercial`, **National Energy SO Open Data Licence v1.0**. Attribute “Supported by National Energy SO Open Data”; observe exclusions. Not OGL. |
| Eight NGED files: four LTDS CSVs and four regional CIM ZIPs | [Tabular][nged-tables] and [CIM][nged-cim] metadata identify legacy `wpd-open-data` / WPD Open Data Licence. Dataset pages link the current [NGED licence][nged-licence]. | `open-commercial`, **NGED Open Data Licence**, December 2020 version. Attribute “Supported by NGED Open Data”; third-party rights excluded. Not OGL. |
| Two SSEN SEPD resources: workbook and EQ ZIP | [Package metadata][ssen-package] explicitly identifies `CC-BY-4.0` and lists both resources. | `open-commercial`, **CC BY 4.0**. Credit SSEN, link the licence and identify changes. Not OGL. |
| Three EirGrid and three SEMO/SEMOpx resources | [Reports][eirgrid-reports], [TYTFS publication][tytfs], [SEMO][semo], [SEMOpx library][semopx]. The [EirGrid Group Open Data Licence][eirgrid-licence] exists and names SEMO/SEMOpx, but express coverage of these exact files was not established. | Retain `research-only` pending file-specific confirmation. Do not claim the group licence is absent or that SEMO disclaimers impose a commercial ban; disclaimers are not data licences. |

No ODbL/OSM or other share-alike dataset is selected. Research-only raw files
must not be committed or redistributed. Commercial clearance remains tracked
in `docs/requirements/TODO-pre-commercial.md`.

## 2. NESO: Actual File Contents

Paths are relative to `/Users/njotob/energy-data/raw/neso/`. Worksheet dimensions
include formatting; counts below use actual populated rows or identified tables.

| File | Observed contents / bounds |
| --- | --- |
| `etys-2025-appendix-b-system-technical-data.xlsx` | **31 sheets**: substation codes, circuits, transformers, reactive plant, changes and HVDC. Existing circuit sheets B-2-1a/b/c/d contain **310/352/614/116 records** after two headings, base year **2025/26**. Does not prove complete GB/Scottish 132 kV asset coverage. |
| `etys-2025-boundary-chart-data.xlsx` | **456 records**, **21 named boundaries**, HE/EE/HT scenarios, annual columns **2025–2045**. Boundaries: B0, B1a, B2, B4, B5, B6, B7a, B8, B9, B13, SC1, SC1.5, SC2, SC3, SW1, LE1, NW1, NW2, NW3, NW4, EC5; not every B0–B17 boundary. |
| `etys-2025-appendix-a-system-schematics.pdf` | **8 pages**, network schematics/maps; source says locations are approximate/illustrative and NGET transmission circuit depiction is 275 kV and above. Not authoritative coordinates. |
| `etys-2025-appendix-c-power-flows.pdf` | **5 pages**, schematic power-flow maps for ETYS years **1, 2, 4, 6, 9**. Not a machine-readable voltage-profile time series. |
| `etys-2025-appendix-d-fault-peak.xlsx` | Menu plus **15 fault-current sheets**, five study years for SHE Transmission/SPT/NGET. Data-row counts by sheet: D1.1–D1.5 **237/235/256/304/325**, D2.1–D2.5 **299/307/324/348/325**, D3.1–D3.5 **618/620/629/661/675**. |
| `etys-2025-appendix-d-fault-min.xlsx` | Menu plus **3 summer-minimum 2025/26 sheets**; D1.0/D2.0/D3.0 contain **237/299/617 records**. |
| `etys-2025-appendix-f-contracted-generation.xlsx` | **2,242 populated rows including two headings**; TEC report explicitly dated **31 October 2025**. Not the November 2026 CSV snapshot. |
| `etys-2025-appendix-g-nodal-demand.xlsx` | **636 node rows**, user-submitted GSP winter peak demands, not a FES nodal breakdown. Eight demand columns; source repeats **31/32** in the last two year headings. Preserve and resolve, rather than inventing a corrected year. |
| `etys-boundary-gis-data-mar25.zip` | Valid `.shp/.shx/.dbf/.prj/.cpg` bundle, **34 features**; boundary geometry, not substation coordinates or circuit routes. |
| `tec-register-10-november-2026.csv` | **2,220 project-stage rows**, **1,759 direct / 461 embedded** agreements, snapshot **10 November 2026**. Connected and future/scoping projects; not a deduplicated operational fleet. |
| `embedded-register-10-november-2026.csv` | **560 Scottish project-stage rows**, HOST TO **SHET 351 / SPT 209**. Live dataset says Scotland; not a GB-wide DNO embedded register. |
| `24-months-ahead-constraint-limit_nov26.csv` | **109 weekly rows**, 12 constraint-group columns, **2026 week 49–2028 week 52**. Forecast limits, not 2024 outturn. |
| `day-ahead-constraints-limits-and-flow-output-v1.5.csv` | **764,304 rows**, 32 groups, **2023-01-01 00:00–2026-11-13 23:30**. Forecast `Limit_MW`/`Flow_MW`, not outturn. Timestamp field explicitly says **GMT_BST**; local-time duplicates and uneven group histories require care. |
| `outturn-system-costs-2024-2025.csv` | **2,190 rows**, six thermal constraint groups, **2024-04-01–2025-03-31** (365 dates). `Daily Cost GBP`, not all balancing costs or a full calendar-year 2024 series. |

## 3. DNO 132 kV Coverage

Five licence areas have downloads: NGED EMIDS/WMIDS/SWALES/SWEST and SSEN SEPD.
This is **partial** D-60 coverage. Paths are relative to
`/Users/njotob/energy-data/raw/dno/`.

| File(s) | Observed contents / bounds |
| --- | --- |
| `ltds-table-1-circuit.csv` | **22,516 equipment rows**, all four NGED regions. **4,183 at 132 kV**: **1,138 circuits**, **1,862 disconnectors**, **1,183 circuit breakers**. R1/X1 are `-` in 3,045 switchgear rows. Do not treat every row as an impedance-bearing circuit. Other levels: 66/33/25/11/6.6 kV. |
| `ltds-table-2a-two-winding-transformer.csv` | **2,705 records**. |
| `ltds-table-2b-three-winding-transformer.csv` | **69 records**. |
| `ltds-attachment-8-node.csv` | **22,590 records**. |
| `nged-cim-{emids,wmids,swales,swest}.zip` | Each contains **3 nested ZIPs** labelled January 2026: existing fault level, existing system capacity and FY2026–2027 system capacity. Fault-level ZIPs contain **EQ/SC/SCR XML**; capacity ZIPs contain **EQ/SYSCAP XML**, CIM100 RDF. Not four flat, ready-to-solve cases. |
| `southern-electric-power-distribution-long-term-development-statement-tables_may-2026.xlsx` | **9 worksheets**, Tables 1–7 with split transformer/fault tables. Table 1: **2,257 data rows** after three headings, **435 at 132 kV**. |
| `ltds_sepd_2025-02_eq_2025-10-20_v1.0.zip` | One **45,332,952-byte EQ XML**, `20251125T1444Z__EQ_1.xml`, with **2,305 ACLineSegment / 1,196 PowerTransformer** objects. Equipment profile, not a solved operating state. Outer and inner date labels differ. |

### Missing Areas and Access Evidence

No accounts were created. On **2026-11-13**, each of the first three listed
datasets returned **403 ForbiddenAccess** from `/records?limit=1`; corresponding
`/exports/csv` requests returned **HTTP 200, one header row, zero records**.
Catalogue record counts are not proof of anonymous record access.

| Missing areas | Exact source | Status |
| --- | --- | --- |
| UKPN EPN/LPN/SPN (3) | `https://ukpowernetworks.opendatasoft.com/api/explore/v2.1/catalog/datasets/ukpn-ltds-table-1-circuit-data` | Needs owner login/access; catalogue lists **5,076** records. |
| SPEN SP Manweb (1) | `https://spenergynetworks.opendatasoft.com/api/explore/v2.1/catalog/datasets/spm-ltds-appendix-1-circuit-data-table-1` | Needs owner login/access; catalogue lists **1,798** records. |
| Northern Powergrid Northeast/Yorkshire (2) | `https://northernpowergrid.opendatasoft.com/api/explore/v2.1/catalog/datasets/ltds-appendix-3` | Needs owner login/access; catalogue lists **1,670** records. |
| Electricity North West (1) | [LTDS secure area][enwl] | Live page requests registration/login. No authenticated download attempted; reuse terms unverified. |

The two Scottish distribution areas, SHEPD and SP Distribution, are separate
from these seven English/Welsh gaps. ETYS represents Scottish 132 kV
transmission; it does **not** establish that all Scottish 132 kV parameters are
“fully captured”. Asset completeness still requires reconciliation.

## 4. EirGrid / SONI

All three files remain `research-only`; paths are relative to
`/Users/njotob/energy-data/raw/eirgrid/`.

| File | Observed contents / bounds |
| --- | --- |
| `TYTFS2024_studyfiles.zip` | **17 members**: disclaimer plus **8 RAW / 8 SEQ** files. Only **Summer Valley (SV)** and **Winter Peak (WP)** for **2024 / 2033**, each in **PSS/E 33 / 35**. No Summer Peak or Autumn. Bus/load/generator/branch/transformer/DC sections are present; these are forecast study cases, not measured operating snapshots. CRCs pass. |
| `System-Data-Qtr-Hourly-2024.xlsx` | **35,136 data rows**, **2024-01-01 00:00–2024-12-31 23:45**. NI/IE/all-island demand, generation, wind/solar, hydro, EWIC/Moyle, inter-jurisdictional flow, SNSP. First 34 data columns have no missing cells. Four repeated local timestamps become **35,136 unique consecutive UTC quarter-hours** after subtracting `GMT Offset`. Never deduplicate local `DateTime` alone. |
| `System-and-Renewable-Data-Summary-Report-V30.xlsx` | **7 sheets**: KPI, System Data Summary, Fuel Mix & CO2, Wind Installed Capacities, NI Connected Wind & Solar, IE Connected Renewables, IE Connected Non-Renewables. Summary month headings span **January 2014–September 2026**; other tabs have different connection-date histories, not one uniform time series. |

## 5. SEMO / SEMOpx

All three files remain `research-only`; paths are relative to
`/Users/njotob/energy-data/raw/semo/`.

| File | Observed contents / bounds |
| --- | --- |
| `lookback_mkt.xlsx` | `auctions_to`: **88,863 records** (DAM 19,753; IDA1 39,506; IDA2 19,728; IDA3 9,876). `idcs_to`: **17,564 records**. Delivery dates **2018-10-01–2020-12-31**, **no 2024 prices**. EUR/GBP auction prices, volumes/net positions, IDC averages/volumes; no field establishing a harmonised reference-price series. Four chart tabs have no cell data; formatted dimensions overstate table rows. |
| `PUB_MnlyRegisteredCapacity_202412011000.xml` | **1,993 records / 702 distinct ResourceName values**, publish time **2024-12-01 10:00:01**. Repeated resources must not be blindly summed. Capacity, not prices. |
| `PUB_DailyRegisteredUnits_202609140734.xml` | **769 distinct UnitID values**, publish time **2026-09-14 07:34:45**. Includes 342 GEN, 221 SU, 129 AU, 60 DSU and other types; not generating units alone or a December 2024 fleet snapshot. |

**Gap:** the intended **calendar-year 2024 Irish prices have not been gathered**.
A separately verified SEMOpx/SEMO/ENTSO-E source is still needed (D-57).

## 6. ENTSO-E and Reproduction

**Needs owner token.** A tokenless request to
`https://web-api.tp.entsoe.eu/api` for A44 prices, Ireland domain
`10Y1001A1001A59C`, **2024-01-01–2024-01-02** returned **401**. No account/token
was created and no ENTSO-E data is in this manifest. Refer to the [official
platform][entsoe] for procedures; manual-export behaviour was not verified.

Requires **Python 3.11+**, standard library only:

```bash
python3 spikes/data-gathering/network-ie/fetch.py --dry-run
python3 spikes/data-gathering/network-ie/fetch.py
```

Normal runs verify cached files against committed sizes/hashes. Missing files
are downloaded and must match those same pins before installation. Corrupt
caches or changed upstream bodies fail instead of silently blessing a new
digest. Cached retrieval dates and licence evidence are preserved. New upstream
versions require deliberate content/licence review before changing the
manifest. Bodies and temporary download `.part` files stay outside git.

[etys]: https://www.neso.energy/publications/electricity-ten-year-statement-etys
[neso-terms]: https://www.neso.energy/terms-and-conditions
[nged-tables]: https://connecteddata.nationalgrid.co.uk/dataset/ltds-tabular-model
[nged-cim]: https://connecteddata.nationalgrid.co.uk/dataset/ltds-common-information-model
[nged-licence]: https://www.nationalgrid.co.uk/open-data-licence
[ssen-package]: https://data-api.ssen.co.uk/api/3/action/package_show?id=695078a0-bbd8-420b-8383-ee6bf7fc0045
[eirgrid-reports]: https://www.eirgrid.ie/grid/system-and-renewable-data-reports
[tytfs]: https://www.eirgrid.ie/all-island-ten-year-transmission-forecast-statement-2024
[eirgrid-licence]: https://www.smartgriddashboard.com/all/open-data-license/
[semo]: https://www.sem-o.com/market-data
[semopx]: https://www.semopx.com/market-data/document-library
[enwl]: https://www.enwl.co.uk/get-connected/network-information/long-term-development-statement/login-to-ltds-secure-area/
[entsoe]: https://transparency.entsoe.eu/
