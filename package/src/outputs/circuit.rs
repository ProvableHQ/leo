//! The serialized circuit output file.

use crate::{errors::CircuitFileError, outputs::OUTPUTS_DIRECTORY_NAME};

use serde::Deserialize;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub static CIRCUIT_FILE_EXTENSION: &str = ".bytes";

#[derive(Deserialize)]
pub struct CircuitFile {
    pub package_name: String,
}

impl CircuitFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn exists_at(&self, path: &PathBuf) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    /// Reads the serialized circuit from the given file path if it exists.
    pub fn read_from(&self, path: &PathBuf) -> Result<Vec<u8>, CircuitFileError> {
        let path = self.setup_file_path(path);

        Ok(fs::read(&path).map_err(|_| CircuitFileError::FileReadError(path.clone()))?)
    }

    /// Writes the given serialized circuit to a file.
    pub fn write_to(&self, path: &PathBuf, serialized_circuit: &[u8]) -> Result<(), CircuitFileError> {
        let path = self.setup_file_path(path);

        let mut file = File::create(&path)?;
        file.write_all(serialized_circuit)?;

        Ok(())
    }

    /// Removes the serialized circuit at the given path if it exists. Returns `true` on success,
    /// `false` if the file doesn't exist, and `Error` if the file system fails during operation.
    pub fn remove(&self, path: &PathBuf) -> Result<bool, CircuitFileError> {
        let path = self.setup_file_path(path);
        if !path.exists() {
            return Ok(false);
        }

        fs::remove_file(&path).map_err(|_| CircuitFileError::FileRemovalError(path.clone()))?;
        Ok(true)
    }

    fn setup_file_path(&self, path: &PathBuf) -> PathBuf {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(OUTPUTS_DIRECTORY_NAME) {
                path.push(PathBuf::from(OUTPUTS_DIRECTORY_NAME));
            }
            path.push(PathBuf::from(format!(
                "{}{}",
                self.package_name, CIRCUIT_FILE_EXTENSION
            )));
        }
        path
    }
}
