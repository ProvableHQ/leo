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

use check_transaction::TransactionStatus;
use leo_package::{Manifest, NetworkName, Package, fetch_program_from_network};

#[cfg(not(feature = "only_testnet"))]
use snarkvm::prelude::{CanaryV0, MainnetV0};
use snarkvm::{
    ledger::store::helpers::memory::BlockMemory,
    prelude::{
        Deployment,
        Program,
        TestnetV0,
        VM,
        deployment_cost,
        query::Query as SnarkVMQuery,
        store::{ConsensusStore, helpers::memory::ConsensusMemory},
    },
};

use aleo_std::StorageMode;
use colored::*;
use snarkvm::prelude::{ConsensusVersion, ProgramID};
use std::path::PathBuf;

type DeploymentTask<N> =
    (ProgramID<N>, Program<N>, Option<Manifest>, Option<u64>, Option<u64>, Option<Record<N, Plaintext<N>>>);

/// Deploys an Aleo program.
#[derive(Parser, Debug)]
pub struct LeoDeploy {
    #[clap(
        long,
        help = "Deploy the programs twice (this is temporarily necessary on local devnets with consensus version 8)",
        default_value = "false"
    )]
    twice: bool,
    #[clap(flatten)]
    pub(crate) fee_options: FeeOptions,
    #[clap(flatten)]
    pub(crate) action: TransactionAction,
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
    #[clap(flatten)]
    pub(crate) extra: ExtraOptions,
    #[clap(long, help = "Skips deployment of any program that contains one of the given substrings.")]
    pub(crate) skip: Vec<String>,
    #[clap(flatten)]
    pub(crate) build_options: BuildOptions,
}

impl Command for LeoDeploy {
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
            NetworkName::TestnetV0 => handle_deploy::<TestnetV0>(&self, context, network, input),
            NetworkName::MainnetV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Mainnet chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                handle_deploy::<MainnetV0>(&self, context, network, input)
            }
            NetworkName::CanaryV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Canary chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                handle_deploy::<CanaryV0>(&self, context, network, input)
            }
        }
    }
}

