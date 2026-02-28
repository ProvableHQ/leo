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

        // Format the source and compare.
        let actual = format_source(&source);
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

        let output = format_source(&input);
        if input != output {
            print_diff(&input, &output, &target_path);
            failures.push(target_path.clone());
        }
    }

    assert!(failures.is_empty(), "{} file(s) not idempotent: {failures:?}", failures.len());
}

/// Compilation and AST validation tests, gated behind `--features validate`.
///
/// These tests use the real Leo compiler to verify that:
/// 1. Formatter test fixtures are semantically valid programs (not just
///    syntactically parseable).
/// 2. Formatting preserves AST equivalence for both local fixtures and
///    real-world Leo repositories (cloned into `target/test-repos/`).
///
/// They are feature-gated because they pull in heavy compiler dependencies
/// that slow down the default `cargo test` cycle.
///
/// Run with: `cargo test -p leo-fmt --features validate`
#[cfg(feature = "validate")]
mod validate {
    use super::*;

    use leo_ast::{NetworkName, NodeBuilder};
    use leo_compiler::Compiler;
    use leo_errors::Handler;
    use leo_span::{create_session_if_not_set_then, source_map::FileName};
    use std::rc::Rc;

    /// Files that are expected to fail type checking (error recovery tests,
    /// import tests, empty programs, annotated functions).
    const SKIP_VALIDATION: &[&str] = &[
        "error_recovery_item",
        "error_recovery_stmt",
        "error_recovery_struct",
        "import_single",
        "import_multiple",
        "empty_program",
        "function_annotated",
        "comment_before_program",
    ];

    /// Parse a Leo source string and return a normalized AST JSON value
    /// with spans and node IDs stripped, suitable for semantic comparison.
    ///
    /// Returns `None` if parsing fails (some source files have intentionally
    /// malformed whitespace that the compiler's parser rejects).
    fn source_to_ast_json(name: &str, source: &str) -> Option<serde_json::Value> {
        let (handler, _) = Handler::new_with_buf();
        let mut compiler = Compiler::new(
            None,
            false,
            handler,
            Rc::new(NodeBuilder::default()),
            "/tmp".into(),
            None,
            Default::default(),
            NetworkName::TestnetV0,
        );
        let program = compiler.parse_and_return_ast(source, FileName::Custom(name.into()), &[]).ok()?;
        let mut json = serde_json::to_value(&program).unwrap();
        for key in ["span", "_span", "id", "lo", "hi"] {
            json = leo_ast::remove_key_from_json(json, key);
        }
        Some(leo_ast::normalize_json_value(json))
    }

    /// Validate that target files pass Leo type checking.
    ///
    /// Runs parse + intermediate passes (name validation, type checking, static
    /// analysis) on each target file, skipping files in SKIP_VALIDATION.
    /// Stops before code generation. Catches issues like undeclared variables,
    /// type mismatches, and invalid syntax that the formatter's rowan parser
    /// wouldn't flag.
    ///
    /// Run with: `cargo test -p leo-fmt --features validate -- validate_targets_compile`
    #[test]
    fn validate_targets_compile() {
        let target_dir = tests_dir().join("target");
        let target_files = collect_leo_files(&target_dir);

        create_session_if_not_set_then(|_| {
            let mut failures = Vec::new();

            for target_path in &target_files {
                let name = target_path.file_stem().unwrap().to_str().unwrap();
                if SKIP_VALIDATION.contains(&name) {
                    continue;
                }

                let source = std::fs::read_to_string(target_path)
                    .unwrap_or_else(|_| panic!("Failed to read: {}", target_path.display()));

                let (handler, buf) = Handler::new_with_buf();
                let mut compiler = Compiler::new(
                    None,
                    false,
                    handler,
                    Rc::new(NodeBuilder::default()),
                    "/tmp".into(),
                    None,
                    Default::default(),
                    NetworkName::TestnetV0,
                );

                let result = compiler
                    .parse(&source, FileName::Custom(name.into()), &[])
                    .and_then(|_| compiler.intermediate_passes().map(|_| ()));

                if let Err(e) = result {
                    println!("\n=== TYPE CHECK ERROR: {} ===", target_path.display());
                    println!("{e}");
                    println!("{}", buf.extract_errs());
                    println!("=== END ===\n");
                    failures.push(target_path.clone());
                }
            }

            assert!(failures.is_empty(), "{} file(s) failed type checking: {failures:?}", failures.len());
        });
    }

    /// Validate that formatting preserves AST semantics.
    ///
    /// Parses each source file before and after formatting, strips spans and
    /// node IDs, and asserts the resulting ASTs are identical.
    ///
    /// Run with: `cargo test -p leo-fmt --features validate -- validate_ast_equivalence`
    #[test]
    fn validate_ast_equivalence() {
        let source_dir = tests_dir().join("source");
        let source_files = collect_leo_files(&source_dir);

        create_session_if_not_set_then(|_| {
            let mut failures = Vec::new();

            for source_path in &source_files {
                let name = source_path.file_stem().unwrap().to_str().unwrap();
                if SKIP_VALIDATION.contains(&name) {
                    continue;
                }

                let source = std::fs::read_to_string(source_path)
                    .unwrap_or_else(|_| panic!("Failed to read: {}", source_path.display()));

                // Skip files whose source can't be parsed by the compiler
                // (e.g. files with intentionally exaggerated whitespace in paths).
                let Some(before) = source_to_ast_json(name, &source) else {
                    continue;
                };
                let after = source_to_ast_json(name, &format_source(&source))
                    .unwrap_or_else(|| panic!("Formatted output of {name} failed to parse"));

                if before != after {
                    println!("\n=== AST MISMATCH: {} ===", source_path.display());
                    println!("ASTs differ after formatting (ignoring spans/IDs)");
                    println!("=== END ===\n");
                    failures.push(source_path.clone());
                }
            }

            assert!(failures.is_empty(), "{} file(s) have AST mismatches: {failures:?}", failures.len());
        });
    }

