// Copyright (C) 2019-2020 Aleo Systems Inc.
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

// Tests for package manifest

use crate::test_dir;
use leo_package::root::{Manifest, MANIFEST_FILENAME};

use std::{
    convert::TryFrom,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

const OLD_MANIFEST_FORMAT: &str = r#"[package]
name = "test-package"
version = "0.1.0"
description = "Testing manifest updates."
license = "MIT"
remote = "author/test-package"
"#;

const NEW_REMOTE_FORMAT: &str = r#"
[remote]
author = "author"
"#;

const OLD_PROJECT_FORMAT: &str = "[package]";
const NEW_PROJECT_FORMAT: &str = "[project]";

/// Create a manifest file with outdated formatting.
fn create_outdated_manifest_file(path: PathBuf) -> PathBuf {
    let mut path = path;
    if path.is_dir() {
        path.push(PathBuf::from(MANIFEST_FILENAME));
    }

    let mut file = File::create(&path).unwrap();
    file.write_all(OLD_MANIFEST_FORMAT.as_bytes()).unwrap();

    path
}

/// Read the manifest file into a string.
fn read_manifest_file(path: &PathBuf) -> String {
    let mut file = File::open(path.clone()).unwrap();
    let size = file.metadata().unwrap().len() as usize;

    let mut buffer = String::with_capacity(size);
    file.read_to_string(&mut buffer).unwrap();

    buffer
}

/// Read the manifest file and check that the remote format is updated.
fn remote_is_updated(path: &PathBuf) -> bool {
    let manifest_string = read_manifest_file(&path);
    for line in manifest_string.lines() {
        if line.starts_with("remote") {
            return false;
        }
    }

    manifest_string.contains(NEW_REMOTE_FORMAT)
}

/// Read the manifest file and check that the project format is updated.
fn project_is_updated(path: &PathBuf) -> bool {
    let manifest_string = read_manifest_file(&path);

    !manifest_string.contains(OLD_PROJECT_FORMAT) && manifest_string.contains(NEW_PROJECT_FORMAT)
}

#[test]
#[cfg_attr(
    any(feature = "manifest_refactor_project", feature = "manifest_refactor_remote"),
    ignore
)]
fn test_manifest_no_refactors() {
    // Create an outdated manifest file.
    let test_directory = test_dir();
    let manifest_path = create_outdated_manifest_file(test_directory);

    // Load the manifest file, and discard the new struct.
    let _manifest = Manifest::try_from(&manifest_path).unwrap();

    // Check that the manifest file project has NOT been updated.
    assert!(!project_is_updated(&manifest_path));

    // Check that the manifest file remote has NOT been updated.
    assert!(!remote_is_updated(&manifest_path));
}

#[test]
#[cfg_attr(
    any(feature = "manifest_refactor_project", not(feature = "manifest_refactor_remote")),
    ignore
)]
fn test_manifest_refactor_remote() {
    // Create an outdated manifest file.
    let test_directory = test_dir();
    let manifest_path = create_outdated_manifest_file(test_directory);

    // Load the manifest file, and discard the new struct.
    let _manifest = Manifest::try_from(&manifest_path).unwrap();

    // Check that the manifest file project has NOT been updated.
    assert!(!project_is_updated(&manifest_path));

    // Check that the manifest file remote has been updated.
    assert!(remote_is_updated(&manifest_path));
}

#[test]
#[cfg_attr(
    any(not(feature = "manifest_refactor_project"), feature = "manifest_refactor_remote"),
    ignore
)]
fn test_manifest_refactor_project() {
    // Create an outdated manifest file.
    let test_directory = test_dir();
    let manifest_path = create_outdated_manifest_file(test_directory);

    // Load the manifest file, and discard the new struct.
    let _manifest = Manifest::try_from(&manifest_path).unwrap();

    // Check that the manifest file project has been updated.
    assert!(project_is_updated(&manifest_path));

    // Check that the manifest file remote has NOT been updated.
    assert!(!remote_is_updated(&manifest_path));
}

#[test]
#[cfg_attr(
    any(
        not(feature = "manifest_refactor_project"),
        not(feature = "manifest_refactor_remote")
    ),
    ignore
)]
fn test_manifest_refactors() {
    // Create an outdated manifest file.
    let test_directory = test_dir();
    let manifest_path = create_outdated_manifest_file(test_directory);

    // Load the manifest file, and discard the new struct.
    let _manifest = Manifest::try_from(&manifest_path).unwrap();

    // Check that the manifest file project has been updated.
    assert!(project_is_updated(&manifest_path));

    // Check that the manifest file remote has been updated.
    assert!(remote_is_updated(&manifest_path));
}
