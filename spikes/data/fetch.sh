#!/bin/sh
set -eu

# Resolve caller-relative raw paths before changing to the script directory.
if [ -n "${SPIKE_RAW_DIR:-}" ]; then
    case "$SPIKE_RAW_DIR" in /*) ;; *) SPIKE_RAW_DIR="$PWD/$SPIKE_RAW_DIR" ;; esac
    export SPIKE_RAW_DIR
fi
cd "$(dirname "$0")"
include_uncleared=false
allow_drift=false
for arg in "$@"; do
    case "$arg" in
        --include-uncleared) include_uncleared=true ;;
        --allow-drift) allow_drift=true ;;
        *) printf 'Usage: sh spikes/data/fetch.sh [--include-uncleared] [--allow-drift]\n' >&2; exit 2 ;;
    esac
done
raw_dir="${SPIKE_RAW_DIR:-$PWD/raw}"
mkdir -p "$raw_dir"
while IFS="$(printf '\t')" read -r clearance name url; do
    case "$clearance" in ""|\#*) continue ;; esac
    if [ "$clearance" = research-only ] && [ "$include_uncleared" = false ]; then
        printf 'SKIP research-only source: %s\n' "$name"
        continue
    fi
    printf '%s: %s\n' "$clearance" "$name"
    curl --fail --location --silent --show-error --retry 2 --connect-timeout 20 --max-time 180 \
        "$url" --output "$raw_dir/$name.part"
    mv "$raw_dir/$name.part" "$raw_dir/$name"
done < sources.tsv
if [ "$allow_drift" = true ]; then
    python3 verify_hashes.py --raw-dir "$raw_dir" --allow-drift
else
    python3 verify_hashes.py --raw-dir "$raw_dir"
fi