    /// External Leo repos to validate AST equivalence against.
    /// Each entry is (directory_name, git_url, pinned_rev).
    ///
    /// TODO: Replace "HEAD" with pinned commit SHAs once these repos are updated
    /// to 4.0 syntax. Using HEAD for now since they're still on 3.5.
    const EXTERNAL_REPOS: &[(&str, &str, &str)] = &[
        ("aleo-multisig", "https://github.com/AleoNet/aleo-multisig", "HEAD"),
        ("compliant-transfer-aleo", "https://github.com/sealance-io/compliant-transfer-aleo", "HEAD"),
        ("hyperlane-aleo", "https://github.com/hyperlane-xyz/hyperlane-aleo", "HEAD"),
        ("leo-examples", "https://github.com/ProvableHQ/leo-examples", "HEAD"),
    ];

    /// Directory where external repos are cached between test runs.
    fn repos_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..").join("target").join("test-repos")
    }

    /// Ensure a repo is cloned locally and checked out to a pinned revision.
    /// Returns the path to the repo directory.
    ///
    /// Uses shallow clones to minimize download size. If the repo is already
    /// cloned, uses the existing checkout without pulling (delete `target/test-repos/`
    /// to force a fresh clone).
    fn ensure_repo(name: &str, url: &str, rev: &str) -> PathBuf {
        let dir = repos_dir();
        std::fs::create_dir_all(&dir).expect("failed to create test-repos directory");
        let repo_dir = dir.join(name);
        if !repo_dir.join(".git").exists() {
            let status = std::process::Command::new("git")
                .args(["clone", "--depth", "1", url])
                .arg(&repo_dir)
                .status()
                .expect("failed to run git clone");
            assert!(status.success(), "git clone failed for {url}");
        }
        let checkout = std::process::Command::new("git")
            .arg("-C")
            .arg(&repo_dir)
            .args(["checkout", "--detach", rev])
            .status()
            .expect("failed to run git checkout");
        if !checkout.success() {
            let fetch = std::process::Command::new("git")
                .arg("-C")
                .arg(&repo_dir)
                .args(["fetch", "--depth", "1", "origin", rev])
                .status()
                .expect("failed to run git fetch");
            assert!(fetch.success(), "git fetch failed for {url} @ {rev}");
            let retry_checkout = std::process::Command::new("git")
                .arg("-C")
                .arg(&repo_dir)
                .args(["checkout", "--detach", rev])
                .status()
                .expect("failed to run git checkout");
            assert!(retry_checkout.success(), "git checkout failed for {url} @ {rev}");
        }
        repo_dir
    }

    /// Validate AST equivalence against real-world Leo repositories.
    ///
    /// Clones external repos (cached in `target/test-repos/`), formats every
    /// `.leo` file, and asserts the AST is unchanged. Files that can't be parsed
    /// by the compiler (e.g. due to unresolved imports) are skipped.
    ///
    /// Run with: `cargo test -p leo-fmt --features validate -- validate_ast_equivalence_repos`
    #[test]
    fn validate_ast_equivalence_repos() {
        create_session_if_not_set_then(|_| {
            let mut failures = Vec::new();
            let mut tested = 0;
            let mut skipped = 0;

            for (name, url, rev) in EXTERNAL_REPOS {
                let repo_dir = ensure_repo(name, url, rev);
                let leo_files = collect_leo_files(&repo_dir);
                println!("{name}: found {} .leo files", leo_files.len());

                for file_path in &leo_files {
                    let file_name = file_path.file_stem().unwrap().to_str().unwrap();
                    let source = std::fs::read_to_string(file_path)
                        .unwrap_or_else(|_| panic!("Failed to read: {}", file_path.display()));

                    // Skip files the compiler can't parse (e.g. unresolved imports).
                    let Some(before) = source_to_ast_json(file_name, &source) else {
                        skipped += 1;
                        continue;
                    };

                    let formatted = format_source(&source);
                    let Some(after) = source_to_ast_json(file_name, &formatted) else {
                        // Original parsed OK but formatted output didn't — formatter broke something.
                        println!("\n=== FORMAT BROKE PARSING: {} ===", file_path.display());
                        println!("Original parsed OK but formatted output failed to parse");
                        println!("=== END ===\n");
                        failures.push(file_path.clone());
                        continue;
                    };

                    if before != after {
                        println!("\n=== AST MISMATCH: {} ===", file_path.display());
                        println!("ASTs differ after formatting (ignoring spans/IDs)");
                        println!("=== END ===\n");
                        failures.push(file_path.clone());
                    }

                    tested += 1;
                }
            }

            println!("\nExternal repos: {tested} files tested, {skipped} skipped (unparseable)");
            assert!(failures.is_empty(), "{} file(s) have AST mismatches: {failures:?}", failures.len());
        });
    }
}
