#!/usr/bin/env bash
set -euo pipefail

# Create and push a release tag for a binary crate.
#
# Usage:
#   ./scripts/release.sh <binary-name>
#
# Examples:
#   ./scripts/release.sh leo        # Tags leo-v4.0.1 (version from crates/leo/Cargo.toml)
#   ./scripts/release.sh leo-fmt    # Tags leo-fmt-v4.0.1

BIN_NAME="${1:-}"
if [ -z "$BIN_NAME" ]; then
  echo "Usage: $0 <binary-name>"
  echo "Available binaries:"
  grep -rl '^\[\[bin\]\]' crates/*/Cargo.toml | while read -r toml; do
    name=$(grep -A5 '^\[\[bin\]\]' "$toml" | grep '^name' | head -1 | sed 's/.*= *"\(.*\)"/\1/')
    version=$(grep '^version' "$toml" | head -1 | sed 's/.*= *"\(.*\)"/\1/')
    echo "  $name (v$version)"
  done
  exit 1
fi

# Find the crate whose [[bin]] section declares a matching name.
# Uses -A5 to tolerate comments or blank lines between [[bin]] and name.
FOUND=""
for toml in crates/*/Cargo.toml; do
  if grep -A5 '^\[\[bin\]\]' "$toml" | grep -q "^name = \"$BIN_NAME\""; then
    FOUND="$toml"
    break
  fi
done

if [ -z "$FOUND" ]; then
  echo "Error: no crate found with binary name '$BIN_NAME'"
  exit 1
fi

VERSION=$(grep '^version' "$FOUND" | head -1 | sed 's/.*= *"\(.*\)"/\1/')
REPO_URL=$(grep '^repository' "$FOUND" | head -1 | sed 's/.*= *"\(.*\)"/\1/')
TAG="${BIN_NAME}-v${VERSION}"

echo "Crate:   $(dirname "$FOUND")"
echo "Binary:  $BIN_NAME"
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
