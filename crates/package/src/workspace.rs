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

use crate::{Dependency, Location, MANIFEST_FILENAME, Manifest, errors};

use leo_ast::DiGraph;
use leo_errors::{Backtraced, Result};
use leo_span::file_source::{DiskFileSource, FileSource};

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub const WORKSPACE_MANIFEST_FILENAME: &str = "workspace.json";

/// The contents of a `workspace.json` manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceManifest {
    pub members: Vec<String>,
}

impl WorkspaceManifest {
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> std::result::Result<Self, Backtraced> {
        Self::read_from_file_source(path, &DiskFileSource)
    }

    /// FileSource-aware counterpart to [`Self::read_from_file`].
    pub fn read_from_file_source<P: AsRef<Path>>(
        path: P,
        file_source: &dyn FileSource,
    ) -> std::result::Result<Self, Backtraced> {
        let contents = file_source
            .read_file(path.as_ref())
            .map_err(|e| errors::workspace_manifest_error(path.as_ref().display(), e))?;
        serde_json::from_str(&contents).map_err(|e| errors::workspace_manifest_error(path.as_ref().display(), e))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> std::result::Result<(), Backtraced> {
        let mut contents = serde_json::to_string_pretty(self)
            .map_err(|e| errors::workspace_manifest_error(path.as_ref().display(), e))?;
        contents.push('\n');
        std::fs::write(&path, contents).map_err(|e| errors::workspace_manifest_error(path.as_ref().display(), e))
    }
}

/// A Leo workspace - a collection of member packages under a single root.
#[derive(Debug, Clone)]
pub struct Workspace {
    /// The canonicalized root directory containing `workspace.json`.
    pub root_directory: PathBuf,
    /// Member directories in dependency order, each an absolute path.
    pub member_paths: Vec<PathBuf>,
    /// Member program names (from each member's `program.json`), in the same order.
    pub member_names: Vec<String>,
}

impl Workspace {
    /// Read the workspace at `path` from the real filesystem.
    ///
    /// Returns `Ok(None)` if no `workspace.json` exists there.
    pub fn from_directory(path: &Path) -> Result<Option<Self>> {
        Self::from_directory_with_file_source(path, &DiskFileSource)
    }

    /// FileSource-aware counterpart to [`Self::from_directory`]. Used by the
    /// wasm build path to expand workspace manifests served from an in-memory
    /// file map; the native CLI passes `&DiskFileSource`.
    pub fn from_directory_with_file_source(path: &Path, file_source: &dyn FileSource) -> Result<Option<Self>> {
        let manifest_path = path.join(WORKSPACE_MANIFEST_FILENAME);
        if !file_source.is_file(&manifest_path) {
            return Ok(None);
        }

        let root_directory =
            file_source.canonicalize(path).map_err(|e| errors::workspace_manifest_error(manifest_path.display(), e))?;

        let manifest = WorkspaceManifest::read_from_file_source(&manifest_path, file_source)?;

        // Resolve and validate each member entry, expanding glob patterns relative to the root.
        let mut dir_to_name: Vec<(PathBuf, String)> = Vec::with_capacity(manifest.members.len());
        let mut seen: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();
        for member in &manifest.members {
            if is_glob_pattern(member) {
                let expanded = expand_member_pattern(&root_directory, member, file_source)?;
                if expanded.is_empty() {
                    tracing::warn!(
                        "workspace member glob `{member}` in {} matched no packages",
                        root_directory.display(),
                    );
                    continue;
                }
                for entry in expanded {
                    let record = load_member_record(&root_directory, &entry, file_source)?;
                    if seen.insert(record.0.clone()) {
                        dir_to_name.push(record);
                    }
                }
            } else {
                let record = load_member_record(&root_directory, member, file_source)?;
                if seen.insert(record.0.clone()) {
                    dir_to_name.push(record);
                }
            }
        }

        // Build a dependency graph to determine the correct build order.
        let ordered = order_members(&dir_to_name, file_source)?;

        let member_paths = ordered.iter().map(|(p, _)| p.clone()).collect();
        let member_names = ordered.into_iter().map(|(_, n)| n).collect();

        Ok(Some(Workspace { root_directory, member_paths, member_names }))
    }

    /// Walk up from `start_dir` looking for `workspace.json`, returning the
    /// fully resolved workspace. Native (real-disk) variant.
    pub fn discover(start_dir: &Path) -> Result<Option<Self>> {
        Self::discover_with_file_source(start_dir, &DiskFileSource)
    }

    /// FileSource-aware counterpart to [`Self::discover`].
    pub fn discover_with_file_source(start_dir: &Path, file_source: &dyn FileSource) -> Result<Option<Self>> {
        match discover_root(start_dir, file_source)? {
            Some(root) => Self::from_directory_with_file_source(&root, file_source),
            None => Ok(None),
        }
    }

