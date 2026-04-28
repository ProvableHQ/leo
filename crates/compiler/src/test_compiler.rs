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

use leo_ast::{NodeBuilder, Stub};
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

/// Runs a compiler test by compiling a sequence of stub programs followed by a final program.
///
/// The input string may contain multiple sources separated by `PROGRAM_DELIMITER`, where earlier
/// entries represent dependencies and the final entry is the program under test. Dependencies are
/// compiled either as Leo stubs or Aleo bytecode depending on `stub_type`, then registered into a
/// `Process` to validate correctness and linking behavior.
///
/// Library dependencies are parsed separately and tracked so their parent relationships can be
/// reconstructed for the final program. The function returns all resulting bytecodes concatenated
/// in source order using the same delimiter.
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

    // Process dependency sections in file order so that `import_stubs` reflects
    // the topological ordering expressed in the test file (dependencies before dependents).
    for source in rest {
        if let Some(aleo_source) = super::test_utils::extract_aleo_stub_header(source) {
            let program = handler
                .extend_if_error(disassemble_from_str::<TestnetV0>("", aleo_source).map_err(|err| err.into()))?;
            let name = program.stub_id.as_symbol();
            import_stubs.insert(name, program.into());
            add_aleo_program(aleo_source)?;
            continue;
        }

        if let Some((library_name, library_source)) = super::test_utils::extract_library_header(source) {
            // Library dependency: always parsed as a Leo library regardless of stub mode.
            let library = handler.extend_if_error(super::test_utils::parse_library(
                &library_name,
                library_source,
                handler,
                node_builder,
            ))?;
            import_stubs.insert(Symbol::intern(&library_name), library.into());
            if handler.err_count() != 0 {
                return Err(());
            }
            continue;
        }

        match stub_type {
            StubType::FromLeo => {
                let (program, program_name) = handler.extend_if_error(super::test_utils::parse_program(
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
                // Register this program as a parent of all known libraries so that library
                // constants are visible during compilation.
                let program_name_for_parents =
                    handler.extend_if_error(super::test_utils::extract_program_name(source, handler))?;
                let program_symbol = Symbol::intern(&program_name_for_parents);
                for stub in import_stubs.values_mut() {
                    if stub.is_library() {
                        stub.add_parent(program_symbol);
                    }
                }

                let (programs, program_name) = handler.extend_if_error(super::test_utils::whole_compile(
                    source,
                    handler,
                    node_builder,
                    import_stubs.clone(),
                ))?;

                // Note: also validates that the bytecode is well-formed.
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
        }
    }

    // Extract the name of the final program so we can treat it as a parent.
    let final_program_name = handler.extend_if_error(super::test_utils::extract_program_name(last, handler))?;
    let final_symbol = Symbol::intern(&final_program_name);

    // Populate parents for libraries: parents come after them in the stub order
    {
        let symbols: Vec<Symbol> = import_stubs.keys().copied().collect();

        for (i, symbol) in symbols.iter().enumerate() {
            if let Some(Stub::FromLibrary { parents, .. }) = import_stubs.get_mut(symbol) {
                // Parents that appear later in the stub order
                parents.extend(symbols[i + 1..].iter().copied());

                // The final program also depends on earlier libraries
                parents.insert(final_symbol);
            }
        }
    }

    // Full compile for final program.
    let (compiled, _program_name) =
        handler.extend_if_error(super::test_utils::whole_compile(last, handler, node_builder, import_stubs.clone()))?;

    // Only error out if there are errors. Warnings are okay but we still want to print them later.
    if handler.err_count() != 0 {
        return Err(());
    }

    // In FromLeo mode, register compiled import bytecodes (the Leo dependencies that were
    // compiled as part of the final program). Aleo stubs were already registered during the
    // dependency loop above.
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

// ---------------------------------------------------------------------------
// Interface ABI tests
// ---------------------------------------------------------------------------

/// Compiles a multi-program source (FromLeo mode only) and returns the
/// generated interface ABIs serialized as pretty JSON.
fn abi_runner(source: &str) -> String {
    let buf = BufferEmitter::new();
    let handler = Handler::new(buf.clone());
    let node_builder = Rc::new(NodeBuilder::default());

    create_session_if_not_set_then(|_| {
        let mut import_stubs = IndexMap::new();
        let sources: Vec<&str> = source.split(super::test_utils::PROGRAM_DELIMITER).collect();
        let (last, rest) = sources.split_last().expect("non-empty sources");

        for dep_source in rest {
            if let Some((library_name, library_source)) = super::test_utils::extract_library_header(dep_source) {
                let library =
                    match super::test_utils::parse_library(&library_name, library_source, &handler, &node_builder) {
                        Ok(lib) => lib,
                        Err(_) => return format!("{}{}", buf.extract_errs(), buf.extract_warnings()),
                    };
                import_stubs.insert(Symbol::intern(&library_name), library.into());
                continue;
            }
            let (program, program_name) =
                match super::test_utils::parse_program(dep_source, &handler, &node_builder, import_stubs.clone()) {
                    Ok(p) => p,
                    Err(_) => return format!("{}{}", buf.extract_errs(), buf.extract_warnings()),
                };
            import_stubs.insert(Symbol::intern(&program_name), program.into());
        }

        let compiled = match super::test_utils::whole_compile(last, &handler, &node_builder, import_stubs) {
            Ok((compiled, _name)) => compiled,
            Err(_) => return format!("{}{}", buf.extract_errs(), buf.extract_warnings()),
        };

        let mut interfaces = compiled.interfaces;
        interfaces.sort_by(|a, b| {
            let owner_a = match &a.owner {
                leo_abi::interfaces::InterfaceOwner::Local => "",
                leo_abi::interfaces::InterfaceOwner::External { owner_program } => owner_program,
            };
            let owner_b = match &b.owner {
                leo_abi::interfaces::InterfaceOwner::Local => "",
                leo_abi::interfaces::InterfaceOwner::External { owner_program } => owner_program,
            };
            owner_a.cmp(owner_b).then_with(|| a.abi.path.cmp(&b.abi.path))
        });

        let mut output = String::new();
        for (i, ci) in interfaces.iter().enumerate() {
            if i > 0 {
                output.push('\n');
            }
            let owner_label = match &ci.owner {
                leo_abi::interfaces::InterfaceOwner::Local => "local".to_string(),
                leo_abi::interfaces::InterfaceOwner::External { owner_program } => owner_program.clone(),
            };
            output.push_str(&format!("// owner: {owner_label}\n"));
            output.push_str(&serde_json::to_string_pretty(&ci.abi).unwrap());
            output.push('\n');
        }

        format!("{}{output}", buf.extract_warnings())
    })
}

#[test]
#[serial]
fn test_interface_abi() {
    leo_test_framework::run_tests("interface_abi", abi_runner);
}
