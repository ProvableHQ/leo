//! The proving key file.

use crate::{errors::ProvingKeyFileError, outputs::OUTPUTS_DIRECTORY_NAME};

use serde::Deserialize;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub static PROVING_KEY_FILE_EXTENSION: &str = ".lpk";

#[derive(Deserialize)]
pub struct ProvingKeyFile {
    pub package_name: String,
}

impl ProvingKeyFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn exists_at(&self, path: &PathBuf) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    /// Reads the proving key from the given file path if it exists.
    pub fn read_from(&self, path: &PathBuf) -> Result<Vec<u8>, ProvingKeyFileError> {
        let path = self.setup_file_path(path);

        Ok(fs::read(&path).map_err(|_| ProvingKeyFileError::FileReadError(path.clone()))?)
    }

    /// Writes the given proving key to a file.
    pub fn write_to(&self, path: &PathBuf, proving_key: &[u8]) -> Result<(), ProvingKeyFileError> {
        let path = self.setup_file_path(path);

        let mut file = File::create(&path)?;
        file.write_all(proving_key)?;

        Ok(())
    }

    /// Removes the proving key at the given path if it exists. Returns `true` on success,
    /// `false` if the file doesn't exist, and `Error` if the file system fails during operation.
    pub fn remove(&self, path: &PathBuf) -> Result<bool, ProvingKeyFileError> {
        let path = self.setup_file_path(path);
        if !path.exists() {
            return Ok(false);
        }

        fs::remove_file(&path).map_err(|_| ProvingKeyFileError::FileRemovalError(path.clone()))?;
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
                self.package_name, PROVING_KEY_FILE_EXTENSION
            )));
        }
        path
    }
}
