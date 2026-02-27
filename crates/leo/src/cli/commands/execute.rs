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

use check_transaction::TransactionStatus;
use leo_ast::NetworkName;
use leo_package::{Package, ProgramData, fetch_program_from_network};

use aleo_std::StorageMode;
use rand::{CryptoRng, Rng};
use snarkvm::prelude::{
    Authorization,
    Execution,
    Fee,
    Field,
    Itertools,
    Network,
    Program,
    execution_cost,
    execution_cost_for_authorization,
};

use clap::Parser;
use colored::*;
use std::{convert::TryFrom, path::PathBuf};

#[cfg(not(feature = "only_testnet"))]
use snarkvm::circuit::{AleoCanaryV0, AleoV0};
use snarkvm::{
    circuit::{Aleo, AleoTestnetV0},
    prelude::{
        ConsensusVersion,
        Identifier,
        ProgramID,
        VM,
        query::{Query as SnarkVMQuery, QueryTrait},
        store::{
            ConsensusStore,
            helpers::memory::{BlockMemory, ConsensusMemory},
        },
    },
};

/// Build, Prove and Run Leo program with inputs
#[derive(Parser, Debug)]
pub struct LeoExecute {
    #[clap(
        name = "NAME",
        help = "The name of the function to execute, e.g `helloworld.aleo/main` or `main`.",
        default_value = "main"
    )]
    name: String,
    #[clap(
        name = "INPUTS",
        help = "The program inputs e.g. `1u32`, `record1...` (record ciphertext), or `{ owner: ...}` "
    )]
    inputs: Vec<String>,
    #[clap(long, help = "Skips proving.")]
    pub(crate) skip_execute_proof: bool,
    #[clap(flatten)]
    pub(crate) fee_options: FeeOptions,
    #[clap(flatten)]
    pub(crate) action: TransactionAction,
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
    #[clap(flatten)]
    pub(crate) extra: ExtraOptions,
    #[clap(flatten)]
    build_options: BuildOptions,
}

impl Command for LeoExecute {
    type Input = Option<Package>;
    type Output = ExecuteOutput;

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        // Get the path to the current directory.
        let path = context.dir()?;
        // Get the path to the home directory.
        let home_path = context.home()?;
        // Get the network, accounting for overrides.
        let network = get_network(&self.env_override.network)?;
        // Get the endpoint, accounting for overrides.
        let endpoint = get_endpoint(&self.env_override.endpoint)?;
        // If the current directory is a valid Leo package, then build it.
        if Package::from_directory_no_graph(path, home_path, Some(network), Some(&endpoint)).is_ok() {
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
        // Get the network, accounting for overrides.
        let network = get_network(&self.env_override.network)?;
        // Handle each network with the appropriate parameterization.
        match network {
            NetworkName::TestnetV0 => handle_execute::<AleoTestnetV0>(self, context, network, input),
            NetworkName::MainnetV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Mainnet chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                handle_execute::<AleoV0>(self, context, network, input)
            }
            NetworkName::CanaryV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Canary chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                handle_execute::<AleoCanaryV0>(self, context, network, input)
            }
        }
    }
}

