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

//! The serialized struct output file.

use crate::outputs::OUTPUTS_DIRECTORY_NAME;
use leo_errors::{PackageError, Result};

use serde::Deserialize;
use std::{borrow::Cow, fmt, fs, path::Path};

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
        write!(f, "{}", match self {
            Self::Initial => "initial_ast",
            Self::ImportsResolved => "imports_resolved_ast",
            Self::TypeInference => "type_inferenced_ast",
            Self::Canonicalization => "canonicalization_ast",
        })
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

impl SnapshotFile {
    pub fn new(package_name: &str, snapshot: Snapshot) -> Self {
        Self { package_name: package_name.to_string(), snapshot }
    }

    pub fn exists_at(&self, path: &Path) -> bool {
        let path = self.snapshot_file_path(path);
        path.exists()
    }

    /// Reads the serialized struct from the given file path if it exists.
    pub fn read_from(&self, path: &Path) -> Result<String> {
        let path = self.snapshot_file_path(path);

        let result =
            fs::read_to_string(&path).map_err(|_| PackageError::failed_to_read_snapshot_file(path.into_owned()))?;

        Ok(result)
    }

    /// Removes the serialized struct at the given path if it exists. Returns `true` on success,
    /// `false` if the file doesn't exist, and `Error` if the file system fails during operation.
    pub fn remove(&self, path: &Path) -> Result<bool> {
        let path = self.snapshot_file_path(path);
        if !path.exists() {
            return Ok(false);
        }

        fs::remove_file(&path).map_err(|_| PackageError::failed_to_remove_snapshot_file(path.into_owned()))?;
        Ok(true)
    }

    fn snapshot_file_path<'a>(&self, path: &'a Path) -> Cow<'a, Path> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            if !path.ends_with(OUTPUTS_DIRECTORY_NAME) {
                path.to_mut().push(OUTPUTS_DIRECTORY_NAME);
            }
            path.to_mut().push(format!("{}{AST_SNAPSHOT_FILE_EXTENSION}", self.snapshot));
        }
        path
    }
}
