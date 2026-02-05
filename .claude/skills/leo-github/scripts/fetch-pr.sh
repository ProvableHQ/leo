#!/bin/bash
# Fetch PR context into workspace
set -e

NUM=$1
OWNER="ProvableHQ"
REPO="leo"
WS=".claude/workspace"

[ -z "$NUM" ] && echo "Usage: fetch-pr.sh <number>" && exit 1
mkdir -p "$WS"

echo "Fetching PR #$NUM..."

# Metadata
gh pr view "$NUM" --json title,body,author,state,headRefName,baseRefName,additions,deletions,changedFiles,labels,reviews,mergeable,createdAt,updatedAt > "$WS/context-pr-$NUM.json"

# Files
gh pr view "$NUM" --json files --jq '.files[] | "\(.additions)+/\(.deletions)- \(.path)"' > "$WS/files-pr-$NUM.txt"

# Commits (paginated)
gh api "repos/$OWNER/$REPO/pulls/$NUM/commits" --paginate | jq '[.[] | {sha: .sha[0:7], message: .commit.message | split("\n")[0]}]' > "$WS/commits-pr-$NUM.json"

# PR comments (paginated)
gh api "repos/$OWNER/$REPO/issues/$NUM/comments" --paginate | jq '[.[] | {author: .user.login, date: .created_at[0:10], body: .body}]' > "$WS/comments-pr-$NUM.json"

# Check runs / CI status
gh pr checks "$NUM" --json name,state,conclusion 2>/dev/null | jq '[.[] | {name, state, conclusion}]' > "$WS/checks-pr-$NUM.json" || echo "[]" > "$WS/checks-pr-$NUM.json"

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

# Linked issues (parse from body)
jq -r '.body // ""' "$WS/context-pr-$NUM.json" | grep -oE '#[0-9]+|issues/[0-9]+|ProvableHQ/leo/issues/[0-9]+' | grep -oE '[0-9]+' | sort -u > "$WS/linked-issues-pr-$NUM.txt" 2>/dev/null || true

echo "PR #$NUM fetched successfully"
