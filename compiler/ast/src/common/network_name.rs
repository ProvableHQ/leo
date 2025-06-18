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

use leo_errors::{CliError, LeoError};

use serde::{Deserialize, Serialize};
use snarkvm::prelude::{CanaryV0, MainnetV0, Network, TestnetV0};
use std::{fmt, str::FromStr};

// Retrievable networks for an external program
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum NetworkName {
    #[default]
    #[serde(rename = "testnet")]
    TestnetV0,
    #[serde(rename = "mainnet")]
    MainnetV0,
    #[serde(rename = "canary")]
    CanaryV0,
}

impl NetworkName {
    pub fn id(&self) -> u16 {
        match self {
            NetworkName::TestnetV0 => TestnetV0::ID,
            NetworkName::MainnetV0 => MainnetV0::ID,
            NetworkName::CanaryV0 => CanaryV0::ID,
        }
    }
}

impl FromStr for NetworkName {
    type Err = LeoError;

    fn from_str(s: &str) -> Result<Self, LeoError> {
        match s {
            "testnet" => Ok(NetworkName::TestnetV0),
            "mainnet" => Ok(NetworkName::MainnetV0),
            "canary" => Ok(NetworkName::CanaryV0),
            _ => Err(LeoError::CliError(CliError::invalid_network_name(s))),
        }
    }
}

impl fmt::Display for NetworkName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NetworkName::TestnetV0 => write!(f, "testnet"),
            NetworkName::MainnetV0 => write!(f, "mainnet"),
            NetworkName::CanaryV0 => write!(f, "canary"),
        }
    }
}
