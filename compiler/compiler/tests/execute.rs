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
    buffer_if_err,
    compile_and_process,
    dotenv_private_key,
    get_build_options,
    get_cwd_option,
    hash_asts,
    hash_content,
    hash_symbol_tables,
    parse_program,
    setup_build_directory,
    Aleo,
    BufferEmitter,
    Network,
};

use leo_compiler::{CompilerOptions, OutputOptions};
use leo_errors::emitter::Handler;
use leo_span::symbol::create_session_if_not_set_then;
use leo_test_framework::{
    runner::{Namespace, ParseType, Runner},
    test::TestExpectationMode,
    Test,
};

use snarkvm::{console, prelude::*};

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::{collections::BTreeMap, fs, path::Path, rc::Rc};

// TODO: Evaluate namespace.
struct ExecuteNamespace;

impl Namespace for ExecuteNamespace {
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

// TODO: Format this better.
#[derive(Deserialize, PartialEq, Eq, Serialize)]
struct ExecuteOutput {
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
    pub results: BTreeMap<String, Vec<BTreeMap<String, String>>>,
}

fn run_test(test: Test, handler: &Handler, err_buf: &BufferEmitter) -> Result<Value, ()> {
    // Check that config expectation is always pass.
    if test.config.expectation != TestExpectationMode::Pass {
        buffer_if_err(err_buf, Err("Test expectation must be `Pass` for `Execute` tests.".to_string()))?;
    }

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
        let rng = &mut TestRng::default();

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
                let private_key = match case.get(&Value::from("private_key")) {
                    Some(private_key) => {
                        PrivateKey::from_str(private_key.as_str().expect("expected string for private key"))
                            .expect("unable to parse private key")
                    }
                    None => dotenv_private_key(package.directory()).unwrap(),
                };

                // TODO: Add support for custom config like custom private keys.
                // Execute the program and get the outputs.
                let output_string = match package.run::<Aleo, _>(&private_key, function_name, &inputs, rng) {
                    Ok((response, _)) => format!(
                        "[{}]",
                        response
                            .outputs()
                            .iter()
                            .map(|output| {
                                match output {
                                    // Remove the `_nonce` from the record string.
                                    console::program::Value::Record(record) => {
                                        let pattern = Regex::new(r"_nonce: \d+group.public").unwrap();
                                        pattern.replace(&record.to_string(), "").to_string()
                                    }
                                    _ => output.to_string(),
                                }
                            })
                            .join(", ")
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
        let (initial_ast, unrolled_ast, ssa_ast, flattened_ast, destructured_ast, inlined_ast, dce_ast) = hash_asts();

        // Hash the symbol tables.
        let (initial_symbol_table, type_checked_symbol_table, unrolled_symbol_table) = hash_symbol_tables();

        // Clean up the output directory.
        if fs::read_dir("/tmp/output").is_ok() {
            fs::remove_dir_all(Path::new("/tmp/output")).expect("Error failed to clean up output dir.");
        }

        let final_output = ExecuteOutput {
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
            warnings: err_buf.1.take().to_string(),
            results,
        };
        outputs.push(final_output);
    }
    Ok(serde_yaml::to_value(outputs).expect("serialization failed"))
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
