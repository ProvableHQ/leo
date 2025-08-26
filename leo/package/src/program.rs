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

use crate::{MAX_PROGRAM_SIZE, *};

use leo_errors::{PackageError, Result, UtilError};
use leo_span::Symbol;

use snarkvm::prelude::{Program as SvmProgram, TestnetV0};

use indexmap::{IndexMap, IndexSet};
use std::path::Path;

/// Information about an Aleo program.
#[derive(Clone, Debug)]
pub struct Program {
    // The name of the program (no ".aleo" suffix).
    pub name: Symbol,
    pub data: ProgramData,
    pub edition: Option<u16>,
    pub dependencies: IndexSet<Dependency>,
    pub is_local: bool,
    pub is_test: bool,
}

impl Program {
    /// Given the location `path` of a `.aleo` file, read the filesystem
    /// to obtain a `Program`.
    pub fn from_aleo_path<P: AsRef<Path>>(name: Symbol, path: P, map: &IndexMap<Symbol, Dependency>) -> Result<Self> {
        Self::from_aleo_path_impl(name, path.as_ref(), map)
    }

    fn from_aleo_path_impl(name: Symbol, path: &Path, map: &IndexMap<Symbol, Dependency>) -> Result<Self> {
        let bytecode = std::fs::read_to_string(path).map_err(|e| {
            UtilError::util_file_io_error(format_args!("Trying to read aleo file at {}", path.display()), e)
        })?;

        let dependencies = parse_dependencies_from_aleo(name, &bytecode, map)?;

        Ok(Program {
            name,
            data: ProgramData::Bytecode(bytecode),
            edition: None,
            dependencies,
            is_local: true,
            is_test: false,
        })
    }

    /// Given the location `path` of a local Leo package, read the filesystem
    /// to obtain a `Program`.
    pub fn from_package_path<P: AsRef<Path>>(name: Symbol, path: P) -> Result<Self> {
        Self::from_package_path_impl(name, path.as_ref())
    }

