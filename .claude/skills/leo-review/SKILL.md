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

```bash
if [[ ! -f "$WS/state-pr-$PR.md" ]]; then
  "$SKILL_DIR/scripts/fetch-pr.sh" "$PR"
fi
```

Read: `$WS/state-pr-$PR.md`, `$WS/files-pr-$PR.txt`

## 2. Triage by Risk

Categorize changed files by risk:
- **HIGH**: passes/ (code_generation, type_checking, flattening), parser/, ast/types/, span/
- **MEDIUM**: ast/ (non-types), compiler/, interpreter/, errors/
- **LOW**: test-framework/, package/, docs, tests

Update `$WS/state-pr-$PR.md` with risk assessment.

**For large PRs (30+ files):** Focus HIGH risk areas first. Use parallel subagents by category (passes, AST/parser, other).

## 3. Understand Intent

Before diving into code:
1. Read PR description: `jq -r .body "$WS/context-pr-$PR.json"`
2. Check linked issues: `cat "$WS/linked-issues-pr-$PR.txt"`
3. What compiler invariants must hold?
4. What could go wrong?

## 4. Analyze Code

**Small/medium PRs (< 30 files):** Sequential. For each HIGH/MEDIUM risk file — read full file, trace logic, check boundaries, write findings to state file, release from memory.

**Large PRs (30+ HIGH/MEDIUM files):** Spawn parallel Task subagents by category (passes, AST/parser, other). Each reads assigned files, applies AGENTS.md checklists, returns findings table.

Apply **AGENTS.md Review Checklist** (Correctness, Compiler-Specific, Memory & Performance, Security) to each file.

## 5. Validate

Validate affected crates per AGENTS.md.

## 6. Report

Update `$WS/state-pr-$PR.md` with findings:

| Sev | Location | Issue | Suggested Fix |
|-----|----------|-------|---------------|

**Severity:** BLOCKER (must fix), BUG (should fix), ISSUE (consider), NIT (optional).

**Recommendation:** Approve, Request changes, or Needs discussion.

## 7. Handoff

If requesting changes, create handoff for `/leo-fix-pr`:

```bash
sed -e "s/{{NUM}}/$PR/g" "$SKILL_DIR/templates/handoff.md" > "$WS/handoff-pr-$PR.md"
```

## Common Vulnerability Patterns in Leo

- **Pass ordering**: A pass assumes invariants from an earlier pass that may not hold
- **Span loss**: Transformations drop spans, making errors point to wrong locations
- **NodeID reuse**: New nodes reuse IDs from old nodes, breaking traversal
- **Type erasure**: Type information lost during lowering, producing invalid Aleo instructions
- **Grammar ambiguity**: Parser accepts invalid syntax that crashes later passes
