// Copyright (C) 2019-2024 Aleo Systems Inc.
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

use crate::parse_file_paths;

use leo_errors::{PackageError, Result};

use crate::source::MAIN_FILENAME;
use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
};

pub static TEST_DIRECTORY_NAME: &str = "tests/";

pub struct TestDirectory;

impl TestDirectory {
    /// Creates a directory at the provided path with the default directory name.
    pub fn create(path: &Path) -> Result<()> {
        let mut path = Cow::from(path);
        if path.is_dir() && !path.ends_with(TEST_DIRECTORY_NAME) {
            path.to_mut().push(TEST_DIRECTORY_NAME);
        }

        fs::create_dir_all(&path).map_err(PackageError::failed_to_create_test_directory)?;
        Ok(())
    }

    /// Returns a list of files in the test directory.
    pub fn files(path: &Path) -> Result<Vec<PathBuf>> {
        let mut path = Cow::from(path);
        if path.is_dir() && !path.ends_with(TEST_DIRECTORY_NAME) {
            path.to_mut().push(TEST_DIRECTORY_NAME);
        }

        let directory = fs::read_dir(&path).map_err(|err| PackageError::failed_to_read_file(path.display(), err))?;
        let mut file_paths = Vec::new();

        parse_file_paths(directory, &mut file_paths)?;

        Ok(file_paths)
    }
}
