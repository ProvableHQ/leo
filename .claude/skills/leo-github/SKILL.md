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

Ensure `gh` CLI is authenticated:
```bash
gh auth status || gh auth login
```

## Fetch PR Context

```bash
SKILL_DIR="$(dirname "$(readlink -f "$0")")"
"$SKILL_DIR/scripts/fetch-pr.sh" $ARGUMENTS
```

This fetches:
- **context-pr-N.json** — PR metadata (title, author, state, branch, stats)
- **files-pr-N.txt** — Changed files with +/- counts
- **crates-pr-N.txt** — Affected Leo crates (auto-detected)
- **commits-pr-N.json** — Commit history
- **comments-pr-N.json** — PR conversation comments
- **checks-pr-N.json** — CI/check run status
- **threads-pr-N.jsonl** — All review threads
- **unresolved-pr-N.json** — Unresolved review comments
- **resolved-pr-N.json** — Resolved review comments
- **linked-issues-pr-N.txt** — Issues referenced in PR body
- **state-pr-N.md** — Working state file for tracking findings

## Fetch Issue Context

```bash
SKILL_DIR="$(dirname "$(readlink -f "$0")")"
"$SKILL_DIR/scripts/fetch-issue.sh" $ARGUMENTS
```

This fetches:
- **context-issue-N.json** — Issue metadata (title, body, state, author, labels)
- **comments-issue-N.jsonl** — All comments
- **timeline-issue-N.json** — Timeline events (cross-references, linked PRs)
- **linked-prs-issue-N.txt** — PRs that reference this issue
- **state-issue-N.md** — Working state file for investigation

## Quick Refresh (PR threads only)

For faster updates when you only need current review thread status:

```bash
SKILL_DIR="$(dirname "$(readlink -f "$0")")"
"$SKILL_DIR/scripts/refresh-threads.sh" <pr_number>
```

## Caching

Context is cached for 1 hour. Use `--force` to bypass:
```bash
/leo-github pr 123 --force
```

Or delete workspace files to refresh:
```bash
rm .claude/workspace/*pr*123*
```

## Leo Crate Detection

After fetching PR files, automatically detects affected crates:

| Directory | Crate |
|-----------|-------|
| compiler/ast | leo-ast |
| compiler/compiler | leo-compiler |
| compiler/parser | leo-parser |
| compiler/parser-lossless | leo-parser-lossless |
| compiler/passes | leo-passes |
| compiler/span | leo-span |
| errors | leo-errors |
| interpreter | leo-interpreter |
| leo/package | leo-package |
| test-framework | leo-test-framework |
| utils/disassembler | leo-disassembler |

## Workspace Structure

```
.claude/workspace/
├── context-pr-123.json      # PR metadata
├── files-pr-123.txt         # Changed files
├── crates-pr-123.txt        # Affected crates
├── commits-pr-123.json      # Commits
├── comments-pr-123.json     # PR comments
├── checks-pr-123.json       # CI status
├── threads-pr-123.jsonl     # Review threads (raw)
├── unresolved-pr-123.json   # Unresolved comments
├── resolved-pr-123.json     # Resolved comments
├── linked-issues-pr-123.txt # Linked issues
├── state-pr-123.md          # Working state file
└── handoff-pr-123.md        # Review→fix handoff (if created)
```

## Integration with Other Skills

This skill provides context for:
- **leo-review** — Security-focused PR review
- **leo-fix-pr** — Fix PR review feedback
- **leo-fix** — Fix GitHub issues

Always ensure context is loaded before running those skills.
