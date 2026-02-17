// Copyright (C) 2019-2026 Provable Inc.
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

use anyhow::Result;
use std::path::{Path, PathBuf};

/// Returns true if `name` matches a known devnet artifact directory prefix.
fn is_devnet_artifact(name: &str) -> bool {
    name.starts_with("ledger-") || name.starts_with("node-data-") || name.starts_with(".logs-")
}

/// Discover devnet artifact directories under `storage`.
pub fn find_devnet_artifacts(storage: &Path) -> Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();
    for entry in std::fs::read_dir(storage)? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if entry.file_type()?.is_dir() && is_devnet_artifact(&name) {
            dirs.push(entry.path());
        }
    }
    dirs.sort();
    Ok(dirs)
}