    /// Find a member by directory name or program name (with or without `.aleo` suffix).
    pub fn find_member(&self, name: &str) -> Option<&PathBuf> {
        // Try matching by directory basename.
        if let Some(pos) = self.member_paths.iter().position(|p| p.file_name().and_then(|n| n.to_str()) == Some(name)) {
            return Some(&self.member_paths[pos]);
        }
        // Try matching by program name (exact or with/without .aleo).
        let name_with_aleo = if name.ends_with(".aleo") { name.to_string() } else { format!("{name}.aleo") };
        let name_without_aleo = name.strip_suffix(".aleo").unwrap_or(name);
        self.member_names.iter().zip(self.member_paths.iter()).find_map(|(prog_name, path)| {
            if prog_name == name || prog_name == &name_with_aleo || prog_name == name_without_aleo {
                Some(path)
            } else {
                None
            }
        })
    }

    /// Check whether a given canonicalized path is one of the member directories.
    /// Native-only — the wasm build doesn't need post-load membership checks.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn is_member(&self, path: &Path) -> bool {
        let Ok(canonical) = path.canonicalize() else {
            return false;
        };
        self.member_paths.iter().any(|p| p == &canonical)
    }

    /// If a workspace contains `member_dir`, append `member_dir` to its
    /// `workspace.json` (unless already covered by a literal entry or a glob).
    ///
    /// Native-only: writes the manifest back to disk.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn auto_register_member(member_dir: &Path) -> Result<bool> {
        let canonical_member = member_dir.canonicalize().map_err(|e| errors::failed_path(member_dir.display(), e))?;

        let Some(parent) = canonical_member.parent() else {
            return Ok(false);
        };
        // Only the workspace root is needed here, so locate it without resolving
        // members; that keeps `leo new` from failing when an unrelated existing
        // member is broken.
        let Some(root_directory) = discover_root(parent, &DiskFileSource)? else {
            return Ok(false);
        };

        let relative = match canonical_member.strip_prefix(&root_directory) {
            Ok(rel) => rel,
            Err(_) => {
                tracing::warn!(
                    "new package at `{}` is not inside the discovered workspace root `{}`; skipping auto-add",
                    canonical_member.display(),
                    root_directory.display(),
                );
                return Ok(false);
            }
        };
        let Some(relative_str) = relative.to_str() else {
            tracing::warn!("new package path `{}` is not valid UTF-8; skipping auto-add", canonical_member.display(),);
            return Ok(false);
        };
        let entry = relative_str.replace('\\', "/");

        // Re-read from disk in case the manifest was modified between discover and write,
        // and check coverage against those fresh entries rather than the stale snapshot.
        let manifest_path = root_directory.join(WORKSPACE_MANIFEST_FILENAME);
        let mut manifest = WorkspaceManifest::read_from_file(&manifest_path)?;

        if pattern_matches_relative(&manifest.members, &entry) {
            return Ok(false);
        }

        manifest.members.push(entry);
        manifest.write_to_file(&manifest_path)?;
        Ok(true)
    }

    /// Create a fresh workspace skeleton named `name` inside `parent`.
    ///
    /// Writes a `workspace.json` with an empty `members` array. The caller is
    /// responsible for ensuring `parent` exists.
    ///
    /// Returns the absolute path of the new workspace directory.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn initialize_skeleton(name: &str, parent: &Path) -> Result<PathBuf> {
        if !crate::is_valid_library_name(name) {
            return Err(errors::cli_invalid_package_name("workspace", name).into());
        }

        let parent = parent.canonicalize().map_err(|e| errors::failed_path(parent.display(), e))?;
        let full_path = parent.join(name);

        if full_path.exists() {
            return Err(errors::failed_to_initialize_package(name, &full_path, "Directory already exists").into());
        }

        std::fs::create_dir(&full_path).map_err(|e| errors::failed_to_initialize_package(name, &full_path, e))?;

        let manifest = WorkspaceManifest { members: Vec::new() };
        manifest.write_to_file(full_path.join(WORKSPACE_MANIFEST_FILENAME))?;

        Ok(full_path)
    }
}

/// Walk up from `start_dir` to find the directory containing `workspace.json`.
///
/// Returns the canonicalized workspace root, or `Ok(None)` if none is found.
/// Unlike [`Workspace::discover`], this does not resolve or validate members.
fn discover_root(start_dir: &Path, file_source: &dyn FileSource) -> Result<Option<PathBuf>> {
    let start =
        file_source.canonicalize(start_dir).map_err(|e| errors::workspace_manifest_error(start_dir.display(), e))?;
    let mut dir = start.as_path();
    loop {
        if file_source.is_file(&dir.join(WORKSPACE_MANIFEST_FILENAME)) {
            return Ok(Some(dir.to_path_buf()));
        }
        match dir.parent() {
            Some(parent) => dir = parent,
            None => return Ok(None),
        }
    }
}

