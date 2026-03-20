// Copyright (C) 2019-2026 Provable Inc.
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
(runner_name, [(PassStruct, input), ...])
```

- `runner_name` – the function name for this pass runner (snake_case).
- `[(PassStruct, input), ...]` – a list of passes to run sequentially. Each entry is a tuple of `(pass_struct, input)`.
- `input` – the argument to the pass. Can be `()` if none, or a struct literal like `(SsaFormingInput { rename_defs: true })`.

Examples:

```rust
// Single pass with typical prelude
(new_pass_runner, [
    (ForceInlineConversion, ()),
    (GlobalVarsCollection, ()),
    (PathResolution, ()),
    (GlobalItemsCollection, ()),
    (TypeChecking, (TypeCheckingInput::new(NetworkName::TestnetV0))),
    (NewPass, (NewPassInput { option: true }))
]),

// Multiple passes run sequentially
(multi_pass_runner, [
    (ForceInlineConversion, ()),
    (GlobalVarsCollection, ()),
    (PathResolution, ()),
    (GlobalItemsCollection, ()),
    (TypeChecking, (TypeCheckingInput::new(NetworkName::TestnetV0))),
    (FirstPass, ()),
    (SecondPass, (SecondPassInput { value: NetworkName::TestnetV0 }))
]),

// Pass without prelude (if prelude not needed)
(no_prelude_runner, [
    (SomePass, ())
]),
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
use indexmap::{IndexMap, IndexSet};
use leo_ast::{NetworkName, Stub};
use leo_errors::{BufferEmitter, Handler, Result};
use leo_parser::{parse_library, parse_program};
use leo_span::{Symbol, create_session_if_not_set_then, source_map::FileName, with_session_globals};
use serial_test::serial;
use std::rc::Rc;

/// Table of all compiler passes and their runner names.
/// Each entry is a tuple of `(runner_name, [(pass_struct, input), ...])`
/// - `runner_name` – the function name for this pass runner (snake_case).
/// - `[(pass_struct, input), ...]` – a list of passes to run sequentially. Each entry is a tuple of `(pass_struct, input)`.
///   Include the prelude passes (GlobalVarsCollection, PathResolution, GlobalItemsCollection, TypeChecking) at the beginning if needed.
/// - `input` – the argument to the pass. Can be `()` if none, or a struct literal like `(SsaFormingInput { rename_defs: true })`.
macro_rules! compiler_passes {
    ($macro:ident) => {
        $macro! {
            (common_subexpression_elimination_runner, [
                (GlobalVarsCollection, ()),
                (PathResolution, ()),
                (GlobalItemsCollection, ()),
                (TypeChecking, (TypeCheckingInput::new(NetworkName::TestnetV0))),
                (Disambiguate, ()),
                (CommonSubexpressionEliminating, ())
            ]),
            (const_prop_unroll_and_morphing_runner, [
                (GlobalVarsCollection, ()),
                (PathResolution, ()),
                (GlobalItemsCollection, ()),
                (TypeChecking, (TypeCheckingInput::new(NetworkName::TestnetV0))),
                (Disambiguate, ()),
                (ConstPropUnrollAndMorphing, (TypeCheckingInput::new(NetworkName::TestnetV0)))
            ]),
            (destructuring_runner, [
                (GlobalVarsCollection, ()),
                (PathResolution, ()),
                (GlobalItemsCollection, ()),
                (TypeChecking, (TypeCheckingInput::new(NetworkName::TestnetV0))),
                (Disambiguate, ()),
                (Destructuring, ())
            ]),
            (dead_code_elimination_runner, [
                (GlobalVarsCollection, ()),
                (PathResolution, ()),
                (GlobalItemsCollection, ()),
                (TypeChecking, (TypeCheckingInput::new(NetworkName::TestnetV0))),
                (Disambiguate, ()),
                (DeadCodeEliminating, ())
            ]),
            (flattening_runner, [
                (GlobalVarsCollection, ()),
                (PathResolution, ()),
                (GlobalItemsCollection, ()),
                (TypeChecking, (TypeCheckingInput::new(NetworkName::TestnetV0))),
                (Disambiguate, ()),
                (SsaForming, (SsaFormingInput { rename_defs: true })),
                (Flattening, ())
            ]),
            (function_inlining_runner, [
                (GlobalVarsCollection, ()),
                (PathResolution, ()),
                (GlobalItemsCollection, ()),
                (TypeChecking, (TypeCheckingInput::new(NetworkName::TestnetV0))),
                (Disambiguate, ()),
                (FunctionInlining, ())
            ]),
            (option_lowering_runner, [
                (GlobalVarsCollection, ()),
                (PathResolution, ()),
                (GlobalItemsCollection, ()),
                (TypeChecking, (TypeCheckingInput::new(NetworkName::TestnetV0))),
                (Disambiguate, ()),
                (OptionLowering, (TypeCheckingInput::new(NetworkName::TestnetV0)))
            ]),
            (processing_async_runner, [
                (GlobalVarsCollection, ()),
                (PathResolution, ()),
                (GlobalItemsCollection, ()),
                (TypeChecking, (TypeCheckingInput::new(NetworkName::TestnetV0))),
                (Disambiguate, ()),
                (ProcessingAsync, (TypeCheckingInput::new(NetworkName::TestnetV0)))
            ]),
            (ssa_forming_runner, [
                (GlobalVarsCollection, ()),
                (PathResolution, ()),
                (GlobalItemsCollection, ()),
                (TypeChecking, (TypeCheckingInput::new(NetworkName::TestnetV0))),
                (Disambiguate, ()),
                (SsaForming, (SsaFormingInput { rename_defs: true }))
            ]),
            (storage_lowering_runner, [
                (GlobalVarsCollection, ()),
                (PathResolution, ()),
                (GlobalItemsCollection, ()),
                (TypeChecking, (TypeCheckingInput::new(NetworkName::TestnetV0))),
                (Disambiguate, ()),
                (StorageLowering, (TypeCheckingInput::new(NetworkName::TestnetV0)))
            ]),
            (write_transforming_runner, [
                (GlobalVarsCollection, ()),
                (PathResolution, ()),
                (GlobalItemsCollection, ()),
                (TypeChecking, (TypeCheckingInput::new(NetworkName::TestnetV0))),
                (Disambiguate, ()),
                (WriteTransforming, ())
            ]),
            (remove_unreachable_runner, [
                (RemoveUnreachable, ())
            ]),
            (ssa_const_propagation_runner, [
                (GlobalVarsCollection, ()),
                (PathResolution, ()),
                (GlobalItemsCollection, ()),
                (TypeChecking, (TypeCheckingInput::new(NetworkName::TestnetV0))),
                (Disambiguate, ()),
                (SsaForming, (SsaFormingInput { rename_defs: true })),
                (SsaConstPropagation, ()),
            ]),
            (disambiguate_runner, [
                (GlobalVarsCollection, ()),
                (PathResolution, ()),
                (GlobalItemsCollection, ()),
                (TypeChecking, (TypeCheckingInput::new(NetworkName::TestnetV0))),
                (Disambiguate, ()),
            ]),
            (check_interfaces_runner, [
                (GlobalVarsCollection, ()),
                (PathResolution, ()),
                (GlobalItemsCollection, ()),
                (CheckInterfaces, ()),
            ]),
        }
    };
}

