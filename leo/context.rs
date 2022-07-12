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

use crate::commands::Network;
use leo_errors::{CliError, Result};
use snarkvm::file::Manifest;

use std::{env::current_dir, path::PathBuf};

/// Project context, manifest, current directory etc
/// All the info that is relevant in most of the commands
#[derive(Clone)]
pub struct Context {
    /// Path at which the command is called, None when default
    pub path: Option<PathBuf>,
}

impl Context {
    pub fn new(path: Option<PathBuf>) -> Result<Context> {
        Ok(Context { path })
    }

    /// Returns the path to the Leo package.
    pub fn dir(&self) -> Result<PathBuf> {
        match &self.path {
            Some(path) => Ok(path.clone()),
            None => Ok(current_dir().map_err(CliError::cli_io_error)?),
        }
    }

    /// Returns the program name as a String.
    pub fn program_name(&self) -> Result<String> {
        // Open the manifest file.
        let path = self.dir()?;
        let manifest = Manifest::<Network>::open(&path).map_err(CliError::failed_to_open_manifest)?;

        // Lookup the program id.
        let program_id = manifest.program_id();

        // Get package name from program id.
        Ok(program_id.name().to_string())
    }
}
