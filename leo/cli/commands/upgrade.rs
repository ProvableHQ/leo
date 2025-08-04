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
use std::{collections::HashSet, fs};

use leo_ast::NetworkName;
use leo_package::{Package, ProgramData, fetch_program_from_network};

#[cfg(not(feature = "only_testnet"))]
use snarkvm::prelude::{CanaryV0, MainnetV0};
use snarkvm::{
    ledger::query::Query as SnarkVMQuery,
    prelude::{
        Program,
        TestnetV0,
        VM,
        store::{ConsensusStore, helpers::memory::ConsensusMemory},
    },
};

use crate::cli::{check_transaction::TransactionStatus, commands::deploy::validate_deployment_limits};
use aleo_std::StorageMode;
use colored::*;
use leo_span::Symbol;
use snarkvm::prelude::{ConsensusVersion, ProgramID, Stack, store::helpers::memory::BlockMemory};
use std::path::PathBuf;

/// Upgrades an Aleo program.
#[derive(Parser, Debug)]
pub struct LeoUpgrade {
    #[clap(flatten)]
    pub(crate) fee_options: FeeOptions,
    #[clap(flatten)]
    pub(crate) action: TransactionAction,
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
    #[clap(flatten)]
    pub(crate) extra: ExtraOptions,
    #[clap(long, help = "Skips the upgrade of any program that contains one of the given substrings.")]
    pub(crate) skip: Vec<String>,
    #[clap(flatten)]
    pub(crate) build_options: BuildOptions,
}

impl Command for LeoUpgrade {
    type Input = Package;
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        LeoBuild {
            env_override: self.env_override.clone(),
            options: {
                let mut options = self.build_options.clone();
                options.no_cache = true;
                options
            },
        }
        .execute(context)
    }

    fn apply(self, context: Context, input: Self::Input) -> Result<Self::Output> {
        // Get the network, accounting for overrides.
        let network = context.get_network(&self.env_override.network)?.parse()?;
        // Handle each network with the appropriate parameterization.
        match network {
            NetworkName::TestnetV0 => handle_upgrade::<TestnetV0>(&self, context, network, input),
            NetworkName::MainnetV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Mainnet chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                handle_upgrade::<MainnetV0>(&self, context, network, input)
            }
            NetworkName::CanaryV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Canary chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                handle_upgrade::<CanaryV0>(&self, context, network, input)
            }
        }
    }
}

