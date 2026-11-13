import gzip
import json
from pathlib import Path
import sys


def main():
    path = Path(sys.argv[1])
    compressed = sys.argv[2] == "true"
    try:
        with gzip.open(path, "rt") if compressed else path.open() as source:
            payload = json.load(source)
        rows = payload if isinstance(payload, list) else payload.get("data")
        if not isinstance(rows, list) or not rows or not all(isinstance(row, dict) and row for row in rows):
            raise ValueError("expected a nonempty array of records, directly or under data")
        dataset = path.parent.name
        partition = path.name.split(".")[0]
        if dataset == "bmunits":
            required = {"nationalGridBmUnit", "elexonBmUnit", "eic"}
        elif dataset == "remit":
            required = {"mrid", "revisionNumber", "publishTime", "messageType"}
        elif dataset == "boalf":
            required = {"settlementDate", "settlementPeriodFrom", "settlementPeriodTo", "timeFrom", "timeTo"}
        else:
            required = {"settlementDate", "settlementPeriod"}
        required |= {
            "system_prices": {"systemBuyPrice", "systemSellPrice"},
            "demand_outturn": {"initialDemandOutturn", "initialTransmissionSystemDemandOutturn"},
            "fuelhh": {"fuelType", "generation"},
            "interconnectors": {"interconnectorName", "generation"},
            "mid": {"dataProvider", "price", "volume"},
            "boalf": {"bmUnit", "nationalGridBmUnit", "acceptanceNumber"},
            "bod": {"bmUnit", "nationalGridBmUnit", "timeFrom", "timeTo", "pairId", "offer", "bid"},
            "mels": {"bmUnit", "nationalGridBmUnit", "timeFrom", "timeTo", "notificationSequence"},
            "mils": {"bmUnit", "nationalGridBmUnit", "timeFrom", "timeTo", "notificationSequence"},
            "pn": {"bmUnit", "nationalGridBmUnit", "timeFrom", "timeTo"},
        }.get(dataset, set())
        for row in rows:
            if not required <= row.keys():
                raise ValueError(f"missing required {dataset} fields")
            if "settlementDate" in required and row["settlementDate"] != partition:
                raise ValueError("settlement date does not match the requested file")
            if dataset == "remit" and not row["publishTime"].startswith(partition):
                raise ValueError("publication time does not match the requested month")
    except (OSError, EOFError, ValueError, AttributeError) as error:
        print(f"Invalid response {path}: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
