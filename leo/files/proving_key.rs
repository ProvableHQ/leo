//! The `main.leo` file.

use crate::directories::inputs::INPUTS_DIRECTORY_NAME;
use crate::errors::MainFileError;

use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub static PROVING_FILE_EXTENSION: &str = ".leo.pk";

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

    pub fn exists_at(self, path: &PathBuf) -> bool {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(INPUTS_DIRECTORY_NAME) {
                path.push(PathBuf::from(INPUTS_DIRECTORY_NAME));
            }
            path.push(PathBuf::from(format!("{}{}", self.package_name, PROVING_FILE_EXTENSION)));
        }
        path.exists()
    }

    pub fn write_to(self, path: &PathBuf, proving_key: &[u8]) -> Result<(), MainFileError> {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(INPUTS_DIRECTORY_NAME) {
                path.push(PathBuf::from(INPUTS_DIRECTORY_NAME));
            }
            path.push(PathBuf::from(format!("{}{}", self.package_name, PROVING_FILE_EXTENSION)));
        }

        let mut file = File::create(&path)?;
        file.write_all(proving_key)?;

        log::info!("Proving key stored in {:?}", path);

        Ok(())
    }
}