// A helper function to handle upgrade logic.
fn handle_upgrade<N: Network>(
    command: &LeoUpgrade,
    context: Context,
    network: NetworkName,
    package: Package,
) -> Result<<LeoDeploy as Command>::Output> {
    // Get the private key and associated address, accounting for overrides.
    let private_key = context.get_private_key(&command.env_override.private_key)?;
    let address =
        Address::try_from(&private_key).map_err(|e| CliError::custom(format!("Failed to parse address: {e}")))?;

    // Get the endpoint, accounting for overrides.
    let endpoint = context.get_endpoint(&command.env_override.endpoint)?;

    // Get all the programs but tests.
    let programs = package.programs.iter().filter(|program| !program.is_test).cloned();

    let programs_and_bytecode: Vec<(leo_package::Program, String)> = programs
        .into_iter()
        .map(|program| {
            let bytecode = match &program.data {
                ProgramData::Bytecode(s) => s.clone(),
                ProgramData::SourcePath { .. } => {
                    // We need to read the bytecode from the filesystem.
                    let aleo_name = format!("{}.aleo", program.name);
                    let aleo_path = if package.manifest.program == aleo_name {
                        // The main program in the package, so its .aleo file
                        // will be in the build directory.
                        package.build_directory().join("main.aleo")
                    } else {
                        // Some other dependency, so look in `imports`.
                        package.imports_directory().join(aleo_name)
                    };
                    fs::read_to_string(aleo_path.clone())
                        .map_err(|e| CliError::custom(format!("Failed to read file {}: {e}", aleo_path.display())))?
                }
            };

            Ok((program, bytecode))
        })
        .collect::<Result<_>>()?;

    // Parse the fee options.
    let fee_options = parse_fee_options(&private_key, &command.fee_options, programs_and_bytecode.len())?;

    let tasks: Vec<Task<N>> = programs_and_bytecode
        .into_iter()
        .zip(fee_options)
        .map(|((program, bytecode), (_base_fee, priority_fee, record))| {
            let id_str = format!("{}.aleo", program.name);
            let id =
                id_str.parse().map_err(|e| CliError::custom(format!("Failed to parse program ID {id_str}: {e}")))?;
            let bytecode = bytecode.parse().map_err(|e| CliError::custom(format!("Failed to parse program: {e}")))?;
            Ok(Task {
                id,
                program: bytecode,
                edition: program.edition,
                is_local: program.is_local,
                priority_fee,
                record,
            })
        })
        .collect::<Result<_>>()?;

    // Get the program IDs.
    let program_ids = tasks.iter().map(|task| task.id).collect::<Vec<_>>();

    // Split the tasks into local and remote dependencies.
    let (local, remote) = tasks.into_iter().partition::<Vec<_>, _>(|task| task.is_local);

    // Get the skipped programs.
    let skipped: HashSet<ProgramID<N>> = local
        .iter()
        .filter_map(|task| {
            let id_string = task.id.to_string();
            command.skip.iter().any(|skip| id_string.contains(skip)).then_some(task.id)
        })
        .collect();

    // Get the consensus version.
    let consensus_version = get_consensus_version(&command.extra.consensus_version, &endpoint, network, &context)?;

    // Print a summary of the deployment plan.
    print_deployment_plan(
        &private_key,
        &address,
        &endpoint,
        &network,
        &local,
        &skipped,
        &remote,
        &check_tasks_for_warnings(&endpoint, network, &local, consensus_version, command),
        consensus_version,
        &command.into(),
    );

    // Prompt the user to confirm the plan.
    if !confirm("Do you want to proceed with upgrade?", command.extra.yes)? {
        println!("❌ Upgrade aborted.");
        return Ok(());
    }

    // Initialize an RNG.
    let rng = &mut rand::thread_rng();

    // Initialize a new VM.
    let vm = VM::from(ConsensusStore::<N, ConsensusMemory<N>>::open(StorageMode::Production)?)?;

    // Load all the programs from the network into the VM.
    let programs_and_editions = program_ids
        .iter()
        .map(|id| {
            // Load the program from the network.
            let program = leo_package::Program::fetch(
                Symbol::intern(&id.name().to_string()),
                None,
                context.home()?,
                network,
                &endpoint,
                true,
            )?;
            let ProgramData::Bytecode(bytecode) = program.data else {
                panic!("Expected bytecode when fetching a remote program");
            };
            // Parse the program bytecode.
            let bytecode = Program::<N>::from_str(&bytecode)
                .map_err(|e| CliError::custom(format!("Failed to parse program: {e}")))?;
            // Return the bytecode and edition.
            // Note: We default to edition 1 since snarkVM execute may produce spurious errors if the program does not have a constructor but uses edition 0.
            Ok((bytecode, program.edition.unwrap_or(1)))
        })
        .collect::<Result<Vec<_>>>()?;
    vm.process().write().add_programs_with_editions(&programs_and_editions)?;

    // Specify the query
    let query = SnarkVMQuery::<N, BlockMemory<N>>::from(&endpoint);

    // For each of the programs, generate a deployment transaction.
    let mut transactions = Vec::new();
    for Task { id, program, priority_fee, record, .. } in local {
        // If the program is a local dependency that is not skipped, generate a deployment transaction.
        if !skipped.contains(&id) {
            println!("📦 Creating deployment transaction for '{}'...\n", id.to_string().bold());
            // Generate the transaction.
            let transaction =
                vm.deploy(&private_key, &program, record, priority_fee.unwrap_or(0), Some(&query), rng)
                    .map_err(|e| CliError::custom(format!("Failed to generate deployment transaction: {e}")))?;
            // Get the deployment.
            let deployment = transaction.deployment().expect("Expected a deployment in the transaction");
            // Print the deployment stats.
            print_deployment_stats(&vm, &id.to_string(), deployment, priority_fee)?;
            // Validate the deployment limits.
            validate_deployment_limits(deployment, &id, &network)?;
            // Save the transaction.
            transactions.push((id, transaction));
        }
        // Add the program to the VM.
        vm.process().write().add_program(&program)?;
    }

    // If the `print` option is set, print the deployment transaction to the console.
    // The transaction is printed in JSON format.
    if command.action.print {
        for (program_name, transaction) in transactions.iter() {
            // Pretty-print the transaction.
            let transaction_json = serde_json::to_string_pretty(transaction)
                .map_err(|e| CliError::custom(format!("Failed to serialize transaction: {e}")))?;
            println!("🖨️ Printing deployment for {program_name}\n{transaction_json}")
        }
    }

    // If the `save` option is set, save each deployment transaction to a file in the specified directory.
    // The file format is `program_name.deployment.json`.
    // The directory is created if it doesn't exist.
    if let Some(path) = &command.action.save {
        // Create the directory if it doesn't exist.
        std::fs::create_dir_all(path).map_err(|e| CliError::custom(format!("Failed to create directory: {e}")))?;
        for (program_name, transaction) in transactions.iter() {
            // Save the transaction to a file.
            let file_path = PathBuf::from(path).join(format!("{program_name}.deployment.json"));
            println!("💾 Saving deployment for {program_name} at {}", file_path.display());
            let transaction_json = serde_json::to_string_pretty(transaction)
                .map_err(|e| CliError::custom(format!("Failed to serialize transaction: {e}")))?;
            std::fs::write(file_path, transaction_json)
                .map_err(|e| CliError::custom(format!("Failed to write transaction to file: {e}")))?;
        }
    }

    // If the `broadcast` option is set, broadcast each upgrade transaction to the network.
    if command.action.broadcast {
        for (i, (program_id, transaction)) in transactions.iter().enumerate() {
            println!("📡 Broadcasting upgrade for {program_id}...");
            // Get and confirm the fee with the user.
            let fee = transaction.fee_transition().expect("Expected a fee in the transaction");
            if !confirm_fee(&fee, &private_key, &address, &endpoint, network, &context, command.extra.yes)? {
                println!("⏩ Upgrade skipped.");
                continue;
            }
            let fee_id = fee.id().to_string();
            let id = transaction.id().to_string();
            let height_before = check_transaction::current_height(&endpoint, network)?;
            // Broadcast the transaction to the network.
            let (message, status) = handle_broadcast(
                &format!("{endpoint}/{network}/transaction/broadcast"),
                transaction,
                &program_id.to_string(),
            )?;

            let fail_and_prompt = |msg| {
                println!("❌ Failed to upgrade program {program_id}: {msg}.");
                let count = transactions.len() - i - 1;
                // Check if the user wants to continue with the next upgrade.
                if count > 0 {
                    confirm("Do you want to continue with the next upgrade?", command.extra.yes)
                } else {
                    Ok(false)
                }
            };

            match status {
                200..=299 => {
                    let status = check_transaction::check_transaction_with_message(
                        &id,
                        Some(&fee_id),
                        &endpoint,
                        network,
                        height_before + 1,
                        command.extra.max_wait,
                        command.extra.blocks_to_check,
                    )?;
                    if status == Some(TransactionStatus::Accepted) {
                        println!("✅ Upgrade confirmed!");
                    } else if fail_and_prompt("could not find the transaction on the network")? {
                        continue;
                    } else {
                        return Ok(());
                    }
                }
                _ => {
                    if fail_and_prompt(&message)? {
                        continue;
                    } else {
                        return Ok(());
                    }
                }
            }
        }
    }

    Ok(())
}

