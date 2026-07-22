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

//! Tests for git-dependency support: reference resolution, the `leo.lock` lock file,
//! manifest validation, end-to-end resolution, and workspace lock sharing.

use crate::{
    GitReference,
    LOCK_FILENAME,
    Lock,
    MANIFEST_FILENAME,
    Package,
    WORKSPACE_MANIFEST_FILENAME,
    git::resolve,
    test_util::{
        file_url,
        fixture_repo,
        git_available,
        init_repo,
        manifest_json,
        read_manifest,
        run_git,
        unique_dir,
        write_consumer,
        write_file,
        write_library,
        write_program,
    },
};

use leo_span::Symbol;

// Reference resolution (`crate::git::resolve`).

#[test]
fn resolves_default_branch_tag_and_rev() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let base = unique_dir("resolve");
    let home = base.join("home");
    let (url, c1, c2) = fixture_repo(&base);

    // Default branch -> latest commit on main.
    let (dir, commit) = resolve(&home, "dep", &url, &GitReference::DefaultBranch, None, false).unwrap();
    assert_eq!(commit, c2);
    assert_eq!(std::fs::read_to_string(dir.join("a.txt")).unwrap(), "two");

    // Tag -> the tagged (first) commit.
    let (dir, commit) = resolve(&home, "dep", &url, &GitReference::Tag("v1".into()), None, false).unwrap();
    assert_eq!(commit, c1);
    assert_eq!(std::fs::read_to_string(dir.join("a.txt")).unwrap(), "one");

    // Branch -> the feature branch's content.
    let (dir, _) = resolve(&home, "dep", &url, &GitReference::Branch("feature".into()), None, false).unwrap();
    assert_eq!(std::fs::read_to_string(dir.join("b.txt")).unwrap(), "feat");

    // Rev -> the exact commit.
    let (dir, commit) = resolve(&home, "dep", &url, &GitReference::Rev(c1.clone()), None, false).unwrap();
    assert_eq!(commit, c1);
    assert_eq!(std::fs::read_to_string(dir.join("a.txt")).unwrap(), "one");

    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn locked_commit_is_reused_without_network() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let base = unique_dir("locked");
    let home = base.join("home");
    let (url, c1, _c2) = fixture_repo(&base);

    // Populate the cache for the tagged commit.
    let (_, commit) = resolve(&home, "dep", &url, &GitReference::Tag("v1".into()), None, false).unwrap();
    assert_eq!(commit, c1);

    // The bogus URL hashes to a different checkout dir, so the locked commit isn't reused: this
    // must fail, confirming the lock fast-path is keyed on the URL too.
    let bogus = "file:///nonexistent/repo";
    let bogus_locked = resolve(&home, "dep", bogus, &GitReference::Tag("v1".into()), Some(&c1), false);
    assert!(bogus_locked.is_err());

    // The real URL with the locked commit reuses the checkout (offline succeeds).
    let (dir, commit) = resolve(&home, "dep", &url, &GitReference::Tag("v1".into()), Some(&c1), true).unwrap();
    assert_eq!(commit, c1);
    assert_eq!(std::fs::read_to_string(dir.join("a.txt")).unwrap(), "one");

    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn mutable_reference_re_resolves_online_but_reuses_locked_offline() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let base = unique_dir("mutable");
    let home = base.join("home");
    let (url, _c1, c2) = fixture_repo(&base);
    let src = base.join("src");

    // Resolve the default branch; it pins to the current tip.
    let (_, first) = resolve(&home, "dep", &url, &GitReference::DefaultBranch, None, false).unwrap();
    assert_eq!(first, c2);

    // Advance the branch with a new commit.
    std::fs::write(src.join("a.txt"), "three").unwrap();
    run_git(&src, &["commit", "-qam", "c3"]);
    let c3 = run_git(&src, &["rev-parse", "HEAD"]);
    assert_ne!(c3, c2);

    // Online, the locked commit is ignored for a mutable reference: it re-resolves to the tip.
    let (dir, refreshed) = resolve(&home, "dep", &url, &GitReference::DefaultBranch, Some(&c2), false).unwrap();
    assert_eq!(refreshed, c3);
    assert_eq!(std::fs::read_to_string(dir.join("a.txt")).unwrap(), "three");

    // Offline, the locked commit is reused even for a mutable reference (no network access).
    let (dir, offline) = resolve(&home, "dep", &url, &GitReference::DefaultBranch, Some(&c2), true).unwrap();
    assert_eq!(offline, c2);
    assert_eq!(std::fs::read_to_string(dir.join("a.txt")).unwrap(), "two");

    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn branch_reference_re_resolves_to_new_tip_online() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let base = unique_dir("branch");
    let home = base.join("home");
    let (url, _c1, _c2) = fixture_repo(&base);
    let src = base.join("src");

    // Pin the feature branch's current tip.
    let (_, first) = resolve(&home, "dep", &url, &GitReference::Branch("feature".into()), None, false).unwrap();

    // Advance the feature branch with a new commit.
    run_git(&src, &["checkout", "-q", "feature"]);
    std::fs::write(src.join("b.txt"), "feat2").unwrap();
    run_git(&src, &["commit", "-qam", "feat2"]);
    let advanced = run_git(&src, &["rev-parse", "HEAD"]);
    assert_ne!(advanced, first);

    // Online, a mutable branch reference ignores the lock and re-resolves to the new tip.
    let (dir, refreshed) =
        resolve(&home, "dep", &url, &GitReference::Branch("feature".into()), Some(&first), false).unwrap();
    assert_eq!(refreshed, advanced);
    assert_eq!(std::fs::read_to_string(dir.join("b.txt")).unwrap(), "feat2");

    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn immutable_reference_reuses_locked_commit_online_without_fetching() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let base = unique_dir("immutable");
    let home = base.join("home");
    let (url, c1, _c2) = fixture_repo(&base);

    // Cache the tagged commit.
    let (_, commit) = resolve(&home, "dep", &url, &GitReference::Tag("v1".into()), None, false).unwrap();
    assert_eq!(commit, c1);

    // Make the source repository unreachable; any fetch would now fail.
    std::fs::remove_dir_all(base.join("src")).unwrap();

    // Online, an immutable locked reference is served from the cache without contacting the remote.
    let (dir, reused) = resolve(&home, "dep", &url, &GitReference::Tag("v1".into()), Some(&c1), false).unwrap();
    assert_eq!(reused, c1);
    assert_eq!(std::fs::read_to_string(dir.join("a.txt")).unwrap(), "one");

    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn offline_without_cache_errors() {
    let base = unique_dir("offline");
    let home = base.join("home");
    let result = resolve(&home, "dep", "file:///nonexistent/repo", &GitReference::DefaultBranch, None, true);
    assert!(result.is_err());
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn clone_failure_errors() {
    // Online resolution of an unreachable repository must error (not panic).
    let base = unique_dir("clonefail");
    let home = base.join("home");
    let result = resolve(&home, "dep", "file:///no/such/leo/repo", &GitReference::DefaultBranch, None, false);
    assert!(result.is_err());
    let _ = std::fs::remove_dir_all(&base);
}

/// A locked immutable reference is honored even when the checkout is gone (e.g. a fresh
/// machine): the repository is re-fetched but the LOCKED commit is checked out, so a tag moved
/// upstream cannot change what is built.
#[test]
fn locked_tag_wins_when_checkout_missing() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let base = unique_dir("relock");
    let home = base.join("home");
    let (url, c1, c2) = fixture_repo(&base);
    let src = base.join("src");

    // Pin the tag, then clear the cache and move the tag upstream.
    let (_, commit) = resolve(&home, "dep", &url, &GitReference::Tag("v1".into()), None, false).unwrap();
    assert_eq!(commit, c1);
    std::fs::remove_dir_all(home.join("git")).unwrap();
    run_git(&src, &["tag", "-f", "v1", &c2]);

    // Re-resolution fetches again but checks out the locked commit, not the moved tag.
    let (dir, commit) = resolve(&home, "dep", &url, &GitReference::Tag("v1".into()), Some(&c1), false).unwrap();
    assert_eq!(commit, c1);
    assert_eq!(std::fs::read_to_string(dir.join("a.txt")).unwrap(), "one");

    let _ = std::fs::remove_dir_all(&base);
}

