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
use leo_span::{
    Symbol,
    file_source::{DiskFileSource, FileSource},
};

use indexmap::{IndexMap, map::Entry};
#[cfg(not(target_arch = "wasm32"))]
use snarkvm::prelude::anyhow;
use std::path::{Path, PathBuf};

/// Network-dependent configuration for `Package::from_directory_*`.
///
/// Bundled into one parameter so the `from_directory_impl` machinery can be
/// invoked with `Some(...)` from the CLI path (where network deps may need
/// to be fetched) and with `None` from the wasm path (where any encountered
/// network dep is a hard error — the caller must stage `.aleo` bytecode in
/// the virtual file map up front).
///
/// The struct compiles on every target; only the actual network-fetch glue
/// inside `graph_build` is `#[cfg(not(target_arch = "wasm32"))]`-gated.
pub struct NetworkConfig<'a> {
    pub home_path: PathBuf,
    pub network: Option<leo_ast::NetworkName>,
    pub endpoint: Option<&'a str>,
    pub no_cache: bool,
    pub no_local: bool,
    pub network_retries: u32,
}

/// Either the bytecode of an Aleo program (if it was a network dependency) or
/// A Leo package.
#[derive(Clone, Debug)]
pub struct Package {
    /// The directory on the filesystem where the package is located, canonicalized.
    pub base_directory: PathBuf,

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
    /// every per-unit path below is composed from it.
    pub fn build_directory(&self) -> PathBuf {
        self.base_directory.join(BUILD_DIRECTORY)
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

    /// Path to a unit's AST-snapshot directory: `build/<name>/snapshots/`.
    /// Populated only when a snapshot CLI flag is set; created lazily by the
    /// compiler on the first write, so absent on builds that don't request snapshots.
    pub fn unit_snapshots_directory(&self, name: &str) -> PathBuf {
        self.unit_build_directory(name).join(SNAPSHOTS_DIRNAME)
    }

    pub fn source_directory(&self) -> PathBuf {
        self.base_directory.join(SOURCE_DIRECTORY)
    }

    pub fn tests_directory(&self) -> PathBuf {
        self.base_directory.join(TESTS_DIRECTORY)
    }

    /// Examine the Leo package at `path` to create a `Package`, but don't find dependencies.
    ///
    /// This may be useful if you just need other information like the manifest file.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_directory_no_graph<P: AsRef<Path>, Q: AsRef<Path>>(
        path: P,
        home_path: Q,
        network: Option<NetworkName>,
        endpoint: Option<&str>,
        network_retries: u32,
    ) -> Result<Self> {
        Self::from_directory_impl(
            path.as_ref(),
            /* build_graph */ false,
            /* with_tests */ false,
            Some(NetworkConfig {
                home_path: home_path.as_ref().to_path_buf(),
                network,
                endpoint,
                no_cache: false,
                no_local: false,
                network_retries,
            }),
            &DiskFileSource,
        )
    }

