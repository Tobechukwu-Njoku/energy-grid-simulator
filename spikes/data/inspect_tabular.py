import collections
import csv
import hashlib
import json
from pathlib import Path
import xml.etree.ElementTree as ET

import openpyxl


ROOT = Path(__file__).resolve().parent
RAW = ROOT / "raw"
SAMPLES = ROOT / "samples"


def rows_for(name):
    with (RAW / name).open(encoding="utf-8-sig", errors="replace", newline="") as stream:
        return list(csv.DictReader(stream))


def csv_sample(name, rows):
    with (SAMPLES / name).open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]), lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows[:3])


demand = rows_for("demand-2024.csv")
fuel = [row for row in rows_for("generation-mix.csv") if row["DATETIME"].startswith("2024-")]
day_sizes = collections.Counter(row["SETTLEMENT_DATE"] for row in demand)
result = {"demand_2024": {"rows": len(demand), "days": len(day_sizes),
                         "day_period_counts": dict(collections.Counter(day_sizes.values())),
                         "non_48_period_dates": {date: count for date, count in day_sizes.items() if count != 48},
                         "duplicate_keys": len(demand) - len({(row["SETTLEMENT_DATE"], row["SETTLEMENT_PERIOD"]) for row in demand}),
                         "missing_cells": sum(value == "" for row in demand for value in row.values())},
          "generation_2024": {"rows": len(fuel), "first": fuel[0]["DATETIME"], "last": fuel[-1]["DATETIME"],
                              "duplicate_timestamps": len(fuel) - len({row["DATETIME"] for row in fuel}),
                              "missing_cells": sum(value == "" for row in fuel for value in row.values())}}
assert len(demand) == len(fuel) == 17568
csv_sample("demand-2024.csv", demand)
csv_sample("generation-2024.csv", fuel)
for name, key in [("tec-register.csv", "Project Status"), ("repd.csv", "Development Status"),
                  ("nged-circuits.csv", "Circuit Status"), ("nged-nodes.csv", "Status"),
                  ("nged-transformers.csv", "Status")]:
    rows = rows_for(name)
    result[name] = {"rows": len(rows), "status_counts": dict(collections.Counter(row[key].strip() for row in rows))}
    if name in {"tec-register.csv", "nged-circuits.csv", "nged-nodes.csv", "nged-transformers.csv"}:
        csv_sample(name, rows)
    if name == "nged-circuits.csv":
        target = [row for row in rows if row["Operating Voltage (kV)"] == "132"]
        csv_sample(name, [row for row in target if row["Equipment"] == "Circuit"])
        result[name]["132kv_rows_including_changes"] = len(target)
        result[name]["132kv_equipment_counts"] = dict(collections.Counter(row["Equipment"] for row in target))
        result[name]["132kv_numeric_r_x_b_and_winter_summer_ratings"] = sum(all(row[key] not in {"", "-"} for key in ["R1", "X1", "B1", "Winter", "Summer"]) for row in target)
for name in ["bmunits.json", "demand-day-correct.json", "fuel-day.json", "bod-period.json", "boa-period.json", "market-index-day.json"]:
    response = json.loads((RAW / name).read_text())
    rows = response if isinstance(response, list) else response["data"]
    result[name] = {"rows": len(rows)}
    if name != "market-index-day.json":
        (SAMPLES / name).write_text(json.dumps(rows[:2], indent=2) + "\n")
book = openpyxl.load_workbook(RAW / "dukes-5.11.xlsx", read_only=True, data_only=True)
result["dukes_5_11"] = {"full_list_rows_with_numeric_capacity": sum(isinstance(row[7], (int, float)) and bool(row[1]) for row in list(book["5.11 Full list"].values)[6:])}
flow = ET.parse(RAW / "isem-flow-20240101.xml").getroot()
result["isem_scheduled_flow_sample"] = {"interval_records": len(flow.findall(".//{*}Interval")),
                                      "timeseries": len(flow.findall("{*}CapacityTimeSeries"))}
(SAMPLES / "tabular-statistics.json").write_text(json.dumps(result, indent=2) + "\n")
manifest = []
for name in ["etys-2025-appendix-b.xlsx", "tytfs-2024.zip", "tytfs-2024.pdf", "etys-boundaries.zip",
             "demand-2024.csv", "generation-mix.csv", "tec-register.csv", "repd.csv", "dukes-5.11.xlsx",
             "nged-circuits.csv", "nged-nodes.csv", "nged-transformers.csv", "bmunits.json",
             "demand-day-correct.json", "fuel-day.json", "bod-period.json", "boa-period.json", "market-index-day.json",
             "isem-flow-20240101.xml", "neso-licence.html", "neso-terms.html", "eirgrid-disclaimer.html",
             "elexon-data-licence.html", "nged-licence.html", "nged-tabular-page.html", "era5-page.html", "era5-licence.html",
             "era5-catalogue.json", "era5-api-process.json", "ogl.html", "cc-by.html"]:
    path = RAW / name
    manifest.append({"file": name, "bytes": path.stat().st_size, "sha256": hashlib.sha256(path.read_bytes()).hexdigest()})
(SAMPLES / "download-manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
print(json.dumps(result, indent=2))
