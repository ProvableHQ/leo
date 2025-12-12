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
    algorithms::crypto_hash::sha256,
    circuit::{Aleo, AleoTestnetV0},
    prelude::{
        ProgramID,
        ToBytes,
        VM,
        store::{ConsensusStore, helpers::memory::ConsensusMemory},
    },
    synthesizer::program::StackTrait,
};

use clap::Parser;
use serde::Serialize;
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
    #[clap(name = "NAME", help = "The name of the program to synthesize, e.g `helloworld.aleo`")]
    pub(crate) program_name: String,
    #[arg(short, long, help = "Use the local Leo project.")]
    pub(crate) local: bool,
    #[arg(short, long, help = "Skip functions that contain any of the given substrings")]
    pub(crate) skip: Vec<String>,
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
        // If the `--local` option is enabled, then build the project.
        if self.local {
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
        }

        // Get the network, accounting for overrides.
        let network = get_network(&self.env_override.network)?;
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
) -> Result<<LeoSynthesize as Command>::Output> {
    // Get the endpoint, accounting for overrides.
    let endpoint = get_endpoint(&command.env_override.endpoint)?;

    // Parse the program name as a `ProgramID`.
    let program_id = ProgramID::<A::Network>::from_str(&command.program_name)
        .map_err(|e| CliError::custom(format!("Failed to parse program name: {e}")))?;

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

    // Initialize an RNG.
    let rng = &mut rand::thread_rng();

    // Initialize a new VM.
    let vm = VM::from(ConsensusStore::<A::Network, ConsensusMemory<A::Network>>::open(StorageMode::Production)?)?;

    // If the program is not local, then download it and its dependencies for the network.
    // Note: The dependencies are downloaded in "post-order" (child before parent).
    if !is_local {
        println!("⬇️ Downloading {program_id} and its dependencies from {endpoint}...");
        programs = load_latest_programs_from_network(&context, program_id, network, &endpoint)?;
    };

    // Add the programs to the VM.
    println!("\n➕ Adding programs to the VM in the following order:");
    let programs_and_editions = programs
        .into_iter()
        .map(|(program, edition)| {
            print_program_source(&program.id().to_string(), edition);
            let edition = edition.unwrap_or(LOCAL_PROGRAM_DEFAULT_EDITION);
            (program, edition)
        })
        .collect::<Vec<_>>();
    vm.process().write().add_programs_with_editions(&programs_and_editions)?;

    // Get the edition and function IDs from the program.
    let stack = vm.process().read().get_stack(program_id)?;
    let edition = *stack.program_edition();
    let function_ids = stack
        .program()
        .functions()
        .keys()
        .filter(|id| !command.skip.iter().any(|substring| id.to_string().contains(substring)))
        .collect::<Vec<_>>();

    // A helper function to hash the keys.
    let hash = |bytes: &[u8]| -> anyhow::Result<String> {
        let digest = sha256(bytes);
        let mut hex = String::new();
        for byte in digest {
            write!(&mut hex, "{byte:02x}")?;
        }
        Ok(hex)
    };

    println!("\n🌱 Synthesizing the following keys in {program_id}:");
    for id in &function_ids {
        println!("    - {id}");
    }

    for function_id in function_ids {
        stack.synthesize_key::<A, _>(function_id, rng)?;
        let proving_key = stack.get_proving_key(function_id)?;
        let verifying_key = stack.get_verifying_key(function_id)?;

        println!("\n🔑 Synthesized keys for {program_id}/{function_id} (edition {edition})");
        println!("ℹ️ Circuit Information:");
        println!("    - Public Inputs: {}", verifying_key.circuit_info.num_public_inputs);
        println!("    - Variables: {}", verifying_key.circuit_info.num_public_and_private_variables);
        println!("    - Constraints: {}", verifying_key.circuit_info.num_constraints);
        println!("    - Non-Zero Entries in A: {}", verifying_key.circuit_info.num_non_zero_a);
        println!("    - Non-Zero Entries in B: {}", verifying_key.circuit_info.num_non_zero_b);
        println!("    - Non-Zero Entries in C: {}", verifying_key.circuit_info.num_non_zero_c);
        println!("    - Circuit ID: {}", verifying_key.id);

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
        let write_to_file = |path: PathBuf, data: &[u8]| -> Result<()> {
            std::fs::write(path, data).map_err(|e| CliError::custom(format!("Failed to write to file: {e}")))?;
            Ok(())
        };

        // If the `save` option is set, save the proving and verifying keys to a file in the specified directory.
        // The file format is `program_id.function_id.edition_or_local.type.timestamp`.
        // The directory is created if it doesn't exist.
        if let Some(path) = &command.action.save {
            // Create the directory if it doesn't exist.
            std::fs::create_dir_all(path).map_err(|e| CliError::custom(format!("Failed to create directory: {e}")))?;
            // Get the current timestamp.
            let timestamp = chrono::Utc::now().timestamp();
            // The edition.
            let edition = if command.local { "local".to_string() } else { edition.to_string() };
            // The prefix for the file names.
            let prefix = format!("{network}.{program_id}.{function_id}.{edition}");
            // Get the file paths.
            let prover_file_path = PathBuf::from(path).join(format!("{prefix}.prover.{timestamp}"));
            let verifier_file_path = PathBuf::from(path).join(format!("{prefix}.verifier.{timestamp}"));
            let metadata_file_path = PathBuf::from(path)
                .join(format!("{network}.{program_id}.{function_id}.{edition}.metadata.{timestamp}"));
            // Print the save location.
            println!(
                "💾 Saving proving key, verifying key, and metadata to: {}/{network}.{program_id}.{function_id}.{edition}.prover|verifier|metadata.{timestamp}",
                metadata_file_path.parent().unwrap().display()
            );
            // Save the keys.
            write_to_file(prover_file_path, &prover_bytes)?;
            write_to_file(verifier_file_path, &verifier_bytes)?;
            write_to_file(metadata_file_path, metadata_pretty.as_bytes())?;
        }
    }

    Ok(())
}
