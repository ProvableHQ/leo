// Copyright (C) 2019-2023 Aleo Systems Inc.
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
use aleo_std::StorageMode;
use clap::Parser;
use snarkvm::{
    cli::{helpers::dotenv_private_key, Execute as SnarkVMExecute},
    prelude::{MainnetV0, Network, Parser as SnarkVMParser, TestnetV0},
};

use crate::cli::query::QueryCommands;
use leo_retriever::NetworkName;
use snarkvm::{
    ledger::Transaction::Execute as ExecuteTransaction,
    prelude::{
        execution_cost,
        query::Query as SnarkVMQuery,
        store::{
            helpers::memory::{BlockMemory, ConsensusMemory},
            ConsensusStore,
        },
        Address,
        Process,
        Program as SnarkVMProgram,
        ProgramID,
        VM,
    },
};

/// Build, Prove and Run Leo program with inputs
#[derive(Parser, Debug)]
pub struct Execute {
    #[clap(name = "NAME", help = "The name of the function to execute.", default_value = "main")]
    name: String,
    #[clap(name = "INPUTS", help = "The inputs to the program.")]
    inputs: Vec<String>,
    #[clap(short, long, help = "Execute the transition on-chain.", default_value = "false")]
    broadcast: bool,
    #[clap(short, long, help = "Execute the local program on-chain.", default_value = "false")]
    local: bool,
    #[clap(short, long, help = "The program to execute on-chain.")]
    program: Option<String>,
    #[clap(flatten)]
    fee_options: FeeOptions,
    #[clap(flatten)]
    compiler_options: BuildOptions,
    #[arg(short, long, help = "The inputs to the program, from a file. Overrides the INPUTS argument.")]
    file: Option<String>,
}

impl Command for Execute {
    type Input = <Build as Command>::Output;
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        // No need to build if we are executing an external program.
        if self.program.is_some() {
            return Ok(());
        }
        (Build { options: self.compiler_options.clone() }).execute(context)
    }

    fn apply(self, context: Context, _input: Self::Input) -> Result<Self::Output> {
        // Parse the network.
        let network = NetworkName::try_from(self.compiler_options.network.as_str())?;
        match network {
            NetworkName::MainnetV0 => handle_execute::<MainnetV0>(self, context),
            NetworkName::TestnetV0 => handle_execute::<TestnetV0>(self, context),
        }
    }
}

