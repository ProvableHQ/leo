// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use walkdir::WalkDir;

// The following license text that should be present at the beginning of every source file.
const EXPECTED_LICENSE_TEXT: &str = include_str!(".resources/license_header");

// The following directories will be excluded from the license scan.
const DIRS_TO_SKIP: [&str; 9] =
    [".cargo", ".circleci", ".git", ".github", ".resources", "docs", "examples", "target", "tests"];

fn compare_license_text(path: &Path, expected_lines: &[&str]) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for (i, (file_line, expected_line)) in reader.lines().zip(expected_lines).enumerate() {
        let file_line =
            file_line.unwrap_or_else(|_| panic!("Can't read line {} in file \"{}\"!", i + 1, path.display()));
        assert_eq!(
            &file_line,
            expected_line,
            "Line {} in file \"{}\" was expected to contain the license text \"{}\", but contains \"{}\" instead! \
            Consult the expected license text in \".resources/license_header\"",
            i + 1,
            path.display(),
            expected_line,
            file_line
        );
    }
}

fn check_file_licenses<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();
    let license_lines: Vec<_> = EXPECTED_LICENSE_TEXT.lines().collect();

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
            compare_license_text(entry.path(), &license_lines);
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
