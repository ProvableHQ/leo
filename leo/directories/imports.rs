use crate::errors::ImportsDirectoryError;

use std::{fs, path::PathBuf};

pub(crate) static IMPORTS_DIRECTORY_NAME: &str = "imports/";

pub struct ImportsDirectory;

impl ImportsDirectory {
    /// Creates a directory at the provided path with the default directory name.
    pub fn create(path: &PathBuf) -> Result<(), ImportsDirectoryError> {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(IMPORTS_DIRECTORY_NAME) {
            path.push(PathBuf::from(IMPORTS_DIRECTORY_NAME));
        }

        fs::create_dir_all(&path).map_err(ImportsDirectoryError::Creating)
    }

    /// Removes the directory at the provided path.
    pub fn remove(path: &PathBuf) -> Result<(), ImportsDirectoryError> {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(IMPORTS_DIRECTORY_NAME) {
            path.push(PathBuf::from(IMPORTS_DIRECTORY_NAME));
        }

        if path.exists() {
            fs::remove_dir_all(&path).map_err(ImportsDirectoryError::Removing)?;
        }

        Ok(())
    }
}
