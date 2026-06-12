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

//! Fetching and checking out git dependencies.
//!
//! A git dependency resolves to an exact commit, checked out into a content-addressed cache
//! (`<home>/git/checkouts/<repo>-<urlhash>/<commit>/`) and then treated as a local package.

use crate::GitReference;

use snarkvm::algorithms::crypto_hash::sha256;

use leo_errors::Result;

use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

const CHECKOUTS_SUBDIR: &str = "git/checkouts";
const TMP_SUBDIR: &str = "git/tmp";

/// The root of the git checkout cache under the Leo home directory.
pub(crate) fn checkouts_root(home: &Path) -> PathBuf {
    home.join(CHECKOUTS_SUBDIR)
}

/// The directory a given commit is checked out into, keyed by URL hash and commit so all
/// dependencies into the same repository share one checkout.
pub(crate) fn checkout_dir(home: &Path, url: &str, commit: &str) -> PathBuf {
    let digest = sha256(url.as_bytes());
    let url_hash: String = digest.iter().take(8).map(|b| format!("{b:02x}")).collect();
    // A human-readable prefix from the URL's last path segment; uniqueness comes from the hash.
    let repo: String = url
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or(url)
        .trim_end_matches(".git")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-'))
        .take(32)
        .collect();
    let repo = if repo.is_empty() { "repo".to_string() } else { repo };
    checkouts_root(home).join(format!("{repo}-{url_hash}")).join(commit)
}

/// Resolve a git dependency to a `(checkout_directory, commit_hash)`.
///
/// A `locked_commit` from `leo.lock` is reused when its checkout exists and the reference is
/// immutable (tag/rev) or `offline`; a mutable reference re-resolves against the remote when online.
/// `offline` forbids network access, so it then succeeds only from an existing checkout.
pub fn resolve(
    home: &Path,
    name: &str,
    url: &str,
    reference: &GitReference,
    locked_commit: Option<&str>,
    offline: bool,
) -> Result<(PathBuf, String)> {
    // Fast path: reuse a cached locked commit, but only for an immutable reference (or when
    // offline) — a mutable branch tip must be re-resolved online.
    if let Some(commit) = locked_commit
        && (offline || !reference.is_mutable())
    {
        let dir = checkout_dir(home, url, commit);
        if dir.is_dir() {
            return Ok((dir, commit.to_string()));
        }
    }

    if offline {
        return Err(crate::errors::git_offline_unavailable(name, url).into());
    }

    // Clean up the transient clone whether resolution succeeds or fails (an interrupted fetch
    // returns `Err`, so this still runs).
    let tmp = unique_dir(&home.join(TMP_SUBDIR), "clone");
    let result = clone_resolve_checkout(home, name, url, reference, locked_commit, &tmp);
    let _ = std::fs::remove_dir_all(&tmp);
    result
}

fn clone_resolve_checkout(
    home: &Path,
    name: &str,
    url: &str,
    reference: &GitReference,
    locked_commit: Option<&str>,
    tmp: &Path,
) -> Result<(PathBuf, String)> {
    let _ = std::fs::remove_dir_all(tmp);
    if let Some(parent) = tmp.parent() {
        std::fs::create_dir_all(parent).map_err(|e| crate::errors::git_error(name, url, e))?;
    }

    // A no-ref clone fetches all branches and tags, so any reference resolves against it.
    let parsed = gix::url::parse(url.into()).map_err(|e| crate::errors::git_error(name, url, e))?;
    let mut prepare = gix::prepare_clone(parsed, tmp)
        .map_err(|e| crate::errors::git_error(name, url, e))?
        .configure_connection(|connection| {
            // Public repos only: never supply credentials, so a private repo fails rather than
            // resolving via the developer's local git credentials (not reproducible elsewhere).
            #[allow(clippy::result_large_err)] // gix's error type is large and not boxable here.
            connection.set_credentials(|_action| Ok(None));
            Ok(())
        });
    let (checkout, _) = prepare
        .fetch_then_checkout(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
        .map_err(|e| crate::errors::git_error(name, url, e))?;
    // Keep the fetched objects but skip the default worktree; we check out an exact tree below.
    let repo = checkout.persist();

    let revspec = match (locked_commit, reference) {
        // Immutable ref + locked commit: use the pin so a moved tag can't change the build.
        (Some(commit), reference) if !reference.is_mutable() => commit.to_string(),
        (_, GitReference::DefaultBranch) => "HEAD".to_string(),
        (_, GitReference::Branch(branch)) => format!("origin/{branch}"),
        (_, GitReference::Tag(tag)) => format!("refs/tags/{tag}"),
        (_, GitReference::Rev(rev)) => rev.clone(),
    };
    let id = repo
        .rev_parse_single(revspec.as_str())
        .map_err(|e| crate::errors::git_reference_error(name, url, &revspec, e))?;
    let commit = id.detach().to_string();

    let dir = checkout_dir(home, url, &commit);
    if !dir.is_dir() {
        // Atomic rename-in so a concurrent/interrupted resolution never sees a partial checkout.
        let staging = staging_dir(&dir);
        let renamed =
            checkout_tree(&repo, id, &staging).and_then(|()| std::fs::rename(&staging, &dir).map_err(Into::into));
        let _ = std::fs::remove_dir_all(&staging);
        // A rename failure is fine when another resolution won the race and the dir now exists.
        if let Err(e) = renamed
            && !dir.is_dir()
        {
            return Err(crate::errors::git_error(name, url, e).into());
        }
    }

    Ok((dir, commit))
}

/// Materialise the tree of `id` into `dir` (a staging directory), which is (re)created empty.
fn checkout_tree(
    repo: &gix::Repository,
    id: gix::Id<'_>,
    dir: &Path,
) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let tree = id.object()?.peel_to_tree()?;
    let mut index = repo.index_from_tree(&tree.id())?;
    let mut opts = repo.checkout_options(gix::worktree::stack::state::attributes::Source::IdMapping)?;
    opts.destination_is_initially_empty = true;

    // Start clean so no files from a partial previous attempt linger.
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir)?;

    gix::worktree::state::checkout(
        &mut index,
        dir,
        repo.objects.clone().into_arc()?,
        &gix::progress::Discard,
        &gix::progress::Discard,
        &gix::interrupt::IS_INTERRUPTED,
        opts,
    )?;
    Ok(())
}

/// A unique directory under `parent` for transient work (`<prefix>-<pid>-<seq>`).
fn unique_dir(parent: &Path, prefix: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
    parent.join(format!("{prefix}-{}-{seq}", std::process::id()))
}

/// A unique staging directory next to `final_dir` (same filesystem, so the rename is atomic).
fn staging_dir(final_dir: &Path) -> PathBuf {
    // The leading `.` keeps a lingering staging dir out of the by-name package search.
    unique_dir(final_dir.parent().unwrap_or(final_dir), ".staging")
}
