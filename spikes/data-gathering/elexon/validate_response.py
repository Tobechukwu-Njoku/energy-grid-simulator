from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent))
from stream_rows import rows_from


def main():
    path = Path(sys.argv[1])
    compressed = sys.argv[2] == "true"
    try:
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
        count = 0
        for row in rows_from(path, compressed):
            count += 1
            if not required <= row.keys():
                raise ValueError(f"missing required {dataset} fields")
            if "settlementDate" in required and row["settlementDate"] != partition:
                raise ValueError("settlement date does not match the requested file")
            if dataset == "remit" and not row["publishTime"].startswith(partition):
                raise ValueError("publication time does not match the requested month")
        if count == 0:
            raise ValueError("expected nonempty records")
    except (OSError, EOFError, ValueError, AttributeError) as error:
        print(f"Invalid response {path}: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
