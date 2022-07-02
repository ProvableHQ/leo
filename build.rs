// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use std::{fs::File, io::Read, path::Path};

use walkdir::WalkDir;

// The following license text that should be present at the beginning of every source file.
const EXPECTED_LICENSE_TEXT: &[u8] = include_bytes!(".resources/license_header");

// The following directories will be excluded from the license scan.
const DIRS_TO_SKIP: [&str; 9] = [
    ".cargo",
    ".circleci",
    ".git",
    ".github",
    ".resources",
    "docs",
    "examples",
    "target",
    "tests",
];

fn check_file_licenses<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();

    let mut iter = WalkDir::new(path).into_iter();
    while let Some(entry) = iter.next() {
        let entry = entry.unwrap();
        let entry_type = entry.file_type();

        // Skip the specified directories.
        if entry_type.is_dir() && DIRS_TO_SKIP.contains(&entry.file_name().to_str().unwrap_or("")) {
            iter.skip_current_dir();

            continue;
        }

        // Check all files with the ".rs" extension.
        if entry_type.is_file() && entry.file_name().to_str().unwrap_or("").ends_with(".rs") {
            let file = File::open(entry.path()).unwrap();
            let mut contents = Vec::with_capacity(EXPECTED_LICENSE_TEXT.len());
            file.take(EXPECTED_LICENSE_TEXT.len() as u64)
                .read_to_end(&mut contents)
                .unwrap();

            assert_eq!(
                contents,
                EXPECTED_LICENSE_TEXT,
                "The license in \"{}\" is either missing or it doesn't match the expected string!",
                entry.path().display()
            );
        }
    }

    // Re-run upon any changes to the workspace.
    println!("cargo:rerun-if-changed=.");
}

// The build script; it currently only checks the licenses.
fn main() {
    // Check licenses in the current folder.
    check_file_licenses(".");
}
