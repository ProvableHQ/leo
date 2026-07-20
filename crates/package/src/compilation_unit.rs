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

use crate::{MAX_PROGRAM_SIZE, *};

use leo_errors::Result;
use leo_span::Symbol;

use snarkvm::prelude::{Program as SvmProgram, TestnetV0};

use indexmap::{IndexMap, IndexSet};
use std::path::Path;

/// Find the latest cached edition for a program in the local registry.
/// Returns None if no cached version exists.
fn find_cached_edition(cache_directory: &Path, name: &str) -> Option<u16> {
    let program_cache = cache_directory.join(name);
    if !program_cache.exists() {
        return None;
    }

    // List edition directories and find the highest one
    std::fs::read_dir(&program_cache)
        .ok()?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let file_name = entry.file_name();
            let name = file_name.to_str()?;
            name.parse::<u16>().ok()
        })
        .max()
}

/// The kind of a Leo compilation unit: a deployable program, a library, or a test.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PackageKind {
    /// A deployable program with a `main.leo` entry point.
    Program,
    /// A library with a `lib.leo` entry point; not directly deployable.
    Library,
    /// A test file; compiled only during `leo test`.
    Test,
}

impl PackageKind {
    pub fn is_program(&self) -> bool {
        matches!(self, Self::Program)
    }

    pub fn is_library(&self) -> bool {
        matches!(self, Self::Library)
    }

    pub fn is_test(&self) -> bool {
        matches!(self, Self::Test)
    }
}

/// Information about a single Leo compilation unit.
#[derive(Clone, Debug)]
pub struct CompilationUnit {
    // The name of the program. For local packages this is the bare name (no ".aleo" suffix,
    // e.g. `my_program` or `my_lib`). For network-fetched programs this includes the ".aleo"
    // suffix (e.g. `credits.aleo`). TODO: unify the invariant so the suffix is always absent.
    pub name: Symbol,
    pub data: ProgramData,
    pub edition: Option<u16>,
    pub dependencies: IndexSet<Dependency>,
    pub is_local: bool,
    pub kind: PackageKind,
}

impl CompilationUnit {
    /// Given the location `path` of a `.aleo` file, read the filesystem
    /// to obtain a `CompilationUnit`.
    pub fn from_aleo_path<P: AsRef<Path>>(name: Symbol, path: P, map: &IndexMap<Symbol, Dependency>) -> Result<Self> {
        Self::from_aleo_path_impl(name, path.as_ref(), map)
    }

    fn from_aleo_path_impl(name: Symbol, path: &Path, map: &IndexMap<Symbol, Dependency>) -> Result<Self> {
        let bytecode = std::fs::read_to_string(path).map_err(|e| {
            crate::errors::util_file_io_error(format_args!("Trying to read aleo file at {}", path.display()), e)
        })?;

        let dependencies = parse_dependencies_from_aleo(name, &bytecode, map)?;

        Ok(CompilationUnit {
            name,
            data: ProgramData::Bytecode(bytecode),
            edition: None,
            dependencies,
            is_local: true,
            kind: PackageKind::Program,
        })
    }

    /// Given the location `path` of a local Leo package, read the filesystem
    /// to obtain a `CompilationUnit`.
    pub fn from_package_path<P: AsRef<Path>>(name: Symbol, path: P) -> Result<Self> {
        Self::from_package_path_impl(name, path.as_ref())
    }

    fn from_package_path_impl(name: Symbol, path: &Path) -> Result<Self> {
        let manifest = Manifest::read_from_file(path.join(MANIFEST_FILENAME))?;
        let manifest_symbol = crate::symbol(&manifest.program)?;
        if name != manifest_symbol {
            return Err(
                crate::errors::conflicting_manifest(format_args!("{name}"), format_args!("{manifest_symbol}")).into()
            );
        }
        let source_directory = path.join(SOURCE_DIRECTORY);
        source_directory.read_dir().map_err(|e| {
            crate::errors::util_file_io_error(
                format_args!("Failed to read directory {}", source_directory.display()),
                e,
            )
        })?;

        let main_path = source_directory.join(MAIN_FILENAME);
        let lib_path = source_directory.join(LIB_FILENAME);

        let (source_path, kind) = match (main_path.exists(), lib_path.exists()) {
            (true, true) => {
                return Err(crate::errors::ambiguous_entry_file(
                    source_directory.display(),
                    MAIN_FILENAME,
                    LIB_FILENAME,
                )
                .into());
            }
            (true, false) => (main_path, PackageKind::Program),
            (false, true) => (lib_path, PackageKind::Library),
            (false, false) => {
                return Err(
                    crate::errors::invalid_entry_file(source_directory.display(), MAIN_FILENAME, LIB_FILENAME).into()
                );
            }
        };

        Ok(CompilationUnit {
            name,
            data: ProgramData::SourcePath { directory: path.to_path_buf(), source: source_path },
            edition: None,
            dependencies: manifest
                .dependencies
                .unwrap_or_default()
                .into_iter()
                .map(|dependency| {
                    let dep = canonicalize_dependency_path_relative_to(path, dependency)?;
                    if dep.location == Location::Workspace { resolve_workspace_dependency(path, dep) } else { Ok(dep) }
                })
                .collect::<Result<IndexSet<_>, _>>()?,
            is_local: true,
            kind,
        })
    }

