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

use leo_ast::DiGraph;
use leo_errors::{CliError, PackageError, Result, UtilError};
use leo_span::Symbol;

use indexmap::{IndexMap, map::Entry};
use snarkvm::prelude::anyhow;
use std::path::{Path, PathBuf};

/// Either the bytecode of an Aleo program (if it was a network dependency) or
/// a path to its source (if it was local).
#[derive(Clone, Debug)]
pub enum ProgramData {
    Bytecode(String),
    /// For a local dependency, `directory` is the directory of the package
    /// For a test dependency, `directory` is the directory of the test file.
    SourcePath {
        directory: PathBuf,
        source: PathBuf,
    },
}

/// A Leo package.
#[derive(Clone, Debug)]
pub struct Package {
    /// The directory on the filesystem where the package is located, canonicalized.
    pub base_directory: PathBuf,

    /// A topologically sorted list of all programs in this package, whether
    /// dependencies or the main program.
    ///
    /// Any program's dependent program will appear before it, so that compiling
    /// them in order should give access to all stubs necessary to compile each
    /// program.
    pub programs: Vec<Program>,

    /// The manifest file of this package.
    pub manifest: Manifest,

    /// The .env file of this package.
    pub env: Env,
}

impl Package {
    pub fn outputs_directory(&self) -> PathBuf {
        self.base_directory.join(OUTPUTS_DIRECTORY)
    }

    pub fn imports_directory(&self) -> PathBuf {
        self.base_directory.join(IMPORTS_DIRECTORY)
    }

    pub fn build_directory(&self) -> PathBuf {
        self.base_directory.join(BUILD_DIRECTORY)
    }

    pub fn source_directory(&self) -> PathBuf {
        self.base_directory.join(SOURCE_DIRECTORY)
    }

    pub fn tests_directory(&self) -> PathBuf {
        self.base_directory.join(TESTS_DIRECTORY)
    }

    /// Create a Leo package by the name `package_name` in a subdirectory of `path`.
    pub fn initialize<P: AsRef<Path>>(
        package_name: &str,
        path: P,
        network: NetworkName,
        endpoint: &str,
    ) -> Result<PathBuf> {
        Self::initialize_impl(package_name, path.as_ref(), network, endpoint)
    }

    fn initialize_impl(package_name: &str, path: &Path, network: NetworkName, endpoint: &str) -> Result<PathBuf> {
        let package_name =
            if package_name.ends_with(".aleo") { package_name.to_string() } else { format!("{package_name}.aleo") };

        if !crate::is_valid_aleo_name(&package_name) {
            return Err(CliError::invalid_program_name(package_name).into());
        }

        let path = path.canonicalize().map_err(|e| PackageError::failed_path(path.display(), e))?;
        let full_path = path.join(package_name.strip_suffix(".aleo").unwrap());

        // Verify that there is no existing directory at the path.
        if full_path.exists() {
            return Err(
                PackageError::failed_to_initialize_package(package_name, &path, "Directory already exists").into()
            );
        }

        // Create the package directory.
        std::fs::create_dir(&full_path)
            .map_err(|e| PackageError::failed_to_initialize_package(&package_name, &full_path, e))?;

        // Change the current working directory to the package directory.
        std::env::set_current_dir(&full_path)
            .map_err(|e| PackageError::failed_to_initialize_package(&package_name, &full_path, e))?;

        // Create the gitignore file.
        const GITIGNORE_TEMPLATE: &str = ".env\n*.avm\n*.prover\n*.verifier\noutputs/\n";

        const GITIGNORE_FILENAME: &str = ".gitignore";

        let gitignore_path = full_path.join(GITIGNORE_FILENAME);

        std::fs::write(gitignore_path, GITIGNORE_TEMPLATE).map_err(PackageError::io_error_gitignore_file)?;

        // Create the .env file.
        let env = Env { network, private_key: TEST_PRIVATE_KEY.to_string(), endpoint: endpoint.to_string() };

        let env_path = full_path.join(ENV_FILENAME);

        env.write_to_file(env_path)?;

        // Create the manifest.
        let manifest = Manifest {
            program: package_name.clone(),
            version: "0.1.0".to_string(),
            description: String::new(),
            license: "MIT".to_string(),
            leo: env!("CARGO_PKG_VERSION").to_string(),
            dependencies: None,
            dev_dependencies: None,
        };

        let manifest_path = full_path.join(MANIFEST_FILENAME);

        manifest.write_to_file(manifest_path)?;

        // Create the source directory.
        let source_path = full_path.join(SOURCE_DIRECTORY);

        std::fs::create_dir(&source_path)
            .map_err(|e| PackageError::failed_to_create_source_directory(source_path.display(), e))?;

        // Create the main.leo file.
        let main_path = source_path.join(MAIN_FILENAME);

        let name_no_aleo = package_name.strip_suffix(".aleo").unwrap();

        std::fs::write(&main_path, main_template(name_no_aleo))
            .map_err(|e| UtilError::util_file_io_error(format_args!("Failed to write `{}`", main_path.display()), e))?;

        // Create the tests directory.
        let tests_path = full_path.join(TESTS_DIRECTORY);

        std::fs::create_dir(&tests_path)
            .map_err(|e| PackageError::failed_to_create_source_directory(tests_path.display(), e))?;

        let test_file_path = tests_path.join(format!("test_{name_no_aleo}.leo"));
        std::fs::write(&test_file_path, test_template(name_no_aleo))
            .map_err(|e| UtilError::util_file_io_error(format_args!("Failed to write `{}`", main_path.display()), e))?;

        Ok(full_path)
    }

