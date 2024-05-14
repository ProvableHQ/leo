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

use crate::{Location, NetworkName, ProgramContext};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockFileEntry {
    name: String,
    network: Option<NetworkName>,
    location: Location,
    path: Option<PathBuf>,
    checksum: String,
    dependencies: Vec<String>,
}

impl LockFileEntry {
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl From<&ProgramContext> for LockFileEntry {
    fn from(context: &ProgramContext) -> Self {
        LockFileEntry {
            name: context.full_name().to_string(),
            network: context.network, // Direct access as per instruction
            location: context.location().clone(),
            path: context.full_path.clone(), // Direct access as per instruction
            checksum: context.checksum().to_string(),
            dependencies: context.dependencies().iter().map(|dep| format!("{}.aleo", dep)).collect(),
        }
    }
}
