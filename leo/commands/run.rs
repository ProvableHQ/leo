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
use leo_errors::{CliError, Result};

use aleo::commands::Run as AleoRun;

use clap::StructOpt;
use tracing::span::Span;

/// Build, Prove and Run Leo program with inputs
#[derive(StructOpt, Debug)]
pub struct Run {
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

    fn apply(self, _context: Context, input: Self::Input) -> Result<Self::Output> {
        // Compose the `aleo run` command.
        let mut arguments = vec![ALEO_CLI_COMMAND.to_string(), "main".to_string()];

        // Get the input values.
        let mut values = match input {
            Some(input_ast) => input_ast.values(),
            None => Vec::new(),
        };
        arguments.append(&mut values);

        tracing::info!("Starting...");

        // Call the `aleo run` command from the Aleo SDK.
        let command = AleoRun::try_parse_from(&arguments).map_err(CliError::failed_to_parse_aleo_run)?;
        let res = command.parse().map_err(CliError::failed_to_execute_aleo_run)?;

        // Log the output of the `aleo run` command.
        tracing::info!("{}", res);

        Ok(())
    }
}