/// Checkouts are keyed by URL and commit, so all dependencies into one repository share them.
#[test]
fn checkouts_are_shared_across_dependency_names() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let base = unique_dir("shared");
    let home = base.join("home");
    let (url, c1, _c2) = fixture_repo(&base);

    let (dir_a, _) = resolve(&home, "depa", &url, &GitReference::Tag("v1".into()), None, false).unwrap();
    let (dir_b, _) = resolve(&home, "depb", &url, &GitReference::Tag("v1".into()), Some(&c1), false).unwrap();
    assert_eq!(dir_a, dir_b, "same repository and commit must share one checkout");

    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn missing_reference_errors() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let base = unique_dir("missingref");
    let home = base.join("home");
    let (url, _c1, _c2) = fixture_repo(&base);

    // A branch, tag, or revision that does not exist must error.
    assert!(resolve(&home, "dep", &url, &GitReference::Branch("nope".into()), None, false).is_err());
    assert!(resolve(&home, "dep", &url, &GitReference::Tag("v9.9.9".into()), None, false).is_err());
    let bad_rev = "0000000000000000000000000000000000000000".to_string();
    assert!(resolve(&home, "dep", &url, &GitReference::Rev(bad_rev), None, false).is_err());

    let _ = std::fs::remove_dir_all(&base);
}

