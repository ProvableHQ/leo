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

use crate::errors::ImportsDirectoryError;

use std::{borrow::Cow, fs, path::Path};

pub static IMPORTS_DIRECTORY_NAME: &str = "imports/";

pub struct ImportsDirectory;

impl ImportsDirectory {
    /// Creates a directory at the provided path with the default directory name.
    pub fn create(path: &Path) -> Result<(), ImportsDirectoryError> {
        let mut path = Cow::from(path);
        if path.is_dir() && !path.ends_with(IMPORTS_DIRECTORY_NAME) {
            path.to_mut().push(IMPORTS_DIRECTORY_NAME);
        }

        fs::create_dir_all(&path).map_err(ImportsDirectoryError::Creating)
    }

    /// Removes the directory at the provided path.
    pub fn remove(path: &Path) -> Result<(), ImportsDirectoryError> {
        let mut path = Cow::from(path);
        if path.is_dir() && !path.ends_with(IMPORTS_DIRECTORY_NAME) {
            path.to_mut().push(IMPORTS_DIRECTORY_NAME);
        }

        if path.exists() {
            fs::remove_dir_all(&path).map_err(ImportsDirectoryError::Removing)?;
        }

        Ok(())
    }

    /// Removes an imported package in the imports directory at the provided path.
    pub fn remove_import(path: &Path, package_name: &str) -> Result<(), ImportsDirectoryError> {
        let mut path = Cow::from(path);
        if path.is_dir() && !path.ends_with(IMPORTS_DIRECTORY_NAME) {
            path.to_mut().push(IMPORTS_DIRECTORY_NAME);
        }

        path.to_mut().push(package_name);

        if !path.exists() || !path.is_dir() {
            return Err(ImportsDirectoryError::ImportDoesNotExist(package_name.into()));
        }

        fs::remove_dir_all(&path).map_err(ImportsDirectoryError::Removing)?;

        Ok(())
    }
}
