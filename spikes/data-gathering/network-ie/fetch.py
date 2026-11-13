#!/usr/bin/env python3
"""
Data gathering script for GB & Ireland grid simulation data.
Retrieves datasets from NESO, DNOs (NGED, SSEN), EirGrid/SONI, and SEMO/SEMOpx.
Stores raw data files outside the git working tree in /Users/njotob/energy-data/raw/<source>/.
Emits manifest.toml containing URL, retrieval date, size in bytes, SHA-256 hash, and licence info.
"""

from __future__ import annotations

import argparse
import hashlib
import os
import sys
import time
import urllib.error
import urllib.request
from datetime import datetime, timezone
from pathlib import Path
from typing import Dict, List, Optional, Tuple

RAW_DATA_DIR = Path("/Users/njotob/energy-data/raw")
SCRIPT_DIR = Path(__file__).resolve().parent
MANIFEST_PATH = SCRIPT_DIR / "manifest.toml"

DATASETS: List[Dict[str, str]] = [
    # --- NESO GB Transmission Datasets ---
    {
        "category": "neso",
        "name": "etys-2025-appendix-b-system-technical-data",
        "filename": "etys-2025-appendix-b-system-technical-data.xlsx",
        "url": "https://www.neso.energy/document/383936/download",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NESO ETYS 2025 Appendix B: System technical data workbook (busbars, circuits, transformers)",
    },
    {
        "category": "neso",
        "name": "etys-2025-boundary-chart-data",
        "filename": "etys-2025-boundary-chart-data.xlsx",
        "url": "https://www.neso.energy/document/383896/download",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NESO ETYS 2025 Boundary capability chart data and boundary transfer limits",
    },
    {
        "category": "neso",
        "name": "etys-2025-appendix-a-system-schematics",
        "filename": "etys-2025-appendix-a-system-schematics.pdf",
        "url": "https://www.neso.energy/document/383931/download",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NESO ETYS 2025 Appendix A: System schematics and geographic drawings",
    },
    {
        "category": "neso",
        "name": "etys-2025-appendix-c-power-flows",
        "filename": "etys-2025-appendix-c-power-flows.pdf",
        "url": "https://www.neso.energy/document/383946/download",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NESO ETYS 2025 Appendix C: System power flow diagrams",
    },
    {
        "category": "neso",
        "name": "etys-2025-appendix-d-fault-peak",
        "filename": "etys-2025-appendix-d-fault-peak.xlsx",
        "url": "https://www.neso.energy/document/383951/download",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NESO ETYS 2025 Appendix D: Fault levels peak data",
    },
    {
        "category": "neso",
        "name": "etys-2025-appendix-d-fault-min",
        "filename": "etys-2025-appendix-d-fault-min.xlsx",
        "url": "https://www.neso.energy/document/383961/download",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NESO ETYS 2025 Appendix D: Fault levels minimum data",
    },
    {
        "category": "neso",
        "name": "etys-2025-appendix-f-contracted-generation",
        "filename": "etys-2025-appendix-f-contracted-generation.xlsx",
        "url": "https://www.neso.energy/document/379146/download",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NESO ETYS 2025 Appendix F: Contracted generation background",
    },
    {
        "category": "neso",
        "name": "etys-2025-appendix-g-nodal-demand",
        "filename": "etys-2025-appendix-g-nodal-demand.xlsx",
        "url": "https://www.neso.energy/document/379136/download",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NESO ETYS 2025 Appendix G: FES Nodal demand data",
    },
    {
        "category": "neso",
        "name": "etys-boundary-gis-data-mar25",
        "filename": "etys-boundary-gis-data-mar25.zip",
        "url": "https://api.neso.energy/dataset/997f4820-1ad4-499b-b1fe-4b8d3d7fbc72/resource/e914fcec-1dc9-4f1f-97e7-59c0d9521bea/download/etys-boundary-gis-data-mar25.zip",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NESO ETYS GB Transmission System Boundary GIS shapefiles/geodata",
    },
    {
        "category": "neso",
        "name": "tec-register",
        "filename": "tec-register-10-november-2026.csv",
        "url": "https://api.neso.energy/dataset/cbd45e54-e6e2-4a38-99f1-8de6fd96d7c1/resource/17becbab-e3e8-473f-b303-3806f43a6a10/download/tec-register-10-november-2026.csv",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NESO Transmission Entry Capacity (TEC) Register of generators connected to GB transmission",
    },
    {
        "category": "neso",
        "name": "embedded-capacity-register",
        "filename": "embedded-register-10-november-2026.csv",
        "url": "https://api.neso.energy/dataset/4b0e109c-aa23-45ac-86c1-0bda557d1aab/resource/68b6f3a1-e1bf-403b-9062-0269fc758d77/download/embedded-register-10-november-2026.csv",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NESO GB Embedded Capacity Register for generation connected to distribution networks",
    },
    {
        "category": "neso",
        "name": "24-months-ahead-constraint-limits",
        "filename": "24-months-ahead-constraint-limit_nov26.csv",
        "url": "https://api.neso.energy/dataset/d515b4a9-60a1-489c-a126-004efc04f121/resource/3c359e33-3dac-4bdd-87d1-efbf4cbc2f07/download/24-months-ahead-constraint-limit_nov26.csv",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NESO 24-months ahead boundary constraint limits and capability forecast",
    },
    {
        "category": "neso",
        "name": "day-ahead-constraint-flows-and-limits",
        "filename": "day-ahead-constraints-limits-and-flow-output-v1.5.csv",
        "url": "https://api.neso.energy/dataset/cf3cbc92-2d5d-4c2b-bd29-e11a21070b26/resource/38a18ec1-9e40-465d-93fb-301e80fd1352/download/day-ahead-constraints-limits-and-flow-output-v1.5.csv",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NESO Day-ahead transmission boundary constraint flows and limits",
    },
    {
        "category": "neso",
        "name": "outturn-system-costs-constraint",
        "filename": "outturn-system-costs-2024-2025.csv",
        "url": "https://api.neso.energy/dataset/f0055054-c55c-4068-a01c-61da4334e58f/resource/27df75fd-6233-466c-beed-3e6e2261e6b1/download/outturn-system-costs-2024-2025.csv",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NESO Outturn System Costs (thermal and constraint costs 2024-2025)",
    },

    # --- DNO 132 kV LTDS Datasets (Direct Unauthenticated Downloads) ---
    {
        "category": "dno",
        "name": "nged-ltds-table-1-circuit",
        "filename": "ltds-table-1-circuit.csv",
        "url": "https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/7892e22f-e489-49fa-a582-b1384006e4e2/download/ltds-table-1-circuit.csv",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NGED LTDS Table 1: Circuit parameters (132 kV, 66 kV, 33 kV) across East Midlands, West Midlands, South Wales, South West",
    },
    {
        "category": "dno",
        "name": "nged-ltds-table-2a-transformer",
        "filename": "ltds-table-2a-two-winding-transformer.csv",
        "url": "https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/8c964d61-a7fb-41e7-99b4-6032d063a8c6/download/ltds-table-2a-two-winding-transformer.csv",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NGED LTDS Table 2a: Two-winding transformer parameters across all 4 NGED areas",
    },
    {
        "category": "dno",
        "name": "nged-ltds-table-2b-transformer",
        "filename": "ltds-table-2b-three-winding-transformer.csv",
        "url": "https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/56033622-8ed7-4425-916c-82a030df4822/download/ltds-table-2b-three-winding-transformer.csv",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NGED LTDS Table 2b: Three-winding transformer parameters across all 4 NGED areas",
    },
    {
        "category": "dno",
        "name": "nged-ltds-attachment-8-node",
        "filename": "ltds-attachment-8-node.csv",
        "url": "https://connecteddata.nationalgrid.co.uk/dataset/5c3c9ca0-5c57-4cdc-99be-f31e38d1e825/resource/a92da9e4-2010-4406-b86a-825cd65761d6/download/ltds-attachment-8-node.csv",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NGED LTDS Attachment 8: Busbar/node definitions across all 4 NGED areas",
    },
    {
        "category": "dno",
        "name": "nged-cim-east-midlands",
        "filename": "nged-cim-emids.zip",
        "url": "https://connecteddata.nationalgrid.co.uk/dataset/7367f8a5-7714-4647-84db-dc35119bea78/resource/2ef4b5ac-3e57-4f84-a376-9014c146c1de/download/emids.zip",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NGED LTDS CIM Model XML archive for East Midlands licence area",
    },
    {
        "category": "dno",
        "name": "nged-cim-west-midlands",
        "filename": "nged-cim-wmids.zip",
        "url": "https://connecteddata.nationalgrid.co.uk/dataset/7367f8a5-7714-4647-84db-dc35119bea78/resource/ab83d544-7a2a-4801-9d2a-4dbec5c8efbf/download/wmids.zip",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NGED LTDS CIM Model XML archive for West Midlands licence area",
    },
    {
        "category": "dno",
        "name": "nged-cim-south-wales",
        "filename": "nged-cim-swales.zip",
        "url": "https://connecteddata.nationalgrid.co.uk/dataset/7367f8a5-7714-4647-84db-dc35119bea78/resource/46575bac-99cf-4edd-933e-9df58c2b95e0/download/swales.zip",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NGED LTDS CIM Model XML archive for South Wales licence area",
    },
    {
        "category": "dno",
        "name": "nged-cim-south-west",
        "filename": "nged-cim-swest.zip",
        "url": "https://connecteddata.nationalgrid.co.uk/dataset/7367f8a5-7714-4647-84db-dc35119bea78/resource/38dbec57-e1ff-48c3-b4dc-c9759ebed3c0/download/swest.zip",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "NGED LTDS CIM Model XML archive for South West licence area",
    },
    {
        "category": "dno",
        "name": "ssen-sepd-ltds-tables-may-2026",
        "filename": "southern-electric-power-distribution-long-term-development-statement-tables_may-2026.xlsx",
        "url": "https://data-api.ssen.co.uk/dataset/695078a0-bbd8-420b-8383-ee6bf7fc0045/resource/02c9431f-6126-4384-9f46-9079dbcd7057/download/southern-electric-power-distribution-long-term-development-statement-tables_may-2026.xlsx",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "SSEN SEPD (Southern Electric) LTDS Tables 1-7 including 132 kV circuit parameters",
    },
    {
        "category": "dno",
        "name": "ssen-sepd-ltds-cim-eq",
        "filename": "ltds_sepd_2025-02_eq_2025-10-20_v1.0.zip",
        "url": "https://data-api.ssen.co.uk/dataset/695078a0-bbd8-420b-8383-ee6bf7fc0045/resource/d56de5a2-4150-48e9-986a-7c315f570a7b/download/ltds_sepd_2025-02_eq_2025-10-20_v1.0.zip",
        "licence_name": "Open Government Licence v3.0",
        "licence_status": "open-commercial",
        "description": "SSEN SEPD (Southern Electric) CIM Equipment (EQ) profile XML archive",
    },

    # --- EirGrid / SONI All-Island Transmission Datasets ---
    {
        "category": "eirgrid",
        "name": "eirgrid-tytfs-2024-study-files",
        "filename": "TYTFS2024_studyfiles.zip",
        "url": "https://cms.eirgrid.ie/sites/default/files/publications/TYTFS2024_studyfiles.zip",
        "licence_name": "EirGrid / SONI Network Study Models",
        "licence_status": "research-only",
        "description": "EirGrid/SONI All-Island TYTFS 2024 Study Files (PSS/E RAW network models and SEQ sequence data)",
    },
    {
        "category": "eirgrid",
        "name": "eirgrid-system-data-qtr-hourly-2024",
        "filename": "System-Data-Qtr-Hourly-2024.xlsx",
        "url": "https://cms.eirgrid.ie/sites/default/files/publications/System-Data-Qtr-Hourly-2024.xlsx",
        "licence_name": "EirGrid / SONI System Operational Data",
        "licence_status": "research-only",
        "description": "EirGrid/SONI 2024 Quarter-Hourly operational SCADA data (IEDemand, NIDemand, wind, solar, EWIC, Moyle, SNSP)",
    },
    {
        "category": "eirgrid",
        "name": "eirgrid-system-renewable-summary-report",
        "filename": "System-and-Renewable-Data-Summary-Report-V30.xlsx",
        "url": "https://cms.eirgrid.ie/sites/default/files/publications/System-and-Renewable-Data-Summary-Report-V30.xlsx",
        "licence_name": "EirGrid / SONI System Operational Data",
        "licence_status": "research-only",
        "description": "EirGrid/SONI System & Renewable monthly/annual summary metrics workbook",
    },

    # --- SEMO / SEMOpx Market Results ---
    {
        "category": "semo",
        "name": "semopx-market-lookback",
        "filename": "lookback_mkt.xlsx",
        "url": "https://www.semopx.com/documents/general-publications/lookback_mkt.xlsx",
        "licence_name": "SEMOpx Market Data Publication",
        "licence_status": "research-only",
        "description": "SEMOpx Historical Market Results Lookback: Day-Ahead (DAM) and Intraday (IDM) prices, volumes, and reference metrics",
    },
    {
        "category": "semo",
        "name": "semo-pub-monthly-registered-capacity",
        "filename": "PUB_MnlyRegisteredCapacity_202412011000.xml",
        "url": "https://reports.sem-o.com/documents/PUB_MnlyRegisteredCapacity_202412011000.xml",
        "licence_name": "SEMO Market Data Publication",
        "licence_status": "research-only",
        "description": "SEMO Balancing Market Monthly Registered Capacity report (December 2024 all-island generator capacities)",
    },
    {
        "category": "semo",
        "name": "semo-pub-daily-registered-units",
        "filename": "PUB_DailyRegisteredUnits_202609140734.xml",
        "url": "https://reports.sem-o.com/documents/PUB_DailyRegisteredUnits_202609140734.xml",
        "licence_name": "SEMO Market Data Publication",
        "licence_status": "research-only",
        "description": "SEMO Balancing Market Registered Units master reference report",
    },
]


