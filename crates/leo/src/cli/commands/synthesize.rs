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
use std::{fmt::Write, path::PathBuf};

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
    type Output = SynthesizeOutput;

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
            return Ok(SynthesizeOutput::default());
        } else if self.action.print {
            println!(
                "❌ `--print` is not a valid option for `leo synthesize`. Please use `--save` and specify a valid directory."
            );
            return Ok(SynthesizeOutput::default());
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
            .compilation_units
            .iter()
            .clone()
            .filter(|unit| !unit.kind.is_library())
            .map(|unit| {
                let program_id = ProgramID::<A::Network>::from_str(&format!("{}", unit.name))
                    .map_err(|e| CliError::custom(format!("Failed to parse program ID: {e}")))?;
                match &unit.data {
                    ProgramData::Bytecode(bytecode) => Ok((program_id, bytecode.to_string(), unit.edition)),
                    ProgramData::SourcePath { source, .. } => {
                        // Get the path to the built bytecode.
                        let bytecode_path = if source.as_path() == source_directory.join("main.leo") {
                            build_directory.join("main.aleo")
                        } else {
                            imports_directory.join(format!("{}", unit.name))
                        };
                        // Fetch the bytecode.
                        let bytecode = std::fs::read_to_string(&bytecode_path).map_err(|e| {
                            CliError::custom(format!("Failed to read bytecode at {}: {e}", bytecode_path.display()))
                        })?;
                        // Return the bytecode and the manifest.
                        Ok((program_id, bytecode, unit.edition))
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
        programs = load_latest_programs_from_network(
            &context,
            program_id,
            network,
            &endpoint,
            command.env_override.network_retries,
        )?;
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

    // Collect record names for translation key synthesis.
    let record_names: Vec<_> = stack.program().records().keys().cloned().collect();

    println!("\n🌱 Synthesizing the following keys in {program_id}:");
    for id in &function_ids {
        println!("    - {id} (function)");
    }
    for name in &record_names {
        println!("    - {name} (record translation)");
    }

    let mut synthesized_functions = Vec::new();

    // Helper: process a synthesized key (function or translation), print circuit info,
    // record metadata, and optionally save keys to disk.
    let mut process_key = |name: &str, label: &str| -> Result<()> {
        let name_id = snarkvm::prelude::Identifier::<A::Network>::from_str(name)?;
        let proving_key = stack.get_proving_key(&name_id)?;
        let verifying_key = stack.get_verifying_key(&name_id)?;

        println!("\n🔑 Synthesized {label} for {program_id}/{name} (edition {edition})");
        println!("ℹ️ Circuit Information:");
        println!("    - Public Inputs: {}", verifying_key.circuit_info.num_public_inputs);
        println!("    - Variables: {}", verifying_key.circuit_info.num_public_and_private_variables);
        println!("    - Constraints: {}", verifying_key.circuit_info.num_constraints);
        println!("    - Non-Zero Entries in A: {}", verifying_key.circuit_info.num_non_zero_a);
        println!("    - Non-Zero Entries in B: {}", verifying_key.circuit_info.num_non_zero_b);
        println!("    - Non-Zero Entries in C: {}", verifying_key.circuit_info.num_non_zero_c);
        println!("    - Circuit ID: {}", verifying_key.id);

        let prover_bytes = proving_key.to_bytes_le()?;
        let verifier_bytes = verifying_key.to_bytes_le()?;
        let prover_checksum = hash(&prover_bytes)?;
        let verifier_checksum = hash(&verifier_bytes)?;

        let metadata = Metadata {
            prover_checksum,
            prover_size: prover_bytes.len(),
            verifier_checksum,
            verifier_size: verifier_bytes.len(),
        };
        let metadata_pretty = serde_json::to_string_pretty(&metadata)
            .map_err(|e| CliError::custom(format!("Failed to serialize metadata: {e}")))?;

        let circuit_info = CircuitInfo {
            num_public_inputs: verifying_key.circuit_info.num_public_inputs as u64,
            num_variables: verifying_key.circuit_info.num_public_and_private_variables as u64,
            num_constraints: verifying_key.circuit_info.num_constraints as u64,
            num_non_zero_a: verifying_key.circuit_info.num_non_zero_a as u64,
            num_non_zero_b: verifying_key.circuit_info.num_non_zero_b as u64,
            num_non_zero_c: verifying_key.circuit_info.num_non_zero_c as u64,
            circuit_id: verifying_key.id.to_string(),
        };

        synthesized_functions.push(SynthesizedFunction {
            name: name.to_string(),
            circuit_info,
            metadata: metadata.clone(),
        });

        if let Some(path) = &command.action.save {
            std::fs::create_dir_all(path).map_err(|e| CliError::custom(format!("Failed to create directory: {e}")))?;
            let timestamp = chrono::Utc::now().timestamp();
            let edition_str = if command.local { "local".to_string() } else { edition.to_string() };
            let prefix = format!("{network}.{program_id}.{name}.{edition_str}");
            let prover_file_path = PathBuf::from(path).join(format!("{prefix}.prover.{timestamp}"));
            let verifier_file_path = PathBuf::from(path).join(format!("{prefix}.verifier.{timestamp}"));
            let metadata_file_path = PathBuf::from(path).join(format!("{prefix}.metadata.{timestamp}"));
            println!(
                "💾 Saving {label} to: {}/{prefix}.prover|verifier|metadata.{timestamp}",
                metadata_file_path.parent().unwrap().display()
            );
            std::fs::write(&prover_file_path, &prover_bytes)
                .map_err(|e| CliError::custom(format!("Failed to write to file: {e}")))?;
            std::fs::write(&verifier_file_path, &verifier_bytes)
                .map_err(|e| CliError::custom(format!("Failed to write to file: {e}")))?;
            std::fs::write(&metadata_file_path, metadata_pretty.as_bytes())
                .map_err(|e| CliError::custom(format!("Failed to write to file: {e}")))?;
        }

        Ok(())
    };

    // Synthesize function keys.
    for function_id in function_ids {
        stack.synthesize_key::<A, _>(function_id, rng)?;
        process_key(&function_id.to_string(), "keys")?;
    }

    // Synthesize record translation keys.
    for record_name in &record_names {
        stack.synthesize_translation_key::<A, _>(record_name, rng)?;
        process_key(&record_name.to_string(), "translation key")?;
    }

    Ok(SynthesizeOutput { program: program_id.to_string(), functions: synthesized_functions })
}
