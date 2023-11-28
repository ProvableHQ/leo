// Copyright (C) 2019-2023 Aleo Systems Inc.
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

pub static INPUT_FILE_EXTENSION: &str = ".in";

#[derive(Deserialize)]
pub struct InputFile {
    pub package_name: String,
}

impl InputFile {
    pub fn new(package_name: &str) -> Self {
        Self { package_name: package_name.to_string() }
    }

    pub fn filename(&self) -> String {
        format!("{INPUTS_DIRECTORY_NAME}{}{INPUT_FILE_EXTENSION}", self.package_name)
    }

    pub fn exists_at(&self, path: &Path) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    /// Reads the program input variables from the given file path if it exists.
    pub fn read_from<'a>(&self, path: &'a Path) -> Result<(String, Cow<'a, Path>)> {
        let path = self.setup_file_path(path);

        let input = fs::read_to_string(&path)
            .map_err(|_| PackageError::failed_to_read_input_file(path.clone().into_owned()))?;
        Ok((input, path))
    }

    /// Writes the standard input format to a file.
    pub fn write_to(self, path: &Path) -> Result<()> {
        let path = self.setup_file_path(path);
        let mut file = File::create(path).map_err(PackageError::io_error_input_file)?;

        file.write_all(self.template().as_bytes()).map_err(PackageError::io_error_input_file)?;
        Ok(())
    }

    fn template(&self) -> String {
        format!(
            r#"// The program input for {}/src/main.leo
[main]
public a: u32 = 1u32;
b: u32 = 2u32;
"#,
            self.package_name
        )
    }

    pub fn setup_file_path<'a>(&self, path: &'a Path) -> Cow<'a, Path> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            if !path.ends_with(INPUTS_DIRECTORY_NAME) {
                path.to_mut().push(INPUTS_DIRECTORY_NAME);
            }
            path.to_mut().push(format!("{}{INPUT_FILE_EXTENSION}", self.package_name));
        }
        path
    }
}