/// Macro to generate a single runner function for compiler passes.
///
/// Each runner:
/// - Sets up a BufferEmitter and Handler for error/warning reporting.
/// - Parse the test into an AST.
/// - Runs the specified list of compiler passes sequentially.
/// - Returns the resulting AST or formatted errors/warnings.
macro_rules! make_runner {
    ($runner_name:ident, [$(($pass:ident, $input:expr)),* $(,)?]) => {
        fn $runner_name(source: &str) -> String {
            let buf = BufferEmitter::new();
            let handler = Handler::new(buf.clone());
            let node_builder = Rc::new(leo_ast::NodeBuilder::default());

            create_session_if_not_set_then(|_| {
                let mut state = CompilerState { handler: handler.clone(), node_builder: Rc::clone(&node_builder), ..Default::default() };

                state.ast = match handler.extend_if_error(parse_passes_test_source(
                    source,
                    &handler,
                    &node_builder,
                    NetworkName::TestnetV0,
                )) {
                    Ok(ast) => ast,
                    Err(()) => return format!("{}{}", buf.extract_errs(), buf.extract_warnings()),
                };

                // Run the specified passes sequentially
                $(
                    if handler.extend_if_error($pass::do_pass($input, &mut state)).is_err() {
                        return format!("{}{}", buf.extract_errs(), buf.extract_warnings());
                    }
                )*

                // Success: return AST with any warnings
                format!("{}{}", buf.extract_warnings(), state.ast)
            })
        }
    };
}

/// Macro to generate all runners from the compiler_passes table.
macro_rules! make_all_runners {
    ($(($runner:ident, $passes:tt)),* $(,)?) => {
        $(
            make_runner!($runner, $passes);
        )*
    };
}
compiler_passes!(make_all_runners);

/// Macro to generate `#[test]` functions for all compiler passes.
///
/// Each test function:
/// - Uses the runner function generated above.
/// - Uses `leo_test_framework::run_tests` with a path derived from the last pass struct name (the actual pass being tested).
/// - Uses `paste::paste!` to safely concatenate identifiers.
macro_rules! make_all_tests {
    ($(($runner:ident, [$(($pass:ident, $input:tt)),* $(,)?])),* $(,)?) => {
        $(
            paste::paste! {
                #[test]
                #[serial]
                fn [<$runner _test>]() {
                    // Automatically derive the snake_case directory name from the last pass name (the actual pass being tested)
                    // We need to extract the last pass from the list
                    make_all_tests_inner!($runner, [$(($pass, $input)),*]);
                }
            }
        )*
    };
}

