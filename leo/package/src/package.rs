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

use crate::{
    build::BuildDirectory,
    inputs::{InputFile, InputsDirectory},
    root::{Env, Gitignore},
    source::{MainFile, SourceDirectory},
};
use leo_errors::{PackageError, Result};
use snarkvm::console::prelude::Network;

use serde::Deserialize;
use std::{marker::PhantomData, path::Path};

#[derive(Deserialize)]
pub struct Package<N: Network> {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub license: Option<String>,
    _phantom: PhantomData<N>,
}

impl<N: Network> Package<N> {
    pub fn new(package_name: &str) -> Result<Self> {
        // Check that the package name is valid.
        if !Self::is_package_name_valid(package_name) {
            return Err(PackageError::invalid_package_name(package_name).into());
        }

        Ok(Self {
            name: package_name.to_owned(),
            version: "0.1.0".to_owned(),
            description: None,
            license: None,
            _phantom: PhantomData,
        })
    }

    /// Returns `true` if the package name is valid.
    ///
    /// Package names can only contain ASCII alphanumeric characters and underscores.
    pub fn is_package_name_valid(package_name: &str) -> bool {
        // Check that the package name is nonempty.
        if package_name.is_empty() {
            tracing::error!("Project names must be nonempty");
            return false;
        }

        let first = package_name.chars().next().unwrap();

        // Check that the first character is not an underscore.
        if first == '_' {
            tracing::error!("Project names cannot begin with an underscore");
            return false;
        }

        // Check that the first character is not a number.
        if first.is_numeric() {
            tracing::error!("Project names cannot begin with a number");
            return false;
        }

        // Iterate and check that the package name is valid.
        for current in package_name.chars() {
            // Check that the package name contains only ASCII alphanumeric or underscores.
            if !current.is_ascii_alphanumeric() && current != '_' {
                tracing::error!("Project names must can only contain ASCII alphanumeric characters and underscores.");
                return false;
            }
        }

        true
    }

    /// Returns `true` if a package is can be initialized at a given path.
    pub fn can_initialize(package_name: &str, path: &Path) -> bool {
        // Check that the package name is valid.
        if !Self::is_package_name_valid(package_name) {
            return false;
        }

        let mut result = true;
        let mut existing_files = vec![];

        // Check if the input file already exists.
        let input_file = InputFile::new(package_name);
        if input_file.exists_at(path) {
            existing_files.push(input_file.filename());
            result = false;
        }

        // Check if the main file already exists.
        if MainFile::exists_at(path) {
            existing_files.push(MainFile::filename());
            result = false;
        }

        if !existing_files.is_empty() {
            tracing::error!("File(s) {:?} already exist", existing_files);
        }

        result
    }

    /// Returns `true` if a package is initialized at the given path
    pub fn is_initialized(package_name: &str, path: &Path) -> bool {
        // Check that the package name is valid.
        if !Self::is_package_name_valid(package_name) {
            return false;
        }

        // Check if the input file exists.
        let input_file = InputFile::new(package_name);
        if !input_file.exists_at(path) {
            return false;
        }

        // Check if the main file exists.
        if !MainFile::exists_at(path) {
            return false;
        }

        true
    }

    /// Creates a Leo package at the given path
    pub fn initialize(package_name: &str, path: &Path) -> Result<()> {
        // Verify that the .gitignore file does not exist.
        if !Gitignore::exists_at(path) {
            // Create the .gitignore file.
            Gitignore::new().write_to(path)?;
        }

        // Verify that the .env file does not exist.
        if !Env::<N>::exists_at(path) {
            // Create the .env file.
            Env::<N>::new()?.write_to(path)?;
        }

        // Create the source directory.
        SourceDirectory::create(path)?;

        // Create the inputs directory.
        InputsDirectory::create(path)?;

        // Create the Leo build/ directory
        BuildDirectory::create(path)?;

        // Create the input file in the inputs directory.
        InputFile::new(package_name).write_to(path)?;

        // Create the main file in the source directory.
        MainFile::new(package_name).write_to(path)?;

        // Next, verify that a valid Leo package has been initialized in this directory
        if !Self::is_initialized(package_name, path) {
            return Err(PackageError::failed_to_initialize_package(package_name, path.as_os_str()).into());
        }

        Ok(())
    }
    //
    // /// Removes the package at the given path
    // pub fn remove_imported_package(package_name: &str, path: &Path) -> Result<()> {
    //     ImportsDirectory::remove_import(path, package_name)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    type CurrentNetwork = snarkvm::prelude::Testnet3;

    #[test]
    fn test_is_package_name_valid() {
        assert!(Package::<CurrentNetwork>::is_package_name_valid("foo"));
        assert!(Package::<CurrentNetwork>::is_package_name_valid("foo_bar"));
        assert!(Package::<CurrentNetwork>::is_package_name_valid("foo1"));
        assert!(Package::<CurrentNetwork>::is_package_name_valid("foo_bar___baz_"));

        assert!(!Package::<CurrentNetwork>::is_package_name_valid("foo-bar"));
        assert!(!Package::<CurrentNetwork>::is_package_name_valid("foo-bar-baz"));
        assert!(!Package::<CurrentNetwork>::is_package_name_valid("foo-1"));
        assert!(!Package::<CurrentNetwork>::is_package_name_valid(""));
        assert!(!Package::<CurrentNetwork>::is_package_name_valid("-"));
        assert!(!Package::<CurrentNetwork>::is_package_name_valid("-foo"));
        assert!(!Package::<CurrentNetwork>::is_package_name_valid("-foo-"));
        assert!(!Package::<CurrentNetwork>::is_package_name_valid("_foo"));
        assert!(!Package::<CurrentNetwork>::is_package_name_valid("foo--bar"));
        assert!(!Package::<CurrentNetwork>::is_package_name_valid("foo---bar"));
        assert!(!Package::<CurrentNetwork>::is_package_name_valid("foo--bar--baz"));
        assert!(!Package::<CurrentNetwork>::is_package_name_valid("foo---bar---baz"));
        assert!(!Package::<CurrentNetwork>::is_package_name_valid("foo*bar"));
        assert!(!Package::<CurrentNetwork>::is_package_name_valid("foo,bar"));
        assert!(!Package::<CurrentNetwork>::is_package_name_valid("1-foo"));
    }
}
