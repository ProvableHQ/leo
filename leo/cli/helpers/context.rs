// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use super::*;
use leo_errors::{CliError, PackageError, Result};
use leo_package::build::{BuildDirectory, BUILD_DIRECTORY_NAME};

use snarkvm::file::Manifest;

use std::{
    env::current_dir,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

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

    /// Returns the package name as a String.
    /// Opens the manifest file `program.json` and creates the build directory if it doesn't exist.
    pub fn open_manifest(&self) -> Result<Manifest<CurrentNetwork>> {
        // Open the manifest file.
        let path = self.dir()?;
        let manifest = Manifest::<CurrentNetwork>::open(&path).map_err(PackageError::failed_to_open_manifest)?;

        // Lookup the program id.
        // let program_id = manifest.program_id();

        // Create the Leo build/ directory if it doesn't exist.
        let build_path = path.join(Path::new(BUILD_DIRECTORY_NAME));
        if !build_path.exists() {
            BuildDirectory::create(&build_path)?;
        }

        // Mirror the program.json file in the Leo build/ directory for Aleo SDK compilation.

        // Read the manifest file to string.
        let manifest_string =
            std::fs::read_to_string(manifest.path()).map_err(PackageError::failed_to_open_manifest)?;

        // Construct the file path.
        let build_manifest_path = build_path.join(Manifest::<CurrentNetwork>::file_name());

        // Write the file.
        File::create(build_manifest_path)
            .map_err(PackageError::failed_to_open_manifest)?
            .write_all(manifest_string.as_bytes())
            .map_err(PackageError::failed_to_open_manifest)?;

        // Get package name from program id.
        Ok(manifest)
    }
}
