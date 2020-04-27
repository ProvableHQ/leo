//! The `main.leo` file.

use crate::directories::source::SOURCE_DIRECTORY_NAME;
use crate::errors::MainFileError;

use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub static MAIN_FILE_NAME: &str = "main.leo";

#[derive(Deserialize)]
pub struct MainFile {
    pub package_name: String,
}

impl MainFile {
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
            path.push(PathBuf::from(MAIN_FILE_NAME));
        }
        path.exists()
    }

    pub fn write_to(self, path: &PathBuf) -> Result<(), MainFileError> {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(SOURCE_DIRECTORY_NAME) {
                path.push(PathBuf::from(SOURCE_DIRECTORY_NAME));
            }
            path.push(PathBuf::from(MAIN_FILE_NAME));
        }

        let mut file = File::create(&path)?;
        Ok(file.write_all(self.template().as_bytes())?)
    }

    fn template(&self) -> String {
        format!(
            r#"// The '{}' main function.
function main() -> (u32) {{
    a = 1 + 1
    return a
}}
"#,
            self.package_name
        )
    }
}
