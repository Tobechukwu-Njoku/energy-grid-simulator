import collections
import csv
from datetime import datetime, timedelta, timezone
import hashlib
import json
import math
import xml.etree.ElementTree as ET
from zoneinfo import ZoneInfo

import openpyxl

from spike_paths import ROOT, RAW, OUTPUT, SAMPLES, available


OUTPUT.mkdir(parents=True, exist_ok=True)


def rows_for(name):
    with (RAW / name).open(encoding="utf-8-sig", errors="replace", newline="") as stream:
        return list(csv.DictReader(stream))


def csv_sample(name, rows):
    if not rows:
        return
    with (OUTPUT / name).open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]), lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows[:3])


def finite_number(value):
    try:
        return math.isfinite(float(value))
    except (TypeError, ValueError):
        return False


def settlement_date(value):
    try:
        return datetime.fromisoformat(value)
    except ValueError:
        return datetime.strptime(value, "%d-%b-%Y")


def settlement_utc(date, period):
    # Convert local midnight first, then add elapsed half-hours in UTC. Adding
    # them in local time would skip/repeat instants at the DST transitions.
    local_midnight = settlement_date(date).replace(tzinfo=ZoneInfo("Europe/London"))
    start = local_midnight.astimezone(timezone.utc)
    end = (local_midnight + timedelta(days=1)).astimezone(timezone.utc)
    period = int(period)
    if not 1 <= period <= int((end - start).total_seconds() / 1800):
        raise ValueError(f"Invalid settlement period: {date} SP{period}")
    return start + timedelta(minutes=30 * (period - 1))


def generation_utc(value):
    stamp = datetime.fromisoformat(value.replace("Z", "+00:00"))
    return stamp.replace(tzinfo=timezone.utc) if stamp.tzinfo is None else stamp.astimezone(timezone.utc)


result = {}
demand_by_utc = fuel_by_utc = None
if available("demand-2024.csv"):
    demand = rows_for("demand-2024.csv")
    day_sizes = collections.Counter(settlement_date(row["SETTLEMENT_DATE"]).date().isoformat() for row in demand)
    demand_by_utc = {settlement_utc(row["SETTLEMENT_DATE"], row["SETTLEMENT_PERIOD"]): row for row in demand}
    assert len(demand) == 17568 and len(demand_by_utc) == len(demand)
    assert day_sizes["2024-03-31"] == 46 and day_sizes["2024-10-27"] == 50
    result["demand_2024"] = {
        "rows": len(demand), "days": len(day_sizes), "time_basis": "Europe/London settlement day",
        "day_period_counts": dict(collections.Counter(day_sizes.values())),
        "non_48_period_dates": {date: count for date, count in day_sizes.items() if count != 48},
        "duplicate_keys": len(demand) - len(demand_by_utc),
        "missing_cells": sum(value == "" for row in demand for value in row.values())}
    csv_sample("demand-2024.csv", demand)
else:
    result["demand_2024"] = {"status": "not downloaded"}
if available("generation-mix.csv"):
    fuel = [row for row in rows_for("generation-mix.csv") if row["DATETIME"].startswith("2024-")]
    fuel_by_utc = {generation_utc(row["DATETIME"]): row for row in fuel}
    fuel_days = collections.Counter(stamp.date().isoformat() for stamp in fuel_by_utc)
    assert len(fuel) == 17568 and len(fuel_by_utc) == len(fuel)
    assert len(fuel_days) == 366 and set(fuel_days.values()) == {48}
    result["generation_2024"] = {
        "rows": len(fuel), "first": min(fuel_by_utc).isoformat(), "last": max(fuel_by_utc).isoformat(),
        "time_basis": "UTC", "day_period_counts": dict(collections.Counter(fuel_days.values())),
        "duplicate_timestamps": len(fuel) - len(fuel_by_utc),
        "missing_cells": sum(value == "" for row in fuel for value in row.values())}
    csv_sample("generation-2024.csv", fuel)
else:
    result["generation_2024"] = {"status": "not downloaded"}
if demand_by_utc is not None and fuel_by_utc is not None:
    # A timestamp-keyed join, independent of CSV ordering or settlement-day size.
    joined = {stamp: (row, fuel_by_utc[stamp]) for stamp, row in demand_by_utc.items() if stamp in fuel_by_utc}
    assert demand_by_utc.keys() == fuel_by_utc.keys()
    probes = [("2024-03-31", 46, "2024-03-31T22:30:00+00:00"),
              ("2024-04-01", 1, "2024-03-31T23:00:00+00:00"),
              ("2024-10-27", 3, "2024-10-27T00:00:00+00:00"),
              ("2024-10-27", 5, "2024-10-27T01:00:00+00:00")]
    for date, period, expected in probes:
        stamp = settlement_utc(date, period)
        assert stamp.isoformat() == expected
        demand_row, fuel_row = joined[stamp]
        assert (settlement_date(demand_row["SETTLEMENT_DATE"]).date().isoformat(), int(demand_row["SETTLEMENT_PERIOD"])) == (date, period)
        assert generation_utc(fuel_row["DATETIME"]) == stamp
    result["demand_generation_utc_join"] = {"matched_instants": len(joined), "clock_change_probes": len(probes)}