    /// Examine the Leo package at `path` to create a `Package`, but don't find dependencies.
    ///
    /// This may be useful if you just need other information like the manifest or env file.
    pub fn from_directory_no_graph<P: AsRef<Path>, Q: AsRef<Path>>(
        path: P,
        home_path: Q,
        network: NetworkName,
        endpoint: &str,
    ) -> Result<Self> {
        Self::from_directory_impl(
            path.as_ref(),
            home_path.as_ref(),
            /* build_graph */ false,
            /* with_tests */ false,
            /* no_cache */ false,
            /* no_local */ false,
            network,
            endpoint,
        )
    }

    /// Examine the Leo package at `path` to create a `Package`, including all its dependencies,
    /// obtaining dependencies from the file system or network and topologically sorting them.
    pub fn from_directory<P: AsRef<Path>, Q: AsRef<Path>>(
        path: P,
        home_path: Q,
        no_cache: bool,
        no_local: bool,
        network: NetworkName,
        endpoint: &str,
    ) -> Result<Self> {
        Self::from_directory_impl(
            path.as_ref(),
            home_path.as_ref(),
            /* build_graph */ true,
            /* with_tests */ false,
            no_cache,
            no_local,
            network,
            endpoint,
        )
    }

    /// Examine the Leo package at `path` to create a `Package`, including all its dependencies
    /// and its tests, obtaining dependencies from the file system or network and topologically sorting them.
    pub fn from_directory_with_tests<P: AsRef<Path>, Q: AsRef<Path>>(
        path: P,
        home_path: Q,
        no_cache: bool,
        no_local: bool,
        network: NetworkName,
        endpoint: &str,
    ) -> Result<Self> {
        Self::from_directory_impl(
            path.as_ref(),
            home_path.as_ref(),
            /* build_graph */ true,
            /* with_tests */ true,
            no_cache,
            no_local,
            network,
            endpoint,
        )
    }

    pub fn test_files(&self) -> impl Iterator<Item = PathBuf> {
        let path = self.tests_directory();
        // This allocation isn't ideal but it's not performance critical and
        // easily resolves lifetime issues.
        let data: Vec<PathBuf> = Self::files_with_extension(&path, "leo").collect();
        data.into_iter()
    }

    pub fn import_files(&self) -> impl Iterator<Item = PathBuf> {
        let path = self.imports_directory();
        // This allocation isn't ideal but it's not performance critical and
        // easily resolves lifetime issues.
        let data: Vec<PathBuf> = Self::files_with_extension(&path, "aleo").collect();
        data.into_iter()
    }

