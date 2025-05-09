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
use std::path::{Path, PathBuf};

/// Either the bytecode of an Aleo program (if it was a network dependency) or
/// a path to its source (if it was local).
#[derive(Clone, Debug)]
pub enum ProgramData {
    Bytecode(String),
    SourcePath(PathBuf),
}

/// A Leo package.
#[derive(Clone, Debug)]
pub struct Package {
    /// The directory on the filesystem where the package is located.
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
            dependencies: None,
            upgrade: Some(UpgradeConfig::NoUpgrade),
        };

        let manifest_path = full_path.join(MANIFEST_FILENAME);

        manifest.write_to_file(manifest_path)?;

        // Create the source directory.
        let source_path = full_path.join(SOURCE_DIRECTORY);

        std::fs::create_dir(&source_path)
            .map_err(|e| PackageError::failed_to_create_source_directory(source_path.display(), e))?;

        // Create the main.leo file.
        let main_path = source_path.join(MAIN_FILENAME);

        std::fs::write(&main_path, main_template(package_name.strip_suffix(".aleo").unwrap()))
            .map_err(|e| UtilError::util_file_io_error(format_args!("Failed to write `{}`", main_path.display()), e))?;

        Ok(full_path)
    }

    /// Examine the Leo package at `path` to create a `Package`, but don't find dependencies.
    ///
    /// This may be useful if you just need other information like the manifest or env file.
    pub fn from_directory_no_graph<P: AsRef<Path>, Q: AsRef<Path>>(path: P, home_path: Q) -> Result<Self> {
        Self::from_directory_impl(path.as_ref(), home_path.as_ref(), /* build_graph */ false)
    }

    /// Examine the Leo package at `path` to create a `Package`, including all its dependencies,
    /// obtaining dependencies from the file system or network and topologically sorting them.
    pub fn from_directory<P: AsRef<Path>, Q: AsRef<Path>>(path: P, home_path: Q) -> Result<Self> {
        Self::from_directory_impl(path.as_ref(), home_path.as_ref(), /* build_graph */ true)
    }

    fn from_directory_impl(path: &Path, home_path: &Path, build_graph: bool) -> Result<Self> {
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

            Self::graph_build(&home_path, env.network, &env.endpoint, first_dependency, &mut map, &mut digraph)?;

            let ordered_dependency_symbols =
                digraph.post_order().map_err(|_| UtilError::circular_dependency_error())?;

            ordered_dependency_symbols.into_iter().map(|symbol| map.swap_remove(&symbol).unwrap().1).collect()
        } else {
            Vec::new()
        };

        Ok(Package { base_directory: path, programs, env, manifest })
    }

    fn graph_build(
        home_path: &Path,
        network: NetworkName,
        endpoint: &str,
        new: Dependency,
        map: &mut IndexMap<Symbol, (Dependency, Program)>,
        graph: &mut DiGraph<Symbol>,
    ) -> Result<()> {
        let name_symbol = crate::symbol(&new.name)?;

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
                    return Err(PackageError::conflicting_dependency(format_args!("{name_symbol}.aleo")).into());
                }
                return Ok(());
            }
            Entry::Vacant(vacant) => {
                let program = if let Some(path) = new.path.as_ref() {
                    // It's a local dependency.
                    Program::from_path(name_symbol, path.clone())?
                } else {
                    // It's a network dependency.
                    Program::fetch(name_symbol, new.edition, home_path, network, endpoint, true)?
                };

                vacant.insert((new, program.clone()));

                program
            }
        };

        graph.add_node(name_symbol);

        for dependency in program.dependencies.iter() {
            let dependency_symbol = crate::symbol(&dependency.name)?;
            graph.add_edge(name_symbol, dependency_symbol);
            Self::graph_build(home_path, network, endpoint, dependency.clone(), map, graph)?;
        }

        Ok(())
    }

    /// Get the program ID, program, optional edition, and optional manifest for all programs in the package.
    /// This method assumes that the package has already been built (`leo build` has been run).
    #[allow(clippy::type_complexity)]
    pub fn get_programs_and_manifests<P: AsRef<Path>>(
        &self,
        home_path: P,
    ) -> Result<Vec<(String, String, Option<u16>, Option<Manifest>)>> {
        self.get_programs_and_manifests_impl(home_path.as_ref())
    }

    #[allow(clippy::type_complexity)]
    fn get_programs_and_manifests_impl(
        &self,
        home_path: &Path,
    ) -> Result<Vec<(String, String, Option<u16>, Option<Manifest>)>> {
        self.programs
            .iter()
            .map(|program| {
                match &program.data {
                    ProgramData::Bytecode(bytecode) => {
                        Ok((program.name.to_string(), bytecode.clone(), program.edition, None))
                    }
                    ProgramData::SourcePath(path) => {
                        // Get the path to the built bytecode.
                        let bytecode_path = if path.as_path() == self.source_directory().join("main.leo") {
                            self.build_directory().join("main.aleo")
                        } else {
                            self.imports_directory().join(format!("{}.aleo", program.name))
                        };
                        // Fetch the bytecode.
                        let bytecode = std::fs::read_to_string(&bytecode_path)
                            .map_err(|e| PackageError::failed_to_read_file(bytecode_path.display(), e))?;
                        // Get the package from the directory.
                        let mut path = path.clone();
                        path.pop();
                        path.pop();
                        let package = Package::from_directory_no_graph(&path, home_path)?;
                        // Return the bytecode and the manifest.
                        Ok((program.name.to_string(), bytecode, program.edition, Some(package.manifest.clone())))
                    }
                }
            })
            .collect::<Result<Vec<_>>>()
    }
}

fn main_template(name: &str) -> String {
    format!(
        r#"// The '{name}' program.
program {name}.aleo {{
    transition main(public a: u32, b: u32) -> u32 {{
        let c: u32 = a + b;
        return c;
    }}
    
    // This is the constructor for the program.
    // It is called when the program is deployed or upgraded.
    // It is currently configured to **prevent** upgrades.
    // For more information, please refer to the documentation: <TODO>.
    async constructor() {{
        assert_eq(self.edition, 0u16);
    }}
}}
"#
    )
}
