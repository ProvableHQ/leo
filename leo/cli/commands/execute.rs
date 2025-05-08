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

use leo_package::{NetworkName, Package, ProgramData};

use aleo_std::StorageMode;
use clap::Parser;
use snarkvm::prelude::{Execution, Network};
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
        Value,
        execution_cost_v1,
        execution_cost_v2,
        query::Query as SnarkVMQuery,
        store::{ConsensusStore, helpers::memory::ConsensusMemory},
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
    #[clap(name = "INPUTS", help = "The inputs to the program.")]
    inputs: Vec<String>,
    #[clap(flatten)]
    pub(crate) fee_options: FeeOptions,
    #[clap(flatten)]
    pub(crate) action: TransactionAction,
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
    #[clap(flatten)]
    compiler_options: BuildOptions,
    #[arg(short, long, help = "The inputs to the program, from a file. Overrides the INPUTS argument.")]
    file: Option<String>,
}

impl Command for LeoExecute {
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
        if let Ok(package) = Package::from_directory(path, home_path) {
            LeoBuild { options: self.compiler_options.clone() }.execute(context.clone())?;
            // Return the package.
            Ok(Some(package))
        } else {
            Ok(None)
        }
    }

    fn apply(self, context: Context, input: Self::Input) -> Result<Self::Output> {
        // Get the network, accounting for overrides.
        let network = match &self.env_override.network {
            Some(network_string) => {
                NetworkName::from_str(network_string).map_err(|_| CliError::invalid_network_name(network_string))?
            }
            // Use the `network` from the package if we are in a Leo package.
            None => match &input {
                Some(package) => package.env.network,
                None => {
                    return Err(CliError::custom(
                        "Could not find a valid network via an `.env` file or `--network` CLI option.".to_string(),
                    )
                    .into());
                }
            },
        };
        // Handle each network with the appropriate parameterization.
        match network {
            NetworkName::TestnetV0 => handle_execute::<AleoTestnetV0>(self, context, network, input),
            NetworkName::MainnetV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Mainnet chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                return handle_execute::<AleoV0>(self, context, network, input);
            }
            NetworkName::CanaryV0 => {
                #[cfg(feature = "only_testnet")]
                panic!("Canary chosen with only_testnet feature");
                #[cfg(not(feature = "only_testnet"))]
                return handle_execute::<AleoCanaryV0>(self, context, network, input);
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
    let Some(private_key_string) =
        command.env_override.private_key.clone().or(package.as_ref().map(|package| package.env.private_key.clone()))
    else {
        return Err(CliError::custom(
            "Could not find a valid private key via an `.env` file or `--private-key` CLI option.".to_string(),
        )
        .into());
    };
    let private_key = PrivateKey::<A::Network>::from_str(&private_key_string)
        .map_err(|e| CliError::custom(format!("Failed to parse private key: {e}")))?;
    let address = Address::<A::Network>::try_from(&private_key)
        .map_err(|e| CliError::custom(format!("Failed to parse address: {e}")))?;

    // Get the endpoint, accounting for overrides.
    let Some(endpoint) =
        command.env_override.endpoint.clone().or(package.as_ref().map(|package| package.env.endpoint.clone()))
    else {
        return Err(CliError::custom(
            "Could not find a valid endpoint via an `.env` file or `--endpoint` CLI option.".to_string(),
        )
        .into());
    };

    // Parse the <NAME> into an optional program name and a function name.
    // If only a function name is provided, then use the program name from the package.
    let (program_name, function_name) = match command.name.split_once('/') {
        Some((program_name, function_name)) => (program_name.to_string(), function_name.to_string()),
        None => match &package {
            Some(package) => (
                package.programs.last().expect("There must be at least one program in a Leo package").name.to_string(),
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
    ProgramID::<A::Network>::from_str(&program_name)
        .map_err(|e| CliError::custom(format!("Failed to parse program name: {e}")))?;
    // Parse the function name as an `Identifier`.
    let function_id = Identifier::<A::Network>::from_str(&function_name)
        .map_err(|e| CliError::custom(format!("Failed to parse function name: {e}")))?;

    // Get all the dependencies in the package if it exists.
    // Get the programs and optional manifests for all of the programs.
    let programs = if let Some(package) = &package {
        package
            .programs
            .iter()
            .clone()
            .map(|program| {
                match &program.data {
                    ProgramData::Bytecode(bytecode) => Ok((program.name.to_string(), bytecode.to_string())),
                    ProgramData::SourcePath(path) => {
                        // Define the path to the program's bytecode.
                        let path = path.join("build/main.aleo");
                        // Fetch the bytecode.
                        let bytecode = std::fs::read_to_string(&path)
                            .map_err(|e| CliError::custom(format!("Failed to read bytecode: {e}")))?;
                        // Return the bytecode and the manifest.
                        Ok((program.name.to_string(), bytecode))
                    }
                }
            })
            .collect::<Result<Vec<_>>>()?
    } else {
        Vec::new()
    };

    // Parse the program strings into AVM programs.
    let programs: Vec<(String, snarkvm::prelude::Program<A::Network>)> = programs
        .into_iter()
        .map(|(name, bytecode)| {
            // Parse the program.
            let program = snarkvm::prelude::Program::<A::Network>::from_str(&bytecode)
                .map_err(|e| CliError::custom(format!("Failed to parse program: {e}")))?;
            // Return the program and its name.
            Ok((name, program))
        })
        .collect::<Result<Vec<_>>>()?;

    // Determine whether the program is local or remote.
    let is_local = programs.iter().any(|(name, _)| name == &program_name);

    // If the program is local, then check that the function exists.
    if is_local {
        let program =
            &programs.iter().find(|(name, _)| name == &program_name).expect("Program should exist since it is local").1;
        if !program.contains_function(&function_id) {
            return Err(CliError::custom(format!(
                "Function `{function_name}` does not exist in program `{program_name}`."
            ))
            .into());
        }
    }

    // Parse the inputs.
    // If the `file` option is set, read the inputs from the file.
    let input_strings = if let Some(file) = command.file {
        // Read the inputs from the file.
        let file = std::fs::read_to_string(file).map_err(|e| CliError::custom(format!("Failed to read file: {e}")))?;
        // Parse the inputs from the file.
        file.lines().map(|line| line.to_string()).collect::<Vec<_>>()
    } else {
        // Use the inputs from the command line.
        command.inputs
    };
    let inputs = input_strings
        .into_iter()
        .map(|input| {
            Value::from_str(&input).map_err(|e| CliError::custom(format!("Failed to parse input: {e}")).into())
        })
        .collect::<Result<Vec<_>>>()?;

    // Get the first fee option.
    let (_, priority_fee, record) =
        parse_fee_options(&private_key, &command.fee_options, 1)?.into_iter().next().unwrap_or((None, None, None));

    // Get the consensus version.
    let consensus_version = match command.fee_options.consensus_version {
        Some(1) => ConsensusVersion::V1,
        Some(2) => ConsensusVersion::V2,
        Some(3) => ConsensusVersion::V3,
        Some(4) => ConsensusVersion::V4,
        Some(5) => ConsensusVersion::V4,
        None => ConsensusVersion::V4,
        Some(version) => return Err(CliError::custom(format!("Invalid consensus version: {version}")).into()),
    };

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
    );

    // Prompt the user to confirm the plan.
    if !confirm("Do you want to proceed with execution?", command.fee_options.yes)? {
        println!("❌ Execution aborted.");
        return Ok(());
    }

    // Initialize an RNG.
    let rng = &mut rand::thread_rng();

    // Initialize a new VM.
    let vm = VM::from(ConsensusStore::<A::Network, ConsensusMemory<A::Network>>::open(StorageMode::Production)?)?;

    // Specify the query
    let query = SnarkVMQuery::from(&endpoint);

    // If the program is not local, then download it and its dependencies for the network.
    // Note: The depdenencies are downloaded in "pre-order" (parent before child).
    let programs = if is_local {
        programs
    } else {
        println!("      ⬇️ Downloading {program_name} and its dependencies from {endpoint}...");
        let mut programs = load_programs_from_network(&context, &program_name, &network.to_string(), &endpoint)?;
        programs.reverse();
        programs
    };

    // Add the programs to the VM.
    println!("      Adding programs to the VM ...");
    for (_, program) in programs {
        vm.process().write().add_program(&program)?;
    }

    // Parse the inputs.

    // Execute the program and produce a transaction.
    let transaction = vm.execute(
        &private_key,
        (&program_name, &function_name),
        inputs.iter(),
        record,
        priority_fee.unwrap_or(0),
        Some(query),
        rng,
    )?;

    // Print the execution stats.
    print_execution_stats::<A::Network>(
        &vm,
        &program_name,
        transaction.execution().expect("Expected execution"),
        priority_fee,
        consensus_version,
    )?;

    // Print the transaction.
    // If the `print` option is set, print the execution transaction to the console.
    // The transaction is printed in JSON format.
    if command.action.print {
        let transaction_json = serde_json::to_string_pretty(&transaction)
            .map_err(|e| CliError::custom(format!("Failed to serialize transaction: {e}")))?;
        println!("      🖨️ Printing execution for {program_name}\n{transaction_json}");
    }

    // If the `save` option is set, save the execution transaction to a file in the specified directory.
    // The file format is `program_name.execution.json`.
    // The directory is created if it doesn't exist.
    if let Some(path) = &command.action.save {
        // Create the directory if it doesn't exist.
        std::fs::create_dir_all(path).map_err(|e| CliError::custom(format!("Failed to create directory: {e}")))?;
        // Save the transaction to a file.
        let file_path = PathBuf::from(path).join(format!("{program_name}.execution.json"));
        println!("      💾 Saving execution for {program_name} at {}", file_path.display());
        let transaction_json = serde_json::to_string_pretty(&transaction)
            .map_err(|e| CliError::custom(format!("Failed to serialize transaction: {e}")))?;
        std::fs::write(file_path, transaction_json)
            .map_err(|e| CliError::custom(format!("Failed to write transaction to file: {e}")))?;
    }

    // If the `broadcast` option is set, broadcast each deployment transaction to the network.
    if command.action.broadcast {
        // If the fee is a public fee, check the balance of the private key.
        let fee = transaction.fee_transition().expect("Expected a fee in the transaction");
        let total_cost = *fee.amount()?;
        if fee.is_fee_public() {
            check_balance(
                &private_key,
                &endpoint,
                &network.to_string(),
                &context,
                total_cost,
                command.fee_options.yes,
            )?;
        }
        // Broadcast the transaction to the network.
        println!("      📡 Broadcasting execution for {program_name}...");
        let response =
            handle_broadcast(&format!("{}/{}/transaction/broadcast", endpoint, network), &transaction, &program_name)?;
        match response.status() {
            200 => println!(
                "      ✅ Successfully broadcast execution with transaction ID '{}' and fee ID '{}'!",
                transaction.id(),
                fee.id()
            ),
            _ => {
                let error_message =
                    response.into_string().map_err(|e| CliError::custom(format!("Failed to read response: {e}")))?;
                println!("      ❌ Failed to broadcast execution: {}", error_message);
            }
        }
    }

    Ok(())
}

/// Pretty-print the execution plan in a readable format.
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
) {
    use colored::*;
    println!("\n{}", "🚀 Execution Plan Summary".bold().underline());
    println!("{}", "──────────────────────────────────────────────".dimmed());

    println!("{}", "🔧 Configuration:".bold());
    println!("  {:16}{}", "Private Key:", format!("{}...", &private_key.to_string()[..12]).yellow());
    println!("  {:16}{}", "Address:", address.to_string().yellow());
    println!("  {:16}{}", "Endpoint:", endpoint.yellow());
    println!("  {:16}{}", "Network:", network.to_string().yellow());

    println!("\n{}", "🎯 Execution Target:".bold());
    println!("  {:16}{}", "Program:", program_name.cyan());
    println!("  {:16}{}", "Function:", function_name.cyan());
    println!("  {:16}{}", "Source:", if is_local { "local" } else { "remote" });

    println!("\n{}", "💸 Fee Info:".bold());
    println!("  {:16}{}", "Priority Fee:", format!("{} μcredits", priority_fee).green());
    println!("  {:16}{}", "Fee Record:", if fee_record { "yes" } else { "no (public fee)" });

    println!("\n{}", "⚙️  Actions:".bold());
    if !is_local {
        println!("  - Program and its dependencies will be downloaded from the network.");
    }
    if action.print {
        println!("  - Transaction will be printed to the console.");
    }
    if let Some(path) = &action.save {
        println!("  - Transaction will be saved to {}", path.bold());
    }
    if action.broadcast {
        println!("  - Transaction will be broadcast to {}", endpoint.bold());
    }
    println!("{}", "──────────────────────────────────────────────\n".dimmed());
}

fn print_execution_stats<N: Network>(
    vm: &VM<N, ConsensusMemory<N>>,
    program_name: &str,
    execution: &Execution<N>,
    priority_fee: Option<u64>,
    consensus_version: ConsensusVersion,
) -> Result<()> {
    use colored::*;
    use text_tables::render;

    // Compute the execution cost using the VM's process.
    let (base_fee, (storage_cost, execution_cost)) = if consensus_version == ConsensusVersion::V1 {
        execution_cost_v1(&vm.process().read(), execution)?
    } else {
        execution_cost_v2(&vm.process().read(), execution)?
    };

    // Convert microcredits to credits
    let base_fee_value = base_fee as f64 / 1_000_000.0;
    let priority_fee_value = priority_fee.unwrap_or(0) as f64 / 1_000_000.0;
    let total_fee = base_fee_value + priority_fee_value;

    // Print summary
    println!("\n{} {}", "📊 Execution Stats for".bold(), program_name.bold());
    println!("      Base execution cost for '{}' is {:.6} credits.\n", program_name.bold(), base_fee_value);

    let data = [
        [program_name, "Cost (credits)"],
        ["Transaction Storage", &format!("{:.6}", storage_cost as f64 / 1_000_000.0)],
        ["On-chain Execution", &format!("{:.6}", execution_cost as f64 / 1_000_000.0)],
        ["Priority Fee", &format!("{:.6}", priority_fee_value)],
        ["Total", &format!("{:.6}", total_fee)],
    ];

    let mut out = Vec::new();
    render(&mut out, data).map_err(CliError::table_render_failed)?;
    println!("      {}", std::str::from_utf8(&out).map_err(CliError::table_render_failed)?);

    Ok(())
}
