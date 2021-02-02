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

use leo_ast::Circuit;

/// List of imported core circuit structs.
/// This struct is created from a `CorePackageList`
pub struct CoreCircuitStructList {
    /// [(circuit_name, circuit_struct)]
    symbols: Vec<(String, Circuit)>,
}

impl CoreCircuitStructList {
    pub(crate) fn new() -> Self {
        Self { symbols: vec![] }
    }

    pub(crate) fn push(&mut self, name: String, circuit: Circuit) {
        self.symbols.push((name, circuit))
    }

    pub fn symbols(&self) -> impl Iterator<Item = &(String, Circuit)> {
        self.symbols.iter()
    }
}
