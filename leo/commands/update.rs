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

use crate::{commands::Command, config::Config, context::Context, updater::Updater};
use anyhow::{anyhow, Result};
use structopt::StructOpt;
use tracing::span::Span;

/// Setting for automatic updates of Leo
#[derive(Debug, StructOpt, PartialEq)]
pub enum Sub {
    Automatic {
        #[structopt(name = "bool", help = "Boolean value: true or false", parse(try_from_str))]
        value: bool,
    },
}

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

    /// Setting for automatic updates of Leo
    #[structopt(subcommand)]
    automatic: Option<Sub>,
}

impl Update {
    pub fn new(list: bool, studio: bool, automatic: Option<Sub>) -> Update {
        Update {
            list,
            studio,
            automatic,
        }
    }
}

impl Command for Update {
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

        // in case automatic subcommand was called
        if let Some(Sub::Automatic { value }) = self.automatic {
            Config::set_update_automatic(value)?;

            match value {
                true => tracing::info!("Automatic updates are enabled. Leo will update as new versions are released"),
                false => {
                    tracing::info!("Automatic updates are disabled. Leo will not update as new versions are released")
                }
            };

            return Ok(());
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
