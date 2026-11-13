#!/usr/bin/env bash
# Elexon Insights Solution (BMRS) Data Fetcher for GB Market Backtest (2024)
# Resumable, idempotent, concurrent with retries and exponential backoff.
# Licence: BMRS Open Data Licence (open-commercial)

set -euo pipefail

BASE_URL="https://data.elexon.co.uk/bmrs/api/v1"
RAW_BASE="${DATA_DIR:-${HOME}/energy-data/raw/elexon}"
PYTHON="${PYTHON:-python3}"
"$PYTHON" -I -c 'import ijson'
CONCURRENCY="${CONCURRENCY:-6}"
MAX_RETRIES="${MAX_RETRIES:-5}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PIDS=""

if [[ ! "$CONCURRENCY" =~ ^[1-9][0-9]*$ ]]; then
    echo "CONCURRENCY must be a positive integer" >&2
    exit 1
fi
if [[ ! "$MAX_RETRIES" =~ ^[1-9][0-9]*$ ]]; then
    echo "MAX_RETRIES must be a positive integer" >&2
    exit 1
fi

validate_response() {
    "$PYTHON" -I "$SCRIPT_DIR/validate_response.py" "$1" "$2"
}

wait_batch() {
    local failed=0
    local process_id
    for process_id in $PIDS; do
        if ! wait "$process_id"; then
            failed=1
        fi
    done
    PIDS=""
    return "$failed"
}

# Generate all 366 dates for leap year 2024
get_dates_2024() {
    "$PYTHON" -I -c "
import datetime
start = datetime.date(2024, 1, 1)
end = datetime.date(2024, 12, 31)
cur = start
while cur <= end:
    print(cur.isoformat())
    cur += datetime.timedelta(days=1)
"
}

# Fetch single URL with retries and exponential backoff
fetch_one() (
    local url="$1"
    local dest="$2"
    local accept_gzip="${3:-false}"

    if [[ -s "$dest" ]] && validate_response "$dest" "$accept_gzip"; then
        return 0
    fi

    local tmp
    tmp=$(mktemp "${dest}.tmp.XXXXXX")
    trap 'rm -f "$tmp" "$tmp.gz"' EXIT
    trap 'exit 130' INT
    trap 'exit 143' HUP TERM
    local attempt=1
    local delay=2

    while [[ $attempt -le $MAX_RETRIES ]]; do
        local http_code
        http_code=$(curl -sS -w "%{http_code}" --compressed \
            --connect-timeout 20 --max-time 180 "$url" -o "$tmp" || echo "000")

        if [[ "$http_code" == "200" ]] && validate_response "$tmp" false; then
            if [[ "$accept_gzip" == "true" ]]; then
                gzip -n -c "$tmp" > "$tmp.gz"
                mv "$tmp.gz" "$dest"
            else
                mv "$tmp" "$dest"
            fi
            return 0
        fi

        echo "Attempt $attempt failed validation or HTTP ($http_code) for $url." >&2
        rm -f "$tmp"
        if (( attempt < MAX_RETRIES )); then
            sleep "$delay"
        fi
        delay=$((delay * 2))
        attempt=$((attempt + 1))
    done

    echo "ERROR: Failed to download $url after $MAX_RETRIES attempts" >&2
    return 1
)

fetch_bmunits() {
    local target_dir="$RAW_BASE/bmunits"
    mkdir -p "$target_dir"
    echo "Fetching BM Unit reference data..."
    fetch_one "$BASE_URL/reference/bmunits/all?format=json" "$target_dir/bmunits.json" false
    echo "BM Unit reference data downloaded."
}

fetch_system_prices() {
    local target_dir="$RAW_BASE/system_prices"
    mkdir -p "$target_dir"
    echo "Fetching System Prices (imbalance) for 2024 (366 dates, concurrency=$CONCURRENCY)..."
    local count=0
    for dt in $(get_dates_2024); do
        local dest="$target_dir/${dt}.json"
        fetch_one "$BASE_URL/balancing/settlement/system-prices/${dt}?format=json" "$dest" false &
        PIDS="$PIDS $!"
        count=$((count + 1))
        if (( count % CONCURRENCY == 0 )); then
            wait_batch
        fi
    done
    wait_batch
    echo "System Prices complete ($(ls "$target_dir"/*.json 2>/dev/null | wc -l | tr -d ' ') files)."
}

