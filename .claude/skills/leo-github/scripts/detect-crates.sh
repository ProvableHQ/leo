#!/bin/bash
# Detect affected Leo crates from PR file list
# Usage: detect-crates.sh <pr_number>

set -euo pipefail

WS="${WS:-.claude/workspace}"
NUM="${1:-}"

if [[ -z "$NUM" ]]; then
  echo "Usage: detect-crates.sh <pr_number>"
  exit 1
fi

if [[ ! -f "$WS/files-pr-$NUM.txt" ]]; then
  echo "No files list found. Run fetch-pr.sh first."
  exit 1
fi

echo "Detecting affected crates for PR #$NUM..."

# Extract unique directories and map to crates
CRATES=""

while IFS= read -r line; do
  # Extract path (after the +/- stats)
  path=$(echo "$line" | sed 's/^[0-9]*+\/[0-9]*- //')

  case "$path" in
    compiler/ast/*) CRATES="$CRATES leo-ast" ;;
    compiler/compiler/*) CRATES="$CRATES leo-compiler" ;;
    compiler/parser/*) CRATES="$CRATES leo-parser" ;;
    compiler/parser-lossless/*) CRATES="$CRATES leo-parser-lossless" ;;
    compiler/passes/*) CRATES="$CRATES leo-passes" ;;
    compiler/span/*) CRATES="$CRATES leo-span" ;;
    errors/*) CRATES="$CRATES leo-errors" ;;
    interpreter/*) CRATES="$CRATES leo-interpreter" ;;
    leo/package/*) CRATES="$CRATES leo-package" ;;
    test-framework/*) CRATES="$CRATES leo-test-framework" ;;
    utils/disassembler/*) CRATES="$CRATES leo-disassembler" ;;
  esac
done < "$WS/files-pr-$NUM.txt"

# Deduplicate and sort
UNIQUE_CRATES=$(echo "$CRATES" | tr ' ' '\n' | sort -u | grep -v '^$' | tr '\n' ' ')

if [[ -n "$UNIQUE_CRATES" ]]; then
  echo "$UNIQUE_CRATES" > "$WS/crates-pr-$NUM.txt"
  echo "Affected crates: $UNIQUE_CRATES"
else
  echo "No Leo crates detected (may be docs/config changes)"
  echo "" > "$WS/crates-pr-$NUM.txt"
fi
