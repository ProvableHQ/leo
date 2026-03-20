# tree-sitter-leo

Tree-sitter grammar for the Leo programming language.

This grammar now lives in the Leo monorepo so it can evolve alongside
`crates/parser-rowan`, which remains the source of truth for Leo syntax. The
Rust bindings and parser-backed integration tests live in
`crates/tree-sitter-leo`.

## Development

Generate parser artifacts from this directory with the tree-sitter CLI:

```bash
tree-sitter generate
tree-sitter test
```

If you do not have the CLI installed globally, you can run the corpus tests with:

```bash
npx --yes tree-sitter-cli@0.25.10 test
```

The generated files in `src/` are committed so normal Rust builds do not require
Node.js or the tree-sitter CLI. CI installs the CLI separately for corpus checks.

## Parser-backed verification

Run the Rust-side integration checks with:

```bash
cargo test -p tree-sitter-leo
```

These tests parse a focused set of Leo fixtures with both the Rust bindings
crate (`tree-sitter-leo`) and `leo-parser-rowan` to catch syntax drift early.
In practice, changes to `crates/parser-rowan` should surface here as test
failures when the grammar needs to be updated.

## Keeping in Sync with Leo

This grammar is derived from the Leo `parser-rowan` crate (`crates/parser-rowan/src/`
in [ProvableHQ/leo](https://github.com/ProvableHQ/leo)). When Leo's syntax changes,
this grammar must be updated to match.

### Source of Truth

The single source of truth for Leo syntax is the `parser-rowan` crate. Key files:

| File | What it defines |
|------|-----------------|
| `syntax_kind.rs` | Every token and AST node kind - new keywords, operators, or node types show up here first |
| `lexer.rs` | Token patterns (regexes, keyword table) - changes to literal formats or new tokens |
| `parser/items.rs` | Top-level and program-level declarations - new item types |
| `parser/statements.rs` | Statement forms - new statement types or control flow |
| `parser/expressions.rs` | Expression forms and operator precedence - new operators or expression syntax |
| `parser/types.rs` | Type syntax - new type forms or modifiers |

### When to Update

This grammar does not need to be manually updated for every parser change.
Instead, `parser-rowan` remains the source of truth, and the tree-sitter corpus
tests plus the Rust-side compatibility test should make syntax drift visible.

When those checks fail because Leo syntax has changed, update this grammar to
match. Typical triggers include Leo PRs that:

1. Adds or removes a keyword.
2. Adds a new statement, expression, or item type.
3. Changes operator precedence.
4. Adds or modifies a type form.
5. Changes literal syntax.

### How to Update

1. Diff the `parser-rowan` crate against the last synced commit to confirm the syntax changes.
2. Identify changes in tokens, items, statements, expressions, types, and precedence.
3. Update `grammar.js` to match the new syntax.
4. Update `queries/*.scm` for highlighting, folds, indents, locals, and text objects.
5. Add corpus coverage in `test/corpus/*.txt`.
6. Validate with `tree-sitter generate`, `tree-sitter test`, and `cargo test -p tree-sitter-leo`.
7. Record the new sync point in the table below.

### Last Synced

| Field | Value |
|-------|-------|
| Leo repo | `ProvableHQ/leo` |
| Commit | `13de1ad9d53ff0672fc8ebd139e3da8feacea8dd` |
| Date | `2026-03-11` |
| Leo version | `4.x.x` |
