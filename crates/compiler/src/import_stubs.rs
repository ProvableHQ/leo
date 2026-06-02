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

//! Package/manifest-driven import stub loading, shared between `leo build`
//! and the LSP's semantic analysis. Reads from the real filesystem and pulls
//! in the full `snarkvm` umbrella crate via `disassemble_dependency_bytecode`,
//! so the whole module is native-only — `crates/compiler/src/lib.rs` gates
//! the `mod import_stubs;` declaration on `not(target_arch = "wasm32")`.

use crate::{Compiler, CompilerOptions, disassemble_dependency_bytecode};

use leo_ast::{AleoProgram, FunctionStub, Identifier, NetworkName, NodeBuilder, Program, ProgramId, Stub};
use leo_errors::{Handler, Result};
use leo_package::{
    CompilationUnit,
    Dependency,
    Location,
    MANIFEST_FILENAME,
    Manifest,
    PackageKind,
    ProgramData,
    resolve_workspace_dependency,
};
use leo_span::{
    Symbol,
    create_session_if_not_set_then,
    file_source::{DiskFileSource, FileSource},
};

use indexmap::{IndexMap, map::Entry};
use std::{
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

/// Import stubs together with the filesystem inputs that invalidate them.
pub struct LoadedImportStubs {
    /// Import stubs available for compiler or LSP frontend analysis.
    pub stubs: IndexMap<Symbol, Stub>,
    /// Package inputs whose metadata changes should force a stub reload.
    pub watch_paths: Vec<PathBuf>,
}

/// Loads only locally resolvable dependency stubs for a package.
///
/// The LSP should not fetch or install dependencies while the user is typing, so
/// this helper walks the local manifest tree, builds stubs for local packages and
/// checked-in `.aleo` files, and silently skips network-only dependencies.
///
/// The returned `watch_paths` cover the manifests and source files that can
/// change the stub set. Editor caches can hash or stat those paths to know when
/// dependency-backed semantic state must be rebuilt.
pub fn load_import_stubs_for_package(package_root: &Path, network: NetworkName) -> Result<LoadedImportStubs> {
    load_import_stubs_for_package_with_file_source(package_root, network, &DiskFileSource)
}

/// Load local dependency stubs using an explicit file source for Leo source reads.
///
/// This variant lets editor integrations serve unsaved overlays and record the
/// exact disk bytes used for dependency source stubs. Manifest discovery still
/// reads the real filesystem because dependencies are package-level metadata,
/// but every parsed Leo source file flows through `file_source`.
pub fn load_import_stubs_for_package_with_file_source(
    package_root: &Path,
    network: NetworkName,
    file_source: &dyn FileSource,
) -> Result<LoadedImportStubs> {
    create_session_if_not_set_then(|_| {
        let package_root =
            package_root.canonicalize().map_err(|error| crate::errors::failed_path(package_root.display(), error))?;
        let declared_dependencies = collect_local_declared_dependencies(&package_root)?;
        let mut import_stubs = IndexMap::new();
        let mut watch_paths = vec![package_root.join(MANIFEST_FILENAME)];

        for (name, dependency) in &declared_dependencies {
            let Some(path) = dependency.path.as_ref() else {
                continue;
            };

            let unit = if path.extension().is_some_and(|extension| extension == "aleo") && path.is_file() {
                watch_paths.push(path.clone());
                CompilationUnit::from_aleo_path(*name, path, &declared_dependencies, network)?
            } else {
                let unit = CompilationUnit::from_package_path(*name, path)?;
                watch_paths.extend(unit_watch_paths(&unit, file_source)?);
                unit
            };

            let stub = match &unit.data {
                ProgramData::Bytecode(bytecode) => disassemble_dependency_bytecode(unit.name, bytecode, network)?,
                ProgramData::SourcePath { directory, source } => load_source_dependency_stub(
                    &unit,
                    source,
                    dependency_source_directory(directory, source),
                    network,
                    file_source,
                )?,
            };
            import_stubs.insert(unit.name, stub);
        }

        watch_paths.sort();
        watch_paths.dedup();

        Ok(LoadedImportStubs { stubs: import_stubs, watch_paths })
    })
}

/// Return the directory root the parser should scan for sibling Leo modules.
fn dependency_source_directory(directory: &Path, source: &Path) -> PathBuf {
    let source_root = directory.join("src");
    if source.starts_with(&source_root) { source_root } else { directory.to_path_buf() }
}

/// Collect the transitive set of manifest-declared local dependencies.
///
/// Network dependencies are intentionally excluded here because editor semantic
/// analysis must stay local-only.
fn collect_local_declared_dependencies(package_root: &Path) -> Result<IndexMap<Symbol, Dependency>> {
    let manifest = Manifest::read_from_file(package_root.join(MANIFEST_FILENAME))?;
    let mut declared = IndexMap::new();
    collect_local_declared_dependencies_recursive(package_root, &manifest, &mut declared)?;
    Ok(declared)
}

/// Walk local manifests recursively and record each dependency once.
fn collect_local_declared_dependencies_recursive(
    base_path: &Path,
    manifest: &Manifest,
    declared: &mut IndexMap<Symbol, Dependency>,
) -> Result<()> {
    for dependency in manifest.dependencies.iter().flatten() {
        let dependency = normalize_local_dependency(base_path, dependency.clone())?;
        // Resolve workspace deps early - converts to Location::Local with an absolute path.
        let dependency = if dependency.location == Location::Workspace {
            resolve_workspace_dependency(base_path, dependency)?
        } else {
            dependency
        };
        if dependency.location != Location::Local {
            continue;
        }

        let Some(path) = dependency.path.as_ref() else {
            continue;
        };
        let symbol = Symbol::intern(&dependency.name);

        match declared.entry(symbol) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(entry) => {
                entry.insert(dependency.clone());
                let manifest_path = path.join(MANIFEST_FILENAME);
                if path.is_dir() && manifest_path.is_file() {
                    let child = Manifest::read_from_file(manifest_path)?;
                    collect_local_declared_dependencies_recursive(path, &child, declared)?;
                }
            }
        }
    }

    Ok(())
}

