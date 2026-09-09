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

use crate::*;

use leo_ast::DiGraph;
use leo_errors::Result;
use leo_span::Symbol;

use indexmap::{IndexMap, map::Entry};
use snarkvm::prelude::{Program as SvmProgram, TestnetV0, anyhow};
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

    /// Canonicalized workspace root, when the package lives inside a workspace
    /// tree (an ancestor directory contains `workspace.json`). `None` for
    /// standalone packages. When `Some`, `build_directory()` returns
    /// `<workspace_root>/build/` so every package under the workspace root -
    /// member or not - shares one flat, unit-keyed build root, and a unit
    /// built once by any member is reused structurally by all the others.
    /// Populated once in `from_directory_impl`; never mutated afterwards.
    pub workspace_root: Option<PathBuf>,

    /// A topologically sorted list of all compilation units in this package, whether
    /// dependencies or the main program.
    ///
    /// Any unit's dependent unit will appear before it, so that compiling
    /// them in order should give access to all stubs necessary to compile each
    /// compilation unit.
    pub compilation_units: Vec<CompilationUnit>,

    /// The manifest file of this package.
    pub manifest: Manifest,

    /// The dependency graph of the package.
    pub dep_graph: DiGraph<Symbol>,
}

impl Package {
    /// The root of the build directory.
    ///
    /// This is the single place that knows where build artifacts are rooted;
    /// every per-unit path below is composed from it. For a package inside a
    /// workspace tree this returns `<workspace_root>/build/` so every member
    /// shares one flat, unit-keyed build root; for a standalone package it
    /// returns `<base_directory>/build/`.
    pub fn build_directory(&self) -> PathBuf {
        self.workspace_root.as_deref().unwrap_or(&self.base_directory).join(BUILD_DIRECTORY)
    }

    /// The package's own compilation unit, identified via the manifest.
    /// Robust under `--build-tests` (unlike `compilation_units.last()`).
    pub fn primary_unit(&self) -> Option<&CompilationUnit> {
        let primary = bare_unit_name(&self.manifest.program);
        self.compilation_units.iter().find(|u| !u.kind.is_test() && bare_unit_name(&u.name.to_string()) == primary)
    }

    /// The `build/<name>/` directory for a single compilation unit - a program,
    /// library, or test - whether it is this package's own unit, a local
    /// dependency, or a fetched network import.
    pub fn unit_build_directory(&self, name: &str) -> PathBuf {
        self.build_directory().join(bare_unit_name(name))
    }

    /// Path to a unit's compiled Aleo bytecode: `build/<name>/<name>.aleo`.
    /// Only programs and tests produce bytecode; libraries do not.
    pub fn unit_bytecode_path(&self, name: &str) -> PathBuf {
        let bare = bare_unit_name(name);
        self.unit_build_directory(name).join(format!("{bare}.aleo"))
    }

    /// Path to a unit's Leo ABI: `build/<name>/abi.json`.
    pub fn unit_abi_path(&self, name: &str) -> PathBuf {
        self.unit_build_directory(name).join(ABI_FILENAME)
    }

    /// Path to a unit's interface ABI directory: `build/<name>/interfaces/`.
    /// Both programs and libraries can declare interfaces.
    pub fn unit_interfaces_directory(&self, name: &str) -> PathBuf {
        self.unit_build_directory(name).join(INTERFACES_DIRNAME)
    }

    pub fn source_directory(&self) -> PathBuf {
        self.base_directory.join(SOURCE_DIRECTORY)
    }

    pub fn tests_directory(&self) -> PathBuf {
        self.base_directory.join(TESTS_DIRECTORY)
    }

    /// Create a Leo package by the name `package_name` in a subdirectory of `path`.
    pub fn initialize<P: AsRef<Path>>(package_name: &str, path: P, is_library: bool) -> Result<PathBuf> {
        Self::initialize_impl(package_name, path.as_ref(), is_library)
    }

