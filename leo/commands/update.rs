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

use crate::{cli::CLI, cli_types::*, config::Config, updater::Updater};

use clap::AppSettings;

#[derive(Debug)]
pub struct UpdateCommand;

impl CLI for UpdateCommand {
    // (show_all_versions, quiet)
    type Options = Option<(bool, bool)>;
    type Output = ();

    const ABOUT: AboutType = "Update Leo to the latest version";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[("--list"), ("--quiet")];
    const NAME: NameType = "update";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[
        // (name, description, options, settings)
        (
            AutomaticUpdate::NAME,
            AutomaticUpdate::ABOUT,
            AutomaticUpdate::ARGUMENTS,
            AutomaticUpdate::FLAGS,
            &AutomaticUpdate::OPTIONS,
            &[
                AppSettings::ColoredHelp,
                AppSettings::DisableHelpSubcommand,
                AppSettings::DisableVersion,
            ],
        ),
    ];

    fn parse(arguments: &clap::ArgMatches) -> Result<Self::Options, crate::errors::CLIError> {
        match arguments.subcommand() {
            ("automatic", Some(arguments)) => {
                // Run the `automatic` subcommand
                let options = AutomaticUpdate::parse(arguments)?;
                let _output = AutomaticUpdate::output(options)?;
                return Ok(None);
            }
            _ => {}
        };

        let show_all_versions = arguments.is_present("list");
        let quiet = arguments.is_present("quiet");

        Ok(Some((show_all_versions, quiet)))
    }

    fn output(options: Self::Options) -> Result<Self::Output, crate::errors::CLIError> {
        // Begin "Updating" context for console logging
        let span = tracing::span!(tracing::Level::INFO, "Updating");
        let _enter = span.enter();

        let (show_all_versions, quiet) = match options {
            Some(options) => options,
            None => return Ok(()),
        };

        match show_all_versions {
            true => match Updater::show_available_releases() {
                Ok(_) => return Ok(()),
                Err(e) => {
                    tracing::error!("Could not fetch that latest version of Leo");
                    tracing::error!("{}", e);
                }
            },
            false => match Updater::update_to_latest_release(!quiet) {
                Ok(status) => {
                    if status.uptodate() {
                        tracing::info!("Leo is already on the latest version {}", status.version());
                    } else if status.updated() {
                        tracing::info!("Leo has successfully updated to version {}", status.version());
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

#[derive(Debug)]
pub struct AutomaticUpdate;

impl CLI for AutomaticUpdate {
    // (is_automatic, quiet)
    type Options = (Option<bool>, bool);
    type Output = ();

    const ABOUT: AboutType = "Set automatic update configuration";
    const ARGUMENTS: &'static [ArgumentType] = &[
        // (name, description, required, index)
        (
            "AUTOMATIC",
            "Set the automatic update configuration [possible values: true, false]",
            false,
            1u64,
        ),
    ];
    const FLAGS: &'static [FlagType] = &[("--quiet")];
    const NAME: NameType = "automatic";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    fn parse(arguments: &clap::ArgMatches) -> Result<Self::Options, crate::errors::CLIError> {
        let quiet = arguments.is_present("quiet");

        match arguments.value_of("AUTOMATIC") {
            Some(automatic) => {
                // TODO enforce that the possible values is true or false
                let automatic = match automatic {
                    "true" => Some(true),
                    "false" => Some(false),
                    _ => {
                        // TODO (raychu86) fix this log output
                        tracing::info!("Please set the automatic update flag to true or false");
                        None
                    }
                };

                Ok((automatic, quiet))
            }
            None => Ok((None, quiet)),
        }
    }

    fn output(options: Self::Options) -> Result<Self::Output, crate::errors::CLIError> {
        // Begin "Updating" context for console logging
        if let Some(automatic) = options.0 {
            Config::set_update_automatic(automatic)?;

            if !options.1 {
                // TODO (raychu86) fix this log output
                tracing::info!("Leo automatic update configuration set to {}", automatic);
            }
        } else {
            let config = Config::read_config()?;
            tracing::info!("Leo automatic update configuration is {}", config.update.automatic);
        }

        Ok(())
    }
}
