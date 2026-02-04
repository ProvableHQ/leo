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

//! Test harness for the Leo formatter.
//!
//! Uses source/target file pairs (rustfmt-style) to test formatting:
//! - `tests/source/*.leo` — unformatted input files
//! - `tests/target/*.leo` — expected formatted output files
//!
//! Tests verify:
//! 1. Source files format to match target files
//! 2. Target files are idempotent (format to themselves)
//! 3. Target files parse successfully

use leo_errors::Handler;
use leo_fmt::format_source;
use similar::{ChangeTag, TextDiff};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Get the path to the tests directory.
fn tests_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests")
}

/// Collect all .leo files in a directory.
fn collect_leo_files(dir: &Path) -> Vec<PathBuf> {
    if !dir.exists() {
        return Vec::new();
    }

    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|x| x == "leo"))
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// Print a unified diff between expected and actual content.
fn print_diff(expected: &str, actual: &str, file: &Path) {
    let diff = TextDiff::from_lines(expected, actual);
    println!("\n=== MISMATCH: {} ===", file.display());
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        print!("{sign}{change}");
    }
    println!("=== END ===\n");
}

/// Test that formatting source files produces the expected target files.
///
/// For each file in `tests/source/`, formats it and compares the result
/// to the corresponding file in `tests/target/`.
#[test]
fn test_source_to_target() {
    let source_dir = tests_dir().join("source");
    let source_files = collect_leo_files(&source_dir);

    let mut failures = Vec::new();

    for source_path in source_files {
        // Compute the corresponding target path.
        let relative = source_path.strip_prefix(&source_dir).unwrap();
        let target_path = tests_dir().join("target").join(relative);

        // Read source (unformatted input).
        let source = std::fs::read_to_string(&source_path)
            .unwrap_or_else(|_| panic!("Failed to read source: {}", source_path.display()));

        // Read target (expected output).
        let expected = std::fs::read_to_string(&target_path)
            .unwrap_or_else(|_| panic!("Missing target file: {}", target_path.display()));

        // Format the source.
        let actual = match format_source(&source) {
            Ok(formatted) => formatted,
            Err(e) => {
                println!("Format failed for {}: {e}", source_path.display());
                failures.push(source_path.clone());
                continue;
            }
        };

        // Compare.
        if actual != expected {
            print_diff(&expected, &actual, &source_path);
            failures.push(source_path.clone());
        }
    }

    assert!(failures.is_empty(), "{} test(s) failed: {failures:?}", failures.len());
}

/// Test that formatted files are idempotent.
///
/// Formatting a target file should produce identical output.
/// This ensures the formatter's output is stable.
#[test]
fn test_idempotency() {
    let target_dir = tests_dir().join("target");
    let target_files = collect_leo_files(&target_dir);

    let mut failures = Vec::new();

    for target_path in target_files {
        let input = std::fs::read_to_string(&target_path)
            .unwrap_or_else(|_| panic!("Failed to read: {}", target_path.display()));

        let output = match format_source(&input) {
            Ok(formatted) => formatted,
            Err(e) => {
                println!("Format failed for {}: {e}", target_path.display());
                failures.push(target_path.clone());
                continue;
            }
        };

        if input != output {
            print_diff(&input, &output, &target_path);
            failures.push(target_path.clone());
        }
    }

    assert!(failures.is_empty(), "{} file(s) not idempotent: {failures:?}", failures.len());
}

/// Test that all target files parse successfully.
///
/// This ensures the formatter never produces invalid Leo code.
#[test]
fn test_parse_safety() {
    let target_dir = tests_dir().join("target");
    let target_files = collect_leo_files(&target_dir);

    let mut failures = Vec::new();

    for target_path in target_files {
        let content = std::fs::read_to_string(&target_path)
            .unwrap_or_else(|_| panic!("Failed to read: {}", target_path.display()));

        let handler = Handler::default();
        if leo_parser_lossless::parse_main(handler, &content, 0).is_err() {
            println!("Parse failed for target file: {}", target_path.display());
            failures.push(target_path.clone());
        }
    }

    assert!(failures.is_empty(), "{} target file(s) don't parse: {failures:?}", failures.len());
}
