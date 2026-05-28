#!/usr/bin/env bash
set -euo pipefail

# Generates the Markdown release-body prefix and leo-release.toml asset for a
# per-crate GitHub release from a tag like `leo-lsp-v4.0.2`.
#
# The script reads workspace package metadata from `cargo metadata`, the exact
# snarkVM version from Cargo.lock, and the compatible snarkOS version from
# .resources/snarkos-version.
#
# Expected tools: bash 4+, cargo, git, jq, awk, and standard POSIX utilities.
# GitHub generates PR-based release notes separately; this script only writes
# the compatibility table, first-release note, and machine-readable metadata.

if [ "$#" -lt 1 ] || [ "$#" -gt 2 ]; then
  echo "Usage: $0 <tag> [output-dir]" >&2
  exit 2
fi

TAG="$1"
OUTPUT_DIR="${2:-.}"
if ! [[ "$TAG" =~ ^(.+)-v([0-9][A-Za-z0-9.+-]*)$ ]]; then
  echo "Error: failed to parse release tag '$TAG'" >&2
  exit 1
fi
CRATE_NAME="${BASH_REMATCH[1]}"
VERSION="${BASH_REMATCH[2]}"
GITHUB_REPOSITORY="${GITHUB_REPOSITORY:-ProvableHQ/leo}"
REPO_URL="https://github.com/${GITHUB_REPOSITORY}"
SNARKOS_REPO_URL="https://github.com/ProvableHQ/snarkOS"
BODY_PATH="${OUTPUT_DIR}/release-body.md"
TOML_PATH="${OUTPUT_DIR}/leo-release.toml"
METADATA_PATH=""
WORKSPACE_ROOT=""

mkdir -p "$OUTPUT_DIR"

cleanup() {
  if [ -n "$METADATA_PATH" ]; then
    rm -f "$METADATA_PATH"
  fi
}
trap cleanup EXIT

require_tool() {
  local tool="$1"
  if ! command -v "$tool" >/dev/null 2>&1; then
    echo "Error: required tool '$tool' not found" >&2
    exit 1
  fi
}

load_cargo_metadata() {
  METADATA_PATH="$(mktemp "${TMPDIR:-/tmp}/leo-cargo-metadata.XXXXXX")"
  cargo metadata --locked --format-version 1 --no-deps > "$METADATA_PATH"
  WORKSPACE_ROOT="$(jq -er '.workspace_root' "$METADATA_PATH")"
}

workspace_relative_path() {
  local path="$1"

  if [ "$path" = "$WORKSPACE_ROOT" ]; then
    printf '.\n'
  elif [[ "$path" == "$WORKSPACE_ROOT/"* ]]; then
    printf '%s\n' "${path#"$WORKSPACE_ROOT"/}"
  else
    printf '%s\n' "$path"
  fi
}

package_manifest() {
  local package="$1"
  local manifest

  if ! manifest="$(jq -er --arg package "$package" '
    first(.packages[] | select(.name == $package) | .manifest_path) // empty
  ' "$METADATA_PATH")"; then
    echo "Error: no Cargo metadata found for package '$package'" >&2
    return 1
  fi

  workspace_relative_path "$manifest"
}

package_version() {
  local package="$1"

  jq -er --arg package "$package" '
    first(.packages[] | select(.name == $package) | .version) // empty
  ' "$METADATA_PATH"
}

