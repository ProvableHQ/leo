// Copyright (C) 2019-2026 Provable Inc.
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

use crate::Location;
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Information about a dependency, as represented in the `program.json` manifest.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Dependency {
    /// The name of the program. As this corresponds to what appears in `program.json`,
    /// it should have the ".aleo" suffix.
    pub name: String,
    /// Network, local, or test dependency?
    pub location: Location,
    /// For a local dependency, where is its package? Or, for a test, where is its source file?
    pub path: Option<PathBuf>,
    /// For a network dependency, what is its edition?
    pub edition: Option<u16>,
}

impl Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (on {:?})", self.name, self.location)?;
        if let Some(path) = &self.path {
            write!(f, " (at {})", path.display())?;
        }
        if let Some(edition) = self.edition {
            write!(f, " (edition {edition})")?;
        }
        Ok(())
    }
}
