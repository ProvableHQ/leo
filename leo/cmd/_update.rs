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
    type Options = Option<(bool, bool, bool)>;
    type Output = ();

    const ABOUT: AboutType = "Update Leo to the latest version";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[
        "[list] -l --list 'List all available versions of Leo'",
        "[quiet] -q --quiet 'Suppress outputs to terminal'",
        "[studio] -s --studio 'For Aleo Studio only'",
    ];
    const NAME: NameType = "update";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[
        // (name, description, options, settings)
        (
            UpdateAutomatic::NAME,
            UpdateAutomatic::ABOUT,
            UpdateAutomatic::ARGUMENTS,
            UpdateAutomatic::FLAGS,
            &UpdateAutomatic::OPTIONS,
            &[
                AppSettings::ColoredHelp,
                AppSettings::DisableHelpSubcommand,
                AppSettings::DisableVersion,
            ],
        ),
    ];

    fn parse(arguments: &clap::ArgMatches) -> Result<Self::Options, crate::errors::CLIError> {
        if let ("automatic", Some(arguments)) = arguments.subcommand() {
            // Run the `automatic` subcommand
            let options = UpdateAutomatic::parse(arguments)?;
            let _output = UpdateAutomatic::output(options)?;
            return Ok(None);
        };

        let show_all_versions = arguments.is_present("list");
        let quiet = arguments.is_present("quiet");
        let studio = arguments.is_present("studio");

        Ok(Some((show_all_versions, quiet, studio)))
    }

    fn output(options: Self::Options) -> Result<Self::Output, crate::errors::CLIError> {
        // Begin "Updating" context for console logging
        let span = tracing::span!(tracing::Level::INFO, "Updating");
        let _enter = span.enter();

        let (show_all_versions, quiet, studio) = match options {
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
            false => {
                let config = Config::read_config()?;

                // If update is run with studio and the automatic update is off, finish quietly
                if studio && !config.update.automatic {
                    return Ok(());
                }

                match Updater::update_to_latest_release(!quiet) {
                    Ok(status) => {
                        if !quiet {
                            if status.uptodate() {
                                tracing::info!("Leo is already on the latest version {}", status.version());
                            } else if status.updated() {
                                tracing::info!("Leo has successfully updated to version {}", status.version());
                            }
                        }
                        return Ok(());
                    }
                    Err(e) => {
                        if !quiet {
                            tracing::error!("Could not update Leo to the latest version");
                            tracing::error!("{}", e);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

//TODO (raychu86) Move this to dedicated file/module
#[derive(Debug)]
pub struct UpdateAutomatic;

impl CLI for UpdateAutomatic {
    // (is_automatic, quiet)
    type Options = (Option<bool>, bool);
    type Output = ();

    const ABOUT: AboutType = "Setting for automatic updates of Leo";
    const ARGUMENTS: &'static [ArgumentType] = &[
        // (name, description, possible_values, required, index)
        (
            "automatic",
            "Enable or disable automatic updates",
            &["true", "false"],
            false,
            1u64,
        ),
    ];
    const FLAGS: &'static [FlagType] = &["[quiet] -q --quiet 'Suppress outputs to terminal'"];
    const NAME: NameType = "automatic";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    fn parse(arguments: &clap::ArgMatches) -> Result<Self::Options, crate::errors::CLIError> {
        let quiet = arguments.is_present("quiet");

        match arguments.value_of("automatic") {
            Some(automatic) => {
                // TODO enforce that the possible values is true or false
                let automatic = match automatic {
                    "true" => Some(true),
                    "false" => Some(false),
                    _ => unreachable!(),
                };

                Ok((automatic, quiet))
            }
            None => Ok((None, quiet)),
        }
    }

    fn output(options: Self::Options) -> Result<Self::Output, crate::errors::CLIError> {
        // Begin "Settings" context for console logging
        let span = tracing::span!(tracing::Level::INFO, "Settings");
        let enter = span.enter();

        // If a boolean value is provided, update the saved
        // `automatic` configuration value to this boolean value.
        if let Some(automatic) = options.0 {
            Config::set_update_automatic(automatic)?;
        }

        // If --quiet is not enabled, log the output.
        if !options.1 {
            // Read the `automatic` value now.
            let automatic = Config::read_config()?.update.automatic;

            // Log the output.
            tracing::debug!("automatic = {}", automatic);
            match automatic {
                true => tracing::info!("Automatic updates are enabled. Leo will update as new versions are released."),
                false => {
                    tracing::info!("Automatic updates are disabled. Leo will not update as new versions are released.")
                }
            };
        }

        Ok(())
    }
}
