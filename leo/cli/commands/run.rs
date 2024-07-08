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

use leo_retriever::NetworkName;
use snarkvm::{
    cli::Run as SnarkVMRun,
    prelude::{CanaryV0, MainnetV0, Network, Parser as SnarkVMParser, TestnetV0},
};

/// Build, Prove and Run Leo program with inputs
#[derive(Parser, Debug)]
pub struct Run {
    #[clap(name = "NAME", help = "The name of the program to run.", default_value = "main")]
    pub(crate) name: String,

    #[clap(name = "INPUTS", help = "The inputs to the program.")]
    pub(crate) inputs: Vec<String>,

    #[arg(short, long, help = "The inputs to the program, from a file. Overrides the INPUTS argument.")]
    pub(crate) file: Option<String>,

    #[clap(flatten)]
    pub(crate) compiler_options: BuildOptions,
}

impl Command for Run {
    type Input = <Build as Command>::Output;
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        (Build { options: self.compiler_options.clone() }).execute(context)
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        // Parse the network.
        let network = NetworkName::try_from(context.get_network(&self.compiler_options.network)?)?;
        match network {
            NetworkName::MainnetV0 => handle_run::<MainnetV0>(self, context),
            NetworkName::TestnetV0 => handle_run::<TestnetV0>(self, context),
            NetworkName::CanaryV0 => handle_run::<CanaryV0>(self, context),
        }
    }
}

// A helper function to handle the run command.
fn handle_run<N: Network>(command: Run, context: Context) -> Result<<Run as Command>::Output> {
    let mut inputs = command.inputs;

    // Compose the `run` command.
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

    // Open the Leo build/ directory
    let path = context.dir()?;
    let build_directory = BuildDirectory::open(&path)?;

    // Change the cwd to the Leo build/ directory to compile aleo files.
    std::env::set_current_dir(&build_directory)
        .map_err(|err| PackageError::failed_to_set_cwd(build_directory.display(), err))?;

    // Unset the Leo panic hook
    let _ = std::panic::take_hook();

    // Call the `run` command.
    println!();
    let command = SnarkVMRun::try_parse_from(&arguments).map_err(CliError::failed_to_parse_run)?;
    let res = command.parse().map_err(CliError::failed_to_execute_run)?;

    // Log the output of the `run` command.
    tracing::info!("{}", res);

    Ok(())
}
