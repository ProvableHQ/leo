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

//! Leo source code formatter.
//!
//! This crate provides an opinionated, zero-configuration formatter for Leo source code.
//! The formatter operates on the lossless syntax tree from `leo-parser-rowan` and
//! produces consistently formatted code.
//!
//! # Example
//!
//! ```ignore
//! use leo_fmt::format_source;
//!
//! let source = "program test.aleo{fn main()->u64{return 1u64;}}";
//! let formatted = format_source(source);
//! ```

mod format;
mod output;

use clap::{Args, Parser};
use colored::Colorize;
use leo_parser_rowan::parse_main;
use similar::{ChangeTag, TextDiff};
use std::{
    fs,
    io,
    io::ErrorKind,
    path::{Path, PathBuf},
    process::ExitCode,
};

use output::Output;

/// Indentation string: 4 spaces.
pub const INDENT: &str = "    ";

/// Newline character.
pub const NEWLINE: &str = "\n";

/// Maximum line width before wrapping.
pub const LINE_WIDTH: usize = 100;

/// Shared CLI arguments for formatting commands.
///
/// Used by both the standalone `leo-fmt` binary and `leo fmt` command.
#[derive(Debug, Clone, Args)]
pub struct FormatCliArgs {
    /// Check if files are formatted without modifying them.
    /// Returns exit code 1 if any files need formatting.
    #[clap(long, short)]
    pub check: bool,

    /// Files or directories to format.
    /// Defaults to current directory if not specified.
    #[clap(value_name = "PATH")]
    pub paths: Vec<PathBuf>,
}

/// Standalone `leo-fmt` CLI parser.
#[derive(Debug, Parser)]
#[command(name = "leo-fmt", about = "Format Leo source files", long_about = None)]
pub struct LeoFmtCli {
    #[command(flatten)]
    pub format: FormatCliArgs,
}

/// Run filesystem formatting/checking for CLI callers.
pub fn run_format_cli(args: &FormatCliArgs, base_dir: &Path) -> io::Result<bool> {
    let paths = resolve_paths(&args.paths, base_dir);
    let leo_files = collect_leo_files(&paths)?;
    let mut has_unformatted = false;

    for file_path in leo_files {
        let source = fs::read_to_string(&file_path).map_err(|error| {
            io::Error::new(error.kind(), format!("failed to read {}: {error}", file_path.display()))
        })?;
        let formatted = format_source(&source);

        if source == formatted {
            continue;
        }

        if args.check {
            print_diff(&file_path, &source, &formatted);
            has_unformatted = true;
        } else {
            fs::write(&file_path, formatted).map_err(|error| {
                io::Error::new(error.kind(), format!("failed to write {}: {error}", file_path.display()))
            })?;
        }
    }

    Ok(has_unformatted)
}

/// Standalone `leo-fmt` entrypoint logic.
pub fn run_standalone() -> ExitCode {
    let args = LeoFmtCli::parse();
    let base_dir = match std::env::current_dir() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("failed to resolve current directory: {error}");
            return ExitCode::from(1);
        }
    };

    match run_format_cli(&args.format, &base_dir) {
        Ok(has_unformatted) if args.format.check && has_unformatted => ExitCode::from(1),
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(1)
        }
    }
}

/// Determine paths to format.
/// Default to project src directory if in a Leo project, otherwise current directory.
fn resolve_paths(input_paths: &[PathBuf], base_dir: &Path) -> Vec<PathBuf> {
    if !input_paths.is_empty() {
        return input_paths.to_vec();
    }

    let src_dir = base_dir.join("src");
    if src_dir.exists() { vec![src_dir] } else { vec![base_dir.to_path_buf()] }
}

/// Collect all .leo files.
fn collect_leo_files(paths: &[PathBuf]) -> io::Result<Vec<PathBuf>> {
    let mut leo_files = Vec::new();

    for path in paths {
        if !path.exists() {
            return Err(io::Error::new(ErrorKind::InvalidInput, format!("Path not found: {}", path.display())));
        }

        if path.is_file() {
            if path.extension().and_then(|s| s.to_str()) == Some("leo") {
                leo_files.push(path.clone());
            } else {
                return Err(io::Error::new(
                    ErrorKind::InvalidInput,
                    format!("Expected a .leo file, got: {}", path.display()),
                ));
            }
        } else if path.is_dir() {
            for entry in walkdir::WalkDir::new(path)
                .into_iter()
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.file_type().is_file())
                .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("leo"))
            {
                leo_files.push(entry.path().to_path_buf());
            }
        }
    }

    Ok(leo_files)
}

/// Number of unchanged lines shown around each diff hunk for context.
const CONTEXT_LINES: usize = 3;

/// Print a colored diff between original and formatted source, matching `cargo fmt --check` output.
fn print_diff(path: &Path, original: &str, formatted: &str) {
    let diff = TextDiff::from_lines(original, formatted);

    for hunk in diff.unified_diff().context_radius(CONTEXT_LINES).iter_hunks() {
        // Extract the old-file start line from the hunk header (e.g. "@@ -10,5 +10,7 @@").
        let header = hunk.header().to_string();
        let line_num = header
            .strip_prefix("@@ -")
            .and_then(|s| s.split(',').next())
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1);
        println!("Diff in {}:{}:", path.display(), line_num);
        for change in hunk.iter_changes() {
            match change.tag() {
                ChangeTag::Equal => print!(" {change}"),
                ChangeTag::Delete => print!("{}", format!("-{change}").red()),
                ChangeTag::Insert => print!("{}", format!("+{change}").green()),
            }
            if change.missing_newline() {
                println!();
            }
        }
    }
}

/// Format Leo source code.
///
/// Takes Leo source code as input and returns formatted source code.
///
/// # Guarantees
///
/// - **Idempotent**: `format_source(format_source(x)) == format_source(x)`
/// - **Deterministic**: Same input always produces same output
/// - **Comment-preserving**: All comments are retained
pub fn format_source(source: &str) -> String {
    let tree = parse_main(source).expect("rowan parser should never fail");

    let mut out = Output::new();
    format::format_node(&tree, &mut out);
    out.finish()
}

/// Check if source code is already formatted.
///
/// Returns `true` if the source code matches what the formatter would produce.
pub fn check_formatted(source: &str) -> bool {
    source == format_source(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = "program test.aleo {}\n";

    #[test]
    fn valid_code_ok() {
        assert_eq!(format_source(VALID), VALID);
    }

    #[test]
    fn normalizes_trailing_newline() {
        // Adds missing newline
        assert!(format_source("program test.aleo {}").ends_with('\n'));
        // Removes extra newlines
        assert!(format_source("program test.aleo {}\n\n\n").ends_with("}\n"));
    }
}
