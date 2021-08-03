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

//! The proof file.

use crate::outputs::OUTPUTS_DIRECTORY_NAME;
use leo_errors::{LeoError, PackageError};

use backtrace::Backtrace;
use eyre::eyre;
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

pub static PROOF_FILE_EXTENSION: &str = ".proof";

#[derive(Deserialize)]
pub struct ProofFile {
    pub package_name: String,
}

impl ProofFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn exists_at(&self, path: &Path) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    /// Reads the proof from the given file path if it exists.
    pub fn read_from(&self, path: &Path) -> Result<String, LeoError> {
        let path = self.setup_file_path(path);

        fs::read_to_string(&path)
            .map_err(|_| PackageError::failed_to_read_proof_file(path.into_owned(), Backtrace::new()).into())
    }

    /// Writes the given proof to a file.
    pub fn write_to(&self, path: &Path, proof: &[u8]) -> Result<(), LeoError> {
        let path = self.setup_file_path(path);

        // Have to handle error mapping this way because of rust error: https://github.com/rust-lang/rust/issues/42424.
        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(e) => return Err(PackageError::io_error_proof_file(eyre!(e), Backtrace::new()).into()),
        };

        // Have to handle error mapping this way because of rust error: https://github.com/rust-lang/rust/issues/42424.
        if let Err(e) = file.write_all(proof) {
            return Err(PackageError::io_error_proof_file(eyre!(e), Backtrace::new()).into());
        };

        tracing::info!("Saving proof... ({:?})", path);

        Ok(())
    }

    /// Removes the proof at the given path if it exists. Returns `true` on success,
    /// `false` if the file doesn't exist, and `Error` if the file system fails during operation.
    pub fn remove(&self, path: &Path) -> Result<bool, LeoError> {
        let path = self.setup_file_path(path);
        if !path.exists() {
            return Ok(false);
        }

        // Have to handle error mapping this way because of rust error: https://github.com/rust-lang/rust/issues/42424.
        match fs::remove_file(&path) {
            Ok(_) => Ok(true),
            Err(_) => Err(PackageError::failed_to_remove_proof_file(path.into_owned(), Backtrace::new()).into()),
        }
    }

    fn setup_file_path<'a>(&self, path: &'a Path) -> Cow<'a, Path> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            if !path.ends_with(OUTPUTS_DIRECTORY_NAME) {
                path.to_mut().push(OUTPUTS_DIRECTORY_NAME);
            }
            path.to_mut()
                .push(format!("{}{}", self.package_name, PROOF_FILE_EXTENSION));
        }
        path
    }
}