fetch_demand_outturn() {
    local target_dir="$RAW_BASE/demand_outturn"
    mkdir -p "$target_dir"
    echo "Fetching Demand Outturn (INDO/ITSDO) for 2024 (366 dates, concurrency=$CONCURRENCY)..."
    local count=0
    for dt in $(get_dates_2024); do
        local dest="$target_dir/${dt}.json"
        fetch_one "$BASE_URL/demand/outturn?settlementDateFrom=${dt}&settlementDateTo=${dt}&format=json" "$dest" false &
        PIDS="$PIDS $!"
        count=$((count + 1))
        if (( count % CONCURRENCY == 0 )); then
            wait_batch
        fi
    done
    wait_batch
    echo "Demand Outturn complete ($(ls "$target_dir"/*.json 2>/dev/null | wc -l | tr -d ' ') files)."
}

fetch_fuelhh() {
    local target_dir="$RAW_BASE/fuelhh"
    mkdir -p "$target_dir"
    echo "Fetching Generation by Fuel Type (FUELHH) for 2024 (366 dates, concurrency=$CONCURRENCY)..."
    local count=0
    for dt in $(get_dates_2024); do
        local dest="$target_dir/${dt}.json"
        fetch_one "$BASE_URL/datasets/FUELHH/stream?settlementDateFrom=${dt}&settlementDateTo=${dt}" "$dest" false &
        PIDS="$PIDS $!"
        count=$((count + 1))
        if (( count % CONCURRENCY == 0 )); then
            wait_batch
        fi
    done
    wait_batch
    echo "FUELHH complete ($(ls "$target_dir"/*.json 2>/dev/null | wc -l | tr -d ' ') files)."
}

fetch_interconnectors() {
    local target_dir="$RAW_BASE/interconnectors"
    mkdir -p "$target_dir"
    echo "Fetching Interconnector Flows for 2024 (366 dates, concurrency=$CONCURRENCY)..."
    local count=0
    for dt in $(get_dates_2024); do
        local dest="$target_dir/${dt}.json"
        fetch_one "$BASE_URL/generation/outturn/interconnectors?settlementDateFrom=${dt}&settlementDateTo=${dt}&format=json" "$dest" false &
        PIDS="$PIDS $!"
        count=$((count + 1))
        if (( count % CONCURRENCY == 0 )); then
            wait_batch
        fi
    done
    wait_batch
    echo "Interconnectors complete ($(ls "$target_dir"/*.json 2>/dev/null | wc -l | tr -d ' ') files)."
}

fetch_mid() {
    local target_dir="$RAW_BASE/mid"
    mkdir -p "$target_dir"
    echo "Fetching Market Index Data (Market Index Price) for 2024 (366 dates, concurrency=$CONCURRENCY)..."
    local count=0
    for dt in $(get_dates_2024); do
        local dest="$target_dir/${dt}.json"
        fetch_one "$BASE_URL/datasets/MID/stream?from=${dt}&to=${dt}&settlementPeriodFrom=1&settlementPeriodTo=50" "$dest" false &
        PIDS="$PIDS $!"
        count=$((count + 1))
        if (( count % CONCURRENCY == 0 )); then
            wait_batch
        fi
    done
    wait_batch
    echo "Market Index Data complete ($(ls "$target_dir"/*.json 2>/dev/null | wc -l | tr -d ' ') files)."
}

