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

use leo_fmt::{check_formatted, format_source};
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

#[test]
fn manual_cli_fmt_fixtures_remain_ugly_inputs_with_formatted_expectations() {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    let fixtures = [
        ("tests/tests/cli/test_fmt/contents/src/main.leo", "tests/expectations/cli/test_fmt/contents/src/main.leo"),
        (
            "tests/tests/cli/test_fmt/contents/ugly_lib/src/lib.leo",
            "tests/expectations/cli/test_fmt/contents/ugly_lib/src/lib.leo",
        ),
    ];

    for (source_rel, expected_rel) in fixtures {
        let source_path = workspace_root.join(source_rel);
        let expected_path = workspace_root.join(expected_rel);

        let source = std::fs::read_to_string(&source_path)
            .unwrap_or_else(|_| panic!("Failed to read CLI fmt source fixture: {}", source_path.display()));
        let expected = std::fs::read_to_string(&expected_path)
            .unwrap_or_else(|_| panic!("Failed to read CLI fmt expectation fixture: {}", expected_path.display()));

        assert!(
            !check_formatted(&source),
            "CLI fmt source fixture must remain intentionally ugly: {}",
            source_path.display()
        );
        assert!(
            check_formatted(&expected),
            "CLI fmt expectation fixture must remain formatted: {}",
            expected_path.display()
        );
        assert_eq!(
            format_source(&source),
            expected,
            "CLI fmt fixture pair drifted: {} -> {}",
            source_path.display(),
            expected_path.display()
        );
    }
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
    use leo_parser_rowan::{SyntaxElement, SyntaxNode, parse_file};
    use leo_span::{create_session_if_not_set_then, source_map::FileName};
    use std::{process::Command, rc::Rc};

    const ALEO_STUB_HEADER: &str = "// --- aleo stub --- //";
    const PROGRAM_DELIMITER: &str = "// --- Next Program --- //";

    const WORKSPACE_LEO_EXCLUDES: &[&str] = &[
        "crates/fmt/tests/*",
        "tests/tests/cli/test_fmt/*",
        "tests/expectations/cli/test_fmt/*",
        "tests/tests/parser-expression/*",
        "tests/tests/parser-module/*",
        "tests/tests/parser-statement/*",
        "tests/tests/parser-library/*",
    ];

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
        "malformed_missing_program_rbrace",
        "wrap_binary_chain",
    ];

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum ValidationMode {
        CompilerAst,
        RowanFallback,
    }

    #[derive(Debug, PartialEq, Eq)]
    enum ValidationSnapshot {
        CompilerAst(serde_json::Value),
        Rowan(RowanSnapshot),
    }

    #[derive(Debug, PartialEq, Eq)]
    struct RowanSnapshot {
        tree: Vec<String>,
        parse_errors: Vec<String>,
        lex_errors: Vec<String>,
    }

    /// Parse a Leo source string and return a normalized AST JSON value
    /// with spans and node IDs stripped, suitable for semantic comparison.
    fn try_source_to_ast_json(name: &str, source: &str) -> Result<serde_json::Value, String> {
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
        let program =
            compiler.parse_and_return_program(source, FileName::Custom(name.into()), &[]).map_err(|e| e.to_string())?;
        let mut json =
            serde_json::to_value(&program).expect("serializing the compiler AST to JSON should always succeed");
        for key in ["span", "_span", "id", "lo", "hi"] {
            json = leo_ast::remove_key_from_json(json, key);
        }
        Ok(leo_ast::normalize_json_value(json))
    }

    fn comparison_label(mode: ValidationMode) -> &'static str {
        match mode {
            ValidationMode::CompilerAst => "AST",
            ValidationMode::RowanFallback => "rowan parse tree",
        }
    }

    fn snapshot_for_source(name: &str, source: &str) -> (ValidationMode, ValidationSnapshot, Option<String>) {
        match try_source_to_ast_json(name, source) {
            Ok(ast) => (ValidationMode::CompilerAst, ValidationSnapshot::CompilerAst(ast), None),
            Err(error) => {
                (ValidationMode::RowanFallback, ValidationSnapshot::Rowan(rowan_snapshot(source)), Some(error))
            }
        }
    }

    fn snapshot_in_mode(mode: ValidationMode, name: &str, source: &str) -> Result<ValidationSnapshot, String> {
        match mode {
            ValidationMode::CompilerAst => try_source_to_ast_json(name, source).map(ValidationSnapshot::CompilerAst),
            ValidationMode::RowanFallback => Ok(ValidationSnapshot::Rowan(rowan_snapshot(source))),
        }
    }

    fn rowan_snapshot(source: &str) -> RowanSnapshot {
        let parse = parse_file(source);
        RowanSnapshot {
            tree: normalize_rowan_tree(&parse.syntax()),
            parse_errors: parse
                .errors()
                .iter()
                .map(|error| format!("{}|found={:?}|expected={:?}", error.message, error.found, error.expected))
                .collect(),
            lex_errors: parse.lex_errors().iter().map(|error| format!("{:?}", error.kind)).collect(),
        }
    }

    fn normalize_rowan_tree(node: &SyntaxNode) -> Vec<String> {
        fn visit(node: &SyntaxNode, out: &mut Vec<String>) {
            out.push(format!("ENTER:{:?}", node.kind()));
            for child in node.children_with_tokens() {
                match child {
                    SyntaxElement::Node(child) => visit(&child, out),
                    SyntaxElement::Token(token) if !token.kind().is_trivia() => {
                        out.push(format!("TOKEN:{:?}:{:?}", token.kind(), token.text()));
                    }
                    SyntaxElement::Token(_) => {}
                }
            }
            out.push(format!("EXIT:{:?}", node.kind()));
        }

        let mut out = Vec::new();
        visit(node, &mut out);
        out
    }

    /// Get the repository workspace root.
    fn workspace_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
    }

    /// Collect tracked workspace `.leo` files, excluding formatter fixtures,
    /// hand-authored CLI `fmt` test inputs/expectations, and parser fixture
    /// source directories whose formatting is validated separately.
    fn collect_workspace_leo_files() -> Vec<PathBuf> {
        let root = workspace_root();
        let mut args = vec!["ls-files".to_string(), "-z".to_string(), "--".to_string(), "*.leo".to_string()];
        args.extend(WORKSPACE_LEO_EXCLUDES.iter().map(|pattern| format!(":(exclude){pattern}")));
        let output = Command::new("git").arg("-C").arg(&root).args(&args).output().expect("failed to run git ls-files");
        assert!(output.status.success(), "git ls-files failed for {}", root.display());

        output
            .stdout
            .split(|byte| *byte == 0)
            .filter(|entry| !entry.is_empty())
            .map(|entry| {
                let path = std::str::from_utf8(entry).expect("git ls-files should return UTF-8 paths");
                root.join(path)
            })
            .collect()
    }

    #[test]
    fn collect_workspace_leo_files_excludes_manual_cli_and_parser_fixtures() {
        let leo_files = collect_workspace_leo_files();
        let excluded_roots = [
            workspace_root().join("crates/fmt/tests"),
            workspace_root().join("tests/tests/cli/test_fmt"),
            workspace_root().join("tests/expectations/cli/test_fmt"),
            workspace_root().join("tests/tests/parser-expression"),
            workspace_root().join("tests/tests/parser-module"),
            workspace_root().join("tests/tests/parser-statement"),
            workspace_root().join("tests/tests/parser-library"),
        ];

        for file_path in leo_files {
            assert!(
                excluded_roots.iter().all(|root| !file_path.starts_with(root)),
                "workspace formatter scope should exclude manual CLI/parser fixtures: {}",
                file_path.display()
            );
        }
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
                    .parse_program(&source, FileName::Custom(name.into()), &[])
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

                let (mode, before, _) = snapshot_for_source(name, &source);
                let label = comparison_label(mode);
                let after = snapshot_in_mode(mode, name, &format_source(&source))
                    .unwrap_or_else(|error| panic!("Formatted output of {name} failed {label} validation: {error}"));

                if before != after {
                    println!("\n=== AST MISMATCH: {} ===", source_path.display());
                    println!("{label} differs after formatting");
                    println!("=== END ===\n");
                    failures.push(source_path.clone());
                }
            }

            assert!(failures.is_empty(), "{} file(s) have AST mismatches: {failures:?}", failures.len());
        });
    }

    /// Validate AST equivalence across tracked workspace `.leo` files.
    ///
    /// Uses the same tracked-file scope as the CI `leo-fmt --check` step:
    /// all tracked `.leo` files excluding formatter fixtures and the manual
    /// CLI `fmt` test source/expectation trees.
    ///
    /// Files that cannot be lowered into the compiler AST fall back to a
    /// normalized rowan parse-tree comparison, which still validates the exact
    /// parser structure the formatter sees.
    ///
    /// Run with: `cargo test -p leo-fmt --features validate -- validate_workspace_ast_equivalence`
    #[test]
    fn validate_workspace_ast_equivalence() {
        let leo_files = collect_workspace_leo_files();

        create_session_if_not_set_then(|_| {
            let mut failures = Vec::new();
            let mut compiler_ast_tested = 0;
            let mut rowan_tested = 0;

            for file_path in &leo_files {
                let name = file_path.file_stem().unwrap().to_str().unwrap();
                let source = std::fs::read_to_string(file_path)
                    .unwrap_or_else(|_| panic!("Failed to read: {}", file_path.display()));

                // Multi-section compiler test files can embed raw Aleo bytecode stubs
                // (lines between `ALEO_STUB_HEADER` and the next `PROGRAM_DELIMITER`).
                // Strip those lines so that only Leo source is passed to the formatter.
                let leo_source: String = if source.contains(ALEO_STUB_HEADER) {
                    let mut filtered = String::with_capacity(source.len());
                    let mut in_aleo = false;
                    for line in source.lines() {
                        if line.trim() == ALEO_STUB_HEADER {
                            in_aleo = true;
                        } else if in_aleo && line.trim() == PROGRAM_DELIMITER {
                            in_aleo = false;
                            filtered.push_str(line);
                            filtered.push('\n');
                        } else if !in_aleo {
                            filtered.push_str(line);
                            filtered.push('\n');
                        }
                    }
                    filtered
                } else {
                    source
                };

                let (mode, before, ast_error) = snapshot_for_source(name, &leo_source);
                let label = comparison_label(mode);

                match mode {
                    ValidationMode::CompilerAst => compiler_ast_tested += 1,
                    ValidationMode::RowanFallback => {
                        rowan_tested += 1;
                        if std::env::var_os("LEO_FMT_PRINT_FALLBACKS").is_some() {
                            println!("\n=== ROWAN FALLBACK: {} ===", file_path.display());
                            println!(
                                "{}",
                                ast_error.expect("rowan fallback should always carry the compiler AST error"),
                            );
                            println!("=== END ===\n");
                        }
                    }
                }

                let formatted = format_source(&leo_source);
                let Ok(after) = snapshot_in_mode(mode, name, &formatted) else {
                    println!("\n=== FORMAT BROKE PARSING: {} ===", file_path.display());
                    println!("Original source passed {label} validation but formatted output did not");
                    println!("=== END ===\n");
                    failures.push(file_path.clone());
                    continue;
                };

                if before != after {
                    println!("\n=== AST MISMATCH: {} ===", file_path.display());
                    println!("{label} differs after formatting");
                    println!("=== END ===\n");
                    failures.push(file_path.clone());
                }
            }

            println!(
                "\nWorkspace files: {} tested ({} compiler AST, {} rowan fallback)",
                leo_files.len(),
                compiler_ast_tested,
                rowan_tested
            );
            assert_eq!(
                compiler_ast_tested + rowan_tested,
                leo_files.len(),
                "every tracked workspace .leo file should use either compiler AST or rowan fallback validation"
            );
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
    /// `.leo` file, and asserts the compiler AST is unchanged when available.
    /// Files the compiler AST cannot represent fall back to normalized rowan
    /// parse-tree comparison instead of being skipped.
    ///
    /// Run with: `cargo test -p leo-fmt --features validate -- validate_ast_equivalence_repos`
    #[test]
    fn validate_ast_equivalence_repos() {
        create_session_if_not_set_then(|_| {
            let mut failures = Vec::new();
            let mut compiler_ast_tested = 0;
            let mut rowan_tested = 0;

            for (name, url, rev) in EXTERNAL_REPOS {
                let repo_dir = ensure_repo(name, url, rev);
                let leo_files = collect_leo_files(&repo_dir);
                println!("{name}: found {} .leo files", leo_files.len());

                for file_path in &leo_files {
                    let file_name = file_path.file_stem().unwrap().to_str().unwrap();
                    let source = std::fs::read_to_string(file_path)
                        .unwrap_or_else(|_| panic!("Failed to read: {}", file_path.display()));
                    let (mode, before, _) = snapshot_for_source(file_name, &source);
                    let label = comparison_label(mode);

                    match mode {
                        ValidationMode::CompilerAst => compiler_ast_tested += 1,
                        ValidationMode::RowanFallback => rowan_tested += 1,
                    }

                    let formatted = format_source(&source);
                    let Ok(after) = snapshot_in_mode(mode, file_name, &formatted) else {
                        println!("\n=== FORMAT BROKE PARSING: {} ===", file_path.display());
                        println!("Original source passed {label} validation but formatted output did not");
                        println!("=== END ===\n");
                        failures.push(file_path.clone());
                        continue;
                    };

                    if before != after {
                        println!("\n=== AST MISMATCH: {} ===", file_path.display());
                        println!("{label} differs after formatting");
                        println!("=== END ===\n");
                        failures.push(file_path.clone());
                    }
                }
            }

            println!(
                "\nExternal repos: {} files tested ({} compiler AST, {} rowan fallback)",
                compiler_ast_tested + rowan_tested,
                compiler_ast_tested,
                rowan_tested
            );
            assert!(failures.is_empty(), "{} file(s) have AST mismatches: {failures:?}", failures.len());
        });
    }
}
