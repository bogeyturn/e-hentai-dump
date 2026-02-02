#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUT_DIR="$SCRIPT_DIR/b"
PAD_WIDTH=2
FROM=1
TO=12

if [[ $# -lt 2 ]]; then
    echo "Usage: $0 <upper-limit> <year> [from] [to]"
    echo "Example: $0 25 2026 1 12"
    exit 1
fi

UPPER_LIMIT="$1"
YEAR="$2"

if [[ ! -z "$3" ]]; then
    FROM="$3"
fi
if [[ ! -z "$4" ]]; then
    TO="$4"
fi

mkdir -p "$OUT_DIR"

for cmd in curl; do
    if ! command -v "$cmd" >/dev/null 2>&1; then
        echo "Error: '$cmd' is required but not installed." >&2
        exit 1
    fi
done

for MONTH in $(seq -w "$FROM" "$TO"); do
    DATE="${YEAR}-$(printf "%02d" "$MONTH")"
    BASE_URL="https://ehgt.org/b/$DATE"
    mkdir -p "$OUT_DIR/$DATE"

    echo "=== Processing $DATE ==="

    for i in $(seq 1 "$UPPER_LIMIT"); do
        ID=$(printf "%02d" "$i")

        URL="$BASE_URL/$ID.webp"
        OUT="$OUT_DIR/${DATE}/${ID}.webp"

        echo "Processing $URL"

        if ! curl -fL "$URL" -o "$OUT"; then
            echo "  Not found, skipping."
            continue
        fi

        echo "  Saved → $OUT"
    done
done

echo "Done."
