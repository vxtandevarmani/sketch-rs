#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <binary>"
    exit 1
fi

BIN="$1"
OUT="${BIN##*/}_dump"

mkdir -p "$OUT"

for SECTION in .dynsym .dynstr .rela.plt .rela.dyn; do
    echo "[*] Extracting $SECTION"

    if readelf -SW "$BIN" | grep -Fq "$SECTION"; then
        objcopy \
            --dump-section "$SECTION=$OUT/${SECTION#.}.bin" \
            "$BIN" 2>/dev/null || true
    else
        echo "[-] $SECTION not found"
    fi
done

echo
echo "[+] Raw sections saved to: $OUT/"
ls -lh "$OUT"
