#!/bin/bash
# Validate affected crates (check, clippy, test)
set -e

NUM=$1
WS=".claude/workspace"

[ -z "$NUM" ] && echo "Usage: cargo-validate.sh <number>" && exit 1
[ ! -f "$WS/crates-pr-$NUM.txt" ] && echo "No crates list found. Run detect-crates.sh first." && exit 1

CRATES=$(cat "$WS/crates-pr-$NUM.txt")
[ -z "$CRATES" ] && echo "No crates to validate" && exit 0

echo "Validating crates: $CRATES"
echo ""

FAILED=""

for crate in $CRATES; do
  echo "=== $crate ==="

  echo "  cargo check..."
  if ! cargo check -p "$crate" 2>&1 | tail -5; then
    FAILED="$FAILED $crate:check"
  fi

  echo "  cargo clippy..."
  if ! cargo clippy -p "$crate" -- -D warnings 2>&1 | tail -5; then
    FAILED="$FAILED $crate:clippy"
  fi

  echo "  cargo test..."
  if ! cargo test -p "$crate" --lib 2>&1 | tail -10; then
    FAILED="$FAILED $crate:test"
  fi

  echo ""
done

if [ -n "$FAILED" ]; then
  echo "FAILED:$FAILED"
  exit 1
else
  echo "All validations passed"
fi
