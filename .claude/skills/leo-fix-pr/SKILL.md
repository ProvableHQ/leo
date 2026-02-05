---
name: leo-fix-pr
version: 1.0.0
description: Fix PR review feedback with TDD
user-invocable: true
tools:
  - Bash
  - Read
  - Write
  - Edit
  - Grep
  - Glob
  - Task
  - AskUserQuestion
arguments:
  - name: pr
    description: "PR number"
    required: true
---

# Leo PR Feedback Fixer

Address PR review comments using test-driven development.

## Usage

```
/leo-fix-pr <pr-number>
```

## Prerequisites

Run `/leo-github pr <number>` first to fetch context.

## Workflow

1. **Context** - Read PR state and unresolved comments
2. **Analyze** - Understand each reviewer request
3. **Plan** - Present fix plan for approval
4. **Implement** - TDD: update tests, fix, verify
5. **Validate** - Run check, clippy, fmt, test
6. **Report** - Summarize resolutions

## Instructions

<instructions>
Parse arguments: `$ARGUMENTS` contains the PR number.

## Phase 1: Context

Check for context files:
```bash
PR=$ARGUMENTS
WS=".claude/workspace"
ls -la "$WS"/*pr*$PR* 2>/dev/null || echo "No context found"
```

If missing, instruct user to run `/leo-github pr $PR` first.

Read state and unresolved comments:
```bash
cat "$WS/state-pr-$PR.md"
cat "$WS/unresolved-pr-$PR.json" | jq -r '.[] | "- \(.path):\(.line) [\(.reviewer)]: \(.comment[0:100])..."'
[ -f "$WS/handoff-pr-$PR.md" ] && cat "$WS/handoff-pr-$PR.md"
```

## Phase 2: Analyze

For each unresolved comment, determine:
1. What is the request?
2. What's the concern? (Correctness / Performance / Security / Style)
3. What could break?
4. Risk level?

Update `$WS/state-pr-$PR.md` with analysis.

**Think hard. Do not proceed until you understand each request.**

## Phase 3: Plan (APPROVAL REQUIRED)

Present plan as table:

| # | Location | Request | Fix | Risk |
|---|----------|---------|-----|------|

**Use AskUserQuestion to get approval before implementing.**

## Phase 4: Implement

Establish baseline first:
```bash
cargo check -p <crate> && cargo clippy -p <crate> -- -D warnings && cargo test -p <crate> --lib
```

For each fix:
1. Write/update test (should fail if applicable)
2. Make minimal change (match existing style exactly)
3. Verify: `cargo check && cargo clippy && cargo test`
4. Log progress to state file

For parser/compiler tests:
```bash
REWRITE_EXPECTATIONS=1 cargo test -p leo-parser  # Update expectations
TEST_FILTER=test_name cargo test                  # Run specific test
```

## Phase 5: Validate

Run full validation:
```bash
cargo check -p <crate>
cargo clippy -p <crate> -- -D warnings
cargo +nightly fmt --check
cargo test -p <crate>
```

## Phase 6: Report

Output resolution table:

| # | Comment | Resolution | Verified |
|---|---------|------------|----------|

Do not commit unless explicitly asked.
</instructions>

## Handling Different Feedback Types

### Correctness Issues
- Add/update test that exposes the issue
- Fix the logic
- Verify test passes

### Performance Issues
- Profile if possible
- Apply fix (pre-allocation, iterator usage, avoid clone)
- Verify no regression in tests

### Security Issues
- Identify trust boundary
- Add validation
- Add test for malicious input

### Style Issues
- Match existing patterns exactly
- Run `cargo +nightly fmt`
- Verify clippy passes
