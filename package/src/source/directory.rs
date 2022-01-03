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

use leo_errors::{PackageError, Result};

use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
};

use crate::PackageDirectory;

pub static SOURCE_DIRECTORY_NAME: &str = "src/";

pub static SOURCE_FILE_EXTENSION: &str = ".leo";

pub struct SourceDirectory;

impl PackageDirectory for SourceDirectory {
    const NAME: &'static str = "src/";
}

impl SourceDirectory {
    /// Returns a list of files in the source directory.
    pub fn files(path: &Path) -> Result<Vec<PathBuf>> {
        let mut path = Cow::from(path);
        path.to_mut().push(Self::NAME);

        let directory = fs::read_dir(&path).map_err(PackageError::failed_to_read_inputs_directory)?;

        let mut file_paths = Vec::new();
        for file_entry in directory.into_iter() {
            let file_entry = file_entry.map_err(PackageError::failed_to_get_source_file_entry)?;
            let file_path = file_entry.path();

            // Verify that the entry is structured as a valid file
            let file_type = file_entry
                .file_type()
                .map_err(|e| PackageError::failed_to_get_source_file_type(file_path.as_os_str().to_owned(), e))?;
            if !file_type.is_file() {
                return Err(PackageError::invalid_source_file_type(file_path.as_os_str().to_owned(), file_type).into());
            }

            // Verify that the file has the default file extension
            let file_extension = file_path
                .extension()
                .ok_or_else(|| PackageError::failed_to_get_source_file_extension(file_path.as_os_str().to_owned()))?;
            if file_extension != SOURCE_FILE_EXTENSION.trim_start_matches('.') {
                return Err(PackageError::invalid_source_file_extension(
                    file_path.as_os_str().to_owned(),
                    file_extension.to_owned(),
                )
                .into());
            }

            file_paths.push(file_path);
        }

        Ok(file_paths)
    }
}
