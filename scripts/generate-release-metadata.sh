#!/usr/bin/env bash
set -euo pipefail

# Generates the Markdown body and leo-release.toml asset for a per-crate GitHub
# release from a tag like `leo-lsp-v4.0.2`.
#
# The script reads crate versions from Cargo.toml files, the exact snarkVM
# version from Cargo.lock, and the compatible snarkOS version from
# .resources/snarkos-version. It avoids JSON/TOML helper dependencies so it can
# run in GitHub Actions with the standard checkout plus Rust install.
#
# Expected tools: bash 4+, awk, git, grep, tr.

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

mkdir -p "$OUTPUT_DIR"

toml_value() {
  local file="$1"
  local section="$2"
  local field="$3"
  awk -v section="$section" -v field="$field" '
    /^[[:space:]]*#/ { next }
    /^[[:space:]]*$/ { next }
    /^\[[^]]+\][[:space:]]*$/ {
      current = $0
      sub(/^[[:space:]]*\[/, "", current)
      sub(/\][[:space:]]*$/, "", current)
      next
    }
    current == section {
      line = $0
      sub(/[[:space:]]*#.*/, "", line)
      split(line, parts, "=")
      key = parts[1]
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", key)
      if (key != field) {
        next
      }
      value = substr(line, index(line, "=") + 1)
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", value)
      gsub(/^"|"$/, "", value)
      print value
      found = 1
      exit
    }
    END {
      if (!found) {
        exit 1
      }
    }
  ' "$file"
}

package_manifest() {
  local package="$1"
  local toml
  for toml in crates/*/Cargo.toml; do
    if [ "$(toml_value "$toml" package name 2>/dev/null || true)" = "$package" ]; then
      printf '%s\n' "$toml"
      return 0
    fi
  done
  echo "Error: no Cargo.toml found for package '$package'" >&2
  return 1
}

package_bins() {
  local file="$1"
  awk '
    /^\[\[bin\]\][[:space:]]*$/ {
      in_bin = 1
      next
    }
    /^\[/ {
      in_bin = 0
      next
    }
    in_bin {
      line = $0
      sub(/[[:space:]]*#.*/, "", line)
      split(line, parts, "=")
      key = parts[1]
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", key)
      if (key != "name") {
        next
      }
      value = substr(line, index(line, "=") + 1)
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", value)
      gsub(/^"|"$/, "", value)
      print value
    }
  ' "$file"
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

if [ ! -f Cargo.lock ]; then
  echo "Error: Cargo.lock missing" >&2
  exit 1
fi
if [ ! -f .resources/snarkos-version ]; then
  echo "Error: .resources/snarkos-version missing" >&2
  exit 1
fi

CRATE_MANIFEST="$(package_manifest "$CRATE_NAME")"
TOML_VERSION="$(toml_value "$CRATE_MANIFEST" package version)"
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
CURRENT_REF="$TAG"
if COMMIT="$(git rev-parse --verify -q "${TAG}^{commit}")"; then
  :
else
  COMMIT="$(git rev-parse HEAD)"
  CURRENT_REF="HEAD"
fi

mapfile -t TARGETS < <(target_list)
if [ "${#TARGETS[@]}" -eq 0 ]; then
  echo "Error: no release targets found in release-crate workflow" >&2
  exit 1
fi

COMPONENTS=(leo-lang leo-fmt leo-lsp)
declare -A COMPONENT_VERSIONS
declare -A COMPONENT_BINS

for component in "${COMPONENTS[@]}"; do
  manifest="$(package_manifest "$component")"
  COMPONENT_VERSIONS["$component"]="$(toml_value "$manifest" package version)"
  mapfile -t bins < <(package_bins "$manifest")
  if [ "${#bins[@]}" -eq 0 ]; then
    echo "Error: package '$component' has no [[bin]] entries" >&2
    exit 1
  fi
  COMPONENT_BINS["$component"]="$(printf '%s\n' "${bins[@]}")"
done

{
  printf '# %s v%s\n\n' "$CRATE_NAME" "$VERSION"
  printf '## Changes\n\n'

  PREVIOUS_TAG="$(previous_same_crate_tag || true)"
  if [ -n "$PREVIOUS_TAG" ]; then
    printf 'Commits since `%s`:\n\n' "$PREVIOUS_TAG"
    if git log --format='%H%x09%h%x09%s' "${PREVIOUS_TAG}..${CURRENT_REF}" | grep -q .; then
      git log --format='%H%x09%h%x09%s' "${PREVIOUS_TAG}..${CURRENT_REF}" \
        | while IFS="$(printf '\t')" read -r full short subject; do
            printf -- '- [%s](%s/commit/%s) %s\n' "$short" "$REPO_URL" "$full" "$subject"
          done
    else
      printf -- '- No commits found in this release range.\n'
    fi
  else
    printf 'This is the first tagged release for `%s`.\n' "$CRATE_NAME"
  fi

  printf '\n## Compatible Versions\n\n'
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
