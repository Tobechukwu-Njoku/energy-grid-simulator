#!/usr/bin/env bash
# Elexon Insights Solution (BMRS) Data Fetcher for GB Market Backtest (2024)
# Resumable, idempotent, concurrent with retries and exponential backoff.
# Licence: BMRS Open Data Licence (open-commercial)

set -euo pipefail

BASE_URL="https://data.elexon.co.uk/bmrs/api/v1"
RAW_BASE="${DATA_DIR:-/Users/njotob/energy-data/raw/elexon}"
YEAR="2024"
CONCURRENCY="${CONCURRENCY:-6}"
MAX_RETRIES=5

# Generate all 366 dates for leap year 2024
get_dates_2024() {
    python3 -c "
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
fetch_one() {
    local url="$1"
    local dest="$2"
    local accept_gzip="${3:-false}"

    if [[ -s "$dest" ]]; then
        # Already downloaded and non-empty
        return 0
    fi

    local tmp="${dest}.tmp.$$"
    local attempt=1
    local delay=2

    while [[ $attempt -le $MAX_RETRIES ]]; do
        local http_code
        if [[ "$accept_gzip" == "true" ]]; then
            # Write raw gzipped response directly
            http_code=$(curl -s -w "%{http_code}" -H "Accept-Encoding: gzip" \
                --connect-timeout 20 --max-time 180 "$url" -o "$tmp" || echo "000")
        else
            # Request gzip from server, decompress on the fly
            http_code=$(curl -s -w "%{http_code}" --compressed \
                --connect-timeout 20 --max-time 180 "$url" -o "$tmp" || echo "000")
        fi

        if [[ "$http_code" == "200" ]] && [[ -s "$tmp" ]]; then
            local first_char
            if [[ "$accept_gzip" == "true" ]]; then
                first_char=$(gzip -dc "$tmp" 2>/dev/null | head -c 1 || echo "")
            else
                first_char=$(head -c 1 "$tmp" || echo "")
            fi

            if [[ "$first_char" == "{" || "$first_char" == "[" ]]; then
                mv "$tmp" "$dest"
                return 0
            else
                rm -f "$tmp"
                echo "Warning: Response for $url did not start with { or [ (got '$first_char'), retrying..." >&2
            fi
        fi

        echo "Attempt $attempt failed (HTTP $http_code) for $url. Retrying in ${delay}s..." >&2
        rm -f "$tmp"
        sleep "$delay"
        delay=$((delay * 2))
        attempt=$((attempt + 1))
    done

    echo "ERROR: Failed to download $url after $MAX_RETRIES attempts" >&2
    return 1
}

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
        if [[ ! -s "$dest" ]]; then
            fetch_one "$BASE_URL/balancing/settlement/system-prices/${dt}?format=json" "$dest" false &
            count=$((count + 1))
            if (( count % CONCURRENCY == 0 )); then
                wait
            fi
        fi
    done
    wait
    echo "System Prices complete ($(ls "$target_dir"/*.json 2>/dev/null | wc -l | tr -d ' ') files)."
}

fetch_demand_outturn() {
    local target_dir="$RAW_BASE/demand_outturn"
    mkdir -p "$target_dir"
    echo "Fetching Demand Outturn (INDO/ITSDO) for 2024 (366 dates, concurrency=$CONCURRENCY)..."
    local count=0
    for dt in $(get_dates_2024); do
        local dest="$target_dir/${dt}.json"
        if [[ ! -s "$dest" ]]; then
            fetch_one "$BASE_URL/demand/outturn?settlementDateFrom=${dt}&settlementDateTo=${dt}&format=json" "$dest" false &
            count=$((count + 1))
            if (( count % CONCURRENCY == 0 )); then
                wait
            fi
        fi
    done
    wait
    echo "Demand Outturn complete ($(ls "$target_dir"/*.json 2>/dev/null | wc -l | tr -d ' ') files)."
}

fetch_fuelhh() {
    local target_dir="$RAW_BASE/fuelhh"
    mkdir -p "$target_dir"
    echo "Fetching Generation by Fuel Type (FUELHH) for 2024 (366 dates, concurrency=$CONCURRENCY)..."
    local count=0
    for dt in $(get_dates_2024); do
        local dest="$target_dir/${dt}.json"
        if [[ ! -s "$dest" ]]; then
            fetch_one "$BASE_URL/datasets/FUELHH/stream?settlementDateFrom=${dt}&settlementDateTo=${dt}" "$dest" false &
            count=$((count + 1))
            if (( count % CONCURRENCY == 0 )); then
                wait
            fi
        fi
    done
    wait
    echo "FUELHH complete ($(ls "$target_dir"/*.json 2>/dev/null | wc -l | tr -d ' ') files)."
}

fetch_interconnectors() {
    local target_dir="$RAW_BASE/interconnectors"
    mkdir -p "$target_dir"
    echo "Fetching Interconnector Flows for 2024 (366 dates, concurrency=$CONCURRENCY)..."
    local count=0
    for dt in $(get_dates_2024); do
        local dest="$target_dir/${dt}.json"
        if [[ ! -s "$dest" ]]; then
            fetch_one "$BASE_URL/generation/outturn/interconnectors?settlementDateFrom=${dt}&settlementDateTo=${dt}&format=json" "$dest" false &
            count=$((count + 1))
            if (( count % CONCURRENCY == 0 )); then
                wait
            fi
        fi
    done
    wait
    echo "Interconnectors complete ($(ls "$target_dir"/*.json 2>/dev/null | wc -l | tr -d ' ') files)."
}

fetch_mid() {
    local target_dir="$RAW_BASE/mid"
    mkdir -p "$target_dir"
    echo "Fetching Market Index Data (Market Index Price) for 2024 (366 dates, concurrency=$CONCURRENCY)..."
    local count=0
    for dt in $(get_dates_2024); do
        local dest="$target_dir/${dt}.json"
        if [[ ! -s "$dest" ]]; then
            fetch_one "$BASE_URL/datasets/MID/stream?from=${dt}&to=${dt}&settlementPeriodFrom=1&settlementPeriodTo=50" "$dest" false &
            count=$((count + 1))
            if (( count % CONCURRENCY == 0 )); then
                wait
            fi
        fi
    done
    wait
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
        if [[ ! -s "$dest" ]]; then
            fetch_one "$BASE_URL/datasets/REMIT/stream?publishDateTimeFrom=${from_ts}&publishDateTimeTo=${to_ts}" "$dest" false
        fi
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
        if [[ ! -s "$dest" ]]; then
            fetch_one "$BASE_URL/datasets/BOALF/stream?from=${dt}&to=${dt}&settlementPeriodFrom=1&settlementPeriodTo=50" "$dest" false &
            count=$((count + 1))
            if (( count % CONCURRENCY == 0 )); then
                wait
            fi
        fi
    done
    wait
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
        if [[ ! -s "$dest" ]]; then
            fetch_one "$BASE_URL/datasets/${upper_ds}/stream?from=${dt}&to=${dt}&settlementPeriodFrom=1&settlementPeriodTo=50" "$dest" true &
            count=$((count + 1))
            if (( count % CONCURRENCY == 0 )); then
                wait
            fi
        fi
    done
    wait
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
