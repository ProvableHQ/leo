# Leo Skills

Skills for Leo compiler development workflows.

## Available Skills

| Skill | Command | Description |
|-------|---------|-------------|
| leo-github | `/leo-github pr <n>` or `/leo-github issue <n>` | Fetch GitHub context into workspace |
| leo-review | `/leo-review <pr>` | Security-focused PR review |
| leo-fix | `/leo-fix <issue>` | Fix GitHub issue with TDD |
| leo-fix-pr | `/leo-fix-pr <pr>` | Fix PR review feedback |

## Workspace

All skills use `.claude/workspace/` for state persistence:

```
.claude/workspace/
├── context-{type}-{id}.json    # Structured data (PR metadata, files list)
├── state-{type}-{id}.md        # Human-readable state + progress log
└── handoff-{type}-{id}.md      # Cross-skill communication
```

## Workflow

1. **Fetch context**: `/leo-github pr 123` or `/leo-github issue 456`
2. **Review**: `/leo-review 123` - produces findings in state file
3. **Fix**: `/leo-fix 456` or `/leo-fix-pr 123` - TDD workflow

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
