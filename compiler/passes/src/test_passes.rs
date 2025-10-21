// Copyright (C) 2019-2025 Provable Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

// Unit tests for individual compiler passes.
// This module provides test infrastructure to verify the behavior of individual
// compiler passes in isolation. Each pass can be tested by providing Leo source
// code and verifying the AST output after the pass runs.

/*!
# Compiler Pass Test Runners

This module provides automatically generated test runners for all compiler transform passes in the Leo compiler.

## Adding a New Compiler Pass Test

To add a new compiler pass, you need to update this file and create the test directories:

### 1. Update this file

1. Add a new entry to the `compiler_passes!` table:

```rust
(runner_name, PassStruct, (input))
```

- `runner_name` – the function name for this pass runner (snake_case).
- `PassStruct` – the compiler pass struct you are testing.
- `input` – the argument to the pass (`()` if none, or a struct literal like `(SsaFormingInput { rename_defs: true })`).

Example:

```rust
(new_pass_runner, NewPass, (NewPassInput { option: true })),
```

2. No other code needs to change — macros automatically generate:
   - The runner function
   - The test function (`#[test] fn new_pass_runner_test()`)

---

### 2. Create the test directories

Each pass requires two directories in the Leo repository:

1. **Source tests**: `leo/tests/tests/passes/<pass_name>`
   - Contains `.leo` files with source programs to test this pass.

2. **Expected outputs**: `leo/tests/expectations/passes/<pass_name>`
   - Contains expected output files for each source test file.

Example for `common_subexpression_elimination`:

```
leo/tests/tests/passes/common_subexpression_elimination/
leo/tests/expectations/passes/common_subexpression_elimination/
```

- The runner will compare the output AST (or errors/warnings) against these expectations.

---

This structure ensures that **adding a new compiler pass test is minimal**:
- Add a single line to the `compiler_passes!` table.
- Create the two directories for tests and expected outputs.
- All runners and test functions are generated automatically.
*/

use crate::*;
use leo_ast::{Ast, NetworkName, NodeBuilder};
use leo_errors::{BufferEmitter, Handler};
use leo_parser::parse_ast;
use leo_span::{create_session_if_not_set_then, source_map::FileName, with_session_globals};
use serial_test::serial;

/// Table of all compiler passes and their runner names.
/// Each entry is a tuple of `(runner_name, pass_struct, input)`
/// - `input` is the argument to the pass, can be `()` or a struct literal.
macro_rules! compiler_passes {
    ($macro:ident) => {
        $macro! {
            (common_subexpression_elimination_runner, CommonSubexpressionEliminating, ()),
            (const_prop_unroll_and_morphing_runner, ConstPropUnrollAndMorphing, (TypeCheckingInput::new(NetworkName::TestnetV0))),
            (destructuring_runner, Destructuring, ()),
            (dead_code_elimination_runner, DeadCodeEliminating, ()),
            (flattening_runner, Flattening, ()),
            (function_inlining_runner, FunctionInlining, ()),
            (option_lowering_runner, OptionLowering, (TypeCheckingInput::new(NetworkName::TestnetV0))),
            (processing_async_runner, ProcessingAsync, (TypeCheckingInput::new(NetworkName::TestnetV0))),
            (processing_script_runner, ProcessingScript, ()),
            (ssa_forming_runner, SsaForming, (SsaFormingInput { rename_defs: true })),
            (storage_lowering_runner, StorageLowering, (TypeCheckingInput::new(NetworkName::TestnetV0))),
            (write_transforming_runner, WriteTransforming, ())
        }
    };
}

/// Parse a Leo source program into an AST, returning errors via the handler.
fn parse_program(source: &str, handler: &Handler) -> Result<Ast, ()> {
    let node_builder = NodeBuilder::default();
    let filename = FileName::Custom("test".into());

    // Add the source to the session's source map
    let source_file = with_session_globals(|s| s.source_map.new_source(source, filename));

    handler.extend_if_error(parse_ast(handler.clone(), &node_builder, &source_file, &[], NetworkName::TestnetV0))
}

/// Macro to generate a single runner function for a compiler pass.
///
/// Each runner:
/// - Sets up a BufferEmitter and Handler for error/warning reporting.
/// - Runs the first three fixed passes: PathResolution, SymbolTableCreation, TypeChecking.
/// - Runs the specified compiler pass.
/// - Returns the resulting AST or formatted errors/warnings.
macro_rules! make_runner {
    ($runner_name:ident, $pass:ident, $input:expr) => {
        fn $runner_name(source: &str) -> String {
            let buf = BufferEmitter::new();
            let handler = Handler::new(buf.clone());

            create_session_if_not_set_then(|_| {
                // Parse program into AST
                let mut state = match parse_program(source, &handler) {
                    Ok(ast) => CompilerState { ast, handler: handler.clone(), ..Default::default() },
                    Err(()) => return format!("{}{}", buf.extract_errs(), buf.extract_warnings()),
                };

                // Always run these three passes before the tested pass; they populate symbol & type tables,
                // which are required for the following compiler pass to function correctly.
                if handler.extend_if_error(PathResolution::do_pass((), &mut state)).is_err()
                    || handler.extend_if_error(SymbolTableCreation::do_pass((), &mut state)).is_err()
                    || handler
                        .extend_if_error(TypeChecking::do_pass(TypeCheckingInput::new(state.network), &mut state))
                        .is_err()
                {
                    return format!("{}{}", buf.extract_errs(), buf.extract_warnings());
                }

                // Run the specific pass
                if handler.extend_if_error($pass::do_pass($input, &mut state)).is_err() {
                    return format!("{}{}", buf.extract_errs(), buf.extract_warnings());
                }

                // Success: return AST with any warnings
                format!("{}{}", buf.extract_warnings(), state.ast.ast)
            })
        }
    };
}

/// Macro to generate all runners from the compiler_passes table.
macro_rules! make_all_runners {
    ($(($runner:ident, $pass:ident, $input:tt)),* $(,)?) => {
        $(
            make_runner!($runner, $pass, $input);
        )*
    };
}
compiler_passes!(make_all_runners);

/// Macro to generate `#[test]` functions for all compiler passes.
///
/// Each test function:
/// - Uses the runner function generated above.
/// - Uses `leo_test_framework::run_tests` with a path derived from the pass struct name.
/// - Uses `paste::paste!` to safely concatenate identifiers.
macro_rules! make_all_tests {
    ($(($runner:ident, $pass:ident, $input:tt)),* $(,)?) => {
        $(
            paste::paste! {
                #[test]
                #[serial]
                fn [<$runner _test>]() {
                    // Automatically derive the snake_case directory name from the pass name
                    leo_test_framework::run_tests(
                        concat!("passes/", stringify!([<$pass:snake>])),
                        $runner,
                    );
                }
            }
        )*
    };
}
compiler_passes!(make_all_tests);
