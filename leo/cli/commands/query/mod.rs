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

use leo_errors::UtilError;
use leo_package::{NetworkName, fetch_from_network, verify_valid_program};

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
    #[clap(short, long, global = true, help = "Endpoint to retrieve network state from. Defaults to entry in `.env`.")]
    pub endpoint: Option<Url>,
    #[clap(short, long, global = true, help = "Network to use. Defaults to entry in `.env`.")]
    pub(crate) network: Option<String>,
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
        let network: NetworkName = context.get_network(&self.network)?.parse()?;
        let endpoint = context.get_endpoint(&self.endpoint)?;
        handle_query(self, context, network, &endpoint)
    }
}

// A helper function to handle the `query` command.
fn handle_query(
    query: LeoQuery,
    context: Context,
    network: NetworkName,
    endpoint: &Url,
) -> Result<<LeoQuery as Command>::Output> {
    let recursive = context.recursive;

    let provable_api_endpoint = Url::try_from("https://api.explorer.provable.com/v1")
        .map_err(|e| CliError::failed_to_parse_endpoint_as_valid_url(e))?;

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
            if endpoint == &provable_api_endpoint {
                tracing::warn!(
                    "⚠️  `leo query mempool` is only valid when using a custom endpoint. Specify one using `--endpoint`."
                );
            }
            (None, command.apply(context, ())?)
        }
        QueryCommands::Peers { command } => {
            if endpoint == &provable_api_endpoint {
                tracing::warn!(
                    "⚠️  `leo query peers` is only valid when using a custom endpoint. Specify one using `--endpoint`."
                );
            }
            (None, command.apply(context, ())?)
        }
    };

    // Make GET request to retrieve on-chain state.
    // let url = format!("{}/{}/{output}", endpoint, network);
    let mut url = endpoint.to_owned();

    url.path_segments_mut()
        .map_err(|_| CliError::failed_to_parse_endpoint_as_valid_url("cannot be a base"))?
        .extend(&[network.to_string(), output]);

    let result = fetch_from_network(url.as_str())?;
    if !recursive {
        tracing::info!("✅ Successfully retrieved data from '{url}'.\n");
        println!("{}\n", result);
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
