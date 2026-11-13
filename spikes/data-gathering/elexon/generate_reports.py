#!/usr/bin/env python3
"""Audit downloaded Elexon 2024 dataset, compute SHA-256 checksums,
and generate manifest.toml and COVERAGE.md.
"""

import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import sys
import tomllib

sys.path.insert(0, str(Path(__file__).resolve().parent))
from stream_rows import rows_from

RAW_BASE = Path(os.environ.get("DATA_DIR", str(Path.home() / "energy-data/raw/elexon")))
OUTPUT_DIR = Path(__file__).resolve().parent
LICENCE_URL = "https://www.elexon.co.uk/bsc/data/balancing-mechanism-reporting-agent/copyright-licence-bmrs-data/"
SETTLEMENT_DATES = [(datetime.date(2024, 1, 1) + datetime.timedelta(days=offset)).isoformat() for offset in range(366)]
EXPECTED_PERIODS = {
    (day, period) for day in SETTLEMENT_DATES
    for period in range(1, {"2024-03-31": 46, "2024-10-27": 50}.get(day, 48) + 1)
}

DATASETS_CONFIG = [
    {
        "id": "bmunits",
        "name": "BM Unit Reference Data",
        "category": "Reference",
        "endpoint": "/reference/bmunits/all?format=json",
        "date_range": "Current reference snapshot; not point-in-time 2024 reference data",
        "expected_sp": None,
        "is_stream": False,
        "is_gz": False,
        "pattern": "bmunits/*.json",
        "description": "National Grid BM Unit ID, Elexon BM Unit ID, fuel type, generation/demand capacity, lead party, and GSP group.",
    },
    {
        "id": "system_prices",
        "name": "System Buy/Sell Prices (Imbalance Prices)",
        "category": "Balancing Mechanism",
        "endpoint": "/balancing/settlement/system-prices/{settlementDate}?format=json",
        "date_range": "2024-01-01 to 2024-12-31",
        "expected_sp": 17568,
        "is_stream": False,
        "is_gz": False,
        "pattern": "system_prices/*.json",
        "description": "Settlement system sell and buy prices (SSP/SBP, single imbalance price regime since BSC P305), net imbalance volume (NIV).",
    },
    {
        "id": "demand_outturn",
        "name": "Demand Outturn (INDO/ITSDO)",
        "category": "Demand",
        "endpoint": "/demand/outturn?settlementDateFrom={d}&settlementDateTo={d}&format=json",
        "date_range": "2024-01-01 to 2024-12-31",
        "expected_sp": 17568,
        "is_stream": False,
        "is_gz": False,
        "pattern": "demand_outturn/*.json",
        "description": "Initial Demand Outturn (INDO) and Initial Transmission System Demand Outturn (ITSDO) in MW.",
    },
    {
        "id": "fuelhh",
        "name": "Half-Hourly Generation by Fuel Type (FUELHH)",
        "category": "Generation",
        "endpoint": "/datasets/FUELHH/stream?settlementDateFrom={d}&settlementDateTo={d}",
        "date_range": "2024-01-01 to 2024-12-31",
        "expected_sp": 17568,
        "is_stream": True,
        "is_gz": False,
        "pattern": "fuelhh/*.json",
        "description": "Outturn generation in MW by reported fuel type and interconnector category per settlement period.",
    },
    {
        "id": "interconnectors",
        "name": "Interconnector Flows",
        "category": "Interconnectors",
        "endpoint": "/generation/outturn/interconnectors?settlementDateFrom={d}&settlementDateTo={d}&format=json",
        "date_range": "2024-01-01 to 2024-12-31",
        "expected_sp": 17568,
        "is_stream": False,
        "is_gz": False,
        "pattern": "interconnectors/*.json",
        "description": "Half-hourly reported flows for 10 interconnector categories, including sparse Greenlink observations; category presence is not a commissioning date.",
    },
    {
        "id": "mid",
        "name": "Market Index Data (Market Index Price)",
        "category": "Wholesale Market",
        "endpoint": "/datasets/MID/stream?from={d}&to={d}&settlementPeriodFrom=1&settlementPeriodTo=50",
        "date_range": "2024-01-01 to 2024-12-31",
        "expected_sp": 17568,
        "is_stream": True,
        "is_gz": False,
        "pattern": "mid/*.json",
        "description": "Market Index Data (MID) reflecting half-hourly wholesale trading prices (APXMIDP and N2EXMIDP) used in settlement. Labelled strictly as 'Market Index Price', NOT day-ahead auction price.",
    },
    {
        "id": "remit",
        "name": "REMIT Market Messages",
        "category": "Transparency / REMIT",
        "endpoint": "/datasets/REMIT/stream?publishDateTimeFrom={ts}&publishDateTimeTo={ts}",
        "date_range": "2024-01-01T00:00:00Z to 2025-01-01T00:00:00Z",
        "expected_sp": None,
        "is_stream": True,
        "is_gz": False,
        "pattern": "remit/*.json",
        "description": "Market messages published under REMIT covering outages, other market information, revisions, available capacities and causes.",
    },
    {
        "id": "boalf",
        "name": "Bid-Offer Acceptances (BOALF)",
        "category": "Balancing Mechanism",
        "endpoint": "/datasets/BOALF/stream?from={d}&to={d}&settlementPeriodFrom=1&settlementPeriodTo=50",
        "date_range": "2024-01-01 to 2024-12-31",
        "expected_sp": 17568,
        "is_stream": True,
        "is_gz": False,
        "pattern": "boalf/*.json",
        "description": "Acceptance profile segments issued by ESO, with acceptance numbers, unit IDs, MW levels, time intervals, SO/STOR flags and amendments.",
    },
    {
        "id": "mels",
        "name": "Maximum Export Limits (MEL)",
        "category": "Balancing Mechanism",
        "endpoint": "/datasets/MELS/stream?from={d}&to={d}&settlementPeriodFrom=1&settlementPeriodTo=50",
        "date_range": "2024-01-01 to 2024-12-31",
        "expected_sp": 17568,
        "is_stream": True,
        "is_gz": True,
        "pattern": "mels/*.json.gz",
        "description": "Maximum Export Limit notifications submitted by BM units per settlement period.",
    },
    {
        "id": "mils",
        "name": "Maximum Import Limits (MIL)",
        "category": "Balancing Mechanism",
        "endpoint": "/datasets/MILS/stream?from={d}&to={d}&settlementPeriodFrom=1&settlementPeriodTo=50",
        "date_range": "2024-01-01 to 2024-12-31",
        "expected_sp": 17568,
        "is_stream": True,
        "is_gz": True,
        "pattern": "mils/*.json.gz",
        "description": "Maximum Import Limit notifications submitted by BM units per settlement period.",
    },
    {
        "id": "pn",
        "name": "Physical Notifications (PN)",
        "category": "Balancing Mechanism",
        "endpoint": "/datasets/PN/stream?from={d}&to={d}&settlementPeriodFrom=1&settlementPeriodTo=50",
        "date_range": "2024-01-01 to 2024-12-31",
        "expected_sp": 17568,
        "is_stream": True,
        "is_gz": True,
        "pattern": "pn/*.json.gz",
        "description": "Intended generation or demand profiles notified by BM units per settlement period.",
    },
    {
        "id": "bod",
        "name": "Bid-Offer Data (BOD)",
        "category": "Balancing Mechanism",
        "endpoint": "/datasets/BOD/stream?from={d}&to={d}&settlementPeriodFrom=1&settlementPeriodTo=50",
        "date_range": "2024-01-01 to 2024-12-31",
        "expected_sp": 17568,
        "is_stream": True,
        "is_gz": True,
        "pattern": "bod/*.json.gz",
        "description": "Submitted bid and offer price/MW pairs for balancing mechanism BM units across all settlement periods.",
    },
]