package_bins() {
  local package="$1"

  jq -r --arg package "$package" '
    first(.packages[] | select(.name == $package)) as $pkg
    | $pkg.targets[]?
    | select((.kind // []) | index("bin"))
    | .name
  ' "$METADATA_PATH" | sort -u
}

lock_version() {
  local package="$1"
  awk -v package="$package" '
    /^\[\[package\]\][[:space:]]*$/ {
      in_package = 1
      seen_name = 0
      next
    }
    /^\[/ && $0 !~ /^\[\[package\]\]/ {
      in_package = 0
      seen_name = 0
      next
    }
    in_package {
      line = $0
      sub(/[[:space:]]*#.*/, "", line)
      split(line, parts, "=")
      key = parts[1]
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", key)
      value = substr(line, index(line, "=") + 1)
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", value)
      gsub(/^"|"$/, "", value)
      if (key == "name" && value == package) {
        seen_name = 1
      } else if (seen_name && key == "version") {
        print value
        found = 1
        exit
      }
    }
    END {
      if (!found) {
        exit 1
      }
    }
  ' Cargo.lock
}

target_list() {
  awk '/^[[:space:]]*- target:/ { print $3 }' .github/workflows/release-crate.yml
}

previous_same_crate_tag() {
  git tag --list "${CRATE_NAME}-v[0-9]*" --sort=-v:refname \
    | awk -v current="$TAG" 'seen { print; exit } $0 == current { seen = 1 }'
}

toml_array() {
  local item
  local first=1
  printf '['
  for item in "$@"; do
    if [ "$first" -eq 0 ]; then
      printf ', '
    fi
    first=0
    printf '"%s"' "$item"
  done
  printf ']'
}

component_row() {
  local name="$1"
  local version="$2"
  local version_url="$3"
  local downloads_url="$4"
  local downloads_label="$5"
  printf '| `%s` | [%s](%s) | [%s](%s) |\n' "$name" "$version" "$version_url" "$downloads_label" "$downloads_url"
}

write_component_toml() {
  local name="$1"
  local version="$2"
  local tag="${name}-v${version}"
  shift 2
  local bins=("$@")

  {
    printf '\n[components.%s]\n' "$name"
    printf 'version = "%s"\n' "$version"
    printf 'tag = "%s"\n' "$tag"
    printf 'crate_url = "https://crates.io/crates/%s/%s"\n' "$name" "$version"
    printf 'release_url = "%s/releases/tag/%s"\n' "$REPO_URL" "$tag"
    printf 'archive_url_template = "%s/releases/download/%s/%s-{target}.zip"\n' "$REPO_URL" "$tag" "$tag"
    printf 'binaries = '
    toml_array "${bins[@]}"
    printf '\n'
  } >> "$TOML_PATH"
}

for tool in awk cargo git jq mktemp sort tr; do
  require_tool "$tool"
done

if [ ! -f Cargo.lock ]; then
  echo "Error: Cargo.lock missing" >&2
  exit 1
fi
if [ ! -f .resources/snarkos-version ]; then
  echo "Error: .resources/snarkos-version missing" >&2
  exit 1
fi

load_cargo_metadata
CRATE_MANIFEST="$(package_manifest "$CRATE_NAME")"
TOML_VERSION="$(package_version "$CRATE_NAME")"
if [ "$TOML_VERSION" != "$VERSION" ]; then
  echo "Error: tag version ($VERSION) does not match $CRATE_MANIFEST version ($TOML_VERSION)" >&2
  exit 1
fi

SNARKOS_VERSION="$(tr -d '[:space:]' < .resources/snarkos-version)"
if ! [[ "$SNARKOS_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "Error: .resources/snarkos-version must contain a plain semver version, got '$SNARKOS_VERSION'" >&2
  exit 1
fi
SNARKVM_VERSION="$(lock_version snarkvm)"
if COMMIT="$(git rev-parse --verify -q "${TAG}^{commit}")"; then
  :
else
  COMMIT="$(git rev-parse HEAD)"
fi

mapfile -t TARGETS < <(target_list)
if [ "${#TARGETS[@]}" -eq 0 ]; then
  echo "Error: no release targets found in release-crate workflow" >&2
  exit 1
fi
PREVIOUS_TAG="${PREVIOUS_TAG:-$(previous_same_crate_tag || true)}"

COMPONENTS=(leo-lang leo-fmt leo-lsp)
declare -A COMPONENT_VERSIONS
declare -A COMPONENT_BINS

for component in "${COMPONENTS[@]}"; do
  package_manifest "$component" >/dev/null
  COMPONENT_VERSIONS["$component"]="$(package_version "$component")"
  mapfile -t bins < <(package_bins "$component")
  if [ "${#bins[@]}" -eq 0 ]; then
    echo "Error: package '$component' has no [[bin]] entries" >&2
    exit 1
  fi
  COMPONENT_BINS["$component"]="$(printf '%s\n' "${bins[@]}")"
done

{
  if [ -z "$PREVIOUS_TAG" ]; then
    printf 'This is the first tagged release for `%s`.\n' "$CRATE_NAME"
    printf '\n'
  fi

  printf '## Compatible Versions\n\n'
  printf '| Component | Version | Downloads |\n'
  printf '|-----------|---------|-----------|\n'
  for component in "${COMPONENTS[@]}"; do
    component_version="${COMPONENT_VERSIONS[$component]}"
    component_release_url="${REPO_URL}/releases/tag/${component}-v${component_version}"
    component_row "$component" "$component_version" "$component_release_url" "$component_release_url" "release assets"
  done
  component_row snarkvm "$SNARKVM_VERSION" \
    "https://crates.io/crates/snarkvm/${SNARKVM_VERSION}" \
    "https://crates.io/crates/snarkvm/${SNARKVM_VERSION}" \
    "crate"
  component_row snarkOS "$SNARKOS_VERSION" \
    "${SNARKOS_REPO_URL}/releases/tag/v${SNARKOS_VERSION}" \
    "${SNARKOS_REPO_URL}/releases/tag/v${SNARKOS_VERSION}" \
    "release assets"
} > "$BODY_PATH"

{
  printf '[release]\n'
  printf 'crate = "%s"\n' "$CRATE_NAME"
  printf 'version = "%s"\n' "$VERSION"
  printf 'tag = "%s"\n' "$TAG"
  printf 'commit = "%s"\n' "$COMMIT"
  printf 'repository = "%s"\n' "$REPO_URL"
  printf 'targets = '
  toml_array "${TARGETS[@]}"
  printf '\n'
} > "$TOML_PATH"

for component in "${COMPONENTS[@]}"; do
  mapfile -t bins < <(printf '%s\n' "${COMPONENT_BINS[$component]}")
  write_component_toml "$component" "${COMPONENT_VERSIONS[$component]}" "${bins[@]}"
done

{
  printf '\n[components.snarkvm]\n'
  printf 'version = "%s"\n' "$SNARKVM_VERSION"
  printf 'crate_url = "https://crates.io/crates/snarkvm/%s"\n' "$SNARKVM_VERSION"

  printf '\n[components.snarkos]\n'
  printf 'version = "%s"\n' "$SNARKOS_VERSION"
  printf 'tag = "v%s"\n' "$SNARKOS_VERSION"
  printf 'release_url = "%s/releases/tag/v%s"\n' "$SNARKOS_REPO_URL" "$SNARKOS_VERSION"
  printf 'archive_url_template = "%s/releases/download/v%s/aleo-v%s-{target}.zip"\n' \
    "$SNARKOS_REPO_URL" "$SNARKOS_VERSION" "$SNARKOS_VERSION"
} >> "$TOML_PATH"

printf 'Wrote %s\n' "$BODY_PATH"
printf 'Wrote %s\n' "$TOML_PATH"
