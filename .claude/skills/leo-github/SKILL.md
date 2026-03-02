---
name: leo-github
description: |
  Fetch GitHub context (PR or issue) into workspace for Leo development.
  WHEN: User says "fetch pr", "fetch issue", "get PR context", "load issue",
  mentions PR/issue numbers like "#123" or "PR 456", or needs GitHub context
  before reviewing/fixing.
  WHEN NOT: User already has fresh context loaded, or is asking about
  non-Leo repositories.
allowed-tools: Bash, Read, Write
---

# Leo GitHub Context

Fetch PR or issue context from ProvableHQ/leo into `.claude/workspace/`.

## Usage

```
/leo-github pr <number> [--force]
/leo-github issue <number> [--force]
```

## Prerequisites

```bash
gh auth status || gh auth login
```

## Fetch PR Context

```bash
SKILL_DIR="$(dirname "$(readlink -f "$0")")"
"$SKILL_DIR/scripts/fetch-pr.sh" $ARGUMENTS
```

Produces: `context-pr-N.json`, `files-pr-N.txt`, `commits-pr-N.json`, `comments-pr-N.json`, `checks-pr-N.json`, `threads-pr-N.jsonl`, `unresolved-pr-N.json`, `resolved-pr-N.json`, `linked-issues-pr-N.txt`, `state-pr-N.md`

## Fetch Issue Context

```bash
SKILL_DIR="$(dirname "$(readlink -f "$0")")"
"$SKILL_DIR/scripts/fetch-issue.sh" $ARGUMENTS
```

Produces: `context-issue-N.json`, `comments-issue-N.jsonl`, `timeline-issue-N.json`, `linked-prs-issue-N.txt`, `state-issue-N.md`

## Quick Refresh (PR threads only)

```bash
SKILL_DIR="$(dirname "$(readlink -f "$0")")"
"$SKILL_DIR/scripts/refresh-threads.sh" <pr_number>
```

## Caching

Context is cached for 1 hour. Use `--force` to bypass.

## Integration

This skill provides context for **leo-review**, **leo-fix-pr**, and **leo-fix**. Those skills auto-fetch if context is missing.
