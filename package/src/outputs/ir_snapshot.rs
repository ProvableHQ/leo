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

use crate::PackageFile;

use serde::Deserialize;
use std::fmt;

/// Enum to handle all 3 types of snapshots.
#[derive(Deserialize)]
pub enum IrSnapshot {
    Formatted,
    Input,
    Raw,
}

/// Display Snapshot file extension.
impl fmt::Display for IrSnapshot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Formatted => ".leo.ir.fmt",
                Self::Input => ".leo.ir.input",
                Self::Raw => ".leo.ir",
            }
        )
    }
}

/// IR Snapshot File wrapper, handles file logic.
#[derive(Deserialize)]
pub struct IrSnapshotFile {
    pub package_name: String,
    pub snapshot: IrSnapshot,
}

impl IrSnapshotFile {
    pub fn new(package_name: &str, snapshot: IrSnapshot) -> Self {
        Self {
            package_name: package_name.to_string(),
            snapshot,
        }
    }
}

impl PackageFile for IrSnapshotFile {
    type ParentDirectory = super::OutputsDirectory;

    fn template(&self) -> String {
        unimplemented!("IRSnapshotFile doesn't have a template.");
    }
}

impl fmt::Display for IrSnapshotFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.package_name, self.snapshot)
    }
}
