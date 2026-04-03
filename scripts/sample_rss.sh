#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -lt 2 ]; then
  echo "usage: $0 <pid> <output.csv> [interval_seconds]" >&2
  exit 1
fi

pid="$1"
output="$2"
interval="${3:-1}"

echo "timestamp_epoch_s,rss_kb" > "$output"
while kill -0 "$pid" 2>/dev/null; do
  rss="$(ps -o rss= -p "$pid" | tr -d ' ')"
  printf '%s,%s\n' "$(date +%s)" "${rss:-0}" >> "$output"
  sleep "$interval"
done
