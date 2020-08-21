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

use crate::{cli::*, cli_types::*, errors::CLIError};
use clap::ArgMatches;
use leo_package::{
    imports::{ImportsDirectory, IMPORTS_DIRECTORY_NAME},
    outputs::OutputsDirectory,
    root::{Gitignore, Manifest},
    source::SourceDirectory,
};
use std::env::current_dir;

#[derive(Debug)]
pub struct RemoveCommand;

impl CLI for RemoveCommand {
    type Options = Option<String>;
    type Output = ();

    const ABOUT: AboutType = "Uninstall a package from the current package (*)";
    const ARGUMENTS: &'static [ArgumentType] = &[
        // (name, description, required, index)
        ("NAME", "Removes the package from the current directory", true, 1u64),
    ];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "remove";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(match arguments.value_of("NAME") {
            Some(name) => Some(name.to_string()),
            None => unreachable!(),
        })
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        let package_name = options.unwrap();

        // Create path for the package
        let mut path = current_dir()?;
        ImportsDirectory::create(&path)?;
        path.push(IMPORTS_DIRECTORY_NAME);
        path.push(&package_name);

        // Remove all Leo source files
        SourceDirectory::remove_files(&path)?;

        // Remove imports directory
        ImportsDirectory::remove(&path)?;

        // Remove outputs directory
        OutputsDirectory::remove(&path)?;

        // Remove manifest file
        Manifest::remove(&path)?;

        // Remove gitignore file
        Gitignore::remove(&path)?;

        // If the package directory is empty then remove it
        if path.read_dir()?.next().is_none() {
            std::fs::remove_dir(path)?;
            log::info!("Package {} removed successfully", package_name);
        } else {
            log::warn!("Cannot remove package. Package directory contains some foreign files");
        }

        Ok(())
    }
}
