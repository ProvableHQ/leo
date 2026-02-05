---
name: leo-review
version: 1.0.0
description: Security-focused PR review
user-invocable: true
tools:
  - Bash
  - Read
  - Write
  - Grep
  - Glob
  - Task
arguments:
  - name: pr
    description: "PR number"
    required: true
---

# Leo PR Reviewer

Security-focused code review for Leo compiler PRs.

**Assume there is a bug. Find it.**

## Usage

```
/leo-review <pr-number>
```

## Prerequisites

Run `/leo-github pr <number>` first to fetch context.

## Workflow

1. **Context** - Read PR state and file list
2. **Triage** - Categorize files by risk
3. **Understand** - Grasp the change's purpose
4. **Analyze** - Deep-dive each file
5. **Verify** - Run validation commands
6. **Report** - Output findings table
7. **Handoff** - Write handoff file if fixes needed

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

Read state and files:
```bash
cat "$WS/state-pr-$PR.md"
cat "$WS/files-pr-$PR.txt"
cat "$WS/crates-pr-$PR.txt" 2>/dev/null || echo "No crates detected"
```

## Phase 2: Triage

Categorize files by risk level:

**HIGH** (prioritize):
- `compiler/passes/` (especially code_generation, type_checking, flattening)
- `compiler/ast/types/`
- `compiler/parser/`, `compiler/parser-lossless/`

**MEDIUM**:
- `compiler/ast/`
- `compiler/compiler/`
- `interpreter/`
- `errors/`

**LOW**:
- `test-framework/`
- `leo/package/`
- docs, tests

Update `$WS/state-pr-$PR.md` with risk assessment.

For large PRs (30+ files), focus on HIGH-risk areas first. Ask user if they want to focus on specific areas.

## Phase 3: Understand

Before code analysis, answer:
- What problem does this PR solve?
- What compiler invariants must hold?
- What could go wrong?

## Phase 4: Analyze

For each file (prioritize by risk):
1. Read the FULL file (not just diff) for context
2. Fetch diff: `gh pr diff $PR -- path/to/file.rs`
3. Trace logic step-by-step
4. Check boundary conditions (zero, empty, max, off-by-one)
5. Verify AST transformations preserve semantics
6. Check spans and NodeIDs are handled correctly
7. **Write findings to state file immediately**
8. Move to next file (previous content no longer needed)

Apply AGENTS.md checklists:
- Correctness
- Compiler-Specific
- Memory & Performance
- Security

## Phase 5: Verify

Run validation on affected crates:
```bash
CRATES=$(cat "$WS/crates-pr-$PR.txt")
for crate in $CRATES; do
  cargo check -p $crate
  cargo clippy -p $crate -- -D warnings
  cargo test -p $crate
done
```

For parser changes:
```bash
cargo test -p leo-parser
cargo test -p leo-parser-lossless
```

## Phase 6: Report

Re-read `$WS/state-pr-$PR.md`, then output findings:

| Sev | Location | Issue | Fix |
|-----|----------|-------|-----|

**Severities**:
- BLOCKER: Must fix before merge (correctness, security)
- BUG: Should fix (logic error, edge case)
- ISSUE: Quality concern (performance, maintainability)
- NIT: Style/minor (optional)

**Recommendation**: Approve / Request changes / Needs discussion

## Phase 7: Handoff (if needed)

If fixes are required, write `$WS/handoff-pr-$PR.md`:

```markdown
# Handoff: PR #$PR

**From:** leo-review
**To:** leo-fix-pr

## Required Fixes

| # | Location | Issue | Suggested Fix |
|---|----------|-------|---------------|

## Notes

[Any additional context for the fixer]
```
</instructions>

## Review Checklist

### Correctness
- [ ] Logic traced step-by-step
- [ ] Boundary conditions handled: zero, empty, max, off-by-one
- [ ] Error handling correct; no panics in production paths
- [ ] AST transformations preserve semantics

### Compiler-Specific
- [ ] Spans preserved through transformations for error reporting
- [ ] NodeIDs assigned correctly for new nodes
- [ ] Pass ordering dependencies respected
- [ ] Generated Aleo instructions are valid

### Memory & Performance
- [ ] No unnecessary allocations in hot paths
- [ ] Pre-allocation with `with_capacity` where size known
- [ ] No unnecessary `.clone()` - prefer references
- [ ] Iterators used efficiently; no intermediate collections

### Security
- [ ] Input validation at trust boundaries
- [ ] No information leakage in error messages
- [ ] Fail-closed (reject on uncertainty)

## Deep Analysis Techniques

### Trace Compilation
1. Start from source code input
2. Follow through lexer -> parser -> AST construction
3. Track each pass transformation
4. Verify output instructions match input semantics

### Enumerate Failure Modes
For each operation, ask:
- What if input is empty/malformed?
- What if types don't match?
- What if identifiers collide?
- What if limits are exceeded?

### Check Invariants
- AST nodes always have valid spans
- Type annotations consistent after type checking
- No unresolved identifiers after name resolution
- All loops unrolled after loop unrolling pass
