// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::{cmd::Cmd, config::Config, context::Context, updater::Updater};
use anyhow::{anyhow, Result};
use structopt::StructOpt;
use tracing::span::Span;

/// Update Leo to the latest version
#[derive(StructOpt, Debug, Default)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Update {
    /// List all available versions of Leo
    #[structopt(short, long)]
    list: bool,

    /// For Aleo Studio only
    #[structopt(short, long)]
    studio: bool,
}

impl Update {
    pub fn new(list: bool, studio: bool) -> Update {
        Update { list, studio }
    }
}

impl Cmd for Update {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Updating")
    }

    fn prelude(&self) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, _: Context, _: Self::Input) -> Result<Self::Output> {
        // if --list is passed - simply list everything and exit
        if self.list {
            return Updater::show_available_releases().map_err(|e| anyhow!("Could not fetch versions: {}", e));
        }

        let config = Config::read_config()?;
        // If update is run with studio and the automatic update is off, finish quietly
        if self.studio && !config.update.automatic {
            return Ok(());
        }

        match Updater::update_to_latest_release(true) {
            Ok(status) => match (status.uptodate(), status.updated()) {
                (true, _) => tracing::info!("Leo is already on the latest version"),
                (_, true) => tracing::info!("Leo has successfully updated to version {}", status.version()),
                (_, _) => (),
            },
            Err(e) => {
                tracing::error!("Could not update Leo to the latest version");
                tracing::error!("{}", e);
            }
        }

        Ok(())
    }
}

// TODO: maybe move to another module
// #[derive(Debug)]
// pub struct UpdateAutomatic;

// impl CLI for UpdateAutomatic {
//     // (is_automatic, quiet)
//     type Options = (Option<bool>, bool);
//     type Output = ();

//     const ABOUT: AboutType = "Setting for automatic updates of Leo";
//     const ARGUMENTS: &'static [ArgumentType] = &[
//         // (name, description, possible_values, required, index)
//         (
//             "automatic",
//             "Enable or disable automatic updates",
//             &["true", "false"],
//             false,
//             1u64,
//         ),
//     ];
//     const FLAGS: &'static [FlagType] = &["[quiet] -q --quiet 'Suppress outputs to terminal'"];
//     const NAME: NameType = "automatic";
//     const OPTIONS: &'static [OptionType] = &[];
//     const SUBCOMMANDS: &'static [SubCommandType] = &[];

//     fn parse(arguments: &clap::ArgMatches) -> Result<Self::Options, crate::errors::CLIError> {
//         let quiet = arguments.is_present("quiet");

//         match arguments.value_of("automatic") {
//             Some(automatic) => {
//                 // TODO enforce that the possible values is true or false
//                 let automatic = match automatic {
//                     "true" => Some(true),
//                     "false" => Some(false),
//                     _ => unreachable!(),
//                 };

//                 Ok((automatic, quiet))
//             }
//             None => Ok((None, quiet)),
//         }
//     }

//     fn output(options: Self::Options) -> Result<Self::Output, crate::errors::CLIError> {
//         // Begin "Settings" context for console logging
//         let span = tracing::span!(tracing::Level::INFO, "Settings");
//         let enter = span.enter();

//         // If a boolean value is provided, update the saved
//         // `automatic` configuration value to this boolean value.
//         if let Some(automatic) = options.0 {
//             Config::set_update_automatic(automatic)?;
//         }

//         // If --quiet is not enabled, log the output.
//         if !options.1 {
//             // Read the `automatic` value now.
//             let automatic = Config::read_config()?.update.automatic;

//             // Log the output.
//             tracing::debug!("automatic = {}", automatic);
//             match automatic {
//                 true => tracing::info!("Automatic updates are enabled. Leo will update as new versions are released."),
//                 false => {
//                     tracing::info!("Automatic updates are disabled. Leo will not update as new versions are released.")
//                 }
//             };
//         }

//         Ok(())
//     }
// }
