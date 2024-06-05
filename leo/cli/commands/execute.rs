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
use clap::Parser;
use leo_retriever::NetworkName;
use snarkos_cli::commands::{Developer, Execute as SnarkOSExecute};
use snarkvm::{
    cli::{helpers::dotenv_private_key, Execute as SnarkVMExecute},
    prelude::{MainnetV0, Network, Parser as SnarkVMParser, TestnetV0},
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
    // If the `broadcast` flag is set, then broadcast the transaction.
    if command.broadcast {
        // Get the program name.
        let program_name = match (command.program, command.local) {
            (Some(name), true) => {
                let local = context.open_manifest::<N>()?.program_id().to_string();
                // Throw error if local name doesn't match the specified name.
                if name == local {
                    local
                } else {
                    return Err(PackageError::conflicting_on_chain_program_name(local, name).into());
                }
            }
            (Some(name), false) => name,
            (None, true) => context.open_manifest::<N>()?.program_id().to_string(),
            (None, false) => return Err(PackageError::missing_on_chain_program_name().into()),
        };

        // Get the private key.
        let private_key = match command.fee_options.private_key {
            Some(private_key) => private_key,
            None => dotenv_private_key().map_err(CliError::failed_to_read_environment_private_key)?.to_string(),
        };

        // Set deploy arguments.
        let mut fee_args = vec![
            "snarkos".to_string(),
            "--private-key".to_string(),
            private_key.clone(),
            "--query".to_string(),
            command.compiler_options.endpoint.clone(),
            "--priority-fee".to_string(),
            command.fee_options.priority_fee.to_string(),
            "--network".to_string(),
            N::ID.to_string(),
            "--broadcast".to_string(),
            format!("{}/{}/transaction/broadcast", command.compiler_options.endpoint, command.compiler_options.network)
                .to_string(),
        ];

        // Use record as payment option if it is provided.
        if let Some(record) = command.fee_options.record.clone() {
            fee_args.push("--record".to_string());
            fee_args.push(record);
        };

        // Execute program.
        Developer::Execute(
            SnarkOSExecute::try_parse_from(
                [
                    // The arguments for determining fee.
                    fee_args,
                    // The program ID and function name.
                    vec![program_name, command.name],
                    // The function inputs.
                    command.inputs,
                ]
                .concat(),
            )
            .unwrap(),
        )
        .parse()
        .map_err(CliError::failed_to_execute_deploy)?;

        return Ok(());
    }

    // If input values are provided, then run the program with those inputs.
    // Otherwise, use the input file.
    let mut inputs = command.inputs;

    // Compose the `execute` command.
    let mut arguments = vec![SNARKVM_COMMAND.to_string(), command.name];

    // Add the inputs to the arguments.
    match command.file {
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
