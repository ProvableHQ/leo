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
use utilities::{buffer_if_err, compile_and_process, parse_program, BufferEmitter};
use utilities::{get_cwd_option, setup_build_directory, Aleo, Network};

use leo_errors::emitter::Handler;
use leo_span::symbol::create_session_if_not_set_then;
use leo_test_framework::{
    runner::{Namespace, ParseType, Runner},
    Test,
};

use snarkvm::console;
use snarkvm::prelude::*;

use crate::utilities::{hash_asts, hash_content};
use leo_test_framework::test::TestExpectationMode;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::BTreeMap;
use std::{fs, path::Path, rc::Rc};

// TODO: Evaluate namespace.
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
    pub bytecode: String,
    pub results: BTreeMap<String, Vec<BTreeMap<String, String>>>,
}

fn run_test(test: Test, handler: &Handler, err_buf: &BufferEmitter) -> Result<Value, ()> {
    // Check that config expectation is always pass.
    if test.config.expectation != TestExpectationMode::Pass {
        buffer_if_err(
            err_buf,
            Err("Test expectation must be `Pass` for `Execute` tests.".to_string()),
        )?;
    }

    // Check for CWD option:
    let cwd = get_cwd_option(&test);

    // Parse the program.
    let mut parsed = handler.extend_if_error(parse_program(handler, &test.content, cwd))?;

    // Compile the program to bytecode.
    let program_name = format!("{}.{}", parsed.program_name, parsed.network);
    let bytecode = handler.extend_if_error(compile_and_process(&mut parsed))?;

    // Extract the cases from the test config.
    let all_cases = test
        .config
        .extra
        .get("cases")
        .expect("An `Execute` config must have a `cases` field.")
        .as_mapping()
        .unwrap();

    // Initialize a map for the expected results.
    let mut results = BTreeMap::new();

    // Setup the build directory.
    let package = setup_build_directory(&program_name, &bytecode, handler)?;

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
                .get(&Value::from("input"))
                .unwrap()
                .as_sequence()
                .unwrap()
                .iter()
                .map(|input| console::program::Value::<Network>::from_str(input.as_str().unwrap()).unwrap())
                .collect();
            let input_string = format!("[{}]", inputs.iter().map(|input| input.to_string()).join(", "));

            // TODO: Add support for custom config like custom private keys.
            // Execute the program and get the outputs.
            let output_string = match package.run::<Aleo, _>(
                None,
                package.manifest_file().development_private_key(),
                function_name,
                &inputs,
                rng,
            ) {
                Ok((response, _, _, _)) => format!(
                    "[{}]",
                    response.outputs().iter().map(|output| output.to_string()).join(", ")
                ),
                Err(err) => format!("SnarkVMError({err})"),
            };

            // Store the inputs and outputs in a map.
            let mut result = BTreeMap::new();
            result.insert("input".to_string(), input_string);
            result.insert("output".to_string(), output_string);

            // Add the hashes of the inputs and outputs to the function results.
            function_results.push(result);
        }
        results.insert(function_name.to_string(), function_results);
    }

    // Hash the ast files.
    let (initial_ast, unrolled_ast, ssa_ast, flattened_ast) = hash_asts();

    // Clean up the output directory.
    if fs::read_dir("/tmp/output").is_ok() {
        fs::remove_dir_all(Path::new("/tmp/output")).expect("Error failed to clean up output dir.");
    }

    let final_output = ExecuteOutput {
        initial_ast,
        unrolled_ast,
        ssa_ast,
        flattened_ast,
        bytecode: hash_content(&bytecode),
        results,
    };
    Ok(serde_yaml::to_value(final_output).expect("serialization failed"))
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
