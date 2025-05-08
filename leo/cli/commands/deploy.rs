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

use leo_package::{Manifest, NetworkName, Package, ProgramData};

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
use std::path::PathBuf;
use text_tables;

/// Deploys an Aleo program.
#[derive(Parser, Debug)]
pub struct LeoDeploy {
    #[clap(flatten)]
    pub(crate) fee_options: FeeOptions,
    #[clap(flatten)]
    pub(crate) action: TransactionAction,
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
    #[clap(long, help = "Time in seconds to wait between consecutive deployments.", default_value = "15")]
    pub(crate) wait: u64,
    #[clap(flatten)]
    pub(crate) options: BuildOptions,
}

impl Command for LeoDeploy {
    type Input = Package;
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        LeoBuild { options: self.options.clone() }.execute(context)
    }

    fn apply(self, context: Context, input: Self::Input) -> Result<Self::Output> {
        // Get the network, accounting for overrides.
        let network = match &self.env_override.network {
            Some(network_string) => {
                NetworkName::from_str(network_string).map_err(|_| CliError::invalid_network_name(network_string))?
            }
            None => input.env.network,
        };
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
    let private_key_string = command.env_override.private_key.clone().unwrap_or(package.env.private_key);
    let private_key = PrivateKey::<N>::from_str(&private_key_string)
        .map_err(|e| CliError::custom(format!("Failed to parse private key: {e}")))?;
    let address =
        Address::try_from(&private_key).map_err(|e| CliError::custom(format!("Failed to parse address: {e}")))?;

    // Get the endpoint, accounting for overrides.
    let endpoint = command.env_override.endpoint.clone().unwrap_or(package.env.endpoint);

    // Get the programs and optional manifests for all of the programs.
    let programs_and_manifests: Vec<(String, String, Option<Manifest>)> = package
        .programs
        .into_iter()
        .clone()
        .map(|program| {
            match program.data {
                ProgramData::Bytecode(bytecode) => Ok((program.name.to_string(), bytecode, None)),
                ProgramData::SourcePath(path) => {
                    // Define the path to the program's bytecode.
                    let path = path.join("build/main.aleo");
                    // Fetch the bytecode.
                    let bytecode = std::fs::read_to_string(&path)
                        .map_err(|e| CliError::custom(format!("Failed to read bytecode: {e}")))?;
                    // Get the package from the directory.
                    let home = context
                        .home()
                        .map_err(|e| CliError::custom(format!("Failed to find the Aleo home directory: {e}")))?;
                    let package = Package::from_directory_no_graph(&path, home)
                        .map_err(|e| CliError::custom(format!("Failed to load package at {:?}: {e}", path)))?;
                    // Return the bytecode and the manifest.
                    Ok((program.name.to_string(), bytecode, Some(package.manifest.clone())))
                }
            }
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

    // Print a summary of the deployment plan.
    print_deployment_plan(&private_key, &address, &endpoint, &network, &tasks, &command.action);

    // Prompt the user to confirm the plan.
    if !confirm("Do you want to proceed with deployment?", command.fee_options.yes)? {
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
    for (program_name, program_bytecode, manifest, _, priority_fee, fee_record) in tasks {
        // Parse the program bytecode.
        let program = Program::<N>::from_str(&program_bytecode)
            .map_err(|e| CliError::custom(format!("Failed to parse program bytecode: {e}")))?;
        // If the program is a local depdency, generate a deployment transaction.
        if manifest.is_some() {
            println!("      📦 Creating deployment transaction for '{}'...\n", &program_name.bold());
            // Generate the transaction.
            let transaction = vm
                .deploy(&private_key, &program, fee_record, priority_fee.unwrap_or(0), Some(query.clone()), rng)
                .map_err(|e| CliError::custom(format!("Failed to generate deployment transaction: {e}")))?;
            // Get the deployment.
            let deployment = transaction.deployment().expect("Expected a deployment in the transaction");
            // Print the deployment stats.
            print_deployment_stats(&program_name, deployment, priority_fee)?;
            // Check if the number of variables and constraints are within the limits.
            if deployment.num_combined_variables()? > N::MAX_DEPLOYMENT_VARIABLES {
                return Err(
                    CliError::variable_limit_exceeded(program_name, N::MAX_DEPLOYMENT_VARIABLES, network).into()
                );
            }
            if deployment.num_combined_constraints()? > N::MAX_DEPLOYMENT_CONSTRAINTS {
                return Err(
                    CliError::constraint_limit_exceeded(program_name, N::MAX_DEPLOYMENT_CONSTRAINTS, network).into()
                );
            }
            // Save the transaction.
            transactions.push((program_name, transaction));
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
            println!("      🖨️ Printing deployment for {program_name}\n{transaction_json}")
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
            println!("      💾 Saving deployment for {program_name} at {}", file_path.display());
            let transaction_json = serde_json::to_string_pretty(transaction)
                .map_err(|e| CliError::custom(format!("Failed to serialize transaction: {e}")))?;
            std::fs::write(file_path, transaction_json)
                .map_err(|e| CliError::custom(format!("Failed to write transaction to file: {e}")))?;
        }
    }

    // If the `broadcast` option is set, broadcast each deployment transaction to the network.
    if command.action.broadcast {
        for (program_name, transaction) in transactions.iter() {
            // If the fee is a public fee, check the balance of the private key.
            let fee = transaction.fee_transition().expect("Expected a fee in the transaction");
            let total_cost = *fee.amount()?;
            if fee.is_fee_public() {
                let public_balance =
                    get_public_balance(&private_key, &endpoint, &network.to_string(), &context)? as f64 / 1_000_000.0;
                println!("     💰Your current public balance is {public_balance} credits.\n");
                if public_balance < total_cost as f64 {
                    return Err(PackageError::insufficient_balance(address, public_balance, total_cost).into());
                } else {
                    confirm(
                        &format!(
                            "      This transaction will cost you {public_balance} credits. Do you want to proceed?"
                        ),
                        command.fee_options.yes,
                    )?;
                }
            }
            // Broadcast the transaction to the network.
            println!("      📡 Broadcasting deployment for {program_name}...");
            let response = handle_broadcast(
                &format!("{}/{}/transaction/broadcast", endpoint, network),
                transaction,
                program_name,
            )?;
            match response.status() {
                200 => println!(
                    "      ✅ Successfully broadcast deployment with transaction ID '{}' and fee ID '{}'!",
                    transaction.id(),
                    fee.id()
                ),
                _ => {
                    let error_message = response
                        .into_string()
                        .map_err(|e| CliError::custom(format!("Failed to read response: {e}")))?;
                    println!("      ❌ Failed to broadcast deployment: {}", error_message);
                }
            }
            // Wait between successive deployments to prevent out of order deployments.
            println!("      ⏲️ Waiting for {} seconds to allow the deployment to confirm...", command.wait)
        }
    }

    Ok(())
}

/// Pretty-print the deployment plan in a readable format.
fn print_deployment_plan<N: Network>(
    private_key: &PrivateKey<N>,
    address: &Address<N>,
    endpoint: &str,
    network: &NetworkName,
    tasks: &[(String, String, Option<Manifest>, Option<u64>, Option<u64>, Option<Record<N, Plaintext<N>>>)],
    action: &TransactionAction,
) {
    use colored::*;
    use text_tables::render;

    // Break down the tasks into the local and remote dependencies.
    let (local, remote) = tasks.iter().partition::<Vec<_>, _>(|(_, _, manifest, _, _, _)| manifest.is_some());

    println!("\n{}", "🛠️  Deployment Plan Summary".bold().underline());
    println!("{}", "──────────────────────────────────────────────".dimmed());

    // Config
    println!("{}", "🔧 Configuration:".bold());
    println!("  {:16}{}", "Private Key:".cyan(), format!("{}...", &private_key.to_string()[..12]).yellow());
    println!("  {:16}{}", "Address:".cyan(), address.to_string().yellow());
    println!("  {:16}{}", "Endpoint:".cyan(), endpoint.yellow());
    println!("  {:16}{}", "Network:".cyan(), network.to_string().yellow());

    // Tasks
    println!("\n{}", "📦 Deployment Tasks:".bold());

    let mut table =
        vec![["Program".to_string(), "Base Fee".to_string(), "Priority Fee".to_string(), "Fee Record".to_string()]];

    for (name, _, _, _, priority_fee, record) in local.iter() {
        // Base fees are not used at the moment, so we can ignore them.
        let base_fee = "auto".to_string();
        let priority_fee = priority_fee.map_or("0".into(), |v| v.to_string());
        let record = match record.is_some() {
            true => "yes".to_string(),
            false => "no (public fee)".to_string(),
        };

        table.push([name.clone(), base_fee, priority_fee, record]);
    }

    let mut buf = Vec::new();
    render(&mut buf, table).expect("table render failed");
    println!("{}", std::str::from_utf8(&buf).expect("utf8 fail"));

    // Skipped programs
    if !remote.is_empty() {
        println!("{}", "🌐 Remote Dependencies:".bold().red());
        println!("{}", "(Leo will not generate transactions for these programs):".bold().red());
        for (symbol, _, _, _, _, _) in remote {
            println!("  - {}", symbol.to_string().dimmed());
        }
    }

    // Actions
    println!("\n{}", "⚙️  Actions:".bold());
    if action.print {
        println!("  - Your transaction(s) will be printed to the console.");
    }
    if let Some(path) = &action.save {
        println!("  - Your transaction(s) will be saved to {}", path.bold());
    }
    if action.broadcast {
        println!("  - Your transaction(s) will be broadcast to {}", endpoint.bold());
    }
    println!("{}", "──────────────────────────────────────────────\n".dimmed());
}

fn print_deployment_stats<N: Network>(
    program_name: &str,
    deployment: &Deployment<N>,
    priority_fee: Option<u64>,
) -> Result<()> {
    use colored::*;
    use num_format::{Locale, ToFormattedString};
    use text_tables::render;

    // Extract stats
    let variables = deployment.num_combined_variables()?;
    let constraints = deployment.num_combined_constraints()?;

    let (base_fee, (storage_cost, synthesis_cost, namespace_cost)) = deployment_cost(deployment)?;

    // Compute final fee
    let priority_fee_value = priority_fee.unwrap_or(0) as f64 / 1_000_000.0;
    let base_fee_value = base_fee as f64 / 1_000_000.0;
    let total_fee = base_fee_value + priority_fee_value;

    // Print summary
    println!("\n{} {}", "📊 Deployment Stats for".bold(), program_name.bold());
    println!(
        "      Total Variables:   {:>10}\n      Total Constraints: {:>10}\n",
        variables.to_formatted_string(&Locale::en),
        constraints.to_formatted_string(&Locale::en)
    );

    // Print cost breakdown inline
    println!("      Base deployment cost for '{}' is {:.6} credits.\n", program_name.bold(), base_fee_value);

    let data = [
        [program_name, "Cost (credits)"],
        ["Transaction Storage", &format!("{:.6}", storage_cost as f64 / 1_000_000.0)],
        ["Program Synthesis", &format!("{:.6}", synthesis_cost as f64 / 1_000_000.0)],
        ["Namespace", &format!("{:.6}", namespace_cost as f64 / 1_000_000.0)],
        ["Priority Fee", &format!("{:.6}", priority_fee_value)],
        ["Total", &format!("{:.6}", total_fee)],
    ];

    let mut out = Vec::new();
    render(&mut out, data).map_err(CliError::table_render_failed)?;
    println!("      {}", std::str::from_utf8(&out).map_err(CliError::table_render_failed)?);

    Ok(())
}
