// Copyright (C) 2019-2022 Aleo Systems Inc.
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

//! The program JSON file for an aleo file.

use crate::outputs::OUTPUTS_DIRECTORY_NAME;
use leo_errors::{PackageError, Result};

use serde::Deserialize;
use std::{
    borrow::Cow,
    fs::{
        File, {self},
    },
    io::Write,
    path::Path,
};

pub static PROGRAM_JSON_FILE_NAME: &str = "program.json";

#[derive(Deserialize)]
pub struct ProgramJson {
    pub program: String,
    pub version: String,
    pub description: String,
    pub license: String,
}

impl ProgramJson {
    pub fn new(
        program: String,
        version: String,
        description: String,
        license: String,
    ) -> Self {
        Self {
            program,
            version,
            description,
            license,
        }
    }

    /// Writes the given program id to a program json file.
    pub fn write_to(&self, path: &Path) -> Result<()> {
        let path = self.setup_file_path(path);
        let mut file = File::create(&path).map_err(PackageError::io_error_aleo_file)?;

        // Write program json file.
        let aleo_file = format!(
            r#"{{
    "program": "{program}",
    "version": "{version}",
    "description": "{description}",
    "license": "{license}"
}}"#,
            program = self.program,
            version = self.version,
            description = self.description,
            license = self.license,
        );

        file.write_all(aleo_file.as_bytes())
            .map_err(PackageError::io_error_aleo_file)?;
        Ok(())
    }

    /// Removes the program json file at the given path if it exists. Returns `true` on success,
    /// `false` if the file doesn't exist, and `Error` if the file system fails during operation.
    pub fn remove(&self, path: &Path) -> Result<bool> {
        let path = self.setup_file_path(path);
        if !path.exists() {
            return Ok(false);
        }

        fs::remove_file(&path).map_err(|_| PackageError::failed_to_remove_aleo_file(path.into_owned()))?;
        Ok(true)
    }

    fn setup_file_path<'a>(&self, path: &'a Path) -> Cow<'a, Path> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            if !path.ends_with(OUTPUTS_DIRECTORY_NAME) {
                path.to_mut().push(OUTPUTS_DIRECTORY_NAME);
            }
            path.to_mut()
                .push(PROGRAM_JSON_FILE_NAME);
        }
        path
    }
}
