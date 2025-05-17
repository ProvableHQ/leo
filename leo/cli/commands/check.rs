// Copyright (C) 2019-2025 Provable Inc.
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

use leo_errors::CliError;
use leo_package::{NetworkName, Package, ProgramData};
use leo_span::Symbol;

use snarkvm::prelude::{MainnetV0, Network, Program, ProgramID, TestnetV0};

use indexmap::IndexMap;
use snarkvm::prelude::CanaryV0;

/// Compile the program(s) and check for validity against the network.
#[derive(Parser, Debug)]
pub struct LeoCheck {
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
    #[clap(flatten)]
    pub(crate) extra: ExtraOptions,
    #[clap(flatten)]
    pub(crate) build_options: BuildOptions,
}

impl Command for LeoCheck {
    type Input = Package;
    type Output = Package;

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        // Run the build command to compile the program(s).
        LeoBuild { options: self.build_options.clone(), env_override: self.env_override.clone() }.execute(context)
    }

    fn apply(self, context: Context, package: Self::Input) -> Result<Self::Output> {
        // Parse the network.
        let network: NetworkName = context.get_network(&self.env_override.network)?.parse()?;
        match network {
            NetworkName::MainnetV0 => handle_check::<MainnetV0>(&self, context, package),
            NetworkName::TestnetV0 => handle_check::<TestnetV0>(&self, context, package),
            NetworkName::CanaryV0 => handle_check::<CanaryV0>(&self, context, package),
        }
    }
}

// A helper function to handle the check command.
fn handle_check<N: Network>(
    command: &LeoCheck,
    context: Context,
    package: Package,
) -> Result<<LeoCheck as Command>::Output> {
    let home_path = context.home()?;

    // Get the network, accounting for overrides.
    let network: NetworkName = context.get_network(&command.env_override.network)?.parse()?;

    // Get the endpoint, accounting for overrides.
    let endpoint: String = context.get_endpoint(&command.env_override.endpoint)?;

    // Accumulate errors.
    let mut errors = Vec::<String>::new();

    // Get the programs and optional manifests for all the programs.
    let programs_and_manifests = package
        .get_programs_and_manifests(&home_path)?
        .into_iter()
        .map(|(program_name, program_string, manifest)| {
            // Parse the program ID from the program name.
            let program_id = ProgramID::<N>::from_str(&format!("{}.aleo", program_name))
                .map_err(|e| CliError::custom(format!("Failed to parse program ID: {e}")))?;
            // Parse the program bytecode.
            let bytecode = Program::<N>::from_str(&program_string)
                .map_err(|e| CliError::custom(format!("Failed to parse program: {e}")))?;
            Ok((program_id, (bytecode, manifest)))
        })
        .collect::<Result<IndexMap<_, _>>>()?;

    // Split the programs in the package into local and network dependencies.
    let (_, remote) =
        programs_and_manifests.into_iter().partition::<IndexMap<_, _>, _>(|(_, (_, manifest))| manifest.is_some());

    // Check that the all the remote programs exist on the network.
    for (program_id, (local_program, _)) in remote {
        // Get the latest version of the program from the network.
        match leo_package::Program::fetch(
            Symbol::intern(&program_id.name().to_string()),
            &home_path,
            network,
            &endpoint,
            true,
        ) {
            Ok(program) => {
                let ProgramData::Bytecode(bytecode) = &program.data else {
                    panic!("Expected bytecode when fetching a remote program");
                };
                // Check that the program source code matches the one in the package.
                let remote_program = Program::<N>::from_str(bytecode)
                    .map_err(|e| CliError::custom(format!("Failed to parse program: {e}")))?;
                if local_program != remote_program {
                    errors.push(format!(
                        "The dependency `{}` has been does not match the local one. The local copy has been updated to match the remote one.",
                        program_id
                    ));
                }
            }
            Err(err) => {
                errors.push(format!("Could not find remote dependency '{program_id}' on the network: {err}"));
                continue;
            }
        }
    }

    // Pretty print the errors.
    if !errors.is_empty() {
        println!("⚠️ The following issues while checking the program(s):\n");
        for error in errors {
            println!("  - {error}");
        }
        return Err(CliError::custom("Issues found while checking the program(s)").into());
    }
    Ok(package)
}
