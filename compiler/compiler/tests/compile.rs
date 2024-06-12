// Copyright (C) 2019-2023 Aleo Systems Inc.
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

mod utilities;
use utilities::{
    compile_and_process,
    get_build_options,
    get_cwd_option,
    hash_asts,
    hash_content,
    hash_symbol_tables,
    parse_program,
    BufferEmitter,
    CompileOutput,
    CurrentNetwork,
};

use leo_compiler::{CompilerOptions, OutputOptions};
use leo_disassembler::disassemble_from_str;
use leo_errors::{emitter::Handler, LeoError};
use leo_span::symbol::create_session_if_not_set_then;
use leo_test_framework::{
    runner::{Namespace, ParseType, Runner},
    Test,
    PROGRAM_DELIMITER,
};

use snarkvm::console::prelude::*;

use indexmap::IndexMap;
use leo_span::Symbol;
use regex::Regex;
use serde_yaml::Value;
use snarkvm::{prelude::Process, synthesizer::program::ProgramCore};
use std::{fs, path::Path, rc::Rc};

struct CompileNamespace;

impl Namespace for CompileNamespace {
    fn parse_type(&self) -> ParseType {
        ParseType::Whole
    }

    fn run_test(&self, test: Test) -> Result<Value, String> {
        let buf = BufferEmitter(Rc::default(), Rc::default());
        let handler = Handler::new(Box::new(buf.clone()));
        create_session_if_not_set_then(|_| {
            run_test(test, &handler, &buf).map_err(|()| buf.0.take().to_string() + &buf.1.take().to_string())
        })
    }
}

#[derive(Deserialize, PartialEq, Eq, Serialize)]
struct CompileOutputs {
    pub compile: Vec<CompileOutput>,
}

fn run_test(test: Test, handler: &Handler, buf: &BufferEmitter) -> Result<Value, ()> {
    // Check for CWD option:
    let cwd = get_cwd_option(&test);

    // Extract the compiler build configurations from the config file.
    let build_options = get_build_options(&test.config);

    // Initialize a `Process`. This should always succeed.
    let process = Process::<CurrentNetwork>::load().unwrap();

    let mut all_outputs = Vec::with_capacity(build_options.len());

    for build in build_options {
        let compiler_options = CompilerOptions {
            build,
            output: OutputOptions {
                symbol_table_spans_enabled: false,
                initial_symbol_table: true,
                type_checked_symbol_table: true,
                unrolled_symbol_table: true,
                ast_spans_enabled: false,
                initial_ast: true,
                unrolled_ast: true,
                ssa_ast: true,
                flattened_ast: true,
                destructured_ast: true,
                inlined_ast: true,
                dce_ast: true,
            },
        };

        // Split the test content into individual program strings based on the program delimiter.
        let program_strings = test.content.split(PROGRAM_DELIMITER).collect::<Vec<&str>>();

        // Initialize storage for the stubs.
        let mut import_stubs = IndexMap::new();

        // Clone the process.
        let mut process = process.clone();

        // Initialize storage for the compilation outputs.
        let mut compile = Vec::with_capacity(program_strings.len());

        // Compile each program string separately.
        for program_string in program_strings {
            // Parse the program name from the program string.
            let re = Regex::new(r"program\s+([^\s.]+)\.aleo").unwrap();
            let program_name = re.captures(program_string).unwrap().get(1).unwrap().as_str();

            // Parse the program.
            let mut parsed = handler.extend_if_error(parse_program(
                program_name.to_string(),
                handler,
                program_string,
                cwd.clone(),
                Some(compiler_options.clone()),
                import_stubs.clone(),
            ))?;

            // Compile the program to bytecode.
            let program_name = parsed.program_name.to_string();
            let bytecode = handler.extend_if_error(compile_and_process(&mut parsed))?;

            // Parse the bytecode as an Aleo program.
            // Note that this function checks that the bytecode is well-formed.
            let aleo_program = handler.extend_if_error(ProgramCore::from_str(&bytecode).map_err(LeoError::Anyhow))?;

            // Add the program to the process.
            // Note that this function performs an additional validity check on the bytecode.
            handler.extend_if_error(process.add_program(&aleo_program).map_err(LeoError::Anyhow))?;

            // Add the bytecode to the import stubs.
            let stub = handler.extend_if_error(
                disassemble_from_str::<CurrentNetwork>(&program_name, &bytecode).map_err(|err| err.into()),
            )?;
            import_stubs.insert(Symbol::intern(&program_name), stub);

            // Hash the ast files.
            let (initial_ast, unrolled_ast, ssa_ast, flattened_ast, destructured_ast, inlined_ast, dce_ast) =
                hash_asts(&program_name);

            // Hash the symbol tables.
            let (initial_symbol_table, type_checked_symbol_table, unrolled_symbol_table) =
                hash_symbol_tables(&program_name);

            // Clean up the output directory.
            if fs::read_dir("/tmp/output").is_ok() {
                fs::remove_dir_all(Path::new("/tmp/output")).expect("Error failed to clean up output dir.");
            }

            let output = CompileOutput {
                initial_symbol_table,
                type_checked_symbol_table,
                unrolled_symbol_table,
                initial_ast,
                unrolled_ast,
                ssa_ast,
                flattened_ast,
                destructured_ast,
                inlined_ast,
                dce_ast,
                bytecode: hash_content(&bytecode),
                errors: buf.0.take().to_string(),
                warnings: buf.1.take().to_string(),
            };
            compile.push(output);
        }

        // Combine all compilation outputs.
        let compile_outputs = CompileOutputs { compile };
        all_outputs.push(compile_outputs);
    }
    Ok(serde_yaml::to_value(all_outputs).expect("serialization failed"))
}

struct TestRunner;

impl Runner for TestRunner {
    fn resolve_namespace(&self, name: &str) -> Option<Box<dyn Namespace>> {
        Some(match name {
            "Compile" => Box::new(CompileNamespace),
            _ => return None,
        })
    }
}

#[test]
pub fn compiler_tests() {
    leo_test_framework::run_tests(&TestRunner, "compiler");
}
