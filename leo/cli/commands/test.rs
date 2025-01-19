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

use super::*;

use leo_ast::TestManifest;
use leo_errors::TestError;

use snarkvm::prelude::{Itertools, Value};

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
    #[clap(long, help = "Skip running the native tests.", default_value = "false")]
    skip_native: bool,
    // TODO: The default should eventually be `false`.
    #[clap(long, help = "Skip running the interpreted tests.", default_value = "true")]
    skip_interpreted: bool,
    // TODO: The default should eventually be `false`.
    #[clap(long, help = "Skip running the end-to-end tests.", default_value = "true")]
    skip_end_to_end: bool,
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
    ThreadPoolBuilder::new().num_threads(jobs).build_global().map_err(TestError::test_error)?;

    // Get the individual test directories within the build directory at `<PACKAGE_PATH>/build/tests`
    let package_path = context.dir()?;
    let build_directory = BuildDirectory::open(&package_path)?;
    let tests_directory = build_directory.join("tests");
    let test_directories = std::fs::read_dir(tests_directory)
        .map_err(TestError::test_error)?
        .into_iter()
        .flat_map(|dir_entry| dir_entry.map(|dir_entry| dir_entry.path()))
        .collect_vec();

    println!("Running tests...");
    println!("Test directories: {:?}", test_directories);

    // For each test package within the tests directory:
    //  - Open the package as a snarkVM `Package`.
    //  - Open the manifest.
    //  - Initialize the VM.
    //  - Run each test within the manifest sequentially.
    let errors: Vec<_> = test_directories
        .into_par_iter()
        .map(|path| -> Result<<LeoTest as Command>::Output> {
            // Open the package as a snarkVM `Package`.
            let package = snarkvm::package::Package::<A::Network>::open(&path).map_err(TestError::test_error)?;

            // Load the `manifest.json` within the test directory.
            let manifest_path = path.join("manifest.json");
            let manifest_json = std::fs::read_to_string(&manifest_path).map_err(TestError::test_error)?;
            let manifest: TestManifest<A::Network> =
                serde_json::from_str(&manifest_json).map_err(TestError::test_error)?;

            // TODO (@d0cd). Change this to VM initialization.
            // Initialize the process.
            let process = package.get_process()?;

            // TODO (@d0cd). Handle this correctly.
            // Initialize a vector to aggregate the results.
            // let results = Vec::with_capacity(manifest.tests.len());

            // Run each test within the manifest.
            for test in manifest.tests {
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
                    None => PrivateKey::new(rng)?,
                };

                // Determine whether or not the function should fail.
                let should_fail = test.should_fail;

                // Execute the function.
                let inputs: Vec<Value<A::Network>> = Vec::new();
                let authorization = process.authorize::<A, _>(
                    &private_key,
                    &manifest.program_id,
                    &test.function_name,
                    inputs.iter(),
                    rng,
                )?;
                let result = process.execute::<A, _>(authorization, rng);

                // Construct the result.
                // TODO (@d0cd) Pipe result output.
                match result {
                    Ok(_) => println!("Test passed!"),
                    Err(e) => println!("Test failed: {e}"),
                }
            }

            Ok(())
        })
        .collect();

    Ok(())
}
