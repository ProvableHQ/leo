// Copyright (C) 2019-2022 Aleo Systems Inc.
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
use utilities::{compile_and_process, hash_content, hash_file, parse_program, temp_dir, BufferEmitter};

use leo_errors::{emitter::Handler, LeoError};
use leo_span::symbol::create_session_if_not_set_then;
use leo_test_framework::{
    runner::{Namespace, ParseType, Runner},
    Test,
};

use snarkvm::console;
use snarkvm::file::Manifest;
use snarkvm::package::Package;
use snarkvm::prelude::*;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::BTreeMap;
use std::{fs, path::Path, rc::Rc};
use std::{fs::File, io::Write};

type Network = Testnet3;
pub type Aleo = snarkvm::circuit::AleoV0;

struct ExecuteNamespace;

impl Namespace for ExecuteNamespace {
    fn parse_type(&self) -> ParseType {
        ParseType::Whole
    }

    fn run_test(&self, test: Test) -> Result<Value, String> {
        let buf = BufferEmitter(Rc::default(), Rc::default());
        let handler = Handler::new(Box::new(buf.clone()));

        create_session_if_not_set_then(|_| run_test(test, &handler, &buf).map_err(|()| buf.0.take().to_string()))
    }
}

// TODO: Format this better.
#[derive(Deserialize, PartialEq, Eq, Serialize)]
struct ExecuteOutput {
    pub initial_ast: String,
    pub unrolled_ast: String,
    pub ssa_ast: String,
    pub flattened_ast: String,
    pub results: BTreeMap<String, Vec<(String, String)>>,
}

fn run_test(test: Test, handler: &Handler, _err_buf: &BufferEmitter) -> Result<Value, ()> {
    // Check for CWD option:
    // ``` cwd: import ```
    // When set, uses different working directory for current file.
    // If not, uses file path as current working directory.
    let cwd = test.config.get("cwd").map(|val| {
        let mut cwd = test.path.clone();
        cwd.pop();
        cwd.join(val.as_str().unwrap())
    });

    // Parse the program.
    let mut parsed = handler.extend_if_error(parse_program(handler, &test.content, cwd))?;

    // Compile the program to bytecode.
    let program_name = format!("{}.{}", parsed.program_name, parsed.network);
    let bytecode = handler.extend_if_error(compile_and_process(&mut parsed))?;

    // Extract the cases from the test config.
    let all_cases = test
        .config
        .get("cases")
        .expect("An `Execute` config must have a `cases` field.")
        .as_mapping()
        .unwrap();

    // Initialize a map for the expected results.
    let mut results = BTreeMap::new();

    // Run snarkvm package.
    {
        // Initialize a temporary directory.
        let directory = temp_dir();

        // Create the program id.
        let program_id = ProgramID::<Network>::from_str(&program_name).unwrap();

        // Write the program string to a file in the temporary directory.
        let path = directory.join("main.aleo");
        let mut file = File::create(path).unwrap();
        file.write_all(bytecode.as_bytes()).unwrap();

        // Create the manifest file.
        let manifest_file = Manifest::create(&directory, &program_id).unwrap();

        // Create the build directory.
        let build_directory = directory.join("build");
        std::fs::create_dir_all(build_directory).unwrap();

        // Open the package at the temporary directory.
        let package = handler.extend_if_error(Package::<Testnet3>::open(&directory).map_err(LeoError::Anyhow))?;

        // Initialize an rng.
        let rng = &mut rand::thread_rng();

        // Run each test case for each function.
        for (function_name, function_cases) in all_cases {
            let function_name = Identifier::from_str(function_name.as_str().unwrap()).unwrap();
            let cases = function_cases.as_sequence().unwrap();
            let mut function_results = Vec::with_capacity(cases.len());

            for case in cases {
                let case = case.as_mapping().unwrap();
                let inputs: Vec<_> = case
                    .get(&Value::from("inputs"))
                    .unwrap()
                    .as_sequence()
                    .unwrap()
                    .iter()
                    .map(|input| console::program::Value::<Network>::from_str(input.as_str().unwrap()).unwrap())
                    .collect();
                let inputs_hash = hash_content(&format!(
                    "[{}]",
                    inputs.iter().map(|input| input.to_string()).join(", ")
                ));

                let outputs: Vec<_> = case
                    .get(&Value::from("expected"))
                    .unwrap()
                    .as_sequence()
                    .unwrap()
                    .iter()
                    .map(|output| console::program::Value::<Network>::from_str(output.as_str().unwrap()).unwrap())
                    .collect();
                let outputs_hash = hash_content(&format!(
                    "[{}]",
                    outputs.iter().map(|output| output.to_string()).join(", ")
                ));

                // Execute the program and get the outputs.
                let (_response, _, _, _) = handler.extend_if_error(
                    package
                        .run::<Aleo, _>(
                            None,
                            manifest_file.development_private_key(),
                            function_name,
                            &inputs,
                            rng,
                        )
                        .map_err(LeoError::Anyhow),
                )?;

                // TODO: Check that outputs match

                // Add the hashes of the inputs and outputs to the function results.
                function_results.push((inputs_hash, outputs_hash));
            }
            results.insert(function_name.to_string(), function_results);
        }
    }

    let initial_ast = hash_file("/tmp/output/test.initial_ast.json");
    let unrolled_ast = hash_file("/tmp/output/test.unrolled_ast.json");
    let ssa_ast = hash_file("/tmp/output/test.ssa_ast.json");
    let flattened_ast = hash_file("/tmp/output/test.flattened_ast.json");

    if fs::read_dir("/tmp/output").is_ok() {
        fs::remove_dir_all(Path::new("/tmp/output")).expect("Error failed to clean up output dir.");
    }

    let final_output = ExecuteOutput {
        initial_ast,
        unrolled_ast,
        ssa_ast,
        flattened_ast,
        results,
    };
    Ok(serde_yaml::to_value(&final_output).expect("serialization failed"))
}

struct TestRunner;

impl Runner for TestRunner {
    fn resolve_namespace(&self, name: &str) -> Option<Box<dyn Namespace>> {
        Some(match name {
            "Execute" => Box::new(ExecuteNamespace),
            _ => return None,
        })
    }
}

#[test]
pub fn execution_tests() {
    leo_test_framework::run_tests(&TestRunner, "execution");
}