def sha256_file(filepath):
    h = hashlib.sha256()
    with open(filepath, "rb") as f:
        while chunk := f.read(1024 * 1024):
            h.update(chunk)
    return h.hexdigest()

def analyze_dataset(cfg, refresh_checksums=False):
    ds_id = cfg["id"]
    files = sorted(RAW_BASE.glob(cfg["pattern"]))
    if {f for f in (RAW_BASE / ds_id).iterdir()} != set(files):
        raise ValueError(f"unexpected files in {ds_id}")
    file_count = len(files)
    total_bytes = 0
    checksum_lines = []

    print(f"Auditing {ds_id}: {file_count} files...")
    
    index_file = OUTPUT_DIR / f"{ds_id}_sha256.txt"
    for filepath in files:
        total_bytes += filepath.stat().st_size
        checksum_lines.append(f"{sha256_file(filepath)}  {filepath.name}")
    if not files:
        raise ValueError(f"no input files for {ds_id}")
    actual_index = "\n".join(checksum_lines) + "\n"
    if refresh_checksums:
        index_file.write_text(actual_index)
    else:
        indexed = {}
        for line in index_file.read_text().splitlines():
            digest, filename = line.split(maxsplit=1)
            filename = filename.removeprefix("*")
            if filename in indexed:
                raise ValueError(f"duplicate checksum entry: {filename}")
            indexed[filename] = digest
        actual = {line.split(maxsplit=1)[1]: line.split(maxsplit=1)[0] for line in checksum_lines}
        if indexed != actual:
            raise ValueError(f"{ds_id}: checksum or filename mismatch; investigate before explicitly refreshing indexes")
    index_digest = hashlib.sha256(index_file.read_bytes()).hexdigest()

    # Deep data inspection
    distinct_dates = set()
    distinct_sp = set()
    date_sp_counts = {}
    total_records = 0
    sample_record = None
    revisions_seen = 0
    series_periods = {}
    remit_identifiers = {}
    remit_seen = set()
    remit_duplicates = 0
    dimension = {"mid": "dataProvider", "fuelhh": "fuelType", "interconnectors": "interconnectorName"}.get(ds_id)

    for f in files:
        for r in rows_from(f):
            total_records += 1
            if sample_record is None:
                sample_record = r
            dt = r.get("settlementDate")
            sp = r.get("settlementPeriod")
            if dt and sp:
                distinct_dates.add(dt)
                distinct_sp.add((dt, sp))
                date_sp_counts.setdefault(dt, set()).add(sp)
                if dimension:
                    series_periods.setdefault(r[dimension], set()).add((dt, sp))
            elif ds_id == "boalf" and dt:
                spf = r.get("settlementPeriodFrom")
                spt = r.get("settlementPeriodTo")
                distinct_dates.add(dt)
                if spf:
                    distinct_sp.add((dt, spf))
                    date_sp_counts.setdefault(dt, set()).add(spf)
                if spt:
                    distinct_sp.add((dt, spt))
                    date_sp_counts.setdefault(dt, set()).add(spt)

            if "revisionNumber" in r and r.get("revisionNumber", 0) > 1:
                revisions_seen += 1
            if ds_id == "remit":
                remit_identifiers.setdefault(r["messageType"], set()).add(r["mrid"])
                fingerprint = hashlib.sha256(json.dumps(r, sort_keys=True).encode()).digest()
                remit_duplicates += fingerprint in remit_seen
                remit_seen.add(fingerprint)
            if "amendmentFlag" in r and r.get("amendmentFlag") not in (None, False, "ORI"):
                revisions_seen += 1

    # Check clock change days
    spring_sp = len(date_sp_counts.get("2024-03-31", []))
    autumn_sp = len(date_sp_counts.get("2024-10-27", []))
    regular_sp_days = [d for d, sps in date_sp_counts.items() if len(sps) == 48]

    return {
        "cfg": cfg,
        "file_count": file_count,
        "total_bytes": total_bytes,
        "index_file": index_file.name,
        "index_sha256": index_digest,
        "total_records": total_records,
        "distinct_dates_count": len(distinct_dates),
        "distinct_sp_count": len(distinct_sp),
        "covered_sp_count": len(distinct_sp & EXPECTED_PERIODS),
        "missing_dates": sorted(set(SETTLEMENT_DATES) - {f.name[:10] for f in files}) if cfg["expected_sp"] else [],
        "missing_periods": sorted(EXPECTED_PERIODS - distinct_sp) if cfg["expected_sp"] else [],
        "unexpected_periods": sorted(distinct_sp - EXPECTED_PERIODS),
        "spring_sp": spring_sp,
        "autumn_sp": autumn_sp,
        "regular_48_days": len(regular_sp_days),
        "revisions_seen": revisions_seen,
        "sample": sample_record,
        "series_periods": {label: len(periods) for label, periods in sorted(series_periods.items())},
        "remit_identifiers": {kind: len(identifiers) for kind, identifiers in sorted(remit_identifiers.items())},
        "remit_identifier_union": len(set().union(*remit_identifiers.values())) if remit_identifiers else 0,
        "exact_duplicates": remit_duplicates if ds_id == "remit" else None,
    }

