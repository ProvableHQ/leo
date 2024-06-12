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

use aleo_std_storage::StorageMode;
use snarkvm::{console, prelude::*};

use indexmap::IndexMap;
use leo_disassembler::disassemble_from_str;
use leo_errors::LeoError;
use leo_span::Symbol;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use snarkvm::{
    prelude::store::{helpers::memory::ConsensusMemory, ConsensusStore},
    synthesizer::program::ProgramCore,
};
use std::{fs, panic::AssertUnwindSafe, path::Path, rc::Rc};

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

    // Initialize an rng.
    let rng = &mut match test.config.extra.get("seed").map(|seed| seed.as_u64()) {
        Some(Some(seed)) => TestRng::from_seed(seed),
        _ => TestRng::from_seed(1234567890),
    };

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

        // Initialize a `VM`. This should always succeed.
        let vm =
            VM::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::from(ConsensusStore::open(None).unwrap()).unwrap();

        // Initialize a genesis private key.
        let genesis_private_key = PrivateKey::new(rng).unwrap();

        // Construct the genesis block.
        let genesis_block = vm.genesis_beacon(&genesis_private_key, rng).unwrap();

        // Initialize a `Ledger`. This should always succeed.
        let ledger =
            Ledger::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::load(genesis_block, StorageMode::Production)
                .unwrap();

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

            // Add the program to the ledger.
            // Note that this function performs an additional validity check on the bytecode.
            let deployment = handler.extend_if_error(
                ledger.vm().deploy(&genesis_private_key, &aleo_program, None, 0, None, rng).map_err(LeoError::Anyhow),
            )?;
            let block = handler.extend_if_error(
                ledger
                    .prepare_advance_to_next_beacon_block(&genesis_private_key, vec![], vec![], vec![deployment], rng)
                    .map_err(LeoError::Anyhow),
            )?;
            handler.extend_if_error(ledger.advance_to_next_block(&block).map_err(LeoError::Anyhow))?;

            // Check that the deployment transaction was accepted.
            if block.transactions().num_accepted() != 1 {
                handler.emit_err(LeoError::Anyhow(anyhow!("Deployment transaction was not accepted.")));
            }

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
            let private_key = match case.get(&Value::from("private_key")) {
                Some(private_key) => {
                    PrivateKey::from_str(private_key.as_str().expect("expected string for private key"))
                        .expect("unable to parse private key")
                }
                None => genesis_private_key,
            };

            // Check if the vm contains the program.
            println!("Program exists: {}", ledger.vm().contains_program(&ProgramID::from_str(program_name).unwrap()));

            // Initialize the statuses of execution.
            let mut execution = None;
            let mut verified = false;
            let mut status = "none";

            // Execute the program, construct a block and add it to the ledger.
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                ledger
                    .vm()
                    .execute(&private_key, (program_name, function_name), inputs.iter(), None, 0, None, rng)
                    .and_then(|transaction| {
                        verified = ledger.vm().check_transaction(&transaction, None, rng).is_ok();
                        execution = Some(transaction.clone());
                        ledger.prepare_advance_to_next_beacon_block(
                            &private_key,
                            vec![],
                            vec![],
                            vec![transaction],
                            rng,
                        )
                    })
                    .and_then(|block| {
                        status = match block.aborted_transaction_ids().is_empty() {
                            false => "aborted",
                            true => match block.transactions().num_accepted() == 1 {
                                true => "accepted",
                                false => "rejected",
                            },
                        };
                        ledger.advance_to_next_block(&block)
                    })
            }));

            // Emit any errors from panics.
            match result {
                Err(err) => {
                    handler.emit_err(LeoError::Anyhow(anyhow!("PanicError({:?})", err)));
                }
                Ok(Err(err)) => {
                    handler.emit_err(LeoError::Anyhow(anyhow!("SnarkVMError({:?})", err)));
                }
                _ => {}
            }

            // Extract the execution and remove the global state root.
            let execution = match execution {
                Some(Transaction::Execute(_, execution, _)) => {
                    let proof = execution.proof().cloned();
                    let transitions = execution.into_transitions();
                    Some(Execution::from(transitions, Default::default(), proof).unwrap())
                }
                _ => None,
            };

            // Aggregate the output.
            let output = ExecuteOutput {
                execution,
                verified,
                status: status.to_string(),
                errors: buf.0.take().to_string(),
                warnings: buf.1.take().to_string(),
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
