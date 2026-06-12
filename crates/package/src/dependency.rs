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

use crate::Location;
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Information about a dependency, as represented in the `program.json` manifest.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Dependency {
    /// The name of the program. As this corresponds to what appears in `program.json`,
    /// it should have the ".aleo" suffix.
    pub name: String,
    pub location: Location,
    /// For a local dependency, where is its package? Or, for a test, where is its source file?
    pub path: Option<PathBuf>,
    /// For a network dependency, what is its edition?
    pub edition: Option<u16>,
    /// For a git dependency, the repository URL and the reference to track.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git: Option<GitSource>,
}

/// The `git` entry of a git dependency: a repository URL and at most one of `branch`/`tag`/`rev`.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GitSource {
    pub url: String,
    /// Git branch to track (exclusive with `tag`/`rev`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// Git tag to pin (exclusive with `branch`/`rev`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Git revision to pin (exclusive with `branch`/`tag`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rev: Option<String>,
}

impl GitSource {
    /// The [`GitReference`] this source tracks (see [`GitReference::from_opts`]).
    pub fn reference(&self) -> Result<GitReference, &'static str> {
        GitReference::from_opts(&self.branch, &self.tag, &self.rev)
    }
}

/// The git reference a dependency tracks, derived from its `branch`/`tag`/`rev` fields.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GitReference {
    /// Tip of a named branch. Mutable: re-resolved online each build.
    Branch(String),
    /// A tag. Immutable: reused from cache once fetched.
    Tag(String),
    /// A revision (commit-ish). Immutable: reused from cache once fetched.
    Rev(String),
    /// The repository's default branch. Mutable, like `Branch`.
    DefaultBranch,
}

impl GitReference {
    /// Whether the reference tracks a moving target (a branch tip) and must be re-resolved online.
    pub fn is_mutable(&self) -> bool {
        matches!(self, GitReference::Branch(_) | GitReference::DefaultBranch)
    }

    /// Stable string form stored in `leo.lock`; a change here forces re-resolution.
    pub fn lock_string(&self) -> String {
        match self {
            GitReference::Branch(b) => format!("branch={b}"),
            GitReference::Tag(t) => format!("tag={t}"),
            GitReference::Rev(r) => format!("rev={r}"),
            GitReference::DefaultBranch => "default".to_string(),
        }
    }

    /// Build a reference from `branch`/`tag`/`rev`, defaulting to `DefaultBranch`. Errors if more than
    /// one is set, or if `rev` isn't a commit hash (a symbolic revspec like `HEAD` is not a stable pin).
    pub fn from_opts(
        branch: &Option<String>,
        tag: &Option<String>,
        rev: &Option<String>,
    ) -> Result<GitReference, &'static str> {
        match (branch, tag, rev) {
            (Some(branch), None, None) => Ok(GitReference::Branch(branch.clone())),
            (None, Some(tag), None) => Ok(GitReference::Tag(tag.clone())),
            (None, None, Some(rev)) if is_commit_hash(rev) => Ok(GitReference::Rev(rev.clone())),
            (None, None, Some(_)) => {
                Err("a git `rev` must be a commit hash; use `branch` or `tag` for a named reference")
            }
            (None, None, None) => Ok(GitReference::DefaultBranch),
            _ => Err("a git dependency may specify at most one of `branch`, `tag`, or `rev`"),
        }
    }
}

/// Whether `s` looks like a commit hash (hex, 4–64 chars) rather than a symbolic revspec.
fn is_commit_hash(s: &str) -> bool {
    (4..=64).contains(&s.len()) && s.bytes().all(|b| b.is_ascii_hexdigit())
}

impl Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (on {:?})", self.name, self.location)?;
        if let Some(path) = &self.path {
            write!(f, " (at {})", path.display())?;
        }
        if let Some(edition) = self.edition {
            write!(f, " (edition {edition})")?;
        }
        if let Some(git) = &self.git {
            write!(f, " (git {})", git.url)?;
            if let Some(branch) = &git.branch {
                write!(f, " (branch {branch})")?;
            }
            if let Some(tag) = &git.tag {
                write!(f, " (tag {tag})")?;
            }
            if let Some(rev) = &git.rev {
                write!(f, " (rev {rev})")?;
            }
        }
        Ok(())
    }
}
