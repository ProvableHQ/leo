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
    get_build_options,
    get_cwd_option,
    hash_asts,
    hash_content,
    hash_symbol_tables,
    parse_program,
    BufferEmitter,
    CompileOutput,
    CurrentAleo,
    CurrentNetwork,
    ExecuteOutput,
};

use leo_compiler::{CompilerOptions, OutputOptions};
use leo_errors::emitter::Handler;
use leo_span::symbol::create_session_if_not_set_then;
use leo_test_framework::{
    runner::{Namespace, ParseType, Runner},
    test::TestExpectationMode,
    Test,
    PROGRAM_DELIMITER,
};

use snarkvm::{console, prelude::*};

use disassembler::disassemble_from_str;
use indexmap::IndexMap;
use leo_errors::LeoError;
use leo_span::Symbol;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use snarkvm::synthesizer::program::ProgramCore;
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
        create_session_if_not_set_then(|_| {
            run_test(test, &handler, &buf).map_err(|()| buf.0.take().to_string() + &buf.1.take().to_string())
        })
    }
}

#[derive(Deserialize, PartialEq, Eq, Serialize)]
struct CompileAndExecuteOutputs {
    pub compile: Vec<CompileOutput>,
    pub execute: Vec<ExecuteOutput>,
}

fn run_test(test: Test, handler: &Handler, buf: &BufferEmitter) -> Result<Value, ()> {
    // Check that config expectation is always pass.
    if test.config.expectation != TestExpectationMode::Pass {
        buffer_if_err(buf, Err("Test expectation must be `Pass` for `Execute` tests.".to_string()))?;
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

        // Initialize a `Process`. This should always succeed.
        let mut process = Process::<CurrentNetwork>::load().unwrap();

        // Initialize storage for the compilation outputs.
        let mut compile = Vec::with_capacity(program_strings.len());

        // Compile each program string separately.
        for program_string in program_strings {
            // Parse the program.
            let mut parsed = handler.extend_if_error(parse_program(
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
            let stub = handler.extend_if_error(disassemble_from_str(&bytecode).map_err(|err| err.into()))?;
            import_stubs.insert(Symbol::intern(&program_name), stub);

            // Hash the ast files.
            let (initial_ast, unrolled_ast, ssa_ast, flattened_ast, destructured_ast, inlined_ast, dce_ast) =
                hash_asts();

            // Hash the symbol tables.
            let (initial_symbol_table, type_checked_symbol_table, unrolled_symbol_table) = hash_symbol_tables();

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
                warnings: buf.1.take().to_string(),
            };

            compile.push(output);
        }

        // Extract the cases from the test config.
        let all_cases = test
            .config
            .extra
            .get("cases")
            .expect("An `Execute` config must have a `cases` field.")
            .as_sequence()
            .unwrap();

        // Initialize storage for the execution outputs.
        let mut execute = Vec::with_capacity(all_cases.len());

        // Initialize an rng.
        let rng = &mut match test.config.extra.get("seed").map(|seed| seed.as_u64()) {
            Some(Some(seed)) => TestRng::from_seed(seed),
            _ => TestRng::default(),
        };

        // Run each test case for each function.
        for case in all_cases {
            let case = case.as_mapping().unwrap();
            let program_name = case.get(&Value::from("program")).expect("expected program name").as_str().unwrap();
            let function_name = case.get(&Value::from("function")).expect("expected function name").as_str().unwrap();
            let inputs: Vec<_> = case
                .get(&Value::from("input"))
                .unwrap()
                .as_sequence()
                .unwrap()
                .iter()
                .map(|input| console::program::Value::<CurrentNetwork>::from_str(input.as_str().unwrap()).unwrap())
                .collect();
            let input_string = format!("[{}]", inputs.iter().map(|input| input.to_string()).join(", "));
            let private_key = match case.get(&Value::from("private_key")) {
                Some(private_key) => {
                    PrivateKey::from_str(private_key.as_str().expect("expected string for private key"))
                        .expect("unable to parse private key")
                }
                None => PrivateKey::new(rng).unwrap(),
            };

            // TODO: Add support for custom config like custom private keys.
            // Compute the authorization, execute, and return the result as a string.
            let output_string = match process
                .authorize::<CurrentAleo, _>(&private_key, program_name, function_name, inputs.iter(), rng)
                .and_then(|authorization| process.execute::<CurrentAleo, _>(authorization, rng))
            {
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
                Err(err) => format!("SnarkVMExecutionError({err})"),
            };

            // Construct the output.
            let output = ExecuteOutput {
                program: program_name.to_string(),
                function: function_name.to_string(),
                inputs: input_string,
                outputs: output_string,
            };
            execute.push(output);
        }
        // Construct the combined output.
        let combined_output = CompileAndExecuteOutputs { compile, execute };
        outputs.push(combined_output);
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
