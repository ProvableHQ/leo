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

use leo_ast::NodeBuilder;
use leo_disassembler::disassemble_from_str;
use leo_errors::{BufferEmitter, Handler, LeoError};
use leo_span::{Symbol, create_session_if_not_set_then};

use snarkvm::{
    prelude::{Process, TestnetV0},
    synthesizer::program::ProgramCore,
};

use indexmap::IndexMap;
use itertools::Itertools as _;
use serial_test::serial;
use std::{rc::Rc, str::FromStr};

// This determines how the dependent programs are imported. They can either be imported as Leo
// programs directly (which usually allows more features to be supported such as external generic
// structs, etc.) or as Aleo programs (such as depending on the deployed credits.aleo).
#[derive(PartialEq)]
enum StubType {
    FromLeo,
    FromAleo,
}

fn run_test(stub_type: StubType, test: &str, handler: &Handler, node_builder: &Rc<NodeBuilder>) -> Result<String, ()> {
    let mut process = Process::<TestnetV0>::load().unwrap();

    let mut import_stubs = IndexMap::new();

    let mut bytecodes = Vec::<String>::new();

    let sources: Vec<&str> = test.split(super::test_utils::PROGRAM_DELIMITER).collect();

    // Helper: parse and register Aleo program into `process`.
    // Note that this performs an additional validity check on the bytecode.
    let mut add_aleo_program = |code: &str| -> Result<(), ()> {
        let program = handler.extend_if_error(ProgramCore::from_str(code).map_err(LeoError::Anyhow))?;
        handler.extend_if_error(process.add_program(&program).map_err(LeoError::Anyhow))?;
        Ok(())
    };

    let (last, rest) = sources.split_last().expect("non-empty sources");

    for source in rest {
        match stub_type {
            StubType::FromLeo => {
                let (program, program_name) = handler.extend_if_error(super::test_utils::parse(
                    source,
                    handler,
                    node_builder,
                    import_stubs.clone(),
                ))?;

                import_stubs.insert(Symbol::intern(&program_name), program.into());

                if handler.err_count() != 0 {
                    return Err(());
                }
            }
            StubType::FromAleo => {
                let (programs, program_name) = handler.extend_if_error(super::test_utils::whole_compile(
                    source,
                    handler,
                    node_builder,
                    import_stubs.clone(),
                ))?;

                // Parse the bytecode as an Aleo program.
                // Note that this function checks that the bytecode is well-formed.
                add_aleo_program(&programs.primary.bytecode)?;

                let program = handler.extend_if_error(
                    disassemble_from_str::<TestnetV0>(&program_name, &programs.primary.bytecode)
                        .map_err(|err| err.into()),
                )?;

                import_stubs.insert(Symbol::intern(&program_name), program.into());

                if handler.err_count() != 0 {
                    return Err(());
                }

                bytecodes.push(programs.primary.bytecode);
            }
        };
    }

    // Full compile for final program.
    let (compiled, _program_name) =
        handler.extend_if_error(super::test_utils::whole_compile(last, handler, node_builder, import_stubs.clone()))?;

    // Only error out if there are errors. Warnings are okay but we still want to print them later.
    if handler.err_count() != 0 {
        return Err(());
    }

    // Add imports but only if the imports are in Leo form. Aleo stubs are added earlier.
    if stub_type == StubType::FromLeo {
        for import in &compiled.imports {
            add_aleo_program(&import.bytecode)?;
            bytecodes.push(import.bytecode.clone());
        }
    }

    // Add main program.
    let primary_bytecode = compiled.primary.bytecode.clone();
    add_aleo_program(&primary_bytecode)?;
    bytecodes.push(primary_bytecode);

    Ok(bytecodes.iter().format(&format!("{}\n", super::test_utils::PROGRAM_DELIMITER)).to_string())
}

/// Runs the test twice:
/// 1. Treating dependencies as Leo
/// 2. Treating dependencies as Aleo
///
/// If the outputs differ, both results are reported with a clear explanation.
fn runner(source: &str) -> String {
    let from_leo_output = run_with_stub(StubType::FromLeo, source);
    let from_aleo_output = run_with_stub(StubType::FromAleo, source);

    if from_leo_output != from_aleo_output {
        format!(
            "{from_leo_output}\n\n\
             ---\n\
             Note: Treating dependencies as Aleo produces different results:\n\n\
             {from_aleo_output}"
        )
    } else {
        from_leo_output
    }
}

/// Runs a single test pass with the given stub type and returns all diagnostics
/// (errors, warnings, and output) as a single formatted string.
fn run_with_stub(stub: StubType, source: &str) -> String {
    let buf = BufferEmitter::new();
    let handler = Handler::new(buf.clone());
    let node_builder = Rc::new(NodeBuilder::default());

    create_session_if_not_set_then(|_| {
        match run_test(stub, source, &handler, &node_builder) {
            Ok(output) => {
                // Successful compilation: warnings (if any) followed by output
                format!("{}{}", buf.extract_warnings(), output)
            }
            Err(()) => {
                // Failed compilation: errors first, then warnings
                format!("{}{}", buf.extract_errs(), buf.extract_warnings())
            }
        }
    })
}

#[test]
#[serial]
fn test_compiler() {
    leo_test_framework::run_tests("compiler", runner);
}
