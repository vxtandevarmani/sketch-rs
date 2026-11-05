#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <binary>"
    exit 1
fi

BIN="$1"

if [[ ! -f "$BIN" ]]; then
    echo "Binary not found: $BIN"
    exit 1
fi

echo "========================================"
echo "             ELF PARSER"
echo "========================================"

echo
echo "============== .dynsym =============="
readelf --dyn-syms -W "$BIN"

echo
echo "============== .dynstr =============="
readelf --string-dump=.dynstr "$BIN"

echo
echo "============== .rela.plt =============="
readelf -rW "$BIN" | awk '
/Relocation section .*\.rela\.plt/ { show=1 }
show { print }
/^$/ && show { show=0 }
'

echo
echo "============== .rela.dyn =============="
readelf -rW "$BIN" | awk '
/Relocation section .*\.rela\.dyn/ { show=1 }
show { print }
/^$/ && show { show=0 }
'

echo
echo "============== .dynamic =============="
readelf -dW "$BIN"

echo
echo "============== DONE =============="
