# tree-sitter-leo (Rust bindings)

This crate provides the Rust bindings and compatibility tests for the Leo
tree-sitter grammar, whose source of truth lives in the repository's top-level
`tree-sitter/` directory.

It compiles the generated parser from that directory, exposes the Leo
`tree_sitter::Language`, and runs the Rust-side checks that compare the grammar
against `leo-parser-rowan`.

For grammar development and corpus tests, work from `tree-sitter/`. For
Rust-side verification, run:

```bash
cargo test -p tree-sitter-leo
```
