//! The `README.md` file.

use crate::errors::READMEError;

use serde::Deserialize;
use std::{fs::File, io::Write, path::PathBuf};

pub static README_FILE_NAME: &str = "README.md";

#[derive(Deserialize)]
pub struct README {
    pub package_name: String,
}

impl README {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn package_name(&self) -> String {
        self.package_name.clone()
    }

    pub fn exists_at(path: &PathBuf) -> bool {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(README_FILE_NAME));
        }
        path.exists()
    }

    pub fn write_to(self, path: &PathBuf) -> Result<(), READMEError> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(README_FILE_NAME));
        }

        let mut file = File::create(&path)?;
        Ok(file.write_all(self.template().as_bytes())?)
    }

    fn template(&self) -> String {
        format!("# {}\n", self.package_name)
    }
}
