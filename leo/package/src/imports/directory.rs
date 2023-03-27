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

use crate::parse_file_paths;
use leo_errors::{PackageError, Result};

use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
};

pub static IMPORTS_DIRECTORY_NAME: &str = "imports/";

pub struct ImportsDirectory;

impl ImportsDirectory {
    /// Creates a directory at the provided path with the default directory name if it does not exist.
    pub fn create(path: &Path) -> Result<PathBuf> {
        let mut path = Cow::from(path);
        if path.is_dir() && !path.ends_with(IMPORTS_DIRECTORY_NAME) {
            path.to_mut().push(IMPORTS_DIRECTORY_NAME);
        }

        if !path.exists() {
            fs::create_dir_all(&path)
                .map_err(|err| PackageError::failed_to_create_directory(IMPORTS_DIRECTORY_NAME, err))?;
        }

        Ok(path.to_path_buf())
    }

    /// Removes the directory at the provided path.
    pub fn remove(path: &Path) -> Result<String> {
        let mut path = Cow::from(path);
        if path.is_dir() && !path.ends_with(IMPORTS_DIRECTORY_NAME) {
            path.to_mut().push(IMPORTS_DIRECTORY_NAME);
        }

        if path.exists() {
            fs::remove_dir_all(&path).map_err(|e| PackageError::failed_to_remove_directory(path.display(), e))?;
        }

        Ok(format!("(in \"{}\")", path.display()))
    }

    /// Returns true if the imports directory does not exist or does not contain files.
    pub fn is_empty(path: &Path) -> Result<bool> {
        let imports_path = path.join(Path::new(IMPORTS_DIRECTORY_NAME));
        if !imports_path.exists() {
            return Ok(true);
        }

        Ok(imports_path
            .read_dir()
            .map_err(|err| PackageError::failed_to_read_file(IMPORTS_DIRECTORY_NAME, err))?
            .next()
            .is_none())
    }

    /// Returns a list of files in the imports directory.
    pub fn files(path: &Path) -> Result<Vec<PathBuf>> {
        let mut path = Cow::from(path);
        if path.is_dir() && !path.ends_with(IMPORTS_DIRECTORY_NAME) {
            path.to_mut().push(IMPORTS_DIRECTORY_NAME);
        }

        let directory = fs::read_dir(&path).map_err(|err| PackageError::failed_to_read_file(path.display(), err))?;
        let mut file_paths = Vec::new();

        parse_file_paths(directory, &mut file_paths)?;

        Ok(file_paths)
    }
}
