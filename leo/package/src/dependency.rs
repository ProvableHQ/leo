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

use crate::Location;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Information about a dependency, as represented in the `program.json` manifest.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Dependency {
    /// The name of the program. As this corresponds to what appears in `program.json`,
    /// it should have the ".aleo" suffix.
    pub name: String,
    /// Network or local dependency? Note that this isn't really used, as `network`
    /// and `path` provide us this information.
    pub location: Location,
    /// For a local dependency, where is its package?
    pub path: Option<PathBuf>,
    /// For a network dependency, what is its edition?
    pub edition: Option<u16>,
}

impl Dependency {
    pub fn new(name: String, location: Location, path: Option<PathBuf>, edition: Option<u16>) -> Self {
        Self { name, location, path, edition }
    }
}
