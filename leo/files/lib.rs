//! The `lib.leo` file.

use crate::{directories::source::SOURCE_DIRECTORY_NAME, errors::LibFileError};

use serde::Deserialize;
use std::{fs::File, io::Write, path::PathBuf};

pub static LIB_FILE_NAME: &str = "lib.leo";

#[derive(Deserialize)]
pub struct LibFile {
    pub package_name: String,
}

impl LibFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn exists_at(path: &PathBuf) -> bool {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(SOURCE_DIRECTORY_NAME) {
                path.push(PathBuf::from(SOURCE_DIRECTORY_NAME));
            }
            path.push(PathBuf::from(LIB_FILE_NAME));
        }
        path.exists()
    }

    pub fn write_to(self, path: &PathBuf) -> Result<(), LibFileError> {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(SOURCE_DIRECTORY_NAME) {
                path.push(PathBuf::from(SOURCE_DIRECTORY_NAME));
            }
            path.push(PathBuf::from(LIB_FILE_NAME));
        }

        let mut file = File::create(&path)?;
        Ok(file.write_all(self.template().as_bytes())?)
    }

    fn template(&self) -> String {
        format!(
            r#"// The '{}' lib function.
circuit Circ {{
    c: field
}}
"#,
            self.package_name
        )
    }
}
