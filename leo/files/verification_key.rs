//! The verification key file.

use crate::{directories::outputs::OUTPUTS_DIRECTORY_NAME, errors::VerificationKeyFileError};

use serde::Deserialize;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub static VERIFICATION_KEY_FILE_EXTENSION: &str = ".lvk";

#[derive(Deserialize)]
pub struct VerificationKeyFile {
    pub package_name: String,
}

impl VerificationKeyFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn exists_at(&self, path: &PathBuf) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    /// Reads the verification key from the given file path if it exists.
    pub fn read_from(&self, path: &PathBuf) -> Result<Vec<u8>, VerificationKeyFileError> {
        let path = self.setup_file_path(path);

        Ok(fs::read(&path).map_err(|_| VerificationKeyFileError::FileReadError(path.clone()))?)
    }

    /// Writes the given verification key to a file.
    pub fn write_to(&self, path: &PathBuf, verification_key: &[u8]) -> Result<(), VerificationKeyFileError> {
        let path = self.setup_file_path(path);

        let mut file = File::create(&path)?;
        file.write_all(verification_key)?;

        log::info!("Verification key stored to {:?}", path);

        Ok(())
    }

    fn setup_file_path(&self, path: &PathBuf) -> PathBuf {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(OUTPUTS_DIRECTORY_NAME) {
                path.push(PathBuf::from(OUTPUTS_DIRECTORY_NAME));
            }
            path.push(PathBuf::from(format!(
                "{}{}",
                self.package_name, VERIFICATION_KEY_FILE_EXTENSION
            )));
        }
        path
    }
}
