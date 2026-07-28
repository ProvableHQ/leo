#!/usr/bin/env bash
# Shared scaffolding for `leo update` CLI tests: serves a local mock of the GitHub
# release API and provides helpers to register mock releases. `leo` is pointed at
# the mock server via the LEO_UPDATE_*_BASE_URL environment variables.
#
# Sourced from each test_update* COMMANDS via `$LEO_CLI_TESTS_DIR/update_mock.sh`.

LEO_BIN=${1}

WORK=$(mktemp -d)
# Resolve symlinks (e.g. `/var` -> `/private/var` on macOS) so the path redaction in
# `run_leo` matches the canonical paths that `leo` prints.
WORK=$(cd "$WORK" && pwd -P)

cleanup() {
    [ -n "$SERVER_PID" ] && kill "$SERVER_PID" 2>/dev/null
    rm -rf "$WORK"
}
trap cleanup EXIT

# The compilation target of the `leo` binary, used in release asset names.
case "$(uname -s)-$(uname -m)" in
    Darwin-arm64)  TARGET=aarch64-apple-darwin ;;
    Darwin-x86_64) TARGET=x86_64-apple-darwin ;;
    Linux-x86_64)  TARGET=x86_64-unknown-linux-gnu ;;
    Linux-aarch64) TARGET=aarch64-unknown-linux-gnu ;;
    *) echo "unsupported platform for leo update tests" >&2; exit 1 ;;
esac

MOCK="$WORK/mock"
API="$MOCK/repos/ProvableHQ/leo"
DOWNLOADS="$MOCK/ProvableHQ/leo/releases/download"
mkdir -p "$API/releases/tags" "$DOWNLOADS"

# A static file server for the mock GitHub API. Endpoint responses are stored as
# `<path>.json` files so a path like `repos/ProvableHQ/leo/releases` can be both
# an endpoint and a directory (`releases/tags/...`).
cat > "$WORK/server.py" <<'PYEOF'
import http.server
import os
import sys

class Handler(http.server.SimpleHTTPRequestHandler):
    def translate_path(self, path):
        path = super().translate_path(path)
        if not os.path.isfile(path) and os.path.isfile(path + ".json"):
            return path + ".json"
        return path

    def log_message(self, *args):
        pass

os.chdir(sys.argv[1])
server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Handler)
with open(sys.argv[2], "w") as port_file:
    port_file.write(str(server.server_address[1]))
server.serve_forever()
PYEOF

python3 "$WORK/server.py" "$MOCK" "$WORK/port" >/dev/null 2>&1 &
SERVER_PID=$!
# Detach the server job so bash doesn't print a termination notice when it is killed.
disown
for _ in $(seq 1 100); do
    [ -s "$WORK/port" ] && break
    sleep 0.1
done
BASE="http://localhost:$(cat "$WORK/port")"

export LEO_UPDATE_API_BASE_URL="$BASE"
export LEO_UPDATE_DOWNLOAD_BASE_URL="$BASE"
# Sandbox the update-check cache (`~/.aleo/leo`) away from the real home directory.
export HOME="$WORK/home"
mkdir -p "$HOME"

RELEASES=()

# release_json <tag> <asset-name> – a GitHub API release object with a single asset.
release_json() {
    cat <<EOF
{"tag_name": "$1", "name": "$1", "created_at": "2026-01-01T00:00:00Z",
 "assets": [{"name": "$2", "url": "$BASE/ProvableHQ/leo/releases/download/$1/$2"}]}
EOF
}

# add_release <tag> <bin-name> <bin-content> – registers a release whose archive
# contains a fake binary that prints <bin-content>.
add_release() {
    local tag=$1 bin=$2 content=$3
    local asset="$tag-$TARGET.zip"
    mkdir -p "$DOWNLOADS/$tag"
    python3 - "$DOWNLOADS/$tag/$asset" "$bin" "$content" <<'PYEOF'
import sys
import zipfile

with zipfile.ZipFile(sys.argv[1], "w") as archive:
    archive.writestr(sys.argv[2], "#!/bin/sh\necho '%s'\n" % sys.argv[3])
PYEOF
    release_json "$tag" "$asset" > "$API/releases/tags/$tag.json"
    RELEASES+=("$(release_json "$tag" "$asset")")
}

# add_release_foreign <tag> – registers a release whose only asset is for another platform.
add_release_foreign() {
    RELEASES+=("$(release_json "$1" "$1-riscv64-unknown-none.zip")")
}

# Write the registered releases to the list endpoint, in registration order.
write_release_list() {
    local IFS=,
    printf '[%s]' "${RELEASES[*]}" > "$API/releases.json"
}

# Forget all registered releases and their archives.
reset_releases() {
    RELEASES=()
    rm -rf "$API/releases" "$API/releases.json" "$DOWNLOADS"
    mkdir -p "$API/releases/tags" "$DOWNLOADS"
}

# Place a fresh copy of the real `leo` binary in the work directory.
fresh_leo() {
    cp "$LEO_BIN" "$WORK/leo"
}

# Run the work-directory copy of `leo`, redacting the work directory from its output.
run_leo_impl() {
    "$WORK/leo" "$@" > "$WORK/stdout.txt" 2> "$WORK/stderr.txt"
    local code=$?
    sed "s|$WORK|WORKDIR|g" "$WORK/stdout.txt"
    sed "s|$WORK|WORKDIR|g" "$WORK/stderr.txt" >&2
    return $code
}

run_leo() {
    run_leo_impl --disable-update-check "$@"
}

run_leo_with_update_check() {
    run_leo_impl "$@"
}
