use crate::errors::OutputsDirectoryError;

use std::fs;
use std::path::PathBuf;

pub(crate) static OUTPUTS_DIRECTORY_NAME: &str = "outputs/";

pub struct OutputsDirectory;

impl OutputsDirectory {
    /// Creates a directory at the provided path with the default directory name.
    pub fn create(path: &PathBuf) -> Result<(), OutputsDirectoryError> {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(OUTPUTS_DIRECTORY_NAME) {
            path.push(PathBuf::from(OUTPUTS_DIRECTORY_NAME));
        }

        fs::create_dir_all(&path).map_err(OutputsDirectoryError::Creating)
    }

    /// Removes the directory at the provided path.
    pub fn remove(path: &PathBuf) -> Result<(), OutputsDirectoryError> {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(OUTPUTS_DIRECTORY_NAME) {
            path.push(PathBuf::from(OUTPUTS_DIRECTORY_NAME));
        }

        if path.exists() {
            fs::remove_dir_all(&path).map_err(OutputsDirectoryError::Removing)?;
        }

        Ok(())
    }
}