// A helper function to handle the `execute` command.
fn handle_execute<N: Network>(command: Execute, context: Context) -> Result<<Execute as Command>::Output> {
    // If input values are provided, then run the program with those inputs.
    // Otherwise, use the input file.
    let mut inputs = command.inputs.clone();

    // Compose the `execute` command.
    let mut arguments = vec![SNARKVM_COMMAND.to_string(), command.name.clone()];

    // Add the inputs to the arguments.
    match command.file.clone() {
        Some(file) => {
            // Get the contents from the file.
            let path = context.dir()?.join(file);
            let raw_content =
                std::fs::read_to_string(&path).map_err(|err| PackageError::failed_to_read_file(path.display(), err))?;
            // Parse the values from the file.
            let mut content = raw_content.as_str();
            let mut values = vec![];
            while let Ok((remaining, value)) = snarkvm::prelude::Value::<N>::parse(content) {
                content = remaining;
                values.push(value);
            }
            // Check that the remaining content is empty.
            if !content.trim().is_empty() {
                return Err(PackageError::failed_to_read_input_file(path.display()).into());
            }
            // Convert the values to strings.
            let mut inputs_from_file = values.into_iter().map(|value| value.to_string()).collect::<Vec<String>>();
            // Add the inputs from the file to the arguments.
            arguments.append(&mut inputs_from_file);
        }
        None => arguments.append(&mut inputs),
    }

    // If the `broadcast` flag is set, then broadcast the transaction.
    if command.broadcast {
        // Get the program name.
        let program_name = match (command.program.clone(), command.local) {
            (Some(name), true) => {
                let local = context.open_manifest::<N>()?.program_id().to_string();
                // Throw error if local name doesn't match the specified name.
                if name == local {
                    local
                } else {
                    return Err(PackageError::conflicting_on_chain_program_name(local, name).into());
                }
            }
            (Some(name), false) => name.clone(),
            (None, true) => context.open_manifest::<N>()?.program_id().to_string(),
            (None, false) => return Err(PackageError::missing_on_chain_program_name().into()),
        };

        // Get the private key.
        let private_key = match command.fee_options.private_key.clone() {
            Some(key) => PrivateKey::from_str(&key)?,
            None => PrivateKey::from_str(
                &dotenv_private_key().map_err(CliError::failed_to_read_environment_private_key)?.to_string(),
            )?,
        };

        // Specify the query
        let query = SnarkVMQuery::<N, BlockMemory<N>>::from(command.compiler_options.endpoint.clone());

        // Initialize an RNG.
        let rng = &mut rand::thread_rng();

        // Initialize the storage.
        let store = ConsensusStore::<N, ConsensusMemory<N>>::open(StorageMode::Production)?;

        // Initialize the VM.
        let vm = VM::from(store)?;

        // Load the main program, and all of its imports.
        let program_id = &ProgramID::<N>::from_str(&format!("{}.aleo", program_name))?;
        // TODO: create a local version too
        load_program_from_network(&command, context.clone(), &mut vm.process().write(), program_id)?;

        let fee_record = if let Some(record) = command.fee_options.record {
            Some(parse_record(&private_key, &record)?)
        } else {
            None
        };

        // Create a new transaction.
        let transaction = vm.execute(
            &private_key,
            (program_id, command.name),
            inputs.iter(),
            fee_record.clone(),
            command.fee_options.priority_fee,
            Some(query),
            rng,
        )?;

        // Check the transaction cost.
        let (total_cost, (storage_cost, finalize_cost)) = if let ExecuteTransaction(_, execution, _) = &transaction {
            execution_cost(&vm.process().read(), execution)?
        } else {
            panic!("All transactions should be of type Execute.")
        };

        // Check if the public balance is sufficient.
        if fee_record.is_none() {
            // Derive the account address.
            let address = Address::<N>::try_from(ViewKey::try_from(&private_key)?)?;
            // Query the public balance of the address on the `account` mapping from `credits.aleo`.
            let mut public_balance = Query {
                endpoint: command.compiler_options.endpoint.clone(),
                network: command.compiler_options.network.clone(),
                command: QueryCommands::Program {
                    command: crate::cli::commands::query::Program {
                        name: "credits".to_string(),
                        mappings: false,
                        mapping_value: Some(vec!["account".to_string(), address.to_string()]),
                    },
                },
            }
            .execute(context.clone())?;
            // Check balance.
            // Remove the last 3 characters since they represent the `u64` suffix.
            public_balance.truncate(public_balance.len() - 3);
            if public_balance.parse::<u64>().unwrap() < total_cost {
                return Err(PackageError::insufficient_balance(public_balance, total_cost).into());
            }
        }

        // Print the cost breakdown.
        if command.fee_options.estimate_fee {
            execution_cost_breakdown(
                &program_name,
                total_cost as f64 / 1_000_000.0,
                storage_cost as f64 / 1_000_000.0,
                finalize_cost as f64 / 1_000_000.0,
            );
            return Ok(());
        }

        println!("✅ Created execution transaction for '{}'", program_id.to_string().bold());

        handle_broadcast(
            &format!(
                "{}/{}/transaction/broadcast",
                command.compiler_options.endpoint, command.compiler_options.network
            ),
            transaction,
            &program_name,
        )?;

        return Ok(());
    }

    // Add the compiler options to the arguments.
    if command.compiler_options.offline {
        arguments.push(String::from("--offline"));
    }

    // Add the endpoint to the arguments.
    arguments.push(String::from("--endpoint"));
    arguments.push(command.compiler_options.endpoint.clone());

    // Open the Leo build/ directory.
    let path = context.dir()?;
    let build_directory = BuildDirectory::open(&path)?;

    // Change the cwd to the Leo build/ directory to compile aleo files.
    std::env::set_current_dir(&build_directory)
        .map_err(|err| PackageError::failed_to_set_cwd(build_directory.display(), err))?;

    // Unset the Leo panic hook.
    let _ = std::panic::take_hook();

    // Call the `execute` command.
    println!();
    let command = SnarkVMExecute::try_parse_from(&arguments).map_err(CliError::failed_to_parse_execute)?;
    let res = command.parse().map_err(CliError::failed_to_execute_execute)?;

    // Log the output of the `execute` command.
    tracing::info!("{}", res);

    Ok(())
}

/// A helper function to recursively load the program and all of its imports into the process. Lifted from snarkOS.
fn load_program_from_network<N: Network>(
    command: &Execute,
    context: Context,
    process: &mut Process<N>,
    program_id: &ProgramID<N>,
) -> Result<()> {
    // Fetch the program.
    let program_src = Query {
        endpoint: command.compiler_options.endpoint.clone(),
        network: command.compiler_options.network.clone(),
        command: QueryCommands::Program {
            command: crate::cli::commands::query::Program {
                name: program_id.to_string(),
                mappings: false,
                mapping_value: None,
            },
        },
    }
    .execute(context.clone())?;
    let program = SnarkVMProgram::<N>::from_str(&program_src).unwrap();

    // Return early if the program is already loaded.
    if process.contains_program(program.id()) {
        return Ok(());
    }

    // Iterate through the program imports.
    for import_program_id in program.imports().keys() {
        // Add the imports to the process if does not exist yet.
        if !process.contains_program(import_program_id) {
            // Recursively load the program and its imports.
            load_program_from_network(command, context.clone(), process, import_program_id)?;
        }
    }

    // Add the program to the process if it does not already exist.
    if !process.contains_program(program.id()) {
        process.add_program(&program)?;
    }

    Ok(())
}

// A helper function to display a cost breakdown of the execution.
fn execution_cost_breakdown(name: &String, total_cost: f64, storage_cost: f64, finalize_cost: f64) {
    println!("✅ Estimated execution cost for '{}' is {} credits.", name.bold(), total_cost);
    // Display the cost breakdown in a table.
    let data = [
        [name, "Cost (credits)", "Cost reduction tips"],
        [
            "Storage",
            &format!("{:.6}", storage_cost),
            "Use fewer nested transition functions and smaller input and output datatypes",
        ],
        [
            "On-chain",
            &format!("{:.6}", finalize_cost),
            "Remove operations that are expensive computationally (Ex: hash functions) or storage-wise (Ex: Mapping insertions)",
        ],
        ["Total", &format!("{:.6}", total_cost), ""],
    ];
    let mut out = Vec::new();
    text_tables::render(&mut out, data).unwrap();
    println!("{}", ::std::str::from_utf8(&out).unwrap());
}
