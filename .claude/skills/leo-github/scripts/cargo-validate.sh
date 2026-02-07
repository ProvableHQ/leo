#!/bin/bash
# Validate Rust crates with cargo check, clippy, fmt, and test
# Usage: cargo-validate.sh [crate1 crate2 ...] or reads from PR files

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WS="${WS:-.claude/workspace}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Get crates to validate
CRATES=()
if [[ $# -gt 0 ]]; then
  CRATES=("$@")
else
  # Try to detect from PR files
  PR_FILES=$(ls "$WS"/files-pr-*.txt 2>/dev/null | head -1 || true)
  if [[ -n "$PR_FILES" ]]; then
    log_info "Detecting crates from $PR_FILES"
    while IFS= read -r crate; do
      CRATES+=("$crate")
    done < <(cut -d' ' -f2 "$PR_FILES" | "$SCRIPT_DIR/detect-crates.sh")
  fi
fi

# Fallback to root crate if nothing found
if [[ ${#CRATES[@]} -eq 0 ]]; then
  if [[ -f "Cargo.toml" ]]; then
    CRATES=("leo-lang")
    log_warn "No crates detected, using root crate"
  else
    log_error "No crates to validate and no Cargo.toml found"
    exit 1
  fi
fi

log_info "Validating crates: ${CRATES[*]}"

FAILED=0

for crate in "${CRATES[@]}"; do
  log_info "=== Checking $crate ==="

  # cargo check
  if ! cargo check -p "$crate" 2>&1; then
    log_error "cargo check failed for $crate"
    FAILED=1
    continue
  fi

  # cargo clippy
  if ! cargo clippy -p "$crate" -- -D warnings 2>&1; then
    log_error "cargo clippy failed for $crate"
    FAILED=1
    continue
  fi

  log_info "$crate: check + clippy passed"
done

# Format check (once, not per-crate)
log_info "=== Checking formatting ==="
if ! cargo +nightly fmt --check 2>&1; then
  log_warn "Formatting issues detected (run 'cargo +nightly fmt --all' to fix)"
fi

# Run tests for each crate
for crate in "${CRATES[@]}"; do
  log_info "=== Testing $crate ==="
  if ! cargo test -p "$crate" --lib 2>&1; then
    log_error "cargo test failed for $crate"
    FAILED=1
  fi
done

if [[ $FAILED -eq 0 ]]; then
  log_info "All validations passed!"
  exit 0
else
  log_error "Some validations failed"
  exit 1
fi