    /// Examine the Leo package at `path` to create a `Package`, including all its dependencies,
    /// obtaining dependencies from the file system or network and topologically sorting them.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_directory<P: AsRef<Path>, Q: AsRef<Path>>(
        path: P,
        home_path: Q,
        no_cache: bool,
        no_local: bool,
        network: Option<NetworkName>,
        endpoint: Option<&str>,
        network_retries: u32,
    ) -> Result<Self> {
        Self::from_directory_impl(
            path.as_ref(),
            /* build_graph */ true,
            /* with_tests */ false,
            Some(NetworkConfig {
                home_path: home_path.as_ref().to_path_buf(),
                network,
                endpoint,
                no_cache,
                no_local,
                network_retries,
            }),
            &DiskFileSource,
        )
    }

    /// Examine the Leo package at `path` to create a `Package`, including all its dependencies
    /// and its tests, obtaining dependencies from the file system or network and topologically sorting them.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_directory_with_tests<P: AsRef<Path>, Q: AsRef<Path>>(
        path: P,
        home_path: Q,
        no_cache: bool,
        no_local: bool,
        network: Option<NetworkName>,
        endpoint: Option<&str>,
        network_retries: u32,
    ) -> Result<Self> {
        Self::from_directory_impl(
            path.as_ref(),
            /* build_graph */ true,
            /* with_tests */ true,
            Some(NetworkConfig {
                home_path: home_path.as_ref().to_path_buf(),
                network,
                endpoint,
                no_cache,
                no_local,
                network_retries,
            }),
            &DiskFileSource,
        )
    }

    /// FileSource-aware counterpart to [`Package::from_directory_no_graph`].
    ///
    /// Reads `program.json` (and nothing else) through `file_source` so the
    /// same code path serves wasm callers via an `InMemoryFileSource`.
    pub fn from_directory_no_graph_with_file_source<P: AsRef<Path>>(
        path: P,
        file_source: &impl FileSource,
    ) -> Result<Self> {
        Self::from_directory_impl(
            path.as_ref(),
            /* build_graph */ false,
            /* with_tests */ false,
            None,
            file_source,
        )
    }

    /// FileSource-aware counterpart to [`Package::from_directory`].
    ///
    /// Source-only mode: any [`Location::Network`] dependency encountered
    /// during the walk is a hard error — wasm callers must stage `.aleo`
    /// bytecode in the file map and reference it with `Location::Local`.
    pub fn from_directory_with_file_source<P: AsRef<Path>>(path: P, file_source: &impl FileSource) -> Result<Self> {
        Self::from_directory_impl(
            path.as_ref(),
            /* build_graph */ true,
            /* with_tests */ false,
            None,
            file_source,
        )
    }

    /// FileSource-aware counterpart to [`Package::from_directory_with_tests`].
    ///
    /// Same source-only contract as [`Package::from_directory_with_file_source`].
    pub fn from_directory_with_tests_with_file_source<P: AsRef<Path>>(
        path: P,
        file_source: &impl FileSource,
    ) -> Result<Self> {
        Self::from_directory_impl(
            path.as_ref(),
            /* build_graph */ true,
            /* with_tests */ true,
            None,
            file_source,
        )
    }

    pub fn test_files(&self) -> impl Iterator<Item = PathBuf> {
        // The native CLI's only caller — uses `DiskFileSource::list_leo_files`
        // for parity with the on-disk test runner.
        DiskFileSource.list_leo_files(&self.tests_directory(), Path::new("")).unwrap_or_default().into_iter()
    }

    fn from_directory_impl(
        path: &Path,
        build_graph: bool,
        with_tests: bool,
        // `Some(...)` on the native CLI path (where network deps may be
        // fetched); `None` on the wasm path (where any `Location::Network`
        // encountered while walking errors out).
        network_config: Option<NetworkConfig<'_>>,
        file_source: &impl FileSource,
    ) -> Result<Self> {
        let map_err = |path: &Path, err| {
            crate::errors::util_file_io_error(format_args!("Trying to find path at {}", path.display()), err)
        };

        let path = file_source.canonicalize(path).map_err(|err| map_err(path, err))?;

        let manifest = Manifest::read_from_file_source(path.join(MANIFEST_FILENAME), file_source)?;

        let (compilation_units, digraph) = if build_graph {
            // Real-disk canonicalize the home path only when a network config
            // is present; wasm callers pass `None` and the home dir is unused.
            #[cfg(not(target_arch = "wasm32"))]
            let mut network_config = network_config;
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(nc) = network_config.as_mut() {
                nc.home_path = nc.home_path.canonicalize().map_err(|err| map_err(&nc.home_path, err))?;
            }

            let mut map: IndexMap<Symbol, (Dependency, CompilationUnit)> = IndexMap::new();
            let mut digraph = DiGraph::<Symbol>::new(Default::default());

            // Pre-collect all declared dependencies from the manifest tree so that
            // .aleo file import classification doesn't depend on processing order.
            let declared_deps = collect_declared_deps(&path, &manifest, with_tests, file_source)?;

            let first_dependency = Dependency {
                name: manifest.program.clone(),
                location: Location::Local,
                path: Some(path.clone()),
                edition: None,
            };

            let test_dependencies: Vec<Dependency> = if with_tests {
                let tests_directory = path.join(TESTS_DIRECTORY);
                let mut test_dependencies: Vec<Dependency> = file_source
                    .list_leo_files(&tests_directory, Path::new(""))
                    .unwrap_or_default()
                    .into_iter()
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
                    network_config.as_ref(),
                    &first_dependency,
                    dependency,
                    &mut map,
                    &mut digraph,
                    &declared_deps,
                    file_source,
                )?;
            }

            let ordered_dependency_symbols =
                digraph.post_order().map_err(|_| crate::errors::circular_dependency_error())?;

            (
                ordered_dependency_symbols.into_iter().map(|symbol| map.swap_remove(&symbol).unwrap().1).collect(),
                digraph,
            )
        } else {
            (Vec::new(), DiGraph::default())
        };

        Ok(Package { base_directory: path, compilation_units, manifest, dep_graph: digraph })
    }

    #[allow(clippy::too_many_arguments)]
    fn graph_build(
        // `Some` on the CLI path (where network deps may resolve via HTTP);
        // `None` on the wasm path (where any encountered network dep errors).
        network_config: Option<&NetworkConfig<'_>>,
        main_program: &Dependency,
        new: Dependency,
        map: &mut IndexMap<Symbol, (Dependency, CompilationUnit)>,
        graph: &mut DiGraph<Symbol>,
        declared_deps: &IndexMap<Symbol, Dependency>,
        file_source: &impl FileSource,
    ) -> Result<()> {
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
                {
                    return Err(crate::errors::conflicting_dependency(existing_dep, new).into());
                }
                return Ok(());
            }
            Entry::Vacant(vacant) => {
                let no_local = network_config.is_some_and(|nc| nc.no_local);
                let unit = match (new.path.as_ref(), new.location) {
                    (Some(path), Location::Local) if !no_local => {
                        // It's a local dependency.
                        if path.extension().and_then(|p| p.to_str()) == Some("aleo") && file_source.is_file(path) {
                            CompilationUnit::from_aleo_path_with_file_source(
                                name_symbol,
                                path,
                                declared_deps,
                                file_source,
                            )?
                        } else {
                            CompilationUnit::from_package_path_with_file_source(name_symbol, path, file_source)?
                        }
                    }
                    (Some(path), Location::Test) => {
                        // It's a test dependency - the path points to the source file,
                        // not a package.
                        CompilationUnit::from_test_path_with_file_source(path, main_program.clone(), file_source)?
                    }
                    (_, Location::Network) | (Some(_), Location::Local) => {
                        // Network dependency. Only resolvable when the caller
                        // supplied a `NetworkConfig`; wasm callers stage the
                        // bytecode in the file map and reference it locally.
                        fetch_network_dependency(network_config, &new, name_symbol)?
                    }
                    (_, Location::Workspace) => {
                        return Err(workspace_unresolved_error(&new.name));
                    }
                    _ => return Err(invalid_dependency_error(&new.name)),
                };

                vacant.insert((new, unit.clone()));

                unit
            }
        };

        graph.add_node(name_symbol);

        for dependency in unit.dependencies.iter() {
            let dependency_symbol = symbol(&dependency.name)?;
            graph.add_edge(name_symbol, dependency_symbol);
            Self::graph_build(
                network_config,
                main_program,
                dependency.clone(),
                map,
                graph,
                declared_deps,
                file_source,
            )?;
        }

        Ok(())
    }
}

