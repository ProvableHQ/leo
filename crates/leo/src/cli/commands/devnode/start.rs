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
use std::{net::SocketAddr, path::PathBuf};

use aleo_std_storage::StorageMode;
use snarkvm::{
    ledger::store::helpers::{memory::ConsensusMemory, rocksdb::ConsensusDB},
    prelude::{
        Block,
        FromBytes,
        Ledger,
        PrivateKey,
        TEST_CONSENSUS_VERSION_HEIGHTS,
        TestnetV0,
        store::ConsensusStorage,
    },
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
    #[clap(short = 'a', long, help = "devnode REST API server address", default_value = "127.0.0.1:3030")]
    pub(crate) socket_addr: String,
    /// Path to the genesis block file.
    #[clap(short = 'g', long, help = "path to genesis block file", default_value = "blank")]
    pub(crate) genesis_path: String,
    /// Enable manual block creation mode.
    #[clap(short = 'm', long, help = "disables automatic block creation after broadcast")]
    pub(crate) manual_block_creation: bool,
    /// Optional flag for persisting the ledger to disk. If not set, the ledger will be stored in memory and will not persist across restarts.
    #[clap(short = 's', long, help = "directory for ledger persistence", num_args = 0..=1, default_missing_value = "devnode")]
    pub(crate) storage: Option<PathBuf>,
    /// If set alongside --storage, clears the ledger directory before starting.
    #[clap(short = 'c', long, help = "Remove existing devnode storage before starting", requires = "storage")]
    pub(crate) clear_storage: bool,
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
    // Load the private key from the command line or environment variable, and start the server.
    let private_key = resolve_private_key(&private_key)?;
    initialize_terminal_logger(command.verbosity).expect("Failed to initialize logger");

    // Parse the listener address.
    let socket_addr: SocketAddr = command
        .socket_addr
        .parse()
        .map_err(|e| CliError::custom(format!("Failed to parse listener address '{}': {}", command.socket_addr, e)))?;
    // Load the genesis block.
    let genesis_block: Block<TestnetV0> = if command.genesis_path != "blank" {
        Block::from_bytes_le(&std::fs::read(&command.genesis_path).map_err(|e| {
            CliError::custom(format!("Failed to read genesis block file '{}': {}", command.genesis_path, e))
        })?)?
    } else {
        // This genesis block is stored in $TMPDIR when running snarkos start --dev 0 --dev-num-validators N
        Block::from_bytes_le(include_bytes!("resources/genesis_8d710d7e2_40val_snarkos_dev_network.bin"))?
    };
    match command.storage {
        Some(path) => {
            if command.clear_storage && path.exists() {
                for entry in std::fs::read_dir(&path)
                    .map_err(|e| CliError::custom(format!("Failed to read ledger directory: {e}")))?
                {
                    let entry = entry.map_err(|e| CliError::custom(format!("Failed to read entry: {e}")))?;
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        std::fs::remove_dir_all(&entry_path).map_err(|e| {
                            CliError::custom(format!("Failed to remove '{}': {e}", entry_path.display()))
                        })?;
                    } else {
                        std::fs::remove_file(&entry_path).map_err(|e| {
                            CliError::custom(format!("Failed to remove '{}': {e}", entry_path.display()))
                        })?;
                    }
                }
                println!("Cleaned ledger directory: {}", path.display());
            }
            println!("Using persistent ledger at: {}", path.display());
            let storage_mode = StorageMode::Custom(path);
            let ledger: Ledger<TestnetV0, ConsensusDB<TestnetV0>> =
                tokio::task::spawn_blocking(move || Ledger::load(genesis_block, storage_mode))
                    .await
                    .map_err(|e| CliError::custom(format!("Failed to load ledger: {e}")))??;
            run_devnode(socket_addr, ledger, command.manual_block_creation, private_key).await?
        }
        None => {
            let storage_mode = StorageMode::new_test(None);
            let ledger: Ledger<TestnetV0, ConsensusMemory<TestnetV0>> =
                tokio::task::spawn_blocking(move || Ledger::load(genesis_block, storage_mode))
                    .await
                    .map_err(|e| CliError::custom(format!("Failed to load ledger: {e}")))??;
            run_devnode(socket_addr, ledger, command.manual_block_creation, private_key).await?
        }
    }

    Ok(())
}

async fn run_devnode<C: 'static + ConsensusStorage<TestnetV0>>(
    socket_addr: SocketAddr,
    ledger: Ledger<TestnetV0, C>,
    manual_block_creation: bool,
    private_key: PrivateKey<TestnetV0>,
) -> Result<()> {
    let rps = 999999999;

    // Record the height before handing the ledger off, so we know how far to advance.
    let current_height = ledger.latest_height();

    Rest::start(socket_addr, rps, ledger, manual_block_creation, private_key)
        .await
        .expect("Failed to start the REST API server");
    println!("Server running on http://{socket_addr}");

    if !manual_block_creation {
        let last_height = TEST_CONSENSUS_VERSION_HEIGHTS.last().unwrap().1;
        let blocks_to_advance = last_height.saturating_sub(current_height);
        if blocks_to_advance > 0 {
            println!("Advancing the Devnode to the latest consensus version");
            let client = reqwest::Client::new();
            let payload = json!({ "num_blocks": blocks_to_advance });
            let _response = client
                .post(format!("http://{}/testnet/block/create", socket_addr))
                .header("Content-Type", "application/json")
                .json(&payload)
                .send()
                .await;
        }
    }

    std::future::pending::<()>().await;
    Ok(())
}

fn resolve_private_key(private_key: &Option<String>) -> Result<PrivateKey<TestnetV0>> {
    match private_key {
        Some(pk) => {
            Ok(PrivateKey::<TestnetV0>::from_str(pk)
                .map_err(|e| CliError::custom(format!("Invalid private key: {e}")))?)
        }
        None => {
            let pk = std::env::var("PRIVATE_KEY").map_err(|e| {
                CliError::custom(format!(
                    "
Failed to load `PRIVATE_KEY` from the environment: {e}
Please either:
1. Use the --private-key flag: `leo devnode start --private-key <PRIVATE_KEY>`
2. Set the PRIVATE_KEY environment variable"
                ))
            })?;
            Ok(PrivateKey::<TestnetV0>::from_str(&pk)
                .map_err(|e| CliError::custom(format!("Invalid private key: {e}")))?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_PRIVATE_KEY: &str = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH";
    const INVALID_PRIVATE_KEY: &str = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWa";

    #[test]
    fn test_blank_private_key_from_flag() {
        let err = resolve_private_key(&Some(String::new())).unwrap_err();
        assert!(err.to_string().contains("Invalid private key"));
    }

    #[test]
    fn test_invalid_private_key_from_flag() {
        let err = resolve_private_key(&Some(INVALID_PRIVATE_KEY.to_string())).unwrap_err();
        assert!(err.to_string().contains("Invalid private key"));
    }

    #[test]
    fn test_valid_private_key_from_flag() {
        assert!(resolve_private_key(&Some(VALID_PRIVATE_KEY.to_string())).is_ok());
    }
}
