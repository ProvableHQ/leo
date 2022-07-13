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

use crate::{
    commands::{Command, ALEO_CLI_COMMAND},
    context::Context,
};
use leo_errors::{CliError, Result};
use leo_package::package::Package;

use aleo::commands::New as AleoNew;

use clap::StructOpt;
use tracing::span::Span;

/// Create new Leo project
#[derive(StructOpt, Debug)]
pub struct New {
    #[structopt(name = "NAME", help = "Set package name")]
    name: String,
}

impl Command for New {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "New")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        tracing::info!("Starting...");

        // Call the `aleo new` command from the Aleo SDK.
        let command =
            AleoNew::try_parse_from(&[ALEO_CLI_COMMAND, &self.name]).map_err(CliError::failed_to_parse_aleo_new)?;
        let result = command.parse().map_err(CliError::failed_to_execute_aleo_new)?;

        // Derive the program directory path.
        let mut path = context.dir()?;
        path.push(&self.name);

        // Initialize the Leo package in the directory created by `aleo new`.
        Package::initialize(&self.name, &path)?;

        // todo: modify the readme file to recommend building with `leo build`.

        // Log the output of the `aleo new` command.
        tracing::info!("{}", result);

        Ok(())
    }
}
