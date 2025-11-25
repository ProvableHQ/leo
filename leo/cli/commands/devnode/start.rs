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

use super::{logger::initialize_terminal_logger, *};
use serde_json::json;
use std::net::SocketAddr;

use aleo_std_storage::StorageMode;
use snarkvm::{
    ledger::store::helpers::memory::ConsensusMemory,
    prelude::{Block, FromBytes, Ledger, TestnetV0, TEST_CONSENSUS_VERSION_HEIGHTS},
};

use crate::cli::commands::devnode::rest::Rest;

// Command for starting the Devnode server.
#[derive(Parser, Debug)]
pub struct Start {
    /// Verbosity level for logging (0-2).
    #[clap(short = 'v', long, help = "devnode verbosity (0-2)", default_value = "2")]
    pub(crate) verbosity: u8,
    /// Address to bind the Devnode REST API server to.
    #[clap(long, help = "devnode REST API server address", default_value = "127.0.0.1:3030")]
    pub(crate) listener_addr: String,
    /// Path to the genesis block file.
    #[clap(long, help = "path to genesis block file", default_value = "blank")]
    pub(crate) genesis_path: String,
    /// Enable manual block creation mode.
    #[clap(long, help = "disables automatic block creation after broadcast", default_value = "false")]
    pub(crate) manual_block_creation: bool,
    /// Environment override options.
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
}

impl Command for Start {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _context: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, _context: Context, _: Self::Input) -> Result<Self::Output> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _ = rt.block_on(async { start_devnode(self).await });
        Ok(())
    }
}

// This will start a local node that can be used for testing and development purposes.
// The Devnode will run in the background and will be accessible via a REST API.
// The Devnode will be configured to use the local network and will be pre-populated with test accounts and data.
pub(crate) async fn start_devnode(command: Start) -> Result<<Start as Command>::Output> {
    // Start the Devnode server.
    println!("Starting the Devnode server...");
    initialize_terminal_logger(command.verbosity).expect("Failed to initialize logger");
    let socket_addr: SocketAddr = command.listener_addr.parse().map_err(|e| {
        CliError::custom(format!("Failed to parse listener address '{}': {}", command.listener_addr, e))
    })?;
    let rps = 999999999;
    // Load the genesis block.
    let genesis_block: Block<TestnetV0> = if command.genesis_path != "blank" {
        Block::from_bytes_le(&std::fs::read(command.genesis_path.clone()).map_err(|e| {
            CliError::custom(format!("Failed to read genesis block file '{}': {}", command.genesis_path, e))
        })?)?
    } else {
        Block::from_bytes_le(include_bytes!("./rest/genesis_8d710d7e2_40val_snarkos_dev_network.bin"))?
    };
    // Initialize the storage mode.
    let storage_mode = StorageMode::new_test(None);
    // Fetch the private key from the environment or command line.
    let private_key = match command.env_override.private_key {
        Some(key) => key,
        None => std::env::var("PRIVATE_KEY")
            .map_err(|e| CliError::custom(format!("Failed to load `PRIVATE_KEY` from the environment: {e}")))?,
    };
    // Initialize the ledger - use spawn_blocking for the blocking load operation
    let ledger: Ledger<TestnetV0, ConsensusMemory<TestnetV0>> =
        tokio::task::spawn_blocking(move || Ledger::load(genesis_block, storage_mode))
            .await
            .map_err(|e| CliError::custom(format!("Failed to load ledger: {e}")))??;
    // Start the REST API server.
    Rest::start(socket_addr, rps, ledger, command.manual_block_creation, private_key.clone())
        .await
        .expect("Failed to start the REST API server");
    println!("Server running on http://{socket_addr}");

    // Default setting should fast forward to the block corresponding to the latest consensus version.
    // Enabling manual block creation will initialize the ledger to the genesis block.
    if !command.manual_block_creation {
        println!("Advancing the Devnode to the latest consensus version");
        let last_height = TEST_CONSENSUS_VERSION_HEIGHTS.last().unwrap().1;
        // Call the REST API to advance the ledger by one block.
        let client = reqwest::blocking::Client::new();

        let payload = json!({
            "num_blocks": last_height,
        });

        let _response = client
            .post("http://localhost:3030/testnet/block/create")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send();
    }
    // Prevent main from exiting.
    std::future::pending::<()>().await;

    Ok(())
}
