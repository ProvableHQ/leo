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

use leo_errors::{PackageError, Result};

use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
};

pub static BUILD_DIRECTORY_NAME: &str = "build/";

pub struct BuildDirectory;

impl BuildDirectory {
    /// Returns the path to the build directory if it exists.
    pub fn open(path: &Path) -> Result<PathBuf> {
        let mut path = Cow::from(path);
        if path.is_dir() && !path.ends_with(BUILD_DIRECTORY_NAME) {
            path.to_mut().push(BUILD_DIRECTORY_NAME);
        }

        if path.exists() {
            Ok(path.to_path_buf())
        } else {
            Err(PackageError::directory_not_found(BUILD_DIRECTORY_NAME, path.display()).into())
        }
    }

    /// Creates a directory at the provided path with the default directory name.
    pub fn create(path: &Path) -> Result<PathBuf> {
        let mut path = Cow::from(path);
        if path.is_dir() && !path.ends_with(BUILD_DIRECTORY_NAME) {
            path.to_mut().push(BUILD_DIRECTORY_NAME);
        }

        fs::create_dir_all(&path).map_err(|err| PackageError::failed_to_create_directory(BUILD_DIRECTORY_NAME, err))?;
        Ok(path.to_path_buf())
    }

    /// Removes the directory at the provided path.
    pub fn remove(path: &Path) -> Result<String> {
        let mut path = Cow::from(path);
        if path.is_dir() && !path.ends_with(BUILD_DIRECTORY_NAME) {
            path.to_mut().push(BUILD_DIRECTORY_NAME);
        }

        if path.exists() {
            fs::remove_dir_all(&path).map_err(|e| PackageError::failed_to_remove_directory(path.display(), e))?;
        }

        Ok(format!("(in \"{}\")", path.display()))
    }
}
