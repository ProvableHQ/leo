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
    pub fn initialize(package_name: &str, is_lib: bool, path: &PathBuf) -> Result<(), PackageError> {
        // First, verify that this directory is not already initialized as a Leo package.
        {
            // Verify the manifest file does not already exist.
            if Manifest::exists_at(&path) {
                return Err(PackageError::FileAlreadyExists(Manifest::filename(), path.as_os_str().to_owned()).into());
            }

            if is_lib {
                // Verify the library file does not exist.
                if LibFile::exists_at(&path) {
                    return Err(
                        PackageError::FileAlreadyExists(LibFile::filename(), path.as_os_str().to_owned()).into(),
                    );
                }
            } else {
                // Verify the input file does not exist.
                let input_file = InputFile::new(&package_name);
                if input_file.exists_at(&path) {
                    return Err(
                        PackageError::FileAlreadyExists(input_file.filename(), path.as_os_str().to_owned()).into(),
                    );
                }

                // Verify the state file does not exist.
                let state_file = StateFile::new(&package_name);
                if state_file.exists_at(&path) {
                    return Err(
                        PackageError::FileAlreadyExists(state_file.filename(), path.as_os_str().to_owned()).into(),
                    );
                }

                // Verify the main file does not exist.
                if MainFile::exists_at(&path) {
                    return Err(
                        PackageError::FileAlreadyExists(MainFile::filename(), path.as_os_str().to_owned()).into(),
                    );
                }
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
                LibFile::new(&package_name).write_to(&path)?;
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

        Ok(())
    }

    /// Removes the package at the given path
    pub fn remove_package(_package_name: &str) -> Result<(), PackageError> {
        unimplemented!()
    }
}
