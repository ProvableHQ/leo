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

use crate::{BuildOptions, Compiler, CompilerOptions};

use leo_ast::Stub;
use leo_disassembler::disassemble_from_str;
use leo_errors::{
    LeoError,
    emitter::{BufferEmitter, Handler},
};
use leo_span::{
    source_map::FileName,
    symbol::{Symbol, create_session_if_not_set_then},
};

use snarkvm::{
    prelude::{Process, TestnetV0},
    synthesizer::program::ProgramCore,
};

use indexmap::IndexMap;
use itertools::Itertools as _;
use serial_test::serial;
use std::str::FromStr;

pub const PROGRAM_DELIMITER: &str = "// --- Next Program --- //";

pub fn whole_compile(
    source: &str,
    dce_enabled: bool,
    handler: &Handler,
    import_stubs: IndexMap<Symbol, Stub>,
) -> Result<(String, String), LeoError> {
    let options = CompilerOptions { build: BuildOptions { dce_enabled, ..Default::default() }, ..Default::default() };

    let mut compiler =
        Compiler::<TestnetV0>::new(handler.clone(), "/fakedirectory-wont-use".into(), Some(options), import_stubs);

    let filename = FileName::Custom("compiler-test".into());

    let bytecode = compiler.compile(source, filename)?;

    Ok((bytecode, compiler.program_name))
}

fn run_test(test: &str, dce_enabled: bool, handler: &Handler) -> Result<String, ()> {
    // Initialize a `Process`. This should always succeed.
    let mut process = Process::<TestnetV0>::load().unwrap();

    let mut import_stubs = IndexMap::new();

    let mut bytecodes = Vec::<String>::new();

    // Compile each source file separately.
    for source in test.split(PROGRAM_DELIMITER) {
        let (bytecode, program_name) =
            handler.extend_if_error(whole_compile(source, dce_enabled, handler, import_stubs.clone()))?;

        // Parse the bytecode as an Aleo program.
        // Note that this function checks that the bytecode is well-formed.
        let aleo_program = handler.extend_if_error(ProgramCore::from_str(&bytecode).map_err(LeoError::Anyhow))?;

        // Add the program to the process.
        // Note that this function performs an additional validity check on the bytecode.
        handler.extend_if_error(process.add_program(&aleo_program).map_err(LeoError::Anyhow))?;

        // Add the bytecode to the import stubs.
        let stub = handler
            .extend_if_error(disassemble_from_str::<TestnetV0>(&program_name, &bytecode).map_err(|err| err.into()))?;
        import_stubs.insert(Symbol::intern(&program_name), stub);

        if handler.err_count() != 0 || handler.warning_count() != 0 {
            return Err(());
        }

        bytecodes.push(bytecode);
    }

    Ok(bytecodes.iter().format(&format!("{}\n", PROGRAM_DELIMITER)).to_string())
}

fn runner(source: &str) -> String {
    fn run(source: &str, dce_enabled: bool) -> String {
        let buf = BufferEmitter::new();
        let handler = Handler::new(buf.clone());

        match run_test(source, dce_enabled, &handler) {
            Ok(x) => x,
            Err(()) => format!("{}{}", buf.extract_errs(), buf.extract_warnings()),
        }
    }

    let with_dce_enabled = create_session_if_not_set_then(|_| run(source, true));

    let with_dce_disabled = create_session_if_not_set_then(|_| run(source, false));

    format!("DCE_ENABLED:\n{with_dce_enabled}\nDCE_DISABLED:\n{with_dce_disabled}")
}

#[test]
#[serial]
fn test_compiler() {
    leo_test_framework::run_tests("compiler", runner);
}
