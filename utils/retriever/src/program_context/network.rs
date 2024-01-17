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
#[derive(Debug, Clone, std::cmp::Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Network {
    #[serde(rename = "testnet3")]
    Testnet3,
    #[serde(rename = "mainnet")]
    Mainnet,
}

impl From<&String> for Network {
    fn from(network: &String) -> Self {
        match network.to_ascii_lowercase().as_str() {
            "testnet3" => Network::Testnet3,
            "mainnet" => Network::Mainnet,
            _ => panic!("Invalid network"),
        }
    }
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Network::Testnet3 => write!(f, "testnet3"),
            Network::Mainnet => write!(f, "mainnet"),
        }
    }
}
