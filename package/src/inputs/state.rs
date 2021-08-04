// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

//! The `program.state` file.

use crate::inputs::INPUTS_DIRECTORY_NAME;
use leo_errors::{PackageError, Result};

use serde::Deserialize;
use std::{
    borrow::Cow,
    fs::{
        File,
        {self},
    },
    io::Write,
    path::Path,
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

    pub fn filename(&self) -> String {
        format!("{}{}{}", INPUTS_DIRECTORY_NAME, self.package_name, STATE_FILE_EXTENSION)
    }

    pub fn exists_at(&self, path: &Path) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    /// Reads the state input variables from the given file path if it exists.
    pub fn read_from<'a>(&self, path: &'a Path) -> Result<(String, Cow<'a, Path>)> {
        let path = self.setup_file_path(path);

        let input = fs::read_to_string(&path)
            .map_err(|_| PackageError::failed_to_read_state_file(path.clone().into_owned()))?;
        Ok((input, path))
    }

    /// Writes the standard input format to a file.
    pub fn write_to(self, path: &Path) -> Result<()> {
        let path = self.setup_file_path(path);
        let mut file = File::create(&path).map_err(|e| PackageError::io_error_state_file(e))?;

        Ok(file
            .write_all(self.template().as_bytes())
            .map_err(|e| PackageError::io_error_state_file(e))?)
    }

    fn template(&self) -> String {
        format!(
            r#"// The program state for {}/src/main.leo
[[public]]

[state]
leaf_index: u32 = 0;
root: [u8; 32] = [0; 32];

[[private]]

[record]
serial_number: [u8; 64] = [0; 64];
commitment: [u8; 32] = [0; 32];
owner: address = aleo1daxej63vwrmn2zhl4dymygagh89k5d2vaw6rjauueme7le6k2q8sjn0ng9;
is_dummy: bool = false;
value: u64 = 0;
payload: [u8; 32] = [0; 32];
birth_program_id: [u8; 48] = [0; 48];
death_program_id: [u8; 48] = [0; 48];
serial_number_nonce: [u8; 32] = [0; 32];
commitment_randomness: [u8; 32] = [0; 32];

[state_leaf]
path: [u8; 128] = [0; 128];
memo: [u8; 32] = [0; 32];
network_id: u8 = 0;
leaf_randomness: [u8; 32] = [0; 32];
"#,
            self.package_name
        )
    }

    fn setup_file_path<'a>(&self, path: &'a Path) -> Cow<'a, Path> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            if !path.ends_with(INPUTS_DIRECTORY_NAME) {
                path.to_mut().push(INPUTS_DIRECTORY_NAME);
            }
            path.to_mut()
                .push(format!("{}{}", self.package_name, STATE_FILE_EXTENSION));
        }
        path
    }
}
