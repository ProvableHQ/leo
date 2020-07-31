//! The `program.in` file.

use crate::{directories::inputs::INPUTS_DIRECTORY_NAME, errors::InputsFileError};

use serde::Deserialize;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub static INPUTS_FILE_EXTENSION: &str = ".in";

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

    pub fn exists_at(&self, path: &PathBuf) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    /// Reads the inputs from the given file path if it exists.
    pub fn read_from(&self, path: &PathBuf) -> Result<String, InputsFileError> {
        let path = self.setup_file_path(path);

        let inputs = fs::read_to_string(&path).map_err(|_| InputsFileError::FileReadError(path.clone()))?;
        Ok(inputs)
    }

    /// Writes the standard input format to a file.
    pub fn write_to(self, path: &PathBuf) -> Result<(), InputsFileError> {
        let path = self.setup_file_path(path);

        let mut file = File::create(&path)?;
        Ok(file.write_all(self.template().as_bytes())?)
    }

    fn template(&self) -> String {
        format!(
            r#"// The program inputs for {}/src/main.leo
[main]
a: u32 = 1;
b: u32 = 2;
"#,
            self.package_name
        )
    }

    fn setup_file_path(&self, path: &PathBuf) -> PathBuf {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(INPUTS_DIRECTORY_NAME) {
                path.push(PathBuf::from(INPUTS_DIRECTORY_NAME));
            }
            path.push(PathBuf::from(format!("{}{}", self.package_name, INPUTS_FILE_EXTENSION)));
        }
        path
    }
}
