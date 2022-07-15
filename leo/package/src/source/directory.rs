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

use std::fs::ReadDir;
use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
};

pub static SOURCE_DIRECTORY_NAME: &str = "src/";

pub static SOURCE_FILE_EXTENSION: &str = ".leo";

pub struct SourceDirectory;

impl SourceDirectory {
    /// Creates a directory at the provided path with the default directory name.
    pub fn create(path: &Path) -> Result<()> {
        let mut path = Cow::from(path);
        if path.is_dir() && !path.ends_with(SOURCE_DIRECTORY_NAME) {
            path.to_mut().push(SOURCE_DIRECTORY_NAME);
        }

        fs::create_dir_all(&path).map_err(PackageError::failed_to_create_source_directory)?;
        Ok(())
    }

    /// Returns a list of files in the source directory.
    pub fn files(path: &Path) -> Result<Vec<PathBuf>> {
        let mut path = Cow::from(path);
        path.to_mut().push(SOURCE_DIRECTORY_NAME);

        let directory = fs::read_dir(&path).map_err(|err| PackageError::failed_to_read_file(path.display(), err))?;
        let mut file_paths = Vec::new();

        parse_file_paths(directory, &mut file_paths)?;

        println!("{:?}", file_paths);

        Ok(file_paths)
    }
}

fn parse_file_paths(directory: ReadDir, file_paths: &mut Vec<PathBuf>) -> Result<()> {
    for file_entry in directory {
        let file_entry = file_entry.map_err(PackageError::failed_to_get_source_file_entry)?;
        let file_path = file_entry.path();

        // Verify that the entry is structured as a valid file or directory
        if file_path.is_dir() {
            let directory =
                fs::read_dir(&file_path).map_err(|err| PackageError::failed_to_read_file(file_path.display(), err))?;

            parse_file_paths(directory, file_paths)?;
            continue;
        } else {
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
    }

    Ok(())
}
