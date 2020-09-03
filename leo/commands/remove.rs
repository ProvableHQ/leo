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
use leo_package::LeoPackage;

use clap::ArgMatches;
use std::env::current_dir;

#[derive(Debug)]
pub struct RemoveCommand;

impl CLI for RemoveCommand {
    type Options = Option<String>;
    type Output = ();

    const ABOUT: AboutType = "Uninstall a package from the current package";
    const ARGUMENTS: &'static [ArgumentType] = &[
        // (name, description, possible_values, required, index)
        (
            "NAME",
            "Removes the package from the current directory",
            &[],
            true,
            1u64,
        ),
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
        // Begin "Removing" context for console logging
        let span = tracing::span!(tracing::Level::INFO, "Removing");
        let _enter = span.enter();

        let path = current_dir()?;

        if let Some(package_name) = options {
            LeoPackage::remove_imported_package(&package_name, &path)?;
            tracing::info!("Successfully removed package \"{}\"\n", package_name);
        }

        Ok(())
    }
}
