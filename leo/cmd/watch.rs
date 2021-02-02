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

use crate::{cmd::Cmd, context::Context};

use anyhow::Error;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::{sync::mpsc::channel, time::Duration};
use structopt::StructOpt;

use super::build::Build;

const LEO_SOURCE_DIR: &str = "src/";

/// Time interval for watching files, in seconds
const INTERVAL: u64 = 3;

/// Add package from Aleo Package Manager
#[derive(StructOpt, Debug, Default)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Watch {}

impl Watch {
    pub fn new() -> Watch {
        Watch {}
    }
}

impl Cmd for Watch {
    type Output = ();

    fn apply(self, ctx: Context) -> Result<Self::Output, Error> {
        // Begin "Watching" context for console logging
        let span = tracing::span!(tracing::Level::INFO, "Watching");
        let _enter = span.enter();

        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(INTERVAL)).unwrap();
        watcher.watch(LEO_SOURCE_DIR, RecursiveMode::Recursive).unwrap();

        tracing::info!("Watching Leo source code");

        loop {
            match rx.recv() {
                // See changes on the write event
                Ok(DebouncedEvent::Write(_write)) => {
                    match Build::new().apply(ctx.clone()) {
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