    /// Given the path to the source file of a test, create a `CompilationUnit`.
    ///
    /// Unlike `CompilationUnit::from_package_path`, the path is to the source file,
    /// and the name of the program is determined from the filename.
    ///
    /// `main_program` must be provided since every test is dependent on it.
    pub fn from_test_path<P: AsRef<Path>>(source_path: P, main_program: Dependency) -> Result<Self> {
        Self::from_path_test_impl(source_path.as_ref(), main_program)
    }

    fn from_path_test_impl(source_path: &Path, main_program: Dependency) -> Result<Self> {
        let name = filename_no_leo_extension(source_path)
            .ok_or_else(|| crate::errors::failed_path(source_path.display(), ""))?;
        let test_directory = source_path.parent().ok_or_else(|| {
            crate::errors::failed_to_open_file(format_args!(
                "Failed to find directory for test {}",
                source_path.display()
            ))
        })?;
        let package_directory = test_directory.parent().ok_or_else(|| {
            crate::errors::failed_to_open_file(format_args!(
                "Failed to find package for test {}",
                source_path.display()
            ))
        })?;
        let manifest = Manifest::read_from_file(package_directory.join(MANIFEST_FILENAME))?;
        // Like Cargo, tests see the package's `dependencies` plus test-only `dev_dependencies`; a
        // library in both lists is redundant but harmless, since the `IndexSet` dedups the entries.
        let mut dependencies = manifest
            .dependencies
            .into_iter()
            .flatten()
            .chain(manifest.dev_dependencies.into_iter().flatten())
            .map(|dependency| {
                let dep = canonicalize_dependency_path_relative_to(package_directory, dependency)?;
                if dep.location == Location::Workspace {
                    resolve_workspace_dependency(package_directory, dep)
                } else {
                    Ok(dep)
                }
            })
            .collect::<Result<IndexSet<_>, _>>()?;
        dependencies.insert(main_program);

        Ok(CompilationUnit {
            name: Symbol::intern(&(name.to_owned() + ".aleo")),
            edition: None,
            data: ProgramData::SourcePath {
                directory: test_directory.to_path_buf(),
                source: source_path.to_path_buf(),
            },
            dependencies,
            is_local: true,
            kind: PackageKind::Test,
        })
    }

    /// Given an Aleo program on a network, fetch it to build a `CompilationUnit`.
    /// If no edition is found, the latest edition is pulled from the network.
    pub fn fetch<P: AsRef<Path>>(
        name: Symbol,
        edition: Option<u16>,
        home_path: P,
        network: NetworkName,
        endpoint: &str,
        no_cache: bool,
        network_retries: u32,
    ) -> Result<Self> {
        Self::fetch_impl(name, edition, home_path.as_ref(), network, endpoint, no_cache, network_retries)
    }

