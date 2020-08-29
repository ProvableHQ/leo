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
    errors::{CLIError, NewError},
};
use leo_package::LeoPackage;

use clap::ArgMatches;
use std::{env::current_dir, fs};

#[derive(Debug)]
pub struct NewCommand;

impl CLI for NewCommand {
    type Options = (Option<String>, bool);
    type Output = ();

    const ABOUT: AboutType = "Create a new Leo package in a new directory";
    const ARGUMENTS: &'static [ArgumentType] = &[
        // (name, description, required, index)
        (
            "NAME",
            "Sets the resulting package name, defaults to the directory name",
            true,
            1u64,
        ),
    ];
    const FLAGS: &'static [FlagType] = &[("--lib"), ("--bin")];
    const NAME: NameType = "new";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        let is_lib = arguments.is_present("lib");
        match arguments.value_of("NAME") {
            Some(name) => Ok((Some(name.to_string()), is_lib)),
            None => Ok((None, is_lib)),
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        // Begin "Initializing" context for console logging
        let span = tracing::span!(tracing::Level::INFO, "Initializing");
        let _enter = span.enter();

        let mut path = current_dir()?;

        // Derive the package name
        let package_name = match options.0 {
            Some(name) => name,
            None => path
                .file_stem()
                .ok_or_else(|| NewError::ProjectNameInvalid(path.as_os_str().to_owned()))?
                .to_string_lossy()
                .to_string(),
        };

        // Derive the package directory path
        path.push(&package_name);

        // Verify the package directory path does not exist yet
        if path.exists() {
            return Err(NewError::DirectoryAlreadyExists(path.as_os_str().to_owned()).into());
        }

        // Create the package directory
        fs::create_dir_all(&path)
            .map_err(|error| NewError::CreatingRootDirectory(path.as_os_str().to_owned(), error))?;

        LeoPackage::create(&package_name, options.1, &path)?;

        tracing::info!("Successfully initialized package \"{}\"\n", package_name);

        Ok(())
    }
}
