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

//! The `program.out` file.

use leo_errors::{CompilerError, Result};

use std::{
    borrow::Cow,
    fs::{
        File, {self},
    },
    io::Write,
    path::Path,
};

pub static OUTPUTS_DIRECTORY_NAME: &str = "outputs/";
pub static OUTPUT_FILE_EXTENSION: &str = ".out";

pub struct OutputFile {
    pub package_name: String,
}

impl OutputFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    /// Writes output to a file.
    pub fn write(&self, path: &Path, bytes: &[u8]) -> Result<()> {
        // create output file
        let path = self.setup_file_path(path);
        let mut file = File::create(&path).map_err(CompilerError::output_file_io_error)?;

        Ok(file.write_all(bytes).map_err(CompilerError::output_file_io_error)?)
    }

    /// Removes the output file at the given path if it exists. Returns `true` on success,
    /// `false` if the file doesn't exist, and `Error` if the file system fails during operation.
    pub fn remove(&self, path: &Path) -> Result<bool> {
        let path = self.setup_file_path(path);
        if !path.exists() {
            return Ok(false);
        }

        fs::remove_file(&path).map_err(|_| CompilerError::output_file_cannot_remove(path))?;
        Ok(true)
    }

    fn setup_file_path<'a>(&self, path: &'a Path) -> Cow<'a, Path> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            if !path.ends_with(OUTPUTS_DIRECTORY_NAME) {
                path.to_mut().push(OUTPUTS_DIRECTORY_NAME);
            }
            path.to_mut()
                .push(format!("{}{}", self.package_name, OUTPUT_FILE_EXTENSION));
        }
        path
    }
}

#[cfg(test)]
mod test_output_file {
    use crate::{OutputFile, OUTPUTS_DIRECTORY_NAME};
    use std::{error::Error, fs};

    #[test]
    fn test_all() -> Result<(), Box<dyn Error>> {
        let dir = tempfile::tempdir()?;
        let file = OutputFile::new("test");
        let path = dir.path();

        assert!(file.write(path, Default::default()).is_err());
        assert!(!(file.remove(path)?));

        fs::create_dir(dir.path().join(OUTPUTS_DIRECTORY_NAME))?;

        assert!(file.write(path, Default::default()).is_ok());
        assert!(file.remove(path)?);

        Ok(())
    }
}
