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
    cli::helpers::dotenv_private_key,
    prelude::{Network, Parser as SnarkVMParser},
};
use std::collections::HashMap;

use crate::cli::query::QueryCommands;
use dialoguer::{theme::ColorfulTheme, Confirm};
use leo_retriever::NetworkName;
use snarkvm::{
    circuit::{Aleo, AleoCanaryV0, AleoTestnetV0, AleoV0},
    cli::LOCALE,
    ledger::Transaction::Execute as ExecuteTransaction,
    package::Package as SnarkVMPackage,
    prelude::{
        execution_cost,
        query::Query as SnarkVMQuery,
        store::{
            helpers::memory::{BlockMemory, ConsensusMemory},
            ConsensusStore,
        },
        Identifier,
        Locator,
        Process,
        Program as SnarkVMProgram,
        ProgramID,
        Value,
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
    #[clap(long, help = "Disables building of the project before execution.", default_value = "false")]
    pub(crate) no_build: bool,
}

impl Command for Execute {
    type Input = <Build as Command>::Output;
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        // No need to build if we are executing an external program.
        if self.program.is_some() || self.no_build {
            return Ok(());
        }
        (Build { options: self.compiler_options.clone() }).execute(context)
    }

    fn apply(self, context: Context, _input: Self::Input) -> Result<Self::Output> {
        // Parse the network.
        let network = NetworkName::try_from(context.get_network(&self.compiler_options.network)?)?;
        let endpoint = context.get_endpoint(&self.compiler_options.endpoint)?;
        match network {
            NetworkName::MainnetV0 => handle_execute::<AleoV0>(self, context, network, &endpoint),
            NetworkName::TestnetV0 => handle_execute::<AleoTestnetV0>(self, context, network, &endpoint),
            NetworkName::CanaryV0 => handle_execute::<AleoCanaryV0>(self, context, network, &endpoint),
        }
    }
}

