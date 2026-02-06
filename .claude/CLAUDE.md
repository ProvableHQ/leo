@../AGENTS.md

# Memory & Workspace Management

This is a compiler codebase. Manage your context window deliberately.

## Workspace

Use `.claude/workspace/` for scratch files. This directory is gitignored.

```
.claude/workspace/
├── context-{type}-{id}.json    # Structured data (PR metadata, files list)
├── state-{type}-{id}.md        # Human-readable state + progress log
└── handoff-{type}-{id}.md      # Cross-skill communication
```

Initialize workspace:
```bash
mkdir -p .claude/workspace
```

## Memory Rules

1. **Summarize, don't retain** - After reading raw API output or large diffs, extract what matters into scratch files, then proceed. Don't keep raw data in conversation.

2. **Fetch incrementally** - Never load full diff upfront for large PRs. Get file list first, then fetch diffs per-file or per-directory as needed.

3. **Rotate context** - When moving to a new file/area, write findings to scratch file first. Previous raw content is no longer needed.

4. **Re-read scratch files** - If you need prior context, read from scratch files rather than asking me to repeat or scrolling back.

5. **Index what you've done** - Track reviewed files, open questions, and findings in structured scratch files.

## Large PR Protocol

For PRs with 30+ files or 1000+ lines changed:

1. **Triage first** - Fetch metadata + file list only. Categorize files by risk/importance.
2. **Ask for focus** - Or identify critical paths (passes, codegen, type system).
3. **Deep-dive selectively** - Analyze high-priority areas thoroughly.
4. **Summarize and continue** - Write findings, move to next area.
5. **Synthesize at end** - Read scratch files to produce final report.

## Pagination

Always paginate when fetching:
- Review threads (can exceed 100)
- Comments within threads (can exceed 10)
- Commits (can exceed 30)
- PR comments (can exceed 30)

Use `--paginate` for REST, loop on `hasNextPage` for GraphQL.

## Session Recovery

If resuming after a break:
1. Check workspace: `ls -la .claude/workspace/`
2. Read state file: `cat .claude/workspace/state-*.md`
3. Check Implementation Log for last completed step
4. Continue from that point
5. If context is stale, re-fetch as needed

## Context Budget

For large tasks (>10 files or >500 lines changed):
- Write findings to workspace IMMEDIATELY after analyzing each file
- Do not keep raw diffs in conversation after extracting findings
- Before starting a new phase, summarize the previous phase to workspace
- If responses slow down or truncate, stop and consolidate state

These are guidelines, not mandates. Adapt based on task complexity. The goal is reliable state persistence.


<claude-mem-context>
# Recent Activity

### Feb 6, 2026

| ID | Time | T | Title | Read |
|----|------|---|-------|------|
| #73 | 9:37 AM | 🔵 | Leo Permissions Configuration for Skills | ~480 |
</claude-mem-context>