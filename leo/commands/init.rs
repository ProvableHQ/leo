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
use leo_package::{
    inputs::*,
    root::{Gitignore, Manifest, README},
    source::{LibFile, MainFile, SourceDirectory},
};

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

        // Verify the directory exists
        if !path.exists() {
            return Err(InitError::DirectoryDoesNotExist(path.as_os_str().to_owned()).into());
        }

        // Verify a manifest file does not already exist
        if Manifest::exists_at(&path) {
            return Err(InitError::PackageAlreadyExists(path.as_os_str().to_owned()).into());
        }

        // Create the manifest file
        Manifest::new(&package_name).write_to(&path)?;

        // Create the .gitignore file
        Gitignore::new().write_to(&path)?;

        // Create the README.md file
        README::new(&package_name).write_to(&path)?;

        // Create the source directory
        SourceDirectory::create(&path)?;

        // Create a new library or binary file

        if options {
            // Verify the library file does not exist
            if !LibFile::exists_at(&path) {
                // Create the library file in the source directory
                LibFile::new(&package_name).write_to(&path)?;
            }
        } else {
            // Create the input directory
            InputsDirectory::create(&path)?;

            // Verify the input file does not exist
            let input_file = InputFile::new(&package_name);
            if !input_file.exists_at(&path) {
                // Create the input file in the inputs directory
                input_file.write_to(&path)?;
            }

            // Verify the state file does not exist
            let state_file = StateFile::new(&package_name);
            if !state_file.exists_at(&path) {
                // Create the state file in the inputs directory
                state_file.write_to(&path)?;
            }

            // Verify the main file does not exist
            if !MainFile::exists_at(&path) {
                // Create the main file in the source directory
                MainFile::new(&package_name).write_to(&path)?;
            }
        }

        Ok(())
    }
}
