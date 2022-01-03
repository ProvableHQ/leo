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

#![doc = include_str!("../README.md")]

pub mod imports;
pub mod inputs;
pub mod outputs;
pub mod package;
pub mod root;
pub mod source;

use std::path::Path;

use leo_errors::PackageError;
use leo_errors::Result;
use std::borrow::Cow;
use std::fs::{self, File};
use std::io::Write;

pub struct LeoPackage;

impl LeoPackage {
    /// Initializes a Leo package at the given path.
    pub fn initialize(package_name: &str, path: &Path, author: Option<String>) -> Result<()> {
        package::Package::initialize(package_name, path, author)
    }

    /// Returns `true` if the given Leo package name is valid.
    pub fn is_package_name_valid(package_name: &str) -> bool {
        package::Package::is_package_name_valid(package_name)
    }

    /// Removes an imported Leo package
    pub fn remove_imported_package(package_name: &str, path: &Path) -> Result<()> {
        package::Package::remove_imported_package(package_name, path)
    }
}

/// Main trait for Package files, handles basic I/O operations as well as the templates
/// management and file paths. ParentDirectory type should point to a corresponding
/// PackageDirectory.
///
/// Display implementation for each file MUST be the full name of the file including
/// extension.
pub trait PackageFile: std::fmt::Display {
    /// ParentDirectory type for the specific type.
    type ParentDirectory: PackageDirectory;

    /// Returns template for a file if it should be created on package init.
    /// Files that don't have a template should have unimplemented! or panic! dummy implementation.
    fn template(&self) -> String;

    /// Returns file path local to the root folder of the project.
    fn filename(&self) -> String {
        format!("{}{}", Self::ParentDirectory::NAME, self)
    }

    /// Checks whether file exists at given path.
    fn exists_at(&self, path: &Path) -> bool {
        let path = self.file_path(path);
        path.exists()
    }

    /// Reads the serialized circuit from the given file path if it exists.
    fn read_from(&self, path: &Path) -> Result<String> {
        let path = self.file_path(path);

        let result =
            fs::read_to_string(&path).map_err(|err| PackageError::failed_to_read_file(self.to_string(), err))?;

        Ok(result)
    }

    /// Call write_to with template.
    fn write_template(&self, path: &Path) -> Result<()>
    where
        Self: Sized,
    {
        self.write_to(path, self.template().as_bytes())
    }

    /// Write data to the given path.
    fn write_to(&self, path: &Path, data: &[u8]) -> Result<()>
    where
        Self: Sized,
    {
        let path = self.file_path(path);

        let mut file = File::create(&path).map_err(|err| PackageError::io_error(self.to_string(), err))?;
        Ok(file
            .write_all(data)
            .map_err(|err| PackageError::io_error(self.to_string(), err))?)
    }

    /// Removes the serialized circuit at the given path if it exists. Returns `true` on success,
    /// `false` if the file doesn't exist, and `Error` if the file system fails during operation.
    fn remove(&self, path: &Path) -> Result<bool> {
        let path = self.file_path(path);
        if !path.exists() {
            return Ok(false);
        }

        std::fs::remove_file(&path).map_err(|err| PackageError::failed_to_remove_file(self, err))?;
        Ok(true)
    }

    /// Get full path for the file.
    fn file_path<'a>(&self, path: &'a Path) -> Cow<'a, Path> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            if !path.ends_with(Self::ParentDirectory::NAME) {
                path.to_mut().push(Self::ParentDirectory::NAME);
            }
            path.to_mut().push(self.to_string());
        }
        path
    }
}

/// Handles the logic for folders. Creation and removal as well as stores
/// the directory name for easier access in PackageFile.
pub trait PackageDirectory {
    /// Name of the directory with a backslash on the end (e.g. src/).
    const NAME: &'static str;

    /// Creates a directory at the provided path.
    fn create(path: &Path) -> Result<()> {
        let mut path = std::borrow::Cow::from(path);
        if path.is_dir() && !path.ends_with(Self::NAME) {
            path.to_mut().push(Self::NAME);
        }

        fs::create_dir_all(&path).map_err(|err| PackageError::failed_to_create_directory(Self::NAME, err))?;
        Ok(())
    }

    /// Removes the directory at the provided path.
    fn remove(path: &Path) -> Result<()> {
        let mut path = Cow::from(path);
        if path.is_dir() && !path.ends_with(Self::NAME) {
            path.to_mut().push(Self::NAME);
        }

        if path.exists() {
            fs::remove_dir_all(&path).map_err(|err| PackageError::failed_to_remove_directory(Self::NAME, err))?;
        }

        Ok(())
    }
}
