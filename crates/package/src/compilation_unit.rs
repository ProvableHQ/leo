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
use leo_span::{Symbol, file_source::FileSource};

#[cfg(not(target_arch = "wasm32"))]
use snarkvm::prelude::{Program as SvmProgram, TestnetV0};

use indexmap::{IndexMap, IndexSet};
use std::path::{Path, PathBuf};

/// Where the bytecode or source of a compilation unit lives.
///
/// - `Bytecode` is an already-compiled `.aleo` program: typically a network
///   dependency we fetched, or a local file dropped into the dep tree.
/// - `SourcePath` is a Leo package or test source — `directory` is the
///   package root (for `from_package_path`) or the test directory (for
///   `from_test_path`); `source` is the entry file the parser starts at.
#[derive(Clone, Debug)]
pub enum ProgramData {
    Bytecode(String),
    SourcePath { directory: PathBuf, source: PathBuf },
}

/// Find the latest cached edition for a program in the local registry.
/// Returns None if no cached version exists.
#[cfg(not(target_arch = "wasm32"))]
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
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_aleo_path<P: AsRef<Path>>(name: Symbol, path: P, map: &IndexMap<Symbol, Dependency>) -> Result<Self> {
        Self::from_aleo_path_with_file_source(name, path, map, &leo_span::file_source::DiskFileSource)
    }

    /// Read an `.aleo` file via an explicit [`FileSource`].
    ///
    /// Resolves paths against `file_source` instead of the real filesystem so
    /// the same code path serves wasm callers using `InMemoryFileSource`.
    pub fn from_aleo_path_with_file_source<P: AsRef<Path>>(
        name: Symbol,
        path: P,
        map: &IndexMap<Symbol, Dependency>,
        file_source: &impl FileSource,
    ) -> Result<Self> {
        let path = path.as_ref();
        let bytecode = file_source.read_file(path).map_err(|e| {
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
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_package_path<P: AsRef<Path>>(name: Symbol, path: P) -> Result<Self> {
        Self::from_package_path_with_file_source(name, path, &leo_span::file_source::DiskFileSource)
    }

    /// Read a Leo package at `path` via an explicit [`FileSource`].
    pub fn from_package_path_with_file_source<P: AsRef<Path>>(
        name: Symbol,
        path: P,
        file_source: &impl FileSource,
    ) -> Result<Self> {
        let path = path.as_ref();
        let manifest = Manifest::read_from_file_source(path.join(MANIFEST_FILENAME), file_source)?;
        let manifest_symbol = crate::symbol(&manifest.program)?;
        if name != manifest_symbol {
            return Err(
                crate::errors::conflicting_manifest(format_args!("{name}"), format_args!("{manifest_symbol}")).into()
            );
        }
        let source_directory = path.join(SOURCE_DIRECTORY);
        if !file_source.is_dir(&source_directory) {
            return Err(crate::errors::util_file_io_error(
                format_args!("Failed to read directory {}", source_directory.display()),
                std::io::Error::new(std::io::ErrorKind::NotFound, source_directory.display().to_string()),
            )
            .into());
        }

        let main_path = source_directory.join(MAIN_FILENAME);
        let lib_path = source_directory.join(LIB_FILENAME);
        let main_present = file_source.is_file(&main_path);
        let lib_present = file_source.is_file(&lib_path);

        let (source_path, kind) = match (main_present, lib_present) {
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
                    let dep = resolve_dependency_path_relative_to(path, dependency, file_source)?;
                    if dep.location == Location::Workspace {
                        resolve_workspace_dependency_with_file_source(path, dep, file_source)
                    } else {
                        Ok(dep)
                    }
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
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_test_path<P: AsRef<Path>>(source_path: P, main_program: Dependency) -> Result<Self> {
        Self::from_test_path_with_file_source(source_path, main_program, &leo_span::file_source::DiskFileSource)
    }

    /// Read a test source via an explicit [`FileSource`].
    pub fn from_test_path_with_file_source<P: AsRef<Path>>(
        source_path: P,
        main_program: Dependency,
        file_source: &impl FileSource,
    ) -> Result<Self> {
        let source_path = source_path.as_ref();
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
        let manifest = Manifest::read_from_file_source(package_directory.join(MANIFEST_FILENAME), file_source)?;
        let mut dependencies = manifest
            .dev_dependencies
            .unwrap_or_default()
            .into_iter()
            .map(|dependency| {
                let dep = resolve_dependency_path_relative_to(package_directory, dependency, file_source)?;
                if dep.location == Location::Workspace {
                    resolve_workspace_dependency_with_file_source(package_directory, dep, file_source)
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
    ///
    /// Native-only — wasm callers can't reach the network; stage the bytecode
    /// in the file map and pass it as a local `.aleo` dep instead.
    #[cfg(not(target_arch = "wasm32"))]
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

    #[cfg(not(target_arch = "wasm32"))]
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
}

/// If `dependency` has a relative path, assume it's relative to `base` and resolve it.
///
/// On the real disk we canonicalize (resolves `..`, symlinks, etc.). When a
/// `FileSource` is supplied that isn't backed by the real disk (e.g. wasm's
/// virtual FS), we fall back to syntactic normalization — `..` is collapsed,
/// `.` is dropped, and the resulting path is verified to be addressable in
/// the source.
pub(crate) fn resolve_dependency_path_relative_to(
    base: &Path,
    mut dependency: Dependency,
    file_source: &impl FileSource,
) -> Result<Dependency> {
    if let Some(path) = &mut dependency.path
        && !path.is_absolute()
    {
        let joined = base.join(&path);
        *path = normalize_path_via_file_source(&joined, file_source)?;
    }
    Ok(dependency)
}

/// Native-only helper preserved for call sites that haven't been threaded
/// through a `FileSource` yet. Equivalent to
/// `resolve_dependency_path_relative_to(base, dep, &DiskFileSource)` and uses
/// real-disk `canonicalize` so error messages match the historical output.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn canonicalize_dependency_path_relative_to(base: &Path, mut dependency: Dependency) -> Result<Dependency> {
    if let Some(path) = &mut dependency.path
        && !path.is_absolute()
    {
        let joined = base.join(&path);
        *path = joined.canonicalize().map_err(|e| crate::errors::failed_path(joined.display(), e))?;
    }
    Ok(dependency)
}

/// Resolve a path against a [`FileSource`].
///
/// On `DiskFileSource` this still calls `canonicalize` (so symlinks resolve
/// the same way `leo build` has always done). On in-memory sources it falls
/// back to component normalization since there's no real disk to consult.
fn normalize_path_via_file_source(path: &Path, file_source: &impl FileSource) -> Result<PathBuf> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Heuristic: real-disk sources should use the original `canonicalize` for
        // backward-compatible error messages. We detect that by trying the
        // canonical path; if it succeeds we return it.
        if let Ok(canonical) = path.canonicalize() {
            let _ = file_source; // silence unused-warning when the read below isn't reached
            return Ok(canonical);
        }
    }
    let normalized = normalize_path_components(path);
    if !file_source.exists(&normalized) {
        return Err(crate::errors::failed_path(
            normalized.display(),
            std::io::Error::new(std::io::ErrorKind::NotFound, normalized.display().to_string()),
        )
        .into());
    }
    Ok(normalized)
}

/// Collapse `.` and `..` components without consulting any filesystem.
fn normalize_path_components(p: &Path) -> PathBuf {
    use std::path::Component;
    let mut out = PathBuf::new();
    for comp in p.components() {
        match comp {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// Workspace dependency resolution via [`FileSource`] (wasm-safe wrapper
/// around the native [`resolve_workspace_dependency`]).
pub(crate) fn resolve_workspace_dependency_with_file_source(
    base: &Path,
    dep: Dependency,
    _file_source: &impl FileSource,
) -> Result<Dependency> {
    // TODO: Once `Workspace::discover` has a `_with_file_source` variant, wire
    // it through here. For now this delegates to the native resolver, which
    // means `Location::Workspace` deps still require disk access in any wasm
    // caller — the typical wasm setup won't use workspace deps anyway since
    // the caller stages the file map directly.
    #[cfg(not(target_arch = "wasm32"))]
    {
        resolve_workspace_dependency(base, dep)
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = base;
        Err(crate::errors::failed_to_open_file(format_args!(
            "workspace dependency `{}` is not supported in wasm builds yet; declare the dep with an explicit `path` instead",
            dep.name
        ))
        .into())
    }
}

/// Parse the `.aleo` file's imports and construct `Dependency`s.
///
/// On native, we still validate the bytecode by parsing it through snarkVM's
/// full umbrella (`Program::parse`). On `wasm32` the full umbrella isn't in
/// the dep graph, so we extract `import X;` lines with the inline parser only.
/// Both paths share the same shape, so downstream `Package::graph_build`
/// behaves identically across targets.
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

    // Native: validate the full bytecode through snarkVM. Errors at this stage
    // surface as `snarkvm_parsing_error` so the user knows the `.aleo` file is
    // malformed (not just missing imports).
    #[cfg(not(target_arch = "wasm32"))]
    {
        bytecode.parse::<SvmProgram<TestnetV0>>().map_err(|_| crate::errors::snarkvm_parsing_error(name))?;
    }

    // Extract the import lines. The `.aleo` grammar declares all imports at the
    // top of the file as `import <name>;` (whitespace-flexible) before the
    // `program` block, so a small line scanner is sufficient on both targets.
    let imports = extract_aleo_import_names(bytecode);
    let dependencies = imports
        .into_iter()
        .map(|import_name| {
            let sym = Symbol::intern(&import_name);
            if let Some(dependency) = existing.get(&sym) {
                dependency.clone()
            } else {
                Dependency { name: import_name, location: Location::Network, path: None, edition: None }
            }
        })
        .collect();
    Ok(dependencies)
}

/// Extract the program names referenced by `import <name>;` statements at the
/// top of an `.aleo` source file.
///
/// Stops at the first `program ` line — Aleo imports are only legal at the top.
/// Whitespace-tolerant, accepts both `import foo.aleo;` and `import foo;` (the
/// latter without a network suffix), and ignores comments / blank lines.
fn extract_aleo_import_names(bytecode: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in bytecode.lines() {
        let line = line.trim_start();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        if let Some(rest) = line.strip_prefix("import ") {
            let name = rest.trim_end_matches(';').trim();
            if !name.is_empty() {
                out.push(name.to_string());
            }
            continue;
        }
        // First non-import, non-blank, non-comment line ends the import block.
        break;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::extract_aleo_import_names;

    #[test]
    fn extract_imports_basic() {
        let src = "import foo.aleo;\nimport bar.aleo;\n\nprogram baz.aleo;\n";
        assert_eq!(extract_aleo_import_names(src), vec!["foo.aleo", "bar.aleo"]);
    }

    #[test]
    fn extract_imports_with_blank_lines_and_comments() {
        let src = "\
            // top-level comment\n\
            \n\
            import foo.aleo;\n\
            // another comment\n\
            import bar.aleo;\n\
            \n\
            program baz.aleo;\n\
            // imports below `program` are illegal in .aleo and not extracted\n\
        ";
        assert_eq!(extract_aleo_import_names(src), vec!["foo.aleo", "bar.aleo"]);
    }

    #[test]
    fn extract_imports_no_imports() {
        let src = "program baz.aleo;\nfunction main:\n    input r0 as u32.public;\n";
        assert!(extract_aleo_import_names(src).is_empty());
    }

    #[test]
    fn extract_imports_stops_at_program_block() {
        // Anything after a non-import, non-blank, non-comment line is ignored
        // — matches snarkVM's parser, which rejects imports below `program`.
        let src = "import foo.aleo;\nprogram baz.aleo;\nimport sneaky.aleo;\n";
        assert_eq!(extract_aleo_import_names(src), vec!["foo.aleo"]);
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn extract_imports_matches_snarkvm() {
        // Parity check: the inline parser produces the same set of import
        // names snarkVM's full bytecode parser would, for a realistic shape.
        use snarkvm::prelude::{Program as SvmProgram, TestnetV0};
        let src = "import foo.aleo;\nimport bar.aleo;\n\nprogram baz.aleo;\nfunction main:\n    input r0 as u32.public;\n    output r0 as u32.private;\n";
        let svm: SvmProgram<TestnetV0> = src.parse().expect("valid .aleo source");
        let svm_imports: Vec<String> = svm.imports().keys().map(|id| id.to_string()).collect();
        assert_eq!(extract_aleo_import_names(src), svm_imports);
    }
}
