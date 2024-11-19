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

/// Build, Prove and Run Leo program with inputs
#[derive(Parser, Debug)]
pub struct LeoTest {
    #[clap(name = "TESTNAME", help = "If specified, only run tests containing this string in their names.")]
    name: Option<String>,
    #[clap(long, help = "Compile, but don't run the tests", default_value = "false")]
    no_run: bool,
    #[clap(long, help = "Run all tests regardless of failure.", default_value = "false")]
    no_fail_fast: bool,
    #[clap(short, long, help = "Number of parallel jobs, the maximum is the number of CPUs.")]
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
        (LeoBuild { options: self.compiler_options.clone() }).execute(context)
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
    // If the number exceeds the number of CPUs, it is clamped to the number of CPUs.
    let num_cpus = num_cpus::get();
    let jobs = command.jobs.unwrap_or(num_cpus).min(num_cpus);

    Ok(())
}
