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

use leo_span::Symbol;
use serde::{Deserialize, Serialize};
use snarkvm::prelude::{Locator, Network};
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Location {
    /// The program name. e.g. `credits`.
    /// Note. This does not include the `.aleo` network suffix.
    pub program: Symbol,
    pub name: Symbol,
}

impl Location {
    pub fn new(program: Symbol, name: Symbol) -> Location {
        Location { program, name }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.aleo/{}", self.program, self.name)
    }
}

impl<N: Network> From<Locator<N>> for Location {
    fn from(locator: Locator<N>) -> Self {
        Location {
            program: Symbol::intern(&locator.program_id().name().to_string()),
            name: Symbol::intern(&locator.resource().to_string()),
        }
    }
}
