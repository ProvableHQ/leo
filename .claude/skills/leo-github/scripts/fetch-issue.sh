#!/bin/bash
# Fetch issue context into workspace
set -e

NUM=$1
OWNER="ProvableHQ"
REPO="leo"
WS=".claude/workspace"

[ -z "$NUM" ] && echo "Usage: fetch-issue.sh <number>" && exit 1
mkdir -p "$WS"

echo "Fetching issue #$NUM..."

# Metadata + comments (paginated GraphQL)
CURSOR=""
> "$WS/comments-issue-$NUM.jsonl"

RESP=$(gh api graphql -f query='{
  repository(owner: "'"$OWNER"'", name: "'"$REPO"'") {
    issue(number: '"$NUM"') {
      title body state author { login } createdAt labels(first: 10) { nodes { name } }
      comments(first: 100) { pageInfo { hasNextPage endCursor } nodes { body author { login } createdAt } }
    }
  }
}')

echo "$RESP" | jq '{title: .data.repository.issue.title, body: .data.repository.issue.body, state: .data.repository.issue.state, author: .data.repository.issue.author.login, labels: [.data.repository.issue.labels.nodes[].name]}' > "$WS/context-issue-$NUM.json"
echo "$RESP" | jq -c '.data.repository.issue.comments.nodes[]' >> "$WS/comments-issue-$NUM.jsonl" 2>/dev/null || true
HAS_NEXT=$(echo "$RESP" | jq -r '.data.repository.issue.comments.pageInfo.hasNextPage')
CURSOR=$(echo "$RESP" | jq -r '.data.repository.issue.comments.pageInfo.endCursor')

while [ "$HAS_NEXT" = "true" ]; do
  RESP=$(gh api graphql -f query='{
    repository(owner: "'"$OWNER"'", name: "'"$REPO"'") {
      issue(number: '"$NUM"') {
        comments(first: 100, after: "'"$CURSOR"'") { pageInfo { hasNextPage endCursor } nodes { body author { login } createdAt } }
      }
    }
  }')
  echo "$RESP" | jq -c '.data.repository.issue.comments.nodes[]' >> "$WS/comments-issue-$NUM.jsonl" 2>/dev/null || true
  HAS_NEXT=$(echo "$RESP" | jq -r '.data.repository.issue.comments.pageInfo.hasNextPage')
  CURSOR=$(echo "$RESP" | jq -r '.data.repository.issue.comments.pageInfo.endCursor')
done

# Timeline events (linked PRs, references)
gh api "repos/$OWNER/$REPO/issues/$NUM/timeline" --paginate 2>/dev/null | jq '[.[] | select(.event == "cross-referenced" or .event == "referenced") | {event, source: .source.issue.number, actor: .actor.login}]' > "$WS/timeline-issue-$NUM.json" || echo "[]" > "$WS/timeline-issue-$NUM.json"

# Extract linked PRs
jq -r '[.[] | select(.source != null) | .source] | unique | .[]' "$WS/timeline-issue-$NUM.json" > "$WS/linked-prs-issue-$NUM.txt" 2>/dev/null || true

echo "Issue #$NUM fetched successfully"
