// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use super::build::BuildOptions;
use crate::commands::ALEO_CLI_COMMAND;
use crate::{
    commands::{Build, Command},
    context::Context,
};
use leo_errors::{CliError, PackageError, Result};
use leo_package::build::BuildDirectory;

use aleo::commands::Run as AleoRun;

use clap::StructOpt;
use tracing::span::Span;

/// Build, Prove and Run Leo program with inputs
#[derive(StructOpt, Debug)]
pub struct Run {
    #[structopt(name = "NAME", help = "The name of the program to run.", default_value = "main")]
    name: String,

    #[structopt(long = "skip-key-check", help = "Skip key verification on Setup stage")]
    pub(crate) skip_key_check: bool,

    #[structopt(flatten)]
    pub(crate) compiler_options: BuildOptions,
}

impl Command for Run {
    type Input = <Build as Command>::Output;
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Executing")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        (Build {
            compiler_options: self.compiler_options.clone(),
        })
        .execute(context)
    }

    fn apply(self, context: Context, input: Self::Input) -> Result<Self::Output> {
        // Get the input values.
        let mut inputs = match input {
            (Some(input_ast), circuits) => input_ast.program_inputs(&self.name, circuits),
            _ => Vec::new(),
        };

        // Compose the `aleo run` command.
        let mut arguments = vec![ALEO_CLI_COMMAND.to_string(), self.name];
        arguments.append(&mut inputs);

        // Open the Leo build/ directory
        let path = context.dir()?;
        let build_directory = BuildDirectory::open(&path)?;

        // Change the cwd to the Leo build/ directory to compile aleo files.
        std::env::set_current_dir(&build_directory)
            .map_err(|err| PackageError::failed_to_set_cwd(build_directory.display(), err))?;

        // Call the `aleo run` command from the Aleo SDK.
        if self.compiler_options.offline {
            arguments.push(String::from("--offline"));
        }
        let command = AleoRun::try_parse_from(&arguments).map_err(CliError::failed_to_parse_aleo_run)?;
        let res = command.parse().map_err(CliError::failed_to_execute_aleo_run)?;

        // Log the output of the `aleo run` command.
        tracing::info!("{}", res);

        Ok(())
    }
}
