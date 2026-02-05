---
name: leo-fix
version: 1.0.0
description: Fix GitHub issue with TDD
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
  - name: issue
    description: "Issue number"
    required: true
---

# Leo Issue Fixer

Fix GitHub issues using test-driven development.

## Usage

```
/leo-fix <issue-number>
```

## Prerequisites

Run `/leo-github issue <number>` first to fetch context.

## Workflow

1. **Context** - Read issue state and comments
2. **Investigate** - Find root cause
3. **Plan** - Present fix plan for approval
4. **Implement** - TDD: write failing test, fix, verify
5. **Validate** - Run check, clippy, fmt, test
6. **Report** - Summarize changes

## Instructions

<instructions>
Parse arguments: `$ARGUMENTS` contains the issue number.

## Phase 1: Context

Check for context files:
```bash
ISSUE=$ARGUMENTS
WS=".claude/workspace"
ls -la "$WS"/*issue*$ISSUE* 2>/dev/null || echo "No context found"
```

If missing, instruct user to run `/leo-github issue $ISSUE` first.

Read state:
```bash
cat "$WS/state-issue-$ISSUE.md"
cat "$WS/comments-issue-$ISSUE.jsonl" | jq -r '"[\(.author.login)]: \(.body[0:150])..."' | head -10
```

## Phase 2: Investigate

Search for related code using Grep and Glob tools.

Answer these questions (update state file with findings):
- Can you reproduce the issue?
- Expected vs actual behavior?
- Where does the code path go wrong?
- Which crate/pass is affected?

**Think hard. Do not proceed until you can explain the root cause.**

## Phase 3: Plan (APPROVAL REQUIRED)

Present plan to user:
- **Root cause**: [specific explanation]
- **Fix**: [specific changes]
- **Crate**: leo-{ast,parser,passes,compiler,...}
- **Files**: path/to/file.rs - change X to Y
- **Tests**: test_name - verifies Z
- **Risk**: Low/Med/High

**Use AskUserQuestion to get approval before implementing.**

## Phase 4: Implement

Establish baseline first:
```bash
cargo check -p <crate> && cargo clippy -p <crate> -- -D warnings && cargo test -p <crate> --lib
```

TDD workflow:
1. Write failing test (use test-framework expectations if applicable)
2. Make minimal fix (match existing style exactly)
3. Verify test passes
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

Output summary:
- **Issue**: #$ISSUE - [title]
- **Root cause**: [brief]
- **Fix**: [what changed]
- **Test**: [what it verifies]

Do not commit unless explicitly asked.
</instructions>

## Leo Memory Patterns

When investigating Leo issues:

- **Parser bugs**: Check `compiler/parser-lossless/` grammar, `compiler/parser/` AST conversion
- **Type errors**: Check type-checking pass, AST type nodes
- **Code generation bugs**: Check `compiler/passes/code_generation/`, Aleo instruction output
- **Pass ordering issues**: Check `compiler/src/run.rs` pass sequence
- **Span bugs**: Check span preservation through transformations

## Risk Assessment

| Risk | Areas |
|------|-------|
| HIGH | passes/ (code_generation, type_checking, flattening), ast/types, parser |
| MEDIUM | ast/, compiler/, interpreter/, errors/ |
| LOW | test-framework/, package/, docs |
