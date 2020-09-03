// Copyright (C) 2019-2020 Aleo Systems Inc.
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
    cli::*,
    cli_types::*,
    commands::BuildCommand,
    errors::{CLIError, RunError},
};
use leo_package::{
    root::Manifest,
    source::{MAIN_FILENAME, SOURCE_DIRECTORY_NAME},
};

use clap::ArgMatches;
use std::{convert::TryFrom, env::current_dir};

#[derive(Debug)]
pub struct DeployCommand;

impl CLI for DeployCommand {
    type Options = ();
    type Output = ();

    const ABOUT: AboutType = "Deploy the current package as a program to the network (*)";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "deploy";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        // Begin "Deploy" context for console logging
        let span = tracing::span!(tracing::Level::INFO, "Deploying");
        let _enter = span.enter();

        let path = current_dir()?;

        match BuildCommand::output(options)? {
            Some((_program, _checksum_differs)) => {
                // Get the package name
                let _package_name = Manifest::try_from(&path)?.get_package_name();

                tracing::error!("Unimplemented - `leo deploy`");

                Ok(())
            }
            None => {
                let mut main_file_path = path.clone();
                main_file_path.push(SOURCE_DIRECTORY_NAME);
                main_file_path.push(MAIN_FILENAME);

                Err(CLIError::RunError(RunError::MainFileDoesNotExist(
                    main_file_path.into_os_string(),
                )))
            }
        }
    }
}
