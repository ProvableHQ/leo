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

#![allow(clippy::module_inception)]

pub mod initialize;
pub mod manifest;

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
    pub static TEST_ID: RefCell<Option<usize>> = RefCell::new(None);
}

lazy_static! {
    /// Create a testing directory for packages in `target/`
    pub static ref TEST_DIR: PathBuf = {
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
pub(crate) fn test_dir() -> PathBuf {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    TEST_ID.with(|n| *n.borrow_mut() = Some(id));

    let path: PathBuf = TEST_DIR.join(&format!("t{}", id));

    if path.exists() {
        if let Err(e) = fs::remove_dir_all(&path) {
            panic!("failed to remove {:?}: {:?}", &path, e)
        }
    }

    fs::create_dir_all(&path).unwrap();

    path
}