    fn initialize_impl(package_name: &str, path: &Path, is_library: bool) -> Result<PathBuf> {
        let package_name = if is_library {
            if !crate::is_valid_library_name(package_name) {
                return Err(crate::errors::cli_invalid_package_name("library", package_name).into());
            }

            package_name.to_string()
        } else {
            let program_name =
                if package_name.ends_with(".aleo") { package_name.to_string() } else { format!("{package_name}.aleo") };

            if !crate::is_valid_program_name(&program_name) {
                return Err(crate::errors::cli_invalid_package_name("program", &program_name).into());
            }

            program_name
        };

        let path = path.canonicalize().map_err(|e| crate::errors::failed_path(path.display(), e))?;
        let full_path = path.join(package_name.strip_suffix(".aleo").unwrap_or(&package_name));

        // Verify that there is no existing directory at the path.
        if full_path.exists() {
            return Err(
                crate::errors::failed_to_initialize_package(package_name, &path, "Directory already exists").into()
            );
        }

        // Create the package directory.
        std::fs::create_dir(&full_path)
            .map_err(|e| crate::errors::failed_to_initialize_package(&package_name, &full_path, e))?;

        // Change the current working directory to the package directory.
        std::env::set_current_dir(&full_path)
            .map_err(|e| crate::errors::failed_to_initialize_package(&package_name, &full_path, e))?;

        // Create .gitignore
        const GITIGNORE_TEMPLATE: &str = ".env\n*.avm\n*.prover\n*.verifier\nbuild/\n";
        const GITIGNORE_FILENAME: &str = ".gitignore";

        let gitignore_path = full_path.join(GITIGNORE_FILENAME);
        std::fs::write(gitignore_path, GITIGNORE_TEMPLATE).map_err(crate::errors::io_error_gitignore_file)?;

        // Create manifest
        let manifest = Manifest {
            program: package_name.clone(),
            version: "0.1.0".to_string(),
            description: String::new(),
            license: "MIT".to_string(),
            leo: env!("CARGO_PKG_VERSION").to_string(),
            dependencies: None,
            dev_dependencies: None,
            no_std: false,
        };

        let manifest_path = full_path.join(MANIFEST_FILENAME);
        manifest.write_to_file(manifest_path)?;

        // Create src/
        let source_path = full_path.join(SOURCE_DIRECTORY);

        std::fs::create_dir(&source_path)
            .map_err(|e| crate::errors::failed_to_create_source_directory(source_path.display(), e))?;

        let name_no_aleo = package_name.strip_suffix(".aleo").unwrap_or(&package_name);

        if is_library {
            // Create lib.leo with a placeholder function.
            let lib_path = source_path.join("lib.leo");

            std::fs::write(&lib_path, lib_template(name_no_aleo)).map_err(|e| {
                crate::errors::util_file_io_error(format_args!("Failed to write `{}`", lib_path.display()), e)
            })?;

            // Create tests directory with a starter test file.
            let tests_path = full_path.join(TESTS_DIRECTORY);

            std::fs::create_dir(&tests_path)
                .map_err(|e| crate::errors::failed_to_create_source_directory(tests_path.display(), e))?;

            let test_file_path = tests_path.join(format!("test_{name_no_aleo}.leo"));

            std::fs::write(&test_file_path, lib_test_template(name_no_aleo)).map_err(|e| {
                crate::errors::util_file_io_error(format_args!("Failed to write `{}`", test_file_path.display()), e)
            })?;
        } else {
            // Create main.leo
            let main_path = source_path.join(MAIN_FILENAME);

            std::fs::write(&main_path, main_template(name_no_aleo)).map_err(|e| {
                crate::errors::util_file_io_error(format_args!("Failed to write `{}`", main_path.display()), e)
            })?;

            // Create tests directory
            let tests_path = full_path.join(TESTS_DIRECTORY);

            std::fs::create_dir(&tests_path)
                .map_err(|e| crate::errors::failed_to_create_source_directory(tests_path.display(), e))?;

            let test_file_path = tests_path.join(format!("test_{name_no_aleo}.leo"));

            std::fs::write(&test_file_path, test_template(name_no_aleo)).map_err(|e| {
                crate::errors::util_file_io_error(format_args!("Failed to write `{}`", test_file_path.display()), e)
            })?;
        }

        Ok(full_path)
    }

    /// Examine the Leo package at `path` to create a `Package`, but don't find dependencies.
    ///
    /// This may be useful if you just need other information like the manifest file.
    pub fn from_directory_no_graph<P: AsRef<Path>, Q: AsRef<Path>>(
        path: P,
        home_path: Q,
        network: Option<NetworkName>,
        endpoint: Option<&str>,
        network_retries: u32,
    ) -> Result<Self> {
        Self::from_directory_impl(
            path.as_ref(),
            home_path.as_ref(),
            /* build_graph */ false,
            /* with_tests */ false,
            /* no_cache */ false,
            /* no_local */ false,
            /* offline */ false,
            network,
            endpoint,
            network_retries,
        )
    }

    /// Load an Aleo bytecode file as a package, including its local and network imports.
    ///
    /// Local imports use the same layouts as `leo abi`: `<root>/<name>/<name>.aleo` for a build bundle, or
    /// `<imports-directory>/<name>.aleo` for a flat bundle. Imports that are not present there are fetched from the
    /// network.
    #[allow(clippy::too_many_arguments)]
    pub fn from_aleo_file<P: AsRef<Path>, Q: AsRef<Path>>(
        path: P,
        home_path: Q,
        imports_directory: Option<&Path>,
        no_cache: bool,
        no_local: bool,
        network: Option<NetworkName>,
        endpoint: Option<&str>,
        network_retries: u32,
    ) -> Result<Self> {
        Self::from_aleo_file_impl(
            path.as_ref(),
            home_path.as_ref(),
            imports_directory,
            no_cache,
            no_local,
            network,
            endpoint,
            network_retries,
        )
    }

