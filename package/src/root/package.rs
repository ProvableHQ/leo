use crate::{
    errors::PackageError,
    imports::{ImportsDirectory, IMPORTS_DIRECTORY_NAME},
    inputs::*,
    outputs::OutputsDirectory,
    root::{Gitignore, Manifest},
    source::{LibFile, MainFile, SourceDirectory},
};
use std::env::current_dir;

pub struct Package;

impl Package {
    /// Creates a new Leo package
    pub fn create(package_name: &str, is_lib: bool) -> Result<(), PackageError> {
        let path = current_dir()?;

        // Create the manifest file
        Manifest::new(&package_name).write_to(&path)?;

        // Create the .gitignore file
        Gitignore::new().write_to(&path)?;

        // Create the source directory
        SourceDirectory::create(&path)?;

        // Create a new library or binary file
        if is_lib {
            // Create the library file in the source directory
            LibFile::new(&package_name).write_to(&path)?;
        } else {
            // Create the input directory
            InputsDirectory::create(&path)?;

            // Create the input file in the inputs directory
            InputFile::new(&package_name).write_to(&path)?;

            // Create the state file in the inputs directory
            StateFile::new(&package_name).write_to(&path)?;

            // Create the main file in the source directory
            MainFile::new(&package_name).write_to(&path)?;
        }
        Ok(())
    }

    /// Removes the Leo package
    pub fn remove(package_name: &str) -> Result<(), PackageError> {
        // Create path for the package destination
        let mut path = current_dir()?;
        ImportsDirectory::create(&path)?;
        path.push(IMPORTS_DIRECTORY_NAME);
        path.push(&package_name);

        // Remove all Leo source files
        SourceDirectory::remove_files(&path)?;

        // Remove imports directory
        ImportsDirectory::remove(&path)?;

        // Remove outputs directory
        OutputsDirectory::remove(&path)?;

        // Remove manifest file
        Manifest::remove(&path)?;

        // Remove gitignore file
        Gitignore::remove(&path)?;

        // If the package directory is empty then remove it
        if path.read_dir()?.next().is_none() {
            std::fs::remove_dir(path)?;
            log::info!("Package {} removed successfully", package_name);
        } else {
            log::warn!("Cannot remove package. Package directory contains some foreign files");
        }
        Ok(())
    }

    pub fn create_src(_package_name: &str) -> Result<(), PackageError> {
        Ok(())
    }

    pub fn remove_src(_package_name: &str) -> Result<(), PackageError> {
        Ok(())
    }

    pub fn create_imports(_package_name: &str) -> Result<(), PackageError> {
        Ok(())
    }

    pub fn remove_imports(_package_name: &str) -> Result<(), PackageError> {
        Ok(())
    }

    pub fn create_outputs(_package_name: &str) -> Result<(), PackageError> {
        Ok(())
    }

    pub fn remove_outpus(_package_name: &str) -> Result<(), PackageError> {
        Ok(())
    }
}