else:
    result["demand_generation_utc_join"] = {"status": "not downloaded"}
for name, key in [("tec-register.csv", "Project Status"), ("repd.csv", "Development Status"),
                  ("nged-circuits.csv", "Circuit Status"), ("nged-nodes.csv", "Status"),
                  ("nged-transformers.csv", "Status"), ("nged-three-winding-transformers.csv", "Status")]:
    if not available(name):
        result[name] = {"status": "not downloaded"}
        continue
    rows = rows_for(name)
    result[name] = {"rows": len(rows), "status_counts": dict(collections.Counter(row[key].strip() for row in rows))}
    if name == "repd.csv":
        result[name]["normalised_status_counts"] = dict(collections.Counter(row[key].strip().casefold() for row in rows))
    if name in {"tec-register.csv", "nged-nodes.csv", "nged-transformers.csv"}:
        csv_sample(name, rows)
    if name == "nged-circuits.csv":
        target = [row for row in rows if row["Operating Voltage (kV)"].strip() == "132"]
        circuits = [row for row in target if row["Equipment"].strip() == "Circuit"]
        groups = {"operational_baseline_unchanged": "Unchanged", "planned_updated": "Updated",
                  "planned_new": "New", "removed": "Removed"}
        assert {row[key].strip() for row in circuits} <= set(groups.values())
        def circuit_counts(records):
            counts = {}
            for label, status in groups.items():
                selected = [row for row in records if row[key].strip() == status]
                counts[label] = {"rows": len(selected), "finite_r_x_b_winter_summer": sum(
                    all(finite_number(row.get(field)) for field in ["R1", "X1", "B1", "Winter", "Summer"])
                    for row in selected)}
            return counts
        csv_sample(name, [row for row in circuits if row[key].strip() == "Unchanged"])
        result[name].update({
            "132kv_rows_including_changes": len(target),
            "132kv_equipment_counts": dict(collections.Counter(row["Equipment"].strip() for row in target)),
            "132kv_circuits_by_status": circuit_counts(circuits),
            "132kv_by_area": {area: {
                "all_equipment_rows": sum(row["Licence"].strip() == area for row in target),
                "circuits_by_status": circuit_counts([row for row in circuits if row["Licence"].strip() == area])}
                for area in sorted({row["Licence"].strip() for row in target})}})
for name in ["bmunits.json", "demand-day-correct.json", "fuel-day.json", "bod-period.json", "boa-period.json", "market-index-day.json"]:
    if not available(name):
        result[name] = {"status": "not downloaded"}
        continue
    response = json.loads((RAW / name).read_text())
    rows = response if isinstance(response, list) else response["data"]
    result[name] = {"rows": len(rows)}
    if name != "market-index-day.json":
        (OUTPUT / name).write_text(json.dumps(rows[:2], indent=2) + "\n")
if available("dukes-5.11.xlsx"):
    book = openpyxl.load_workbook(RAW / "dukes-5.11.xlsx", read_only=True, data_only=True)
    result["dukes_5_11"] = {"full_list_rows_with_numeric_capacity": sum(isinstance(row[7], (int, float)) and bool(row[1]) for row in list(book["5.11 Full list"].values)[6:])}
    book.close()
else:
    result["dukes_5_11"] = {"status": "not downloaded"}
if available("isem-flow-20240101.xml"):
    flow = ET.parse(RAW / "isem-flow-20240101.xml").getroot()
    result["isem_scheduled_flow_sample"] = {"interval_records": len(flow.findall(".//{*}Interval")),
                                          "timeseries": len(flow.findall("{*}CapacityTimeSeries"))}
else:
    result["isem_scheduled_flow_sample"] = {"status": "not downloaded"}
(OUTPUT / "tabular-statistics.json").write_text(json.dumps(result, indent=2) + "\n")
# Preserve both committed pins and prior output entries when a source is absent.
manifest = {entry["file"]: entry for entry in json.loads((SAMPLES / "download-manifest.json").read_text())}
if (OUTPUT / "download-manifest.json").exists():
    manifest.update({entry["file"]: entry for entry in json.loads((OUTPUT / "download-manifest.json").read_text())})
with (ROOT / "sources.tsv").open() as stream:
    for source in csv.reader(stream, delimiter="\t"):
        if not source or source[0].startswith("#"):
            continue
        clearance, name, url = source
        # Metadata is evidence, not a dataset licence grant. D-56 dataset tags
        # remain open-commercial/research-only; do not misclassify public deeds.
        role = "metadata" if clearance == "metadata" else "dataset"
        status = "public-metadata" if role == "metadata" else clearance
        path = RAW / name
        if not path.is_file():
            if name in manifest:
                manifest[name].update(licence_status=status, source_role=role)
            continue
        manifest[name] = {"file": name, "bytes": path.stat().st_size,
                          "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
                          "source_url": url, "licence_status": status, "source_role": role}
(OUTPUT / "download-manifest.json").write_text(json.dumps(list(manifest.values()), indent=2) + "\n")
print(json.dumps(result, indent=2))
