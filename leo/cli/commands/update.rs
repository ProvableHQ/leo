// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use super::*;
use crate::cli::helpers::updater::Updater;

/// Update Leo to the latest version
#[derive(Debug, Parser)]
pub struct Update {
    /// Lists all available versions of Leo
    #[clap(short = 'l', long)]
    list: bool,
    /// Suppress outputs to terminal
    #[clap(short = 'q', long)]
    quiet: bool,
}

impl Command for Update {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, _: Context, _: Self::Input) -> Result<Self::Output>
    where
        Self: Sized,
    {
        match self.list {
            true => match Updater::show_available_releases() {
                Ok(output) => tracing::info!("{output}"),
                Err(error) => tracing::info!("Failed to list the available versions of Leo\n{error}\n"),
            },
            false => {
                let result = Updater::update_to_latest_release(!self.quiet);
                if !self.quiet {
                    match result {
                        Ok(status) => {
                            if status.uptodate() {
                                tracing::info!("\nLeo is already on the latest version")
                            } else if status.updated() {
                                tracing::info!("\nLeo has updated to version {}", status.version())
                            }
                        }
                        Err(e) => tracing::info!("\nFailed to update Leo to the latest version\n{e}\n"),
                    }
                }
            }
        }
        Ok(())
    }
}