    /// Examine the Leo package at `path` to create a `Package`, including all its dependencies,
    /// obtaining dependencies from the file system or network and topologically sorting them.
    #[allow(clippy::too_many_arguments)]
    pub fn from_directory<P: AsRef<Path>, Q: AsRef<Path>>(
        path: P,
        home_path: Q,
        no_cache: bool,
        no_local: bool,
        offline: bool,
        network: Option<NetworkName>,
        endpoint: Option<&str>,
        network_retries: u32,
    ) -> Result<Self> {
        Self::from_directory_impl(
            path.as_ref(),
            home_path.as_ref(),
            /* build_graph */ true,
            /* with_tests */ false,
            no_cache,
            no_local,
            offline,
            network,
            endpoint,
            network_retries,
        )
    }

    /// Examine the Leo package at `path` to create a `Package`, including all its dependencies
    /// and its tests, obtaining dependencies from the file system or network and topologically sorting them.
    #[allow(clippy::too_many_arguments)]
    pub fn from_directory_with_tests<P: AsRef<Path>, Q: AsRef<Path>>(
        path: P,
        home_path: Q,
        no_cache: bool,
        no_local: bool,
        offline: bool,
        network: Option<NetworkName>,
        endpoint: Option<&str>,
        network_retries: u32,
    ) -> Result<Self> {
        Self::from_directory_impl(
            path.as_ref(),
            home_path.as_ref(),
            /* build_graph */ true,
            /* with_tests */ true,
            no_cache,
            no_local,
            offline,
            network,
            endpoint,
            network_retries,
        )
    }

