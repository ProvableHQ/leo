# Leo

## Before Writing Code
- Search for existing implementations.
- Read the target module and match its patterns exactly.
- Scope out necessary tests ahead of time.
- If uncertain, ask.
- Think very hard in your planning process.
- New files, crates, dependencies, abstractions, traits, or error types require approval.

## Architecture

Leo is a compiler for the Aleo blockchain. Code flows through:

```
Source (.leo) -> Lexer (logos) -> Rowan Parse Tree -> AST -> Passes -> Aleo Bytecode
```

**Crate dependencies:**
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

**ast**
- All AST nodes must implement the `Node` trait (use `simple_node_impl!` macro).
- Every node needs `Span` and `NodeID` for error reporting and traversal.
- Use `IndexMap` (not HashMap) for deterministic ordering.
- Large enum variants must be boxed.

**parser / parser-rowan**
- `parser-rowan` uses a rowan-based grammar defined in `grammar.rs` with logos for lexing.
- `parser` converts the rowan parse tree to a typed AST.
- Parser tests use expectation files in `tests/expectations/parser/`.

**passes**
- Each pass implements the `Pass` trait with `do_pass(input, state) -> Result<Output>`.
- Passes run sequentially via `CompilerState`.
- Common passes: type checking, loop unrolling, monomorphization, flattening, code generation.
- Test passes in isolation where possible.

**errors**
- All errors use macros in `leo-errors/src/common/macros.rs`.
- Error codes: format `E{PREFIX}037{CODE}` (e.g., `EPAR0370042` for parser error 42; categories: PAR 0-999, AST 2000-2999, CMP 6000-6999).
- Use `Result<T>` from `leo_errors`.
- Never leak internal details in error messages.

## Code and Patterns
- Test-driven development: write failing tests first.
- `unwrap`s must be commented with justification.
- Pre-allocate with `with_capacity` when final size is known.
- Prefer arrays/slices over `Vec` when size is known at compile time.
- Use iterators; avoid intermediate vectors and unnecessary `.collect()`.
- Prefer references and `into_iter()` over `.clone()` and `iter().cloned()`.

See @CONTRIBUTING.md for detailed memory and performance guidelines.

## Testing

**Test framework:**
- Tests in `tests/tests/{category}/` with expectations in `tests/expectations/{category}/`.
- Use `UPDATE_EXPECT=1 cargo test` to update expectation files.
- Use `TEST_FILTER=name cargo test` to run specific tests.

**Commands:**
```bash
cargo test -p <crate>                           # Run crate tests
cargo test -p leo-compiler                      # Compiler tests (slow)
UPDATE_EXPECT=1 cargo test -p leo-parser # Update parser expectations
TEST_FILTER=loop cargo test                     # Filter by name
```

**CLI integration tests:**
- Tests in `tests/tests/cli/` with scripts that run `leo` subcommands end-to-end.
- Integration test runner in `leo/tests/integration.rs` spins up `leo devnode`.
- 17 test cases cover build, run, deploy, add, and related commands.
- These tests require a running devnode and are gated by environment (not run in standard `cargo test`).

## Validation

Run in order:
```bash
cargo check -p <crate>
cargo clippy -p <crate> -- -D warnings
cargo +nightly fmt --check
cargo test -p <crate>
```

Clippy warnings are errors. Formatting requires nightly (`cargo +nightly fmt --all` to fix).

## Git
- Never commit unless explicitly asked.
- Stage with `git add` only if requested.
- Run `cargo +nightly fmt --all` before staging.

## Style
- One blank line between functions.
- No trailing whitespace.
- Imports: first-party (crate + leo_*) first, third-party (std + external) second.
- Match existing file patterns exactly.
- Comments must be concise, complete, punctuated sentences.
- Line width: 120 characters max.
- `#![forbid(unsafe_code)]` in span, passes, compiler, errors, and package crates.

## Review Checklist

### Correctness
- [ ] Logic traced step-by-step.
- [ ] Boundary conditions handled: zero, empty, max, off-by-one.
- [ ] Error handling correct; no panics in production paths.
- [ ] AST transformations preserve semantics.

### Compiler-Specific
- [ ] Spans preserved through transformations for error reporting.
- [ ] NodeIDs assigned correctly for new nodes.
- [ ] Pass ordering dependencies respected.
- [ ] Generated Aleo instructions are valid.

### Memory & Performance
- [ ] No unnecessary allocations in hot paths.
- [ ] Pre-allocation with `with_capacity` where size known.
- [ ] No unnecessary `.clone()` - prefer references.
- [ ] Iterators used efficiently; no intermediate collections.

### Security
- [ ] Input validation at trust boundaries.
- [ ] No information leakage in error messages.
- [ ] Fail-closed (reject on uncertainty).

## Deep Analysis Techniques

### Trace Compilation
1. Start from source code input.
2. Follow through lexer -> parser -> AST construction.
3. Track each pass transformation.
4. Verify output instructions match input semantics.

### Enumerate Failure Modes
For each operation, ask:
- What if input is empty/malformed?
- What if types don't match?
- What if identifiers collide?
- What if limits are exceeded?

### Check Invariants
- AST nodes always have valid spans.
- Type annotations consistent after type checking.
- No unresolved identifiers after name resolution.
- All loops unrolled after loop unrolling pass.
