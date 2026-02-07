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

Ensure PR context exists. If missing, fetch it:

```bash
if [[ ! -f "$WS/state-pr-$PR.md" ]]; then
  echo "Context missing. Fetching PR #$PR..."
  "$SKILL_DIR/scripts/fetch-pr.sh" "$PR"
fi
```

Quick-refresh threads to get latest resolved status (faster than full fetch):

```bash
"$SKILL_DIR/scripts/refresh-threads.sh" "$PR"
```

Review current state:

```bash
cat "$WS/state-pr-$PR.md"
echo "--- Unresolved comments ---"
cat "$WS/unresolved-pr-$PR.json" | jq -r '.[] | "- \(.path):\(.line) [\(.reviewer)]: \(.comment[0:100])..."'

# Check if there's a handoff from review
if [[ -f "$WS/handoff-pr-$PR.md" ]]; then
  echo "--- Handoff from review ---"
  cat "$WS/handoff-pr-$PR.md"
fi
```

## 2. Analyze Each Comment

For each unresolved comment, determine:

| # | Path:Line | Reviewer | Request | Concern Type | Risk |
|---|-----------|----------|---------|--------------|------|

**Concern types:**
- **Correctness** — Logic error, wrong behavior
- **Performance** — Inefficiency, unnecessary allocations
- **Security** — Vulnerability, unsafe code
- **Style** — Naming, formatting, idioms
- **Clarity** — Confusing code, missing docs

**Risk levels:**
- **Low** — Isolated change, well-tested area
- **Medium** — Touches multiple components
- **High** — Core logic, compiler passes, parser

Update `$WS/state-pr-$PR.md` with your analysis.

**Think hard. Do not proceed until you understand each request.**

## 3. Plan (APPROVAL REQUIRED)

Present a fix plan:

| # | Location | Request | Proposed Fix | Risk |
|---|----------|---------|--------------|------|
| 1 | path/file.rs:42 | "Add bounds check" | Add `if len > MAX` guard | Low |
| 2 | ... | ... | ... | ... |

**Use AskUserQuestion to get explicit approval before proceeding.**

## 4. Implement Fixes (TDD)

### 4.1 Establish baseline

```bash
# Detect affected crates
CRATES=$(cat "$WS/crates-pr-$PR.txt" 2>/dev/null || echo "")

# Verify baseline passes
for crate in $CRATES; do
  cargo check -p "$crate"
  cargo clippy -p "$crate" -- -D warnings
  cargo test -p "$crate" --lib
done
```

### 4.2 Fix each comment

For each fix:

1. **Write/update test** (if behavioral change)
   ```rust
   #[test]
   fn test_bounds_check_prevents_overflow() {
       // Test the fix
   }
   ```

2. **Verify test fails** (for new behavior):
   ```bash
   cargo test -p <crate> test_bounds_check -- --nocapture
   ```

3. **Make minimal change**
   - Match existing code style
   - Preserve formatting conventions
   - Add comments if non-obvious

4. **Verify fix works**:
   ```bash
   cargo check -p <crate>
   cargo clippy -p <crate> -- -D warnings
   cargo test -p <crate>
   ```

5. **Log to state file**:

   | Action | Result |
   |--------|--------|
   | Fix #1 | Added bounds check at file.rs:42 |

For parser/compiler tests with expectation files:
```bash
REWRITE_EXPECTATIONS=1 cargo test -p leo-parser  # Update expectations
TEST_FILTER=test_name cargo test                  # Run specific test
```

### 4.3 Batch style/nit fixes

For pure style changes (no behavioral impact), batch them:

```bash
# Apply formatting
cargo +nightly fmt --all

# Fix clippy lints
cargo clippy -p <crate> --fix --allow-dirty
```

## 5. Final Validation

Run complete validation:

```bash
cargo check -p <crate>
cargo clippy -p <crate> -- -D warnings
cargo +nightly fmt --check
cargo test -p <crate>
```

Or use the validation script:

```bash
"$SKILL_DIR/scripts/cargo-validate.sh" $CRATES
```

## 6. Report

Summarize resolutions:

| # | Comment | Resolution | Verified |
|---|---------|------------|----------|
| 1 | "Add bounds check" | Added guard at file.rs:42 | cargo test |
| 2 | "Rename variable" | Changed `x` to `count` | style only |
| 3 | "Add doc comment" | Added /// explaining behavior | docs |

**Files changed:**
- path/to/file.rs — [changes]
- path/to/test.rs — [new/updated tests]

**Do not commit unless explicitly asked.**

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

## Handling Disagreements

If you disagree with a review comment:

1. Explain your reasoning clearly
2. Provide evidence (benchmarks, tests, spec references)
3. Propose an alternative if applicable
4. **Use AskUserQuestion** to discuss with user before proceeding

Never silently ignore review feedback.