def download_file(
    url: str,
    target_path: Path,
    max_retries: int = 3,
    chunk_size: int = 65536,
    pause_sec: float = 1.0,
) -> Tuple[int, str, str]:
    target_path.parent.mkdir(parents=True, exist_ok=True)
    temp_path = target_path.with_suffix(target_path.suffix + ".part")

    headers = {
        "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko)"
    }

    if target_path.exists() and target_path.stat().st_size > 0:
        h = hashlib.sha256()
        with open(target_path, "rb") as f:
            while chunk := f.read(chunk_size):
                h.update(chunk)
        mtime = datetime.fromtimestamp(target_path.stat().st_mtime, tz=timezone.utc).isoformat()
        return target_path.stat().st_size, h.hexdigest(), mtime

    retries = 0
    while retries < max_retries:
        try:
            req = urllib.request.Request(url, headers=headers)
            with urllib.request.urlopen(req, timeout=60) as resp:
                h = hashlib.sha256()
                total_bytes = 0
                with open(temp_path, "wb") as f:
                    while chunk := resp.read(chunk_size):
                        f.write(chunk)
                        h.update(chunk)
                        total_bytes += len(chunk)
            temp_path.replace(target_path)
            retrieval_iso = datetime.now(timezone.utc).isoformat()
            time.sleep(pause_sec)
            return total_bytes, h.hexdigest(), retrieval_iso
        except Exception as e:
            retries += 1
            if temp_path.exists():
                temp_path.unlink()
            if retries >= max_retries:
                raise RuntimeError(f"Failed to download {url} after {max_retries} attempts: {e}")
            wait_time = 2 ** retries
            print(f"  [WARN] Download error for {url}: {e}. Retrying in {wait_time}s...")
            time.sleep(wait_time)

    raise RuntimeError(f"Failed to download {url}")


