#!/bin/sh
set -eu

cd "$(dirname "$0")"
include_uncleared=false
case "${1:-}" in
    "") ;;
    --include-uncleared) include_uncleared=true ;;
    *) printf 'Usage: sh spikes/data/fetch.sh [--include-uncleared]\n' >&2; exit 2 ;;
esac
mkdir -p raw samples
while IFS="$(printf '\t')" read -r clearance name url; do
    case "$clearance" in ""|\#*) continue ;; esac
    if [ "$clearance" = inspect-only ] && [ "$include_uncleared" = false ]; then
        printf 'SKIP uncleared inspection source: %s\n' "$name"
        continue
    fi
    printf '%s: %s\n' "$clearance" "$name"
    curl --fail --location --silent --show-error --retry 2 --connect-timeout 20 --max-time 180 \
        "$url" --output "raw/$name.part"
    mv "raw/$name.part" "raw/$name"
done < sources.tsv
