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

//! The `leo.lock` lock file, which pins git dependencies to exact commits.
//!
//! Only git dependencies need locking (network deps are pinned by edition, path deps by location).
//! Entries are keyed by `(name, git, reference)`, so changing the requested reference re-resolves.

use leo_errors::Result;

use serde::{Deserialize, Serialize};
use std::path::Path;

/// File name of the lock file, stored alongside `program.json`.
pub const LOCK_FILENAME: &str = "leo.lock";

const LOCK_VERSION: u32 = 1;

/// A single pinned git dependency.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitLockEntry {
    pub name: String,
    pub git: String,
    /// The requested reference in stable string form (see `GitReference::lock_string`).
    pub reference: String,
    pub commit: String,
}

/// The contents of `leo.lock`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lock {
    version: u32,
    #[serde(default)]
    git: Vec<GitLockEntry>,
}

impl Default for Lock {
    fn default() -> Self {
        Lock { version: LOCK_VERSION, git: Vec::new() }
    }
}

impl Lock {
    /// Read the lock from `dir`, or an empty lock if it is missing. A malformed or unsupported-version
    /// lock is regenerated rather than erroring, but warns (unlike a missing file) so lost pins are visible.
    pub fn read(dir: &Path) -> Self {
        let path = dir.join(LOCK_FILENAME);
        let Ok(contents) = std::fs::read_to_string(&path) else {
            return Lock::default();
        };
        match serde_json::from_str::<Lock>(&contents) {
            Ok(lock) if lock.version == LOCK_VERSION => lock,
            Ok(lock) => {
                tracing::warn!(
                    "⚠️ Ignoring `{}`: unsupported lock version {} (expected {LOCK_VERSION}). It will be regenerated.",
                    path.display(),
                    lock.version,
                );
                Lock::default()
            }
            Err(err) => {
                tracing::warn!("⚠️ Ignoring malformed `{}` ({err}). It will be regenerated.", path.display());
                Lock::default()
            }
        }
    }

    /// The pinned commit for `(name, git, reference)`, or `None` (forcing re-resolution) on any mismatch.
    pub fn commit_for(&self, name: &str, git: &str, reference: &str) -> Option<&str> {
        self.git.iter().find(|e| e.name == name && e.git == git && e.reference == reference).map(|e| e.commit.as_str())
    }

    /// The commit any dependency resolved `(git, reference)` to, regardless of name. Lets a build
    /// reuse a resolution it already performed for another dependency on the same repository.
    pub fn commit_for_source(&self, git: &str, reference: &str) -> Option<&str> {
        self.git.iter().find(|e| e.git == git && e.reference == reference).map(|e| e.commit.as_str())
    }

    /// Record a pinned commit, replacing any existing entry for the same `(name, git, reference)`.
    /// Entries under other references are kept: in a shared workspace lock they may belong to
    /// another member; stale ones are pruned by `carry_over` or `leo remove`.
    pub fn record(&mut self, name: String, git: String, reference: String, commit: String) {
        self.git.retain(|e| !(e.name == name && e.git == git && e.reference == reference));
        self.git.push(GitLockEntry { name, git, reference, commit });
    }

    /// Carry over entries from `old` that were not re-recorded in this lock and that `keep` accepts.
    pub fn carry_over(&mut self, old: &Lock, mut keep: impl FnMut(&GitLockEntry) -> bool) {
        for entry in &old.git {
            if self.commit_for(&entry.name, &entry.git, &entry.reference).is_none() && keep(entry) {
                self.git.push(entry.clone());
            }
        }
    }

    /// Remove all entries pinning the dependency `name`.
    pub fn remove_name(&mut self, name: &str) {
        self.git.retain(|e| e.name != name);
    }

    /// Write the lock to `dir`, entries sorted for determinism. With no git dependencies, no file is
    /// written and a stale one is removed.
    pub fn write(&mut self, dir: &Path) -> Result<()> {
        let path = dir.join(LOCK_FILENAME);
        if self.is_empty() {
            if path.exists() {
                std::fs::remove_file(&path).map_err(|err| crate::errors::failed_to_write_lock(path.display(), err))?;
            }
            return Ok(());
        }
        self.git.sort_by(|a, b| (&a.name, &a.git, &a.reference).cmp(&(&b.name, &b.git, &b.reference)));

        let mut contents = serde_json::to_string_pretty(self)
            .map_err(|err| crate::errors::failed_to_serialize_lock(path.display(), err))?;
        contents.push('\n');
        std::fs::write(&path, contents).map_err(|err| crate::errors::failed_to_write_lock(path.display(), err))?;
        Ok(())
    }

    /// Whether any git dependency is recorded.
    pub fn is_empty(&self) -> bool {
        self.git.is_empty()
    }
}
