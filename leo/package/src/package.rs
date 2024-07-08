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
    root::{Env, Gitignore},
    source::{MainFile, SourceDirectory},
};
use leo_errors::{PackageError, Result};

use leo_retriever::{Manifest, NetworkName};
use serde::Deserialize;
use snarkvm::prelude::{Network, PrivateKey};
use std::{path::Path, str::FromStr};

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub license: Option<String>,
    pub network: NetworkName,
}

impl Package {
    pub fn new(package_name: &str, network: NetworkName) -> Result<Self> {
        // Check that the package name is a valid Aleo program name.
        if !Self::is_aleo_name_valid(package_name) {
            return Err(PackageError::invalid_package_name(package_name).into());
        }

        Ok(Self {
            name: package_name.to_owned(),
            version: "0.1.0".to_owned(),
            description: None,
            license: None,
            network,
        })
    }

    /// Returns `true` if it is a valid Aleo name.
    ///
    /// Aleo names can only contain ASCII alphanumeric characters and underscores.
    pub fn is_aleo_name_valid(name: &str) -> bool {
        // Check that the name is nonempty.
        if name.is_empty() {
            tracing::error!("Aleo names must be nonempty");
            return false;
        }

        let first = name.chars().next().unwrap();

        // Check that the first character is not an underscore.
        if first == '_' {
            tracing::error!("Aleo names cannot begin with an underscore");
            return false;
        }

        // Check that the first character is not a number.
        if first.is_numeric() {
            tracing::error!("Aleo names cannot begin with a number");
            return false;
        }

        // Iterate and check that the name is valid.
        for current in name.chars() {
            // Check that the program name contains only ASCII alphanumeric or underscores.
            if !current.is_ascii_alphanumeric() && current != '_' {
                tracing::error!("Aleo names must can only contain ASCII alphanumeric characters and underscores.");
                return false;
            }
        }

        true
    }

    /// Returns `true` if a package is can be initialized at a given path.
    pub fn can_initialize(package_name: &str, path: &Path) -> bool {
        // Check that the package name is a valid Aleo program name.
        if !Self::is_aleo_name_valid(package_name) {
            return false;
        }

        let mut result = true;
        let mut existing_files = vec![];

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
        // Check that the package name is a valid Aleo program name.
        if !Self::is_aleo_name_valid(package_name) {
            return false;
        }

        // Check if the main file exists.
        if !MainFile::exists_at(path) {
            return false;
        }

        true
    }

    /// Creates a Leo package at the given path
    pub fn initialize<N: Network>(package_name: &str, path: &Path, endpoint: String) -> Result<()> {
        // Construct the path to the package directory.
        let path = path.join(package_name);

        // Verify that there is no existing directory at the path.
        if path.exists() {
            return Err(
                PackageError::failed_to_initialize_package(package_name, &path, "Directory already exists").into()
            );
        }

        // Create the package directory.
        std::fs::create_dir(&path).map_err(|e| PackageError::failed_to_initialize_package(package_name, &path, e))?;

        // Change the current working directory to the package directory.
        std::env::set_current_dir(&path)
            .map_err(|e| PackageError::failed_to_initialize_package(package_name, &path, e))?;

        // Create the .gitignore file.
        Gitignore::new().write_to(&path)?;

        // Create the .env file.
        // Include the private key of validator 0 for ease of use with local devnets, as it will automatically be seeded with funds.
        Env::<N>::new(
            Some(PrivateKey::<N>::from_str("APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH")?),
            endpoint,
        )?
        .write_to(&path)?;

        // Create a manifest.
        let manifest = Manifest::default(package_name);
        manifest.write_to_dir(&path)?;

        // Create the source directory.
        SourceDirectory::create(&path)?;

        // Create the main file in the source directory.
        MainFile::new(package_name).write_to(&path)?;

        // Next, verify that a valid Leo package has been initialized in this directory
        if !Self::is_initialized(package_name, &path) {
            return Err(PackageError::failed_to_initialize_package(
                package_name,
                &path,
                "Failed to correctly initialize package",
            )
            .into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_package_name_valid() {
        assert!(Package::is_aleo_name_valid("foo"));
        assert!(Package::is_aleo_name_valid("foo_bar"));
        assert!(Package::is_aleo_name_valid("foo1"));
        assert!(Package::is_aleo_name_valid("foo_bar___baz_"));

        assert!(!Package::is_aleo_name_valid("foo-bar"));
        assert!(!Package::is_aleo_name_valid("foo-bar-baz"));
        assert!(!Package::is_aleo_name_valid("foo-1"));
        assert!(!Package::is_aleo_name_valid(""));
        assert!(!Package::is_aleo_name_valid("-"));
        assert!(!Package::is_aleo_name_valid("-foo"));
        assert!(!Package::is_aleo_name_valid("-foo-"));
        assert!(!Package::is_aleo_name_valid("_foo"));
        assert!(!Package::is_aleo_name_valid("foo--bar"));
        assert!(!Package::is_aleo_name_valid("foo---bar"));
        assert!(!Package::is_aleo_name_valid("foo--bar--baz"));
        assert!(!Package::is_aleo_name_valid("foo---bar---baz"));
        assert!(!Package::is_aleo_name_valid("foo*bar"));
        assert!(!Package::is_aleo_name_valid("foo,bar"));
        assert!(!Package::is_aleo_name_valid("1-foo"));
    }
}
