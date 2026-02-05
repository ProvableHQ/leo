#!/bin/bash
# Refresh review threads for a PR (useful after new comments)
set -e

NUM=$1
OWNER="ProvableHQ"
REPO="leo"
WS=".claude/workspace"

[ -z "$NUM" ] && echo "Usage: refresh-threads.sh <number>" && exit 1

echo "Refreshing threads for PR #$NUM..."

# Review threads (paginated GraphQL)
CURSOR=""
> "$WS/threads-pr-$NUM.jsonl"
while true; do
  RESP=$(gh api graphql -f query='{
    repository(owner: "'"$OWNER"'", name: "'"$REPO"'") {
      pullRequest(number: '"$NUM"') {
        reviewThreads(first: 100'"${CURSOR:+, after: \"$CURSOR\"}"') {
          pageInfo { hasNextPage endCursor }
          nodes { isResolved path line comments(first: 100) { nodes { body author { login } } } }
        }
      }
    }
  }')
  echo "$RESP" | jq -c '.data.repository.pullRequest.reviewThreads.nodes[]' >> "$WS/threads-pr-$NUM.jsonl" 2>/dev/null || true
  [ "$(echo "$RESP" | jq -r '.data.repository.pullRequest.reviewThreads.pageInfo.hasNextPage')" != "true" ] && break
  CURSOR=$(echo "$RESP" | jq -r '.data.repository.pullRequest.reviewThreads.pageInfo.endCursor')
done

# Extract unresolved
jq -s '[.[] | select(.isResolved==false) | {path, line, reviewer: .comments.nodes[0].author.login, comment: .comments.nodes[0].body[0:200]}]' "$WS/threads-pr-$NUM.jsonl" > "$WS/unresolved-pr-$NUM.json" 2>/dev/null || echo "[]" > "$WS/unresolved-pr-$NUM.json"

UNRESOLVED=$(jq length "$WS/unresolved-pr-$NUM.json")
echo "Threads refreshed. $UNRESOLVED unresolved comments."
