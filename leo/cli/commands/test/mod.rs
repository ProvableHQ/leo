// Copyright (C) 2019-2024 Aleo Systems Inc.
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

mod result;
use result::*;

mod utilities;
use utilities::*;

use super::*;

use leo_ast::TestManifest;
use leo_errors::TestError;

use snarkvm::prelude::{Itertools, Program, Value, execution_cost_v1, execution_cost_v2};

use rand::{Rng, SeedableRng, rngs::OsRng};
use rand_chacha::ChaChaRng;
use rayon::{ThreadPoolBuilder, prelude::*};

/// Build, Prove and Run Leo program with inputs
#[derive(Parser, Debug)]
pub struct LeoTest {
    #[clap(name = "FILTER", help = "If specified, only run tests containing this string in their names.")]
    filter: Option<String>,
    #[clap(long, help = "Compile, but don't run the tests", default_value = "false")]
    no_run: bool,
    #[clap(long, help = "Run all tests regardless of failure.", default_value = "false")]
    no_fail_fast: bool,
    #[clap(short, long, help = "Number of parallel jobs, defaults to the number of CPUs.")]
    jobs: Option<usize>,
    #[clap(flatten)]
    compiler_options: BuildOptions,
}

impl Command for LeoTest {
    type Input = <LeoBuild as Command>::Output;
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        // Set `build_tests` to `true` to ensure that the tests are built.
        let mut options = self.compiler_options.clone();
        options.build_tests = true;
        (LeoBuild { options }).execute(context)
    }

    fn apply(self, context: Context, _input: Self::Input) -> Result<Self::Output> {
        // Parse the network.
        let network = NetworkName::try_from(context.get_network(&self.compiler_options.network)?)?;
        match network {
            NetworkName::TestnetV0 => handle_test::<AleoTestnetV0>(self, context),
            NetworkName::MainnetV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Mainnet chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                return handle_test::<AleoV0>(self, context);
            }
            NetworkName::CanaryV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Canary chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                return handle_test::<AleoCanaryV0>(self, context);
            }
        }
    }
}