/// Fetch a network dependency. Native-only: uses `CompilationUnit::fetch`
/// which does HTTP + cache I/O against `home_path`. On wasm this is a hard
/// error — the caller must stage the dep's bytecode in the file map.
#[cfg(not(target_arch = "wasm32"))]
fn fetch_network_dependency(
    network_config: Option<&NetworkConfig<'_>>,
    new: &Dependency,
    name_symbol: Symbol,
) -> Result<CompilationUnit> {
    let Some(nc) = network_config else {
        return Err(anyhow!(
            "Network dependency `{}` is not supported in this build (no network config supplied).",
            new.name
        )
        .into());
    };
    let Some(endpoint) = nc.endpoint else {
        return Err(anyhow!("An endpoint must be provided to fetch network dependencies.").into());
    };
    let Some(network) = nc.network else {
        return Err(anyhow!("A network must be provided to fetch network dependencies.").into());
    };
    CompilationUnit::fetch(name_symbol, new.edition, &nc.home_path, network, endpoint, nc.no_cache, nc.network_retries)
}

#[cfg(target_arch = "wasm32")]
fn fetch_network_dependency(
    _network_config: Option<&NetworkConfig<'_>>,
    new: &Dependency,
    _name_symbol: Symbol,
) -> Result<CompilationUnit> {
    Err(crate::errors::failed_to_open_file(format_args!(
        "Network dependency `{}` is not supported in wasm builds; stage the `.aleo` bytecode in the file map and reference it with `Location::Local`.",
        new.name
    ))
    .into())
}

