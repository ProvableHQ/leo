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

use aleo_std;
use leo_errors::{CliError, Result};
use leo_package::Manifest;

use aleo_std::aleo_dir;
use std::{env::current_dir, path::PathBuf};

/// Project context, manifest, current directory etc
/// All the info that is relevant in most of the commands
// TODO: Make `path` and `home` not pub, to prevent misuse through direct access.
#[derive(Clone)]
pub struct Context {
    /// Path at which the command is called, None when default
    pub path: Option<PathBuf>,
    /// Path to use for the Aleo registry, None when default
    pub home: Option<PathBuf>,
    /// Recursive flag.
    // TODO: Shift from callee to caller by including display method
    pub recursive: bool,
}

impl Context {
    pub fn new(path: Option<PathBuf>, home: Option<PathBuf>, recursive: bool) -> Result<Context> {
        Ok(Context { path, home, recursive })
    }

    /// Returns the path of the parent directory to the Leo package.
    pub fn parent_dir(&self) -> Result<PathBuf> {
        match &self.path {
            Some(path) => {
                let mut path = path.clone();
                path.pop();
                Ok(path)
            }
            None => Ok(current_dir().map_err(CliError::cli_io_error)?),
        }
    }

    /// Returns the path to the Leo package.
    pub fn dir(&self) -> Result<PathBuf> {
        match &self.path {
            Some(path) => Ok(path.clone()),
            None => Ok(current_dir().map_err(CliError::cli_io_error)?),
        }
    }

    /// Returns the path to the Aleo registry directory.
    pub fn home(&self) -> Result<PathBuf> {
        match &self.home {
            Some(path) => Ok(path.clone()),
            None => Ok(aleo_dir()),
        }
    }

    /// Opens the manifest file `program.json`.
    pub fn open_manifest(&self) -> Result<Manifest> {
        let path = self.dir()?;
        let manifest_path = path.join(leo_package::MANIFEST_FILENAME);
        let manifest = Manifest::read_from_file(manifest_path)?;
        Ok(manifest)
    }
}
