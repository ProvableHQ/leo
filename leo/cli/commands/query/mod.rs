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

mod block;
use block::Block;

mod program;
use program::Program;

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

///  Query live data from the Aleo network.
#[derive(Parser, Debug)]
pub struct Query {
    #[clap(
        short,
        long,
        global = true,
        help = "Endpoint to retrieve network state from. Defaults to https://api.explorer.aleo.org/v1.",
        default_value = "https://api.explorer.aleo.org/v1"
    )]
    pub endpoint: String,
    #[clap(short, long, global = true, help = "Network to use. Defaults to testnet.", default_value = "testnet")]
    pub(crate) network: String,
    #[clap(subcommand)]
    command: QueryCommands,
}

impl Command for Query {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _context: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        let output = match self.command {
            QueryCommands::Block { command } => command.apply(context, ())?,
            QueryCommands::Transaction { command } => command.apply(context, ())?,
            QueryCommands::Program { command } => command.apply(context, ())?,
            QueryCommands::Stateroot { command } => command.apply(context, ())?,
            QueryCommands::Committee { command } => command.apply(context, ())?,
            QueryCommands::Mempool { command } => {
                if self.endpoint == "https://api.explorer.aleo.org/v1" {
                    tracing::warn!(
                        "⚠️  `leo query mempool` is only valid when using a custom endpoint. Specify one using `--endpoint`."
                    );
                }
                command.apply(context, ())?
            }
            QueryCommands::Peers { command } => {
                if self.endpoint == "https://api.explorer.aleo.org/v1" {
                    tracing::warn!(
                        "⚠️  `leo query peers` is only valid when using a custom endpoint. Specify one using `--endpoint`."
                    );
                }
                command.apply(context, ())?
            }
        };

        // Make GET request to retrieve on-chain state.
        let url = format!("{}/{}/{}", self.endpoint, self.network, output);
        let response = ureq::get(&url.clone())
            .set(&format!("X-Aleo-Leo-{}", env!("CARGO_PKG_VERSION")), "true")
            .call()
            .map_err(|err| UtilError::failed_to_retrieve_from_endpoint(err, Default::default()))?;
        if response.status() == 200 {
            tracing::info!("✅ Successfully retrieved data from '{url}'.\n");
            // Unescape the newlines.
            println!("{}\n", response.into_string().unwrap().replace("\\n", "\n"));
            Ok(())
        } else {
            Err(UtilError::network_error(url, response.status(), Default::default()).into())
        }
    }
}

#[derive(Parser, Debug)]
enum QueryCommands {
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