    pub fn test_files(&self) -> impl Iterator<Item = PathBuf> {
        let path = self.tests_directory();
        // This allocation isn't ideal but it's not performance critical and
        // easily resolves lifetime issues.
        let data: Vec<PathBuf> = Self::files_with_extension(&path, "leo").collect();
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
    fn from_aleo_file_impl(
        path: &Path,
        home_path: &Path,
        imports_directory: Option<&Path>,
        no_cache: bool,
        no_local: bool,
        network: Option<NetworkName>,
        endpoint: Option<&str>,
        network_retries: u32,
    ) -> Result<Self> {
        if path.extension().and_then(|extension| extension.to_str()) != Some("aleo") {
            return Err(anyhow!("Expected an Aleo bytecode file with the `.aleo` extension: {}", path.display()).into());
        }

        let path = path.canonicalize().map_err(|error| crate::errors::failed_path(path.display(), error))?;
        if !path.is_file() {
            return Err(anyhow!("Expected an Aleo bytecode file: {}", path.display()).into());
        }
        let home_path =
            home_path.canonicalize().map_err(|error| crate::errors::failed_path(home_path.display(), error))?;
        let bytecode = std::fs::read_to_string(&path).map_err(|error| {
            crate::errors::util_file_io_error(format_args!("Trying to read Aleo file at {}", path.display()), error)
        })?;
        let source_name = path.file_stem().and_then(|name| name.to_str()).unwrap_or("program");
        let main_program: SvmProgram<TestnetV0> =
            bytecode.parse().map_err(|_| crate::errors::snarkvm_parsing_error(source_name))?;
        let program_name = main_program.id().to_string();
        let program_symbol = symbol(&program_name)?;
        let base_directory = path
            .parent()
            .ok_or_else(|| anyhow!("Aleo bytecode file has no parent directory: {}", path.display()))?
            .to_path_buf();

        let main_dependency = Dependency {
            name: program_name.clone(),
            location: Location::Local,
            path: Some(path.clone()),
            edition: None,
            ..Default::default()
        };
        let imports_directory = if no_local {
            None
        } else {
            imports_directory
                .map(|imports_directory| -> Result<PathBuf> {
                    let imports_directory = imports_directory
                        .canonicalize()
                        .map_err(|error| crate::errors::failed_path(imports_directory.display(), error))?;
                    if !imports_directory.is_dir() {
                        return Err(
                            anyhow!("Expected an Aleo imports directory: {}", imports_directory.display()).into()
                        );
                    }
                    Ok(imports_directory)
                })
                .transpose()?
        };
        let declared_deps = IndexMap::from([(program_symbol, main_dependency.clone())]);

        let mut map: IndexMap<Symbol, (Dependency, CompilationUnit)> = IndexMap::new();
        let mut digraph = DiGraph::new(Default::default());
        let old_lock = Lock::default();
        let mut new_lock = Lock::default();
        Self::graph_build(
            &home_path,
            network,
            endpoint,
            &main_dependency,
            main_dependency.clone(),
            &mut map,
            &mut digraph,
            no_cache,
            false,
            imports_directory.as_deref(),
            network_retries,
            &declared_deps,
            &old_lock,
            &mut new_lock,
            false,
        )?;

        let compilation_units = digraph
            .post_order()
            .map_err(|_| crate::errors::circular_dependency_error())?
            .into_iter()
            .map(|name| {
                map.swap_remove(&name)
                    .map(|(_, unit)| unit)
                    .ok_or_else(|| anyhow!("Dependency graph contains an unknown program `{name}`.").into())
            })
            .collect::<Result<Vec<_>>>()?;
        let manifest = Manifest {
            program: program_name,
            version: "0.0.0".to_string(),
            description: String::new(),
            license: String::new(),
            leo: env!("CARGO_PKG_VERSION").to_string(),
            dependencies: None,
            dev_dependencies: None,
            no_std: false,
        };

        Ok(Package { base_directory, workspace_root: None, compilation_units, manifest, dep_graph: digraph })
    }

    #[allow(clippy::too_many_arguments)]
    fn from_directory_impl(
        path: &Path,
        home_path: &Path,
        build_graph: bool,
        with_tests: bool,
        no_cache: bool,
        no_local: bool,
        offline: bool,
        network: Option<NetworkName>,
        endpoint: Option<&str>,
        network_retries: u32,
    ) -> Result<Self> {
        let map_err = |path: &Path, err| {
            crate::errors::util_file_io_error(format_args!("Trying to find path at {}", path.display()), err)
        };

        let path = path.canonicalize().map_err(|err| map_err(path, err))?;

        // Detect an enclosing workspace so build artifacts route to a shared
        // `<workspace_root>/build/`. The walk only checks for `workspace.json`
        // (no manifest parsing, no member resolution), so it is cheap.
        let workspace_root = Workspace::discover_root(&path)?;

        let manifest = Manifest::read_from_file(path.join(MANIFEST_FILENAME))?;

        let (compilation_units, digraph) = if build_graph {
            let home_path = home_path.canonicalize().map_err(|err| map_err(home_path, err))?;

            let mut map: IndexMap<Symbol, (Dependency, CompilationUnit)> = IndexMap::new();

            let mut digraph = DiGraph::<Symbol>::new(Default::default());

            // Pre-collect all declared dependencies from the manifest tree so that
            // .aleo file import classification doesn't depend on processing order.
            let declared_deps = collect_declared_deps(&path, &manifest, with_tests)?;

            // The lock lives at the workspace root, else beside this package's `program.json`.
            let lock_dir = workspace_root.as_deref().unwrap_or(&path).to_path_buf();
            // New lock records only this build's resolutions; others are carried over from the old lock after.
            let old_lock = Lock::read(&lock_dir);
            let mut new_lock = Lock::default();

            let first_dependency = Dependency {
                name: manifest.program.clone(),
                location: Location::Local,
                path: Some(path.clone()),
                edition: None,
                ..Default::default()
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
                        ..Default::default()
                    })
                    .collect();
                if let Some(deps) = manifest.dev_dependencies.as_ref() {
                    // Canonicalize dev-dependency paths like regular dependencies, so the same local
                    // library in both lists dedups instead of comparing relative against absolute.
                    for dep in deps {
                        let dep = canonicalize_dependency_path_relative_to(&path, dep.clone())?;
                        let dep = if dep.location == Location::Workspace {
                            resolve_workspace_dependency(&path, dep)?
                        } else {
                            dep
                        };
                        test_dependencies.push(dep);
                    }
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
                    None,
                    network_retries,
                    &declared_deps,
                    &old_lock,
                    &mut new_lock,
                    offline,
                )?;
            }

            // Workspace: carry all entries since the lock is shared. Standalone: carry only dev-git
            // names (a plain build skips dev deps, so their pins may legitimately be unresolved).
            if workspace_root.is_some() {
                new_lock.carry_over(&old_lock, |_| true);
            } else {
                let dev_git_names: Vec<&str> = if with_tests {
                    Vec::new()
                } else {
                    manifest
                        .dev_dependencies
                        .iter()
                        .flatten()
                        .filter(|dep| dep.location == Location::Git)
                        .map(|dep| dep.name.as_str())
                        .collect()
                };
                new_lock.carry_over(&old_lock, |entry| dev_git_names.contains(&entry.name.as_str()));
            }
            // Persist the lock (and drop a stale one when no git deps remain).
            new_lock.write(&lock_dir)?;

            let ordered_dependency_symbols =
                digraph.post_order().map_err(|_| crate::errors::circular_dependency_error())?;

            (
                ordered_dependency_symbols.into_iter().map(|symbol| map.swap_remove(&symbol).unwrap().1).collect(),
                digraph,
            )
        } else {
            (Vec::new(), DiGraph::default())
        };

