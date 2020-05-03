//! The `main.leo` file.

use crate::directories::outputs::OUTPUTS_DIRECTORY_NAME;
use crate::errors::ProofFileError;

use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub static PROOF_FILE_EXTENSION: &str = ".leo.proof";

#[derive(Deserialize)]
pub struct ProofFile {
    pub package_name: String,
}

impl ProofFile {
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
            path.push(PathBuf::from(format!("{}{}", self.package_name, PROOF_FILE_EXTENSION)));
        }
        path.exists()
    }

    pub fn write_to(self, path: &PathBuf, proof: &[u8]) -> Result<(), ProofFileError> {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(OUTPUTS_DIRECTORY_NAME) {
                path.push(PathBuf::from(OUTPUTS_DIRECTORY_NAME));
            }
            path.push(PathBuf::from(format!("{}{}", self.package_name, PROOF_FILE_EXTENSION)));
        }

        let mut file = File::create(&path)?;
        file.write_all(proof)?;

        log::info!("Proof stored in {:?}", path);

        Ok(())
    }
}
