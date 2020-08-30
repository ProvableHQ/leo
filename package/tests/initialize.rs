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

use lazy_static::lazy_static;
use std::{
    cell::RefCell,
    env,
    fs,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

const PACKAGE_TEST_DIRECTORY: &str = "package-testing";

thread_local! {
    /// Establish a test id for each test.
    static TEST_ID: RefCell<Option<usize>> = RefCell::new(None);
}

lazy_static! {
    /// Create a testing directory for packages in `target/`
    static ref TEST_DIR: PathBuf = {
        let mut path = env::current_exe().unwrap();
        path.pop(); // Remove executable name
        path.pop(); // Remove 'debug'

        // Attempt to point at the `target` directory
        if path.file_name().and_then(|s| s.to_str()) != Some("target") {
            path.pop();
        }

        path.push(PACKAGE_TEST_DIRECTORY);
        fs::create_dir_all(&path).unwrap();

        path
    };
}

/// Create a new directory for each test based on the ID of the test.
fn test_dir() -> PathBuf {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    TEST_ID.with(|n| *n.borrow_mut() = Some(id));

    let path: PathBuf = TEST_DIR.join(&format!("t{}", id)).into();

    if path.exists() {
        if let Err(e) = fs::remove_dir_all(&path) {
            panic!("failed to remove {:?}: {:?}", &path, e)
        }
    }

    fs::create_dir_all(&path).unwrap();

    path
}

// Tests for package initialization
mod initialize_package {
    use super::*;
    use leo_package::{
        inputs::{InputFile, InputsDirectory, StateFile},
        package::Package,
        root::Manifest,
        source::{LibraryFile, MainFile, SourceDirectory},
    };

    const TEST_PACKAGE_NAME: &str = "test-package";

    #[test]
    fn initialize_valid_package() {
        let test_directory = test_dir();

        // Ensure a package can be initialized at the `test_directory`
        assert!(Package::can_initialize(TEST_PACKAGE_NAME, false, &test_directory));

        // Initialize a package at the `test_directory`
        assert!(Package::initialize(TEST_PACKAGE_NAME, false, &test_directory).is_ok());

        // Ensure a package is initialized at the `test_directory`
        assert!(Package::is_initialized(TEST_PACKAGE_NAME, false, &test_directory));
    }

    #[test]
    #[ignore]
    fn initialize_fails_with_invalid_package_names() {
        unimplemented!()
    }

    #[test]
    fn initialize_fails_with_existing_manifest() {
        let test_directory = test_dir();

        // Ensure a package can be initialized at the `test_directory`
        assert!(Package::can_initialize(TEST_PACKAGE_NAME, false, &test_directory));

        // Manually add a manifest file to the `test_directory`
        Manifest::new(TEST_PACKAGE_NAME).write_to(&test_directory).unwrap();

        // Attempt to initialize a package at the `test_directory`
        assert!(Package::initialize(TEST_PACKAGE_NAME, false, &test_directory).is_err());

        // Ensure package is not initialized at the `test_directory`
        assert!(!Package::is_initialized(TEST_PACKAGE_NAME, false, &test_directory));
    }

    #[test]
    fn initialize_fails_with_existing_library_file() {
        let test_directory = test_dir();

        // Ensure a package can be initialized at the `test_directory`
        assert!(Package::can_initialize(TEST_PACKAGE_NAME, true, &test_directory));

        // Manually add a source directory and a library file to the `test_directory`
        SourceDirectory::create(&test_directory).unwrap();
        LibraryFile::new(TEST_PACKAGE_NAME).write_to(&test_directory).unwrap();

        // Attempt to initialize a package at the `test_directory`
        assert!(Package::initialize(TEST_PACKAGE_NAME, true, &test_directory).is_err());

        // Ensure package is not initialized at the `test_directory`
        assert!(!Package::is_initialized(TEST_PACKAGE_NAME, true, &test_directory));
    }

    #[test]
    fn initialize_fails_with_existing_input_file() {
        let test_directory = test_dir();

        // Ensure a package can be initialized at the `test_directory`
        assert!(Package::can_initialize(TEST_PACKAGE_NAME, false, &test_directory));

        // Manually add an inputs directory and an input file to the `test_directory`
        InputsDirectory::create(&test_directory).unwrap();
        InputFile::new(TEST_PACKAGE_NAME).write_to(&test_directory).unwrap();

        // Attempt to initialize a package at the `test_directory`
        assert!(Package::initialize(TEST_PACKAGE_NAME, false, &test_directory).is_err());

        // Ensure package is not initialized at the `test_directory`
        assert!(!Package::is_initialized(TEST_PACKAGE_NAME, false, &test_directory));
    }

    #[test]
    fn initialize_fails_with_existing_state_file() {
        let test_directory = test_dir();

        // Ensure a package can be initialized at the `test_directory`
        assert!(Package::can_initialize(TEST_PACKAGE_NAME, false, &test_directory));

        // Manually add an inputs directory and a state file to the `test_directory`
        InputsDirectory::create(&test_directory).unwrap();
        StateFile::new(TEST_PACKAGE_NAME).write_to(&test_directory).unwrap();

        // Attempt to initialize a package at the `test_directory`
        assert!(Package::initialize(TEST_PACKAGE_NAME, false, &test_directory).is_err());

        // Ensure package is not initialized at the `test_directory`
        assert!(!Package::is_initialized(TEST_PACKAGE_NAME, false, &test_directory));
    }

    #[test]
    fn initialize_fails_with_existing_main_file() {
        let test_directory = test_dir();

        // Ensure a package can be initialized at the `test_directory`
        assert!(Package::can_initialize(TEST_PACKAGE_NAME, false, &test_directory));

        // Manually add a source directory and a main file to the `test_directory`
        SourceDirectory::create(&test_directory).unwrap();
        MainFile::new(TEST_PACKAGE_NAME).write_to(&test_directory).unwrap();

        // Attempt to initialize a package at the `test_directory`
        assert!(Package::initialize(TEST_PACKAGE_NAME, false, &test_directory).is_err());

        // Ensure package is not initialized at the `test_directory`
        assert!(!Package::is_initialized(TEST_PACKAGE_NAME, false, &test_directory));
    }
}
