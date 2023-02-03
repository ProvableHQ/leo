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
use utilities::{BufferEmitter, collect_all_inputs, compile_and_process, hash_file, parse_program, temp_dir};

use leo_errors::{
    emitter::{Handler},
    LeoError,
};
use leo_span::{symbol::create_session_if_not_set_then};
use leo_test_framework::{
    runner::{Namespace, ParseType, Runner},
    Test,
};

use snarkvm::file::Manifest;
use snarkvm::package::Package;
use snarkvm::prelude::*;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::{
    fs,
    path::{Path},
    rc::Rc,
};
use std::{fs::File, io::Write};
use crate::utilities::buffer_if_err;

type CurrentNetwork = Testnet3;

struct CompileNamespace;

impl Namespace for CompileNamespace {
    fn parse_type(&self) -> ParseType {
        ParseType::Whole
    }

    fn run_test(&self, test: Test) -> Result<Value, String> {
        let buf = BufferEmitter(Rc::default(), Rc::default());
        let handler = Handler::new(Box::new(buf.clone()));

        create_session_if_not_set_then(|_| run_test(test, &handler, &buf).map_err(|()| buf.0.take().to_string()))
    }
}

#[derive(Deserialize, PartialEq, Eq, Serialize)]
struct OutputItem {
    pub initial_input_ast: String,
}

#[derive(Deserialize, PartialEq, Eq, Serialize)]
struct CompileOutput {
    pub output: Vec<OutputItem>,
    pub initial_ast: String,
    pub unrolled_ast: String,
    pub ssa_ast: String,
    pub flattened_ast: String,
}

fn run_test(test: Test, handler: &Handler, err_buf: &BufferEmitter) -> Result<Value, ()> {
    // Check for CWD option:
    // ``` cwd: import ```
    // When set, uses different working directory for current file.
    // If not, uses file path as current working directory.
    let cwd = test.config.get("cwd").map(|val| {
        let mut cwd = test.path.clone();
        cwd.pop();
        cwd.join(val.as_str().unwrap())
    });

    let mut parsed = handler.extend_if_error(parse_program(handler, &test.content, cwd))?;

    // (name, content)
    let inputs = buffer_if_err(err_buf, collect_all_inputs(&test))?;

    let mut output_items = Vec::with_capacity(inputs.len());

    if inputs.is_empty() {
        output_items.push(OutputItem {
            initial_input_ast: "no input".to_string(),
        });
    } else {
        // Parse one or more input files to execute the program with.
        for input in inputs {
            let mut parsed = parsed.clone();
            handler.extend_if_error(parsed.parse_input(input))?;
            let initial_input_ast = hash_file("/tmp/output/test.initial_input_ast.json");

            output_items.push(OutputItem { initial_input_ast });
        }
    };

    // Compile the program to bytecode.
    let program_name = format!("{}.{}", parsed.program_name, parsed.network);
    let bytecode = handler.extend_if_error(compile_and_process(&mut parsed))?;

    // Run snarkvm package.
    {
        // Initialize a temporary directory.
        let directory = temp_dir();

        // Create the program id.
        let program_id = ProgramID::<CurrentNetwork>::from_str(&program_name).unwrap();

        // Write the program string to a file in the temporary directory.
        let path = directory.join("main.aleo");
        let mut file = File::create(path).unwrap();
        file.write_all(bytecode.as_bytes()).unwrap();

        // Create the manifest file.
        let _manifest_file = Manifest::create(&directory, &program_id).unwrap();

        // Create the build directory.
        let build_directory = directory.join("build");
        std::fs::create_dir_all(build_directory).unwrap();

        // Open the package at the temporary directory.
        let package = handler.extend_if_error(Package::<Testnet3>::open(&directory).map_err(LeoError::Anyhow))?;

        // Get the program process and check all instructions.
        handler.extend_if_error(package.get_process().map_err(LeoError::Anyhow))?;
    }

    let initial_ast = hash_file("/tmp/output/test.initial_ast.json");
    let unrolled_ast = hash_file("/tmp/output/test.unrolled_ast.json");
    let ssa_ast = hash_file("/tmp/output/test.ssa_ast.json");
    let flattened_ast = hash_file("/tmp/output/test.flattened_ast.json");

    if fs::read_dir("/tmp/output").is_ok() {
        fs::remove_dir_all(Path::new("/tmp/output")).expect("Error failed to clean up output dir.");
    }

    let final_output = CompileOutput {
        output: output_items,
        initial_ast,
        unrolled_ast,
        ssa_ast,
        flattened_ast,
    };
    Ok(serde_yaml::to_value(final_output).expect("serialization failed"))
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
