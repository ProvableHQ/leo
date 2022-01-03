// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::PackageFile;
use leo_errors::{PackageError, Result};

use serde::Deserialize;
use std::path::Path;

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

    /// Reads the program input variables from the given file path if it exists.
    pub fn read_from<'a>(&self, path: &'a Path) -> Result<(String, std::borrow::Cow<'a, Path>)> {
        let path = self.file_path(path);

        let input = std::fs::read_to_string(&path)
            .map_err(|_| PackageError::failed_to_read_input_file(path.clone().into_owned()))?;
        Ok((input, path))
    }
}

impl PackageFile for StateFile {
    type ParentDirectory = super::InputsDirectory;

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
}

impl std::fmt::Display for StateFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.state", self.package_name)
    }
}
