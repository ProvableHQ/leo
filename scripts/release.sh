#!/usr/bin/env bash
set -euo pipefail

# Dispatch GitHub release artifact generation for a binary crate.
#
# Normal releases are crates.io first:
#   1. Bump crate versions.
#   2. Merge to master.
#   3. Let publish-crates.yml publish crates and create package tags.
#
# Use this script for backfills or reruns after a binary crate version is
# already published on crates.io.

CRATE_NAME="${1:-}"
if [ -z "$CRATE_NAME" ]; then
  echo "Usage: $0 <crate-name>"
  echo "Available binary crates:"
  for toml in crates/*/Cargo.toml; do
    if grep -q '^\[\[bin\]\]' "$toml"; then
      name=$(grep '^name' "$toml" | head -1 | sed 's/.*= *"\(.*\)"/\1/')
      version=$(grep '^version' "$toml" | head -1 | sed 's/.*= *"\(.*\)"/\1/')
      echo "  $name (v$version)"
    fi
  done
  exit 1
fi

FOUND=""
for toml in crates/*/Cargo.toml; do
  if grep -q "^name = \"$CRATE_NAME\"" "$toml"; then
    FOUND="$toml"
    break
  fi
done

if [ -z "$FOUND" ]; then
  echo "Error: no crate found with name '$CRATE_NAME'" >&2
  exit 1
fi

if ! grep -q '^\[\[bin\]\]' "$FOUND"; then
  echo "Error: crate '$CRATE_NAME' has no [[bin]] entries" >&2
  exit 1
fi

VERSION=$(grep '^version' "$FOUND" | head -1 | sed 's/.*= *"\(.*\)"/\1/')
TAG="${CRATE_NAME}-v${VERSION}"

echo "Crate:   $(dirname "$FOUND")"
echo "Version: $VERSION"
echo "Tag:     $TAG"
echo ""
echo "Normal releases publish crates from master:"
echo "  gh workflow run publish-crates.yml --ref master"
echo ""
echo "Backfill or rerun GitHub release artifacts with:"
echo "  gh workflow run release-crate.yml --ref master -f tag=$TAG"
echo ""
echo "release-crate.yml creates the tag if it is missing."
echo ""

if ! command -v gh >/dev/null 2>&1; then
  echo "GitHub CLI 'gh' not found; run the command above from a configured shell."
  exit 0
fi

read -rp "Dispatch release-crate.yml for '$TAG'? [y/N] " confirm
if [[ "$confirm" != [yY] ]]; then
  echo "Aborted."
  exit 0
fi

gh workflow run release-crate.yml --ref master -f tag="$TAG"
echo "Dispatched release-crate.yml for $TAG."
