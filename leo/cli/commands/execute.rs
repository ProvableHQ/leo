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
// use snarkos_cli::commands::{Developer, Execute as SnarkOSExecute};
use snarkvm::{cli::Execute as SnarkVMExecute, prelude::Parser as SnarkVMParser};

/// Build, Prove and Run Leo program with inputs
#[derive(Parser, Debug)]
pub struct Execute {
    #[clap(name = "NAME", help = "The name of the function to execute.", default_value = "main")]
    name: String,
    #[clap(name = "INPUTS", help = "The inputs to the program. If none are provided, the input file is used.")]
    inputs: Vec<String>,
    #[clap(long, help = "Execute the transition on chain", default_value = "false")]
    broadcast: bool,
    #[clap(long, help = "Custom priority fee in microcredits", default_value = "1000000")]
    priority_fee: String,
    #[clap(long, help = "Custom network", default_value = "testnet3")]
    network: String,
    #[clap(long, help = "Custom private key")]
    private_key: Option<String>,
    #[arg(short, long, help = "The inputs to the program, from a file. Overrides the INPUTS argument.")]
    file: Option<String>,
    #[clap(flatten)]
    compiler_options: BuildOptions,
}

impl Command for Execute {
    type Input = <Build as Command>::Output;
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        (Build { options: self.compiler_options.clone() }).execute(context)
    }

    fn apply(self, context: Context, _input: Self::Input) -> Result<Self::Output> {
        // If the `broadcast` flag is set, then broadcast the transaction.
        if self.broadcast {
            // // Get the program name
            // let project_name = context.open_manifest()?.program_id().to_string();
            //
            // // Get the private key
            // let mut private_key = self.private_key;
            // if private_key.is_none() {
            //     private_key =
            //         Some(dotenv_private_key().map_err(CliError::failed_to_read_environment_private_key)?.to_string());
            // }
            //
            // // Execute program
            // Developer::Execute(
            //     SnarkOSExecute::try_parse_from(
            //         [
            //             vec![
            //                 "snarkos",
            //                 "--private-key",
            //                 private_key.as_ref().unwrap(),
            //                 "--query",
            //                 self.compiler_options.endpoint.as_str(),
            //                 "--priority-fee",
            //                 self.priority_fee.as_str(),
            //                 "--broadcast",
            //                 format!("{}/{}/transaction/broadcast", self.compiler_options.endpoint, self.network)
            //                     .as_str(),
            //                 project_name.as_str(),
            //                 &self.name,
            //             ],
            //             self.inputs.iter().map(|input| input.as_str()).collect(),
            //         ]
            //         .concat(),
            //     )
            //     .unwrap(),
            // )
            // .parse()
            // .map_err(CliError::failed_to_execute_deploy)?;

            return Err(PackageError::unimplemented_command("leo execute --broadcast").into());
        }

        // If input values are provided, then run the program with those inputs.
        // Otherwise, use the input file.
        let mut inputs = self.inputs;

        // Compose the `execute` command.
        let mut arguments = vec![SNARKVM_COMMAND.to_string(), self.name];

        // Add the inputs to the arguments.
        match self.file {
            Some(file) => {
                // Get the contents from the file.
                let path = context.dir()?.join(file);
                let raw_content = std::fs::read_to_string(&path)
                    .map_err(|err| PackageError::failed_to_read_file(path.display(), err))?;
                // Parse the values from the file.
                let mut content = raw_content.as_str();
                let mut values = vec![];
                while let Ok((remaining, value)) = snarkvm::prelude::Value::<CurrentNetwork>::parse(content) {
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
        if self.compiler_options.offline {
            arguments.push(String::from("--offline"));
        }

        // Add the endpoint to the arguments.
        arguments.push(String::from("--endpoint"));
        arguments.push(self.compiler_options.endpoint.clone());

        // Open the Leo build/ directory
        let path = context.dir()?;
        let build_directory = BuildDirectory::open(&path)?;

        // Change the cwd to the Leo build/ directory to compile aleo files.
        std::env::set_current_dir(&build_directory)
            .map_err(|err| PackageError::failed_to_set_cwd(build_directory.display(), err))?;

        // Unset the Leo panic hook
        let _ = std::panic::take_hook();

        // Call the `execute` command.
        println!();
        let command = SnarkVMExecute::try_parse_from(&arguments).map_err(CliError::failed_to_parse_execute)?;
        let res = command.parse().map_err(CliError::failed_to_execute_execute)?;

        // Log the output of the `execute` command.
        tracing::info!("{}", res);

        Ok(())
    }
}
