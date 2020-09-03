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
    errors::{CLIError, InitError},
};
use leo_package::LeoPackage;

use clap::ArgMatches;
use std::env::current_dir;

#[derive(Debug)]
pub struct InitCommand;

impl CLI for InitCommand {
    type Options = bool;
    type Output = ();

    const ABOUT: AboutType = "Create a new Leo package in an existing directory";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[("--lib"), ("--bin")];
    const NAME: NameType = "init";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(arguments.is_present("lib"))
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        // Begin "Initializing" context for console logging
        let span = tracing::span!(tracing::Level::INFO, "Initializing");
        let _enter = span.enter();

        let path = current_dir()?;

        // Derive the package name
        let package_name = path
            .file_stem()
            .ok_or_else(|| InitError::ProjectNameInvalid(path.as_os_str().to_owned()))?
            .to_string_lossy()
            .to_string();

        // Verify the directory does not exist
        if !path.exists() {
            return Err(InitError::DirectoryDoesNotExist(path.as_os_str().to_owned()).into());
        }

        LeoPackage::initialize(&package_name, options, &path)?;

        tracing::info!("Successfully initialized package \"{}\"\n", package_name);

        Ok(())
    }
}