// A helper function to handle the `execute` command.
fn handle_execute<A: Aleo>(
    command: Execute,
    context: Context,
    network: NetworkName,
    endpoint: &str,
) -> Result<<Execute as Command>::Output> {
    // If input values are provided, then run the program with those inputs.
    // Otherwise, use the input file.
    let mut inputs = command.inputs.clone();

    // Add the inputs to the arguments.
    if let Some(file) = command.file.clone() {
        // Get the contents from the file.
        let path = context.dir()?.join(file);
        let raw_content =
            std::fs::read_to_string(&path).map_err(|err| PackageError::failed_to_read_file(path.display(), err))?;
        // Parse the values from the file.
        let mut content = raw_content.as_str();
        let mut values = vec![];
        while let Ok((remaining, value)) = snarkvm::prelude::Value::<A::Network>::parse(content) {
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
        inputs.append(&mut inputs_from_file);
    }

    // Initialize an RNG.
    let rng = &mut rand::thread_rng();

    // Get the private key.
    let private_key = match command.fee_options.private_key.clone() {
        Some(key) => PrivateKey::from_str(&key)?,
        None => PrivateKey::from_str(
            &dotenv_private_key().map_err(CliError::failed_to_read_environment_private_key)?.to_string(),
        )?,
    };
    let address = Address::try_from(&private_key)?;

    // If the `broadcast` flag is set, then broadcast the transaction.
    if command.broadcast {
        // Get the program name.
        let program_name = match (command.program.clone(), command.local) {
            (Some(name), true) => {
                let local = context.open_manifest::<A::Network>()?.program_id().name().to_string();
                // Throw error if local name doesn't match the specified name.
                if name == local {
                    local
                } else {
                    return Err(PackageError::conflicting_on_chain_program_name(local, name).into());
                }
            }
            (Some(name), false) => name.clone(),
            (None, true) => context.open_manifest::<A::Network>()?.program_id().name().to_string(),
            (None, false) => return Err(PackageError::missing_on_chain_program_name().into()),
        };

        // Specify the query
        let query = SnarkVMQuery::<A::Network, BlockMemory<A::Network>>::from(endpoint);

        // Initialize the storage.
        let store = ConsensusStore::<A::Network, ConsensusMemory<A::Network>>::open(StorageMode::Production)?;

        // Initialize the VM.
        let vm = VM::from(store)?;

        // Load the main program, and all of its imports.
        let program_id = &ProgramID::<A::Network>::from_str(&format!("{}.aleo", program_name))?;
        load_program_from_network(context.clone(), &mut vm.process().write(), program_id, network, endpoint)?;

        let fee_record = if let Some(record) = command.fee_options.record {
            Some(parse_record(&private_key, &record)?)
        } else {
            None
        };

        // Create a new transaction.
        let transaction = vm.execute(
            &private_key,
            (program_id, command.name.clone()),
            inputs.iter(),
            fee_record.clone(),
            command.fee_options.priority_fee,
            Some(query),
            rng,
        )?;

        // Check the transaction cost.
        let (mut total_cost, (storage_cost, finalize_cost)) = if let ExecuteTransaction(_, execution, _) = &transaction
        {
            execution_cost(&vm.process().read(), execution)?
        } else {
            panic!("All transactions should be of type Execute.")
        };

        // Print the cost breakdown.
        total_cost += command.fee_options.priority_fee;
        execution_cost_breakdown(
            &program_name,
            total_cost as f64 / 1_000_000.0,
            storage_cost as f64 / 1_000_000.0,
            finalize_cost as f64 / 1_000_000.0,
            command.fee_options.priority_fee as f64 / 1_000_000.0,
        )?;

        // Check if the public balance is sufficient.
        if fee_record.is_none() {
            check_balance::<A::Network>(&private_key, endpoint, &network.to_string(), context, total_cost)?;
        }

        // Broadcast the execution transaction.
        if !command.fee_options.dry_run {
            if !command.fee_options.yes {
                let prompt = format!(
                    "Do you want to submit execution of function `{}` on program `{program_name}.aleo` to network {} via endpoint {} using address {}?",
                    &command.name, network, endpoint, address
                );
                // Ask the user for confirmation of the transaction.
                let confirmation =
                    Confirm::with_theme(&ColorfulTheme::default()).with_prompt(prompt).default(false).interact();

                // Check if the user confirmed the transaction.
                if let Ok(confirmation) = confirmation {
                    if !confirmation {
                        println!("✅ Successfully aborted the execution transaction for '{}'\n", program_name.bold());
                        return Ok(());
                    }
                } else {
                    return Err(CliError::confirmation_failed().into());
                }
            }
            println!("✅ Created execution transaction for '{}'\n", program_id.to_string().bold());
            handle_broadcast(&format!("{}/{}/transaction/broadcast", endpoint, network), transaction, &program_name)?;
        } else {
            println!("✅ Successful dry run execution for '{}'\n", program_id.to_string().bold());
        }

        return Ok(());
    }

    // Open the Leo build/ directory.
    let path = context.dir()?.join("build/");

    // Unset the Leo panic hook.
    let _ = std::panic::take_hook();

    // Conduct the execution locally (code lifted from snarkVM).
    // Load the package.
    let package = SnarkVMPackage::open(&path)?;
    // Convert the inputs.
    let mut parsed_inputs: Vec<Value<A::Network>> = Vec::new();
    for input in inputs.iter() {
        let value = Value::from_str(input)?;
        parsed_inputs.push(value);
    }
    // Execute the request.
    let (response, execution, metrics) = package
        .execute::<A, _>(
            endpoint.to_string(),
            &private_key,
            Identifier::try_from(command.name.clone())?,
            &parsed_inputs,
            rng,
        )
        .map_err(PackageError::execution_error)?;

    let fee = None;

    // Construct the transaction.
    let transaction = Transaction::from_execution(execution, fee)?;

    // Log the metrics.
    use num_format::ToFormattedString;

    // Count the number of times a function is called.
    let mut program_frequency = HashMap::<String, usize>::new();
    for metric in metrics.iter() {
        // Prepare the function name string.
        let function_name_string = format!("'{}/{}'", metric.program_id, metric.function_name).bold();

        // Prepare the function constraints string
        let function_constraints_string = format!(
            "{function_name_string} - {} constraints",
            metric.num_function_constraints.to_formatted_string(LOCALE)
        );

        // Increment the counter for the function call.
        match program_frequency.get_mut(&function_constraints_string) {
            Some(counter) => *counter += 1,
            None => {
                let _ = program_frequency.insert(function_constraints_string, 1);
            }
        }
    }

    println!("⛓  Constraints\n");
    for (function_constraints, counter) in program_frequency {
        // Log the constraints
        let counter_string = match counter {
            1 => "(called 1 time)".to_string().dimmed(),
            counter => format!("(called {counter} times)").dimmed(),
        };

        println!(" •  {function_constraints} {counter_string}",)
    }

    // Log the outputs.
    match response.outputs().len() {
        0 => (),
        1 => println!("\n➡️  Output\n"),
        _ => println!("\n➡️  Outputs\n"),
    };
    for output in response.outputs() {
        println!(" • {output}");
    }
    println!();

    // Print the transaction.
    println!("{transaction}\n");

    // Prepare the locator.
    let locator = Locator::<A::Network>::from_str(&format!("{}/{}", package.program_id(), command.name))?;
    // Prepare the path string.
    let path_string = format!("(in \"{}\")", path.display());

    println!("✅ Executed '{}' {}", locator.to_string().bold(), path_string.dimmed());
    Ok(())
}

/// A helper function to recursively load the program and all of its imports into the process. Lifted from snarkOS.
fn load_program_from_network<N: Network>(
    context: Context,
    process: &mut Process<N>,
    program_id: &ProgramID<N>,
    network: NetworkName,
    endpoint: &str,
) -> Result<()> {
    // Fetch the program.
    let program_src = Query {
        endpoint: Some(endpoint.to_string()),
        network: Some(network.to_string()),
        command: QueryCommands::Program {
            command: query::Program { name: program_id.to_string(), mappings: false, mapping_value: None },
        },
    }
    .execute(Context::new(context.path.clone(), context.home.clone(), true)?)?;
    let program = SnarkVMProgram::<N>::from_str(&program_src)?;

    // Return early if the program is already loaded.
    if process.contains_program(program.id()) {
        return Ok(());
    }

    // Iterate through the program imports.
    for import_program_id in program.imports().keys() {
        // Add the imports to the process if does not exist yet.
        if !process.contains_program(import_program_id) {
            // Recursively load the program and its imports.
            load_program_from_network(context.clone(), process, import_program_id, network, endpoint)?;
        }
    }

    // Add the program to the process if it does not already exist.
    if !process.contains_program(program.id()) {
        process.add_program(&program)?;
    }

    Ok(())
}

// A helper function to display a cost breakdown of the execution.
fn execution_cost_breakdown(
    name: &String,
    total_cost: f64,
    storage_cost: f64,
    finalize_cost: f64,
    priority_fee: f64,
) -> Result<()> {
    println!("\nBase execution cost for '{}' is {} credits.\n", name.bold(), total_cost);
    // Display the cost breakdown in a table.
    let data = [
        [name, "Cost (credits)"],
        ["Transaction Storage", &format!("{:.6}", storage_cost)],
        ["On-chain Execution", &format!("{:.6}", finalize_cost)],
        ["Priority Fee", &format!("{:.6}", priority_fee)],
        ["Total", &format!("{:.6}", total_cost)],
    ];
    let mut out = Vec::new();
    text_tables::render(&mut out, data).map_err(CliError::table_render_failed)?;
    println!("{}", std::str::from_utf8(&out).map_err(CliError::table_render_failed)?);
    Ok(())
}
