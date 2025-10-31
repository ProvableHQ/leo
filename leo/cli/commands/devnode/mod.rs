// Copyright (C) 2019-2025 Provable Inc.
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

pub mod logger;
pub mod rest;
pub mod start;

#[derive(Parser, Debug)]
pub enum DevnodeCommands{
    #[clap(name = "start", about = "Start the devnode")]
    Start {
        #[clap(flatten)]
        command: start::start_devnode;,
    },
    #[clap(name = "advance", about = "Advance the ledger by a specified number of blocks")]
    Advance {
        #[clap(flatten)]
        command: advance::advance_devnode;,
    },
}

///  Query live data from the Aleo network.
#[derive(Parser, Debug)]
pub struct LeoDevnode {
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
    #[clap(subcommand)]
    pub command: DevnodeCommands,
}

impl Command for LeoDevnode {
    type Input = ();
    type Output = String;

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _context: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        // Parse the network.
        let network: NetworkName = get_network(&self.env_override.network)?;
        let endpoint = get_endpoint(&self.env_override.endpoint)?;
        hanlde_devnode(self, context, network, &endpoint)
    }
}

// A helper function to handle the devnode command based on the subcommand provided.
fn hanlde_devnode(
    devnode_command: LeoDevnode,
    context: Context,
    network: NetworkName,
    endpoint: &str,
) -> Result<<LeoDevnode as Command>::Output> {
    let recursive = context.recursive;
    match devnode_command.command {
        DevnodeCommands::Start { command } => {
            if endpoint != "http://localhost:3030" {
                tracing::warn!(
                    "⚠️  Using a custom endpoint for the devnode: {}. This may lead to unexpected behavior.",
                    endpoint
                );
            }
            tracing::info!("Starting the devnode on network: {} with endpoint: {}", network, endpoint);
            command.apply(context, ())
        }
        DevnodeCommands::Advance { command } => {
            tracing::info!("Advancing the devnode on network: {} with endpoint: {}", network, endpoint);
            command.apply(context, ())
        }
    }
}