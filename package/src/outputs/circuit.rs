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

//! The serialized circuit output file.

use serde::Deserialize;

use crate::PackageFile;

#[derive(Deserialize)]
pub struct CircuitFile {
    pub package_name: String,
}

impl CircuitFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }
}

impl PackageFile for CircuitFile {
    type ParentDirectory = super::OutputsDirectory;

    fn template(&self) -> String {
        unimplemented!("CircuitFile doesn't have a template.");
    }
}

impl std::fmt::Display for CircuitFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.json", self.package_name)
    }
}
