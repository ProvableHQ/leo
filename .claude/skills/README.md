# Leo Claude Skills

Drop-in Claude Code skills for Leo compiler development workflows.

## Skills Overview

| Skill | Purpose | Invoke |
|-------|---------|--------|
| **leo-github** | Fetch PR/issue context from GitHub | `/leo-github pr 123` |
| **leo-fix** | Fix GitHub issues with TDD | `/leo-fix 456` |
| **leo-fix-pr** | Address PR review feedback | `/leo-fix-pr 123` |
| **leo-review** | Security-focused PR review | `/leo-review 123` |

## Prerequisites

1. **GitHub CLI** authenticated:
   ```bash
   gh auth status || gh auth login
   ```

2. **Rust toolchain** with nightly for formatting:
   ```bash
   rustup install nightly
   ```

## Usage Examples

### Review a PR

```
/leo-review 29081
```

Claude will:
1. Fetch PR context (files, commits, comments, CI status)
2. Triage files by risk level (passes > parser > ast > support)
3. Analyze changes for bugs/vulnerabilities
4. Report findings with severity ratings
5. Create handoff file if fixes needed

### Fix PR Feedback

```
/leo-fix-pr 29081
```

Claude will:
1. Load PR context and unresolved review threads
2. Analyze each comment
3. Present fix plan for approval
4. Implement fixes with TDD
5. Validate with cargo check/clippy/test

### Fix an Issue

```
/leo-fix 29000
```

Claude will:
1. Fetch issue context
2. Investigate root cause
3. Present fix plan for approval
4. Write failing test first
5. Implement minimal fix
6. Validate fix

### Just Fetch Context

```
/leo-github pr 29081
/leo-github issue 29000
```

## Workspace Structure

All context is stored in `.claude/workspace/`:

```
.claude/workspace/
├── context-pr-123.json      # PR metadata
├── files-pr-123.txt         # Changed files
├── crates-pr-123.txt        # Affected crates
├── commits-pr-123.json      # Commits
├── comments-pr-123.json     # PR comments
├── checks-pr-123.json       # CI status
├── threads-pr-123.jsonl     # Review threads
├── unresolved-pr-123.json   # Unresolved comments
├── resolved-pr-123.json     # Resolved comments
├── state-pr-123.md          # Working state (findings, log)
├── handoff-pr-123.md        # Review→fix handoff
│
├── context-issue-456.json   # Issue metadata
├── comments-issue-456.jsonl # Issue comments
├── timeline-issue-456.json  # Cross-references
├── state-issue-456.md       # Investigation state
└── ...
```

## Caching

Context is cached for 1 hour. Force refresh with:

```
/leo-github pr 123 --force
```

Or delete workspace files:

```bash
rm .claude/workspace/*pr*123*
```

## Customization

### Risk Categories

Edit `leo-review/SKILL.md` section 2 to adjust file risk categorization.

### Crate Detection

Edit `leo-github/scripts/detect-crates.sh` to update the directory→crate mapping if the repo structure changes.

### Templates

Edit files in `leo-github/templates/` to customize state file formats.

## How It Works

### Skill Structure

```
leo-github/
├── SKILL.md                 # Main skill instructions
├── scripts/
│   ├── fetch-pr.sh          # Fetch PR context
│   ├── fetch-issue.sh       # Fetch issue context
│   ├── refresh-threads.sh   # Quick thread refresh
│   ├── cargo-validate.sh    # Run cargo checks
│   └── detect-crates.sh     # Map files to crates
└── templates/
    ├── state-pr.md          # PR state template
    ├── state-issue.md       # Issue state template
    └── handoff.md           # Review handoff template
```

### Key Features

1. **Shared scripts** — Common logic extracted to reusable shell scripts
2. **Mustache templates** — Consistent state file generation
3. **Auto-detection** — Crates detected from changed files
4. **Context forking** — Review skill runs in isolated context
5. **Parallel analysis** — Large PRs analyzed with subagents
6. **Progressive loading** — Only loads what's needed

## Leo-Specific

### Crate Mapping

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

### Risk Levels

- **HIGH**: `compiler/passes/` (code_generation, type_checking, flattening), `compiler/ast/types/`, `compiler/parser/`
- **MEDIUM**: `compiler/ast/`, `compiler/compiler/`, `interpreter/`, `errors/`
- **LOW**: `test-framework/`, `leo/package/`, docs, tests

See AGENTS.md for detailed guidelines.

## Troubleshooting

### "gh CLI not authenticated"

```bash
gh auth login
```

### "Context missing"

The skill will auto-fetch. Or manually:

```bash
/leo-github pr 123
```

### "cargo check failed"

Ensure you're in the Leo repo root and have the correct Rust toolchain.

### Skills not appearing

Check skill descriptions exceed the character budget:

```
/context
```

Look for warnings about excluded skills.
