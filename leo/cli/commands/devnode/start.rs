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

use super::{logger::initialize_terminal_logger, *};
use serde_json::json;
use std::net::SocketAddr;

use aleo_std_storage::StorageMode;
use snarkvm::{
    ledger::store::helpers::memory::ConsensusMemory,
    prelude::{Block, FromBytes, Ledger, TEST_CONSENSUS_VERSION_HEIGHTS, TestnetV0},
};

use crate::cli::commands::devnode::rest::Rest;

// Command for starting the Devnode server.
#[derive(Parser, Debug)]
#[group(id = "start_args")]
pub struct Start {
    /// Verbosity level for logging (0-2).
    #[clap(short = 'v', long, help = "devnode verbosity (0-2)", default_value = "2")]
    pub(crate) verbosity: u8,
    /// Address to bind the Devnode REST API server to.
    #[clap(long, help = "devnode REST API server address", default_value = "127.0.0.1:3030")]
    pub(crate) socket_addr: String,
    /// Path to the genesis block file.
    #[clap(long, help = "path to genesis block file", default_value = "blank")]
    pub(crate) genesis_path: String,
    /// Enable manual block creation mode.
    #[clap(long, help = "disables automatic block creation after broadcast", default_value = "false")]
    pub(crate) manual_block_creation: bool,
}

impl Command for Start {
    /// The private key, resolved from the parent command's `EnvOptions`.
    type Input = Option<String>;
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _context: Context) -> Result<Self::Input> {
        Ok(None)
    }

    fn apply(self, _context: Context, private_key: Self::Input) -> Result<Self::Output> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async { start_devnode(self, private_key).await })
    }
}

// This command initializes a local development node that is pre-populated with test accounts.
async fn start_devnode(command: Start, private_key: Option<String>) -> Result<()> {
    // Initialize the logger.
    println!("Starting the Devnode server...");
    initialize_terminal_logger(command.verbosity).expect("Failed to initialize logger");
    // Parse the listener address.
    let socket_addr: SocketAddr = command
        .socket_addr
        .parse()
        .map_err(|e| CliError::custom(format!("Failed to parse listener address '{}': {}", command.socket_addr, e)))?;
    let rps = 999999999;
    // Load the genesis block.
    let genesis_block: Block<TestnetV0> = if command.genesis_path != "blank" {
        Block::from_bytes_le(&std::fs::read(&command.genesis_path).map_err(|e| {
            CliError::custom(format!("Failed to read genesis block file '{}': {}", command.genesis_path, e))
        })?)?
    } else {
        // This genesis block is stored in $TMPDIR when running snarkos start --dev 0 --dev-num-validators N
        Block::from_bytes_le(include_bytes!("../../../../resources/genesis_8d710d7e2_40val_snarkos_dev_network.bin"))?
    };
    // Initialize the storage mode.
    let storage_mode = StorageMode::new_test(None);
    // Fetch the private key from the command line or an environment variable.
    let private_key = match private_key {
        Some(key) => key,
        None => std::env::var("PRIVATE_KEY").map_err(|e| {
            CliError::custom(format!(
                "
Failed to load `PRIVATE_KEY` from the environment: {e}
Please either:
1. Use the --private-key flag: `leo devnode start --private-key <PRIVATE_KEY>`
2. Set the PRIVATE_KEY environment variable"
            ))
        })?,
    };
    // Initialize the ledger - use spawn_blocking for the blocking load operation.
    let ledger: Ledger<TestnetV0, ConsensusMemory<TestnetV0>> =
        tokio::task::spawn_blocking(move || Ledger::load(genesis_block, storage_mode))
            .await
            .map_err(|e| CliError::custom(format!("Failed to load ledger: {e}")))??;
    // Start the REST API server.
    Rest::start(socket_addr, rps, ledger, command.manual_block_creation, private_key)
        .await
        .expect("Failed to start the REST API server");
    println!("Server running on http://{socket_addr}");

    // Default setting should fast forward to the block corresponding to the latest consensus version.
    // Enabling manual block creation initializes the ledger to the genesis block.
    if !command.manual_block_creation {
        println!("Advancing the Devnode to the latest consensus version");
        let last_height = TEST_CONSENSUS_VERSION_HEIGHTS.last().unwrap().1;
        // Call the REST API to advance the ledger.
        let client = reqwest::Client::new();

        let payload = json!({
            "num_blocks": last_height,
        });

        let _response = client
            .post(format!("http://{}/testnet/block/create", command.socket_addr))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await;
    }
    // Prevent main from exiting.
    std::future::pending::<()>().await;

    Ok(())
}
