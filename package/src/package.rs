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

use crate::{
    errors::PackageError,
    imports::ImportsDirectory,
    inputs::{InputFile, InputsDirectory, StateFile},
    root::{Gitignore, Manifest, README},
    source::{LibraryFile, MainFile, SourceDirectory},
};

use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub license: Option<String>,
}

impl Package {
    pub fn new(package_name: &str) -> Self {
        Self {
            name: package_name.to_owned(),
            version: "0.1.0".to_owned(),
            description: None,
            license: None,
        }
    }

    /// Returns `true` if a package is can be initialized at a given path.
    pub fn can_initialize(package_name: &str, is_lib: bool, path: &PathBuf) -> bool {
        let mut result = true;
        let mut existing_files = vec![];

        // Check if the manifest file already exists.
        if Manifest::exists_at(&path) {
            existing_files.push(Manifest::filename());
            result = false;
        }

        if is_lib {
            // Check if the library file already exists.
            if LibraryFile::exists_at(&path) {
                existing_files.push(LibraryFile::filename());
                result = false;
            }
        } else {
            // Check if the input file already exists.
            let input_file = InputFile::new(&package_name);
            if input_file.exists_at(&path) {
                existing_files.push(input_file.filename());
                result = false;
            }

            // Check if the state file already exists.
            let state_file = StateFile::new(&package_name);
            if state_file.exists_at(&path) {
                existing_files.push(state_file.filename());
                result = false;
            }

            // Check if the main file already exists.
            if MainFile::exists_at(&path) {
                existing_files.push(MainFile::filename());
                result = false;
            }
        }

        if existing_files.len() > 0 {
            tracing::error!("File(s) {:?} already exist", existing_files);
        }

        result
    }

    /// Returns `true` if a package is initialized at the given path
    pub fn is_initialized(package_name: &str, is_lib: bool, path: &PathBuf) -> bool {
        // Check if the manifest file exists.
        if !Manifest::exists_at(&path) {
            return false;
        }

        if is_lib {
            // Check if the library file exists.
            if !LibraryFile::exists_at(&path) {
                return false;
            }
        } else {
            // Check if the input file exists.
            let input_file = InputFile::new(&package_name);
            if !input_file.exists_at(&path) {
                return false;
            }

            // Check if the state file exists.
            let state_file = StateFile::new(&package_name);
            if !state_file.exists_at(&path) {
                return false;
            }

            // Check if the main file exists.
            if !MainFile::exists_at(&path) {
                return false;
            }
        }

        true
    }

    /// Creates a package at the given path
    pub fn initialize(package_name: &str, is_lib: bool, path: &PathBuf) -> Result<(), PackageError> {
        // First, verify that this directory is not already initialized as a Leo package.
        {
            if !Self::can_initialize(package_name, is_lib, path) {
                return Err(
                    PackageError::FailedToInitialize(package_name.to_owned(), path.as_os_str().to_owned()).into(),
                );
            }
        }
        // Next, initialize this directory as a Leo package.
        {
            // Create the manifest file.
            Manifest::new(&package_name).write_to(&path)?;

            // Verify that the .gitignore file does not exist.
            if !Gitignore::exists_at(&path) {
                // Create the .gitignore file.
                Gitignore::new().write_to(&path)?;
            }

            // Verify that the README.md file does not exist.
            if !README::exists_at(&path) {
                // Create the README.md file.
                README::new(package_name).write_to(&path)?;
            }

            // Create the source directory.
            SourceDirectory::create(&path)?;

            // Create a new library or binary file.
            if is_lib {
                // Create the library file in the source directory.
                LibraryFile::new(&package_name).write_to(&path)?;
            } else {
                // Create the input directory.
                InputsDirectory::create(&path)?;

                // Create the input file in the inputs directory.
                InputFile::new(&package_name).write_to(&path)?;

                // Create the state file in the inputs directory.
                StateFile::new(&package_name).write_to(&path)?;

                // Create the main file in the source directory.
                MainFile::new(&package_name).write_to(&path)?;
            }
        }
        // Next, verify that a valid Leo package has been initialized in this directory
        {
            if !Self::is_initialized(package_name, is_lib, path) {
                return Err(
                    PackageError::FailedToInitialize(package_name.to_owned(), path.as_os_str().to_owned()).into(),
                );
            }
        }

        Ok(())
    }

    /// Removes the package at the given path
    pub fn remove_imported_package(package_name: &str, path: &PathBuf) -> Result<(), PackageError> {
        Ok(ImportsDirectory::remove_import(path, package_name)?)
    }
}