// A helper function to handle the `test` command.
fn handle_test<A: Aleo>(command: LeoTest, context: Context) -> Result<<LeoTest as Command>::Output> {
    // Select the number of jobs, defaulting to the number of CPUs.
    let num_cpus = num_cpus::get();
    let jobs = command.jobs.unwrap_or(num_cpus);

    // Initialize the Rayon thread pool.
    ThreadPoolBuilder::new().num_threads(jobs).build_global().map_err(TestError::default_error)?;

    // Get the individual test directories within the build directory at `<PACKAGE_PATH>/build/tests`
    let package_path = context.dir()?;
    let build_directory = BuildDirectory::open(&package_path)?;
    let tests_directory = build_directory.join("tests");
    let test_directories = std::fs::read_dir(tests_directory)
        .map_err(TestError::default_error)?
        .flat_map(|dir_entry| dir_entry.map(|dir_entry| dir_entry.path()))
        .collect_vec();

    println!("Running {} tests...", test_directories.len());

    // For each test package within the tests directory:
    //  - Open the manifest.
    //  - Initialize the VM.
    //  - Run each test within the manifest sequentially.
    let results: Vec<_> = test_directories
        .into_par_iter()
        .map(|path| -> String {
            // The default bug message.
            const BUG_MESSAGE: &str =
                "This is a bug, please file an issue at https://github.com/ProvableHQ/leo/issues/new?template=0_bug.md";
            // Load the `manifest.json` within the test directory.
            let manifest_path = path.join("manifest.json");
            let manifest_json = match std::fs::read_to_string(&manifest_path) {
                Ok(manifest_json) => manifest_json,
                Err(e) => return format!("Failed to read manifest.json: {e}. {BUG_MESSAGE}"),
            };
            let manifest: TestManifest<A::Network> = match serde_json::from_str(&manifest_json) {
                Ok(manifest) => manifest,
                Err(e) => return format!("Failed to parse manifest.json: {e}. {BUG_MESSAGE}"),
            };

            // Get the programs in the package in the following order:
            //  - If the `imports` directory exists, load the programs from the `imports` directory.
            //  - Load the main program from the package.
            let mut program_paths = Vec::new();
            let imports_directory = path.join("imports");
            if let Ok(dir) = std::fs::read_dir(&imports_directory) {
                if let Ok(paths) =
                    dir.map(|dir_entry| dir_entry.map(|dir_entry| dir_entry.path())).collect::<Result<Vec<_>, _>>()
                {
                    program_paths.extend(paths);
                }
            };
            program_paths.push(path.join("main.aleo"));

            // Read and parse the programs.
            let mut programs = Vec::with_capacity(program_paths.len());
            for path in program_paths {
                let program_string = match std::fs::read_to_string(&path) {
                    Ok(program_string) => program_string,
                    Err(e) => return format!("Failed to read program: {e}. {BUG_MESSAGE}"),
                };
                let program = match Program::<A::Network>::from_str(&program_string) {
                    Ok(program) => program,
                    Err(e) => return format!("Failed to parse program: {e}. {BUG_MESSAGE}"),
                };
                programs.push(program);
            }

            // Initialize the VM.
            let (vm, genesis_private_key) = match initialize_vm(programs) {
                Ok((vm, genesis_private_key)) => (vm, genesis_private_key),
                Err(e) => return format!("Failed to initialize VM: {e}. {BUG_MESSAGE}"),
            };

            // Initialize the results object.
            let mut results = TestResults::new(manifest.program_id.clone());

            // Run each test within the manifest.
            for test in manifest.tests {
                // Get the full test name.
                let test_name = format!("{}/{}", manifest.program_id, test.function_name);

                // If a filter is specified, skip the test if it does not contain the filter.
                if let Some(filter) = &command.filter {
                    if !test_name.contains(filter) {
                        results.skip(test_name);
                        continue;
                    }
                }

                // Get the seed if specified, otherwise use a random seed.
                let seed = match test.seed {
                    Some(seed) => seed,
                    None => OsRng.gen(),
                };

                // Initialize the RNG.
                let rng = &mut ChaChaRng::seed_from_u64(seed);

                // Use the private key if provided, otherwise initialize one using the RNG.
                let private_key = match test.private_key {
                    Some(private_key) => private_key,
                    None => match PrivateKey::new(rng) {
                        Ok(private_key) => private_key,
                        Err(e) => {
                            results
                                .add_result(test_name, format!("Failed to generate private key: {e}. {BUG_MESSAGE}"));
                            continue;
                        }
                    },
                };

                // Determine whether or not the function should fail.
                let should_fail = test.should_fail;

                // Execute the function.
                let inputs: Vec<Value<A::Network>> = Vec::new();
                let authorization = match vm.process().read().authorize::<A, _>(
                    &private_key,
                    &manifest.program_id,
                    &test.function_name,
                    inputs.iter(),
                    rng,
                ) {
                    Ok(authorization) => authorization,
                    Err(e) => {
                        results.add_result(test_name, format!("Failed to authorize: {e}. {BUG_MESSAGE}"));
                        continue;
                    }
                };
                let Transaction::Execute(_, execution, _) =
                    (match vm.execute_authorization(authorization, None, None, rng) {
                        Ok(transaction) => transaction,
                        Err(e) => {
                            // TODO (@d0cd) A failure here may not be a bug.
                            results.add_result(test_name, format!("Failed to execute: {e}. {BUG_MESSAGE}"));
                            continue;
                        }
                    })
                else {
                    unreachable!("VM::execute_authorization always produces an execution")
                };
                let block_height = vm.block_store().current_block_height();
                let result = match block_height < A::Network::CONSENSUS_V2_HEIGHT {
                    true => execution_cost_v1(&vm.process().read(), &execution),
                    false => execution_cost_v2(&vm.process().read(), &execution),
                };
                let base_fee_in_microcredits = match result {
                    Ok((total, _)) => total,
                    Err(e) => {
                        results.add_result(test_name, format!("Failed to get execution cost: {e}. {BUG_MESSAGE}"));
                        continue;
                    }
                };
                let execution_id = match execution.to_execution_id() {
                    Ok(execution_id) => execution_id,
                    Err(e) => {
                        results.add_result(test_name, format!("Failed to get execution ID: {e}. {BUG_MESSAGE}"));
                        continue;
                    }
                };
                let fee_authorization =
                    match vm.authorize_fee_public(&genesis_private_key, base_fee_in_microcredits, 0, execution_id, rng)
                    {
                        Ok(fee_authorization) => fee_authorization,
                        Err(e) => {
                            results.add_result(test_name, format!("Failed to authorize fee: {e}. {BUG_MESSAGE}"));
                            continue;
                        }
                    };
                let fee = match vm.execute_fee_authorization(fee_authorization, None, rng) {
                    Ok(transaction) => transaction,
                    Err(e) => {
                        results
                            .add_result(test_name, format!("Failed to execute fee authorization: {e}. {BUG_MESSAGE}"));
                        continue;
                    }
                };
                let transaction = match Transaction::from_execution(execution, Some(fee)) {
                    Ok(transaction) => transaction,
                    Err(e) => {
                        results.add_result(test_name, format!("Failed to construct transaction: {e}. {BUG_MESSAGE}"));
                        continue;
                    }
                };
                let is_verified = vm.check_transaction(&transaction, None, rng).is_ok();
                let (block, is_accepted) = match construct_next_block(&vm, &genesis_private_key, transaction, rng) {
                    Ok(block) => block,
                    Err(e) => {
                        results.add_result(test_name, format!("Failed to create block: {e}. {BUG_MESSAGE}"));
                        continue;
                    }
                };

                if let Err(e) = vm.add_next_block(&block) {
                    results.add_result(test_name, format!("Failed to add block: {e}. {BUG_MESSAGE}"));
                    continue;
                }

                // Construct the result.
                match (is_verified & is_accepted, should_fail) {
                    (true, true) => results.add_result(test_name, " ❌ Test passed but should have failed".to_string()),
                    (false, false) => {
                        results.add_result(test_name, " ❌ Test failed but should have passed".to_string())
                    }
                    (true, false) => results.add_result(test_name, " ✅ Test passed as expected".to_string()),
                    (false, true) => results.add_result(test_name, " ✅ Test failed as expected".to_string()),
                }
            }

            // Return the results as a string.
            results.to_string()
        })
        .collect();

    // Print the results.
    for result in results {
        println!("{result}");
    }

    Ok(())
}
