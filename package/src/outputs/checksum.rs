//! The build checksum file.

use crate::{errors::ChecksumFileError, outputs::OUTPUTS_DIRECTORY_NAME};

use serde::Deserialize;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub static CHECKSUM_FILE_EXTENSION: &str = ".sum";

#[derive(Deserialize)]
pub struct ChecksumFile {
    pub package_name: String,
}

impl ChecksumFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn exists_at(&self, path: &PathBuf) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    /// Reads the checksum from the given file path if it exists.
    pub fn read_from(&self, path: &PathBuf) -> Result<String, ChecksumFileError> {
        let path = self.setup_file_path(path);

        Ok(fs::read_to_string(&path).map_err(|_| ChecksumFileError::FileReadError(path.clone()))?)
    }

    /// Writes the given checksum to a file.
    pub fn write_to(&self, path: &PathBuf, checksum: String) -> Result<(), ChecksumFileError> {
        let path = self.setup_file_path(path);

        let mut file = File::create(&path)?;
        file.write_all(checksum.as_bytes())?;

        Ok(())
    }

    /// Removes the checksum at the given path if it exists. Returns `true` on success,
    /// `false` if the file doesn't exist, and `Error` if the file system fails during operation.
    pub fn remove(&self, path: &PathBuf) -> Result<bool, ChecksumFileError> {
        let path = self.setup_file_path(path);
        if !path.exists() {
            return Ok(false);
        }

        fs::remove_file(&path).map_err(|_| ChecksumFileError::FileRemovalError(path.clone()))?;
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
                self.package_name, CHECKSUM_FILE_EXTENSION
            )));
        }
        path
    }
}
