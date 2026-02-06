#!/bin/bash
# Quick refresh of PR review threads (faster than full fetch)
# Usage: refresh-threads.sh <pr_number>

set -euo pipefail

OWNER="ProvableHQ"
REPO="leo"
WS="${WS:-.claude/workspace}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Validate arguments
if [[ $# -lt 1 ]]; then
  echo "Usage: refresh-threads.sh <pr_number>"
  exit 1
fi

PR="$1"

# Temp files keyed by PR number to avoid collisions.
QUERY_FILE="/tmp/gh-query-$PR.txt"
RESP_FILE="/tmp/gh-threads-page-$PR.json"

# Clean up temp files on exit.
cleanup() { rm -f "$QUERY_FILE" "$RESP_FILE"; }
trap cleanup EXIT

# Check gh auth
if ! gh auth status &>/dev/null; then
  log_error "gh CLI not authenticated. Run 'gh auth login' first."
  exit 1
fi

mkdir -p "$WS"

log_info "Refreshing review threads for PR #$PR..."

# Fetch review threads with pagination using temp files.
CURSOR=""
> "$WS/threads-pr-$PR.jsonl"

while true; do
  if [ -n "$CURSOR" ]; then
    AFTER=", after: \"$CURSOR\""
  else
    AFTER=""
  fi

  # Write query to temp file to avoid shell variable buffer issues.
  cat > "$QUERY_FILE" << QUERYEOF
{
  repository(owner: "$OWNER", name: "$REPO") {
    pullRequest(number: $PR) {
      reviewThreads(first: 20${AFTER}) {
        pageInfo { hasNextPage endCursor }
        nodes {
          isResolved isOutdated path line startLine diffSide startDiffSide
          comments(first: 100) {
            pageInfo { hasNextPage }
            nodes { body author { login } createdAt updatedAt outdated }
          }
        }
      }
    }
  }
}
QUERYEOF

  # Write response to temp file instead of shell variable.
  gh api graphql -f query="$(cat "$QUERY_FILE")" > "$RESP_FILE" 2>&1

  jq -c '.data.repository.pullRequest.reviewThreads.nodes[]' "$RESP_FILE" >> "$WS/threads-pr-$PR.jsonl" 2>/dev/null

  HAS_NEXT=$(jq -r '.data.repository.pullRequest.reviewThreads.pageInfo.hasNextPage' "$RESP_FILE" 2>/dev/null)
  [[ "$HAS_NEXT" != "true" ]] && break

  CURSOR=$(jq -r '.data.repository.pullRequest.reviewThreads.pageInfo.endCursor' "$RESP_FILE")
done

# Extract unresolved threads grouped by path with full comment chains.
jq -s '
  [.[] | select(.isResolved==false)] |
  group_by(.path) |
  map({key: .[0].path, value: [.[] | {
    line, startLine, isOutdated,
    comments: [.comments.nodes[] | {author: .author.login, body, createdAt}]
  }]}) |
  from_entries
' "$WS/threads-pr-$PR.jsonl" > "$WS/unresolved-pr-$PR.json"

# Extract resolved threads grouped by path with full comment chains.
jq -s '
  [.[] | select(.isResolved==true)] |
  group_by(.path) |
  map({key: .[0].path, value: [.[] | {
    line, startLine, isOutdated,
    comments: [.comments.nodes[] | {author: .author.login, body, createdAt}]
  }]}) |
  from_entries
' "$WS/threads-pr-$PR.jsonl" > "$WS/resolved-pr-$PR.json"

# Compute counts.
TOTAL=$(jq -s 'length' "$WS/threads-pr-$PR.jsonl")
UNRESOLVED=$(jq '[.[]] | add // [] | length' "$WS/unresolved-pr-$PR.json")
RESOLVED=$(jq '[.[]] | add // [] | length' "$WS/resolved-pr-$PR.json")

log_info "Review threads: $UNRESOLVED unresolved / $TOTAL total ($RESOLVED resolved)"

# Output summary for caller
echo "TOTAL=$TOTAL"
echo "UNRESOLVED=$UNRESOLVED"
echo "RESOLVED=$RESOLVED"
