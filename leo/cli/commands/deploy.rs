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

use leo_package::{Manifest, NetworkName, Package, fetch_program_from_network};

#[cfg(not(feature = "only_testnet"))]
use snarkvm::prelude::{CanaryV0, MainnetV0};
use snarkvm::{
    ledger::query::Query as SnarkVMQuery,
    prelude::{
        Deployment,
        Program,
        TestnetV0,
        VM,
        deployment_cost,
        store::{ConsensusStore, helpers::memory::ConsensusMemory},
    },
};

use aleo_std::StorageMode;
use colored::*;
use snarkvm::prelude::{ConsensusVersion, ProgramID};
use std::path::PathBuf;
use text_tables;

type DeploymentTask<N> =
    (ProgramID<N>, Program<N>, Option<Manifest>, Option<u64>, Option<u64>, Option<Record<N, Plaintext<N>>>);

/// Deploys an Aleo program.
#[derive(Parser, Debug)]
pub struct LeoDeploy {
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

impl Command for LeoDeploy {
    type Input = Package;
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        LeoCheck {
            env_override: self.env_override.clone(),
            extra: self.extra.clone(),
            build_options: self.build_options.clone(),
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
                return handle_deploy::<MainnetV0>(&self, context, network, input);
            }
            NetworkName::CanaryV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Canary chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                return handle_deploy::<CanaryV0>(&self, context, network, input);
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
        &command.action,
        consensus_version,
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
            // Check if the number of variables and constraints are within the limits.
            if deployment.num_combined_variables()? > N::MAX_DEPLOYMENT_VARIABLES {
                return Err(CliError::variable_limit_exceeded(program_id, N::MAX_DEPLOYMENT_VARIABLES, network).into());
            }
            if deployment.num_combined_constraints()? > N::MAX_DEPLOYMENT_CONSTRAINTS {
                return Err(
                    CliError::constraint_limit_exceeded(program_id, N::MAX_DEPLOYMENT_CONSTRAINTS, network).into()
                );
            }
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

/// Pretty-print the deployment plan in a readable format.
#[allow(clippy::too_many_arguments)]
fn print_deployment_plan<N: Network>(
    private_key: &PrivateKey<N>,
    address: &Address<N>,
    endpoint: &str,
    network: &NetworkName,
    tasks: &[DeploymentTask<N>],
    skipped: &[DeploymentTask<N>],
    remote: &[DeploymentTask<N>],
    action: &TransactionAction,
    consensus_version: ConsensusVersion,
) {
    use text_tables::render;

    println!("\n{}", "🛠️  Deployment Plan Summary".bold());
    println!("{}", "──────────────────────────────────────────────".dimmed());

    // Config
    println!("{}", "🔧 Configuration:".bold());
    println!("  {:16}{}", "Private Key:".cyan(), format!("{}...", &private_key.to_string()[..24]).yellow());
    println!("  {:16}{}", "Address:".cyan(), format!("{}...", &address.to_string()[..24]).yellow());
    println!("  {:16}{}", "Endpoint:".cyan(), endpoint.yellow());
    println!("  {:16}{}", "Network:".cyan(), network.to_string().yellow());
    println!("  {:16}{}", "Consensus Version:".cyan(), (consensus_version as u8).to_string().yellow());

    // Tasks
    println!("\n{}", "📦 Deployment Tasks:".bold());

    let mut table =
        vec![["Program".to_string(), "Base Fee".to_string(), "Priority Fee".to_string(), "Fee Record".to_string()]];

    for (name, _, _, _, priority_fee, record) in tasks.iter() {
        let name = name.to_string();
        // Base fees are not used at the moment, so we can ignore them.
        let base_fee = "auto".to_string();
        let priority_fee = priority_fee.map_or("0".into(), |v| v.to_string());
        let record = match record.is_some() {
            true => "yes".to_string(),
            false => "no (public fee)".to_string(),
        };

        table.push([name, base_fee, priority_fee, record]);
    }

    let mut buf = Vec::new();
    render(&mut buf, table).expect("table render failed");
    println!("{}", std::str::from_utf8(&buf).expect("utf8 fail"));

    // Skipped programs
    if !skipped.is_empty() {
        println!("{}", "🚫 Skipped Programs:".bold().red());
        for (symbol, _, _, _, _, _) in skipped {
            println!("  - {}", symbol.to_string().dimmed());
        }
    }

    // Remote dependencies
    if !remote.is_empty() {
        println!("{}", "🌐 Remote Dependencies:".bold().red());
        println!("{}", "(Leo will not generate transactions for these programs):".bold().red());
        for (symbol, _, _, _, _, _) in remote {
            println!("  - {}", symbol.to_string().dimmed());
        }
    }

    // Actions
    println!("{}", "⚙️ Actions:".bold());
    if action.print {
        println!("  - Your transaction(s) will be printed to the console.");
    } else {
        println!("  - Your transaction(s) will NOT be printed to the console.");
    }
    if let Some(path) = &action.save {
        println!("  - Your transaction(s) will be saved to {}", path.bold());
    } else {
        println!("  - Your transaction(s) will NOT be saved to a file.");
    }
    if action.broadcast {
        println!("  - Your transaction(s) will be broadcast to {}", endpoint.bold());
    } else {
        println!("  - Your transaction(s) will NOT be broadcast to the network.");
    }

    // Warnings
    let warnings = check_tasks_for_warnings(endpoint, *network, tasks, action);
    if !warnings.is_empty() {
        println!("\n{}", "⚠️ Warnings:".bold().red());
        for warning in warnings {
            println!("  - {}", warning.dimmed());
        }
    }
    println!("{}", "──────────────────────────────────────────────\n".dimmed());
}

fn print_deployment_stats<N: Network>(
    vm: &VM<N, ConsensusMemory<N>>,
    program_id: &str,
    deployment: &Deployment<N>,
    priority_fee: Option<u64>,
) -> Result<()> {
    use colored::*;
    use num_format::{Locale, ToFormattedString};
    use text_tables::render;

    // Extract stats
    let variables = deployment.num_combined_variables()?;
    let constraints = deployment.num_combined_constraints()?;

    let (base_fee, (storage_cost, synthesis_cost, constructor_cost, namespace_cost)) = deployment_cost(&vm.process().read(), deployment)?;

    // Compute final fee
    let priority_fee_value = priority_fee.unwrap_or(0) as f64 / 1_000_000.0;
    let base_fee_value = base_fee as f64 / 1_000_000.0;
    let total_fee = base_fee_value + priority_fee_value;

    // Print summary
    println!("\n{} {}", "📊 Deployment Stats for".bold(), program_id.bold());
    println!(
        "Total Variables:   {:>10}\nTotal Constraints: {:>10}\n",
        variables.to_formatted_string(&Locale::en),
        constraints.to_formatted_string(&Locale::en)
    );

    // Print cost breakdown inline
    println!("Base deployment cost for '{}' is {:.6} credits.\n", program_id.bold(), base_fee_value);

    let data = [
        [program_id, "Cost (credits)"],
        ["Transaction Storage", &format!("{:.6}", storage_cost as f64 / 1_000_000.0)],
        ["Program Synthesis", &format!("{:.6}", synthesis_cost as f64 / 1_000_000.0)],
        ["Constructor", &format!("{:.6}", constructor_cost as f64 / 1_000_000.0)],
        ["Namespace", &format!("{:.6}", namespace_cost as f64 / 1_000_000.0)],
        ["Priority Fee", &format!("{:.6}", priority_fee_value)],
        ["Total", &format!("{:.6}", total_fee)],
    ];

    let mut out = Vec::new();
    render(&mut out, data).map_err(CliError::table_render_failed)?;
    println!("{}", std::str::from_utf8(&out).map_err(CliError::table_render_failed)?);

    Ok(())
}
