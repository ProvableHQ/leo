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

use leo_errors::{CliError, LeoError};
use serde::{Deserialize, Serialize};
use std::fmt;

// Retrievable networks for an external program
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum NetworkName {
    #[serde(rename = "testnet")]
    TestnetV0,
    #[serde(rename = "mainnet")]
    MainnetV0,
}

impl TryFrom<&str> for NetworkName {
    type Error = LeoError;

    fn try_from(network: &str) -> Result<Self, LeoError> {
        match network {
            "testnet" => Ok(NetworkName::TestnetV0),
            "mainnet" => Ok(NetworkName::MainnetV0),
            _ => Err(LeoError::CliError(CliError::invalid_network_name(network))),
        }
    }
}

impl fmt::Display for NetworkName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NetworkName::TestnetV0 => write!(f, "testnet"),
            NetworkName::MainnetV0 => write!(f, "mainnet"),
        }
    }
}