// A helper function to handle the `execute` command.
fn handle_execute<A: Aleo>(
    command: LeoExecute,
    context: Context,
    network: NetworkName,
    package: Option<Package>,
) -> Result<<LeoExecute as Command>::Output> {
    // Get the private key and associated address, accounting for overrides.
    let private_key = get_private_key(&command.env_override.private_key)?;
    let address = Address::<A::Network>::try_from(&private_key)
        .map_err(|e| CliError::custom(format!("Failed to parse address: {e}")))?;

    // Get the endpoint, accounting for overrides.
    let endpoint = get_endpoint(&command.env_override.endpoint)?;

    // Get whether the network is a devnet, accounting for overrides.
    let is_devnet = get_is_devnet(command.env_override.devnet);

    // If the consensus heights are provided, use them; otherwise, use the default heights for the network.
    let consensus_heights =
        command.env_override.consensus_heights.clone().unwrap_or_else(|| get_consensus_heights(network, is_devnet));
    // Validate the provided consensus heights.
    validate_consensus_heights(&consensus_heights)
        .map_err(|e| CliError::custom(format!("Invalid consensus heights: {e}")))?;
    // Print the consensus heights being used.
    let consensus_heights_string = consensus_heights.iter().format(",").to_string();
    println!(
        "\n📢 Using the following consensus heights: {consensus_heights_string}\n  To override, pass in `--consensus-heights` or override the environment variable `CONSENSUS_VERSION_HEIGHTS`.\n"
    );

    // Set the consensus heights in the environment.
    #[allow(unsafe_code)]
    unsafe {
        // SAFETY:
        //  - `CONSENSUS_VERSION_HEIGHTS` is only set once and is only read in `snarkvm::prelude::load_consensus_heights`.
        //  - There are no concurrent threads running at this point in the execution.
        // WHY:
        //  - This is needed because there is no way to set the desired consensus heights for a particular `VM` instance
        //    without using the environment variable `CONSENSUS_VERSION_HEIGHTS`. Which is itself read once, and stored in a `OnceLock`.
        std::env::set_var("CONSENSUS_VERSION_HEIGHTS", consensus_heights_string);
    }

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

    // Get the first fee option.
    let (base_fee, priority_fee, record) =
        parse_fee_options(&private_key, &command.fee_options, 1)?.into_iter().next().unwrap_or((None, None, None));

    // Get the consensus version.
    let consensus_version =
        get_consensus_version(&command.extra.consensus_version, &endpoint, network, &consensus_heights, &context)?;

    // Build the config for JSON output.
    let config = Some(Config {
        address: address.to_string(),
        network: network.to_string(),
        endpoint: Some(endpoint.clone()),
        consensus_version: Some(consensus_version as u8),
    });

    // Print the execution plan.
    print_execution_plan::<A::Network>(
        &private_key,
        &address,
        &endpoint,
        &network,
        &program_name,
        &function_name,
        is_local,
        priority_fee.unwrap_or(0),
        record.is_some(),
        &command.action,
        consensus_version,
        &check_task_for_warnings(&endpoint, network, &programs, consensus_version),
        command.skip_execute_proof,
    );

    // Prompt the user to confirm the plan.
    if !confirm("Do you want to proceed with execution?", command.extra.yes)? {
        println!("❌ Execution aborted.");
        return Ok(ExecuteOutput::default());
    }

    // Initialize an RNG.
    let rng = &mut rand::thread_rng();

    // Initialize a new VM.
    let vm = VM::from(ConsensusStore::<A::Network, ConsensusMemory<A::Network>>::open(StorageMode::Production)?)?;

    // Specify the query.
    let query = SnarkVMQuery::<A::Network, BlockMemory<A::Network>>::from(
        endpoint
            .parse::<Uri>()
            .map_err(|e| CliError::custom(format!("Failed to parse endpoint URI '{endpoint}': {e}")))?,
    );

    // If the program is not local, then download it and its dependencies for the network.
    // Note: The dependencies are downloaded in "post-order" (child before parent).
    if !is_local {
        println!("⬇️ Downloading {program_name} and its dependencies from {endpoint}...");
        programs = load_latest_programs_from_network(&context, program_id, network, &endpoint)?;
    };

    // Add the programs to the VM.
    println!("\n➕Adding programs to the VM in the following order:");
    let programs_and_editions = programs
        .into_iter()
        .map(|(program, edition)| {
            print_program_source(&program.id().to_string(), edition);
            let edition = edition.unwrap_or(LOCAL_PROGRAM_DEFAULT_EDITION);
            (program, edition)
        })
        .collect::<Vec<_>>();
    vm.process().write().add_programs_with_editions(&programs_and_editions)?;

    // Generate the authorization (the method differs based on skip_execute_proof).
    let authorization = if command.skip_execute_proof {
        println!("\n⚙️ Generating transaction WITHOUT a proof for {program_name}/{function_name}...");
        vm.process().read().authorize::<A, _>(&private_key, &program_name, &function_name, inputs.iter(), rng)?
    } else {
        println!("\n⚙️ Executing {program_name}/{function_name}...");
        vm.authorize(&private_key, &program_name, &function_name, inputs.iter(), rng)?
    };

    // Estimate and display execution cost.
    let (estimated_cost, (est_storage, est_exec)) =
        execution_cost_for_authorization(&vm.process().read(), &authorization, consensus_version)?;
    let stats = print_execution_cost_summary(&program_name, est_storage, est_exec, priority_fee);

    // Generate the transaction (the method differs based on skip_execute_proof).
    let (output_name, output, response) = if command.skip_execute_proof {
        // Get the state root.
        let state_root = query.current_state_root()?;

        // Create an execution without the proof.
        let execution = Execution::from(authorization.transitions().values().cloned(), state_root, None)?;

        // Calculate the actual cost for fee authorization.
        let (cost, _) = execution_cost(&vm.process().read(), &execution, consensus_version)?;

        // Generate the fee authorization.
        let id = authorization.to_execution_id()?;
        let fee_authorization = authorize_fee::<A, _>(
            &vm,
            &private_key,
            record,
            base_fee.unwrap_or(cost),
            priority_fee.unwrap_or(0),
            id,
            rng,
        )?;

        // Create a fee transition without a proof.
        let fee = Fee::from(fee_authorization.transitions().into_iter().next().unwrap().1, state_root, None)?;

        // Create the transaction.
        let transaction = Transaction::from_execution(execution, Some(fee))?;

        // Evaluate the transaction to get the response.
        let response = vm.process().read().evaluate::<A>(authorization)?;

        ("transaction", Box::new(transaction), response)
    } else {
        // Determine if a fee is required.
        let is_fee_required = !(authorization.is_split() || authorization.is_upgrade());
        let is_priority_fee_declared = priority_fee.unwrap_or(0) > 0;

        // Build fee authorization using the estimated cost.
        let fee_authorization = if is_fee_required || is_priority_fee_declared {
            let execution_id = authorization.to_execution_id()?;
            Some(authorize_fee::<A, _>(
                &vm,
                &private_key,
                record,
                base_fee.unwrap_or(estimated_cost),
                priority_fee.unwrap_or(0),
                execution_id,
                rng,
            )?)
        } else {
            None
        };

        // Execute with the existing authorization (no re-authorization).
        let (transaction, response) =
            vm.execute_authorization_with_response(authorization, fee_authorization, Some(&query), rng)?;
        ("transaction", Box::new(transaction), response)
    };

    let transaction = output.clone();

    // Print the transaction.
    // If the `print` option is set, print the execution transaction to the console.
    // The transaction is printed in JSON format.
    if command.action.print {
        let transaction_json = serde_json::to_string_pretty(&output)
            .map_err(|e| CliError::custom(format!("Failed to serialize transaction: {e}")))?;
        println!("🖨️ Printing execution for {output_name}\n{transaction_json}");
    }

    // If the `save` option is set, save the execution transaction to a file in the specified directory.
    // The file format is `program_name.execution.json`.
    // The directory is created if it doesn't exist.
    if let Some(path) = &command.action.save {
        // Create the directory if it doesn't exist.
        std::fs::create_dir_all(path).map_err(|e| CliError::custom(format!("Failed to create directory: {e}")))?;
        // Save the transaction to a file.
        let file_path = PathBuf::from(path).join(format!("{output_name}.execution.json"));
        println!("💾 Saving execution for {output_name} at {}", file_path.display());
        let transaction_json = serde_json::to_string_pretty(&output)
            .map_err(|e| CliError::custom(format!("Failed to serialize transaction: {e}")))?;
        std::fs::write(file_path, transaction_json)
            .map_err(|e| CliError::custom(format!("Failed to write transaction to file: {e}")))?;
    }

    let mut broadcast_stats = None;

    // Collect outputs.
    let outputs: Vec<String> = response.outputs().iter().map(|o| o.to_string()).collect();

    match outputs.len() {
        0 => (),
        1 => println!("\n➡️  Output\n"),
        _ => println!("\n➡️  Outputs\n"),
    };
    for o in &outputs {
        println!(" • {o}");
    }
    println!();

    // If the `broadcast` option is set, broadcast each deployment transaction to the network.
    if command.action.broadcast {
        println!("📡 Broadcasting execution for {program_name}...");
        // Get and confirm the fee with the user.
        let mut fee_id = None;
        let mut fee_transaction_id = None;
        if let Some(fee) = transaction.fee_transition() {
            // Most transactions will have fees, but some, like credits.aleo/upgrade executions, may not.
            if !confirm_fee(&fee, &private_key, &address, &endpoint, network, &context, command.extra.yes)? {
                println!("❌ Execution aborted.");
                return Ok(ExecuteOutput {
                    config: config.clone(),
                    program: program_name.clone(),
                    function: function_name.clone(),
                    outputs,
                    transaction_id: transaction.id().to_string(),
                    stats: Some(stats),
                    broadcast: None,
                });
            }
            fee_id = Some(fee.id().to_string());
            fee_transaction_id = Some(Transaction::from_fee(fee.clone())?.id().to_string());
        }
        let id = transaction.id().to_string();
        let height_before = check_transaction::current_height(&endpoint, network)?;
        // Broadcast the transaction to the network.
        let (message, status) =
            handle_broadcast(&format!("{endpoint}/{network}/transaction/broadcast"), &transaction, &program_name)?;

        match status {
            200..=299 => {
                let tx_status = check_transaction::check_transaction_with_message(
                    &id,
                    fee_id.as_deref(),
                    &endpoint,
                    network,
                    height_before + 1,
                    command.extra.max_wait,
                    command.extra.blocks_to_check,
                )?;
                let confirmed = tx_status == Some(TransactionStatus::Accepted);
                if confirmed {
                    println!("✅ Execution confirmed!");
                }
                broadcast_stats = Some(BroadcastStats {
                    fee_id: fee_id.unwrap_or_default(),
                    fee_transaction_id: fee_transaction_id.unwrap_or_default(),
                    confirmed,
                });
            }
            _ => {
                println!("❌ Failed to broadcast execution: {message}.");
            }
        }
    }

    Ok(ExecuteOutput {
        config,
        program: program_name.clone(),
        function: function_name.clone(),
        outputs,
        transaction_id: transaction.id().to_string(),
        stats: Some(stats),
        broadcast: broadcast_stats,
    })
}

