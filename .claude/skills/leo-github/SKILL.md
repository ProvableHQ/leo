---
name: leo-github
version: 1.0.0
description: Fetch GitHub PR or issue context into workspace
user-invocable: true
tools:
  - Bash
  - Read
  - Write
arguments:
  - name: type
    description: "pr or issue"
    required: true
  - name: number
    description: "PR or issue number"
    required: true
---

# Leo GitHub Context Fetcher

Fetch PR or issue context into `.claude/workspace/` for other skills.

## Usage

```
/leo-github pr <number>
/leo-github issue <number>
```

## What It Does

### For PRs

Fetches and stores:
- **context-pr-N.json**: Metadata (title, author, state, stats, labels)
- **files-pr-N.txt**: Changed files with +/- lines
- **commits-pr-N.json**: Commit history
- **comments-pr-N.json**: PR comments (paginated)
- **checks-pr-N.json**: CI status
- **threads-pr-N.jsonl**: Review threads (paginated)
- **unresolved-pr-N.json**: Unresolved review comments
- **linked-issues-pr-N.txt**: Referenced issues
- **state-pr-N.md**: Human-readable state file

### For Issues

Fetches and stores:
- **context-issue-N.json**: Metadata (title, body, state, labels)
- **comments-issue-N.jsonl**: Comments (paginated)
- **timeline-issue-N.json**: Timeline events (linked PRs)
- **linked-prs-issue-N.txt**: Referenced PRs
- **state-issue-N.md**: Human-readable state file

## Caching

Context is cached for 1 hour. Delete `workspace/*{type}*{number}*` to force refresh.

## Leo Crate Detection

After fetching, detects affected crates from file paths:

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

## Instructions

<instructions>
Parse arguments: `$ARGUMENTS` contains "pr <number>" or "issue <number>".

1. **Setup**
   ```bash
   TYPE=$(echo "$ARGUMENTS" | awk '{print $1}')
   NUM=$(echo "$ARGUMENTS" | awk '{print $2}')
   ```
   Validate: TYPE must be "pr" or "issue", NUM must be numeric.

2. **Check freshness**: If `context-{type}-{num}.json` exists and is less than 1 hour old, report cached and exit.

3. **Run fetch script**: Execute `.claude/skills/leo-github/scripts/fetch-{type}.sh $NUM`

4. **Detect crates**: Run `.claude/skills/leo-github/scripts/detect-crates.sh $NUM` (for PRs)

5. **Report**: List fetched files and detected crates.

If any script fails, report the error and suggest checking `gh auth status`.
</instructions>
