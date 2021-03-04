// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::test_dir;
use leo_package::inputs::InputFile;
use leo_package::inputs::InputsDirectory;
use leo_package::inputs::StateFile;
use leo_package::package::Package;
use leo_package::root::Manifest;
use leo_package::source::LibraryFile;
use leo_package::source::MainFile;
use leo_package::source::SourceDirectory;

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
    Manifest::new(TEST_PACKAGE_NAME)
        .unwrap()
        .write_to(&test_directory)
        .unwrap();

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