#[cfg(not(target_arch = "wasm32"))]
fn workspace_unresolved_error(name: &str) -> leo_errors::LeoError {
    anyhow!("Workspace dependency `{name}` was not resolved before graph building. This is a compiler bug.").into()
}

#[cfg(target_arch = "wasm32")]
fn workspace_unresolved_error(name: &str) -> leo_errors::LeoError {
    crate::errors::failed_to_open_file(format_args!(
        "Workspace dependency `{name}` is not supported in wasm builds; declare the dep with an explicit `path` instead."
    ))
    .into()
}

#[cfg(not(target_arch = "wasm32"))]
fn invalid_dependency_error(name: &str) -> leo_errors::LeoError {
    anyhow!("Invalid dependency data for {name} (path must be given).").into()
}

#[cfg(target_arch = "wasm32")]
fn invalid_dependency_error(name: &str) -> leo_errors::LeoError {
    crate::errors::failed_to_open_file(format_args!("Invalid dependency data for {name} (path must be given).")).into()
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
    file_source: &impl FileSource,
) -> Result<IndexMap<Symbol, Dependency>> {
    let mut declared = IndexMap::new();
    collect_declared_deps_recursive(root_path, manifest, with_tests, &mut declared, file_source)?;
    Ok(declared)
}

fn collect_declared_deps_recursive(
    base_path: &Path,
    manifest: &Manifest,
    include_dev: bool,
    declared: &mut IndexMap<Symbol, Dependency>,
    file_source: &impl FileSource,
) -> Result<()> {
    let deps = manifest.dependencies.iter().flatten();
    let dev: Vec<&Dependency> =
        if include_dev { manifest.dev_dependencies.iter().flatten().collect() } else { Vec::new() };
    for dep in deps.chain(dev) {
        let dep = canonicalize_dependency_path_relative_to_with_file_source(base_path, dep.clone(), file_source)?;
        // Resolve workspace deps early - converts to Location::Local with an absolute path.
        let dep = if dep.location == Location::Workspace {
            resolve_workspace_dependency_with_file_source(base_path, dep, file_source)?
        } else {
            dep
        };
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
            if file_source.is_dir(path) && file_source.exists(&manifest_path) {
                let child = Manifest::read_from_file_source(manifest_path, file_source)?;
                // dev_dependencies are not transitive.
                collect_declared_deps_recursive(path, &child, false, declared, file_source)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_package(base: &str) -> Package {
        Package {
            base_directory: PathBuf::from(base),
            compilation_units: Vec::new(),
            manifest: Manifest {
                program: "demo.aleo".to_string(),
                version: "0.1.0".to_string(),
                description: String::new(),
                license: "MIT".to_string(),
                leo: "0.0.0".to_string(),
                dependencies: None,
                dev_dependencies: None,
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
        assert_eq!(pkg.unit_snapshots_directory("token"), PathBuf::from("/tmp/demo/build/token/snapshots"));
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
}
