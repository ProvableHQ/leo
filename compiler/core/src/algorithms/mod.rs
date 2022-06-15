// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::Types;
use leo_span::{sym, Symbol};

use indexmap::IndexSet;

/// A core library circuit
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoreCircuit {
    BHP256,
    BHP512,
    BHP768,
    BHP1024,
    Pedersen64,
    Pedersen128,
    Poseidon2,
    Poseidon4,
    Poseidon8,
}

impl CoreCircuit {
    /// Returns a `CoreCircuit` from the given `Symbol`.
    pub fn from_symbol(symbol: Symbol) -> Option<Self> {
        Some(match symbol {
            sym::bhp256 => Self::BHP256,
            sym::bhp512 => Self::BHP512,
            sym::bhp1024 => Self::BHP1024,
            sym::ped64 => Self::Pedersen64,
            sym::ped128 => Self::Pedersen128,
            sym::psd2 => Self::Poseidon2,
            sym::psd4 => Self::Poseidon4,
            sym::psd8 => Self::Poseidon8,
            _ => return None,
        })
    }
}

// todo (collin): deprecate this code
pub struct Algorithms;

impl Types for Algorithms {
    fn types() -> IndexSet<Symbol> {
        IndexSet::from([Symbol::intern("Poseidon")])
    }
}