/// Resolve a `Location::Workspace` dependency by looking up its name in the
/// enclosing workspace, returning a new `Dependency` with `Location::Local`
/// and the resolved absolute path.
///
/// `package_dir` is the directory of the package that declared the dependency.
pub fn resolve_workspace_dependency(package_dir: &Path, dep: Dependency) -> Result<Dependency> {
    resolve_workspace_dependency_with_file_source(package_dir, dep, &DiskFileSource)
}

/// FileSource-aware counterpart to [`resolve_workspace_dependency`].
pub fn resolve_workspace_dependency_with_file_source(
    package_dir: &Path,
    dep: Dependency,
    file_source: &dyn FileSource,
) -> Result<Dependency> {
    let workspace = Workspace::discover_with_file_source(package_dir, file_source)?
        .ok_or_else(|| errors::workspace_dep_outside_workspace(&dep.name))?;
    let member_path = workspace
        .find_member(&dep.name)
        .ok_or_else(|| errors::workspace_dep_member_not_found(&dep.name, workspace.root_directory.display()))?;
    Ok(Dependency { location: Location::Local, path: Some(member_path.clone()), ..dep })
}

/// Returns `true` if `s` contains any glob metacharacters (`*`, `?`, `[`).
fn is_glob_pattern(s: &str) -> bool {
    s.contains(['*', '?', '['])
}

/// Expand a glob pattern relative to `root` into a list of member directory
/// entries, each a forward-slash path relative to `root`.
///
/// Only directories containing a `program.json` are returned. Other matches
/// (files, directories without a manifest, non-UTF8 paths) are silently skipped.
///
/// Implementation walks every file under `root` via `file_source` (so the
/// wasm path can glob over an in-memory file map without `glob::glob`'s
/// filesystem dependency), then matches each candidate directory against
/// the glob pattern with `glob::Pattern`.
fn expand_member_pattern(root: &Path, pattern: &str, file_source: &dyn FileSource) -> Result<Vec<String>> {
    let pattern = glob::Pattern::new(pattern).map_err(|e| errors::workspace_manifest_error(pattern, e))?;
    let options = glob::MatchOptions { require_literal_separator: true, ..Default::default() };

    // Find each directory that holds a `program.json`. With an in-memory source
    // these are virtual; with disk we walk the real tree.
    let files =
        file_source.list_files_recursive(root).map_err(|e| errors::workspace_manifest_error(root.display(), e))?;

    let mut out = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    for file in files {
        if file.file_name().and_then(|s| s.to_str()) != Some(MANIFEST_FILENAME) {
            continue;
        }
        let Some(dir) = file.parent() else { continue };
        let Ok(relative) = dir.strip_prefix(root) else { continue };
        let Some(relative_str) = relative.to_str() else { continue };
        // Normalize to forward slashes so the entry round-trips cleanly on Windows.
        let normalized = relative_str.replace('\\', "/");
        if !pattern.matches_with(&normalized, options) {
            continue;
        }
        if seen.insert(normalized.clone()) {
            out.push(normalized);
        }
    }
    out.sort();
    Ok(out)
}

/// Check whether any of `patterns` matches `relative` either as a literal
/// entry or as a glob pattern.
fn pattern_matches_relative(patterns: &[String], relative: &str) -> bool {
    // Match with `require_literal_separator` so `*`/`?`/`[]` stop at `/`, mirroring
    // how `glob::glob` enumerates the filesystem (and leaving `**` free to cross).
    let options = glob::MatchOptions { require_literal_separator: true, ..Default::default() };
    patterns.iter().any(|p| {
        if is_glob_pattern(p) {
            glob::Pattern::new(p).map(|pat| pat.matches_with(relative, options)).unwrap_or(false)
        } else {
            p == relative
        }
    })
}

/// Load a single member's `(canonical_path, program_name)` pair, erroring if
/// the directory or its `program.json` is missing.
fn load_member_record(root: &Path, entry: &str, file_source: &dyn FileSource) -> Result<(PathBuf, String)> {
    let member_dir = root.join(entry);
    if !file_source.is_dir(&member_dir) {
        return Err(errors::workspace_member_not_found(entry, root.display()).into());
    }
    let member_manifest_path = member_dir.join(MANIFEST_FILENAME);
    if !file_source.is_file(&member_manifest_path) {
        return Err(errors::workspace_member_not_found(entry, root.display()).into());
    }
    let member_manifest = Manifest::read_from_file_source(&member_manifest_path, file_source)?;
    let canonical =
        file_source.canonicalize(&member_dir).map_err(|e| errors::workspace_manifest_error(member_dir.display(), e))?;
    // `root` is the canonicalized workspace root; reject members (e.g. `../sibling`)
    // that resolve outside it.
    if canonical.strip_prefix(root).is_err() {
        return Err(errors::workspace_member_outside_root(entry, root.display()).into());
    }
    Ok((canonical, member_manifest.program.clone()))
}

