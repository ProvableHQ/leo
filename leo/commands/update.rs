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

use crate::{cli::CLI, cli_types::*, updater::Updater};

#[derive(Debug)]
pub struct UpdateCommand;

impl CLI for UpdateCommand {
    type Options = (bool,);
    type Output = ();

    const ABOUT: AboutType = "Update Leo to the latest version";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[("--list")];
    const NAME: NameType = "update";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    fn parse(arguments: &clap::ArgMatches) -> Result<Self::Options, crate::errors::CLIError> {
        let show_all_versions = arguments.is_present("list");
        Ok((show_all_versions,))
    }

    fn output(options: Self::Options) -> Result<Self::Output, crate::errors::CLIError> {
        // Begin "Updating" context for console logging
        let span = tracing::span!(tracing::Level::INFO, "Updating");
        let _enter = span.enter();

        match options {
            (true,) => match Updater::show_available_releases() {
                Ok(_) => return Ok(()),
                Err(e) => {
                    tracing::error!("Could not fetch that latest version of Leo");
                    tracing::error!("{}", e);
                }
            },
            (false,) => match Updater::update_to_latest_release(true) {
                Ok(status) => {
                    if status.uptodate() {
                        tracing::info!("Leo is already on the latest version: {}", status.version());
                    } else if status.updated() {
                        tracing::info!("Leo has successfully updated to version: {}", status.version());
                    }
                    return Ok(());
                }
                Err(e) => {
                    tracing::error!("Could not update Leo to the latest version");
                    tracing::error!("{}", e);
                }
            },
        }
        Ok(())
    }
}
