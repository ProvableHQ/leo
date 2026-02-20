#!/bin/bash
# Fetch complete PR context from GitHub
# Usage: fetch-pr.sh <pr_number> [--force]

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
  echo "Usage: fetch-pr.sh <pr_number> [--force]"
  exit 1
fi

# Check gh auth
if ! gh auth status &>/dev/null; then
  log_error "gh CLI not authenticated. Run 'gh auth login' first."
  exit 1
fi

mkdir -p "$WS"

# Skip if fresh (< 1 hour old) unless forced
if [[ "$FORCE" = "0" ]] && [[ -f "$WS/context-pr-$NUM.json" ]]; then
  if [[ "$(uname)" == "Darwin" ]]; then
    FILE_AGE=$(( $(date +%s) - $(stat -f %m "$WS/context-pr-$NUM.json") ))
  else
    FILE_AGE=$(( $(date +%s) - $(stat -c %Y "$WS/context-pr-$NUM.json") ))
  fi

  if [[ $FILE_AGE -lt 3600 ]]; then
    log_warn "Context fresh ($(( FILE_AGE / 60 )) min old). Use --force to refresh."
    exit 0
  fi
fi

log_info "Fetching PR #$NUM context..."

# Metadata
log_info "Fetching metadata..."
gh pr view "$NUM" --json title,body,author,state,headRefName,baseRefName,additions,deletions,changedFiles,labels,reviews,mergeable,createdAt,updatedAt > "$WS/context-pr-$NUM.json"

# Files
log_info "Fetching changed files..."
gh pr view "$NUM" --json files --jq '.files[] | "\(.additions)+/\(.deletions)- \(.path)"' > "$WS/files-pr-$NUM.txt"

# Commits (paginated)
log_info "Fetching commits..."
gh api "repos/$OWNER/$REPO/pulls/$NUM/commits" --paginate | \
  jq '[.[] | {sha: .sha[0:7], message: .commit.message | split("\n")[0]}]' > "$WS/commits-pr-$NUM.json"

# PR comments (paginated)
log_info "Fetching comments..."
gh api "repos/$OWNER/$REPO/issues/$NUM/comments" --paginate | \
  jq '[.[] | {author: .user.login, date: .created_at[0:10], body: .body}]' > "$WS/comments-pr-$NUM.json"

# Check runs / CI status
log_info "Fetching CI status..."
gh pr checks "$NUM" --json name,state,conclusion 2>/dev/null | \
  jq '[.[] | {name, state, conclusion}]' > "$WS/checks-pr-$NUM.json" || echo "[]" > "$WS/checks-pr-$NUM.json"

# Review threads (uses shared script)
log_info "Fetching review threads..."
source "$SCRIPT_DIR/refresh-threads.sh" "$NUM"

# Linked issues (parse from body)
log_info "Extracting linked issues..."
jq -r '.body // ""' "$WS/context-pr-$NUM.json" | \
  grep -oE '#[0-9]+|issues/[0-9]+|ProvableHQ/leo/issues/[0-9]+' | \
  grep -oE '[0-9]+' | sort -u > "$WS/linked-issues-pr-$NUM.txt" || true

# Compute counts for state file
TOTAL_THREADS=$(jq -s 'length' "$WS/threads-pr-$NUM.jsonl")
UNRESOLVED_COUNT=$(jq 'length' "$WS/unresolved-pr-$NUM.json")
RESOLVED_COUNT=$(jq 'length' "$WS/resolved-pr-$NUM.json")

# Generate state file from template
log_info "Generating state file..."
TEMPLATE_DIR="$SCRIPT_DIR/../templates"

TITLE=$(jq -r .title "$WS/context-pr-$NUM.json")
AUTHOR=$(jq -r .author.login "$WS/context-pr-$NUM.json")
BRANCH=$(jq -r .headRefName "$WS/context-pr-$NUM.json")
STATS=$(jq -r '"\(.additions)+/\(.deletions)-/\(.changedFiles) files"' "$WS/context-pr-$NUM.json")
CI_STATUS=$(jq -r 'if length == 0 then "none" else [.[] | "\(.name):\(.conclusion // .state)"] | join(", ") end' "$WS/checks-pr-$NUM.json")
sed -e "s|{{NUM}}|$(escape_sed "$NUM")|g" \
    -e "s|{{TITLE}}|$(escape_sed "$TITLE")|g" \
    -e "s|{{AUTHOR}}|$(escape_sed "$AUTHOR")|g" \
    -e "s|{{BRANCH}}|$(escape_sed "$BRANCH")|g" \
    -e "s|{{STATS}}|$(escape_sed "$STATS")|g" \
    -e "s|{{UNRESOLVED}}|$(escape_sed "$UNRESOLVED_COUNT")|g" \
    -e "s|{{TOTAL}}|$(escape_sed "$TOTAL_THREADS")|g" \
    -e "s|{{RESOLVED}}|$(escape_sed "$RESOLVED_COUNT")|g" \
    -e "s|{{CI_STATUS}}|$(escape_sed "$CI_STATUS")|g" \
    "$TEMPLATE_DIR/state-pr.md" > "$WS/state-pr-$NUM.md"

log_info "=== PR #$NUM ready ==="
ls -la "$WS"/*pr*"$NUM"* 2>/dev/null || true
echo ""
log_info "Files fetched: context, files, commits, comments, checks, threads, unresolved, resolved, linked-issues, state"
