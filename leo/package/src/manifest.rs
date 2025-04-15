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

use crate::*;

use leo_errors::PackageError;

use serde::{Deserialize, Serialize};
use std::path::Path;

pub const MANIFEST_FILENAME: &str = "program.json";

/// Struct representation of program's `program.json` specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub program: String,
    pub version: String,
    pub description: String,
    pub license: String,
    pub dependencies: Option<Vec<Dependency>>,
}

impl Manifest {
    /// Write the manifest to the given `path` as a JSON string.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), PackageError> {
        // Serialize the manifest to a JSON string.
        let mut contents = serde_json::to_string_pretty(&self)
            .map_err(|err| PackageError::failed_to_serialize_manifest_file(path.as_ref().display(), err))?;

        // The seralized string doesn't end in a newline.
        contents.push('\n');

        // Write the manifest to the file.
        std::fs::write(path, contents).map_err(PackageError::failed_to_write_manifest)
    }

    /// Read a Manifest from the given file as a JSON string.
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Self, PackageError> {
        // Read the manifest file.
        let contents = std::fs::read_to_string(&path)
            .map_err(|_| PackageError::failed_to_load_package(path.as_ref().display()))?;
        // Deserialize the manifest.
        serde_json::from_str(&contents)
            .map_err(|err| PackageError::failed_to_deserialize_manifest_file(path.as_ref().display(), err))
    }
}
