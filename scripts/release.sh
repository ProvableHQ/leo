#!/usr/bin/env bash
set -euo pipefail

# Create and push a release tag for a binary crate.
#
# Usage:
#   ./scripts/release.sh <crate-name>
#
# Examples:
#   ./scripts/release.sh leo-lang   # Tags leo-lang-v4.0.1
#   ./scripts/release.sh leo-fmt    # Tags leo-fmt-v4.0.1

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

# Find the crate by its package name.
FOUND=""
for toml in crates/*/Cargo.toml; do
  if grep -q "^name = \"$CRATE_NAME\"" "$toml"; then
    FOUND="$toml"
    break
  fi
done

if [ -z "$FOUND" ]; then
  echo "Error: no crate found with name '$CRATE_NAME'"
  exit 1
fi

if ! grep -q '^\[\[bin\]\]' "$FOUND"; then
  echo "Error: crate '$CRATE_NAME' has no [[bin]] entries"
  exit 1
fi

VERSION=$(grep '^version' "$FOUND" | head -1 | sed 's/.*= *"\(.*\)"/\1/')
REPO_URL=$(grep '^repository' "$FOUND" | head -1 | sed 's/.*= *"\(.*\)"/\1/')
TAG="${CRATE_NAME}-v${VERSION}"

echo "Crate:   $(dirname "$FOUND")"
echo "Version: $VERSION"
echo "Tag:     $TAG"
echo "Push to: $REPO_URL"
echo ""

if git rev-parse "$TAG" >/dev/null 2>&1; then
  echo "Error: tag '$TAG' already exists locally"
  exit 1
fi

if git ls-remote --tags "$REPO_URL" "$TAG" | grep -q .; then
  echo "Error: tag '$TAG' already exists on remote"
  exit 1
fi

read -rp "Create and push tag '$TAG'? [y/N] " confirm
if [[ "$confirm" != [yY] ]]; then
  echo "Aborted."
  exit 0
fi

git tag "$TAG"
git push "$REPO_URL" "$TAG"
echo "Done. Release workflow will build and publish artifacts."
