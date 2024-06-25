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

use crate::Dependency;
use leo_errors::PackageError;
use serde::{Deserialize, Serialize};
use std::path::Path;

// Struct representation of program's `program.json` specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    program: String,
    version: String,
    description: String,
    license: String,
    dependencies: Option<Vec<Dependency>>,
}

impl Manifest {
    pub fn new(
        program: &str,
        version: &str,
        description: &str,
        license: &str,
        dependencies: Option<Vec<Dependency>>,
    ) -> Self {
        Self {
            program: program.to_owned(),
            version: version.to_owned(),
            description: description.to_owned(),
            license: license.to_owned(),
            dependencies,
        }
    }

    pub fn default(program: &str) -> Self {
        Self {
            program: format!("{program}.aleo"),
            version: "0.1.0".to_owned(),
            description: "".to_owned(),
            license: "MIT".to_owned(),
            dependencies: None,
        }
    }

    pub fn program(&self) -> &String {
        &self.program
    }

    pub fn version(&self) -> &String {
        &self.version
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn license(&self) -> &String {
        &self.license
    }

    pub fn dependencies(&self) -> &Option<Vec<Dependency>> {
        &self.dependencies
    }

    pub fn write_to_dir(&self, path: &Path) -> Result<(), PackageError> {
        // Serialize the manifest to a JSON string.
        let contents = serde_json::to_string_pretty(&self)
            .map_err(|err| PackageError::failed_to_serialize_manifest_file(path.to_str().unwrap(), err))?;
        // Write the manifest to the file.
        std::fs::write(path.join("program.json"), contents).map_err(PackageError::failed_to_write_manifest)
    }

    pub fn read_from_dir(path: &Path) -> Result<Self, PackageError> {
        // Read the manifest file.
        let contents = std::fs::read_to_string(path.join("program.json"))
            .map_err(|_| PackageError::failed_to_load_package(path.to_str().unwrap()))?;
        // Deserialize the manifest.
        serde_json::from_str(&contents)
            .map_err(|err| PackageError::failed_to_deserialize_manifest_file(path.to_str().unwrap(), err))
    }
}
