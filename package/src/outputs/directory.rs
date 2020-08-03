use crate::errors::OutputDirectoryError;

use std::{fs, path::PathBuf};

pub static OUTPUT_DIRECTORY_NAME: &str = "outputs/";

pub struct OutputDirectory;

impl OutputDirectory {
    /// Creates a directory at the provided path with the default directory name.
    pub fn create(path: &PathBuf) -> Result<(), OutputDirectoryError> {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(OUTPUT_DIRECTORY_NAME) {
            path.push(PathBuf::from(OUTPUT_DIRECTORY_NAME));
        }

        fs::create_dir_all(&path).map_err(OutputDirectoryError::Creating)
    }

    /// Removes the directory at the provided path.
    pub fn remove(path: &PathBuf) -> Result<(), OutputDirectoryError> {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(OUTPUT_DIRECTORY_NAME) {
            path.push(PathBuf::from(OUTPUT_DIRECTORY_NAME));
        }

        if path.exists() {
            fs::remove_dir_all(&path).map_err(OutputDirectoryError::Removing)?;
        }

        Ok(())
    }
}
