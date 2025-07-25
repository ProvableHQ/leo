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

use itertools::Itertools;
use leo_span::Symbol;
use serde::{Deserialize, Serialize};
use snarkvm::prelude::{Locator, Network};
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Location {
    /// The program name. e.g. `credits`.
    /// Note. This does not include the `.aleo` network suffix.
    pub program: Symbol,
    /// The absolute path to the item that this `Location` points to.
    pub path: Vec<Symbol>,
}

impl Location {
    pub fn new(program: Symbol, path: Vec<Symbol>) -> Location {
        Location { program, path }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.aleo/{}", self.program, self.path.iter().format("::"))
    }
}

impl<N: Network> From<Locator<N>> for Location {
    fn from(locator: Locator<N>) -> Self {
        Location {
            program: Symbol::intern(&locator.program_id().name().to_string()),
            path: vec![Symbol::intern(&locator.resource().to_string())],
        }
    }
}
