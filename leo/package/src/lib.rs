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

#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod build;
pub mod imports;
pub mod inputs;
pub mod outputs;
pub mod package;
pub mod root;
pub mod source;

use leo_errors::{PackageError, Result};

use std::{fs, fs::ReadDir, path::PathBuf};

pub static LEO_FILE_EXTENSION: &str = ".leo";

pub(crate) fn parse_file_paths(directory: ReadDir, file_paths: &mut Vec<PathBuf>) -> Result<()> {
    for file_entry in directory {
        let file_entry = file_entry.map_err(PackageError::failed_to_get_leo_file_entry)?;
        let file_path = file_entry.path();

        // Verify that the entry is structured as a valid file or directory
        if file_path.is_dir() {
            let directory =
                fs::read_dir(&file_path).map_err(|err| PackageError::failed_to_read_file(file_path.display(), err))?;

            parse_file_paths(directory, file_paths)?;
            continue;
        } else {
            // Verify that the file has the Leo file extension
            let file_extension = file_path
                .extension()
                .ok_or_else(|| PackageError::failed_to_get_leo_file_extension(file_path.as_os_str().to_owned()))?;
            if file_extension != LEO_FILE_EXTENSION.trim_start_matches('.') {
                return Err(PackageError::invalid_leo_file_extension(
                    file_path.as_os_str().to_owned(),
                    file_extension.to_owned(),
                )
                .into());
            }

            file_paths.push(file_path);
        }
    }

    Ok(())
}