/// Helper macro to extract the last pass name from the list.
macro_rules! make_all_tests_inner {
    ($runner:ident, [($pass:ident, $input:tt)]) => {
        paste::paste! {
            leo_test_framework::run_tests(
                concat!("passes/", stringify!([<$pass:snake>])),
                $runner,
            );
        }
    };
    ($runner:ident, [($pass:ident, $input:tt), $(($rest_pass:ident, $rest_input:tt)),+ $(,)?]) => {
        make_all_tests_inner!($runner, [$(($rest_pass, $rest_input)),+]);
    };
}

compiler_passes!(make_all_tests);

/// Delimiter used to separate library / program sections in multi-part passes tests.
const PASSES_PROGRAM_DELIMITER: &str = "// --- Next Program --- //";

/// Parses a passes test source string into an `Ast`.
///
/// If the source contains no `PASSES_PROGRAM_DELIMITER`, it is treated as a
/// single standalone program (the existing behaviour). Otherwise the source is
/// split into sections:
///
/// - Each section before the last may be a library header
///   (`// --- library: NAME --- //` followed by library source) or a plain
///   program section (currently ignored in passes tests).
/// - The last section is the main program.
///
/// Library stubs are parsed and injected into the main program's `stubs` map
/// so that the prelude passes (GlobalVarsCollection, PathResolution, TypeChecking,
/// …) see the library declarations exactly as they would during a full compile.
fn parse_passes_test_source(
    source: &str,
    handler: &Handler,
    node_builder: &Rc<leo_ast::NodeBuilder>,
    network: NetworkName,
) -> Result<leo_ast::Ast> {
    if !source.contains(PASSES_PROGRAM_DELIMITER) {
        // Fast path: single-program source, existing behaviour.
        let sf = with_session_globals(|s| s.source_map.new_source(source, FileName::Custom("test".into())));
        return parse_program(handler.clone(), node_builder, &sf, &[], network).map(leo_ast::Ast::Program);
    }

    // Multi-part source: split into sections.
    let sections: Vec<&str> = source.split(PASSES_PROGRAM_DELIMITER).collect();
    let (main_section, dep_sections) = sections.split_last().expect("split always yields at least one element");
    let main_source = main_section.trim();

    // Parse library stubs from dependency sections.
    let mut stubs: IndexMap<Symbol, Stub> = IndexMap::new();
    for section in dep_sections {
        let trimmed = section.trim();
        if let Some((lib_name, lib_source)) = extract_passes_library_header(trimmed) {
            let sf =
                with_session_globals(|s| s.source_map.new_source(lib_source, FileName::Custom("compiler-test".into())));
            let library = parse_library(handler.clone(), node_builder, lib_name, &sf, network)?;
            stubs.insert(lib_name, Stub::FromLibrary { library, parents: IndexSet::new() });
        }
        // Non-library program stubs are not needed for individual-pass tests.
    }

    // Parse the main program to obtain its declared name.
    let main_sf = with_session_globals(|s| s.source_map.new_source(main_source, FileName::Custom("test".into())));
    let mut program = parse_program(handler.clone(), node_builder, &main_sf, &[], network)?;

    let main_symbol = program
        .program_scopes
        .values()
        .next()
        .map(|scope| scope.program_id.as_symbol())
        .expect("a program must have at least one scope");

    // Establish parent relationships so GlobalVarsCollection sets up visibility
    // correctly: later stubs are parents of earlier ones (transitive deps), and
    // the main program is a parent of every library it uses.
    let lib_symbols: Vec<Symbol> = stubs.keys().copied().collect();
    for (i, sym) in lib_symbols.iter().enumerate() {
        if let Some(Stub::FromLibrary { parents, .. }) = stubs.get_mut(sym) {
            parents.extend(lib_symbols[i + 1..].iter().copied());
            parents.insert(main_symbol);
        }
    }

    // Inject the collected stubs into the parsed program.
    program.stubs = stubs;

    Ok(leo_ast::Ast::Program(program))
}

/// Extracts a `// --- library: NAME --- //` header from the start of `source`.
///
/// Returns `(library_name_symbol, source_after_header)` on success, or `None`
/// if the source does not begin with a library header.
fn extract_passes_library_header(source: &str) -> Option<(Symbol, &str)> {
    let mut offset = 0;
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("// --- library:") && trimmed.ends_with("--- //") {
            let name = trimmed.trim_start_matches("// --- library:").trim_end_matches("--- //").trim();
            let rest = source[offset + line.len()..].trim_start_matches('\n');
            return Some((Symbol::intern(name), rest));
        }
        offset += line.len() + 1;
    }
    None
}