/// Clones a real public repository over HTTPS, pinned to an immutable commit, and locates a
/// package within it. Ignored by default since it needs network access; run with
/// `cargo test -p leo-package -- --ignored`.
#[test]
#[ignore = "requires network access"]
fn resolves_real_github_repo() {
    let base = unique_dir("real");
    let home = base.join("home");
    let url = "https://github.com/ProvableHQ/leo-examples";
    let rev = "6728690fcf10261a4023cf4f64b9c7960296d4e0";

    let (dir, commit) = resolve(&home, "helloworld.aleo", url, &GitReference::Rev(rev.into()), None, false).unwrap();
    assert_eq!(commit, rev);

    // `helloworld` lives in a subdirectory; it is found as a package directory by name.
    let located = crate::find_in_checkout(&dir, "helloworld.aleo").unwrap();
    assert!(located.is_dir(), "expected a Leo package directory, found a bytecode file");
    assert!(located.join(MANIFEST_FILENAME).is_file());

    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn find_in_checkout_locates_each_kind_and_errors_when_absent() {
    let base = unique_dir("find");
    // A library in a subdirectory, a program in a subdirectory, and a bytecode file at the root.
    std::fs::create_dir_all(base.join("lib")).unwrap();
    std::fs::write(
        base.join("lib/program.json"),
        r#"{"program":"mylib","version":"0.1.0","description":"","license":"MIT"}"#,
    )
    .unwrap();
    std::fs::create_dir_all(base.join("prog")).unwrap();
    std::fs::write(
        base.join("prog/program.json"),
        r#"{"program":"myprog.aleo","version":"0.1.0","description":"","license":"MIT"}"#,
    )
    .unwrap();
    std::fs::write(base.join("mybytes.aleo"), "// bytecode\n").unwrap();

    // The library and program are located as package directories; the root `.aleo` as a file.
    assert!(crate::find_in_checkout(&base, "mylib").unwrap().is_dir());
    assert!(crate::find_in_checkout(&base, "myprog.aleo").unwrap().is_dir());
    let bytes = crate::find_in_checkout(&base, "mybytes.aleo").unwrap();
    assert!(bytes.is_file() && bytes.extension().and_then(|e| e.to_str()) == Some("aleo"));
    // A name present in neither a manifest nor as a root `.aleo` file errors.
    assert!(crate::find_in_checkout(&base, "absent").is_err());

    let _ = std::fs::remove_dir_all(&base);
}

/// A directory declaring exactly the requested name form wins over the alternate (`.aleo`) form.
#[test]
fn find_in_checkout_prefers_exact_name_form() {
    let base = unique_dir("exact");
    std::fs::create_dir_all(base.join("lib")).unwrap();
    std::fs::write(
        base.join("lib/program.json"),
        r#"{"program":"foo","version":"0.1.0","description":"","license":"MIT"}"#,
    )
    .unwrap();
    std::fs::create_dir_all(base.join("prog")).unwrap();
    std::fs::write(
        base.join("prog/program.json"),
        r#"{"program":"foo.aleo","version":"0.1.0","description":"","license":"MIT"}"#,
    )
    .unwrap();

    // `foo` matches the library, `foo.aleo` the program — regardless of directory sort order.
    assert!(crate::find_in_checkout(&base, "foo").unwrap().ends_with("lib"));
    assert!(crate::find_in_checkout(&base, "foo.aleo").unwrap().ends_with("prog"));

    let _ = std::fs::remove_dir_all(&base);
}

/// Multiple directories declaring the same program name are ambiguous, not first-match-wins.
#[test]
fn find_in_checkout_errors_on_ambiguous_name() {
    let base = unique_dir("ambiguous");
    for dir in ["examples/dup", "dup"] {
        std::fs::create_dir_all(base.join(dir)).unwrap();
        std::fs::write(
            base.join(dir).join("program.json"),
            r#"{"program":"dup","version":"0.1.0","description":"","license":"MIT"}"#,
        )
        .unwrap();
    }

    let err = crate::find_in_checkout(&base, "dup").unwrap_err();
    assert!(err.to_string().contains("ambiguous"), "expected ambiguity error: {err}");

    let _ = std::fs::remove_dir_all(&base);
}

/// `find_in_checkout` must not follow symlinks out of the checkout, or a malicious repo could match
/// (and compile) a package outside it.
#[cfg(unix)]
#[test]
fn find_in_checkout_does_not_follow_symlinks() {
    let base = unique_dir("symlink");
    // A package that lives OUTSIDE the checkout directory.
    let outside = base.join("outside");
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::write(
        outside.join("program.json"),
        r#"{"program":"secret","version":"0.1.0","description":"","license":"MIT"}"#,
    )
    .unwrap();
    // The checkout contains only a symlink pointing at the outside package.
    let checkout = base.join("checkout");
    std::fs::create_dir_all(&checkout).unwrap();
    std::os::unix::fs::symlink(&outside, checkout.join("link")).unwrap();

    // The search must not traverse the symlink, so the outside package is not found.
    assert!(crate::find_in_checkout(&checkout, "secret").is_err());

    let _ = std::fs::remove_dir_all(&base);
}

// The `leo.lock` lock file (`crate::Lock`).

#[test]
fn round_trip_and_lookup() {
    let dir = unique_dir("lock");
    let mut lock = Lock::read(&dir);
    assert!(lock.is_empty());

    lock.record("foo.aleo".into(), "https://example.com/foo".into(), "tag=v1".into(), "abc123".into());
    lock.write(&dir).unwrap();

    let reloaded = Lock::read(&dir);
    assert_eq!(reloaded.commit_for("foo.aleo", "https://example.com/foo", "tag=v1"), Some("abc123"));
    // Reference mismatch forces re-resolution.
    assert_eq!(reloaded.commit_for("foo.aleo", "https://example.com/foo", "tag=v2"), None);
    // URL mismatch forces re-resolution.
    assert_eq!(reloaded.commit_for("foo.aleo", "https://example.com/other", "tag=v1"), None);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn record_replaces_existing_commit() {
    let mut lock = Lock::default();
    lock.record("foo".into(), "url".into(), "branch=main".into(), "c1".into());
    lock.record("foo".into(), "url".into(), "branch=main".into(), "c2".into());
    assert_eq!(lock.commit_for("foo", "url", "branch=main"), Some("c2"));
}

#[test]
fn record_keeps_entries_under_other_references() {
    let mut lock = Lock::default();
    lock.record("foo".into(), "url".into(), "default".into(), "c1".into());
    // In a shared workspace lock an entry under another reference may belong to another member,
    // so recording must not evict it; stale entries are pruned by `carry_over` or `leo remove`.
    lock.record("foo".into(), "url".into(), "tag=v1".into(), "c2".into());
    assert_eq!(lock.commit_for("foo", "url", "default"), Some("c1"));
    assert_eq!(lock.commit_for("foo", "url", "tag=v1"), Some("c2"));
}

#[test]
fn carry_over_keeps_only_accepted_unrecorded_entries() {
    let mut old = Lock::default();
    old.record("foo".into(), "url".into(), "tag=v1".into(), "c1".into());
    old.record("bar".into(), "url2".into(), "default".into(), "c2".into());

    let mut new = Lock::default();
    new.record("foo".into(), "url".into(), "tag=v1".into(), "c9".into());
    new.carry_over(&old, |entry| entry.name == "bar");

    // The re-recorded entry is not overwritten, and only accepted old entries are carried.
    assert_eq!(new.commit_for("foo", "url", "tag=v1"), Some("c9"));
    assert_eq!(new.commit_for("bar", "url2", "default"), Some("c2"));
}

#[test]
fn remove_name_drops_all_entries_for_dependency() {
    let mut lock = Lock::default();
    lock.record("foo".into(), "url".into(), "tag=v1".into(), "c1".into());
    lock.record("foo".into(), "url2".into(), "default".into(), "c2".into());
    lock.record("bar".into(), "url".into(), "default".into(), "c3".into());
    lock.remove_name("foo");
    assert_eq!(lock.commit_for("foo", "url", "tag=v1"), None);
    assert_eq!(lock.commit_for("foo", "url2", "default"), None);
    assert_eq!(lock.commit_for("bar", "url", "default"), Some("c3"));
}

#[test]
fn empty_lock_is_not_written() {
    let dir = unique_dir("lock-empty");
    Lock::default().write(&dir).unwrap();
    assert!(!dir.join(LOCK_FILENAME).exists());
    let _ = std::fs::remove_dir_all(&dir);
}

// Manifest validation of git dependencies (`crate::Manifest`).

#[test]
fn manifest_rejects_git_dependency_without_git_url() {
    let err = read_manifest(&manifest_json(r#"[{"name":"foo.aleo","location":"git"}]"#, "null")).unwrap_err();

    assert!(err.to_string().contains("invalid dependency `foo.aleo`"));
    assert!(err.to_string().contains("`git` dependencies must specify `git`"));
}

#[test]
fn manifest_rejects_git_dependency_with_path() {
    let err = read_manifest(&manifest_json(
        r#"[{"name":"foo.aleo","location":"git","git":{"url":"https://example.com/foo"},"path":"../foo"}]"#,
        "null",
    ))
    .unwrap_err();

    assert!(err.to_string().contains("`git` dependencies cannot specify `path`"));
}

#[test]
fn manifest_rejects_git_dependency_with_edition() {
    let err = read_manifest(&manifest_json(
        r#"[{"name":"foo.aleo","location":"git","git":{"url":"https://example.com/foo"},"edition":1}]"#,
        "null",
    ))
    .unwrap_err();

    assert!(err.to_string().contains("`git` dependencies cannot specify `edition`"));
}

#[test]
fn manifest_rejects_git_dependency_with_multiple_references() {
    let err = read_manifest(&manifest_json(
        r#"[{"name":"foo.aleo","location":"git","git":{"url":"https://example.com/foo","branch":"main","tag":"v1"}}]"#,
        "null",
    ))
    .unwrap_err();

    assert!(err.to_string().contains("at most one of `branch`, `tag`, or `rev`"));
}

#[test]
fn manifest_rejects_git_field_on_non_git_dependency() {
    // A stray `git` object on a non-git dependency must error, not be silently ignored.
    let err = read_manifest(&manifest_json(
        r#"[{"name":"foo.aleo","location":"local","path":"../foo","git":{"url":"https://example.com/foo"}}]"#,
        "null",
    ))
    .unwrap_err();
    assert!(err.to_string().contains("`local` dependencies cannot specify `git`"));

    let err = read_manifest(&manifest_json(
        r#"[{"name":"foo.aleo","location":"network","edition":1,"git":{"url":"https://example.com/foo"}}]"#,
        "null",
    ))
    .unwrap_err();
    assert!(err.to_string().contains("`network` dependencies cannot specify `git`"));
}

#[test]
fn manifest_rejects_invalid_git_dependency_name() {
    // The name is matched against checkout manifests, so it must be a valid package name.
    let err = read_manifest(&manifest_json(
        r#"[{"name":"not a name","location":"git","git":{"url":"https://example.com/foo"}}]"#,
        "null",
    ))
    .unwrap_err();
    assert!(err.to_string().contains("must be a valid program or library name"));
}

#[test]
fn manifest_accepts_git_dependency_variants() {
    let manifest = read_manifest(&manifest_json(
        r#"[
  {"name":"git_default.aleo","location":"git","git":{"url":"https://example.com/a"}},
  {"name":"git_branch.aleo","location":"git","git":{"url":"https://example.com/b","branch":"main"}},
  {"name":"git_tag.aleo","location":"git","git":{"url":"https://example.com/c","tag":"v0.1.0"}},
  {"name":"git_rev.aleo","location":"git","git":{"url":"https://example.com/d","rev":"abc123"}}
]"#,
        "null",
    ))
    .unwrap();

    assert_eq!(manifest.dependencies.unwrap().len(), 4);
}

#[test]
fn manifest_accepts_git_dev_dependency() {
    // The same validation applies to `dev_dependencies`, so a git dev-dependency is accepted there.
    let manifest = read_manifest(&manifest_json(
        "null",
        r#"[{"name":"mylib","location":"git","git":{"url":"https://example.com/a","tag":"v0.1.0"}}]"#,
    ))
    .unwrap();

    assert_eq!(manifest.dev_dependencies.unwrap().len(), 1);
}

// End-to-end resolution through `Package::from_directory`.

/// A consumer package with a git dependency on a Leo library resolves the library through a
/// `file://` clone and records the commit in `leo.lock`.
#[test]
fn git_dependency_resolves_and_locks() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("e2e");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    let lib = root.join("mylib_repo");
    write_library(&lib, "mylib", "null");
    init_repo(&lib, None);

    let consumer = root.join("consumer");
    let url = file_url(&lib);
    write_consumer(&consumer, &format!(r#"{{"name":"mylib","location":"git","git":{{"url":"{url}"}}}}"#));

    leo_span::create_session_if_not_set_then(|_| {
        let package = Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();

        // The library was resolved as a Library compilation unit.
        let lib_unit = package
            .compilation_units
            .iter()
            .find(|u| u.name == Symbol::intern("mylib"))
            .expect("mylib compilation unit present");
        assert!(lib_unit.kind.is_library());

        // The lock file was written and pins the library to a commit.
        let lock = Lock::read(&consumer);
        let commit = lock.commit_for("mylib", &url, "default").expect("lock pins mylib");
        assert_eq!(commit.len(), 40);

        // A second resolution re-resolves the mutable default branch and still succeeds (the lock
        // is consulted, but a default-branch reference is re-fetched online; see
        // `immutable_reference_reuses_locked_commit_online_without_fetching` for the no-fetch case).
        let _ = Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// A git dependency listed under `dev_dependencies` is resolved and locked when the package is
/// built with tests (the path `leo test` / `leo build --tests` takes).
#[test]
fn git_dev_dependency_resolves_with_tests() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("dev_dep");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    let lib = root.join("mylib_repo");
    write_library(&lib, "mylib", "null");
    init_repo(&lib, None);
    let url = file_url(&lib);

    let consumer = root.join("consumer");
    write_file(
        &consumer.join(MANIFEST_FILENAME),
        &format!(
            r#"{{"program":"consumer.aleo","version":"0.1.0","description":"","license":"MIT","dependencies":null,"dev_dependencies":[{{"name":"mylib","location":"git","git":{{"url":"{url}"}}}}]}}"#
        ),
    );
    write_file(&consumer.join("src/main.leo"), "// main\n");

    leo_span::create_session_if_not_set_then(|_| {
        // A plain build ignores dev-dependencies; building with tests resolves them.
        let package = Package::from_directory_with_tests(&consumer, &home, false, false, false, None, None, 3).unwrap();
        assert!(
            package.compilation_units.iter().any(|u| u.name == Symbol::intern("mylib")),
            "git dev-dependency resolved",
        );
        assert!(Lock::read(&consumer).commit_for("mylib", &url, "default").is_some(), "dev-dependency locked");
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// With `offline`, a build whose git dependency is locked and cached succeeds without any
/// network access, even for a mutable (default branch) reference.
#[test]
fn offline_build_uses_locked_commit_and_cache() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("offline_e2e");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    let lib = root.join("mylib_repo");
    write_library(&lib, "mylib", "null");
    init_repo(&lib, None);
    let url = file_url(&lib);

    let consumer = root.join("consumer");
    write_consumer(&consumer, &format!(r#"{{"name":"mylib","location":"git","git":{{"url":"{url}"}}}}"#));

    leo_span::create_session_if_not_set_then(|_| {
        // Build online once to populate the lock and the checkout cache.
        Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();
        let commit = Lock::read(&consumer).commit_for("mylib", &url, "default").expect("locked").to_string();

        // Make the source repository unreachable; any fetch would now fail.
        std::fs::remove_dir_all(&lib).unwrap();

        // The offline build reuses the locked commit from the cache.
        Package::from_directory(&consumer, &home, false, false, true, None, None, 3).unwrap();
        assert_eq!(Lock::read(&consumer).commit_for("mylib", &url, "default"), Some(commit.as_str()));
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// A regular `dependency` must be visible to in-package tests, and a local library in both
/// `dependencies` and `dev_dependencies` must dedup rather than conflict. Regression test for #29592.
#[test]
fn library_visible_to_src_and_tests() {
    let run = |dev_dependencies: &str| {
        let root = unique_dir("lib_visibility");
        let home = root.join("home");
        std::fs::create_dir_all(&home).unwrap();

        let lib = root.join("mylib");
        write_library(&lib, "mylib", "null");

        let app = root.join("app");
        write_file(
            &app.join(MANIFEST_FILENAME),
            &format!(
                r#"{{"program":"app.aleo","version":"0.1.0","description":"","license":"MIT","dependencies":[{{"name":"mylib","location":"local","path":"../mylib"}}],"dev_dependencies":{dev_dependencies}}}"#
            ),
        );
        write_file(&app.join("src/main.leo"), "// main\n");
        write_file(&app.join("tests/test_app.leo"), "// test\n");

        leo_span::create_session_if_not_set_then(|_| {
            // Relative `../mylib` and the canonicalized absolute path are the same library.
            let package = Package::from_directory_with_tests(&app, &home, false, false, false, None, None, 3).unwrap();
            let mylib = Symbol::intern("mylib");
            let test_unit = package
                .compilation_units
                .iter()
                .find(|u| u.kind.is_test())
                .expect("the in-package test program is a compilation unit");
            assert!(
                test_unit.dependencies.iter().any(|d| d.name == "mylib"),
                "a regular `dependencies` library is visible to the test program",
            );
            assert_eq!(
                package.compilation_units.iter().filter(|u| u.name == mylib).count(),
                1,
                "the shared local library is resolved exactly once",
            );
        });

        let _ = std::fs::remove_dir_all(&root);
    };

    // Case 2: library only in `dependencies` — must still be visible to the test program.
    run("null");
    // Case 1: library in both lists — must dedup, not conflict.
    run(r#"[{"name":"mylib","location":"local","path":"../mylib"}]"#);
}

/// A plain (non-test) build must not drop a git dev-dependency's pin from `leo.lock`.
#[test]
fn plain_build_keeps_dev_dependency_pin() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("dev_pin");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    let lib = root.join("mylib_repo");
    write_library(&lib, "mylib", "null");
    init_repo(&lib, None);
    let url = file_url(&lib);

    let consumer = root.join("consumer");
    write_file(
        &consumer.join(MANIFEST_FILENAME),
        &format!(
            r#"{{"program":"consumer.aleo","version":"0.1.0","description":"","license":"MIT","dependencies":null,"dev_dependencies":[{{"name":"mylib","location":"git","git":{{"url":"{url}"}}}}]}}"#
        ),
    );
    write_file(&consumer.join("src/main.leo"), "// main\n");

    leo_span::create_session_if_not_set_then(|_| {
        // A test build records the dev pin; a subsequent plain build must carry it over.
        Package::from_directory_with_tests(&consumer, &home, false, false, false, None, None, 3).unwrap();
        let commit = Lock::read(&consumer).commit_for("mylib", &url, "default").expect("locked").to_string();
        Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();
        assert_eq!(Lock::read(&consumer).commit_for("mylib", &url, "default"), Some(commit.as_str()));

        // Once the dev dependency is gone from the manifest, the plain build prunes its pin.
        write_file(
            &consumer.join(MANIFEST_FILENAME),
            r#"{"program":"consumer.aleo","version":"0.1.0","description":"","license":"MIT","dependencies":null,"dev_dependencies":null}"#,
        );
        Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();
        assert!(!consumer.join(LOCK_FILENAME).exists(), "stale lock file removed");
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// Changing a git dependency's reference re-resolves it and prunes the stale lock entry.
#[test]
fn git_dependency_ref_change_updates_lock() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("refchange");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    let lib = root.join("mylib_repo");
    write_library(&lib, "mylib", "null");
    init_repo(&lib, Some("v1"));
    let url = file_url(&lib);
    let consumer = root.join("consumer");

    leo_span::create_session_if_not_set_then(|_| {
        // Track the default branch first.
        write_consumer(&consumer, &format!(r#"{{"name":"mylib","location":"git","git":{{"url":"{url}"}}}}"#));
        Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();
        assert!(Lock::read(&consumer).commit_for("mylib", &url, "default").is_some());

        // Re-pin to the tag: the lock gains the tag entry and drops the stale default one.
        write_consumer(
            &consumer,
            &format!(r#"{{"name":"mylib","location":"git","git":{{"url":"{url}","tag":"v1"}}}}"#),
        );
        Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();
        let lock = Lock::read(&consumer);
        assert!(lock.commit_for("mylib", &url, "tag=v1").is_some(), "tag entry recorded");
        assert!(lock.commit_for("mylib", &url, "default").is_none(), "stale default entry pruned");
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// A git dependency whose own manifest declares another git dependency is resolved transitively.
#[test]
fn transitive_git_dependency() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("transitive");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    // libb (leaf), then liba which depends on libb via git.
    let libb = root.join("libb_repo");
    write_library(&libb, "libb", "null");
    init_repo(&libb, None);
    let url_b = file_url(&libb);

    let liba = root.join("liba_repo");
    write_library(&liba, "liba", &format!(r#"[{{"name":"libb","location":"git","git":{{"url":"{url_b}"}}}}]"#));
    init_repo(&liba, None);
    let url_a = file_url(&liba);

    let consumer = root.join("consumer");
    write_consumer(&consumer, &format!(r#"{{"name":"liba","location":"git","git":{{"url":"{url_a}"}}}}"#));

    leo_span::create_session_if_not_set_then(|_| {
        let package = Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();
        let names: Vec<String> = package.compilation_units.iter().map(|u| u.name.to_string()).collect();
        assert!(names.iter().any(|n| n == "liba"), "liba resolved: {names:?}");
        assert!(names.iter().any(|n| n == "libb"), "transitive libb resolved: {names:?}");

        let lock = Lock::read(&consumer);
        assert!(lock.commit_for("liba", &url_a, "default").is_some());
        assert!(lock.commit_for("libb", &url_b, "default").is_some());
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// A git program dependency that itself declares a git library dependency resolves the whole
/// chain, mixing package kinds (program and library) across the transitive git edges.
#[test]
fn transitive_git_program_depends_on_library() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("transitive_mixed");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    // deeplib (library leaf), then midprog (a program) which depends on deeplib via git.
    let deeplib = root.join("deeplib_repo");
    write_library(&deeplib, "deeplib", "null");
    init_repo(&deeplib, None);
    let url_lib = file_url(&deeplib);

    let midprog = root.join("midprog_repo");
    write_program(
        &midprog,
        "midprog.aleo",
        &format!(r#"[{{"name":"deeplib","location":"git","git":{{"url":"{url_lib}"}}}}]"#),
    );
    init_repo(&midprog, None);
    let url_prog = file_url(&midprog);

    let consumer = root.join("consumer");
    write_consumer(&consumer, &format!(r#"{{"name":"midprog.aleo","location":"git","git":{{"url":"{url_prog}"}}}}"#));

    leo_span::create_session_if_not_set_then(|_| {
        let package = Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();

        // The git program resolved as a program, and its own git library resolved transitively.
        let prog = package
            .compilation_units
            .iter()
            .find(|u| u.name == Symbol::intern("midprog.aleo"))
            .expect("git program resolved");
        assert!(prog.kind.is_program());
        let lib = package
            .compilation_units
            .iter()
            .find(|u| u.name == Symbol::intern("deeplib"))
            .expect("transitive git library resolved");
        assert!(lib.kind.is_library());

        let lock = Lock::read(&consumer);
        assert!(lock.commit_for("midprog.aleo", &url_prog, "default").is_some());
        assert!(lock.commit_for("deeplib", &url_lib, "default").is_some());
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// A git dependency may point at a repository that is itself a Leo workspace. The member is
/// located by name (the repository's `workspace.json` is ignored for location), and a member's
/// intra-workspace `workspace` dependency on a sibling resolves within the same checkout.
#[test]
fn git_dependency_into_workspace_repo_resolves_member_and_sibling() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("git_ws");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    // A single git repository that is a workspace of two libraries, where `libb` depends on
    // `liba` through a `workspace` dependency.
    let repo = root.join("ws_repo");
    write_file(&repo.join(WORKSPACE_MANIFEST_FILENAME), r#"{"members":["liba","libb"]}"#);
    write_library(&repo.join("liba"), "liba", "null");
    write_library(&repo.join("libb"), "libb", r#"[{"name":"liba","location":"workspace"}]"#);
    init_repo(&repo, None);
    let url = file_url(&repo);

    let consumer = root.join("consumer");
    write_consumer(&consumer, &format!(r#"{{"name":"libb","location":"git","git":{{"url":"{url}"}}}}"#));

    leo_span::create_session_if_not_set_then(|_| {
        let package = Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();
        let names: Vec<String> = package.compilation_units.iter().map(|u| u.name.to_string()).collect();
        // The requested member and its in-repo workspace sibling both resolve.
        assert!(names.iter().any(|n| n == "libb"), "git workspace member resolved: {names:?}");
        assert!(names.iter().any(|n| n == "liba"), "sibling workspace member resolved: {names:?}");

        // The sibling is rewritten to a git dependency on the same source, so both are locked
        // (to the same commit, since the repository is resolved once per build).
        let lock = Lock::read(&consumer);
        let libb_commit = lock.commit_for("libb", &url, "default").expect("libb locked");
        assert_eq!(lock.commit_for("liba", &url, "default"), Some(libb_commit));
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// A workspace member reached both as a direct git dependency and through a sibling's
/// intra-workspace dependency is the same dependency, not a conflict.
#[test]
fn direct_and_sibling_route_to_same_git_member_do_not_conflict() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("git_ws_both");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    let repo = root.join("ws_repo");
    write_file(&repo.join(WORKSPACE_MANIFEST_FILENAME), r#"{"members":["liba","libb"]}"#);
    write_library(&repo.join("liba"), "liba", "null");
    write_library(&repo.join("libb"), "libb", r#"[{"name":"liba","location":"workspace"}]"#);
    init_repo(&repo, None);
    let url = file_url(&repo);

    // The consumer depends on BOTH members directly, and libb also reaches liba internally.
    let consumer = root.join("consumer");
    write_consumer(
        &consumer,
        &format!(
            r#"{{"name":"liba","location":"git","git":{{"url":"{url}"}}}},{{"name":"libb","location":"git","git":{{"url":"{url}"}}}}"#
        ),
    );

    leo_span::create_session_if_not_set_then(|_| {
        let package = Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap();
        let names: Vec<String> = package.compilation_units.iter().map(|u| u.name.to_string()).collect();
        assert!(names.iter().any(|n| n == "liba"), "liba resolved: {names:?}");
        assert!(names.iter().any(|n| n == "libb"), "libb resolved: {names:?}");
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// A package fetched via git may not reference local paths outside its own checkout.
#[test]
fn git_dependency_path_escape_errors() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("escape");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    // A library OUTSIDE any checkout that the malicious repo tries to pull in by absolute path.
    let outside = root.join("outside");
    write_library(&outside, "outside", "null");
    let outside_path = outside.canonicalize().unwrap().display().to_string().replace('\\', "/");

    // The repository's package declares a `local` dependency with an absolute path.
    let evil = root.join("evil_repo");
    write_library(&evil, "evil", &format!(r#"[{{"name":"outside","location":"local","path":"{outside_path}"}}]"#));
    init_repo(&evil, None);
    let url = file_url(&evil);

    let consumer = root.join("consumer");
    write_consumer(&consumer, &format!(r#"{{"name":"evil","location":"git","git":{{"url":"{url}"}}}}"#));

    leo_span::create_session_if_not_set_then(|_| {
        let err = Package::from_directory(&consumer, &home, false, false, false, None, None, 3).unwrap_err();
        assert!(err.to_string().contains("inside its own repository checkout"), "path escape must be rejected: {err}");
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// Two dependencies on the same program with different git references conflict.
#[test]
fn conflicting_git_dependency_errors() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("conflict");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    let lib = root.join("shared_repo");
    write_library(&lib, "shared", "null");
    init_repo(&lib, None);
    let url = file_url(&lib);

    // The same dependency name pinned two different ways.
    let consumer = root.join("consumer");
    write_consumer(
        &consumer,
        &format!(
            r#"{{"name":"shared","location":"git","git":{{"url":"{url}","branch":"main"}}}},{{"name":"shared","location":"git","git":{{"url":"{url}"}}}}"#
        ),
    );

    leo_span::create_session_if_not_set_then(|_| {
        let result = Package::from_directory(&consumer, &home, false, false, false, None, None, 3);
        assert!(result.is_err(), "conflicting git references must error");
    });

    let _ = std::fs::remove_dir_all(&root);
}

// Workspace lock sharing across independently-built members.

/// In a workspace, members are built independently but share one `leo.lock` at the root.
/// Building a second member must merge into, not clobber, the first member's git entry.
#[test]
fn workspace_members_share_lock_without_clobbering() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("ws_lock");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    // Two independent git libraries, one per workspace member.
    let liba = root.join("liba_repo");
    write_library(&liba, "liba", "null");
    init_repo(&liba, None);
    let url_a = file_url(&liba);

    let libb = root.join("libb_repo");
    write_library(&libb, "libb", "null");
    init_repo(&libb, None);
    let url_b = file_url(&libb);

    // A workspace whose two members each depend on a different git library.
    let ws = root.join("ws");
    write_file(&ws.join(WORKSPACE_MANIFEST_FILENAME), r#"{"members":["mema","memb"]}"#);
    let mema = ws.join("mema");
    write_file(
        &mema.join(MANIFEST_FILENAME),
        &format!(
            r#"{{"program":"mema.aleo","version":"0.1.0","description":"","license":"MIT","dependencies":[{{"name":"liba","location":"git","git":{{"url":"{url_a}"}}}}]}}"#
        ),
    );
    write_file(&mema.join("src/main.leo"), "// main\n");
    let memb = ws.join("memb");
    write_file(
        &memb.join(MANIFEST_FILENAME),
        &format!(
            r#"{{"program":"memb.aleo","version":"0.1.0","description":"","license":"MIT","dependencies":[{{"name":"libb","location":"git","git":{{"url":"{url_b}"}}}}]}}"#
        ),
    );
    write_file(&memb.join("src/main.leo"), "// main\n");

    leo_span::create_session_if_not_set_then(|_| {
        // Build each member independently, as `leo build` does for a workspace.
        Package::from_directory(&mema, &home, false, false, false, None, None, 3).unwrap();
        Package::from_directory(&memb, &home, false, false, false, None, None, 3).unwrap();

        // The shared lock at the workspace root retains both members' entries.
        let lock = Lock::read(&ws);
        assert!(lock.commit_for("liba", &url_a, "default").is_some(), "first member's entry retained");
        assert!(lock.commit_for("libb", &url_b, "default").is_some(), "second member's entry recorded");
    });

    let _ = std::fs::remove_dir_all(&root);
}

/// Two members pinning the same dependency name and URL at different references must not evict
/// each other's entries from the shared workspace lock.
#[test]
fn workspace_members_keep_different_references_to_same_repo() {
    if !git_available() {
        eprintln!("skipping: `git` CLI not available");
        return;
    }
    let root = unique_dir("ws_refs");
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();

    let lib = root.join("shared_repo");
    write_library(&lib, "shared", "null");
    init_repo(&lib, Some("v1"));
    let url = file_url(&lib);

    let ws = root.join("ws");
    write_file(&ws.join(WORKSPACE_MANIFEST_FILENAME), r#"{"members":["mema","memb"]}"#);
    let mema = ws.join("mema");
    write_file(
        &mema.join(MANIFEST_FILENAME),
        &format!(
            r#"{{"program":"mema.aleo","version":"0.1.0","description":"","license":"MIT","dependencies":[{{"name":"shared","location":"git","git":{{"url":"{url}","tag":"v1"}}}}]}}"#
        ),
    );
    write_file(&mema.join("src/main.leo"), "// main\n");
    let memb = ws.join("memb");
    write_file(
        &memb.join(MANIFEST_FILENAME),
        &format!(
            r#"{{"program":"memb.aleo","version":"0.1.0","description":"","license":"MIT","dependencies":[{{"name":"shared","location":"git","git":{{"url":"{url}"}}}}]}}"#
        ),
    );
    write_file(&memb.join("src/main.leo"), "// main\n");

    leo_span::create_session_if_not_set_then(|_| {
        // Build each member twice; the entries must not thrash.
        for member in [&mema, &memb, &mema, &memb] {
            Package::from_directory(member, &home, false, false, false, None, None, 3).unwrap();
        }
        let lock = Lock::read(&ws);
        assert!(lock.commit_for("shared", &url, "tag=v1").is_some(), "tag entry retained");
        assert!(lock.commit_for("shared", &url, "default").is_some(), "default entry retained");
    });

    let _ = std::fs::remove_dir_all(&root);
}