/// Authorize a fee transition using either a private record or public credits.
fn authorize_fee<A: Aleo, R: Rng + CryptoRng>(
    vm: &VM<A::Network, ConsensusMemory<A::Network>>,
    private_key: &PrivateKey<A::Network>,
    record: Option<Record<A::Network, Plaintext<A::Network>>>,
    base_fee: u64,
    priority_fee: u64,
    execution_id: Field<A::Network>,
    rng: &mut R,
) -> Result<Authorization<A::Network>> {
    match record {
        None => vm.authorize_fee_public(private_key, base_fee, priority_fee, execution_id, rng),
        Some(record) => vm.authorize_fee_private(private_key, record, base_fee, priority_fee, execution_id, rng),
    }
    .map_err(Into::into)
}

/// Check the execution task for warnings.
/// The following properties are checked:
///   - The component programs exist on the network and match the local ones.
fn check_task_for_warnings<N: Network>(
    endpoint: &str,
    network: NetworkName,
    programs: &[(Program<N>, Option<u16>)],
    consensus_version: ConsensusVersion,
) -> Vec<String> {
    let mut warnings = Vec::new();
    for (program, _) in programs {
        // Check if the program exists on the network.
        if let Ok(remote_program) = fetch_program_from_network(&program.id().to_string(), endpoint, network) {
            // Parse the program.
            let remote_program = match Program::<N>::from_str(&remote_program) {
                Ok(program) => program,
                Err(e) => {
                    warnings.push(format!("Could not parse '{}' from the network. Error: {e}", program.id()));
                    continue;
                }
            };
            // Check if the program matches the local one.
            if remote_program != *program {
                warnings.push(format!(
                    "The program '{}' on the network does not match the local copy. If you have a local dependency, you may use the `--no-local` flag to use the network version instead.",
                    program.id()
                ));
            }
        } else {
            warnings.push(format!(
                "The program '{}' does not exist on the network. You may use `leo deploy --broadcast` to deploy it.",
                program.id()
            ));
        }
    }
    // Check for a consensus version mismatch.
    if let Err(e) = check_consensus_version_mismatch(consensus_version, endpoint, network) {
        warnings.push(format!("{e}. In some cases, the execution may fail"));
    }
    warnings
}

