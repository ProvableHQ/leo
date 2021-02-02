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

//! The verification key file.

use crate::{errors::VerificationKeyFileError, outputs::OUTPUTS_DIRECTORY_NAME};

use serde::Deserialize;
use std::{
    borrow::Cow,
    fs::{self, File},
    io::Write,
    path::Path,
};

pub static VERIFICATION_KEY_FILE_EXTENSION: &str = ".lvk";

#[derive(Deserialize)]
pub struct VerificationKeyFile {
    pub package_name: String,
}

impl VerificationKeyFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn full_path<'a>(&self, path: &'a Path) -> Cow<'a, Path> {
        self.setup_file_path(path)
    }

    pub fn exists_at(&self, path: &Path) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    /// Reads the verification key from the given file path if it exists.
    pub fn read_from(&self, path: &Path) -> Result<Vec<u8>, VerificationKeyFileError> {
        let path = self.setup_file_path(path);

        fs::read(&path).map_err(|_| VerificationKeyFileError::FileReadError(path.into_owned()))
    }

    /// Writes the given verification key to a file.
    pub fn write_to<'a>(
        &self,
        path: &'a Path,
        verification_key: &[u8],
    ) -> Result<Cow<'a, Path>, VerificationKeyFileError> {
        let path = self.setup_file_path(path);

        let mut file = File::create(&path)?;
        file.write_all(verification_key)?;

        Ok(path)
    }

    /// Removes the verification key at the given path if it exists. Returns `true` on success,
    /// `false` if the file doesn't exist, and `Error` if the file system fails during operation.
    pub fn remove(&self, path: &Path) -> Result<bool, VerificationKeyFileError> {
        let path = self.setup_file_path(path);
        if !path.exists() {
            return Ok(false);
        }

        fs::remove_file(&path).map_err(|_| VerificationKeyFileError::FileRemovalError(path.into_owned()))?;
        Ok(true)
    }

    fn setup_file_path<'a>(&self, path: &'a Path) -> Cow<'a, Path> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            if !path.ends_with(OUTPUTS_DIRECTORY_NAME) {
                path.to_mut().push(OUTPUTS_DIRECTORY_NAME);
            }
            path.to_mut()
                .push(format!("{}{}", self.package_name, VERIFICATION_KEY_FILE_EXTENSION));
        }
        path
    }
}
