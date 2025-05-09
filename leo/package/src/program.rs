// Copyright (C) 2019-2025 Provable Inc.
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

use crate::*;

use leo_errors::{PackageError, Result, UtilError};
use leo_span::Symbol;

use snarkvm::prelude::{Program as SvmProgram, TestnetV0};

use indexmap::IndexSet;
use std::path::Path;

/// Information about an Aleo program.
#[derive(Clone, Debug)]
pub struct Program {
    // The name of the program (no ".aleo" suffix).
    pub name: Symbol,
    pub data: ProgramData,
    pub edition: Option<u16>,
    pub dependencies: IndexSet<Dependency>,
}

impl Program {
    /// Given the location `path` of a local Leo package, read the filesystem
    /// to obtain a `Program`.
    /// Note: Local programs do not have an edition since the edition is assigned on deployment to the network.
    pub fn from_path<P: AsRef<Path>>(name: Symbol, path: P) -> Result<Self> {
        Self::from_path_impl(name, path.as_ref())
    }

    fn from_path_impl(name: Symbol, path: &Path) -> Result<Self> {
        let manifest = Manifest::read_from_file(path.join(MANIFEST_FILENAME))?;
        let manifest_symbol = crate::symbol(&manifest.program)?;
        if name != manifest_symbol {
            return Err(PackageError::conflicting_manifest(
                format_args!("{name}.aleo"),
                format_args!("{manifest_symbol}.aleo"),
            )
            .into());
        }
        let source_directory = path.join(SOURCE_DIRECTORY);
        let count = source_directory
            .read_dir()
            .map_err(|e| {
                UtilError::util_file_io_error(
                    format_args!("Failed to read directory {}", source_directory.display()),
                    e,
                )
            })?
            .count();

        let source_path = source_directory.join(MAIN_FILENAME);

        if !source_path.exists() || count != 1 {
            return Err(PackageError::source_directory_can_contain_only_one_file(source_directory.display()).into());
        }

        Ok(Program {
            name,
            data: ProgramData::SourcePath(source_path),
            edition: None,
            dependencies: manifest.dependencies.unwrap_or_default().into_iter().collect(),
        })
    }

    /// Given an Aleo program on a network, fetch it to build a `Program`.
    /// If no edition is found, the latest edition is pulled from the network.
    pub fn fetch<P: AsRef<Path>>(
        name: Symbol,
        edition: Option<u16>,
        home_path: P,
        network: NetworkName,
        endpoint: &str,
        use_cache: bool,
    ) -> Result<Self> {
        Self::fetch_impl(name, edition, home_path.as_ref(), network, endpoint, use_cache)
    }

    fn fetch_impl(
        name: Symbol,
        edition: Option<u16>,
        home_path: &Path,
        network: NetworkName,
        endpoint: &str,
        use_cache: bool,
    ) -> Result<Self> {
        // It's not a local program; let's check the cache.
        let cache_directory = home_path.join(format!("registry/{network}"));

        // If the edition is not specified, then query the network for the latest edition.
        let edition = match edition {
            Some(edition) => edition,
            None => {
                let url = format!("{endpoint}/{network}/program/{name}.aleo/latest_edition");
                let contents = fetch_from_network(&url)?;
                contents.parse::<u16>().map_err(|e| {
                    UtilError::failed_to_retrieve_from_endpoint(
                        format!("Failed to parse edition as u16: {e}"),
                        Default::default(),
                    )
                })?
            }
        };

        let full_cache_path = cache_directory.join(format!("{name}.aleo/{edition}"));

        let bytecode = if full_cache_path.exists() && use_cache {
            // Great; apparently this file is already cached.
            std::fs::read_to_string(&full_cache_path).map_err(|e| {
                UtilError::util_file_io_error(
                    format_args!("Trying to read cached file at {}", full_cache_path.display()),
                    e,
                )
            })?
        } else {
            // We need to fetch it from the network.
            let url = format!("{endpoint}/{network}/program/{name}.aleo/{edition}");
            let contents = fetch_from_network(&url)?;

            // Make sure the cache directory exists.
            std::fs::create_dir_all(&cache_directory).map_err(|e| {
                UtilError::util_file_io_error(
                    format_args!("Could not create directory `{}`", cache_directory.display()),
                    e,
                )
            })?;

            // Write the bytecode to the cache.
            std::fs::write(&full_cache_path, &contents).map_err(|err| {
                UtilError::util_file_io_error(format_args!("Could not open file `{}`", full_cache_path.display()), err)
            })?;

            contents
        };

        // Parse the program so we can get its imports.
        let svm_program: SvmProgram<TestnetV0> =
            bytecode.parse().map_err(|_| UtilError::snarkvm_parsing_error(name))?;
        let dependencies = svm_program
            .imports()
            .keys()
            .map(|program_id| {
                let name = program_id.to_string();
                Dependency { name, location: Location::Network, path: None, edition: Some(edition) }
            })
            .collect();

        Ok(Program { name, data: ProgramData::Bytecode(bytecode), edition: Some(edition), dependencies })
    }
}
