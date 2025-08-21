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

use leo_ast::NetworkName;
use leo_package::{Package, ProgramData};

use aleo_std::StorageMode;

#[cfg(not(feature = "only_testnet"))]
use snarkvm::circuit::{AleoCanaryV0, AleoV0};
use snarkvm::{
    circuit::{Aleo, AleoTestnetV0},
    prelude::{
        Identifier,
        ProgramID,
        ToBytes,
        VM,
        store::{ConsensusStore, helpers::memory::ConsensusMemory},
    },
};

use clap::Parser;
use serde::Serialize;
use sha2::Digest;
use std::{fmt::Write, path::PathBuf};

#[derive(Serialize)]
pub struct Metadata {
    pub prover_checksum: String,
    pub prover_size: usize,
    pub verifier_checksum: String,
    pub verifier_size: usize,
}

/// Synthesize proving and verifying keys for a given function.
#[derive(Parser, Debug)]
pub struct LeoSynthesize {
    #[clap(
        name = "NAME",
        help = "The name of the function to execute, e.g `helloworld.aleo/main` or `main`.",
        default_value = "main"
    )]
    pub(crate) name: String,
    #[clap(flatten)]
    pub(crate) action: TransactionAction,
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
    #[clap(flatten)]
    build_options: BuildOptions,
}

impl Command for LeoSynthesize {
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
        // Get the network, accounting for overrides.
        let network = context.get_network(&self.env_override.network)?.parse()?;
        // Get the endpoint, accounting for overrides.
        let endpoint = context.get_endpoint(&self.env_override.endpoint)?;
        // If the current directory is a valid Leo package, then build it.
        if Package::from_directory_no_graph(path, home_path, network, &endpoint).is_ok() {
            let package = LeoBuild {
                env_override: self.env_override.clone(),
                options: {
                    let mut options = self.build_options.clone();
                    options.no_cache = true;
                    options
                },
            }
            .execute(context)?;
            // Return the package.
            Ok(Some(package))
        } else {
            Ok(None)
        }
    }

    fn apply(self, context: Context, input: Self::Input) -> Result<Self::Output> {
        // Verify that the transaction action is not "broadcast" or "print"
        if self.action.broadcast {
            println!(
                "❌ `--broadcast` is not a valid option for `leo synthesize`. Please use `--save` and specify a valid directory."
            );
            return Ok(());
        } else if self.action.print {
            println!(
                "❌ `--print` is not a valid option for `leo synthesize`. Please use `--save` and specify a valid directory."
            );
            return Ok(());
        } else if self.action.save.is_none() {
            println!("⚠️ You are running `leo synthesize` without `--save`");
        }
        // Get the network, accounting for overrides.
        let network = context.get_network(&self.env_override.network)?.parse()?;
        // Handle each network with the appropriate parameterization.
        match network {
            NetworkName::TestnetV0 => handle_synthesize::<AleoTestnetV0>(self, context, network, input),
            NetworkName::MainnetV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Mainnet chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                handle_synthesize::<AleoV0>(self, context, network, input)
            }
            NetworkName::CanaryV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Canary chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                handle_synthesize::<AleoCanaryV0>(self, context, network, input)
            }
        }
    }
}

