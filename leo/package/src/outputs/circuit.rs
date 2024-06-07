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
use std::{
    borrow::Cow,
    fs::{
        File,
        {self},
    },
    io::Write,
    path::Path,
};

pub static CIRCUIT_FILE_EXTENSION: &str = ".json";

#[derive(Deserialize)]
pub struct CircuitFile {
    pub package_name: String,
}

impl CircuitFile {
    pub fn new(package_name: &str) -> Self {
        Self { package_name: package_name.to_string() }
    }

    pub fn exists_at(&self, path: &Path) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    /// Reads the serialized struct from the given file path if it exists.
    pub fn read_from(&self, path: &Path) -> Result<String> {
        let path = self.setup_file_path(path);

        let string =
            fs::read_to_string(&path).map_err(|_| PackageError::failed_to_read_circuit_file(path.into_owned()))?;
        Ok(string)
    }

    /// Writes the given serialized struct to a file.
    pub fn write_to(&self, path: &Path, circuit: String) -> Result<()> {
        let path = self.setup_file_path(path);
        let mut file = File::create(path).map_err(PackageError::io_error_circuit_file)?;

        file.write_all(circuit.as_bytes()).map_err(PackageError::io_error_circuit_file)?;
        Ok(())
    }

    /// Removes the serialized struct at the given path if it exists. Returns `true` on success,
    /// `false` if the file doesn't exist, and `Error` if the file system fails during operation.
    pub fn remove(&self, path: &Path) -> Result<bool> {
        let path = self.setup_file_path(path);
        if !path.exists() {
            return Ok(false);
        }

        fs::remove_file(&path).map_err(|_| PackageError::failed_to_remove_circuit_file(path))?;
        Ok(true)
    }

    fn setup_file_path<'a>(&self, path: &'a Path) -> Cow<'a, Path> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            if !path.ends_with(OUTPUTS_DIRECTORY_NAME) {
                path.to_mut().push(OUTPUTS_DIRECTORY_NAME);
            }
            path.to_mut().push(format!("{}{CIRCUIT_FILE_EXTENSION}", self.package_name));
        }
        path
    }
}
