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

//! The `main.leo` file.

use crate::source::directory::SOURCE_DIRECTORY_NAME;
use leo_errors::{PackageError, Result};

use serde::Deserialize;
use std::{borrow::Cow, fs::File, io::Write, path::Path};

pub static MAIN_FILENAME: &str = "main.leo";

#[derive(Deserialize)]
pub struct MainFile {
    pub package_name: String,
}

impl MainFile {
    pub fn new(package_name: &str) -> Self {
        Self { package_name: package_name.to_string() }
    }

    pub fn filename() -> String {
        format!("{SOURCE_DIRECTORY_NAME}{MAIN_FILENAME}")
    }

    pub fn exists_at(path: &Path) -> bool {
        let mut path = Cow::from(path);
        if path.is_dir() {
            if !path.ends_with(SOURCE_DIRECTORY_NAME) {
                path.to_mut().push(SOURCE_DIRECTORY_NAME);
            }
            path.to_mut().push(MAIN_FILENAME);
        }
        path.exists()
    }

    pub fn write_to(self, path: &Path) -> Result<()> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            if !path.ends_with(SOURCE_DIRECTORY_NAME) {
                path.to_mut().push(SOURCE_DIRECTORY_NAME);
            }
            path.to_mut().push(MAIN_FILENAME);
        }

        let mut file = File::create(&path).map_err(PackageError::io_error_main_file)?;
        Ok(file.write_all(self.template().as_bytes()).map_err(PackageError::io_error_main_file)?)
    }

    // TODO: Generalize to other networks.
    fn template(&self) -> String {
        format!(
            r#"// The '{}' program.
program {}.aleo {{
    transition main(public a: u32, b: u32) -> u32 {{
        let c: u32 = a + b;
        return c;
    }}
}}
"#,
            self.package_name, self.package_name
        )
    }
}