        Ok(Package { base_directory: path, workspace_root, compilation_units, manifest, dep_graph: digraph })
    }

    #[allow(clippy::too_many_arguments)]
    fn graph_build(
        home_path: &Path,
        network: Option<NetworkName>,
        endpoint: Option<&str>,
        main_program: &Dependency,
        new: Dependency,
        map: &mut IndexMap<Symbol, (Dependency, CompilationUnit)>,
        graph: &mut DiGraph<Symbol>,
        no_cache: bool,
        no_local: bool,
        aleo_imports_directory: Option<&Path>,
        network_retries: u32,
        declared_deps: &IndexMap<Symbol, Dependency>,
        old_lock: &Lock,
        new_lock: &mut Lock,
        offline: bool,
    ) -> Result<()> {
        let mut new = new;
        if new.location == Location::Network
            && let Some(imports_directory) = aleo_imports_directory
        {
            let path = aleo_import_path(imports_directory, &new.name);
            if path.exists() {
                if !path.is_file() {
                    return Err(anyhow!("Expected Aleo import `{}` to be a file: {}", new.name, path.display()).into());
                }
                let bytecode = std::fs::read_to_string(&path).map_err(|error| {
                    crate::errors::util_file_io_error(
                        format_args!("Trying to read Aleo file at {}", path.display()),
                        error,
                    )
                })?;
                let imported: SvmProgram<TestnetV0> =
                    bytecode.parse().map_err(|_| crate::errors::snarkvm_parsing_error(bare_unit_name(&new.name)))?;
                if imported.id().to_string() != new.name {
                    return Err(anyhow!(
                        "Aleo import `{}` resolved to `{}`, but that file declares `{}`.",
                        new.name,
                        path.display(),
                        imported.id()
                    )
                    .into());
                }
                new.location = Location::Local;
                new.path = Some(path);
                new.edition = None;
            }
        }

        let name_symbol = symbol(&new.name)?;

        let unit = match map.entry(name_symbol) {
            Entry::Occupied(occupied) => {
                // We've already visited this dependency. Just make sure it's compatible with
                // the one we already have.
                let existing_dep = &occupied.get().0;
                assert_eq!(new.name, existing_dep.name);
                if new.location != existing_dep.location
                    || new.path != existing_dep.path
                    || new.edition != existing_dep.edition
                    || new.git != existing_dep.git
                {
                    return Err(crate::errors::conflicting_dependency(existing_dep, new).into());
                }
                return Ok(());
            }
            Entry::Vacant(vacant) => {
                let unit = match (new.path.as_ref(), new.location) {
                    (Some(path), Location::Local) if !no_local => {
                        // It's a local dependency.
                        if path.extension().and_then(|p| p.to_str()) == Some("aleo") && path.is_file() {
                            CompilationUnit::from_aleo_path(name_symbol, path, declared_deps)?
                        } else {
                            CompilationUnit::from_package_path(name_symbol, path)?
                        }
                    }
                    (Some(path), Location::Test) => {
                        // It's a test dependency - the path points to the source file,
                        // not a package.
                        CompilationUnit::from_test_path(path, main_program.clone())?
                    }
                    (_, Location::Network) | (Some(_), Location::Local) => {
                        // It's a network dependency.
                        let Some(endpoint) = endpoint else {
                            return Err(anyhow!("An endpoint must be provided to fetch network dependencies.").into());
                        };
                        let Some(network) = network else {
                            return Err(anyhow!("A network must be provided to fetch network dependencies.").into());
                        };
                        CompilationUnit::fetch(
                            name_symbol,
                            new.edition,
                            home_path,
                            network,
                            endpoint,
                            no_cache,
                            network_retries,
                        )?
                    }
                    (_, Location::Git) => CompilationUnit::from_git(
                        name_symbol,
                        &new,
                        home_path,
                        old_lock,
                        new_lock,
                        offline,
                        declared_deps,
                    )?,
                    (_, Location::Workspace) => {
                        return Err(anyhow!(
                            "Workspace dependency `{}` was not resolved before graph building. This is a compiler bug.",
                            new.name
                        )
                        .into());
                    }
                    _ => return Err(anyhow!("Invalid dependency data for {} (path must be given).", new.name).into()),
                };

                vacant.insert((new, unit.clone()));

                unit
            }
        };

        graph.add_node(name_symbol);

        // Security: a package in a git checkout may only path-reference its own checkout.
        // Intra-checkout deps were rewritten to git deps in `from_git`; any remaining path dep is an escape.
        let checkouts_root = crate::git::checkouts_root(home_path);
        if let ProgramData::SourcePath { directory, .. } = &unit.data
            && directory.starts_with(&checkouts_root)
        {
            // The checkout root is `<checkouts_root>/<key>/<commit>`.
            let checkout = directory
                .strip_prefix(&checkouts_root)
                .ok()
                .and_then(|rel| {
                    let mut components = rel.components();
                    Some((components.next()?, components.next()?))
                })
                .map(|(key, commit)| checkouts_root.join(key).join(commit));
            for dependency in unit.dependencies.iter() {
                if let Some(path) = &dependency.path
                    && !checkout.as_ref().is_some_and(|checkout| path.starts_with(checkout))
                {
                    return Err(crate::errors::invalid_manifest_dependency(
                        &dependency.name,
                        "a git dependency may only reference paths inside its own repository checkout",
                    )
                    .into());
                }
            }
        }

        for dependency in unit.dependencies.iter() {
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
                aleo_imports_directory,
                network_retries,
                declared_deps,
                old_lock,
                new_lock,
                offline,
            )?;
        }

        Ok(())
    }
}

