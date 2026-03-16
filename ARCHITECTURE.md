# Architecture

Leo is a compiler for the Aleo blockchain. Code flows through:

```
Source (.leo) -> Lexer (logos) -> Rowan Parse Tree -> AST -> Passes -> Aleo Bytecode
```

## Crate dependencies

- `leo-span` - provides Span/source locations (has third-party deps: fxhash, indexmap, serde)
- `leo-errors` - depends on span, all error types
- `leo-ast` - depends on span/errors, all AST nodes
- `leo-parser-rowan` - lexer (logos) + rowan-based parser; grammar defined in `grammar.rs`
- `leo-parser` - converts rowan parse tree to typed AST
- `leo-passes` - ~25 compiler passes (validation, optimization, codegen)
- `leo-compiler` - orchestrates parsing and passes
- `leo-abi` / `leo-abi-types` - ABI generation for compiled programs
- `leo-fmt` - Leo source formatter (uses `leo-parser-rowan`)
- `leo-disassembler` - Aleo bytecode disassembler
- `leo-package` - project structure parsing
- `leo-test-framework` - test harness for .leo files

## Crates

### ast

- All AST nodes must implement the `Node` trait (use `simple_node_impl!` macro).
- Every node needs `Span` and `NodeID` for error reporting and traversal.
- Prefer `IndexMap` (over HashMap) for deterministic ordering.
- Large enum variants should be boxed.

### parser / parser-rowan

- `parser-rowan` uses a rowan-based grammar defined in `grammar.rs` with logos for lexing.
- `parser` converts the rowan parse tree to a typed AST.
- Parser tests use expectation files in `tests/expectations/parser/`.

### passes

- Each pass implements the `Pass` trait with `do_pass(input, state) -> Result<Output>`.
- Passes run sequentially via `CompilerState`.
- Common passes: type checking, loop unrolling, monomorphization, flattening, code generation.
- Test passes in isolation where possible.

### errors

- All errors use macros in `leo-errors/src/common/macros.rs`.
- Error codes: format `E{PREFIX}037{CODE}` (e.g., `EPAR0370042` for parser error 42; categories: PAR 0-999, AST 2000-2999, CMP 6000-6999).
- Use `Result<T>` from `leo_errors`.
- Avoid leaking internal details in error messages.
