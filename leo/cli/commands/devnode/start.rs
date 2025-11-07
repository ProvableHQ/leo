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
use std::net::SocketAddr;

use aleo_std_storage::StorageMode;
use snarkvm::{
    ledger::store::helpers::memory::ConsensusMemory,
    prelude::{Block, FromBytes, Ledger, TestnetV0},
};

use crate::cli::commands::devnode::rest::Rest;

// Command for starting the Devnode server.
#[derive(Parser, Debug)]
pub struct Start;

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
        let _ = rt.block_on(async { start_devnode().await });
        Ok(())
    }
}

// This will start a local node that can be used for testing and development purposes.
// The Devnode will run in the background and will be accessible via a REST API.
// The Devnode will be configured to use the local network and will be pre-populated with test accounts and data.
pub(crate) async fn start_devnode() -> Result<(), Box<dyn std::error::Error>> {
    // Start the Devnode server.
    println!("Starting the Devnode server...");
    initialize_terminal_logger(2).expect("Failed to initialize logger");
    let socket_addr: SocketAddr = "127.0.0.1:3030".parse()?;
    let rps = 999999999;
    // Load the genesis block.
    const GENESIS_BYTES: &[u8] = include_bytes!("./rest/genesis_8d710d7e2_40val_snarkos_dev_network.bin");
    let genesis_block: Block<TestnetV0> = Block::from_bytes_le(GENESIS_BYTES)?;
    // Initialize the storage mode.
    let storage_mode = StorageMode::new_test(None);
    // Initialize the ledger.
    let ledger: Ledger<TestnetV0, ConsensusMemory<TestnetV0>> = Ledger::load(genesis_block, storage_mode)?;
    // Start the REST API server.
    Rest::start(socket_addr, rps, ledger).await.expect("Failed to start the REST API server");
    println!("Server running on http://{socket_addr}");
    // Prevent main from exiting.
    std::future::pending::<()>().await;

    Ok(())
}
