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

use aleo_std;
use leo_errors::Result;
use leo_package::{Manifest, Workspace};

use aleo_std::aleo_dir;
use std::{env::current_dir, path::PathBuf};

/// Project context, manifest, current directory etc
/// All the info that is relevant in most of the commands
// TODO: Make `path` and `home` not pub, to prevent misuse through direct access.
#[derive(Clone)]
pub struct Context {
    /// Path at which the command is called, None when default
    pub path: Option<PathBuf>,
    /// Path to use for the Aleo registry, None when default
    pub home: Option<PathBuf>,
    /// Recursive flag.
    // TODO: Shift from callee to caller by including display method
    pub recursive: bool,
    /// If set, target this specific workspace member.
    pub package_filter: Option<String>,
}

impl Context {
    pub fn new(
        path: Option<PathBuf>,
        home: Option<PathBuf>,
        recursive: bool,
        package_filter: Option<String>,
    ) -> Result<Context> {
        Ok(Context { path, home, recursive, package_filter })
    }

    /// Returns the path of the parent directory to the Leo package.
    pub fn parent_dir(&self) -> Result<PathBuf> {
        match &self.path {
            Some(path) => {
                let mut path = path.clone();
                path.pop();
                Ok(path)
            }
            None => Ok(current_dir().map_err(crate::errors::cli_io_error)?),
        }
    }

    /// Returns the path to the Leo package.
    pub fn dir(&self) -> Result<PathBuf> {
        match &self.path {
            Some(path) => Ok(path.clone()),
            None => Ok(current_dir().map_err(crate::errors::cli_io_error)?),
        }
    }

    /// Returns the path to the Aleo registry directory.
    pub fn home(&self) -> Result<PathBuf> {
        match &self.home {
            Some(path) => Ok(path.clone()),
            None => Ok(aleo_dir()),
        }
    }

    /// Opens the manifest file `program.json`.
    pub fn open_manifest(&self) -> Result<Manifest> {
        let path = self.dir()?;
        let manifest_path = path.join(leo_package::MANIFEST_FILENAME);
        let manifest = Manifest::read_from_file(manifest_path)?;
        Ok(manifest)
    }

    /// Returns the workspace root and the ordered list of member directories to
    /// operate on, respecting `--package` filtering and workspace discovery.
    ///
    /// - At workspace root without `--package`: all members in dependency order.
    /// - At workspace root with `--package`: just that member.
    /// - Inside a member directory: just that member.
    /// - No workspace found: `None` (caller falls through to single-package behavior).
    ///
    /// When `Some`, the first tuple element is the canonicalized workspace root,
    /// so callers needing it (e.g. `leo clean` removing the shared `build/`) do
    /// not have to re-walk for it.
    pub fn resolve_targets(&self) -> Result<Option<(PathBuf, Vec<PathBuf>)>> {
        let dir = self.dir()?;

        let workspace = match Workspace::discover(&dir)? {
            Some(ws) => ws,
            None => {
                if self.package_filter.is_some() {
                    return Err(crate::errors::workspace_no_workspace().into());
                }
                return Ok(None);
            }
        };

        let root = workspace.root_directory.clone();

        if let Some(ref filter) = self.package_filter {
            match workspace.find_member(filter) {
                Some(path) => Ok(Some((root, vec![path.clone()]))),
                None => Err(crate::errors::workspace_package_not_found(filter, root.display()).into()),
            }
        } else {
            let canonical = dir.canonicalize().unwrap_or_else(|_| dir.clone());
            if canonical == workspace.root_directory {
                // At workspace root - operate on all members.
                Ok(Some((root, workspace.member_paths)))
            } else if workspace.is_member(&canonical) {
                // Inside a member - operate on just this member.
                Ok(Some((root, vec![canonical])))
            } else {
                // Inside the workspace tree but not in a member directory.
                Ok(None)
            }
        }
    }

    /// Create a new `Context` pointing at a specific directory.
    pub fn with_path(&self, path: PathBuf) -> Self {
        Context { path: Some(path), home: self.home.clone(), recursive: self.recursive, package_filter: None }
    }
}