/// Pretty-print the execution plan in a readable format.
#[allow(clippy::too_many_arguments)]
fn print_execution_plan<N: Network>(
    private_key: &PrivateKey<N>,
    address: &Address<N>,
    endpoint: &str,
    network: &NetworkName,
    program_name: &str,
    function_name: &str,
    is_local: bool,
    priority_fee: u64,
    fee_record: bool,
    action: &TransactionAction,
    consensus_version: ConsensusVersion,
    warnings: &[String],
    skip_execute_proof: bool,
) {
    println!("\n{}", "🚀 Execution Plan Summary".bold().underline());
    println!("{}", "──────────────────────────────────────────────".dimmed());

    println!("{}", "🔧 Configuration:".bold());
    println!("  {:20}{}", "Private Key:".cyan(), format!("{}...", &private_key.to_string()[..24]).yellow());
    println!("  {:20}{}", "Address:".cyan(), format!("{}...", &address.to_string()[..24]).yellow());
    println!("  {:20}{}", "Endpoint:", endpoint.yellow());
    println!("  {:20}{}", "Network:", network.to_string().yellow());
    println!("  {:20}{}", "Consensus Version:", (consensus_version as u8).to_string().yellow());

    println!("\n{}", "🎯 Execution Target:".bold());
    println!("  {:16}{}", "Program:", program_name.cyan());
    println!("  {:16}{}", "Function:", function_name.cyan());
    println!("  {:16}{}", "Source:", if is_local { "local" } else { "remote" });

    println!("\n{}", "💸 Fee Info:".bold());
    println!("  {:16}{}", "Priority Fee:", format!("{priority_fee} μcredits").green());
    println!("  {:16}{}", "Fee Record:", if fee_record { "yes" } else { "no (public fee)" });

    println!("\n{}", "⚙️ Actions:".bold());
    if !is_local {
        println!("  - Program and its dependencies will be downloaded from the network.");
    }
    if skip_execute_proof {
        println!("  - A transaction will be generated, WITHOUT a proof.");
    }
    if action.print {
        println!("  - Transaction will be printed to the console.");
    } else {
        println!("  - Transaction will NOT be printed to the console.");
    }
    if let Some(path) = &action.save {
        println!("  - Transaction will be saved to {}", path.bold());
    } else {
        println!("  - Transaction will NOT be saved to a file.");
    }
    if action.broadcast {
        println!("  - Transaction will be broadcast to {}", endpoint.bold());
    } else {
        println!("  - Transaction will NOT be broadcast to the network.");
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

/// Print execution cost summary and return stats for JSON output.
fn print_execution_cost_summary(
    program_name: &str,
    storage_cost: u64,
    execution_cost: u64,
    priority_fee: Option<u64>,
) -> ExecutionStats {
    use colored::*;

    let priority = priority_fee.unwrap_or(0);
    let total = storage_cost + execution_cost + priority;
    let stats = ExecutionStats { storage_cost, execution_cost, priority_fee: priority, total_cost: total };

    println!("\n{} {}", "📊 Execution Cost Summary for".bold(), program_name.bold());
    println!("{}", "──────────────────────────────────────────────".dimmed());
    print!("{stats}");
    println!("{}", "──────────────────────────────────────────────".dimmed());

    stats
}