// A helper function to handle deployment logic.
fn handle_deploy<N: Network>(
    command: &LeoDeploy,
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
        .map(|(program_name, program_string, manifest)| {
            // Parse the program ID from the program name.
            let program_id = ProgramID::<N>::from_str(&format!("{}.aleo", program_name))
                .map_err(|e| CliError::custom(format!("Failed to parse program ID: {e}")))?;
            // Parse the program bytecode.
            let bytecode = Program::<N>::from_str(&program_string)
                .map_err(|e| CliError::custom(format!("Failed to parse program: {e}")))?;
            Ok((program_id, bytecode, manifest))
        })
        .collect::<Result<Vec<_>>>()?;

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

    // Get the skipped programs.
    let skipped = local
        .iter()
        .filter(|(program_id, _, _, _, _, _)| command.skip.iter().any(|skip| program_id.to_string().contains(skip)))
        .map(|(program_id, _, _, _, _, _)| *program_id)
        .collect::<Vec<_>>();

    // Get the consensus version.
    let consensus_version = get_consensus_version::<N>(&command.extra.consensus_version, &endpoint, network, &context)?;

    // Print a summary of the deployment plan.
    print_deployment_plan(
        &private_key,
        &address,
        &endpoint,
        &network,
        &local,
        &skipped,
        &remote,
        &check_tasks_for_warnings(&endpoint, network, &local, &command.action),
        consensus_version,
        command,
    );

    // Prompt the user to confirm the plan.
    if !confirm("Do you want to proceed with deployment?", command.extra.yes)? {
        println!("❌ Deployment aborted.");
        return Ok(());
    }

    // Initialize an RNG.
    let rng = &mut rand::thread_rng();

    // Initialize a new VM.
    let vm = VM::from(ConsensusStore::<N, ConsensusMemory<N>>::open(StorageMode::Production)?)?;

    // Load the remote dependencies into the VM.
    for (_, program, _, _, _, _) in remote {
        // If the program is a remote dependency, add it to the VM.
        vm.process().write().add_program(&program)?;
    }

    // Specify the query
    let query = SnarkVMQuery::<_, BlockMemory<_>>::from(&endpoint);

    // For each of the programs, generate a deployment transaction.
    let mut transactions = Vec::new();
    for (program_id, program, manifest, _, priority_fee, fee_record) in local {
        // If the program is a local dependency that is not skipped, generate a deployment transaction.
        let mut deploy = || -> Result<()> {
            if manifest.is_some() && !skipped.contains(&program_id) {
                println!("📦 Creating deployment transaction for '{}'...\n", program_id.to_string().bold());
                // Generate the transaction.
                let transaction = vm
                    .deploy(&private_key, &program, fee_record.clone(), priority_fee.unwrap_or(0), Some(&query), rng)
                    .map_err(|e| CliError::custom(format!("Failed to generate deployment transaction: {e}")))?;
                // Get the deployment.
                let deployment = transaction.deployment().expect("Expected a deployment in the transaction");
                // Print the deployment stats.
                print_deployment_stats(&program_id.to_string(), deployment, priority_fee)?;
                // Save the transaction.
                transactions.push((program_id, transaction));
            }
            Ok(())
        };
        deploy()?;
        // Add the program to the VM.
        vm.process().write().add_program(&program)?;
        if command.twice {
            deploy()?;
        }
    }

    for (program_id, transaction) in transactions.iter() {
        // Validate the deployment limits.
        let deployment = transaction.deployment().expect("Expected a deployment in the transaction");
        validate_deployment_limits(deployment, program_id, &network)?;
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
        for (i, (program_id, transaction)) in transactions.iter().enumerate() {
            println!("\n📡 Broadcasting deployment for {}...", program_id.to_string().bold());
            // Get and confirm the fee with the user.
            let fee = transaction.fee_transition().expect("Expected a fee in the transaction");
            if !confirm_fee(&fee, &private_key, &address, &endpoint, network, &context, command.extra.yes)? {
                println!("⏩ Deployment skipped.");
                continue;
            }
            let fee_id = fee.id().to_string();
            let id = transaction.id().to_string();
            let height_before = check_transaction::current_height(&endpoint, network)?;
            // Broadcast the transaction to the network.
            let (message, status) = handle_broadcast(
                &format!("{}/{}/transaction/broadcast", endpoint, network),
                transaction,
                &program_id.to_string(),
            )?;

            let fail_and_prompt = |msg| {
                println!("❌ Failed to deploy program {program_id}: {msg}.");
                let count = transactions.len() - i - 1;
                // Check if the user wants to continue with the next deployment.
                if count > 0 {
                    confirm("Do you want to continue with the next deployment?", command.extra.yes)
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
                        println!("✅ Deployment confirmed!");
                    } else if fail_and_prompt("Transaction apparently not accepted")? {
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
/// Only local programs are checked.
/// The following properties are checked:
/// - If the transaction is to be broadcast:
///     - The program does not exist on the network.
/// - The program's external dependencies are the latest version.
fn check_tasks_for_warnings<N: Network>(
    endpoint: &str,
    network: NetworkName,
    tasks: &[DeploymentTask<N>],
    action: &TransactionAction,
) -> Vec<String> {
    let mut warnings = Vec::new();
    for (program_id, _, manifest, _, _, _) in tasks {
        if manifest.is_none() || !action.broadcast {
            continue;
        }
        // Check if the program exists on the network.
        if fetch_program_from_network(&program_id.to_string(), endpoint, network).is_ok() {
            warnings.push(format!(
                "The program '{}' already exists on the network. The deployment will likely fail.",
                program_id
            ));
        }
    }
    warnings
}

/// Check if the number of variables and constraints are within the limits.
fn validate_deployment_limits<N: Network>(
    deployment: &Deployment<N>,
    program_id: &ProgramID<N>,
    network: &NetworkName,
) -> Result<()> {
    // Check if the number of variables is within the limits.
    let combined_variables = deployment.num_combined_variables()?;
    if combined_variables > N::MAX_DEPLOYMENT_VARIABLES {
        return Err(CliError::variable_limit_exceeded(
            program_id,
            combined_variables,
            N::MAX_DEPLOYMENT_VARIABLES,
            network,
        )
        .into());
    }

    // Check if the number of constraints is within the limits.
    let constraints = deployment.num_combined_constraints()?;
    if constraints > N::MAX_DEPLOYMENT_CONSTRAINTS {
        return Err(CliError::constraint_limit_exceeded(
            program_id,
            constraints,
            N::MAX_DEPLOYMENT_CONSTRAINTS,
            network,
        )
        .into());
    }

    Ok(())
}

/// Pretty‑print the deployment plan without using a table.
#[allow(clippy::too_many_arguments)]
fn print_deployment_plan<N: Network>(
    private_key: &PrivateKey<N>,
    address: &Address<N>,
    endpoint: &str,
    network: &NetworkName,
    local: &[DeploymentTask<N>],
    skipped: &[ProgramID<N>],
    remote: &[DeploymentTask<N>],
    warnings: &[String],
    consensus_version: ConsensusVersion,
    command: &LeoDeploy,
) {
    use colored::*;

    println!("\n{}", "🛠️  Deployment Plan Summary".bold());
    println!("{}", "──────────────────────────────────────────────".dimmed());

    // ── Configuration ────────────────────────────────────────────────────
    println!("{}", "🔧 Configuration:".bold());
    println!("  {:20}{}", "Private Key:".cyan(), format!("{}...", &private_key.to_string()[..24]).yellow());
    println!("  {:20}{}", "Address:".cyan(), format!("{}...", &address.to_string()[..24]).yellow());
    println!("  {:20}{}", "Endpoint:".cyan(), endpoint.yellow());
    println!("  {:20}{}", "Network:".cyan(), network.to_string().yellow());
    println!("  {:20}{}", "Consensus Version:".cyan(), (consensus_version as u8).to_string().yellow());

    // ── Deployment tasks (bullet list) ───────────────────────────────────
    println!("\n{}", "📦 Deployment Tasks:".bold());
    if local.is_empty() {
        println!("  (none)");
    } else {
        for (name, _, _, _, priority_fee, record) in local.iter().filter(|(p, ..)| !skipped.contains(p)) {
            let priority_fee_str = priority_fee.map_or("0".into(), |v| v.to_string());
            let record_str = if record.is_some() { "yes" } else { "no (public fee)" };
            println!(
                "  • {}  │ priority fee: {}  │ fee record: {}",
                name.to_string().cyan(),
                priority_fee_str,
                record_str
            );
        }
    }

    // ── Skipped programs ─────────────────────────────────────────────────
    if !skipped.is_empty() {
        println!("\n{}", "🚫 Skipped Programs:".bold().red());
        for symbol in skipped {
            println!("  • {}", symbol.to_string().dimmed());
        }
    }

    // ── Remote dependencies ──────────────────────────────────────────────
    if !remote.is_empty() {
        println!("\n{}", "🌐 Remote Dependencies:".bold().red());
        println!("{}", "(Leo will not generate transactions for these programs)".bold().red());
        for (symbol, _, _, _, _, _) in remote {
            println!("  • {}", symbol.to_string().dimmed());
        }
    }

    // ── Actions ──────────────────────────────────────────────────────────
    println!("\n{}", "⚙️ Actions:".bold());
    if command.action.print {
        println!("  • Transaction(s) will be printed to the console.");
    } else {
        println!("  • Transaction(s) will NOT be printed to the console.");
    }
    if let Some(path) = &command.action.save {
        println!("  • Transaction(s) will be saved to {}", path.bold());
    } else {
        println!("  • Transaction(s) will NOT be saved to a file.");
    }
    if command.action.broadcast {
        println!("  • Transaction(s) will be broadcast to {}", endpoint.bold());
    } else {
        println!("  • Transaction(s) will NOT be broadcast to the network.");
    }

    // ── Warnings ─────────────────────────────────────────────────────────
    if !warnings.is_empty() {
        println!("\n{}", "⚠️ Warnings:".bold().red());
        for warning in warnings {
            println!("  • {}", warning.dimmed());
        }
    }

    println!("{}", "──────────────────────────────────────────────\n".dimmed());
}

/// Pretty‑print deployment statistics without a table, using the same UI
/// conventions as `print_deployment_plan`.
fn print_deployment_stats<N: Network>(
    program_id: &str,
    deployment: &Deployment<N>,
    priority_fee: Option<u64>,
) -> Result<()> {
    use colored::*;
    use num_format::{Locale, ToFormattedString};

    // ── Collect statistics ────────────────────────────────────────────────
    let variables = deployment.num_combined_variables()?;
    let constraints = deployment.num_combined_constraints()?;
    let (base_fee, (storage_cost, synthesis_cost, namespace_cost)) = deployment_cost(deployment)?;

    let base_fee_cr = base_fee as f64 / 1_000_000.0;
    let prio_fee_cr = priority_fee.unwrap_or(0) as f64 / 1_000_000.0;
    let total_fee_cr = base_fee_cr + prio_fee_cr;

    // ── Header ────────────────────────────────────────────────────────────
    println!("\n{} {}", "📊 Deployment Summary for".bold(), program_id.bold());
    println!("{}", "──────────────────────────────────────────────".dimmed());

    // ── High‑level metrics ────────────────────────────────────────────────
    println!("  {:22}{}", "Total Variables:".cyan(), variables.to_formatted_string(&Locale::en).yellow());
    println!("  {:22}{}", "Total Constraints:".cyan(), constraints.to_formatted_string(&Locale::en).yellow());
    println!(
        "  {:22}{}",
        "Max Variables:".cyan(),
        N::MAX_DEPLOYMENT_VARIABLES.to_formatted_string(&Locale::en).green()
    );
    println!(
        "  {:22}{}",
        "Max Constraints:".cyan(),
        N::MAX_DEPLOYMENT_CONSTRAINTS.to_formatted_string(&Locale::en).green()
    );

    // ── Cost breakdown ────────────────────────────────────────────────────
    println!("\n{}", "💰 Cost Breakdown (credits)".bold());
    println!(
        "  {:22}{}{:.6}",
        "Transaction Storage:".cyan(),
        "".yellow(), // spacer for alignment
        storage_cost as f64 / 1_000_000.0
    );
    println!("  {:22}{}{:.6}", "Program Synthesis:".cyan(), "".yellow(), synthesis_cost as f64 / 1_000_000.0);
    println!("  {:22}{}{:.6}", "Namespace:".cyan(), "".yellow(), namespace_cost as f64 / 1_000_000.0);
    println!("  {:22}{}{:.6}", "Priority Fee:".cyan(), "".yellow(), prio_fee_cr);
    println!("  {:22}{}{:.6}", "Total Fee:".cyan(), "".yellow(), total_fee_cr);

    // ── Footer rule ───────────────────────────────────────────────────────
    println!("{}", "──────────────────────────────────────────────".dimmed());
    Ok(())
}
