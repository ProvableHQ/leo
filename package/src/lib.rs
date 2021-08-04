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

pub mod imports;
pub mod inputs;
pub mod outputs;
pub mod package;
pub mod root;
pub mod source;

use std::path::Path;

use leo_errors::Result;

pub struct LeoPackage;

impl LeoPackage {
    /// Initializes a Leo package at the given path.
    pub fn initialize(package_name: &str, path: &Path, author: Option<String>) -> Result<()> {
        package::Package::initialize(package_name, path, author)
    }

    /// Returns `true` if the given Leo package name is valid.
    pub fn is_package_name_valid(package_name: &str) -> bool {
        package::Package::is_package_name_valid(package_name)
    }

    /// Removes an imported Leo package
    pub fn remove_imported_package(package_name: &str, path: &Path) -> Result<()> {
        package::Package::remove_imported_package(package_name, path)
    }
}