    fn from_package_path_impl(name: Symbol, path: &Path) -> Result<Self> {
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
            data: ProgramData::SourcePath { directory: path.to_path_buf(), source: source_path },
            edition: None,
            dependencies: manifest
                .dependencies
                .unwrap_or_default()
                .into_iter()
                .map(|dependency| canonicalize_dependency_path_relative_to(path, dependency))
                .collect::<Result<IndexSet<_>, _>>()?,
            is_local: true,
            is_test: false,
        })
    }

    /// Given the path to the source file of a test, create a `Program`.
    ///
    /// Unlike `Program::from_package_path`, the path is to the source file,
    /// and the name of the program is determined from the filename.
    ///
    /// `main_program` must be provided since every test is dependent on it.
    pub fn from_test_path<P: AsRef<Path>>(source_path: P, main_program: Dependency) -> Result<Self> {
        Self::from_path_test_impl(source_path.as_ref(), main_program)
    }

    fn from_path_test_impl(source_path: &Path, main_program: Dependency) -> Result<Self> {
        let name = filename_no_leo_extension(source_path)
            .ok_or_else(|| PackageError::failed_path(source_path.display(), ""))?;
        let test_directory = source_path.parent().ok_or_else(|| {
            UtilError::failed_to_open_file(format_args!("Failed to find directory for test {}", source_path.display()))
        })?;
        let package_directory = test_directory.parent().ok_or_else(|| {
            UtilError::failed_to_open_file(format_args!("Failed to find package for test {}", source_path.display()))
        })?;
        let manifest = Manifest::read_from_file(package_directory.join(MANIFEST_FILENAME))?;
        let mut dependencies = manifest
            .dev_dependencies
            .unwrap_or_default()
            .into_iter()
            .map(|dependency| canonicalize_dependency_path_relative_to(package_directory, dependency))
            .collect::<Result<IndexSet<_>, _>>()?;
        dependencies.insert(main_program);

        Ok(Program {
            name: Symbol::intern(name),
            edition: None,
            data: ProgramData::SourcePath {
                directory: test_directory.to_path_buf(),
                source: source_path.to_path_buf(),
            },
            dependencies,
            is_local: true,
            is_test: true,
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
        no_cache: bool,
    ) -> Result<Self> {
        Self::fetch_impl(name, edition, home_path.as_ref(), network, endpoint, no_cache)
    }

    fn fetch_impl(
        name: Symbol,
        edition: Option<u16>,
        home_path: &Path,
        network: NetworkName,
        endpoint: &str,
        no_cache: bool,
    ) -> Result<Self> {
        // It's not a local program; let's check the cache.
        let cache_directory = home_path.join(format!("registry/{network}"));

        // If the edition is not specified, then query the network for the latest edition.
        let edition = match edition {
            _ if name == Symbol::intern("credits") => Ok(0), // Credits program always has edition 0.
            Some(edition) => Ok(edition),
            None => {
                if name == Symbol::intern("credits") {
                    // credits.aleo is always edition 0 and fetching from the network won't work.
                    Ok(0)
                } else {
                    let url = format!("{endpoint}/{network}/program/{name}.aleo/latest_edition");
                    fetch_from_network(&url).and_then(|contents| {
                        contents.parse::<u16>().map_err(|e| {
                            UtilError::failed_to_retrieve_from_endpoint(
                                url,
                                format!("Failed to parse edition as u16: {e}"),
                            )
                        })
                    })
                }
            }
        };

        // If we failed to get the edition, default to 0.
        let edition = edition.unwrap_or_else(|err| {
            println!("Warning: Could not fetch edition for program `{name}`: {err}. Defaulting to edition 0.");
            0
        });

        // Define the full cache path for the program.
        let cache_directory = cache_directory.join(format!("{name}/{edition}"));
        let full_cache_path = cache_directory.join(format!("{name}.aleo"));
        if !cache_directory.exists() {
            // Create directory if it doesn't exist.
            std::fs::create_dir_all(&cache_directory).map_err(|err| {
                UtilError::util_file_io_error(format!("Could not write path {}", cache_directory.display()), err)
            })?;
        }

        // Get the existing bytecode if the file exists.
        let existing_bytecode = match full_cache_path.exists() {
            false => None,
            true => {
                let existing_contents = std::fs::read_to_string(&full_cache_path).map_err(|e| {
                    UtilError::util_file_io_error(
                        format_args!("Trying to read cached file at {}", full_cache_path.display()),
                        e,
                    )
                })?;
                Some(existing_contents)
            }
        };

        let bytecode = match (existing_bytecode, no_cache) {
            // If we are using the cache, we can just return the bytecode.
            (Some(bytecode), false) => bytecode,
            // Otherwise, we need to fetch it from the network.
            (existing, _) => {
                // Define the primary URL to fetch the program from.
                let primary_url = if name == Symbol::intern("credits") {
                    format!("{endpoint}/{network}/program/credits.aleo")
                } else {
                    format!("{endpoint}/{network}/program/{name}.aleo/{edition}")
                };
                let secondary_url = format!("{endpoint}/{network}/program/{name}.aleo");
                let contents = fetch_from_network(&primary_url)
                    .or_else(|_| fetch_from_network(&secondary_url))
                    .map_err(|err| {
                        UtilError::failed_to_retrieve_from_endpoint(
                            primary_url,
                            format_args!("Failed to fetch program `{name}` from network `{network}`: {err}"),
                        )
                    })?;

                // If the file already exists, compare it to the new contents.
                if let Some(existing_contents) = existing {
                    if existing_contents != contents {
                        println!(
                            "Warning: The cached file at `{}` is different from the one fetched from the network. The cached file will be overwritten.",
                            full_cache_path.display()
                        );
                    }
                }

                // Write the bytecode to the cache.
                std::fs::write(&full_cache_path, &contents).map_err(|err| {
                    UtilError::util_file_io_error(
                        format_args!("Could not open file `{}`", full_cache_path.display()),
                        err,
                    )
                })?;

                contents
            }
        };

        let dependencies = parse_dependencies_from_aleo(name, &bytecode, &IndexMap::new())?;

        Ok(Program {
            name,
            data: ProgramData::Bytecode(bytecode),
            edition: Some(edition),
            dependencies,
            is_local: false,
            is_test: false,
        })
    }
}

/// If `dependency` has a relative path, assume it's relative to `base` and canonicalize it.
///
/// This needs to be done when collecting local dependencies from manifests which
/// may be located at different places on the file system.
fn canonicalize_dependency_path_relative_to(base: &Path, mut dependency: Dependency) -> Result<Dependency> {
    if let Some(path) = &mut dependency.path {
        if !path.is_absolute() {
            let joined = base.join(&path);
            *path = joined.canonicalize().map_err(|e| PackageError::failed_path(joined.display(), e))?;
        }
    }
    Ok(dependency)
}

/// Parse the `.aleo` file's imports and construct `Dependency`s.
fn parse_dependencies_from_aleo(
    name: Symbol,
    bytecode: &str,
    existing: &IndexMap<Symbol, Dependency>,
) -> Result<IndexSet<Dependency>> {
    // Check if the program size exceeds the maximum allowed limit.
    let program_size = bytecode.len();

    if program_size > MAX_PROGRAM_SIZE {
        return Err(leo_errors::LeoError::UtilError(UtilError::program_size_limit_exceeded(
            name,
            program_size,
            MAX_PROGRAM_SIZE,
        )));
    }

    // Parse the bytecode into an SVM program.
    let svm_program: SvmProgram<TestnetV0> = bytecode.parse().map_err(|_| UtilError::snarkvm_parsing_error(name))?;
    let dependencies = svm_program
        .imports()
        .keys()
        .map(|program_id| {
            // If the dependency already exists, use it.
            // Otherwise, assume it's a network dependency.
            if let Some(dependency) = existing.get(&Symbol::intern(&program_id.name().to_string())) {
                dependency.clone()
            } else {
                let name = program_id.to_string();
                Dependency { name, location: Location::Network, path: None, edition: None }
            }
        })
        .collect();
    Ok(dependencies)
}
