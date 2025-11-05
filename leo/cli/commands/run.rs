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

use leo_ast::{NetworkName, TEST_PRIVATE_KEY};
use leo_package::{Package, ProgramData};

use aleo_std::StorageMode;

use clap::Parser;

#[cfg(not(feature = "only_testnet"))]
use snarkvm::circuit::{AleoCanaryV0, AleoV0};
use snarkvm::{
    circuit::{Aleo, AleoTestnetV0},
    prelude::{
        Identifier,
        ProgramID,
        VM,
        store::{ConsensusStore, helpers::memory::ConsensusMemory},
    },
};

/// Run the Leo program with the given inputs, without generating a proof.
#[derive(Parser, Debug)]
pub struct LeoRun {
    #[clap(
        name = "NAME",
        help = "The name of the function to execute, e.g `helloworld.aleo/main` or `main`.",
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
}

impl Command for LeoRun {
    type Input = Option<Package>;
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        // Get the path to the current directory.
        let path = context.dir()?;
        // Get the path to the home directory.
        let home_path = context.home()?;
        // If the current directory is a valid Leo package, then build it.
        if Package::from_directory_no_graph(
            path,
            home_path,
            self.env_override.network,
            self.env_override.endpoint.as_deref(),
        )
        .is_ok()
        {
            let package = LeoBuild { env_override: self.env_override.clone(), options: self.build_options.clone() }
                .execute(context)?;
            // Return the package.
            Ok(Some(package))
        } else {
            Ok(None)
        }
    }

    fn apply(self, context: Context, input: Self::Input) -> Result<Self::Output> {
        // Get the network, defaulting to `TestnetV0` if none is specified.
        let network = match get_network(&self.env_override.network) {
            Ok(network) => network,
            Err(_) => {
                println!("⚠️ No network specified, defaulting to 'testnet'.");
                NetworkName::TestnetV0
            }
        };

        // Handle each network with the appropriate parameterization.
        match network {
            NetworkName::TestnetV0 => handle_run::<AleoTestnetV0>(self, context, network, input),
            NetworkName::MainnetV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Mainnet chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                handle_run::<AleoV0>(self, context, network, input)
            }
            NetworkName::CanaryV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Canary chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                handle_run::<AleoCanaryV0>(self, context, network, input)
            }
        }
    }
}

