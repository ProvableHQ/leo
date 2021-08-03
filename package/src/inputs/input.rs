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

use crate::inputs::INPUTS_DIRECTORY_NAME;

use leo_errors::{LeoError, PackageError};

use backtrace::Backtrace;
use eyre::eyre;
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
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn filename(&self) -> String {
        format!("{}{}{}", INPUTS_DIRECTORY_NAME, self.package_name, INPUT_FILE_EXTENSION)
    }

    pub fn exists_at(&self, path: &Path) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    /// Reads the program input variables from the given file path if it exists.
    pub fn read_from<'a>(&self, path: &'a Path) -> Result<(String, Cow<'a, Path>), LeoError> {
        let path = self.setup_file_path(path);

        match fs::read_to_string(&path) {
            Ok(input) => Ok((input, path)),
            Err(_) => Err(PackageError::failed_to_read_input_file(path.into_owned(), Backtrace::new()).into()),
        }
    }

    /// Writes the standard input format to a file.
    pub fn write_to(self, path: &Path) -> Result<(), LeoError> {
        let path = self.setup_file_path(path);

        // Have to handle error mapping this way because of rust error: https://github.com/rust-lang/rust/issues/42424.
        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(e) => return Err(PackageError::io_error_input_file(eyre!(e), Backtrace::new()).into()),
        };

        // Have to handle error mapping this way because of rust error: https://github.com/rust-lang/rust/issues/42424.
        match file.write_all(self.template().as_bytes()) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(PackageError::io_error_input_file(eyre!(e), Backtrace::new()).into()),
        }
    }

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

    fn setup_file_path<'a>(&self, path: &'a Path) -> Cow<'a, Path> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            if !path.ends_with(INPUTS_DIRECTORY_NAME) {
                path.to_mut().push(INPUTS_DIRECTORY_NAME);
            }
            path.to_mut()
                .push(format!("{}{}", self.package_name, INPUT_FILE_EXTENSION));
        }
        path
    }
}
