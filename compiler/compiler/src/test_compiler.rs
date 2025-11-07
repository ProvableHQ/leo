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

use leo_ast::NodeBuilder;
use leo_errors::{BufferEmitter, Handler, LeoError};
use leo_passes::Bytecode;
use leo_span::{Symbol, create_session_if_not_set_then};

use snarkvm::{
    prelude::{Process, TestnetV0},
    synthesizer::program::ProgramCore,
};

use indexmap::IndexMap;
use itertools::Itertools as _;
use serial_test::serial;
use std::{rc::Rc, str::FromStr};

fn run_test(test: &str, handler: &Handler, node_builder: &Rc<NodeBuilder>) -> Result<String, ()> {
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

    // Parse-only stage for intermediate programs.
    for source in rest {
        let (program, program_name) =
            handler.extend_if_error(super::test_utils::parse(source, handler, node_builder, import_stubs.clone()))?;

        import_stubs.insert(Symbol::intern(&program_name), program.into());

        if handler.err_count() != 0 {
            return Err(());
        }
    }

    // Full compile for final program.
    let (compiled_programs, _program_name) =
        handler.extend_if_error(super::test_utils::whole_compile(last, handler, node_builder, import_stubs.clone()))?;

    // Only error out if there are errors. Warnings are okay but we still want to print them later.
    if handler.err_count() != 0 {
        return Err(());
    }

    // Add imports.
    for Bytecode { bytecode, .. } in compiled_programs.import_bytecodes {
        add_aleo_program(&bytecode)?;
        bytecodes.push(bytecode.clone());
    }

    // Add main program.
    let primary_bytecode = compiled_programs.primary_bytecode.clone();
    add_aleo_program(&primary_bytecode)?;
    bytecodes.push(primary_bytecode);

    Ok(bytecodes.iter().format(&format!("{}\n", super::test_utils::PROGRAM_DELIMITER)).to_string())
}

fn runner(source: &str) -> String {
    let buf = BufferEmitter::new();
    let handler = Handler::new(buf.clone());
    let node_builder = Rc::new(NodeBuilder::default());

    create_session_if_not_set_then(|_| match run_test(source, &handler, &node_builder) {
        Ok(x) => format!("{}{}", buf.extract_warnings(), x),
        Err(()) => format!("{}{}", buf.extract_errs(), buf.extract_warnings()),
    })
}

#[test]
#[serial]
fn test_compiler() {
    leo_test_framework::run_tests("compiler", runner);
}
