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

impl From<&String> for NetworkName {
    fn from(network: &String) -> Self {
        match network.to_ascii_lowercase().as_str() {
            "testnet" => NetworkName::TestnetV0,
            "mainnet" => NetworkName::MainnetV0,
            _ => panic!("Invalid network"),
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
