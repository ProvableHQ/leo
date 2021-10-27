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

//! The `program.in` file.

use crate::PackageFile;

use leo_errors::{PackageError, Result};

use serde::Deserialize;
use std::{borrow::Cow, fs, path::Path};

pub static INPUT_FILE_EXTENSION: &str = ".in";

#[derive(Deserialize)]
pub struct InputFile {
    pub package_name: String,
}

impl PackageFile for InputFile {
    type ParentDirectory = super::InputsDirectory;

    fn template(&self) -> String {
        format!(
            r#"// The program input for {}/src/main.leo
[main]
a: u32 = 1;
b: u32 = 2;

[registers]
r0: u32 = 0;
"#,
            self.package_name
        )
    }
}

impl std::fmt::Display for InputFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.in", self.package_name)
    }
}

impl InputFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    /// Reads the program input variables from the given file path if it exists.
    pub fn read_from<'a>(&self, path: &'a Path) -> Result<(String, Cow<'a, Path>)> {
        let path = self.file_path(path);

        let input = fs::read_to_string(&path)
            .map_err(|_| PackageError::failed_to_read_input_file(path.clone().into_owned()))?;
        Ok((input, path))
    }
}