def generate_manifest_toml(
    entries: List[Dict[str, str]],
    manifest_path: Path,
) -> None:
    now_iso = datetime.now(timezone.utc).strftime('%Y-%m-%d %H:%M:%SZ')
    lines = [
        "# Data Gathering Manifest",
        "# GB & Ireland Electricity Grid Network and Market Datasets",
        f"# Generated: {now_iso}",
        "# Storage location: /Users/njotob/energy-data/raw/<category>/<filename>",
        "",
        "[meta]",
        'repository = "Tobechukwu-Njoku/energy-grid-simulator"',
        'storage_root = "/Users/njotob/energy-data/raw"',
        f"dataset_count = {len(entries)}",
        "",
    ]

    for entry in entries:
        lines.extend([
            "[[datasets]]",
            f'category = "{entry["category"]}"',
            f'name = "{entry["name"]}"',
            f'filename = "{entry["filename"]}"',
            f'relative_path = "{entry["category"]}/{entry["filename"]}"',
            f'source_url = "{entry["url"]}"',
            f'retrieval_date = "{entry["retrieval_date"]}"',
            f'bytes = {entry["bytes"]}',
            f'sha256 = "{entry["sha256"]}"',
            f'licence_name = "{entry["licence_name"]}"',
            f'licence_status = "{entry["licence_status"]}"',
            f'description = "{entry["description"]}"',
            "",
        ])

    with open(manifest_path, "w", encoding="utf-8") as f:
        f.write(chr(10).join(lines) + chr(10))


