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

//! The proof file.

use crate::PackageFile;

use serde::Deserialize;

pub static PROOF_FILE_EXTENSION: &str = ".proof";

#[derive(Deserialize)]
pub struct ProofFile {
    pub package_name: String,
}

impl ProofFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }
}

impl PackageFile for ProofFile {
    type ParentDirectory = super::OutputsDirectory;

    fn template(&self) -> String {
        unimplemented!("PackageFile doesn't have a template.");
    }
}

impl std::fmt::Display for ProofFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.proof", self.package_name)
    }
}
