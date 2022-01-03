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

use leo_errors::{PackageError, Result};

use crate::PackageDirectory;

use std::{borrow::Cow, fs, path::Path};

pub struct ImportsDirectory;

impl PackageDirectory for ImportsDirectory {
    const NAME: &'static str = "imports/";
}

impl ImportsDirectory {
    /// Removes an imported package in the imports directory at the provided path.
    pub fn remove_import(path: &Path, package_name: &str) -> Result<()> {
        let mut path = Cow::from(path);
        if path.is_dir() && !path.ends_with(Self::NAME) {
            path.to_mut().push(Self::NAME);
        }

        path.to_mut().push(package_name);

        if !path.exists() || !path.is_dir() {
            return Err(PackageError::import_does_not_exist(package_name).into());
        }

        fs::remove_dir_all(&path).map_err(PackageError::failed_to_remove_imports_directory)?;

        Ok(())
    }
}