/// Canonicalize a local dependency path relative to the manifest that declared it.
fn normalize_local_dependency(base_path: &Path, mut dependency: Dependency) -> Result<Dependency> {
    if let Some(path) = dependency.path.as_mut()
        && !path.is_absolute()
    {
        let joined = base_path.join(&*path);
        *path = joined.canonicalize().map_err(|error| crate::errors::failed_path(joined.display(), error))?;
    }

    Ok(dependency)
}

/// Return the manifest and source files whose metadata should invalidate one stubbed unit.
fn unit_watch_paths(unit: &CompilationUnit, file_source: &dyn FileSource) -> Result<Vec<PathBuf>> {
    let ProgramData::SourcePath { directory, source } = &unit.data else {
        return Ok(Vec::new());
    };

    let source_directory = dependency_source_directory(directory, source);
    let mut watch_paths = vec![directory.join(MANIFEST_FILENAME), source_directory.clone(), source.clone()];
    if source_directory.is_dir() {
        collect_source_directories(&source_directory, &mut watch_paths)?;
        let mut modules = file_source
            .list_leo_files(&source_directory, source)
            .map_err(|error| crate::errors::file_read_error(source_directory.display().to_string(), error))?;
        watch_paths.append(&mut modules);
    }

    Ok(watch_paths)
}

/// Collect source directories whose mtimes signal nested module creation/removal.
fn collect_source_directories(dir: &Path, watch_paths: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).map_err(|error| crate::errors::file_read_error(dir.display().to_string(), error))? {
        let entry = entry.map_err(|error| crate::errors::file_read_error(dir.display().to_string(), error))?;
        let path = entry.path();
        if path.is_dir() {
            // Watching only existing `.leo` files misses the first file added to
            // an already-existing nested module directory. Include directories
            // so LSP-side cache revisions notice those create/remove events.
            watch_paths.push(path.clone());
            collect_source_directories(&path, watch_paths)?;
        }
    }
    Ok(())
}

/// Parse a local dependency just far enough to recover the public interface
/// stub consumed by downstream import resolution.
fn load_source_dependency_stub(
    unit: &CompilationUnit,
    source: &Path,
    source_directory: PathBuf,
    network: NetworkName,
    file_source: &dyn FileSource,
) -> Result<Stub> {
    let handler = Handler::default();
    let node_builder = Rc::new(NodeBuilder::default());
    let mut compiler = Compiler::new(
        Some(unit.name.to_string()),
        false,
        handler,
        node_builder,
        PathBuf::default(),
        Some(CompilerOptions::default()),
        IndexMap::new(),
        network,
    );

    match unit.kind {
        PackageKind::Library => {
            let library_name = Symbol::intern(&unit.name.to_string());
            let library = compiler.parse_library_from_directory_with_file_source(
                library_name,
                source,
                &source_directory,
                file_source,
            )?;
            Ok(library.into())
        }
        PackageKind::Program | PackageKind::Test => {
            let program =
                compiler.parse_program_from_directory_with_file_source(source, &source_directory, file_source)?;
            Ok(extract_program_interface_stub(unit.name, &program))
        }
    }
}

/// Build the public interface stub for a source dependency program.
fn extract_program_interface_stub(_program_name: Symbol, program: &Program) -> Stub {
    let scope = program.program_scopes.values().next().expect("program AST should contain one program scope");

    // Source dependencies contribute only their public interface to the import
    // graph. Build the same stub shape we would get from disassembled bytecode
    // so downstream passes and the LSP can treat source and bytecode imports
    // uniformly.
    let functions = scope
        .functions
        .iter()
        .map(|(sym, func)| {
            (*sym, FunctionStub {
                annotations: func.annotations.clone(),
                variant: func.variant,
                identifier: func.identifier,
                input: func.input.clone(),
                output: func.output.clone(),
                output_type: func.output_type.clone(),
                span: func.span,
                id: func.id,
            })
        })
        .collect();

    let imports = program
        .imports
        .keys()
        .map(|sym| {
            let sym_str = sym.to_string();
            // Import stubs track bare program names and always use the `aleo`
            // network identifier, matching the normalized form produced by the
            // bytecode disassembler.
            let name_only = sym_str.strip_suffix(".aleo").unwrap_or(&sym_str);
            ProgramId {
                name: Identifier { name: Symbol::intern(name_only), span: Default::default(), id: Default::default() },
                network: Identifier { name: Symbol::intern("aleo"), span: Default::default(), id: Default::default() },
            }
        })
        .collect();

    AleoProgram {
        imports,
        stub_id: scope.program_id,
        consts: scope.consts.clone(),
        composites: scope.composites.clone(),
        mappings: scope.mappings.clone(),
        functions,
        span: scope.span,
    }
    .into()
}
