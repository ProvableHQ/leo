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

use leo_errors::{LeoError, PackageError};

use std::{
    borrow::Cow,
    fs,
    fs::ReadDir,
    path::{Path, PathBuf},
};

use backtrace::Backtrace;
use eyre::eyre;

pub static INPUTS_DIRECTORY_NAME: &str = "inputs/";

pub struct InputsDirectory;

impl InputsDirectory {
    /// Creates a directory at the provided path with the default directory name.
    pub fn create(path: &Path) -> Result<(), LeoError> {
        let mut path = Cow::from(path);
        if path.is_dir() && !path.ends_with(INPUTS_DIRECTORY_NAME) {
            path.to_mut().push(INPUTS_DIRECTORY_NAME);
        }

        fs::create_dir_all(&path)
            .map_err(|e| PackageError::failed_to_create_inputs_directory(eyre!(e), Backtrace::new()).into())
    }

    /// Returns a list of files in the input directory.
    pub fn files(path: &Path) -> Result<Vec<PathBuf>, LeoError> {
        let mut path = path.to_owned();
        path.push(INPUTS_DIRECTORY_NAME);

        // Have to handle error mapping this way because of rust error: https://github.com/rust-lang/rust/issues/42424.
        let directory = match fs::read_dir(&path) {
            Ok(read_dir) => read_dir,
            Err(e) => return Err(PackageError::failed_to_read_inputs_directory(eyre!(e), Backtrace::new()).into()),
        };

        let mut file_paths = Vec::new();
        parse_file_paths(directory, &mut file_paths)?;

        Ok(file_paths)
    }
}

fn parse_file_paths(directory: ReadDir, file_paths: &mut Vec<PathBuf>) -> Result<(), LeoError> {
    for file_entry in directory.into_iter() {
        // Have to handle error mapping this way because of rust error: https://github.com/rust-lang/rust/issues/42424.
        let file_entry = match file_entry {
            Ok(dir_entry) => dir_entry,
            Err(e) => return Err(PackageError::failed_to_get_input_file_entry(eyre!(e), Backtrace::new()).into()),
        };
        let file_path = file_entry.path();

        // Verify that the entry is structured as a valid file or directory
        let file_type = file_entry.file_type().map_err(|e| {
            PackageError::failed_to_get_input_file_type(file_path.as_os_str().to_owned(), eyre!(e), Backtrace::new())
        })?;
        if file_type.is_dir() {
            // Have to handle error mapping this way because of rust error: https://github.com/rust-lang/rust/issues/42424.
            let directory = match fs::read_dir(&file_path) {
                Ok(read_dir) => read_dir,
                Err(e) => return Err(PackageError::failed_to_read_inputs_directory(eyre!(e), Backtrace::new()).into()),
            };

            parse_file_paths(directory, file_paths)?;
            continue;
        } else if !file_type.is_file() {
            return Err(PackageError::invalid_input_file_type(
                file_path.as_os_str().to_owned(),
                file_type,
                Backtrace::new(),
            )
            .into());
        }

        file_paths.push(file_path);
    }

    Ok(())
}
