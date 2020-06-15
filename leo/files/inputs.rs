//! The `inputs.leo` file.

use crate::{directories::inputs::INPUTS_DIRECTORY_NAME, errors::MainFileError};

use serde::Deserialize;
use std::{fs::File, io::Write, path::PathBuf};

pub static INPUTS_FILE_NAME: &str = "inputs.leo";

#[derive(Deserialize)]
pub struct InputsFile {
    pub package_name: String,
}

impl InputsFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn exists_at(path: &PathBuf) -> bool {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(INPUTS_DIRECTORY_NAME) {
                path.push(PathBuf::from(INPUTS_DIRECTORY_NAME));
            }
            path.push(PathBuf::from(INPUTS_FILE_NAME));
        }
        path.exists()
    }

    pub fn write_to(self, path: &PathBuf) -> Result<(), MainFileError> {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(INPUTS_DIRECTORY_NAME) {
                path.push(PathBuf::from(INPUTS_DIRECTORY_NAME));
            }
            path.push(PathBuf::from(INPUTS_FILE_NAME));
        }

        let mut file = File::create(&path)?;
        Ok(file.write_all(self.template().as_bytes())?)
    }

    fn template(&self) -> String {
        format!(
            r#"// The inputs for {}/src/main.leo
[main]
a: u32 = 1;
b: u32 = 2;
"#,
            self.package_name
        )
    }
}