fetch_remit() {
    local target_dir="$RAW_BASE/remit"
    mkdir -p "$target_dir"
    echo "Fetching REMIT outage messages for 2024 (month by month)..."
    local months=(
        "2024-01-01T00:00:00Z 2024-02-01T00:00:00Z 2024-01"
        "2024-02-01T00:00:00Z 2024-03-01T00:00:00Z 2024-02"
        "2024-03-01T00:00:00Z 2024-04-01T00:00:00Z 2024-03"
        "2024-04-01T00:00:00Z 2024-05-01T00:00:00Z 2024-04"
        "2024-05-01T00:00:00Z 2024-06-01T00:00:00Z 2024-05"
        "2024-06-01T00:00:00Z 2024-07-01T00:00:00Z 2024-06"
        "2024-07-01T00:00:00Z 2024-08-01T00:00:00Z 2024-07"
        "2024-08-01T00:00:00Z 2024-09-01T00:00:00Z 2024-08"
        "2024-09-01T00:00:00Z 2024-10-01T00:00:00Z 2024-09"
        "2024-10-01T00:00:00Z 2024-11-01T00:00:00Z 2024-10"
        "2024-11-01T00:00:00Z 2024-12-01T00:00:00Z 2024-11"
        "2024-12-01T00:00:00Z 2025-01-01T00:00:00Z 2024-12"
    )
    for m in "${months[@]}"; do
        read -r from_ts to_ts label <<< "$m"
        local dest="$target_dir/${label}.json"
        fetch_one "$BASE_URL/datasets/REMIT/stream?publishDateTimeFrom=${from_ts}&publishDateTimeTo=${to_ts}" "$dest" false
    done
    echo "REMIT complete ($(ls "$target_dir"/*.json 2>/dev/null | wc -l | tr -d ' ') files)."
}

fetch_boalf() {
    local target_dir="$RAW_BASE/boalf"
    mkdir -p "$target_dir"
    echo "Fetching Bid-Offer Acceptances (BOALF) for 2024 (366 dates, concurrency=$CONCURRENCY)..."
    local count=0
    for dt in $(get_dates_2024); do
        local dest="$target_dir/${dt}.json"
        fetch_one "$BASE_URL/datasets/BOALF/stream?from=${dt}&to=${dt}&settlementPeriodFrom=1&settlementPeriodTo=50" "$dest" false &
        PIDS="$PIDS $!"
        count=$((count + 1))
        if (( count % CONCURRENCY == 0 )); then
            wait_batch
        fi
    done
    wait_batch
    echo "BOALF complete ($(ls "$target_dir"/*.json 2>/dev/null | wc -l | tr -d ' ') files)."
}

fetch_stream_dataset() {
    local ds="$1"
    local target_dir="$RAW_BASE/$ds"
    mkdir -p "$target_dir"
    local upper_ds
    upper_ds=$(echo "$ds" | tr '[:lower:]' '[:upper:]')
    echo "Fetching $upper_ds for 2024 (366 dates, compressed .json.gz, concurrency=$CONCURRENCY)..."
    local count=0
    for dt in $(get_dates_2024); do
        local dest="$target_dir/${dt}.json.gz"
        fetch_one "$BASE_URL/datasets/${upper_ds}/stream?from=${dt}&to=${dt}&settlementPeriodFrom=1&settlementPeriodTo=50" "$dest" true &
        PIDS="$PIDS $!"
        count=$((count + 1))
        if (( count % CONCURRENCY == 0 )); then
            wait_batch
        fi
    done
    wait_batch
    echo "$upper_ds complete ($(ls "$target_dir"/*.json.gz 2>/dev/null | wc -l | tr -d ' ') files)."
}

fetch_bod() { fetch_stream_dataset "bod"; }
fetch_pn() { fetch_stream_dataset "pn"; }
fetch_mels() { fetch_stream_dataset "mels"; }
fetch_mils() { fetch_stream_dataset "mils"; }

usage() {
    echo "Usage: $0 [bmunits|system_prices|demand_outturn|fuelhh|interconnectors|mid|remit|boalf|bod|pn|mels|mils|all]"
    exit 1
}

TARGET="${1:-all}"
case "$TARGET" in
    bmunits) fetch_bmunits ;;
    system_prices) fetch_system_prices ;;
    demand_outturn) fetch_demand_outturn ;;
    fuelhh) fetch_fuelhh ;;
    interconnectors) fetch_interconnectors ;;
    mid) fetch_mid ;;
    remit) fetch_remit ;;
    boalf) fetch_boalf ;;
    bod) fetch_bod ;;
    pn) fetch_pn ;;
    mels) fetch_mels ;;
    mils) fetch_mils ;;
    all)
        fetch_bmunits
        fetch_system_prices
        fetch_demand_outturn
        fetch_fuelhh
        fetch_interconnectors
        fetch_mid
        fetch_remit
        fetch_boalf
        fetch_mels
        fetch_mils
        fetch_pn
        fetch_bod
        ;;
    *) usage ;;
esac
