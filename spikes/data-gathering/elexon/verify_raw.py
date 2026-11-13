import argparse
import collections
import concurrent.futures
import datetime
import hashlib
import json
from pathlib import Path
import sys
import os
import resource
import tomllib
from zoneinfo import ZoneInfo


ROOT = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT))
from stream_rows import rows_from
LONDON = ZoneInfo("Europe/London")
UTC = datetime.timezone.utc
START = datetime.date(2024, 1, 1)
DATES = [(START + datetime.timedelta(days=offset)).isoformat() for offset in range(366)]
PERIODS = {
    day: 46 if day == "2024-03-31" else 50 if day == "2024-10-27" else 48
    for day in DATES
}
EXPECTED = {(day, period) for day in DATES for period in range(1, PERIODS[day] + 1)}


def frozen(value):
    if isinstance(value, dict):
        return tuple((key, frozen(item)) for key, item in sorted(value.items()))
    if isinstance(value, list):
        return tuple(map(frozen, value))
    return value


def audit_dataset(arguments):
    dataset, config, raw = arguments
    folder = Path(raw) / dataset
    files = sorted(folder.iterdir())
    index = ROOT / config["sha256_index_file"]
    expected_hashes = {}
    index_duplicates = []
    for line in index.read_text().splitlines():
        digest, filename = line.split(maxsplit=1)
        filename = filename.strip().removeprefix("*")
        if filename in expected_hashes:
            index_duplicates.append(filename)
        expected_hashes[filename] = digest
    result = {
        "dataset": dataset,
        "files": len(files),
        "bytes": sum(path.stat().st_size for path in files),
        "index_digest_matches": hashlib.sha256(index.read_bytes()).hexdigest()
        == config["sha256_index_digest"],
        "index_duplicate_names": index_duplicates,
        "unlisted_files": sorted({path.name for path in files} - expected_hashes.keys()),
        "missing_files": sorted(expected_hashes.keys() - {path.name for path in files}),
        "hash_mismatches": [],
        "records": 0,
        "exact_duplicates": 0,
        "key_duplicates": 0,
        "duplicate_examples": [],
        "wrong_partition": 0,
        "revisions_gt_one": 0,
        "claimed_records": config["total_records"],
        "amendments": 0,
        "numeric_ranges": {},
        "reference_cardinalities": {},
    }
    periods = set()
    interval_periods = set()
    panel = collections.Counter()
    remit_ids = set()
    remit_keys = set()
    remit_revisions = set()
    remit_types = collections.defaultdict(set)
    global_reference_keys = set()
    reference_values = collections.defaultdict(set)
    reference_nulls = collections.Counter()
    remit_exact = set()
    publication_min = None
    publication_max = None
    row_counts = {}
    interval_cache = {}
    for path in files:
        with path.open("rb") as source:
            digest = hashlib.file_digest(source, "sha256").hexdigest()
        if expected_hashes.get(path.name) != digest:
            result["hash_mismatches"].append(path.name)
        row_counts[path.name] = 0
        exact_seen = remit_exact if dataset == "remit" else set()
        interval_cache.clear()
        keys_seen = set()
        partition = path.name.split(".")[0]
        for row in rows_from(path):
            result["records"] += 1
            row_counts[path.name] += 1
            exact_key = frozen(row) if dataset == "remit" else tuple(sorted(row.items()))
            if exact_key in exact_seen:
                result["exact_duplicates"] += 1
            if exact_key in exact_seen and len(result["duplicate_examples"]) < 3:
                result["duplicate_examples"].append({"file": path.name, "identifier": {field: row.get(field) for field in ("mrid", "revisionNumber", "publishTime", "assetId")}})
            exact_seen.add(exact_key)
            result["amendments"] += row.get("amendmentFlag") not in (None, False, "ORI")
            for field in ("systemBuyPrice", "systemSellPrice") if dataset == "system_prices" else ():
                value = row[field]
                bounds = result["numeric_ranges"].setdefault(field, [value, value])
                bounds[:] = [min(bounds[0], value), max(bounds[1], value)]
            if dataset == "bmunits":
                for field in ("elexonBmUnit", "nationalGridBmUnit", "eic"):
                    reference_nulls[field] += row[field] is None
                    if row[field] is not None:
                        reference_values[field].add(row[field])
            day = row.get("settlementDate")
            period = row.get("settlementPeriod")
            if day and day != partition:
                result["wrong_partition"] += 1
            if day and period is not None:
                periods.add((day, period))
            if dataset == "boalf":
                periods.update((day, row[field]) for field in ("settlementPeriodFrom", "settlementPeriodTo"))
                start = row["timeFrom"]
                end = row["timeTo"]
                interval_key = (start, end)
                if interval_key not in interval_cache:
                    instant = datetime.datetime.fromisoformat(start)
                    finish = datetime.datetime.fromisoformat(end)
                    covered = set()
                    while instant < finish:
                        local = instant.astimezone(LONDON)
                        local_day = local.date()
                        midnight = datetime.datetime.combine(local_day, datetime.time(), LONDON).astimezone(UTC)
                        settlement_period = int((instant - midnight).total_seconds() // 1800) + 1
                        covered.add((local_day.isoformat(), settlement_period))
                        next_boundary = midnight + datetime.timedelta(minutes=30 * settlement_period)
                        instant = min(finish, next_boundary)
                    interval_cache[interval_key] = covered
                interval_periods.update(interval_cache[interval_key])
                key = tuple(row.get(field) for field in (
                    "bmUnit", "nationalGridBmUnit", "acceptanceNumber", "acceptanceTime", "timeFrom", "timeTo", "amendmentFlag"
                ))
            elif dataset == "remit":
                remit_ids.add(row["mrid"])
                remit_revisions.add((row["mrid"], row["revisionNumber"]))
                remit_types[row["messageType"]].add(row["mrid"])
                key = tuple(row.get(field) for field in (
                    "mrid", "revisionNumber", "publishTime", "createdTime", "assetId"
                ))
                if key in remit_keys:
                    result["key_duplicates"] += 1
                remit_keys.add(key)
                result["wrong_partition"] += not row["publishTime"].startswith(partition)
                result["revisions_gt_one"] += row["revisionNumber"] > 1
                publication_min = min(publication_min or row["publishTime"], row["publishTime"])
                publication_max = max(publication_max or row["publishTime"], row["publishTime"])
            elif dataset == "bmunits":
                key = tuple(row.get(field) for field in ("elexonBmUnit", "nationalGridBmUnit", "eic"))
                if key in global_reference_keys:
                    result["key_duplicates"] += 1
                global_reference_keys.add(key)
            elif dataset in {"fuelhh", "interconnectors", "mid"}:
                category = row[{"fuelhh": "fuelType", "interconnectors": "interconnectorName", "mid": "dataProvider"}[dataset]]
                key = (day, period, category)
                panel[(day, period, category)] += 1
            elif dataset in {"mels", "mils", "pn", "bod"}:
                fields = ["settlementDate", "settlementPeriod", "bmUnit", "nationalGridBmUnit", "timeFrom", "timeTo"]
                if dataset in {"mels", "mils"}:
                    fields.extend(["notificationTime", "notificationSequence"])
                if dataset == "bod":
                    fields.append("pairId")
                key = tuple(row.get(field) for field in fields)
            else:
                key = (day, period)
            if dataset not in {"remit", "bmunits"} and key in keys_seen:
                result["key_duplicates"] += 1
                if len(result["duplicate_examples"]) < 3:
                    result["duplicate_examples"].append(key)
            keys_seen.add(key)
        del exact_seen, keys_seen
        if len(row_counts) % 60 == 0:
            print(f"{dataset}: {len(row_counts)}/{len(files)} files", file=sys.stderr, flush=True)
    result["settlement_periods"] = len(periods)
    result["missing_periods"] = sorted(EXPECTED - periods) if dataset not in {"bmunits", "remit"} else []
    result["unexpected_periods"] = sorted(periods - EXPECTED)
    result["clock_changes"] = {
        day: sorted(period for date, period in periods if date == day)
        for day in ("2024-03-31", "2024-10-27")
    }
    result["regular_48_days"] = sum(sum(date == day for date, _ in periods) == 48 for day in DATES)
    result["max_rss_kib"] = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    result["reference_cardinalities"] = {
        field: {"nonnull_unique": len(values), "null_rows": reference_nulls[field]}
        for field, values in reference_values.items()
    }
    result["daily_record_range"] = [min(row_counts.values()), max(row_counts.values())]
    if dataset not in {"bmunits", "remit"}:
        result["missing_dates"] = sorted(set(DATES) - {path.name[:10] for path in files})
    if dataset == "boalf":
        result["interval_covered_periods"] = len(interval_periods & EXPECTED)
        result["interval_missing_periods"] = sorted(EXPECTED - interval_periods)
        result["interval_outside_year"] = sorted(interval_periods - EXPECTED)
    if panel:
        result["panels"] = {}
        categories = sorted({key[2] for key in panel})
        for category in categories:
            present = {(day, period) for day, period, label in panel if label == category}
            missing = sorted(EXPECTED - present)
            result["panels"][category] = {
                "periods": len(present),
                "missing_count": len(missing),
                "missing_examples": missing[:25],
                "missing_within_observed_lifetime": sum(min(present) <= item <= max(present) for item in missing),
                "first": min(present),
                "last": max(present),
            }
    if dataset == "remit":
        result["unique_identifiers"] = len(remit_ids)
        result["unique_message_keys"] = len(remit_keys)
        result["unique_mrid_revision_pairs"] = len(remit_revisions)
        result["unique_identifiers_by_message_type"] = {
            kind: len(identifiers) for kind, identifiers in sorted(remit_types.items())
        }
        result["publication_range"] = [publication_min, publication_max]
    print(f"{dataset}: {result['records']:,} rows, {result['exact_duplicates']} exact duplicates, {len(result['missing_periods'])} absent periods", flush=True)
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--raw", default=os.environ.get("DATA_DIR", str(Path.home() / "energy-data/raw/elexon")))
    parser.add_argument("--workers", type=int, default=2)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--datasets", nargs="+")
    args = parser.parse_args()
    manifest = tomllib.loads((ROOT / "manifest.toml").read_text())
    jobs = [
        (dataset, config, args.raw) for dataset, config in manifest["datasets"].items()
        if args.datasets is None or dataset in args.datasets
    ]
    if args.datasets and set(args.datasets) - manifest["datasets"].keys():
        parser.error("unknown dataset")
    with concurrent.futures.ProcessPoolExecutor(max_workers=args.workers) as executor:
        results = list(executor.map(audit_dataset, jobs))
    args.output.write_text(json.dumps(results, indent=2, sort_keys=True) + "\n")
    return int(any(
        result["hash_mismatches"] or result["missing_files"] or result["unlisted_files"]
        or result["index_duplicate_names"] or not result["index_digest_matches"]
        or result["wrong_partition"] or result["unexpected_periods"]
        or result["records"] != result["claimed_records"]
        or result["exact_duplicates"] or result["key_duplicates"]
        or result["missing_periods"] or result.get("missing_dates")
        or any(panel["missing_within_observed_lifetime"] for panel in result.get("panels", {}).values())
        for result in results
    ))


if __name__ == "__main__":
    sys.exit(main())
