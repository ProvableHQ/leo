//! The `main.leo` file.

use crate::directories::outputs::OUTPUTS_DIRECTORY_NAME;
use crate::errors::VerificationKeyFileError;

use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub static VERIFICATION_KEY_FILE_EXTENSION: &str = ".leo.vk";

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

    pub fn exists_at(self, path: &PathBuf) -> bool {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(OUTPUTS_DIRECTORY_NAME) {
                path.push(PathBuf::from(OUTPUTS_DIRECTORY_NAME));
            }
            path.push(PathBuf::from(format!("{}{}", self.package_name, VERIFICATION_KEY_FILE_EXTENSION)));
        }
        path.exists()
    }

    pub fn write_to(self, path: &PathBuf, verification_key: &[u8]) -> Result<(), VerificationKeyFileError> {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(OUTPUTS_DIRECTORY_NAME) {
                path.push(PathBuf::from(OUTPUTS_DIRECTORY_NAME));
            }
            path.push(PathBuf::from(format!("{}{}", self.package_name, VERIFICATION_KEY_FILE_EXTENSION)));
        }

        let mut file = File::create(&path)?;
        file.write_all(verification_key)?;

        log::info!("Verification key stored in {:?}", path);

        Ok(())
    }
}
