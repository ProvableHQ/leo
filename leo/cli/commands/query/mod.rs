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

use leo_ast::NetworkName;
use leo_errors::UtilError;
use leo_package::{fetch_from_network, verify_valid_program};

use super::*;

mod block;
pub use block::LeoBlock;

mod program;
pub use program::LeoProgram;

mod state_root;
pub use state_root::StateRoot;

mod committee;
pub use committee::LeoCommittee;

mod mempool;
pub use mempool::LeoMempool;

mod peers;
pub use peers::LeoPeers;

mod transaction;
pub use transaction::LeoTransaction;

mod utils;
use utils::*;

///  Query live data from the Aleo network.
#[derive(Parser, Debug)]
pub struct LeoQuery {
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
    #[clap(subcommand)]
    pub command: QueryCommands,
}

impl Command for LeoQuery {
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
        handle_query(self, context, network, &endpoint)
    }
}

// A helper function to handle the `query` command.
fn handle_query(
    query: LeoQuery,
    context: Context,
    network: NetworkName,
    endpoint: &str,
) -> Result<<LeoQuery as Command>::Output> {
    let recursive = context.recursive;
    let (program, output) = match query.command {
        QueryCommands::Block { command } => (None, command.apply(context, ())?),
        QueryCommands::Transaction { command } => (None, command.apply(context, ())?),
        QueryCommands::Program { command } => {
            // Check if querying for program source code.
            let program =
                if command.mappings || command.mapping_value.is_some() { None } else { Some(command.name.clone()) };
            (program, command.apply(context, ())?)
        }
        QueryCommands::Stateroot { command } => (None, command.apply(context, ())?),
        QueryCommands::Committee { command } => (None, command.apply(context, ())?),
        QueryCommands::Mempool { command } => {
            if endpoint == "https://api.explorer.provable.com/v1" {
                tracing::warn!(
                    "⚠️  `leo query mempool` is only valid when using a custom endpoint. Specify one using `--endpoint`."
                );
            }
            (None, command.apply(context, ())?)
        }
        QueryCommands::Peers { command } => {
            if endpoint == "https://api.explorer.provable.com/v1" {
                tracing::warn!(
                    "⚠️  `leo query peers` is only valid when using a custom endpoint. Specify one using `--endpoint`."
                );
            }
            (None, command.apply(context, ())?)
        }
    };

    // Make GET request to retrieve on-chain state.
    let url = format!("{endpoint}/{network}/{output}");
    let result = fetch_from_network(&url)?;
    if !recursive {
        tracing::info!("✅ Successfully retrieved data from '{url}'.\n");
        println!("{result}\n");
    }

    // Verify that the source file parses into a valid Aleo program.
    if let Some(name) = program {
        verify_valid_program(&name, &result)?;
    }

    Ok(result)
}

#[derive(Parser, Debug)]
pub enum QueryCommands {
    #[clap(about = "Query block information")]
    Block {
        #[clap(flatten)]
        command: LeoBlock,
    },
    #[clap(about = "Query transaction information")]
    Transaction {
        #[clap(flatten)]
        command: LeoTransaction,
    },
    #[clap(about = "Query program source code and live mapping values")]
    Program {
        #[clap(flatten)]
        command: LeoProgram,
    },
    #[clap(about = "Query the latest stateroot")]
    Stateroot {
        #[clap(flatten)]
        command: StateRoot,
    },
    #[clap(about = "Query the current committee")]
    Committee {
        #[clap(flatten)]
        command: LeoCommittee,
    },
    #[clap(about = "Query transactions and transmissions from the memory pool")]
    Mempool {
        #[clap(flatten)]
        command: LeoMempool,
    },
    #[clap(about = "Query peer information")]
    Peers {
        #[clap(flatten)]
        command: LeoPeers,
    },
}