def main():
    global OUTPUT_DIR
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=OUTPUT_DIR)
    parser.add_argument("--refresh-checksums", action="store_true", help="explicitly replace indexes after investigating source changes")
    parser.add_argument("--generated-at", help="override the preserved report generation timestamp")
    args = parser.parse_args()
    if not RAW_BASE.is_dir():
        parser.error(f"missing raw input directory: {RAW_BASE}")
    OUTPUT_DIR = args.output_dir
    if not OUTPUT_DIR.is_dir():
        parser.error("output directory must exist and contain the checksum indexes")
    previous = OUTPUT_DIR / "manifest.toml"
    generated_at = args.generated_at or (tomllib.loads(previous.read_text())["manifest"]["generated_at"] if previous.exists() else datetime.datetime.now(datetime.timezone.utc).isoformat())
    results = []
    grand_total_bytes = 0
    grand_total_files = 0
    grand_total_records = 0

    for cfg in DATASETS_CONFIG:
        res = analyze_dataset(cfg, args.refresh_checksums)
        results.append(res)
        grand_total_bytes += res["total_bytes"]
        grand_total_files += res["file_count"]
        grand_total_records += res["total_records"]

    complete_unions = sum(bool(result["cfg"]["expected_sp"]) and not result["missing_periods"] and not result["unexpected_periods"] for result in results)
    sparse_unions = "; ".join(f"{result['cfg']['id']}: {result['distinct_sp_count']:,}" for result in results if result["missing_periods"])
    dimension_gaps = "; ".join(
        f"{result['cfg']['id']}/{label}: {17568 - count:,} absent full-year observations"
        for result in results for label, count in result["series_periods"].items() if count < 17568
    )
    remit_result = next(result for result in results if result["cfg"]["id"] == "remit")

    # 1. Generate manifest.toml
    manifest_lines = [
        "# Elexon Insights Solution (BMRS) Data Manifest - Calendar Year 2024",
        "# GB Electricity Market & Operations Backtest Dataset",
        "# Licence: BMRS Open Data Licence (open-commercial)",
        "",
        "[manifest]",
        'version = "1.0.0"',
        f'generated_at = "{generated_at}"',
        'calendar_year = 2024',
        'is_leap_year = true',
        'expected_settlement_periods = 17568',
        'spring_clock_change = "2024-03-31 (46 settlement periods)"',
        'autumn_clock_change = "2024-10-27 (50 settlement periods)"',
        'licence = "BMRS Open Data Licence"',
        'licence_status = "open-commercial"',
        f'licence_url = "{LICENCE_URL}"',
        'attribution = "Contains BMRS data © Elexon Limited copyright and database right 2024."',
        'reference_attribution = "Contains BMRS data © Elexon Limited copyright and database right 2026."',
        'licence_exclusions = "Personal data; third-party rights Elexon is not authorised to license; other intellectual-property rights."',
        'coverage_scope = "Settlement-period union, not completeness of every provider, unit or message."',
        'base_api_url = "https://data.elexon.co.uk/bmrs/api/v1"',
        f'total_datasets = {len(results)}',
        f'total_files = {grand_total_files}',
        f'total_bytes = {grand_total_bytes}',
        f'total_bytes_human = "{grand_total_bytes / (1024**3):.2f} GiB"',
        f'total_records = {grand_total_records}',
        "",
    ]

    for res in results:
        cfg = res["cfg"]
        ds_id = cfg["id"]
        manifest_lines.extend([
            f"[datasets.{ds_id}]",
            f'name = "{cfg["name"]}"',
            f'category = "{cfg["category"]}"',
            f'endpoint = "{cfg["endpoint"]}"',
            f'date_range = "{cfg["date_range"]}"',
            f'file_count = {res["file_count"]}',
            f'total_bytes = {res["total_bytes"]}',
            f'total_records = {res["total_records"]}',
            f'sha256_index_file = "{res["index_file"]}"',
            f'sha256_index_digest = "{res["index_sha256"]}"',
            f'expected_settlement_periods = {cfg["expected_sp"] if cfg["expected_sp"] is not None else 0}',
            f'actual_settlement_periods = {res["distinct_sp_count"]}',
            'licence = "BMRS Open Data Licence"',
            'licence_status = "open-commercial"',
            f'licence_url = "{LICENCE_URL}"',
            f'attribution = "Contains BMRS data © Elexon Limited copyright and database right {2026 if ds_id == "bmunits" else 2024}."',
            f'coverage_scope = "{"Current reference snapshot" if ds_id == "bmunits" else "Publication-window capture; source duplicate retained" if ds_id == "remit" else "Settlement-period union only; see COVERAGE.md for dimension gaps"}"',
            "",
        ])
        if ds_id == "bmunits":
            manifest_lines.extend(['retrieved_at = "Unknown; not recorded during original download"', 'verified_at = "2026-11-13"', ""])

    manifest_path = OUTPUT_DIR / "manifest.toml"
    manifest_path.write_text("\n".join(manifest_lines) + "\n")
    print(f"Wrote {manifest_path}")

    # 2. Generate COVERAGE.md
    cov_lines = [
        "# Elexon BMRS 2024 Data Gathering Coverage Report",
        "",
        "## Executive Summary",
        "",
        f"- **Coverage Year**: Calendar Year 2024 (Leap year, 366 days, 2024-01-01 to 2024-12-31).",
        f"- **Expected Settlement Periods**: **17,568** half-hour periods (364 days × 48 + 1 short day × 46 + 1 long day × 50).",
        f"- **Settlement-Period Union**: **17,568** in {complete_unions} scheduled time-series datasets. Sparse unions: {sparse_unions or 'none'}. This does not prove completeness of each provider, fuel, interconnector or BM unit.",
        f"- **Total Datasets**: {len(results)} distinct streams and endpoints.",
        f"- **Total Downloaded Files**: {grand_total_files:,} files.",
        f"- **Total Records**: {grand_total_records:,} raw records, including {remit_result['exact_duplicates']} exact REMIT duplicate(s) retained for source fidelity.",
        f"- **Total Storage Volume**: {grand_total_bytes / (1024**3):.2f} GiB ({grand_total_bytes:,} bytes).",
        f"- **Raw Data Storage Location**: `{RAW_BASE}/` (outside git; override with `DATA_DIR`).",
        "- **Licence**: **BMRS Open Data Licence** (`open-commercial` - allows commercial and non-commercial use with attribution).",
        "- **Market Index Data Label**: Strictly labelled as **Market Index Price**, NOT day-ahead auction price.",
        "- **Known Limitations**: Provider/fuel/interconnector panels can be sparse even when the SP union is complete. The reference snapshot is current, not 2024. The raw capture is not deduplicated. See per-series counts below and the dated independent verification report.",
        f"- **Licence Source**: [Licence to use BMRS open data]({LICENCE_URL}), checked 2026-11-13. Personal data and unauthorised third-party rights are excluded; do not imply Elexon endorsement. Carry the attribution and licence link into derived products and sublicences.",
        "- **Historical Data Attribution**: Contains BMRS data © Elexon Limited copyright and database right 2024.",
        "- **Current Reference Attribution**: Contains BMRS data © Elexon Limited copyright and database right 2026.",
        "",
        "---",
        "",
        "## Dataset Summary Table",
        "",
        "| Dataset ID | Dataset Name | Files | Size (MiB) | Records | Expected SP | Actual SP | Coverage | Status |",
        "|---|---|---|---|---|---|---|---|---|",
    ]

    for res in results:
        cfg = res["cfg"]
        exp_sp_str = str(cfg["expected_sp"]) if cfg["expected_sp"] is not None else "N/A"
        act_sp_str = str(res["distinct_sp_count"]) if cfg["expected_sp"] is not None else "N/A"
        cov_pct = "N/A" if cfg["expected_sp"] is None else f"{(res['covered_sp_count']/cfg['expected_sp'])*100:.2f}%"
        status = "Snapshot" if cfg["id"] == "bmunits" else "Source duplicate retained" if cfg["id"] == "remit" else "Partial SP union" if res["missing_periods"] or res["unexpected_periods"] else "SP union only"
        cov_lines.append(
            f"| `{cfg['id']}` | {cfg['name']} | {res['file_count']} | {res['total_bytes'] / (1024**2):.1f} MiB | {res['total_records']:,} | {exp_sp_str} | {act_sp_str} | {cov_pct} | {status} |"
        )

    cov_lines.extend([
        "",
        "---",
        "",
        "## Clock Change Days Analysis",
        "",
        "The UK electricity market operates on 48 half-hour settlement periods per day, except for clock-change days:",
        "",
        "1. **Spring Clock Change (Short Day - 2024-03-31)**:",
        "   - **Expected**: 46 settlement periods (clock advances 01:00 GMT -> 02:00 BST). Settlement period numbers remain consecutive; missing wall-clock times do not create skipped period IDs.",
        "   - **Observed per scheduled dataset**: " + "; ".join(f"{r['cfg']['id']}: {r['spring_sp']}" for r in results if r["cfg"]["expected_sp"]) + ".",
        "   - **Expected Period IDs**: 1 through 46 (total 46 periods).",
        "",
        "2. **Autumn Clock Change (Long Day - 2024-10-27)**:",
        "   - **Expected**: 50 settlement periods (clock falls back 02:00 BST -> 01:00 GMT). Consecutive period numbering distinguishes the repeated local hour.",
        "   - **Observed per scheduled dataset**: " + "; ".join(f"{r['cfg']['id']}: {r['autumn_sp']}" for r in results if r["cfg"]["expected_sp"]) + ".",
        "   - **Expected Period IDs**: 1 through 50 (total 50 periods).",
        "",
        "---",
        "",
        "## Detailed Dataset Coverage & Findings",
        "",
    ])

    for res in results:
        cfg = res["cfg"]
        ds_id = cfg["id"]
        cov_lines.extend([
            f"### {cfg['name']} (`{ds_id}`)",
            f"- **Category**: {cfg['category']}",
            f"- **API Endpoint**: `{cfg['endpoint']}`",
            f"- **Date Range**: {cfg['date_range']}",
            f"- **File Count**: {res['file_count']} files",
            f"- **Total Storage Size**: {res['total_bytes']:,} bytes ({res['total_bytes'] / (1024**2):.2f} MiB)",
            f"- **Total Records**: {res['total_records']:,}",
            f"- **Checksum Manifest**: See index file [`{res['index_file']}`]({res['index_file']}) (SHA-256: `{res['index_sha256']}`)",
            f"- **Description**: {cfg['description']}",
        ])

        if cfg["expected_sp"]:
            cov_lines.extend([
                f"- **Settlement Period Verification**: Expected {cfg['expected_sp']:,}, Actual {res['distinct_sp_count']:,} (**{100 * res['covered_sp_count'] / cfg['expected_sp']:.2f}% SP-union coverage**).",
                f"- **Spring Short Day (2024-03-31)**: {res['spring_sp']} periods.",
                f"- **Autumn Long Day (2024-10-27)**: {res['autumn_sp']} periods.",
                f"- **Standard 48-Period Days**: {res['regular_48_days']} days.",
                f"- **Missing / Unexpected Periods**: {len(res['missing_periods'])} / {len(res['unexpected_periods'])}; missing daily partitions: {len(res['missing_dates'])}.",
            ])
        if res["revisions_seen"] > 0:
            cov_lines.append(f"- **Revisions / Amendments Detected**: {res['revisions_seen']:,} revised or amended records tracked.")

        if res["series_periods"]:
            cov_lines.append("- **Per-Series Period Counts** (17,568 required for a rectangular full-year panel): " + "; ".join(f"`{label}`: {count:,}" for label, count in res["series_periods"].items()) + ".")
        if ds_id == "bmunits":
            cov_lines.append("- **Retrieval Time**: Unknown; not recorded during the original download. Snapshot verified on 2026-11-13.")
            cov_lines.append("- **Snapshot Limitation**: This endpoint supplies the current register, not a historical 2024 register. Preserve both unit identifiers and EIC; missing Elexon IDs and multiple EICs for one BM unit must not create dropped or double-counted units. See the dated independent verification for measured identifier cardinalities.")
        if ds_id == "remit":
            cov_lines.append("- **Unique Identifiers by Type**: " + "; ".join(f"`{kind}`: {count:,}" for kind, count in res["remit_identifiers"].items()) + f". The union contains {res['remit_identifier_union']:,} identifiers, not all outage identifiers; type-specific sets can overlap.")
            cov_lines.append(f"- **Exact Source Duplicates**: {res['exact_duplicates']}; retained in the raw capture. Deduplicate full payloads for analysis without collapsing genuine publications or asset records. `(mrid, revisionNumber)` alone is not a unique message key.")
            cov_lines.append("- **Window Limitation**: Selected by publication time in 2024, not by event overlap with 2024; earlier-published outages may be absent. Monthly file presence cannot prove every source message was captured.")
        if ds_id == "boalf":
            cov_lines.append("- **Absent Endpoint-Labelled Periods**: " + ", ".join(f"`{day}/{period}`" for day, period in res["missing_periods"]) + ". Labels alone cannot establish absence of balancing activity. See the dated independent verification for UTC-interval overlap analysis and targeted API probes.")

        cov_lines.append("")

    cov_lines.extend([
        "---",
        "",
        "## Data Integrity & Known Traps Observed",
        "",
        "1. **Clock Change Handling**:",
        "   - Short-day period IDs are 1–46; long-day IDs are 1–50. Stream queries that accept period bounds request 1–50. Independent validation checks the exact period sets, not just their counts.",
        "",
        "2. **Market Index Data (MID) Labeling**:",
        "   - Market Index Data consists of half-hourly volume-weighted average prices from APX (EPEX SPOT) and N2EX (Nord Pool) power exchanges.",
        "   - In accordance with market requirements, this series is strictly labelled as **Market Index Price**, NOT day-ahead auction price.",
        "",
        "3. **Physical Units and Notifications (BOD, PN, MEL, MIL)**:",
        "   - BOD, PN, MEL and MIL are stored as complete gzip JSON arrays. The independent audit parses every array and checks compressed-file SHA-256 values. Notification segments and amendments are not independent generating units, and nullable Elexon IDs must not replace the National Grid IDs in uniqueness keys.",
        "",
        "4. **Gaps and Duplicates Are Explicit**:",
        f"   - Per-series full-year absences: {dimension_gaps or 'none detected'}. Sparse settlement-period unions: {sparse_unions or 'none'}. REMIT exact duplicates: {remit_result['exact_duplicates']}. File presence is not a no-gaps guarantee; missing data is not automatically zero.",
        "   - Full-year unit-by-period completeness cannot be inferred for changing, event-driven BM notification datasets without a historical unit roster and source notification history. No such roster was collected here.",
        "",
        "5. **Licence Verification**:",
        f"   - Source: [Licence to use BMRS open data]({LICENCE_URL}), not the separate BSC Open Data Licence. The required Elexon attribution statements appear above and in the manifest. Commercial reuse is permitted for covered rights; personal data and third-party rights not licensed by Elexon remain excluded.",
        "",
    ])

    cov_path = OUTPUT_DIR / "COVERAGE.md"
    cov_path.write_text("\n".join(cov_lines) + "\n")
    print(f"Wrote {cov_path}")

if __name__ == "__main__":
    main()
