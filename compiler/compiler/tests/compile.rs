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
    setup_build_directory,
    BufferEmitter,
};

use leo_compiler::{CompilerOptions, OutputOptions};
use leo_errors::{emitter::Handler, LeoError};
use leo_span::symbol::create_session_if_not_set_then;
use leo_test_framework::{
    runner::{Namespace, ParseType, Runner},
    Test,
};

use snarkvm::console::prelude::*;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;
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
struct CompileOutput {
    pub initial_symbol_table: String,
    pub type_checked_symbol_table: String,
    pub unrolled_symbol_table: String,
    pub initial_ast: String,
    pub unrolled_ast: String,
    pub ssa_ast: String,
    pub flattened_ast: String,
    pub destructured_ast: String,
    pub inlined_ast: String,
    pub dce_ast: String,
    pub bytecode: String,
    pub warnings: String,
}

fn run_test(test: Test, handler: &Handler, buf: &BufferEmitter) -> Result<Value, ()> {
    // Check for CWD option:
    let cwd = get_cwd_option(&test);

    // Extract the compiler build configurations from the config file.
    let build_options = get_build_options(&test.config);

    let mut outputs = Vec::with_capacity(build_options.len());

    for build in build_options {
        let compiler_options = CompilerOptions {
            build,
            output: OutputOptions {
                symbol_table_spans_enabled: false,
                initial_symbol_table: true,
                type_checked_symbol_table: true,
                unrolled_symbol_table: true,
                ast_spans_enabled: false,
                initial_input_ast: true,
                initial_ast: true,
                unrolled_ast: true,
                ssa_ast: true,
                flattened_ast: true,
                destructured_ast: true,
                inlined_ast: true,
                dce_ast: true,
            },
        };

        // Parse the program.
        let mut parsed =
            handler.extend_if_error(parse_program(handler, &test.content, cwd.clone(), Some(compiler_options)))?;

        // Compile the program to bytecode.
        let program_name = format!("{}.{}", parsed.program_name, parsed.network);
        let bytecode = handler.extend_if_error(compile_and_process(&mut parsed))?;

        // Set up the build directory.
        // Note that this function checks that the bytecode is well-formed.
        let package = setup_build_directory(&program_name, &bytecode, handler)?;

        // Get the program process and check all instructions.
        handler.extend_if_error(package.get_process().map_err(LeoError::Anyhow))?;

        // Hash the ast files.
        let (initial_ast, unrolled_ast, ssa_ast, flattened_ast, destructured_ast, inlined_ast, dce_ast) = hash_asts();

        // Hash the symbol tables.
        let (initial_symbol_table, type_checked_symbol_table, unrolled_symbol_table) = hash_symbol_tables();

        // Clean up the output directory.
        if fs::read_dir("/tmp/output").is_ok() {
            fs::remove_dir_all(Path::new("/tmp/output")).expect("Error failed to clean up output dir.");
        }

        let final_output = CompileOutput {
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
            warnings: buf.1.take().to_string(),
        };

        outputs.push(final_output);
    }
    Ok(serde_yaml::to_value(outputs).expect("serialization failed"))
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
