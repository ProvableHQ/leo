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

//! The `leo.lock` file pins Git commits and network program checksums.

use leo_ast::NetworkName;
use leo_errors::Result;
use snarkvm::prelude::{ProgramID, TestnetV0};

use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::{io::Write, path::Path};

/// File name of the lock file, stored alongside `program.json`.
pub const LOCK_FILENAME: &str = "leo.lock";

const LOCK_VERSION: u32 = 3;

/// A single pinned git dependency.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitLockEntry {
    pub name: String,
    pub git: String,
    /// The requested reference in stable string form (see `GitReference::lock_string`).
    pub reference: String,
    pub commit: String,
}

/// A program identity and checksum recorded when its edition was first fetched.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkLockEntry {
    pub name: String,
    pub network: String,
    pub edition: u16,
    /// The SHA3-256 checksum of the canonical Aleo program, as 32 bytes.
    pub checksum: [u8; 32],
    /// The SHA-256 fingerprint of the trusted endpoint that supplied this edition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<[u8; 32]>,
}

/// The contents of `leo.lock`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lock {
    version: u32,
    #[serde(default)]
    git: Vec<GitLockEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    network: Vec<NetworkLockEntry>,
}

impl Default for Lock {
    fn default() -> Self {
        Lock { version: LOCK_VERSION, git: Vec::new(), network: Vec::new() }
    }
}

impl Lock {
    /// Read the lock from `dir`, or an empty lock if it is missing.
    pub fn read(dir: &Path) -> Result<Self> {
        let path = dir.join(LOCK_FILENAME);
        let contents = match std::fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(err) => return Err(crate::errors::invalid_lock_file(path.display(), err).into()),
        };
        let mut lock: Self =
            serde_json::from_str(&contents).map_err(|err| crate::errors::invalid_lock_file(path.display(), err))?;
        if !matches!(lock.version, 1 | 2 | LOCK_VERSION) {
            return Err(crate::errors::invalid_lock_file(path.display(), "unsupported lock version").into());
        }
        let mut seen = IndexSet::with_capacity(lock.network.len());
        for pin in &lock.network {
            if pin.name.parse::<ProgramID<TestnetV0>>().is_err()
                || pin.network.parse::<NetworkName>().is_err()
                || (pin.name == "credits.aleo" && pin.edition != 0)
            {
                return Err(crate::errors::invalid_lock_file(path.display(), "invalid network program identity").into());
            }
            if !seen.insert((&pin.network, &pin.name)) {
                return Err(crate::errors::invalid_lock_file(path.display(), "duplicate network program pin").into());
            }
        }
        lock.version = LOCK_VERSION;
        Ok(lock)
    }

    /// Find a network pin that matches the requested program and optional edition.
    pub fn network_pin(&self, name: &str, network: NetworkName, edition: Option<u16>) -> Option<&NetworkLockEntry> {
        let name = crate::canonicalize_program_name(name);
        let network = network.to_string();
        self.network.iter().find(|pin| {
            pin.name == name && pin.network == network && edition.is_none_or(|edition| edition == pin.edition)
        })
    }

    /// Record a network edition, preserving entries for other programs and networks.
    pub fn record_network(&mut self, entry: NetworkLockEntry) {
        self.network.retain(|pin| pin.name != entry.name || pin.network != entry.network);
        self.network.push(entry);
    }

    /// Read the recorded Git dependencies.
    pub fn git_entries(&self) -> &[GitLockEntry] {
        &self.git
    }

    /// Read the recorded network dependencies.
    pub fn network_entries(&self) -> &[NetworkLockEntry] {
        &self.network
    }

    /// Release selected branch pins, including other dependencies from the same source.
    pub(crate) fn unlock_git(&mut self, name: Option<&str>) {
        let sources: IndexSet<_> = self
            .git
            .iter()
            .filter(|entry| {
                (entry.reference == "default" || entry.reference.starts_with("branch="))
                    && name.is_none_or(|name| crate::bare_unit_name(&entry.name) == crate::bare_unit_name(name))
            })
            .map(|entry| (entry.git.clone(), entry.reference.clone()))
            .collect();
        self.git
            .retain(|entry| !sources.iter().any(|(git, reference)| &entry.git == git && &entry.reference == reference));
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
        for entry in &old.network {
            if !self.network.iter().any(|pin| pin.name == entry.name && pin.network == entry.network) {
                self.network.push(entry.clone());
            }
        }
        for entry in &old.git {
            if self.commit_for(&entry.name, &entry.git, &entry.reference).is_none() && keep(entry) {
                self.git.push(entry.clone());
            }
        }
    }

    /// Remove Git entries for the dependency `name`, preserving network pins.
    pub fn remove_name(&mut self, name: &str) {
        self.git.retain(|e| e.name != name);
    }

    /// Write the lock to `dir`, with entries sorted for deterministic output.
    pub fn write(&mut self, dir: &Path) -> Result<()> {
        let path = dir.join(LOCK_FILENAME);
        let permissions = match std::fs::symlink_metadata(&path) {
            Ok(metadata) if metadata.is_file() => Some(metadata.permissions()),
            Ok(_) => {
                return Err(crate::errors::failed_to_write_lock(
                    path.display(),
                    "expected a regular file, not a symlink",
                )
                .into());
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
            Err(err) => return Err(crate::errors::failed_to_write_lock(path.display(), err).into()),
        };
        if self.is_empty() {
            if permissions.is_some() {
                std::fs::remove_file(&path).map_err(|err| crate::errors::failed_to_write_lock(path.display(), err))?;
            }
            return Ok(());
        }
        self.git.sort_by(|a, b| (&a.name, &a.git, &a.reference).cmp(&(&b.name, &b.git, &b.reference)));
        self.network.sort_by(|a, b| (&a.network, &a.name).cmp(&(&b.network, &b.name)));

        let mut contents = serde_json::to_string_pretty(self)
            .map_err(|err| crate::errors::failed_to_serialize_lock(path.display(), err))?;
        contents.push('\n');
        // Replace the complete lock only after its temporary file is written.
        let temporary = crate::git::unique_dir(dir, ".leo.lock");
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|err| crate::errors::failed_to_write_lock(path.display(), err))?;
        let result = (|| {
            if let Some(permissions) = permissions {
                file.set_permissions(permissions)?;
            }
            file.write_all(contents.as_bytes())?;
            drop(file);
            std::fs::rename(&temporary, &path)
        })();
        if let Err(err) = result {
            let _ = std::fs::remove_file(&temporary);
            return Err(crate::errors::failed_to_write_lock(path.display(), err).into());
        }
        Ok(())
    }

    /// Whether the lock has no git entries or network pins.
    pub fn is_empty(&self) -> bool {
        self.git.is_empty() && self.network.is_empty()
    }
}
