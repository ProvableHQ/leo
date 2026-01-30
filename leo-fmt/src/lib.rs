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
//! The formatter operates on the lossless syntax tree from `leo-parser-lossless` and
//! produces consistently formatted code.
//!
//! # Example
//!
//! ```ignore
//! use leo_fmt::format_source;
//!
//! let source = "program test.aleo{transition main()->u64{return 1u64;}}";
//! let formatted = format_source(source)?;
//! ```

mod output;

use anyhow::{Context, Result};
use leo_errors::Handler;
use leo_parser_lossless::parse_main;

/// Indentation string: 4 spaces.
pub const INDENT: &str = "    ";

/// Newline character.
pub const NEWLINE: &str = "\n";

/// Format Leo source code.
///
/// Takes Leo source code as input and returns formatted source code.
/// Returns an error if the source code cannot be parsed.
///
/// # Guarantees
///
/// - **Idempotent**: `format_source(format_source(x)?) == format_source(x)?`
/// - **Deterministic**: Same input always produces same output
/// - **Parse-safe**: Formatted output always parses successfully
/// - **Comment-preserving**: All comments are retained
pub fn format_source(source: &str) -> Result<String> {
    let handler = Handler::default();

    // Verify the source parses successfully.
    let _tree = parse_main(handler, source, 0).context("parse failed")?;

    // TODO: Implement actual formatting in follow up PRs.
    // For now, return the input unchanged (stub implementation).
    let mut result = source.trim_end().to_string();
    if !result.is_empty() {
        result.push('\n');
    }

    Ok(result)
}

/// Check if source code is already formatted.
///
/// Returns `true` if the source code matches what the formatter would produce,
/// `false` otherwise. Returns an error if the source code cannot be parsed.
pub fn check_formatted(source: &str) -> Result<bool> {
    let formatted = format_source(source)?;
    Ok(source == formatted)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = "program test.aleo {}\n";

    #[test]
    fn valid_code_ok() {
        assert!(format_source(VALID).is_ok());
    }

    #[test]
    fn invalid_code_err() {
        assert!(format_source("not valid leo").is_err());
    }

    #[test]
    fn normalizes_trailing_newline() {
        // Adds missing newline
        assert!(format_source("program test.aleo {}").unwrap().ends_with('\n'));
        // Removes extra newlines
        assert!(format_source("program test.aleo {}\n\n\n").unwrap().ends_with("}\n"));
    }
}
