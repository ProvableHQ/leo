//! The `program.in` file.

use crate::{errors::InputFileError, inputs::INPUT_DIRECTORY_NAME};

use serde::Deserialize;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub static INPUT_FILE_EXTENSION: &str = ".in";

#[derive(Deserialize)]
pub struct InputFile {
    pub package_name: String,
}

impl InputFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn exists_at(&self, path: &PathBuf) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    /// Reads the program input variables from the given file path if it exists.
    pub fn read_from(&self, path: &PathBuf) -> Result<String, InputFileError> {
        let path = self.setup_file_path(path);

        let input = fs::read_to_string(&path).map_err(|_| InputFileError::FileReadError(path.clone()))?;
        Ok(input)
    }

    /// Writes the standard input format to a file.
    pub fn write_to(self, path: &PathBuf) -> Result<(), InputFileError> {
        let path = self.setup_file_path(path);

        let mut file = File::create(&path)?;
        Ok(file.write_all(self.template().as_bytes())?)
    }

    fn template(&self) -> String {
        format!(
            r#"// The program input for {}/src/main.leo
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
            if !path.ends_with(INPUT_DIRECTORY_NAME) {
                path.push(PathBuf::from(INPUT_DIRECTORY_NAME));
            }
            path.push(PathBuf::from(format!("{}{}", self.package_name, INPUT_FILE_EXTENSION)));
        }
        path
    }
}