    fn files_with_extension(path: &Path, extension: &'static str) -> impl Iterator<Item = PathBuf> {
        path.read_dir()
            .ok()
            .into_iter()
            .flatten()
            .flat_map(|maybe_filename| maybe_filename.ok())
            .filter(|entry| entry.file_type().ok().map(|filetype| filetype.is_file()).unwrap_or(false))
            .flat_map(move |entry| {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == extension) { Some(path) } else { None }
            })
    }

    #[allow(clippy::too_many_arguments)]
    fn from_directory_impl(
        path: &Path,
        home_path: &Path,
        build_graph: bool,
        with_tests: bool,
        no_cache: bool,
        no_local: bool,
        network: NetworkName,
        endpoint: &str,
    ) -> Result<Self> {
        let map_err = |path: &Path, err| {
            UtilError::util_file_io_error(format_args!("Trying to find path at {}", path.display()), err)
        };

        let path = path.canonicalize().map_err(|err| map_err(path, err))?;

        let env = Env::read_from_file_or_environment(&path)?;

        let manifest = Manifest::read_from_file(path.join(MANIFEST_FILENAME))?;

        let programs: Vec<Program> = if build_graph {
            let home_path = home_path.canonicalize().map_err(|err| map_err(home_path, err))?;

            let mut map: IndexMap<Symbol, (Dependency, Program)> = IndexMap::new();

            let mut digraph = DiGraph::<Symbol>::new(Default::default());

            let first_dependency = Dependency {
                name: manifest.program.clone(),
                location: Location::Local,
                path: Some(path.clone()),
                edition: None,
            };

            let test_dependencies: Vec<Dependency> = if with_tests {
                let tests_directory = path.join(TESTS_DIRECTORY);
                let mut test_dependencies: Vec<Dependency> = Self::files_with_extension(&tests_directory, "leo")
                    .map(|path| Dependency {
                        // We just made sure it has a ".leo" extension.
                        name: format!("{}.aleo", crate::filename_no_leo_extension(&path).unwrap()),
                        edition: None,
                        location: Location::Test,
                        path: Some(path.to_path_buf()),
                    })
                    .collect();
                if let Some(deps) = manifest.dev_dependencies.as_ref() {
                    test_dependencies.extend(deps.iter().cloned());
                }
                test_dependencies
            } else {
                Vec::new()
            };

            for dependency in test_dependencies.into_iter().chain(std::iter::once(first_dependency.clone())) {
                Self::graph_build(
                    &home_path,
                    network,
                    endpoint,
                    &first_dependency,
                    dependency,
                    &mut map,
                    &mut digraph,
                    no_cache,
                    no_local,
                )?;
            }

            let ordered_dependency_symbols =
                digraph.post_order().map_err(|_| UtilError::circular_dependency_error())?;

            ordered_dependency_symbols.into_iter().map(|symbol| map.swap_remove(&symbol).unwrap().1).collect()
        } else {
            Vec::new()
        };

        Ok(Package { base_directory: path, programs, env, manifest })
    }

    #[allow(clippy::too_many_arguments)]
    fn graph_build(
        home_path: &Path,
        network: NetworkName,
        endpoint: &str,
        main_program: &Dependency,
        new: Dependency,
        map: &mut IndexMap<Symbol, (Dependency, Program)>,
        graph: &mut DiGraph<Symbol>,
        no_cache: bool,
        no_local: bool,
    ) -> Result<()> {
        let name_symbol = symbol(&new.name)?;

        // Get the existing dependencies.
        let dependencies = map.clone().into_iter().map(|(name, (dep, _))| (name, dep)).collect();

        let program = match map.entry(name_symbol) {
            Entry::Occupied(occupied) => {
                // We've already visited this dependency. Just make sure it's compatible with
                // the one we already have.
                let existing_dep = &occupied.get().0;
                assert_eq!(new.name, existing_dep.name);
                if new.location != existing_dep.location
                    || new.path != existing_dep.path
                    || new.edition != existing_dep.edition
                {
                    return Err(PackageError::conflicting_dependency(existing_dep, new).into());
                }
                return Ok(());
            }
            Entry::Vacant(vacant) => {
                let program = match (new.path.as_ref(), new.location) {
                    (Some(path), Location::Local) if !no_local => {
                        // It's a local dependency.
                        if path.extension().and_then(|p| p.to_str()) == Some("aleo") && path.is_file() {
                            Program::from_aleo_path(name_symbol, path, &dependencies)?
                        } else {
                            Program::from_package_path(name_symbol, path)?
                        }
                    }
                    (Some(path), Location::Test) => {
                        // It's a test dependency - the path points to the source file,
                        // not a package.
                        Program::from_test_path(path, main_program.clone())?
                    }
                    (_, Location::Network) | (Some(_), Location::Local) => {
                        // It's a network dependency.
                        Program::fetch(name_symbol, new.edition, home_path, network, endpoint, no_cache)?
                    }
                    _ => return Err(anyhow!("Invalid dependency data for {} (path must be given).", new.name).into()),
                };

                vacant.insert((new, program.clone()));

                program
            }
        };

        graph.add_node(name_symbol);

        for dependency in program.dependencies.iter() {
            let dependency_symbol = symbol(&dependency.name)?;
            graph.add_edge(name_symbol, dependency_symbol);
            Self::graph_build(
                home_path,
                network,
                endpoint,
                main_program,
                dependency.clone(),
                map,
                graph,
                no_cache,
                no_local,
            )?;
        }

        Ok(())
    }
}

fn main_template(name: &str) -> String {
    format!(
        r#"// The '{name}' program.
program {name}.aleo {{
    // This is the constructor for the program.
    // The constructor allows you to manage program upgrades.
    // It is called when the program is deployed or upgraded.
    // It is currently configured to **prevent** upgrades.
    // Other configurations include:
    //  - @admin(address="aleo1...")
    //  - @checksum(mapping="credits.aleo/fixme", key="0field")
    //  - @custom
    // For more information, please refer to the documentation: `https://docs.leo-lang.org/guides/upgradability`
    @noupgrade
    async constructor() {{}}

    transition main(public a: u32, b: u32) -> u32 {{
        let c: u32 = a + b;
        return c;
    }}
}}
"#
    )
}

fn test_template(name: &str) -> String {
    format!(
        r#"// The 'test_{name}' test program.
import {name}.aleo;
program test_{name}.aleo {{
    @test
    script test_it() {{
        let result: u32 = {name}.aleo/main(1u32, 2u32);
        assert_eq(result, 3u32);
    }}

    @test
    @should_fail
    transition do_nothing() {{
        let result: u32 = {name}.aleo/main(2u32, 3u32);
        assert_eq(result, 3u32);
    }}

    @noupgrade
    async constructor() {{}}
}}
"#
    )
}
