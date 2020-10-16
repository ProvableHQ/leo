// Copyright (C) 2019-2020 Aleo Systems Inc.
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

//! The serialized circuit output file.

use crate::{errors::CircuitFileError, outputs::OUTPUTS_DIRECTORY_NAME};

use serde::Deserialize;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub static CIRCUIT_FILE_EXTENSION: &str = ".json";

#[derive(Deserialize)]
pub struct CircuitFile {
    pub package_name: String,
}

impl CircuitFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn exists_at(&self, path: &PathBuf) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    /// Reads the serialized circuit from the given file path if it exists.
    pub fn read_from(&self, path: &PathBuf) -> Result<String, CircuitFileError> {
        let path = self.setup_file_path(path);

        Ok(fs::read_to_string(&path).map_err(|_| CircuitFileError::FileReadError(path.clone()))?)
    }

    /// Writes the given serialized circuit to a file.
    pub fn write_to(&self, path: &PathBuf, circuit: String) -> Result<(), CircuitFileError> {
        let path = self.setup_file_path(path);

        let mut file = File::create(&path)?;
        file.write_all(circuit.as_bytes())?;

        Ok(())
    }

    /// Removes the serialized circuit at the given path if it exists. Returns `true` on success,
    /// `false` if the file doesn't exist, and `Error` if the file system fails during operation.
    pub fn remove(&self, path: &PathBuf) -> Result<bool, CircuitFileError> {
        let path = self.setup_file_path(path);
        if !path.exists() {
            return Ok(false);
        }

        fs::remove_file(&path).map_err(|_| CircuitFileError::FileRemovalError(path.clone()))?;
        Ok(true)
    }

    fn setup_file_path(&self, path: &PathBuf) -> PathBuf {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(OUTPUTS_DIRECTORY_NAME) {
                path.push(OUTPUTS_DIRECTORY_NAME);
            }
            path.push(format!("{}{}", self.package_name, CIRCUIT_FILE_EXTENSION));
        }
        path
    }
}