/// Return the default directory for local imports of an Aleo bytecode file.
pub fn default_aleo_imports_directory(path: &Path) -> Option<PathBuf> {
    let parent = path.parent()?;
    if parent.file_name() == path.file_stem() {
        return parent.parent().map(Path::to_path_buf);
    }

    let imports = parent.join("imports");
    imports.is_dir().then_some(imports)
}

/// Return the preferred path for an Aleo import in a flat or per-unit imports directory.
pub fn aleo_import_path(imports_directory: &Path, program_name: &str) -> PathBuf {
    let bare_name = bare_unit_name(program_name);
    let per_unit_path = imports_directory.join(bare_name).join(program_name);
    if per_unit_path.exists() { per_unit_path } else { imports_directory.join(program_name) }
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
    constructor() {{}}

    fn main(public a: u32, b: u32) -> u32 {{
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
    @should_fail
    fn test_main_fails() {{
        let result: u32 = {name}.aleo::main(2u32, 3u32);
        assert_eq(result, 3u32);
    }}

    @noupgrade
    constructor() {{}}
}}
"#
    )
}

fn lib_template(name: &str) -> String {
    format!(
        r#"// The '{name}' library.

// Returns the identity of x.
export fn example(x: u32) -> u32 {{
    return x;
}}
"#
    )
}

fn lib_test_template(name: &str) -> String {
    format!(
        r#"// The 'test_{name}' test program.
program test_{name}.aleo {{
    @test
    fn test_example() {{
        assert_eq({name}::example(42u32), 42u32);
    }}

    @noupgrade
    constructor() {{}}
}}
"#
    )
}

/// Walk the manifest tree and collect all declared dependencies.
///
/// This gives `parse_dependencies_from_aleo` full knowledge of which programs are
/// declared as local dependencies, regardless of the order they appear in the manifest.
/// Without this, `.aleo` file imports are classified against a snapshot of
/// already-processed dependencies, requiring the user to list them in topological order.
fn collect_declared_deps(
    root_path: &Path,
    manifest: &Manifest,
    with_tests: bool,
) -> Result<IndexMap<Symbol, Dependency>> {
    let mut declared = IndexMap::new();
    collect_declared_deps_recursive(root_path, manifest, with_tests, &mut declared)?;
    Ok(declared)
}

