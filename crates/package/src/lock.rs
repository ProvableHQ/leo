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

//! The `leo.lock` file pins git commits and trusted network program checksums.

use leo_ast::NetworkName;
use leo_errors::Result;
use snarkvm::prelude::{Program, ProgramID, TestnetV0};

use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// File name of the lock file, stored alongside `program.json`.
pub const LOCK_FILENAME: &str = "leo.lock";

const LOCK_VERSION: u32 = 2;

/// A single pinned git dependency.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitLockEntry {
    pub name: String,
    pub git: String,
    /// The requested reference in stable string form (see `GitReference::lock_string`).
    pub reference: String,
    pub commit: String,
}

/// A program identity obtained from reviewed bytecode or an independent trusted source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkLockEntry {
    pub name: String,
    pub network: String,
    pub edition: u16,
    /// The SHA3-256 checksum of the canonical Aleo program, as 32 bytes.
    pub checksum: [u8; 32],
}

impl NetworkLockEntry {
    /// Verify bytecode before it can enter the cache or dependency graph.
    pub fn verify(&self, bytecode: &str) -> Result<()> {
        if bytecode.len() > crate::MAX_PROGRAM_SIZE {
            return Err(crate::errors::program_size_limit_exceeded(
                &self.name,
                bytecode.len(),
                crate::MAX_PROGRAM_SIZE,
            )
            .into());
        }
        let program: Program<TestnetV0> =
            bytecode.parse().map_err(|_| crate::errors::snarkvm_parsing_error(crate::bare_unit_name(&self.name)))?;
        if program.id().to_string() != self.name {
            return Err(crate::errors::untrusted_network_program(
                &self.name,
                "the bytecode declares a different program ID",
            )
            .into());
        }
        if program.to_checksum().map(|byte| *byte) != self.checksum {
            return Err(crate::errors::untrusted_network_program(
                &self.name,
                "the bytecode does not match the trusted checksum",
            )
            .into());
        }
        Ok(())
    }
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
        Self::parse(&path, &contents)
    }

    /// Read an explicitly selected trust file. Missing files are errors.
    pub fn read_file(path: &Path) -> Result<Self> {
        let contents =
            std::fs::read_to_string(path).map_err(|err| crate::errors::invalid_lock_file(path.display(), err))?;
        Self::parse(path, &contents)
    }

    fn parse(path: &Path, contents: &str) -> Result<Self> {
        let mut lock: Self =
            serde_json::from_str(contents).map_err(|err| crate::errors::invalid_lock_file(path.display(), err))?;
        if !matches!(lock.version, 1 | LOCK_VERSION) {
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

    /// Select the trusted edition. Network responses and cache contents cannot select a pin.
    pub fn network_pin(&self, name: &str, network: NetworkName, edition: Option<u16>) -> Result<&NetworkLockEntry> {
        let name = crate::canonicalize_program_name(name);
        let network = network.to_string();
        let pin = self.network.iter().find(|pin| pin.name == name && pin.network == network).ok_or_else(|| {
            crate::errors::untrusted_network_program(&name, "no trusted checksum pin for this network")
        })?;
        if edition.is_some_and(|edition| edition != pin.edition) {
            return Err(crate::errors::untrusted_network_program(
                &name,
                "the requested edition does not match the trusted pin",
            )
            .into());
        }
        Ok(pin)
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
        // Network pins are user-managed, including pins not used by this build.
        self.network.clone_from(&old.network);
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
        if self.is_empty() {
            if path.exists() {
                std::fs::remove_file(&path).map_err(|err| crate::errors::failed_to_write_lock(path.display(), err))?;
            }
            return Ok(());
        }
        self.git.sort_by(|a, b| (&a.name, &a.git, &a.reference).cmp(&(&b.name, &b.git, &b.reference)));
        self.network.sort_by(|a, b| (&a.network, &a.name).cmp(&(&b.network, &b.name)));

        let mut contents = serde_json::to_string_pretty(self)
            .map_err(|err| crate::errors::failed_to_serialize_lock(path.display(), err))?;
        contents.push('\n');
        std::fs::write(&path, contents).map_err(|err| crate::errors::failed_to_write_lock(path.display(), err))?;
        Ok(())
    }

    /// Whether the lock has no git entries or network pins.
    pub fn is_empty(&self) -> bool {
        self.git.is_empty() && self.network.is_empty()
    }
}
