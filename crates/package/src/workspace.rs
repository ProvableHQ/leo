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

use crate::{Location, MANIFEST_FILENAME, Manifest, errors};

use leo_ast::DiGraph;
use leo_errors::{Backtraced, Result};

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
        let contents =
            std::fs::read_to_string(&path).map_err(|e| errors::workspace_manifest_error(path.as_ref().display(), e))?;
        serde_json::from_str(&contents).map_err(|e| errors::workspace_manifest_error(path.as_ref().display(), e))
    }

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
    /// The workspace manifest.
    pub manifest: WorkspaceManifest,
    /// Member directories in dependency order, each an absolute path.
    pub member_paths: Vec<PathBuf>,
    /// Member program names (from each member's `program.json`), in the same order.
    pub member_names: Vec<String>,
}

impl Workspace {
    /// Read the workspace at `path`, if a `workspace.json` exists there.
    ///
    /// Returns `Ok(None)` if no manifest exists.
    /// Returns `Err` if the manifest exists but is malformed, or if members are missing.
    pub fn from_directory(path: &Path) -> Result<Option<Self>> {
        let manifest_path = path.join(WORKSPACE_MANIFEST_FILENAME);
        if !manifest_path.exists() {
            return Ok(None);
        }

        let root_directory =
            path.canonicalize().map_err(|e| errors::workspace_manifest_error(manifest_path.display(), e))?;

        let manifest = WorkspaceManifest::read_from_file(&manifest_path)?;

        // Resolve and validate each member.
        let mut dir_to_name: Vec<(PathBuf, String)> = Vec::with_capacity(manifest.members.len());
        for member in &manifest.members {
            let member_dir = root_directory.join(member);
            if !member_dir.is_dir() {
                return Err(errors::workspace_member_not_found(member, root_directory.display()).into());
            }
            let member_manifest_path = member_dir.join(MANIFEST_FILENAME);
            if !member_manifest_path.exists() {
                return Err(errors::workspace_member_not_found(member, root_directory.display()).into());
            }
            let member_manifest = Manifest::read_from_file(&member_manifest_path)?;
            let canonical =
                member_dir.canonicalize().map_err(|e| errors::workspace_manifest_error(member_dir.display(), e))?;
            dir_to_name.push((canonical, member_manifest.program.clone()));
        }

        // Build a dependency graph to determine the correct build order.
        let ordered = order_members(&dir_to_name)?;

        let member_paths = ordered.iter().map(|(p, _)| p.clone()).collect();
        let member_names = ordered.into_iter().map(|(_, n)| n).collect();

        Ok(Some(Workspace { root_directory, manifest, member_paths, member_names }))
    }

    /// Walk up from `start_dir` looking for `workspace.json`.
    ///
    /// Returns `Ok(None)` if no workspace root is found.
    pub fn discover(start_dir: &Path) -> Result<Option<Self>> {
        let start = start_dir.canonicalize().map_err(|e| errors::workspace_manifest_error(start_dir.display(), e))?;
        let mut dir = start.as_path();
        loop {
            if let Some(ws) = Self::from_directory(dir)? {
                return Ok(Some(ws));
            }
            match dir.parent() {
                Some(parent) => dir = parent,
                None => return Ok(None),
            }
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
    pub fn is_member(&self, path: &Path) -> bool {
        let Ok(canonical) = path.canonicalize() else {
            return false;
        };
        self.member_paths.iter().any(|p| p == &canonical)
    }
}

/// Determine the build order for workspace members by analysing cross-member
/// local dependencies.
///
/// Each member's `Manifest` is read to find `Location::Local` dependencies
/// whose paths resolve to other workspace member directories. Edges are added
/// to a `DiGraph` from dependent to dependency, and the graph is topologically
/// sorted so that dependencies appear before the members that depend on them.
fn order_members(members: &[(PathBuf, String)]) -> Result<Vec<(PathBuf, String)>> {
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

    // Scan each member's manifest for local dependencies pointing to other members.
    for (member_path, _) in members {
        let member_dir_name = member_path.file_name().and_then(|n| n.to_str()).unwrap_or_default();
        let manifest_path = member_path.join(MANIFEST_FILENAME);
        let manifest = Manifest::read_from_file(&manifest_path)?;

        for dep in manifest.dependencies.iter().flatten() {
            if dep.location != Location::Local {
                continue;
            }
            let Some(dep_path) = &dep.path else {
                continue;
            };
            // Resolve relative paths against the member directory.
            let resolved = if dep_path.is_absolute() { dep_path.clone() } else { member_path.join(dep_path) };
            let Ok(canonical) = resolved.canonicalize() else {
                continue;
            };
            // If this dependency points to another workspace member, add an edge.
            if let Some(&dep_dir_name) = path_to_dir_name.get(canonical.as_path()) {
                graph.add_edge(member_dir_name.to_string(), dep_dir_name.to_string());
            }
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
}