fn collect_declared_deps_recursive(
    base_path: &Path,
    manifest: &Manifest,
    include_dev: bool,
    declared: &mut IndexMap<Symbol, Dependency>,
) -> Result<()> {
    let deps = manifest.dependencies.iter().flatten();
    let dev: Vec<&Dependency> =
        if include_dev { manifest.dev_dependencies.iter().flatten().collect() } else { Vec::new() };
    for dep in deps.chain(dev) {
        let dep = canonicalize_dependency_path_relative_to(base_path, dep.clone())?;
        // Resolve workspace deps early - converts to Location::Local with an absolute path.
        let dep = if dep.location == Location::Workspace { resolve_workspace_dependency(base_path, dep)? } else { dep };
        let sym = symbol(&dep.name)?;
        // Only recurse into newly discovered dependencies to avoid infinite
        // recursion on circular manifests (cycles are caught later by
        // `DiGraph::post_order`).
        let Entry::Vacant(e) = declared.entry(sym) else {
            continue;
        };
        e.insert(dep.clone());
        if dep.location == Location::Local
            && let Some(path) = &dep.path
        {
            let manifest_path = path.join(MANIFEST_FILENAME);
            if path.is_dir() && manifest_path.exists() {
                let child = Manifest::read_from_file(manifest_path)?;
                // dev_dependencies are not transitive.
                collect_declared_deps_recursive(path, &child, false, declared)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use leo_span::create_session_if_not_set_then;

    const LEAF_PROGRAM: &str = "\
program leaf.aleo;

function identity:
    input r0 as u32.private;
    output r0 as u32.private;
";

    const DEPENDENCY_PROGRAM: &str = "\
import leaf.aleo;

program dependency.aleo;

function times_two:
    input r0 as u32.private;
    call leaf.aleo/identity r0 into r1;
    add r1 r1 into r2;
    output r2 as u32.private;
";

    const MAIN_PROGRAM: &str = "\
import dependency.aleo;

program standalone.aleo;

function main:
    input r0 as u32.private;
    call dependency.aleo/times_two r0 into r1;
    output r1 as u32.private;
";

    fn dummy_package(base: &str) -> Package {
        dummy_package_with(base, None)
    }

    fn dummy_package_with(base: &str, workspace_root: Option<PathBuf>) -> Package {
        Package {
            base_directory: PathBuf::from(base),
            workspace_root,
            compilation_units: Vec::new(),
            manifest: Manifest {
                program: "demo.aleo".to_string(),
                version: "0.1.0".to_string(),
                description: String::new(),
                license: "MIT".to_string(),
                leo: "0.0.0".to_string(),
                dependencies: None,
                dev_dependencies: None,
                no_std: false,
            },
            dep_graph: DiGraph::default(),
        }
    }

    #[test]
    fn bare_unit_name_strips_aleo_suffix() {
        assert_eq!(crate::bare_unit_name("token.aleo"), "token");
        assert_eq!(crate::bare_unit_name("token"), "token");
        assert_eq!(crate::bare_unit_name("credits.aleo"), "credits");
    }

    #[test]
    fn unit_paths_are_keyed_by_bare_name() {
        let pkg = dummy_package("/tmp/demo");
        // The directory key is the bare compilation unit name, accepting input
        // with or without the `.aleo` suffix.
        assert_eq!(pkg.unit_build_directory("token.aleo"), PathBuf::from("/tmp/demo/build/token"));
        assert_eq!(pkg.unit_build_directory("token"), PathBuf::from("/tmp/demo/build/token"));
        assert_eq!(pkg.unit_bytecode_path("token.aleo"), PathBuf::from("/tmp/demo/build/token/token.aleo"));
        assert_eq!(pkg.unit_abi_path("token"), PathBuf::from("/tmp/demo/build/token/abi.json"));
        assert_eq!(pkg.unit_interfaces_directory("token"), PathBuf::from("/tmp/demo/build/token/interfaces"));
    }

    #[test]
    fn libraries_are_keyed_like_programs() {
        // A library is keyed by its name exactly like a program: a library
        // `my_lib` declaring interfaces gets `build/my_lib/interfaces/`.
        let pkg = dummy_package("/tmp/demo");
        assert_eq!(pkg.unit_build_directory("my_lib"), PathBuf::from("/tmp/demo/build/my_lib"));
        assert_eq!(pkg.unit_interfaces_directory("my_lib"), PathBuf::from("/tmp/demo/build/my_lib/interfaces"));
    }

    #[test]
    fn build_directory_is_the_single_root() {
        let pkg = dummy_package("/tmp/demo");
        assert_eq!(pkg.build_directory(), PathBuf::from("/tmp/demo/build"));
        // Every per-unit path is rooted at `build_directory()`, the single layout seam.
        assert!(pkg.unit_bytecode_path("x").starts_with(pkg.build_directory()));
        assert!(pkg.unit_interfaces_directory("credits.aleo").starts_with(pkg.build_directory()));
    }

    #[test]
    fn workspace_root_routes_build_directory_to_shared() {
        // When inside a workspace, `build_directory()` routes to the
        // workspace root - not the package's own directory - so every
        // member's per-unit subdirectory collapses under one shared
        // `<root>/build/` and deduplicates structurally on unit name.
        let pkg = dummy_package_with("/tmp/ws/members/token", Some(PathBuf::from("/tmp/ws")));
        assert_eq!(pkg.build_directory(), PathBuf::from("/tmp/ws/build"));
        assert_eq!(pkg.unit_build_directory("token"), PathBuf::from("/tmp/ws/build/token"));
        assert_eq!(pkg.unit_bytecode_path("token"), PathBuf::from("/tmp/ws/build/token/token.aleo"));
        // The package's own base_directory is irrelevant for the per-unit path:
        // a workspace member and a separate dependency keyed by the same unit
        // name resolve to byte-identical paths.
        let dep = dummy_package_with("/tmp/ws/members/swap", Some(PathBuf::from("/tmp/ws")));
        assert_eq!(pkg.unit_bytecode_path("token"), dep.unit_bytecode_path("token"));
    }

    #[test]
    fn standalone_package_keeps_per_base_build_directory() {
        // The standalone path must not change: a package outside any
        // workspace still rooots its build under its own directory.
        let pkg = dummy_package_with("/tmp/standalone", None);
        assert_eq!(pkg.build_directory(), PathBuf::from("/tmp/standalone/build"));
        assert_eq!(pkg.unit_build_directory("demo"), PathBuf::from("/tmp/standalone/build/demo"));
    }

    #[test]
    fn aleo_file_uses_sibling_imports_directory() {
        create_session_if_not_set_then(|_| {
            let root = crate::test_util::unique_dir("aleo-file-flat-imports");
            let program_path = root.join("standalone.aleo");
            let home = root.join("home");
            crate::test_util::write_file(&program_path, MAIN_PROGRAM);
            crate::test_util::write_file(&root.join("imports/dependency.aleo"), DEPENDENCY_PROGRAM);
            crate::test_util::write_file(&root.join("imports/leaf.aleo"), LEAF_PROGRAM);
            std::fs::create_dir_all(&home).expect("test registry directory should be created");

            let package =
                Package::from_aleo_file(&program_path, &home, Some(&root.join("imports")), false, false, None, None, 0)
                    .expect("standalone Aleo program should load with its local import");

            let names = package.compilation_units.iter().map(|unit| unit.name.to_string()).collect::<Vec<_>>();
            assert_eq!(names, ["leaf.aleo", "dependency.aleo", "standalone.aleo"]);
            assert!(package.compilation_units.iter().all(|unit| unit.is_local));
            assert_eq!(package.manifest.program, "standalone.aleo");

            std::fs::remove_dir_all(root).expect("test directory should be removed");
        });
    }

    #[test]
    fn aleo_file_uses_per_unit_build_layout() {
        create_session_if_not_set_then(|_| {
            let root = crate::test_util::unique_dir("aleo-file-per-unit-imports");
            let program_path = root.join("standalone/standalone.aleo");
            let home = root.join("home");
            crate::test_util::write_file(&program_path, MAIN_PROGRAM);
            crate::test_util::write_file(&root.join("dependency/dependency.aleo"), DEPENDENCY_PROGRAM);
            crate::test_util::write_file(&root.join("leaf/leaf.aleo"), LEAF_PROGRAM);
            std::fs::create_dir_all(&home).expect("test registry directory should be created");

            let package = Package::from_aleo_file(&program_path, &home, Some(&root), false, false, None, None, 0)
                .expect("standalone Aleo build artifact should load with its local import");

            let names = package.compilation_units.iter().map(|unit| unit.name.to_string()).collect::<Vec<_>>();
            assert_eq!(names, ["leaf.aleo", "dependency.aleo", "standalone.aleo"]);
            assert!(package.compilation_units.iter().all(|unit| unit.is_local));

            std::fs::remove_dir_all(root).expect("test directory should be removed");
        });
    }

    #[test]
    fn missing_aleo_import_is_classified_as_network() {
        create_session_if_not_set_then(|_| {
            let root = crate::test_util::unique_dir("aleo-file-network-import");
            let program_path = root.join("standalone.aleo");
            crate::test_util::write_file(&program_path, MAIN_PROGRAM);

            let unit =
                CompilationUnit::from_aleo_path(Symbol::intern("standalone.aleo"), &program_path, &IndexMap::new())
                    .expect("test Aleo program should load");

            let dependency = unit.dependencies.first().expect("test program should have one direct import");
            assert_eq!(dependency.name, "dependency.aleo");
            assert_eq!(dependency.location, Location::Network);

            std::fs::remove_dir_all(root).expect("test directory should be removed");
        });
    }

    #[test]
    fn aleo_file_resolves_local_import_below_network_import() {
        create_session_if_not_set_then(|_| {
            let root = crate::test_util::unique_dir("aleo-file-mixed-transitive-imports");
            let program_path = root.join("standalone.aleo");
            let imports = root.join("imports");
            let home = root.join("home");
            crate::test_util::write_file(&program_path, MAIN_PROGRAM);
            crate::test_util::write_file(&imports.join("leaf.aleo"), LEAF_PROGRAM);
            crate::test_util::write_file(
                &home.join("registry/testnet/dependency/0/dependency.aleo"),
                DEPENDENCY_PROGRAM,
            );

            let package = Package::from_aleo_file(
                &program_path,
                &home,
                Some(&imports),
                false,
                false,
                Some(NetworkName::TestnetV0),
                Some("http://localhost:1"),
                0,
            )
            .expect("a local transitive import below a network import should be used");

            let units =
                package.compilation_units.iter().map(|unit| (unit.name.to_string(), unit.is_local)).collect::<Vec<_>>();
            assert_eq!(units, [
                ("leaf.aleo".to_string(), true),
                ("dependency.aleo".to_string(), false),
                ("standalone.aleo".to_string(), true)
            ]);

            std::fs::remove_dir_all(root).expect("test directory should be removed");
        });
    }

    #[test]
    fn aleo_file_rejects_non_directory_imports_path() {
        create_session_if_not_set_then(|_| {
            let root = crate::test_util::unique_dir("aleo-file-invalid-imports-directory");
            let program_path = root.join("standalone.aleo");
            let home = root.join("home");
            crate::test_util::write_file(&program_path, MAIN_PROGRAM);
            std::fs::create_dir_all(&home).expect("test registry directory should be created");

            let error = Package::from_aleo_file(&program_path, &home, Some(&program_path), false, false, None, None, 0)
                .expect_err("an imports path that is not a directory should fail");
            assert!(error.to_string().contains("Expected an Aleo imports directory"));

            std::fs::remove_dir_all(root).expect("test directory should be removed");
        });
    }
}
