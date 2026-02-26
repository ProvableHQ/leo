// Copyright (C) 2019-2026 Provable Inc.
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

mod advance;
pub mod logger;
pub mod rest;
mod start;

use super::*;
use crate::cli::{
    Command,
    EnvOptions,
    commands::{Context, Span},
};
use clap::Parser;

#[derive(Parser, Debug)]
pub enum DevnodeCommands {
    #[clap(name = "start", about = "Start the Devnode")]
    Start {
        #[clap(flatten)]
        command: start::Start,
    },
    #[clap(name = "advance", about = "Advance the ledger by a specified number of blocks")]
    Advance {
        #[clap(flatten)]
        command: advance::Advance,
    },
}

/// Command for initializing and creating blocks for a local client node.
#[derive(Parser, Debug)]
pub struct LeoDevnode {
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
    #[clap(subcommand)]
    pub command: DevnodeCommands,
}

impl Command for LeoDevnode {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _context: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        handle_devnode(self, context)
    }
}

// A helper function to handle the Devnode command based on the subcommand provided.
fn handle_devnode(devnode_command: LeoDevnode, context: Context) -> Result<<LeoDevnode as Command>::Output> {
    let private_key = devnode_command.env_override.private_key;
    match devnode_command.command {
        DevnodeCommands::Start { command } => {
            tracing::info!("Starting the Devnode server...");
            command.apply(context, private_key)
        }
        DevnodeCommands::Advance { command } => command.apply(context, ()),
    }
}
