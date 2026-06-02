// Copyright (C) 2019-2026 Provable Inc.
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

//! `leo run` clap surface. The actual run logic lives in
//! [`leo_cli_core::commands::run::handle_run`]; this file just collects
//! flags, builds the package if applicable, and forwards.

use super::*;

use leo_ast::NetworkName;
use leo_cli_core::commands::run::{RunArgs, RunOutput};
use leo_package::Package;

use snarkvm::circuit::AleoTestnetV0;
#[cfg(not(feature = "only_testnet"))]
use snarkvm::circuit::{AleoCanaryV0, AleoV0};

/// Run the Leo program with the given inputs, without generating a proof.
#[derive(Parser, Debug)]
pub struct LeoRun {
    #[clap(
        name = "NAME",
        help = "The name of the function to execute, e.g `helloworld.aleo::main` or `main`.",
        default_value = "main"
    )]
    pub(crate) name: String,
    #[clap(
        name = "INPUTS",
        help = "The program inputs e.g. `1u32`, `record1...` (record ciphertext), or `{ owner: ...}` "
    )]
    pub(crate) inputs: Vec<String>,
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
    #[clap(flatten)]
    pub(crate) build_options: BuildOptions,
    #[clap(
        long = "with",
        help = "Additional programs to load into the VM (comma-separated). \
            If a path exists locally, it is read as an .aleo bytecode file; \
            otherwise it is fetched from the network endpoint.",
        value_delimiter = ','
    )]
    pub(crate) with: Vec<String>,
}

impl Command for LeoRun {
    type Input = Option<Package>;
    type Output = RunOutput;

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        let path = context.dir()?;
        let home_path = context.home()?;
        // If the current directory is a valid Leo package, then build it.
        if Package::from_directory_no_graph(
            path,
            home_path,
            self.env_override.network,
            self.env_override.endpoint.as_deref(),
            self.env_override.network_retries,
            leo_cli_core::package_fetch::fetch_compilation_unit,
        )
        .is_ok()
        {
            let package = LeoBuild { env_override: self.env_override.clone(), options: self.build_options.clone() }
                .execute(context)?;
            Ok(Some(package))
        } else {
            Ok(None)
        }
    }

    fn apply(self, context: Context, input: Self::Input) -> Result<Self::Output> {
        let network = match get_network(&self.env_override.network) {
            Ok(n) => n,
            Err(_) => {
                println!("⚠️ No network specified, defaulting to 'testnet'.");
                NetworkName::TestnetV0
            }
        };

        let home_path = context.home()?;
        let args = RunArgs {
            name: self.name.clone(),
            inputs: self.inputs.clone(),
            with: &self.with,
            private_key: &self.env_override.private_key,
            endpoint: &self.env_override.endpoint,
            network_retries: self.env_override.network_retries,
            network,
            home_path: &home_path,
            package: input.as_ref(),
        };

        match network {
            NetworkName::TestnetV0 => leo_cli_core::commands::run::handle_run::<AleoTestnetV0>(args),
            NetworkName::MainnetV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Mainnet chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                leo_cli_core::commands::run::handle_run::<AleoV0>(args)
            }
            NetworkName::CanaryV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Canary chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                leo_cli_core::commands::run::handle_run::<AleoCanaryV0>(args)
            }
        }
    }
}