    fn fetch_impl(
        name: Symbol,
        edition: Option<u16>,
        home_path: &Path,
        network: NetworkName,
        endpoint: &str,
        no_cache: bool,
        network_retries: u32,
    ) -> Result<Self> {
        // Callers may pass the name with or without the ".aleo" suffix; normalise to bare name
        // here so cache paths and network URLs are constructed consistently.
        let name = Symbol::intern(name.to_string().strip_suffix(".aleo").unwrap_or(&name.to_string()));

        // It's not a local program; let's check the cache.
        let cache_directory = home_path.join(format!("registry/{network}"));

        // If the edition is not specified, try to find a cached version first,
        // then fall back to querying the network for the latest edition.
        let edition = match edition {
            // Credits program always has edition 0.
            _ if name == Symbol::intern("credits") => 0,
            Some(edition) => edition,
            None if !no_cache => {
                // Check if we have a cached version - avoid network call if possible.
                match find_cached_edition(&cache_directory, &name.to_string()) {
                    Some(cached_edition) => cached_edition,
                    None => crate::fetch_latest_edition(&name.to_string(), endpoint, network, network_retries)?,
                }
            }
            // no_cache is set - user wants fresh data from network.
            None => crate::fetch_latest_edition(&name.to_string(), endpoint, network, network_retries)?,
        };

        // Define the full cache path for the program.

        // Build cache paths.
        let cache_directory = cache_directory.join(format!("{name}/{edition}"));
        let full_cache_path = cache_directory.join(format!("{name}.aleo"));
        if !cache_directory.exists() {
            // Create directory if it doesn't exist.
            std::fs::create_dir_all(&cache_directory).map_err(|err| {
                crate::errors::util_file_io_error(format!("Could not write path {}", cache_directory.display()), err)
            })?;
        }

        // Get the existing bytecode if the file exists.
        let existing_bytecode = match full_cache_path.exists() {
            false => None,
            true => {
                let existing_contents = std::fs::read_to_string(&full_cache_path).map_err(|e| {
                    crate::errors::util_file_io_error(
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
                let contents = fetch_from_network(&primary_url, network_retries)
                    .or_else(|_| fetch_from_network(&secondary_url, network_retries))
                    .map_err(|err| {
                        crate::errors::failed_to_retrieve_from_endpoint(
                            primary_url,
                            format_args!("Failed to fetch program `{name}` from network `{network}`: {err}"),
                        )
                    })?;

                // If the file already exists, compare it to the new contents.
                if let Some(existing_contents) = existing
                    && existing_contents != contents
                {
                    println!(
                        "Warning: The cached file at `{}` is different from the one fetched from the network. The cached file will be overwritten.",
                        full_cache_path.display()
                    );
                }

                // Write the bytecode to the cache.
                std::fs::write(&full_cache_path, &contents).map_err(|err| {
                    crate::errors::util_file_io_error(
                        format_args!("Could not open file `{}`", full_cache_path.display()),
                        err,
                    )
                })?;

                contents
            }
        };

        let dependencies = parse_dependencies_from_aleo(name, &bytecode, &IndexMap::new())?;

        Ok(CompilationUnit {
            // Network programs store the name with the ".aleo" suffix (unlike local packages).
            // TODO: unify the invariant so the suffix is always absent.
            name: Symbol::intern(&(name.to_string() + ".aleo")),
            data: ProgramData::Bytecode(bytecode),
            edition: Some(edition),
            dependencies,
            is_local: false,
            kind: PackageKind::Program,
        })
    }

    /// Resolve a git dependency, check it out, and build a `CompilationUnit`.
    pub fn from_git(
        name: Symbol,
        dependency: &Dependency,
        home_path: &Path,
        old_lock: &Lock,
        new_lock: &mut Lock,
        offline: bool,
        declared_deps: &IndexMap<Symbol, Dependency>,
    ) -> Result<Self> {
        let git = dependency
            .git
            .as_ref()
            .ok_or_else(|| crate::errors::invalid_manifest_dependency(&dependency.name, "missing `git` URL"))?;
        let url = git.url.as_str();
        let reference =
            git.reference().map_err(|reason| crate::errors::invalid_manifest_dependency(&dependency.name, reason))?;
        let reference_str = reference.lock_string();

        // Reuse a resolution of the same `(url, reference)` already performed in this build, so
        // all dependencies into one repository see the same commit and it is cloned only once.
        let memoized = new_lock
            .commit_for_source(url, &reference_str)
            .map(|commit| (crate::git::checkout_dir(home_path, url, commit), commit.to_string()))
            .filter(|(dir, _)| dir.is_dir());
        let (checkout, commit) = match memoized {
            Some(hit) => hit,
            None => {
                let locked = old_lock.commit_for(&dependency.name, url, &reference_str).map(str::to_string);
                crate::git::resolve(home_path, &dependency.name, url, &reference, locked.as_deref(), offline)?
            }
        };
        new_lock.record(dependency.name.clone(), url.to_string(), reference_str, commit);

        let located = find_in_checkout(&checkout, &dependency.name)?;
        let mut unit = if located.extension().and_then(|e| e.to_str()) == Some("aleo") && located.is_file() {
            Self::from_aleo_path(name, located, declared_deps)?
        } else {
            Self::from_package_path(name, located)?
        };

        // Intra-checkout deps (workspace siblings, local paths) become git deps on the same source
        // so every route to a package in this repository is the same dependency.
        unit.dependencies = unit
            .dependencies
            .into_iter()
            .map(|dep| {
                if dep.location == Location::Local && dep.path.as_ref().is_some_and(|p| p.starts_with(&checkout)) {
                    Dependency {
                        name: dep.name,
                        location: Location::Git,
                        path: None,
                        edition: None,
                        git: Some(git.clone()),
                    }
                } else {
                    dep
                }
            })
            .collect();
        Ok(unit)
    }
}

/// Locate the package named `dep_name` within a git checkout (by name, Cargo-style): a package
/// directory whose `program.json` declares it (searched recursively), else a `<name>.aleo` file at
/// the root. A directory declaring exactly the requested form wins over the alternate (`.aleo`)
/// form; multiple candidates for the chosen form are an error.
pub fn find_in_checkout(checkout: &Path, dep_name: &str) -> Result<std::path::PathBuf> {
    // Match both name forms so callers may pass a library (`foo`) or program (`foo.aleo`) name.
    let bare = crate::bare_unit_name(dep_name);
    let with_aleo = format!("{bare}.aleo");
    let (requested, alternate) =
        if dep_name.ends_with(".aleo") { (with_aleo.as_str(), bare) } else { (bare, with_aleo.as_str()) };

    let mut exact = Vec::new();
    let mut alt = Vec::new();
    collect_matching_manifest_dirs(checkout, requested, alternate, 0, &mut exact, &mut alt);
    let matches = if exact.is_empty() { alt } else { exact };
    if matches.len() > 1 {
        let listed = matches
            .iter()
            .map(|d| d.strip_prefix(checkout).unwrap_or(d).display().to_string())
            .collect::<Vec<_>>()
            .join("`, `");
        return Err(crate::errors::git_ambiguous_package(dep_name, listed).into());
    }
    if let Some(dir) = matches.into_iter().next() {
        return Ok(dir);
    }
    let aleo = checkout.join(&with_aleo);
    if aleo.is_file() {
        return Ok(aleo);
    }
    Err(crate::errors::git_package_not_found(dep_name, checkout.display()).into())
}

/// Recursively search `dir` (up to a bounded depth) for directories whose `program.json` declares
/// `requested` (collected into `exact`) or `alternate` (collected into `alt`).
fn collect_matching_manifest_dirs(
    dir: &Path,
    requested: &str,
    alternate: &str,
    depth: usize,
    exact: &mut Vec<std::path::PathBuf>,
    alt: &mut Vec<std::path::PathBuf>,
) {
    // Enough to reach workspace members, and bounds the scan of a deep repository.
    const MAX_DEPTH: usize = 6;
    if depth > MAX_DEPTH {
        return;
    }

    // Just the `program` field, parsed without validation so a malformed sibling can't fail the search.
    #[derive(serde::Deserialize)]
    struct ManifestProgramOnly {
        program: String,
    }

    let manifest = dir.join(MANIFEST_FILENAME);
    if manifest.is_file()
        && !manifest.is_symlink()
        && let Ok(contents) = std::fs::read_to_string(&manifest)
        && let Ok(parsed) = serde_json::from_str::<ManifestProgramOnly>(&contents)
    {
        if parsed.program == requested {
            exact.push(dir.to_path_buf());
        } else if parsed.program == alternate {
            alt.push(dir.to_path_buf());
        }
    }

    // Sort so any ambiguity report is deterministic.
    let mut entries: Vec<_> = std::fs::read_dir(dir).ok().into_iter().flatten().flatten().map(|e| e.path()).collect();
    entries.sort();
    for path in entries {
        // Don't follow symlinks: a repo symlink could let the search escape the checkout.
        if path.is_symlink() || !path.is_dir() {
            continue;
        }
        // Skip dotfiles/VCS and build output directories.
        let Some(name) = path.file_name().map(|n| n.to_string_lossy()) else {
            continue;
        };
        if name.starts_with('.') || name == BUILD_DIRECTORY || name == "target" {
            continue;
        }
        collect_matching_manifest_dirs(&path, requested, alternate, depth + 1, exact, alt);
    }
}

/// Canonicalize the path in `dependency` relative to `base`. Absolute paths are canonicalized too
/// so git-checkout containment checks see resolved symlinks and no `..` components.
pub(crate) fn canonicalize_dependency_path_relative_to(base: &Path, mut dependency: Dependency) -> Result<Dependency> {
    if let Some(path) = &mut dependency.path {
        let joined = if path.is_absolute() { path.clone() } else { base.join(&path) };
        *path = joined.canonicalize().map_err(|e| crate::errors::failed_path(joined.display(), e))?;
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
        return Err(leo_errors::LeoError::Backtraced(crate::errors::program_size_limit_exceeded(
            name,
            program_size,
            MAX_PROGRAM_SIZE,
        )));
    }

    // Parse the bytecode into an SVM program.
    let svm_program: SvmProgram<TestnetV0> =
        bytecode.parse().map_err(|_| crate::errors::snarkvm_parsing_error(name))?;
    let dependencies = svm_program
        .imports()
        .keys()
        .map(|program_id| {
            // If the dependency already exists, use it.
            // Otherwise, assume it's a network dependency.
            if let Some(dependency) = existing.get(&Symbol::intern(&program_id.to_string())) {
                dependency.clone()
            } else {
                let name = program_id.to_string();
                Dependency { name, location: Location::Network, path: None, edition: None, ..Default::default() }
            }
        })
        .collect();
    Ok(dependencies)
}
