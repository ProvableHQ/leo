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

use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
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

    // Watch the directories that contain Rust source files
    println!("cargo:rerun-if-changed=compiler");
    println!("cargo:rerun-if-changed=errors");
    println!("cargo:rerun-if-changed=leo");
    println!("cargo:rerun-if-changed=test-framework");
    println!("cargo:rerun-if-changed=utils");

    // Watch the build script itself
    println!("cargo:rerun-if-changed=build.rs");

    // Watch the license header file
    println!("cargo:rerun-if-changed=.resources/license_header");

    // Watch the Cargo.toml file
    println!("cargo:rerun-if-changed=Cargo.toml");

    // Watch the Cargo.lock file
    println!("cargo:rerun-if-changed=Cargo.lock");
}

/// Generates one Rust `#[test]` per CLI integration test directory.
///
/// Scans `tests/tests/cli` and emits corresponding test functions that invoke
/// `run_single_cli_test` for each case. The generated tests are written to
/// `$OUT_DIR/cli_tests.rs` and included at compile time via `include!`.
///
/// This enables fine-grained test filtering, clearer failure reporting, and
/// per-test execution for CLI integration tests.
fn generate_cli_tests() {
    println!("cargo::rerun-if-changed=tests/tests/cli");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let tests_dir = manifest_dir.join("tests/tests/cli");

    let mut out = String::new();

    for entry in fs::read_dir(&tests_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let name = path.file_name().unwrap().to_string_lossy();
        let fn_name = format!("cli_{}", name.replace('-', "_"));

        out.push_str(&format!(
            r#"
#[test]
fn {fn_name}() {{
    crate::run_single_cli_test(
        std::path::Path::new(r"{path}")
    );
}}
"#,
            fn_name = fn_name,
            path = path.display(),
        ));
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::write(out_dir.join("cli_tests.rs"), out).unwrap();
}

// The build script; it currently:
// 1. Auto-generate e2e CLI tests as individual Rust unit tests (i.e. `[test]`).
// 2. Checks the licenses.
fn main() {
    generate_cli_tests();

    // Check licenses in the current folder.
    check_file_licenses(".");
}