// A helper function to handle the `synthesize` command.
fn handle_synthesize<A: Aleo>(
    command: LeoSynthesize,
    context: Context,
    network: NetworkName,
    package: Option<Package>,
) -> Result<<LeoExecute as Command>::Output> {
    // Get the endpoint, accounting for overrides.
    let endpoint = context.get_endpoint(&command.env_override.endpoint)?;

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
                    "Running `leo synthesize {} ...`, without an explicit program name requires that your current working directory is a valid Leo project.",
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

    // Initialize an RNG.
    let rng = &mut rand::thread_rng();

    // Initialize a new VM.
    let vm = VM::from(ConsensusStore::<A::Network, ConsensusMemory<A::Network>>::open(StorageMode::Production)?)?;

    // If the program is not local, then download it and its dependencies for the network.
    // Note: The dependencies are downloaded in "post-order" (child before parent).
    if !is_local {
        println!("⬇️ Downloading {program_name} and its dependencies from {endpoint}...");
        programs = load_latest_programs_from_network(&context, program_id, network, &endpoint)?;
    };

    // Add the programs to the VM.
    println!("\n➕ Adding programs to the VM in the following order:");
    let programs_and_editions = programs
        .into_iter()
        .map(|(program, edition)| {
            // Note: We default to edition 1 since snarkVM execute may produce spurious errors if the program does not have a constructor but uses edition 0.
            let is_default = edition.is_some();
            let edition = edition.unwrap_or(1);
            // Get the program ID.
            let id = program.id().to_string();
            // Print the program ID and edition.
            match id == "credits.aleo" {
                true => println!("  - {id} (already included)"),
                false => match is_default {
                    true => println!(" - {id} (defaulting to edition {edition})"),
                    false => println!("  - {id} (edition: {edition})"),
                },
            }
            (program, edition)
        })
        .collect::<Vec<_>>();
    vm.process().write().add_programs_with_editions(&programs_and_editions)?;

    println!("🌱 Synthesizing keys for {program_name}/{function_name}");
    vm.process().read().synthesize_key::<A, _>(&program_id, &function_id, rng)?;
    let proving_key = vm.process().read().get_proving_key(program_id, function_id)?;
    let verifying_key = vm.process().read().get_verifying_key(program_id, function_id)?;

    // A helper function to hash the keys.
    let hash = |bytes: &[u8]| -> anyhow::Result<String> {
        let mut hasher = sha2::Sha256::new();
        hasher.update(bytes);
        let digest = hasher.finalize();
        let mut hex = String::new();
        for byte in digest {
            write!(&mut hex, "{byte:02x}")?;
        }
        Ok(hex)
    };

    // Get the checksums of the keys.
    let prover_bytes = proving_key.to_bytes_le()?;
    let verifier_bytes = verifying_key.to_bytes_le()?;
    let prover_checksum = hash(&prover_bytes)?;
    let verifier_checksum = hash(&verifier_bytes)?;

    // Construct the metadata.
    let metadata = Metadata {
        prover_checksum,
        prover_size: prover_bytes.len(),
        verifier_checksum,
        verifier_size: verifier_bytes.len(),
    };
    let metadata_pretty = serde_json::to_string_pretty(&metadata)
        .map_err(|e| CliError::custom(format!("Failed to serialize metadata: {e}")))?;

    // A helper to write to a file.
    let write_to_file = |type_: &str, path: PathBuf, data: &[u8]| -> Result<()> {
        println!("💾 Saving {type_} for {program_name}/{function_name} at {}", path.display());
        std::fs::write(path, data).map_err(|e| CliError::custom(format!("Failed to write to file: {e}")))?;
        Ok(())
    };

    // If the `save` option is set, save the proving and verifying keys to a file in the specified directory.
    // The file format is `program_name.function_name.type.timestamp`.
    // The directory is created if it doesn't exist.
    if let Some(path) = &command.action.save {
        // Create the directory if it doesn't exist.
        std::fs::create_dir_all(path).map_err(|e| CliError::custom(format!("Failed to create directory: {e}")))?;
        // Get the current timestamp.
        let timestamp = chrono::Utc::now().timestamp();
        // Get the file paths.
        let prover_file_path = PathBuf::from(path).join(format!("{program_name}.{function_name}.prover.{timestamp}"));
        let verifier_file_path =
            PathBuf::from(path).join(format!("{program_name}.{function_name}.verifier.{timestamp}"));
        let metadata_file_path =
            PathBuf::from(path).join(format!("{program_name}.{function_name}.metadata.{timestamp}"));
        // Save the keys.
        write_to_file("proving key", prover_file_path, &prover_bytes)?;
        write_to_file("verifying key", verifier_file_path, &verifier_bytes)?;
        write_to_file("metadata", metadata_file_path, metadata_pretty.as_bytes())?;
    }

    Ok(())
}
