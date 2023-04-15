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

use leo_span::{sym, Symbol};

/// A core instruction that maps directly to an AVM bytecode instruction.
#[derive(Clone, PartialEq, Eq)]
pub enum CoreFunction {
    BHP256Commit,
    BHP256Hash,
    BHP512Commit,
    BHP512Hash,
    BHP768Commit,
    BHP768Hash,
    BHP1024Commit,
    BHP1024Hash,

    Pedersen64Commit,
    Pedersen64Hash,
    Pedersen128Commit,
    Pedersen128Hash,

    Poseidon2Hash,
    Poseidon4Hash,
    Poseidon8Hash,

    MappingGet,
    MappingGetOrInit,
    MappingSet,
}

impl CoreFunction {
    /// Returns a `CoreFunction` from the given module and method symbols.
    pub fn from_symbols(module: Symbol, function: Symbol) -> Option<Self> {
        Some(match (module, function) {
            (sym::BHP256, sym::commit) => Self::BHP256Commit,
            (sym::BHP256, sym::hash) => Self::BHP256Hash,
            (sym::BHP512, sym::commit) => Self::BHP512Commit,
            (sym::BHP512, sym::hash) => Self::BHP512Hash,
            (sym::BHP768, sym::commit) => Self::BHP768Commit,
            (sym::BHP768, sym::hash) => Self::BHP768Hash,
            (sym::BHP1024, sym::commit) => Self::BHP1024Commit,
            (sym::BHP1024, sym::hash) => Self::BHP1024Hash,

            (sym::Pedersen64, sym::commit) => Self::Pedersen64Commit,
            (sym::Pedersen64, sym::hash) => Self::Pedersen64Hash,
            (sym::Pedersen128, sym::commit) => Self::Pedersen128Commit,
            (sym::Pedersen128, sym::hash) => Self::Pedersen128Hash,

            (sym::Poseidon2, sym::hash) => Self::Poseidon2Hash,
            (sym::Poseidon4, sym::hash) => Self::Poseidon4Hash,
            (sym::Poseidon8, sym::hash) => Self::Poseidon8Hash,

            (sym::Mapping, sym::get) => Self::MappingGet,
            (sym::Mapping, sym::get_or_init) => Self::MappingGetOrInit,
            (sym::Mapping, sym::set) => Self::MappingSet,
            _ => return None,
        })
    }

    /// Returns the number of arguments required by the instruction.
    pub fn num_args(&self) -> usize {
        match self {
            Self::BHP256Commit => 2,
            Self::BHP256Hash => 1,
            Self::BHP512Commit => 2,
            Self::BHP512Hash => 1,
            Self::BHP768Commit => 2,
            Self::BHP768Hash => 1,
            Self::BHP1024Commit => 2,
            Self::BHP1024Hash => 1,

            Self::Pedersen64Commit => 2,
            Self::Pedersen64Hash => 1,
            Self::Pedersen128Commit => 2,
            Self::Pedersen128Hash => 1,

            Self::Poseidon2Hash => 1,
            Self::Poseidon4Hash => 1,
            Self::Poseidon8Hash => 1,

            Self::MappingGet => 2,
            Self::MappingGetOrInit => 3,
            Self::MappingSet => 3,
        }
    }
}