def main() -> int:
    parser = argparse.ArgumentParser(description="Fetch GB & Ireland energy datasets.")
    parser.add_argument("--dry-run", action="store_true", help="Print datasets without downloading.")
    args = parser.parse_args()

    print(f"Target raw data root: {RAW_DATA_DIR}")
    print(f"Manifest output: {MANIFEST_PATH}")
    print(f"Total datasets defined: {len(DATASETS)}")

    if args.dry_run:
        for ds in DATASETS:
            print(f"- [{ds['category']}] {ds['name']} -> {ds['url']}")
        return 0

    results: List[Dict[str, str]] = []
    total_volume = 0

    for idx, ds in enumerate(DATASETS, 1):
        target = RAW_DATA_DIR / ds["category"] / ds["filename"]
        print(f"[{idx}/{len(DATASETS)}] Fetching {ds['category']}/{ds['filename']}...")
        try:
            bytes_dl, sha256_hex, ret_iso = download_file(ds["url"], target)
            total_volume += bytes_dl
            entry = dict(ds)
            entry["bytes"] = str(bytes_dl)
            entry["sha256"] = sha256_hex
            entry["retrieval_date"] = ret_iso
            results.append(entry)
            print(f"  OK ({bytes_dl:,} bytes, sha256: {sha256_hex[:12]}...)")
        except Exception as e:
            print(f"  ERROR: {e}")
            return 1

    generate_manifest_toml(results, MANIFEST_PATH)
    print(chr(10) + f"Manifest successfully written to {MANIFEST_PATH}")
    print(f"Total data gathered: {total_volume:,} bytes ({total_volume / (1024*1024):.2f} MB)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
