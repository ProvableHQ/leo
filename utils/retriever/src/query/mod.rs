// Copyright (C) 2019-2024 Aleo Systems Inc.
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

use snarkvm::prelude::{
    Field,
    Network,
    Program,
    ProgramID,
    Result,
    StatePath,
    anyhow,
    bail,
    query::QueryTrait,
    store::{BlockStorage, BlockStore},
};

use async_trait::async_trait;

#[derive(Clone)]
pub enum VMQuery<N: Network, B: BlockStorage<N>> {
    /// The block store from the VM.
    VM(BlockStore<N, B>),
    /// The base URL of the node.
    REST(String),
    // TODO: Static queries
}

impl<N: Network, B: BlockStorage<N>> From<BlockStore<N, B>> for VMQuery<N, B> {
    fn from(block_store: BlockStore<N, B>) -> Self {
        Self::VM(block_store)
    }
}

impl<N: Network, B: BlockStorage<N>> From<&BlockStore<N, B>> for VMQuery<N, B> {
    fn from(block_store: &BlockStore<N, B>) -> Self {
        Self::VM(block_store.clone())
    }
}

impl<N: Network, B: BlockStorage<N>> From<String> for VMQuery<N, B> {
    fn from(url: String) -> Self {
        Self::REST(url)
    }
}

impl<N: Network, B: BlockStorage<N>> From<&String> for VMQuery<N, B> {
    fn from(url: &String) -> Self {
        Self::REST(url.to_string())
    }
}

impl<N: Network, B: BlockStorage<N>> From<&str> for VMQuery<N, B> {
    fn from(url: &str) -> Self {
        Self::REST(url.to_string())
    }
}

#[async_trait(?Send)]
impl<N: Network, B: BlockStorage<N>> QueryTrait<N> for VMQuery<N, B> {
    /// Returns the current state root.
    fn current_state_root(&self) -> Result<N::StateRoot> {
        match self {
            Self::VM(block_store) => Ok(block_store.current_state_root()),
            Self::REST(url) => match N::ID {
                snarkvm::console::network::MainnetV0::ID => {
                    Ok(Self::get_request(&format!("{url}/mainnet/stateRoot/latest"))?.into_json()?)
                }
                snarkvm::console::network::TestnetV0::ID => {
                    Ok(Self::get_request(&format!("{url}/testnet/stateRoot/latest"))?.into_json()?)
                }
                snarkvm::console::network::CanaryV0::ID => {
                    Ok(Self::get_request(&format!("{url}/canary/stateRoot/latest"))?.into_json()?)
                }
                _ => bail!("Unsupported network ID in inclusion query"),
            },
        }
    }

    /// Returns the current state root.
    async fn current_state_root_async(&self) -> Result<N::StateRoot> {
        match self {
            Self::VM(block_store) => Ok(block_store.current_state_root()),
            Self::REST(url) => match N::ID {
                snarkvm::console::network::MainnetV0::ID => {
                    Ok(Self::get_request_async(&format!("{url}/mainnet/stateRoot/latest")).await?.json().await?)
                }
                snarkvm::console::network::TestnetV0::ID => {
                    Ok(Self::get_request_async(&format!("{url}/testnet/stateRoot/latest")).await?.json().await?)
                }
                snarkvm::console::network::CanaryV0::ID => {
                    Ok(Self::get_request_async(&format!("{url}/canary/stateRoot/latest")).await?.json().await?)
                }
                _ => bail!("Unsupported network ID in inclusion query"),
            },
        }
    }

    /// Returns a state path for the given `commitment`.
    fn get_state_path_for_commitment(&self, commitment: &Field<N>) -> Result<StatePath<N>> {
        match self {
            Self::VM(block_store) => block_store.get_state_path_for_commitment(commitment),
            Self::REST(url) => match N::ID {
                snarkvm::console::network::MainnetV0::ID => {
                    Ok(Self::get_request(&format!("{url}/mainnet/statePath/{commitment}"))?.into_json()?)
                }
                snarkvm::console::network::TestnetV0::ID => {
                    Ok(Self::get_request(&format!("{url}/testnet/statePath/{commitment}"))?.into_json()?)
                }
                snarkvm::console::network::CanaryV0::ID => {
                    Ok(Self::get_request(&format!("{url}/canary/statePath/{commitment}"))?.into_json()?)
                }
                _ => bail!("Unsupported network ID in inclusion query"),
            },
        }
    }

