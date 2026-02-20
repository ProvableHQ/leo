#!/bin/bash
# Fetch complete issue context from GitHub
# Usage: fetch-issue.sh <issue_number> [--force]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OWNER="ProvableHQ"
REPO="leo"
WS="${WS:-.claude/workspace}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }
escape_sed() { printf '%s' "$1" | sed 's/[|&\\/]/\\&/g'; }

# Parse arguments
FORCE=0
NUM=""
for arg in "$@"; do
  case "$arg" in
    --force|-f) FORCE=1 ;;
    *) [[ -z "$NUM" ]] && NUM="$arg" ;;
  esac
done

if [[ -z "$NUM" ]]; then
  echo "Usage: fetch-issue.sh <issue_number> [--force]"
  exit 1
fi

# Check gh auth
if ! gh auth status &>/dev/null; then
  log_error "gh CLI not authenticated. Run 'gh auth login' first."
  exit 1
fi

mkdir -p "$WS"

# Skip if fresh (< 1 hour old) unless forced
if [[ "$FORCE" = "0" ]] && [[ -f "$WS/context-issue-$NUM.json" ]]; then
  if [[ "$(uname)" == "Darwin" ]]; then
    FILE_AGE=$(( $(date +%s) - $(stat -f %m "$WS/context-issue-$NUM.json") ))
  else
    FILE_AGE=$(( $(date +%s) - $(stat -c %Y "$WS/context-issue-$NUM.json") ))
  fi

  if [[ $FILE_AGE -lt 3600 ]]; then
    log_warn "Context fresh ($(( FILE_AGE / 60 )) min old). Use --force to refresh."
    exit 0
  fi
fi

log_info "Fetching Issue #$NUM context..."

# Metadata + comments via GraphQL (paginated)
CURSOR=""
> "$WS/comments-issue-$NUM.jsonl"

log_info "Fetching issue metadata and comments..."
RESP=$(gh api graphql -f query='{
  repository(owner: "'"$OWNER"'", name: "'"$REPO"'") {
    issue(number: '"$NUM"') {
      title
      body
      state
      author { login }
      createdAt
      labels(first: 10) { nodes { name } }
      comments(first: 100) {
        pageInfo { hasNextPage endCursor }
        nodes { body author { login } createdAt }
      }
    }
  }
}')

# Extract main metadata
echo "$RESP" | jq '{
  title: .data.repository.issue.title,
  body: .data.repository.issue.body,
  state: .data.repository.issue.state,
  author: .data.repository.issue.author.login,
  labels: [.data.repository.issue.labels.nodes[].name]
}' > "$WS/context-issue-$NUM.json"

# Extract comments
echo "$RESP" | jq -c '.data.repository.issue.comments.nodes[]' >> "$WS/comments-issue-$NUM.jsonl" 2>/dev/null || true

HAS_NEXT=$(echo "$RESP" | jq -r '.data.repository.issue.comments.pageInfo.hasNextPage')
CURSOR=$(echo "$RESP" | jq -r '.data.repository.issue.comments.pageInfo.endCursor')

# Paginate remaining comments
while [[ "$HAS_NEXT" = "true" ]]; do
  log_info "Fetching more comments..."
  RESP=$(gh api graphql -f query='{
    repository(owner: "'"$OWNER"'", name: "'"$REPO"'") {
      issue(number: '"$NUM"') {
        comments(first: 100, after: "'"$CURSOR"'") {
          pageInfo { hasNextPage endCursor }
          nodes { body author { login } createdAt }
        }
      }
    }
  }')

  echo "$RESP" | jq -c '.data.repository.issue.comments.nodes[]' >> "$WS/comments-issue-$NUM.jsonl" 2>/dev/null || true

  HAS_NEXT=$(echo "$RESP" | jq -r '.data.repository.issue.comments.pageInfo.hasNextPage')
  CURSOR=$(echo "$RESP" | jq -r '.data.repository.issue.comments.pageInfo.endCursor')
done

# Timeline events (linked PRs, references)
log_info "Fetching timeline events..."
gh api "repos/$OWNER/$REPO/issues/$NUM/timeline" --paginate 2>/dev/null | \
  jq '[.[] | select(.event == "cross-referenced" or .event == "referenced") | {
    event,
    source: .source.issue.number,
    actor: .actor.login
  }]' > "$WS/timeline-issue-$NUM.json" || echo "[]" > "$WS/timeline-issue-$NUM.json"

# Extract linked PRs
log_info "Extracting linked PRs..."
jq -r '[.[] | select(.source != null) | .source] | unique | .[]' \
  "$WS/timeline-issue-$NUM.json" > "$WS/linked-prs-issue-$NUM.txt" 2>/dev/null || true

# Generate state file from template
log_info "Generating state file..."
TEMPLATE_DIR="$SCRIPT_DIR/../templates"

TITLE=$(jq -r .title "$WS/context-issue-$NUM.json")
AUTHOR=$(jq -r .author "$WS/context-issue-$NUM.json")
STATE=$(jq -r .state "$WS/context-issue-$NUM.json")
BODY=$(jq -r '.body[0:500]' "$WS/context-issue-$NUM.json")

BODY_ESCAPED=$(echo "$BODY" | tr '\n' ' ')

sed -e "s|{{NUM}}|$(escape_sed "$NUM")|g" \
    -e "s|{{TITLE}}|$(escape_sed "$TITLE")|g" \
    -e "s|{{AUTHOR}}|$(escape_sed "$AUTHOR")|g" \
    -e "s|{{STATE}}|$(escape_sed "$STATE")|g" \
    -e "s|{{BODY}}|$(escape_sed "$BODY_ESCAPED")|g" \
    "$TEMPLATE_DIR/state-issue.md" > "$WS/state-issue-$NUM.md"

log_info "=== Issue #$NUM ready ==="
ls -la "$WS"/*issue*"$NUM"* 2>/dev/null || true
echo ""
log_info "Files fetched: context, comments, timeline, linked-prs, state"
