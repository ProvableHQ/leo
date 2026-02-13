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

use super::*;

use colored::Colorize;
use similar::{ChangeTag, TextDiff};
use std::path::{Path, PathBuf};

/// Format Leo source files.
#[derive(Parser, Debug)]
pub struct LeoFormat {
    /// Check if files are formatted without modifying them.
    /// Returns exit code 1 if any files need formatting.
    #[clap(long, short)]
    check: bool,

    /// Files or directories to format.
    /// Defaults to current directory if not specified.
    #[clap(value_name = "PATH")]
    paths: Vec<PathBuf>,
}

impl Command for LeoFormat {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        // Determine paths to format.
        let paths: Vec<PathBuf> = if self.paths.is_empty() {
            // Default to project src directory if in a Leo project, otherwise current directory.
            let base_dir = context.dir()?;
            let src_dir = base_dir.join("src");
            if src_dir.exists() { vec![src_dir] } else { vec![base_dir] }
        } else {
            self.paths
        };

        // Collect all .leo files.
        let mut leo_files: Vec<PathBuf> = Vec::new();
        for path in &paths {
            if !path.exists() {
                return Err(CliError::cli_invalid_input(format!("Path not found: {}", path.display())).into());
            }

            if path.is_file() {
                if path.extension().and_then(|s| s.to_str()) == Some("leo") {
                    leo_files.push(path.clone());
                } else {
                    return Err(
                        CliError::cli_invalid_input(format!("Expected a .leo file, got: {}", path.display())).into()
                    );
                }
            } else if path.is_dir() {
                for entry in walkdir::WalkDir::new(path)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.file_type().is_file())
                    .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("leo"))
                {
                    leo_files.push(entry.path().to_path_buf());
                }
            }
        }

        let mut has_diff = false;

        for file_path in &leo_files {
            let source = std::fs::read_to_string(file_path).map_err(CliError::cli_io_error)?;
            let formatted = leo_fmt::format_source(&source);

            if source == formatted {
                continue;
            }

            if self.check {
                print_diff(file_path, &source, &formatted);
                has_diff = true;
            } else {
                std::fs::write(file_path, formatted).map_err(CliError::cli_io_error)?;
            }
        }

        if has_diff {
            std::process::exit(1);
        }
        Ok(())
    }
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
