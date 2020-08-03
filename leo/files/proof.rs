//! The proof file.

use crate::{directories::output::OUTPUT_DIRECTORY_NAME, errors::ProofFileError};

use serde::Deserialize;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub static PROOF_FILE_EXTENSION: &str = ".proof";

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

    pub fn exists_at(&self, path: &PathBuf) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    /// Reads the proof from the given file path if it exists.
    pub fn read_from(&self, path: &PathBuf) -> Result<String, ProofFileError> {
        let path = self.setup_file_path(path);

        let proof = fs::read_to_string(&path).map_err(|_| ProofFileError::FileReadError(path.clone()))?;
        Ok(proof)
    }

    /// Writes the given proof to a file.
    pub fn write_to(&self, path: &PathBuf, proof: &[u8]) -> Result<(), ProofFileError> {
        let path = self.setup_file_path(path);

        let mut file = File::create(&path)?;
        file.write_all(proof)?;

        log::info!("Proof stored ({:?})", path);

        Ok(())
    }

    /// Removes the proof at the given path if it exists. Returns `true` on success,
    /// `false` if the file doesn't exist, and `Error` if the file system fails during operation.
    pub fn remove(&self, path: &PathBuf) -> Result<bool, ProofFileError> {
        let path = self.setup_file_path(path);
        if !path.exists() {
            return Ok(false);
        }

        fs::remove_file(&path).map_err(|_| ProofFileError::FileRemovalError(path.clone()))?;
        Ok(true)
    }

    fn setup_file_path(&self, path: &PathBuf) -> PathBuf {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(OUTPUT_DIRECTORY_NAME) {
                path.push(PathBuf::from(OUTPUT_DIRECTORY_NAME));
            }
            path.push(PathBuf::from(format!("{}{}", self.package_name, PROOF_FILE_EXTENSION)));
        }
        path
    }
}
