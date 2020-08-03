//! The `program.state` file.

use crate::{errors::StateFileError, inputs::INPUTS_DIRECTORY_NAME};

use serde::Deserialize;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub static STATE_FILE_EXTENSION: &str = ".state";

#[derive(Deserialize)]
pub struct StateFile {
    pub package_name: String,
}

impl StateFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn exists_at(&self, path: &PathBuf) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    /// Reads the state input variables from the given file path if it exists.
    pub fn read_from(&self, path: &PathBuf) -> Result<String, StateFileError> {
        let path = self.setup_file_path(path);

        let input = fs::read_to_string(&path).map_err(|_| StateFileError::FileReadError(path.clone()))?;
        Ok(input)
    }

    /// Writes the standard input format to a file.
    pub fn write_to(self, path: &PathBuf) -> Result<(), StateFileError> {
        let path = self.setup_file_path(path);

        let mut file = File::create(&path)?;
        Ok(file.write_all(self.template().as_bytes())?)
    }

    fn template(&self) -> String {
        format!(
            r#"// The program state for {}/src/main.leo
[[public]]

[state]
leaf_index: u32 = 0;
root: u8[32] = [0u8; 32];

[[private]]

[record]
serial_number: u8[32] = [0u8; 32];
commitment: u8[32] = [0u8; 32];
owner: address = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8;
value: u64 = 5;
payload: u8[32] = [0u8; 32]; // TBD
birth_program_id: u8[32] = [0u8; 32];
death_program_id: u8[32] = [0u8; 32];
serial_number_nonce: u8[32] = [0u8; 32];
commitment_randomness: u8[32] = [0u8; 32];

[state_leaf]
path: u8[32][2] = [ [0u8; 32], [0u8; 32] ];
memo: u8[32] = [0u8; 32];
network_id: u8 = 0;
leaf_randomness: u8[32] = [0u8; 32];
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
            path.push(PathBuf::from(format!("{}{}", self.package_name, STATE_FILE_EXTENSION)));
        }
        path
    }
}