    /// Returns a state path for the given `commitment`.
    async fn get_state_path_for_commitment_async(&self, commitment: &Field<N>) -> Result<StatePath<N>> {
        match self {
            Self::VM(block_store) => block_store.get_state_path_for_commitment(commitment),
            Self::REST(url) => match N::ID {
                snarkvm::console::network::MainnetV0::ID => {
                    Ok(Self::get_request_async(&format!("{url}/mainnet/statePath/{commitment}")).await?.json().await?)
                }
                snarkvm::console::network::TestnetV0::ID => {
                    Ok(Self::get_request_async(&format!("{url}/testnet/statePath/{commitment}")).await?.json().await?)
                }
                snarkvm::console::network::CanaryV0::ID => {
                    Ok(Self::get_request_async(&format!("{url}/canary/statePath/{commitment}")).await?.json().await?)
                }
                _ => bail!("Unsupported network ID in inclusion query"),
            },
        }
    }

    /// Returns the current block height.
    fn current_block_height(&self) -> Result<u32> {
        match self {
            Self::VM(block_store) => Ok(block_store.max_height().unwrap_or_default()),
            Self::REST(url) => match N::ID {
                snarkvm::console::network::MainnetV0::ID => {
                    Ok(Self::get_request(&format!("{url}/mainnet/block/height/latest"))?.into_json()?)
                }
                snarkvm::console::network::TestnetV0::ID => {
                    Ok(Self::get_request(&format!("{url}/testnet/block/height/latest"))?.into_json()?)
                }
                snarkvm::console::network::CanaryV0::ID => {
                    Ok(Self::get_request(&format!("{url}/canary/block/height/latest"))?.into_json()?)
                }
                _ => bail!("Unsupported network ID in inclusion query"),
            },
        }
    }

    /// Returns the current block height.
    async fn current_block_height_async(&self) -> Result<u32> {
        match self {
            Self::VM(block_store) => Ok(block_store.max_height().unwrap_or_default()),
            Self::REST(url) => match N::ID {
                snarkvm::console::network::MainnetV0::ID => {
                    Ok(Self::get_request_async(&format!("{url}/mainnet/block/height/latest")).await?.json().await?)
                }
                snarkvm::console::network::TestnetV0::ID => {
                    Ok(Self::get_request_async(&format!("{url}/testnet/block/height/latest")).await?.json().await?)
                }
                snarkvm::console::network::CanaryV0::ID => {
                    Ok(Self::get_request_async(&format!("{url}/canary/block/height/latest")).await?.json().await?)
                }
                _ => bail!("Unsupported network ID in inclusion query"),
            },
        }
    }
}

impl<N: Network, B: BlockStorage<N>> VMQuery<N, B> {
    /// Returns the program for the given program ID.
    pub fn get_program(&self, program_id: &ProgramID<N>) -> Result<Program<N>> {
        match self {
            Self::VM(block_store) => {
                block_store.get_program(program_id)?.ok_or_else(|| anyhow!("Program {program_id} not found in storage"))
            }
            Self::REST(url) => match N::ID {
                snarkvm::console::network::MainnetV0::ID => {
                    Ok(Self::get_request(&format!("{url}/mainnet/program/{program_id}"))?.into_json()?)
                }
                snarkvm::console::network::TestnetV0::ID => {
                    Ok(Self::get_request(&format!("{url}/testnet/program/{program_id}"))?.into_json()?)
                }
                snarkvm::console::network::CanaryV0::ID => {
                    Ok(Self::get_request(&format!("{url}/canary/program/{program_id}"))?.into_json()?)
                }
                _ => bail!("Unsupported network ID in inclusion query"),
            },
        }
    }

    /// Returns the program for the given program ID.
    pub async fn get_program_async(&self, program_id: &ProgramID<N>) -> Result<Program<N>> {
        match self {
            Self::VM(block_store) => {
                block_store.get_program(program_id)?.ok_or_else(|| anyhow!("Program {program_id} not found in storage"))
            }
            Self::REST(url) => match N::ID {
                snarkvm::console::network::MainnetV0::ID => {
                    Ok(Self::get_request_async(&format!("{url}/mainnet/program/{program_id}")).await?.json().await?)
                }
                snarkvm::console::network::TestnetV0::ID => {
                    Ok(Self::get_request_async(&format!("{url}/testnet/program/{program_id}")).await?.json().await?)
                }
                snarkvm::console::network::CanaryV0::ID => {
                    Ok(Self::get_request_async(&format!("{url}/canary/program/{program_id}")).await?.json().await?)
                }
                _ => bail!("Unsupported network ID in inclusion query"),
            },
        }
    }

    /// Performs a GET request to the given URL.
    fn get_request(url: &str) -> Result<ureq::Response> {
        let response = ureq::get(url).call()?;
        if response.status() == 200 { Ok(response) } else { bail!("Failed to fetch from {url}") }
    }

    /// Performs a GET request to the given URL.
    async fn get_request_async(url: &str) -> Result<reqwest::Response> {
        let response = reqwest::get(url).await?;
        if response.status() == 200 { Ok(response) } else { bail!("Failed to fetch from {url}") }
    }
}
