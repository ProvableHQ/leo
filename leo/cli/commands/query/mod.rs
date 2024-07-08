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
use snarkvm::prelude::{CanaryV0, MainnetV0, TestnetV0};

mod block;
use block::Block;

pub mod program;
pub use program::Program;

mod state_root;
use state_root::StateRoot;

mod committee;
use committee::Committee;

mod mempool;
use mempool::Mempool;

mod peers;
use peers::Peers;

mod transaction;
use transaction::Transaction;

mod utils;
use utils::*;

use leo_errors::UtilError;
use leo_retriever::{fetch_from_network, verify_valid_program, NetworkName};

///  Query live data from the Aleo network.
#[derive(Parser, Debug)]
pub struct Query {
    #[clap(short, long, global = true, help = "Endpoint to retrieve network state from. Defaults to entry in `.env`.")]
    pub endpoint: Option<String>,
    #[clap(short, long, global = true, help = "Network to use. Defaults to entry in `.env`.")]
    pub(crate) network: Option<String>,
    #[clap(subcommand)]
    pub command: QueryCommands,
}

impl Command for Query {
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
        let network = NetworkName::try_from(context.get_network(&self.network)?)?;
        let endpoint = context.get_endpoint(&self.endpoint)?;
        match network {
            NetworkName::MainnetV0 => handle_query::<MainnetV0>(self, context, &network.to_string(), &endpoint),
            NetworkName::TestnetV0 => handle_query::<TestnetV0>(self, context, &network.to_string(), &endpoint),
            NetworkName::CanaryV0 => handle_query::<CanaryV0>(self, context, &network.to_string(), &endpoint),
        }
    }
}

// A helper function to handle the `query` command.
fn handle_query<N: Network>(
    query: Query,
    context: Context,
    network: &str,
    endpoint: &str,
) -> Result<<Query as Command>::Output> {
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
            if endpoint == "https://api.explorer.aleo.org/v1" {
                tracing::warn!(
                    "⚠️  `leo query mempool` is only valid when using a custom endpoint. Specify one using `--endpoint`."
                );
            }
            (None, command.apply(context, ())?)
        }
        QueryCommands::Peers { command } => {
            if endpoint == "https://api.explorer.aleo.org/v1" {
                tracing::warn!(
                    "⚠️  `leo query peers` is only valid when using a custom endpoint. Specify one using `--endpoint`."
                );
            }
            (None, command.apply(context, ())?)
        }
    };

    // Make GET request to retrieve on-chain state.
    let url = format!("{}/{}/{output}", endpoint, network);
    let result = fetch_from_network(&url)?;
    if !recursive {
        tracing::info!("✅ Successfully retrieved data from '{url}'.\n");
        println!("{}\n", result);
    }

    // Verify that the source file parses into a valid Aleo program.
    if let Some(name) = program {
        verify_valid_program::<N>(&name, &result)?;
    }

    Ok(result)
}

#[derive(Parser, Debug)]
pub enum QueryCommands {
    #[clap(about = "Query block information")]
    Block {
        #[clap(flatten)]
        command: Block,
    },
    #[clap(about = "Query transaction information")]
    Transaction {
        #[clap(flatten)]
        command: Transaction,
    },
    #[clap(about = "Query program source code and live mapping values")]
    Program {
        #[clap(flatten)]
        command: Program,
    },
    #[clap(about = "Query the latest stateroot")]
    Stateroot {
        #[clap(flatten)]
        command: StateRoot,
    },
    #[clap(about = "Query the current committee")]
    Committee {
        #[clap(flatten)]
        command: Committee,
    },
    #[clap(about = "Query transactions and transmissions from the memory pool")]
    Mempool {
        #[clap(flatten)]
        command: Mempool,
    },
    #[clap(about = "Query peer information")]
    Peers {
        #[clap(flatten)]
        command: Peers,
    },
}
