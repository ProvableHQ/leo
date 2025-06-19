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

use crate::{Identifier, NetworkName};

use core::fmt;
use leo_span::Symbol;
use serde::{Deserialize, Serialize};
use snarkvm::{
    console::program::ProgramID,
    prelude::{CanaryV0, MainnetV0, Network, Result, TestnetV0},
};
use std::str::FromStr;

/// An identifier for a program that is eventually deployed to the network.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ProgramId {
    /// The name of the program.
    pub name: Identifier,
    /// The network associated with the program.
    pub network: Identifier,
}

impl ProgramId {
    /// Initializes a new `ProgramId` from a string, using a network parameter to validate using snarkVM.
    pub fn from_str_with_network(string: &str, network: NetworkName) -> Result<Self> {
        match network {
            NetworkName::MainnetV0 => ProgramID::<MainnetV0>::from_str(string).map(|id| (&id).into()),
            NetworkName::TestnetV0 => ProgramID::<TestnetV0>::from_str(string).map(|id| (&id).into()),
            NetworkName::CanaryV0 => ProgramID::<CanaryV0>::from_str(string).map(|id| (&id).into()),
        }
    }

    /// Converts the `ProgramId` into an address string.
    pub fn to_address_string(&self, network: NetworkName) -> Result<String> {
        match network {
            NetworkName::MainnetV0 => {
                ProgramID::<MainnetV0>::from_str(&self.to_string())?.to_address().map(|addr| addr.to_string())
            }
            NetworkName::TestnetV0 => {
                ProgramID::<TestnetV0>::from_str(&self.to_string())?.to_address().map(|addr| addr.to_string())
            }
            NetworkName::CanaryV0 => {
                ProgramID::<CanaryV0>::from_str(&self.to_string())?.to_address().map(|addr| addr.to_string())
            }
        }
    }
}

impl fmt::Display for ProgramId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.name, self.network)
    }
}

impl<N: Network> From<&ProgramID<N>> for ProgramId {
    fn from(program: &ProgramID<N>) -> Self {
        Self { name: Identifier::from(program.name()), network: Identifier::from(program.network()) }
    }
}

impl From<Identifier> for ProgramId {
    fn from(name: Identifier) -> Self {
        Self {
            name,
            network: Identifier { name: Symbol::intern("aleo"), span: Default::default(), id: Default::default() },
        }
    }
}
