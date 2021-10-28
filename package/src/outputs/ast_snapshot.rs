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

use crate::PackageFile;

use serde::Deserialize;
use std::fmt;

/// Enum to handle all 3 types of snapshots.
#[derive(Deserialize)]
pub enum Snapshot {
    Initial,
    ImportsResolved,
    TypeInference,
    Canonicalization,
}

impl fmt::Display for Snapshot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Initial => "initial_ast.json",
                Self::ImportsResolved => "imports_resolved_ast.json",
                Self::TypeInference => "type_inferenced_ast.json",
                Self::Canonicalization => "canonicalization_ast.json",
            }
        )
    }
}

pub static AST_SNAPSHOT_FILE_EXTENSION: &str = ".json";

/// Generic Snapshot file wrapper. Each package can have up to 3
/// different snapshots: initial_ast, canonicalization_ast and type_inferenced_ast;
#[derive(Deserialize)]
pub struct SnapshotFile {
    pub package_name: String,
    pub snapshot: Snapshot,
}

impl PackageFile for SnapshotFile {
    type ParentDirectory = super::OutputsDirectory;

    fn template(&self) -> String {
        unimplemented!("Snapshot files don't have templates.");
    }
}

impl std::fmt::Display for SnapshotFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.snapshot)
    }
}

impl SnapshotFile {
    pub fn new(package_name: &str, snapshot: Snapshot) -> Self {
        Self {
            package_name: package_name.to_string(),
            snapshot,
        }
    }
}
