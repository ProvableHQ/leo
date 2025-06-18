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
use leo_package::{Package, fetch_program_from_network};

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

use crate::cli::commands::deploy::validate_deployment_limits;
use aleo_std::StorageMode;
use colored::*;
use snarkvm::prelude::{ConsensusVersion, ProgramID, Stack};
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
    #[clap(long, help = "Seconds to wait between consecutive deployments.", default_value = "15")]
    pub(crate) wait: u64,
    #[clap(long, help = "Skips deployment of any program that contains one of the given substrings.")]
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

    // Get the programs and optional manifests for all the programs.
    let programs_and_manifests = package
        .get_programs_and_manifests(context.home()?)?
        .into_iter()
        .map(|(program_name, program_string, _, manifest)| {
            // Parse the program ID from the program name.
            let program_id = ProgramID::<N>::from_str(&format!("{}.aleo", program_name))
                .map_err(|e| CliError::custom(format!("Failed to parse program ID: {e}")))?;
            // Parse the program bytecode.
            let bytecode = Program::<N>::from_str(&program_string)
                .map_err(|e| CliError::custom(format!("Failed to parse program: {e}")))?;
            Ok((program_id, bytecode, manifest))
        })
        .collect::<Result<Vec<_>>>()?;

    // Get all of the program IDs.
    let program_ids = programs_and_manifests.iter().map(|(program_id, _, _)| program_id).copied().collect::<Vec<_>>();

    // Parse the fee options.
    let fee_options = parse_fee_options(&private_key, &command.fee_options, programs_and_manifests.len())?;

    // Zip up the programs and manifests with the fee options.
    let tasks = programs_and_manifests
        .into_iter()
        .zip(fee_options)
        .map(|((program, data, manifest), (base_fee, priority_fee, record))| {
            (program, data, manifest, base_fee, priority_fee, record)
        })
        .collect::<Vec<_>>();

    // Split the tasks into local and remote dependencies.
    let (local, remote) = tasks.into_iter().partition::<Vec<_>, _>(|(_, _, manifest, _, _, _)| manifest.is_some());

    // Split the local tasks into those that should be skipped and those that should not.
    let (skipped, tasks): (Vec<_>, Vec<_>) = local
        .into_iter()
        .partition(|(program_id, _, _, _, _, _)| command.skip.iter().any(|skip| program_id.to_string().contains(skip)));

    // Get the consensus version.
    let consensus_version = get_consensus_version::<N>(&command.extra.consensus_version, &endpoint, network, &context)?;

    // Print a summary of the deployment plan.
    print_deployment_plan(
        &private_key,
        &address,
        &endpoint,
        &network,
        &tasks,
        &skipped,
        &remote,
        &check_tasks_for_warnings(&endpoint, network, &tasks, consensus_version, command),
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

    // Load all of the programs from the network into the VM.
    for program_id in program_ids {
        // Load the program from the network.
        let bytecode = fetch_program_from_network(&program_id.to_string(), &endpoint, network)
            .map_err(|e| CliError::custom(format!("Failed to fetch program from network: {e}")))?;
        // Parse the program bytecode.
        let program =
            Program::<N>::from_str(&bytecode).map_err(|e| CliError::custom(format!("Failed to parse program: {e}")))?;
        // Add the program to the VM.
        vm.process().write().add_program(&program)?;
    }

    // Specify the query
    let query = SnarkVMQuery::from(&endpoint);

    // For each of the programs, generate a deployment transaction.
    let mut transactions = Vec::new();
    for (program_id, program, manifest, _, priority_fee, fee_record) in tasks {
        // If the program is a local dependency, generate a deployment transaction.
        if manifest.is_some() {
            println!("📦 Creating deployment transaction for '{}'...\n", program_id.to_string().bold());
            // Generate the transaction.
            let transaction = vm
                .deploy(&private_key, &program, fee_record, priority_fee.unwrap_or(0), Some(query.clone()), rng)
                .map_err(|e| CliError::custom(format!("Failed to generate deployment transaction: {e}")))?;
            // Get the deployment.
            let deployment = transaction.deployment().expect("Expected a deployment in the transaction");
            // Print the deployment stats.
            print_deployment_stats(&vm, &program_id.to_string(), deployment, priority_fee)?;
            // Validate the deployment limits.
            validate_deployment_limits(deployment, &program_id, &network)?;
            // Save the transaction.
            transactions.push((program_id, transaction));
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

    // If the `broadcast` option is set, broadcast each deployment transaction to the network.
    if command.action.broadcast {
        for (program_id, transaction) in transactions.iter() {
            println!("📡 Broadcasting deployment for {program_id}...");
            // Get and confirm the fee with the user.
            let fee = transaction.fee_transition().expect("Expected a fee in the transaction");
            if !confirm_fee(&fee, &private_key, &address, &endpoint, network, &context, command.extra.yes)? {
                println!("❌ Deployment aborted.");
                return Ok(());
            }
            // Broadcast the transaction to the network.
            let response = handle_broadcast(
                &format!("{}/{}/transaction/broadcast", endpoint, network),
                transaction,
                &program_id.to_string(),
            )?;
            match response.status() {
                200 => println!(
                    "✅ Successfully broadcast deployment with:\n  - transaction ID: '{}'\n  - fee ID: '{}'",
                    transaction.id().to_string().bold().yellow(),
                    fee.id().to_string().bold().yellow()
                ),
                _ => {
                    let error_message = response
                        .into_string()
                        .map_err(|e| CliError::custom(format!("Failed to read response: {e}")))?;
                    println!("❌ Failed to broadcast deployment: {}", error_message);
                }
            }
            // Wait between successive deployments to prevent out of order deployments.
            println!("⏲️ Waiting for {} seconds to allow the deployment to confirm...\n", command.wait);
            std::thread::sleep(std::time::Duration::from_secs(command.wait));
        }
    }

    Ok(())
}

/// Check the tasks to warn the user about any potential issues.
/// The following properties are checked:
/// - If the transaction is to be broadcast:
///     - The program exists on the network and the new program is a valid upgrade.
///     - If the consensus version is less than V8, the program does not use V8 features.
///     - If the consensus version is V8 or greater, the program contains a constructor.
fn check_tasks_for_warnings<N: Network>(
    endpoint: &str,
    network: NetworkName,
    tasks: &[DeploymentTask<N>],
    consensus_version: ConsensusVersion,
    command: &LeoUpgrade,
) -> Vec<String> {
    let mut warnings = Vec::new();
    for (program_id, program, manifest, _, _, _) in tasks {
        if manifest.is_none() || !command.action.broadcast {
            continue;
        }

        // Check if the program exists on the network.
        if let Ok(remote_program) = fetch_program_from_network(&program_id.to_string(), endpoint, network) {
            // Parse the program.
            let remote_program = match Program::<N>::from_str(&remote_program) {
                Ok(program) => program,
                Err(e) => {
                    warnings.push(format!("Could not parse '{program_id}' from the network. Error: {e}",));
                    continue;
                }
            };
            // Check if the program is a valid upgrade.
            if let Err(e) = Stack::check_upgrade_is_valid(&remote_program, program) {
                warnings.push(format!(
                    "The program '{program_id}' is not a valid upgrade. The deployment will likely fail. Error: {e}",
                ));
            }
        } else {
            warnings.push(format!(
                "The program '{}' does not exist on the network. The upgrade will likely fail.",
                program_id
            ));
        }

        // Check if the program uses V8 features.
        if consensus_version < ConsensusVersion::V8 && program.contains_v8_syntax() {
            warnings.push(format!("The program '{}' uses V8 features but the consensus version is less than V8. The deployment will likely fail", program_id));
        }
        // Check if the program contains a constructor.
        if consensus_version >= ConsensusVersion::V8 && !program.contains_constructor() {
            warnings.push(format!(
                "The program '{}' does not contain a constructor. The deployment will likely fail",
                program_id
            ));
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
