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
use aleo_std;
use leo_errors::{CliError, PackageError, Result};
use leo_package::build::{BuildDirectory, BUILD_DIRECTORY_NAME};
use leo_retriever::LockFileEntry;

use snarkvm::file::Manifest;

use aleo_std::aleo_dir;
use indexmap::IndexMap;
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
    /// Path to use for the Aleo registry, None when default
    pub home: Option<PathBuf>,
}

impl Context {
    pub fn new(path: Option<PathBuf>, home: Option<PathBuf>) -> Result<Context> {
        Ok(Context { path, home })
    }

    /// Returns the path of the parent directory to the Leo package.
    pub fn parent_dir(&self) -> Result<PathBuf> {
        match &self.path {
            Some(ref path) => {
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
            std::fs::read_to_string(manifest.path()).map_err(PackageError::failed_to_read_manifest)?;

        // Construct the file path.
        let build_manifest_path = build_path.join(Manifest::<CurrentNetwork>::file_name());

        // Write the file.
        File::create(build_manifest_path)
            .map_err(PackageError::failed_to_create_manifest)?
            .write_all(manifest_string.as_bytes())
            .map_err(PackageError::failed_to_write_manifest)?;

        // Get package name from program id.
        Ok(manifest)
    }

    /// Returns a post ordering of the local dependencies.
    /// Found by reading the lock file `leo.lock`.
    pub fn local_dependency_paths(&self) -> Result<Vec<(String, PathBuf)>> {
        let path = self.dir()?;
        let lock_path = path.join("leo.lock");

        // If there is no lock file can assume no local dependencies
        if !lock_path.exists() {
            return Ok(Vec::new());
        }

        let contents = std::fs::read_to_string(&lock_path)
            .map_err(|err| PackageError::failed_to_read_file(lock_path.to_str().unwrap(), err))?;

        let entry_map: IndexMap<String, Vec<LockFileEntry>> =
            toml::from_str(&contents).map_err(PackageError::failed_to_deserialize_lock_file)?;

        let lock_entries = entry_map.get("package").ok_or_else(PackageError::invalid_lock_file_formatting)?;

        let list: Vec<(String, PathBuf)> = lock_entries
            .iter()
            .filter_map(|entry| {
                entry.path().map(|local_path| (entry.name().to_string(), local_path.clone().join("build")))
            })
            .collect();

        Ok(list)
    }
}
