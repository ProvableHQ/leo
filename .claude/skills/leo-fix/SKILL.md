---
name: leo-fix
description: |
  Fix GitHub issues in Leo using TDD workflow.
  WHEN: User says "fix issue", "fix #123", "address issue", "implement fix for issue",
  or wants to resolve a bug/feature request from GitHub issues.
  WHEN NOT: Fixing PR review feedback (use leo-fix-pr), doing security review
  (use leo-review), or working on non-Leo code.
allowed-tools: Bash, Read, Write, Grep, Glob, Task, AskUserQuestion
---

# Fix GitHub Issue

Fix Leo GitHub issues using test-driven development.

## Usage

```
/leo-fix <issue_number>
```

## Setup

```bash
ISSUE=$ARGUMENTS
WS=".claude/workspace"
SKILL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../leo-github" && pwd)"
```

## 1. Load Context

```bash
if [[ ! -f "$WS/state-issue-$ISSUE.md" ]]; then
  "$SKILL_DIR/scripts/fetch-issue.sh" "$ISSUE"
fi
```

Review: `$WS/state-issue-$ISSUE.md` and recent comments.

## 2. Investigate

Search for related code. Answer:
1. Can you reproduce the issue?
2. What is the expected vs actual behavior?
3. Where does the code path go wrong?
4. What are the edge cases?

**Where to look:**
- Parser bugs → `compiler/parser-lossless/` (grammar), `compiler/parser/` (AST conversion)
- Type errors → type-checking pass in `compiler/passes/`
- Code generation bugs → `compiler/passes/code_generation/`
- Pass ordering → `compiler/compiler/src/compiler.rs` (`intermediate_passes`)
- Span bugs → trace span preservation through transformations

Update `$WS/state-issue-$ISSUE.md` with root cause analysis, relevant code locations, and evidence.

**Think hard. Do not proceed until you can explain the root cause.**

## 3. Plan (APPROVAL REQUIRED)

Present a concrete plan:

| Aspect | Details |
|--------|---------|
| **Root cause** | [specific explanation] |
| **Fix** | [specific code changes] |
| **Files** | path/to/file.rs — change X to Y |
| **Tests** | test_name — verifies Z |
| **Risk** | Low / Medium / High |

**Use AskUserQuestion to get explicit approval before proceeding.**

## 4. Implement (TDD)

### 4.1 Write failing test first

Create a test that reproduces the issue and covers edge cases.

```bash
cargo test -p <crate> test_issue_NNNN -- --nocapture  # Verify it fails
```

### 4.2 Implement minimal fix

- Match existing code style
- Make the smallest change that fixes the issue

### 4.3 Verify

```bash
cargo test -p <crate> test_issue_NNNN -- --nocapture  # Verify it passes
```

### 4.4 Log progress

Update `$WS/state-issue-$ISSUE.md`:

| Action | Result |
|--------|--------|
| Wrote test | test_issue_NNNN in path/to/test.rs |
| Applied fix | Changed X to Y in path/to/file.rs |
| Verified | Test passes |

## 5. Final Validation

Validate affected crates per AGENTS.md.

## 6. Report

```
**Issue**: #$ISSUE — [title]
**Root cause**: [brief explanation]
**Fix**: [what changed and why]
**Test**: [what the test verifies]
**Files changed**:
- path/to/file.rs — [change description]
- path/to/test.rs — [new test]
```

**Do not commit unless explicitly asked.**
