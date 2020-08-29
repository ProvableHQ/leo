use crate::{
    errors::PackageError,
    inputs::{InputFile, InputsDirectory, StateFile},
    root::{Gitignore, Manifest, README},
    source::{LibFile, MainFile, SourceDirectory},
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

    /// Returns `true` if a package exists at the given path
    pub fn exists_at(path: &PathBuf) -> bool {
        Manifest::exists_at(&path)
    }

    /// Creates a package at the given path
    pub fn create_package(package_name: &str, is_lib: bool, path: &PathBuf) -> Result<(), PackageError> {
        // Create the manifest file
        Manifest::new(&package_name).write_to(&path)?;

        // Create the .gitignore file
        Gitignore::new().write_to(&path)?;

        // Create the README.md file
        README::new(&package_name).write_to(&path)?;

        // Create the source directory
        SourceDirectory::create(&path)?;

        // Create a new library or binary file

        if is_lib {
            // Verify the library file does not exist
            if !LibFile::exists_at(&path) {
                // Create the library file in the source directory
                LibFile::new(&package_name).write_to(&path)?;
            }
        } else {
            // Create the input directory
            InputsDirectory::create(&path)?;

            // Verify the input file does not exist
            let input_file = InputFile::new(&package_name);
            if !input_file.exists_at(&path) {
                // Create the input file in the inputs directory
                input_file.write_to(&path)?;
            }

            // Verify the state file does not exist
            let state_file = StateFile::new(&package_name);
            if !state_file.exists_at(&path) {
                // Create the state file in the inputs directory
                state_file.write_to(&path)?;
            }

            // Verify the main file does not exist
            if !MainFile::exists_at(&path) {
                // Create the main file in the source directory
                MainFile::new(&package_name).write_to(&path)?;
            }
        }

        Ok(())
    }

    /// Removes the package at the given path
    pub fn remove_package(_package_name: &str) -> Result<(), PackageError> {
        unimplemented!()
    }
}
