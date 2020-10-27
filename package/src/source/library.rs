// Copyright (C) 2019-2020 Aleo Systems Inc.
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

//! The `lib.leo` file.

use crate::{errors::LibraryFileError, source::directory::SOURCE_DIRECTORY_NAME};

use serde::Deserialize;
use std::{borrow::Cow, fs::File, io::Write, path::Path};

pub static LIBRARY_FILENAME: &str = "lib.leo";

#[derive(Deserialize)]
pub struct LibraryFile {
    pub package_name: String,
}

impl LibraryFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn filename() -> String {
        format!("{}{}", SOURCE_DIRECTORY_NAME, LIBRARY_FILENAME)
    }

    pub fn exists_at(path: &Path) -> bool {
        let mut path = Cow::from(path);
        if path.is_dir() {
            if !path.ends_with(SOURCE_DIRECTORY_NAME) {
                path.to_mut().push(SOURCE_DIRECTORY_NAME);
            }
            path.to_mut().push(LIBRARY_FILENAME);
        }
        path.exists()
    }

    pub fn write_to(self, path: &Path) -> Result<(), LibraryFileError> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            if !path.ends_with(SOURCE_DIRECTORY_NAME) {
                path.to_mut().push(SOURCE_DIRECTORY_NAME);
            }
            path.to_mut().push(LIBRARY_FILENAME);
        }

        let mut file = File::create(&path)?;
        Ok(file.write_all(self.template().as_bytes())?)
    }

    fn template(&self) -> String {
        format!(
            r#"// The '{}' library circuit.
circuit Foo {{
    a: field
}}
"#,
            self.package_name
        )
    }
}
