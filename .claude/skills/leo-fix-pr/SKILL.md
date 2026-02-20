---
name: leo-fix-pr
description: |
  Fix PR review feedback in Leo using TDD workflow.
  WHEN: User says "fix pr", "fix PR feedback", "address review comments",
  "resolve review threads", or wants to address reviewer requests on a PR.
  WHEN NOT: Creating new fixes for issues (use leo-fix), doing security
  review (use leo-review), or initial PR creation.
allowed-tools: Bash, Read, Write, Grep, Glob, Task, AskUserQuestion
---

# Fix PR Review Feedback

Address PR review comments in Leo using test-driven development.

## Usage

```
/leo-fix-pr <pr_number>
```

## Setup

```bash
PR=$ARGUMENTS
WS=".claude/workspace"
SKILL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../leo-github" && pwd)"
```

## 1. Load Context

```bash
if [[ ! -f "$WS/state-pr-$PR.md" ]]; then
  "$SKILL_DIR/scripts/fetch-pr.sh" "$PR"
fi
"$SKILL_DIR/scripts/refresh-threads.sh" "$PR"
```

Review: `$WS/state-pr-$PR.md`, `$WS/unresolved-pr-$PR.json`, and `$WS/handoff-pr-$PR.md` if present.

## 2. Analyze Each Comment

For each unresolved comment, determine:

| # | Path:Line | Reviewer | Request | Concern Type | Risk |
|---|-----------|----------|---------|--------------|------|

**Concern types:** Correctness, Performance, Security, Style, Clarity.

Update `$WS/state-pr-$PR.md` with analysis.

**Think hard. Do not proceed until you understand each request.**

## 3. Plan (APPROVAL REQUIRED)

Present a fix plan:

| # | Location | Request | Proposed Fix | Risk |
|---|----------|---------|--------------|------|

**Use AskUserQuestion to get explicit approval before proceeding.**

## 4. Implement Fixes (TDD)

### 4.1 Fix each comment

For each fix:
1. **Write/update test** (if behavioral change)
2. **Verify test fails** (for new behavior)
3. **Make minimal change** — match existing code style
4. **Verify fix works**
5. **Log to state file**

**By concern type:** For correctness/security, always write a test first. For performance, verify no regression. For style/nit, batch per section 4.2.

### 4.2 Batch style/nit fixes

For pure style changes (no behavioral impact), batch them:
```bash
cargo clippy -p <crate> --fix --allow-dirty
```

## 5. Final Validation

Validate affected crates per AGENTS.md.

## 6. Report

| # | Comment | Resolution | Verified |
|---|---------|------------|----------|
| 1 | "Add bounds check" | Added guard at file.rs:42 | cargo test |

**Files changed:**
- path/to/file.rs — [changes]
- path/to/test.rs — [new/updated tests]

**Do not commit unless explicitly asked.**

## Handling Disagreements

If you disagree with a review comment:

1. Explain your reasoning clearly
2. Provide evidence (benchmarks, tests, spec references)
3. Propose an alternative if applicable
4. **Use AskUserQuestion** to discuss with user before proceeding

Never silently ignore review feedback.
