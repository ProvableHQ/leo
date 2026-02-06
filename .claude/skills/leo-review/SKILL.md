---
name: leo-review
description: |
  Security-focused PR review for Leo compiler codebase.
  WHEN: User says "review PR", "audit PR", "security review", "check PR changes",
  or wants thorough analysis of PR changes for bugs/vulnerabilities.
  WHEN NOT: Fixing review feedback (use leo-fix-pr), fetching context only
  (use leo-github), or fixing issues (use leo-fix).
context: fork
agent: general-purpose
allowed-tools: Bash, Read, Write, Grep, Glob, Task
disable-model-invocation: true
---

# Security-Focused PR Review

**Mindset: Assume there is a bug. Find it.**

This skill runs in a forked context to keep exploration out of your main conversation.

## Usage

```
/leo-review <pr_number>
```

## Setup

```bash
PR=$ARGUMENTS
WS=".claude/workspace"
SKILL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../leo-github" && pwd)"
```

## 1. Load Context

Ensure PR context exists:

```bash
if [[ ! -f "$WS/state-pr-$PR.md" ]]; then
  echo "Context missing. Fetching PR #$PR..."
  "$SKILL_DIR/scripts/fetch-pr.sh" "$PR"
fi
```

Read initial state:

```bash
cat "$WS/state-pr-$PR.md"
cat "$WS/files-pr-$PR.txt"
cat "$WS/crates-pr-$PR.txt" 2>/dev/null || echo "No crates detected"
```

## 2. Triage by Risk

Categorize changed files:

| Risk | Directories | Rationale |
|------|-------------|-----------|
| **HIGH** | `compiler/passes/` (especially code_generation, type_checking, flattening), `compiler/ast/types/`, `compiler/parser/`, `compiler/parser-lossless/` | Core compiler logic, can produce wrong code or reject valid programs |
| **MEDIUM** | `compiler/ast/`, `compiler/compiler/`, `interpreter/`, `errors/` | Can cause program failures, bad error messages |
| **LOW** | `test-framework/`, `leo/package/`, docs, tests | Support code, lower blast radius |

Count files per risk level and update `$WS/state-pr-$PR.md`.

**For large PRs (30+ files):** Focus on HIGH risk areas first. Consider parallel analysis.

## 3. Understand Intent

Before diving into code, answer:

1. **What problem does this PR solve?**
   - Read PR description: `jq -r .body "$WS/context-pr-$PR.json"`
   - Check linked issues: `cat "$WS/linked-issues-pr-$PR.txt"`

2. **What compiler invariants must hold?**
   - Span preservation?
   - NodeID assignment?
   - Pass ordering?

3. **What could go wrong?**
   - Edge cases (zero, empty, max)?
   - Type system holes?
   - Code generation correctness?

## 4. Analyze Code

### For small/medium PRs (< 30 files)

Sequential analysis. For each HIGH/MEDIUM risk file:

1. **Read the full file** (not just diff) for context
2. **Trace the logic** step-by-step
3. **Check boundaries**: zero, empty, max, overflow
4. **Write findings to `$WS/state-pr-$PR.md` immediately**
5. **Release file from working memory** and move on

Fetch diffs selectively:
```bash
gh pr diff $PR -- path/to/file.rs
```

### For large PRs (30+ HIGH/MEDIUM files)

Use **parallel subagents** for faster analysis:

**Spawn Task subagents by category:**

```
Use the Task tool to spawn parallel subagents:

Task 1 (Passes): Analyze files in compiler/passes/
  - Check: Pass ordering, code generation, type checking, flattening
  - Return: Findings table

Task 2 (AST/Parser): Analyze files in compiler/ast/, compiler/parser/, compiler/parser-lossless/
  - Check: Node types, span preservation, NodeID assignment, grammar correctness
  - Return: Findings table

Task 3 (Other): Analyze files in compiler/compiler/, interpreter/, errors/
  - Check: Orchestration, error handling, runtime correctness
  - Return: Findings table
```

Each subagent should:
- Read assigned files completely
- Apply relevant checks from section 4.1
- Return findings in table format
- Note anything requiring cross-file analysis

Merge results into `$WS/state-pr-$PR.md`.

### 4.1 Review Checklist

**Correctness:**
- [ ] Logic traced step-by-step
- [ ] Boundary conditions handled: zero, empty, max, off-by-one
- [ ] Error handling correct; no panics in production paths
- [ ] AST transformations preserve semantics

**Compiler-Specific:**
- [ ] Spans preserved through transformations for error reporting
- [ ] NodeIDs assigned correctly for new nodes
- [ ] Pass ordering dependencies respected
- [ ] Generated Aleo instructions are valid

**Memory & Performance:**
- [ ] No unnecessary allocations in hot paths
- [ ] Pre-allocation with `with_capacity` where size known
- [ ] No unnecessary `.clone()` — prefer references
- [ ] Iterators used efficiently; no intermediate collections

**Security:**
- [ ] Input validation at trust boundaries
- [ ] No information leakage in error messages
- [ ] Fail-closed (reject on uncertainty)

## 5. Verify Build

```bash
"$SKILL_DIR/scripts/cargo-validate.sh"
```

Or selectively:
```bash
CRATES=$(cat "$WS/crates-pr-$PR.txt")
for crate in $CRATES; do
  cargo check -p "$crate"
  cargo clippy -p "$crate" -- -D warnings
  cargo test -p "$crate"
done
```

For parser changes:
```bash
cargo test -p leo-parser
cargo test -p leo-parser-lossless
```

## 6. Report

Re-read `$WS/state-pr-$PR.md` to compile findings.

**Output findings table:**

| Sev | Location | Issue | Suggested Fix |
|-----|----------|-------|---------------|
| BLOCKER | path/file.rs:42 | Unchecked overflow in... | Use checked_add or saturating_add |
| BUG | path/file.rs:100 | Missing bounds check | Add length validation |
| ISSUE | path/file.rs:200 | Inefficient clone | Use reference instead |
| NIT | path/file.rs:300 | Inconsistent naming | Rename to match convention |

**Severity guide:**
- **BLOCKER** — Must fix before merge. Correctness/security issue.
- **BUG** — Should fix. Likely causes problems.
- **ISSUE** — Quality concern. Consider fixing.
- **NIT** — Style/preference. Optional.

**Final recommendation:**
- **Approve** — No blockers, code is sound
- **Request changes** — Blockers or bugs require attention
- **Needs discussion** — Design concerns to resolve

## 7. Handoff (if changes needed)

If requesting changes, create handoff for `/leo-fix-pr`:

```bash
TEMPLATE_DIR="$SKILL_DIR/templates"
sed -e "s/{{NUM}}/$PR/g" "$TEMPLATE_DIR/handoff.md" > "$WS/handoff-pr-$PR.md"
```

Then edit `$WS/handoff-pr-$PR.md` to fill in the required fixes table.

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

## Common Vulnerability Patterns in Leo

- **Pass ordering**: A pass assumes invariants from an earlier pass that may not hold
- **Span loss**: Transformations drop spans, making errors point to wrong locations
- **NodeID reuse**: New nodes reuse IDs from old nodes, breaking traversal
- **Type erasure**: Type information lost during lowering, producing invalid Aleo instructions
- **Grammar ambiguity**: Parser accepts invalid syntax that crashes later passes