// A helper function to handle the `run` command.
fn handle_run<A: Aleo>(
    command: LeoRun,
    context: Context,
    network: NetworkName,
    package: Option<Package>,
) -> Result<<LeoRun as Command>::Output> {
    // Get the private key, defaulting to a test key.
    let private_key = match get_private_key::<A::Network>(&command.env_override.private_key) {
        Ok(private_key) => private_key,
        Err(_) => {
            println!("⚠️ No valid private key specified, defaulting to '{TEST_PRIVATE_KEY}'.");
            PrivateKey::<A::Network>::from_str(TEST_PRIVATE_KEY).expect("Failed to parse the test private key")
        }
    };

    // Parse the <NAME> into an optional program name and a function name.
    // If only a function name is provided, then use the program name from the package.
    let (program_name, function_name) = match command.name.split_once('/') {
        Some((program_name, function_name)) => (program_name.to_string(), function_name.to_string()),
        None => match &package {
            Some(package) => (
                format!(
                    "{}.aleo",
                    package.programs.last().expect("There must be at least one program in a Leo package").name
                ),
                command.name,
            ),
            None => {
                return Err(CliError::custom(format!(
                    "Running `leo execute {} ...`, without an explicit program name requires that your current working directory is a valid Leo project.",
                    command.name
                )).into());
            }
        },
    };

    // Parse the program name as a `ProgramID`.
    let program_id = ProgramID::<A::Network>::from_str(&program_name)
        .map_err(|e| CliError::custom(format!("Failed to parse program name: {e}")))?;
    // Parse the function name as an `Identifier`.
    let function_id = Identifier::<A::Network>::from_str(&function_name)
        .map_err(|e| CliError::custom(format!("Failed to parse function name: {e}")))?;

    // Get all the dependencies in the package if it exists.
    // Get the programs and optional manifests for all programs.
    let programs = if let Some(package) = &package {
        // Get the package directories.
        let build_directory = package.build_directory();
        let imports_directory = package.imports_directory();
        let source_directory = package.source_directory();
        // Get the program names and their bytecode.
        package
            .programs
            .iter()
            .clone()
            .map(|program| {
                let program_id = ProgramID::<A::Network>::from_str(&format!("{}.aleo", program.name))
                    .map_err(|e| CliError::custom(format!("Failed to parse program ID: {e}")))?;
                match &program.data {
                    ProgramData::Bytecode(bytecode) => Ok((program_id, bytecode.to_string(), program.edition)),
                    ProgramData::SourcePath { source, .. } => {
                        // Get the path to the built bytecode.
                        let bytecode_path = if source.as_path() == source_directory.join("main.leo") {
                            build_directory.join("main.aleo")
                        } else {
                            imports_directory.join(format!("{}.aleo", program.name))
                        };
                        // Fetch the bytecode.
                        let bytecode = std::fs::read_to_string(&bytecode_path).map_err(|e| {
                            CliError::custom(format!("Failed to read bytecode at {}: {e}", bytecode_path.display()))
                        })?;
                        // Return the bytecode and the manifest.
                        Ok((program_id, bytecode, program.edition))
                    }
                }
            })
            .collect::<Result<Vec<_>>>()?
    } else {
        Vec::new()
    };

    // Parse the program strings into AVM programs.
    let mut programs = programs
        .into_iter()
        .map(|(_, bytecode, edition)| {
            // Parse the program.
            let program = snarkvm::prelude::Program::<A::Network>::from_str(&bytecode)
                .map_err(|e| CliError::custom(format!("Failed to parse program: {e}")))?;
            // Return the program and its name.
            Ok((program, edition))
        })
        .collect::<Result<Vec<_>>>()?;

    // Determine whether the program is local or remote.
    let is_local = programs.iter().any(|(program, _)| program.id() == &program_id);

    // If the program is local, then check that the function exists.
    if is_local {
        let program = &programs
            .iter()
            .find(|(program, _)| program.id() == &program_id)
            .expect("Program should exist since it is local")
            .0;
        if !program.contains_function(&function_id) {
            return Err(CliError::custom(format!(
                "Function `{function_name}` does not exist in program `{program_name}`."
            ))
            .into());
        }
    }

    let inputs =
        command.inputs.into_iter().map(|string| parse_input(&string, &private_key)).collect::<Result<Vec<_>>>()?;

    // Initialize an RNG.
    let rng = &mut rand::thread_rng();

    // Initialize a new VM.
    let vm = VM::from(ConsensusStore::<A::Network, ConsensusMemory<A::Network>>::open(StorageMode::Production)?)?;

    // If the program is not local, then download it and its dependencies for the network.
    // Note: The dependencies are downloaded in "post-order" (child before parent).
    if !is_local {
        // Get the endpoint, accounting for overrides.
        let endpoint = get_endpoint(&command.env_override.endpoint)?;
        println!("⬇️ Downloading {program_name} and its dependencies from {endpoint}...");
        // Load the programs from the network.
        programs = load_latest_programs_from_network(&context, program_id, network, &endpoint)?;
    };

    // Add the programs to the VM.
    println!("\n➕Adding programs to the VM in the following order:");
    let programs_and_editions = programs
        .into_iter()
        .map(|(program, edition)| {
            // Note: We default to edition 1 since snarkVM execute may produce spurious errors if the program does not have a constructor but uses edition 0.
            let edition = edition.unwrap_or(1);
            // Get the program ID.
            let id = program.id().to_string();
            // Print the program ID and edition.
            match id == "credits.aleo" {
                true => println!("  - {id} (already included)"),
                false => println!("  - {id} (edition: {edition})"),
            }
            (program, edition)
        })
        .collect::<Vec<_>>();
    vm.process().write().add_programs_with_editions(&programs_and_editions)?;

    // Evaluate the program and get a response.
    let authorization = vm.authorize(&private_key, program_id, function_id, inputs.iter(), rng)?;
    let response = vm.process().read().evaluate::<A>(authorization)?;

    // Print the response.
    match response.outputs().len() {
        0 => (),
        1 => println!("\n➡️  Output\n"),
        _ => println!("\n➡️  Outputs\n"),
    };
    for output in response.outputs() {
        println!(" • {output}");
    }

    Ok(())
}
