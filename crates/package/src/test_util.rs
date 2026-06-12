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

//! Shared helpers for git-dependency tests across the crate.

use crate::{MANIFEST_FILENAME, Manifest};
use leo_errors::Backtraced;

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

/// Whether the `git` CLI is available; tests that build fixture repos skip when it is not.
pub(crate) fn git_available() -> bool {
    Command::new("git").arg("--version").output().map(|o| o.status.success()).unwrap_or(false)
}

/// Run a git command in `dir`, returning trimmed stdout; panics on failure.
pub(crate) fn run_git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git").args(args).current_dir(dir).output().expect("failed to spawn git");
    assert!(out.status.success(), "git {args:?} failed: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// A fresh, unique scratch directory tagged for the calling test.
pub(crate) fn unique_dir(tag: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("leo-git-test-{}-{tag}-{seq}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Write `contents` to `path`, creating parent directories as needed.
pub(crate) fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, contents).unwrap();
}

/// Initialise `dir` as a git repository with a single commit (and an optional tag).
pub(crate) fn init_repo(dir: &Path, tag: Option<&str>) {
    run_git(dir, &["init", "-q", "-b", "main"]);
    run_git(dir, &["config", "user.email", "t@example.com"]);
    run_git(dir, &["config", "user.name", "Test"]);
    run_git(dir, &["add", "."]);
    run_git(dir, &["commit", "-q", "-m", "init"]);
    if let Some(tag) = tag {
        run_git(dir, &["tag", tag]);
    }
}

/// A `file://` URL for `path` that is valid on Windows too: forward slashes, plus the leading slash
/// a file URL needs before a drive letter (`file:///C:/...`). Crucially, it embeds in JSON without
/// the raw backslashes a Windows path would otherwise contain.
pub(crate) fn file_url(path: &Path) -> String {
    let s = path.display().to_string().replace('\\', "/");
    if s.starts_with('/') { format!("file://{s}") } else { format!("file:///{s}") }
}

/// Build a fixture repo with a tagged first commit on `main`, a `feature` branch, and a
/// second commit on `main`. Returns `(file_url, first_commit, second_commit)`.
pub(crate) fn fixture_repo(base: &Path) -> (String, String, String) {
    let src = base.join("src");
    std::fs::create_dir_all(&src).unwrap();
    run_git(&src, &["init", "-q", "-b", "main"]);
    run_git(&src, &["config", "user.email", "t@example.com"]);
    run_git(&src, &["config", "user.name", "Test"]);
    std::fs::write(src.join("a.txt"), "one").unwrap();
    run_git(&src, &["add", "."]);
    run_git(&src, &["commit", "-q", "-m", "c1"]);
    run_git(&src, &["tag", "v1"]);
    let c1 = run_git(&src, &["rev-parse", "HEAD"]);
    run_git(&src, &["checkout", "-q", "-b", "feature"]);
    std::fs::write(src.join("b.txt"), "feat").unwrap();
    run_git(&src, &["add", "."]);
    run_git(&src, &["commit", "-qm", "feat"]);
    run_git(&src, &["checkout", "-q", "main"]);
    std::fs::write(src.join("a.txt"), "two").unwrap();
    run_git(&src, &["commit", "-qam", "c2"]);
    let c2 = run_git(&src, &["rev-parse", "HEAD"]);
    (file_url(&src), c1, c2)
}

/// Write a library package (`program.json` + `src/lib.leo`) at `dir`.
pub(crate) fn write_library(dir: &Path, name: &str, deps_json: &str) {
    write_file(
        &dir.join(MANIFEST_FILENAME),
        &format!(
            r#"{{"program":"{name}","version":"0.1.0","description":"","license":"MIT","dependencies":{deps_json}}}"#
        ),
    );
    write_file(&dir.join("src/lib.leo"), "// lib\n");
}

/// Write a program package (`program.json` + `src/main.leo`) at `dir`. `name` carries `.aleo`.
pub(crate) fn write_program(dir: &Path, name: &str, deps_json: &str) {
    write_file(
        &dir.join(MANIFEST_FILENAME),
        &format!(
            r#"{{"program":"{name}","version":"0.1.0","description":"","license":"MIT","dependencies":{deps_json}}}"#
        ),
    );
    write_file(&dir.join("src/main.leo"), "// main\n");
}

/// Write a consumer package at `dir` whose `program.json` lists `deps` (a JSON array body).
pub(crate) fn write_consumer(dir: &Path, deps: &str) {
    write_file(
        &dir.join(MANIFEST_FILENAME),
        &format!(
            r#"{{"program":"consumer.aleo","version":"0.1.0","description":"","license":"MIT","dependencies":[{deps}]}}"#
        ),
    );
    write_file(&dir.join("src/main.leo"), "// main\n");
}

/// Build a `program.json` body with the given `dependencies` and `dev_dependencies` JSON arrays.
pub(crate) fn manifest_json(dependencies: &str, dev_dependencies: &str) -> String {
    format!(
        r#"{{
  "program": "test.aleo",
  "version": "0.1.0",
  "description": "",
  "license": "MIT",
  "dependencies": {dependencies},
  "dev_dependencies": {dev_dependencies}
}}"#
    )
}

/// Parse a `program.json` from `contents` via a throwaway temporary directory.
pub(crate) fn read_manifest(contents: &str) -> Result<Manifest, Backtraced> {
    // Combine the process id with a timestamp and a per-process counter so concurrent tests
    // never collide on the directory name (the clock alone is too coarse under parallelism).
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let unique = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("leo-manifest-test-{}-{unique}-{seq}", std::process::id()));
    fs::create_dir(&dir).unwrap();
    let path = dir.join(MANIFEST_FILENAME);
    fs::write(&path, contents).unwrap();
    let result = Manifest::read_from_file(&path);
    fs::remove_dir_all(dir).unwrap();
    result
}