/// Determine the build order for workspace members by analysing cross-member
/// local dependencies.
///
/// Each member's `Manifest` is read to find `Location::Local` dependencies
/// whose paths resolve to other workspace member directories. Edges are added
/// to a `DiGraph` from dependent to dependency, and the graph is topologically
/// sorted so that dependencies appear before the members that depend on them.
fn order_members(members: &[(PathBuf, String)], file_source: &dyn FileSource) -> Result<Vec<(PathBuf, String)>> {
    // If there are 0 or 1 members, no ordering is needed.
    if members.len() <= 1 {
        return Ok(members.to_vec());
    }

    let mut graph = DiGraph::<String>::new(Default::default());

    // Index members by canonical path for quick lookup.
    let path_to_dir_name: std::collections::HashMap<&Path, &str> = members
        .iter()
        .filter_map(|(path, _)| {
            let dir_name = path.file_name()?.to_str()?;
            Some((path.as_path(), dir_name))
        })
        .collect();

    // Add all members as nodes.
    for (path, _) in members {
        let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or_default();
        graph.add_node(dir_name.to_string());
    }

    // Also index members by program name for workspace dep lookup.
    let name_to_dir_name: std::collections::HashMap<&str, &str> = members
        .iter()
        .filter_map(|(path, prog_name)| {
            let dir_name = path.file_name()?.to_str()?;
            Some((prog_name.as_str(), dir_name))
        })
        .collect();

    // Scan each member's manifest for local/workspace dependencies pointing to other members.
    for (member_path, _) in members {
        let member_dir_name = member_path.file_name().and_then(|n| n.to_str()).unwrap_or_default();
        let manifest_path = member_path.join(MANIFEST_FILENAME);
        let manifest = Manifest::read_from_file_source(&manifest_path, file_source)?;

        for dep in manifest.dependencies.iter().flatten() {
            let dep_dir_name = match dep.location {
                Location::Local => {
                    let Some(dep_path) = &dep.path else { continue };
                    let resolved = if dep_path.is_absolute() { dep_path.clone() } else { member_path.join(dep_path) };
                    let Ok(canonical) = file_source.canonicalize(&resolved) else { continue };
                    let Some(&name) = path_to_dir_name.get(canonical.as_path()) else { continue };
                    name
                }
                Location::Workspace => {
                    // Match by directory basename or program name.
                    if let Some(&name) = path_to_dir_name.values().find(|&&n| {
                        n == dep.name
                            || format!("{n}.aleo") == dep.name
                            || dep.name.strip_suffix(".aleo").is_some_and(|s| s == n)
                    }) {
                        name
                    } else if let Some(&name) = name_to_dir_name.get(dep.name.as_str()) {
                        name
                    } else {
                        // Also try with/without .aleo suffix on program name.
                        let alt = if dep.name.ends_with(".aleo") {
                            dep.name.strip_suffix(".aleo").unwrap().to_string()
                        } else {
                            format!("{}.aleo", dep.name)
                        };
                        let Some(&name) = name_to_dir_name.get(alt.as_str()) else { continue };
                        name
                    }
                }
                _ => continue,
            };
            graph.add_edge(member_dir_name.to_string(), dep_dir_name.to_string());
        }
    }

    let ordered = graph.post_order().map_err(|_| {
        errors::workspace_manifest_error("workspace.json", "circular dependency between workspace members")
    })?;

    // Map the ordered directory names back to (path, program_name) pairs.
    let name_to_member: std::collections::HashMap<&str, &(PathBuf, String)> = members
        .iter()
        .filter_map(|entry| {
            let dir_name = entry.0.file_name()?.to_str()?;
            Some((dir_name, entry))
        })
        .collect();

    Ok(ordered
        .iter()
        .filter_map(|dir_name| name_to_member.get(dir_name.as_str()).map(|e| (e.0.clone(), e.1.clone())))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;

    fn create_member(workspace_dir: &Path, name: &str, deps: &[(&str, &Path)]) {
        let member_dir = workspace_dir.join(name);
        std::fs::create_dir_all(member_dir.join("src")).unwrap();

        let program_name = format!("{name}.aleo");
        let dependencies: Vec<_> = deps
            .iter()
            .map(|(dep_name, dep_path)| crate::Dependency {
                name: format!("{dep_name}.aleo"),
                location: Location::Local,
                path: Some(dep_path.to_path_buf()),
                edition: None,
            })
            .collect();

        let manifest = Manifest {
            program: program_name,
            version: "0.1.0".to_string(),
            description: String::new(),
            license: "MIT".to_string(),
            leo: "0.0.0".to_string(),
            dependencies: if dependencies.is_empty() { None } else { Some(dependencies) },
            dev_dependencies: None,
        };

        manifest.write_to_file(member_dir.join(MANIFEST_FILENAME)).unwrap();

        // Write a minimal source file so the package is valid.
        std::fs::write(
            member_dir.join("src/main.leo"),
            format!("program {name}.aleo {{\n    @noupgrade\n    constructor() {{}}\n}}\n"),
        )
        .unwrap();
    }

    fn create_workspace(dir: &Path, members: &[&str]) {
        let manifest = WorkspaceManifest { members: members.iter().map(|s| s.to_string()).collect() };
        manifest.write_to_file(dir.join(WORKSPACE_MANIFEST_FILENAME)).unwrap();
    }

    #[test]
    fn workspace_manifest_round_trip() {
        let dir = temp_dir().join("ws_test_roundtrip");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let manifest = WorkspaceManifest { members: vec!["alpha".into(), "beta".into()] };
        let path = dir.join(WORKSPACE_MANIFEST_FILENAME);
        manifest.write_to_file(&path).unwrap();

        let loaded = WorkspaceManifest::read_from_file(&path).unwrap();
        assert_eq!(loaded.members, vec!["alpha", "beta"]);

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn workspace_from_directory_valid() {
        let dir = temp_dir().join("ws_test_valid");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        create_member(&dir, "alpha", &[]);
        create_member(&dir, "beta", &[]);
        create_workspace(&dir, &["alpha", "beta"]);

        let ws = Workspace::from_directory(&dir).unwrap().unwrap();
        assert_eq!(ws.member_paths.len(), 2);
        assert_eq!(ws.member_names.len(), 2);

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn workspace_from_directory_missing_member() {
        let dir = temp_dir().join("ws_test_missing");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        create_member(&dir, "alpha", &[]);
        // "beta" is listed but not created.
        create_workspace(&dir, &["alpha", "beta"]);

        let result = Workspace::from_directory(&dir);
        assert!(result.is_err());

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn workspace_discover_from_subdirectory() {
        let dir = temp_dir().join("ws_test_discover");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        create_member(&dir, "alpha", &[]);
        create_workspace(&dir, &["alpha"]);

        let member_dir = dir.join("alpha");
        let ws = Workspace::discover(&member_dir).unwrap().unwrap();
        assert_eq!(ws.root_directory, dir.canonicalize().unwrap());

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn workspace_discover_none() {
        let dir = temp_dir().join("ws_test_no_workspace");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let result = Workspace::discover(&dir).unwrap();
        assert!(result.is_none());

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn workspace_dependency_ordering() {
        let dir = temp_dir().join("ws_test_ordering");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let alpha_dir = dir.join("alpha");

        // alpha has no deps, beta depends on alpha.
        create_member(&dir, "alpha", &[]);
        create_member(&dir, "beta", &[("alpha", &alpha_dir)]);
        create_workspace(&dir, &["beta", "alpha"]); // intentionally wrong order

        let ws = Workspace::from_directory(&dir).unwrap().unwrap();
        // alpha should come before beta regardless of manifest order.
        let names: Vec<&str> = ws.member_names.iter().map(|s| s.as_str()).collect();
        let alpha_pos = names.iter().position(|n| *n == "alpha.aleo").unwrap();
        let beta_pos = names.iter().position(|n| *n == "beta.aleo").unwrap();
        assert!(alpha_pos < beta_pos, "alpha should be ordered before beta");

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn workspace_find_member() {
        let dir = temp_dir().join("ws_test_find");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        create_member(&dir, "alpha", &[]);
        create_workspace(&dir, &["alpha"]);

        let ws = Workspace::from_directory(&dir).unwrap().unwrap();
        assert!(ws.find_member("alpha").is_some());
        assert!(ws.find_member("alpha.aleo").is_some());
        assert!(ws.find_member("nonexistent").is_none());

        std::fs::remove_dir_all(&dir).unwrap();
    }

    /// Create a member whose dependencies use `Location::Workspace` (no path).
    fn create_member_with_workspace_deps(workspace_dir: &Path, name: &str, dep_names: &[&str]) {
        let member_dir = workspace_dir.join(name);
        std::fs::create_dir_all(member_dir.join("src")).unwrap();

        let program_name = format!("{name}.aleo");
        let dependencies: Vec<_> = dep_names
            .iter()
            .map(|dep_name| Dependency {
                name: format!("{dep_name}.aleo"),
                location: Location::Workspace,
                path: None,
                edition: None,
            })
            .collect();

        let manifest = Manifest {
            program: program_name,
            version: "0.1.0".to_string(),
            description: String::new(),
            license: "MIT".to_string(),
            leo: "0.0.0".to_string(),
            dependencies: if dependencies.is_empty() { None } else { Some(dependencies) },
            dev_dependencies: None,
        };

        manifest.write_to_file(member_dir.join(MANIFEST_FILENAME)).unwrap();

        std::fs::write(
            member_dir.join("src/main.leo"),
            format!("program {name}.aleo {{\n    @noupgrade\n    constructor() {{}}\n}}\n"),
        )
        .unwrap();
    }

    #[test]
    fn workspace_resolve_workspace_dep() {
        let dir = temp_dir().join("ws_test_resolve_ws_dep");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        create_member(&dir, "alpha", &[]);
        create_member_with_workspace_deps(&dir, "beta", &["alpha"]);
        create_workspace(&dir, &["alpha", "beta"]);

        let beta_dir = dir.join("beta");
        let dep =
            Dependency { name: "alpha.aleo".to_string(), location: Location::Workspace, path: None, edition: None };
        let resolved = resolve_workspace_dependency(&beta_dir, dep).unwrap();
        assert_eq!(resolved.location, Location::Local);
        assert!(resolved.path.is_some());
        assert!(resolved.path.unwrap().ends_with("alpha"));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn workspace_dependency_ordering_with_workspace_location() {
        let dir = temp_dir().join("ws_test_ordering_ws_loc");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        // alpha has no deps, beta depends on alpha via Location::Workspace.
        create_member(&dir, "alpha", &[]);
        create_member_with_workspace_deps(&dir, "beta", &["alpha"]);
        create_workspace(&dir, &["beta", "alpha"]); // intentionally wrong order

        let ws = Workspace::from_directory(&dir).unwrap().unwrap();
        // alpha should come before beta regardless of manifest order.
        let names: Vec<&str> = ws.member_names.iter().map(|s| s.as_str()).collect();
        let alpha_pos = names.iter().position(|n| *n == "alpha.aleo").unwrap();
        let beta_pos = names.iter().position(|n| *n == "beta.aleo").unwrap();
        assert!(alpha_pos < beta_pos, "alpha should be ordered before beta");

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn workspace_dep_outside_workspace_errors() {
        let dir = temp_dir().join("ws_test_dep_no_ws");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        // No workspace.json - just a standalone directory.
        let dep =
            Dependency { name: "alpha.aleo".to_string(), location: Location::Workspace, path: None, edition: None };
        let result = resolve_workspace_dependency(&dir, dep);
        assert!(result.is_err());

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn workspace_dep_member_not_found_errors() {
        let dir = temp_dir().join("ws_test_dep_not_found");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        create_member(&dir, "alpha", &[]);
        create_workspace(&dir, &["alpha"]);

        // Try to resolve a workspace dep on "nonexistent" which is not a member.
        let dep = Dependency {
            name: "nonexistent.aleo".to_string(),
            location: Location::Workspace,
            path: None,
            edition: None,
        };
        let result = resolve_workspace_dependency(&dir.join("alpha"), dep);
        assert!(result.is_err());

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn auto_register_appends_new_member() {
        let dir = temp_dir().join("ws_test_auto_register_basic");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        create_member(&dir, "alpha", &[]);
        create_workspace(&dir, &["alpha"]);

        create_member(&dir, "beta", &[]);
        let beta_dir = dir.join("beta");
        let registered = Workspace::auto_register_member(&beta_dir).unwrap();
        assert!(registered);

        let manifest = WorkspaceManifest::read_from_file(dir.join(WORKSPACE_MANIFEST_FILENAME)).unwrap();
        assert_eq!(manifest.members, vec!["alpha".to_string(), "beta".to_string()]);

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn auto_register_skips_when_glob_matches() {
        let dir = temp_dir().join("ws_test_auto_register_glob");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("packages")).unwrap();

        create_workspace(&dir, &["packages/*"]);
        create_member(&dir.join("packages"), "foo", &[]);
        let foo_dir = dir.join("packages/foo");

        let registered = Workspace::auto_register_member(&foo_dir).unwrap();
        assert!(!registered, "should skip when a glob already covers the new member");

        let manifest = WorkspaceManifest::read_from_file(dir.join(WORKSPACE_MANIFEST_FILENAME)).unwrap();
        assert_eq!(manifest.members, vec!["packages/*".to_string()]);

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn auto_register_skips_when_already_listed() {
        let dir = temp_dir().join("ws_test_auto_register_dup");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        create_member(&dir, "foo", &[]);
        create_workspace(&dir, &["foo"]);
        let foo_dir = dir.join("foo");

        let registered = Workspace::auto_register_member(&foo_dir).unwrap();
        assert!(!registered);

        let manifest = WorkspaceManifest::read_from_file(dir.join(WORKSPACE_MANIFEST_FILENAME)).unwrap();
        assert_eq!(manifest.members, vec!["foo".to_string()]);

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn auto_register_skips_outside_workspace() {
        let dir = temp_dir().join("ws_test_auto_register_outside");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        // No workspace.json - the member directory has no enclosing workspace.
        create_member(&dir, "foo", &[]);
        let foo_dir = dir.join("foo");

        let registered = Workspace::auto_register_member(&foo_dir).unwrap();
        assert!(!registered, "auto-register should be a no-op when no workspace exists");

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn auto_register_preserves_existing_order() {
        let dir = temp_dir().join("ws_test_auto_register_order");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        create_member(&dir, "alpha", &[]);
        create_member(&dir, "charlie", &[]);
        create_workspace(&dir, &["alpha", "charlie"]);

        create_member(&dir, "beta", &[]);
        let beta_dir = dir.join("beta");
        Workspace::auto_register_member(&beta_dir).unwrap();

        let manifest = WorkspaceManifest::read_from_file(dir.join(WORKSPACE_MANIFEST_FILENAME)).unwrap();
        // New entry appended at the end; existing order preserved.
        assert_eq!(manifest.members, vec!["alpha".to_string(), "charlie".to_string(), "beta".to_string()]);

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn auto_register_succeeds_despite_broken_member() {
        // A new package must register even when a sibling member listed in
        // `workspace.json` is broken: auto-registration only needs the workspace
        // root, not a fully resolved member list.
        let dir = temp_dir().join("ws_test_auto_register_broken_member");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        create_member(&dir, "alpha", &[]);
        // `ghost` is listed but never created - resolving the workspace would fail.
        create_workspace(&dir, &["alpha", "ghost"]);

        create_member(&dir, "beta", &[]);
        let beta_dir = dir.join("beta");
        let registered = Workspace::auto_register_member(&beta_dir).unwrap();
        assert!(registered, "a new package should register despite a broken sibling member");

        let manifest = WorkspaceManifest::read_from_file(dir.join(WORKSPACE_MANIFEST_FILENAME)).unwrap();
        assert_eq!(manifest.members, vec!["alpha".to_string(), "ghost".to_string(), "beta".to_string()]);

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn auto_register_registers_glob_subdir() {
        let dir = temp_dir().join("ws_test_auto_register_glob_subdir");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("packages/sub")).unwrap();

        create_workspace(&dir, &["packages/*"]);
        create_member(&dir.join("packages/sub"), "foo", &[]);
        let foo_dir = dir.join("packages/sub/foo");

        let registered = Workspace::auto_register_member(&foo_dir).unwrap();
        assert!(registered, "`packages/*` does not cover a nested package, so it should be registered");

        let manifest = WorkspaceManifest::read_from_file(dir.join(WORKSPACE_MANIFEST_FILENAME)).unwrap();
        assert_eq!(manifest.members, vec!["packages/*".to_string(), "packages/sub/foo".to_string()]);

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn auto_register_skips_when_recursive_glob_matches() {
        let dir = temp_dir().join("ws_test_auto_register_glob_recursive");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("packages/sub")).unwrap();

        create_workspace(&dir, &["packages/**"]);
        create_member(&dir.join("packages/sub"), "foo", &[]);
        let foo_dir = dir.join("packages/sub/foo");

        let registered = Workspace::auto_register_member(&foo_dir).unwrap();
        assert!(!registered, "`packages/**` crosses `/` and covers nested packages, so it should be skipped");

        let manifest = WorkspaceManifest::read_from_file(dir.join(WORKSPACE_MANIFEST_FILENAME)).unwrap();
        assert_eq!(manifest.members, vec!["packages/**".to_string()]);

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn initialize_skeleton_creates_workspace_json() {
        let dir = temp_dir().join("ws_test_init_skeleton_basic");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let full_path = Workspace::initialize_skeleton("my_workspace", &dir).unwrap();
        assert!(full_path.is_dir());
        assert_eq!(full_path.file_name().and_then(|n| n.to_str()), Some("my_workspace"));

        let manifest_path = full_path.join(WORKSPACE_MANIFEST_FILENAME);
        assert!(manifest_path.exists());
        let manifest = WorkspaceManifest::read_from_file(&manifest_path).unwrap();
        assert!(manifest.members.is_empty());

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn initialize_skeleton_rejects_existing_dir() {
        let dir = temp_dir().join("ws_test_init_skeleton_existing");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("my_workspace")).unwrap();

        let result = Workspace::initialize_skeleton("my_workspace", &dir);
        assert!(result.is_err());

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn initialize_skeleton_rejects_invalid_name() {
        let dir = temp_dir().join("ws_test_init_skeleton_invalid_name");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        // Underscore-prefixed names are rejected by `is_valid_package_name`.
        let result = Workspace::initialize_skeleton("_oops", &dir);
        assert!(result.is_err());
        // Empty names are rejected.
        let result = Workspace::initialize_skeleton("", &dir);
        assert!(result.is_err());
        // Names containing "aleo" are rejected.
        let result = Workspace::initialize_skeleton("my_aleo_ws", &dir);
        assert!(result.is_err());

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn workspace_glob_member_basic() {
        let dir = temp_dir().join("ws_test_glob_basic");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("programs")).unwrap();

        let programs = dir.join("programs");
        create_member(&programs, "alpha", &[]);
        create_member(&programs, "beta", &[]);
        create_workspace(&dir, &["programs/*"]);

        let ws = Workspace::from_directory(&dir).unwrap().unwrap();
        assert_eq!(ws.member_paths.len(), 2);
        let names: Vec<&str> = ws.member_names.iter().map(|s| s.as_str()).collect();
        assert!(names.contains(&"alpha.aleo"));
        assert!(names.contains(&"beta.aleo"));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn workspace_glob_member_recursive() {
        let dir = temp_dir().join("ws_test_glob_recursive");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("programs/sub")).unwrap();

        create_member(&dir.join("programs"), "alpha", &[]);
        create_member(&dir.join("programs/sub"), "beta", &[]);
        create_workspace(&dir, &["programs/**"]);

        let ws = Workspace::from_directory(&dir).unwrap().unwrap();
        let names: Vec<&str> = ws.member_names.iter().map(|s| s.as_str()).collect();
        assert!(names.contains(&"alpha.aleo"));
        assert!(names.contains(&"beta.aleo"));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn workspace_glob_member_no_match() {
        let dir = temp_dir().join("ws_test_glob_no_match");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        create_workspace(&dir, &["programs/*"]);

        let ws = Workspace::from_directory(&dir).unwrap().unwrap();
        assert!(ws.member_paths.is_empty());

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn workspace_glob_member_mixed() {
        let dir = temp_dir().join("ws_test_glob_mixed");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("programs")).unwrap();

        create_member(&dir, "literal_one", &[]);
        create_member(&dir.join("programs"), "globbed", &[]);
        create_workspace(&dir, &["literal_one", "programs/*"]);

        let ws = Workspace::from_directory(&dir).unwrap().unwrap();
        let names: Vec<&str> = ws.member_names.iter().map(|s| s.as_str()).collect();
        assert!(names.contains(&"literal_one.aleo"));
        assert!(names.contains(&"globbed.aleo"));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn workspace_glob_skips_non_packages() {
        let dir = temp_dir().join("ws_test_glob_skip_non_pkg");
        let _ = std::fs::remove_dir_all(&dir);
        let programs = dir.join("programs");
        std::fs::create_dir_all(programs.join("junk")).unwrap();

        create_member(&programs, "real", &[]);
        std::fs::write(programs.join("notes.txt"), "scratch").unwrap();

        create_workspace(&dir, &["programs/*"]);

        let ws = Workspace::from_directory(&dir).unwrap().unwrap();
        assert_eq!(ws.member_paths.len(), 1);
        assert_eq!(ws.member_names[0], "real.aleo");

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn workspace_glob_dep_ordering() {
        let dir = temp_dir().join("ws_test_glob_dep_order");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("programs")).unwrap();

        let programs = dir.join("programs");
        create_member(&programs, "alpha", &[]);
        create_member_with_workspace_deps(&programs, "beta", &["alpha"]);
        create_workspace(&dir, &["programs/*"]);

        let ws = Workspace::from_directory(&dir).unwrap().unwrap();
        let names: Vec<&str> = ws.member_names.iter().map(|s| s.as_str()).collect();
        let alpha_pos = names.iter().position(|n| *n == "alpha.aleo").unwrap();
        let beta_pos = names.iter().position(|n| *n == "beta.aleo").unwrap();
        assert!(alpha_pos < beta_pos, "alpha should be ordered before beta even when discovered via glob");

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn workspace_glob_dedup() {
        let dir = temp_dir().join("ws_test_glob_dedup");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("programs")).unwrap();

        create_member(&dir.join("programs"), "alpha", &[]);
        create_workspace(&dir, &["programs/alpha", "programs/*"]);

        let ws = Workspace::from_directory(&dir).unwrap().unwrap();
        assert_eq!(ws.member_paths.len(), 1, "duplicate member from literal + glob should be deduplicated");

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn workspace_glob_invalid_pattern() {
        let dir = temp_dir().join("ws_test_glob_invalid");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        create_workspace(&dir, &["[invalid"]);

        let result = Workspace::from_directory(&dir);
        assert!(result.is_err(), "malformed glob pattern should produce a structured error");

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn workspace_member_outside_root_errors() {
        let parent = temp_dir().join("ws_test_member_outside_root");
        let _ = std::fs::remove_dir_all(&parent);
        std::fs::create_dir_all(&parent).unwrap();

        // A package that lives next to the workspace, not inside it.
        create_member(&parent, "sibling", &[]);

        let ws_dir = parent.join("ws");
        std::fs::create_dir_all(&ws_dir).unwrap();
        create_workspace(&ws_dir, &["../sibling"]);

        let result = Workspace::from_directory(&ws_dir);
        assert!(result.is_err(), "a member resolving outside the workspace root should be rejected");
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("outside the workspace root"), "error should be the outside-root error: {err_msg}");

        std::fs::remove_dir_all(&parent).unwrap();
    }

    #[test]
    fn workspace_circular_workspace_deps_error() {
        let dir = temp_dir().join("ws_test_circular_ws_deps");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        // alpha depends on beta, beta depends on alpha, both via Location::Workspace.
        create_member_with_workspace_deps(&dir, "alpha", &["beta"]);
        create_member_with_workspace_deps(&dir, "beta", &["alpha"]);
        create_workspace(&dir, &["alpha", "beta"]);

        let result = Workspace::from_directory(&dir);
        assert!(result.is_err(), "circular workspace deps should be detected");
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("circular"), "error should mention circularity: {err_msg}");

        let _ = std::fs::remove_dir_all(&dir);
    }
}