/// Check the tasks to warn the user about any potential issues.
/// The following properties are checked:
/// - If the transaction is to be broadcast:
///     - The program exists on the network and the new program is a valid upgrade.
///     - If the consensus version is less than V9, the program does not use V9 features.
///     - If the consensus version is V9 or greater, the program contains a constructor.
fn check_tasks_for_warnings<N: Network>(
    endpoint: &str,
    network: NetworkName,
    tasks: &[Task<N>],
    consensus_version: ConsensusVersion,
    command: &LeoUpgrade,
) -> Vec<String> {
    let mut warnings = Vec::new();
    for Task { id, program, is_local, .. } in tasks {
        if !is_local || !command.action.broadcast {
            continue;
        }

        // Check if the program exists on the network.
        if let Ok(remote_program) = fetch_program_from_network(&id.to_string(), endpoint, network) {
            // Parse the program.
            let remote_program = match Program::<N>::from_str(&remote_program) {
                Ok(program) => program,
                Err(e) => {
                    warnings.push(format!("Could not parse '{id}' from the network. Error: {e}",));
                    continue;
                }
            };
            // Check if the program is a valid upgrade.
            if let Err(e) = Stack::check_upgrade_is_valid(&remote_program, program) {
                warnings.push(format!(
                    "The program '{id}' is not a valid upgrade. The upgrade will likely fail. Error: {e}",
                ));
            }
        } else {
            warnings.push(format!("The program '{id}' does not exist on the network. The upgrade will likely fail.",));
        }

        // Check if the program uses V9 features.
        if consensus_version < ConsensusVersion::V9 && program.contains_v9_syntax() {
            warnings.push(format!("The program '{id}' uses V9 features but the consensus version is less than V9. The upgrade will likely fail"));
        }
        // Check if the program contains a constructor.
        if consensus_version >= ConsensusVersion::V9 && !program.contains_constructor() {
            warnings.push(format!("The program '{id}' does not contain a constructor. The upgrade will likely fail",));
        }
    }
    warnings
}

// Convert the `LeoUpgrade` into a `LeoDeploy` command.
impl From<&LeoUpgrade> for LeoDeploy {
    fn from(upgrade: &LeoUpgrade) -> Self {
        Self {
            fee_options: upgrade.fee_options.clone(),
            action: upgrade.action.clone(),
            env_override: upgrade.env_override.clone(),
            extra: upgrade.extra.clone(),
            skip: upgrade.skip.clone(),
            build_options: upgrade.build_options.clone(),
        }
    }
}
