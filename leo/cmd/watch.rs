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

use crate::{cmd::Cmd, context::Context};

use anyhow::{anyhow, Result};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::{sync::mpsc::channel, time::Duration};
use structopt::StructOpt;

use super::build::Build;
use tracing::span::Span;

const LEO_SOURCE_DIR: &str = "src/";

/// Watch file changes in src/ directory and run Build Command
#[derive(StructOpt, Debug, Default)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Watch {
    /// Set up watch interval
    #[structopt(short, long, default_value = "3")]
    interval: u64,
}

impl Watch {
    pub fn new(interval: u64) -> Watch {
        Watch { interval }
    }
}

impl Cmd for Watch {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Watching")
    }

    fn prelude(&self) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, _ctx: Context, _: Self::Input) -> Result<Self::Output> {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(self.interval)).unwrap();

        watcher.watch(LEO_SOURCE_DIR, RecursiveMode::Recursive).map_err(|e| {
            anyhow!(
                "Unable to watch, check that directory contains Leo.toml file. Error: {}",
                e
            )
        })?;

        tracing::info!("Watching Leo source code");

        loop {
            match rx.recv() {
                // See changes on the write event
                Ok(DebouncedEvent::Write(_write)) => {
                    match Build::new().execute() {
                        Ok(_output) => {
                            tracing::info!("Built successfully");
                        }
                        Err(e) => {
                            // Syntax error
                            tracing::error!("Error {:?}", e);
                        }
                    };
                }
                // Other events
                Ok(_event) => {}

                // Watch error
                Err(e) => {
                    tracing::error!("watch error: {:?}", e)
                    // TODO (howardwu): Add graceful termination.
                }
            }
        }
    }
}
