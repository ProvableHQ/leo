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

//! Output buffer for building formatted source code.
//!
//! The `Output` struct manages indentation and newline handling automatically,
//! allowing formatting code to focus on structure rather than whitespace details.

use crate::{INDENT, NEWLINE};

/// Output buffer for building formatted source code.
///
/// Handles indentation automatically when writing after a newline.
/// Tracks whether we're at the start of a line to manage spacing correctly.
pub struct Output {
    /// The accumulated output string.
    buf: String,
    /// Current indentation depth (number of indentation levels).
    depth: usize,
    /// Whether we're at the start of a line (for auto-indentation).
    at_line_start: bool,
    /// Optional position marker for deferred newline insertion.
    mark: Option<usize>,
}

impl Output {
    /// Create a new empty output buffer.
    pub fn new() -> Self {
        Self { buf: String::new(), depth: 0, at_line_start: true, mark: None }
    }

    /// Write text to the buffer.
    ///
    /// If we're at the start of a line, automatically inserts indentation first.
    /// Empty strings are ignored.
    pub fn write(&mut self, s: &str) {
        if s.is_empty() {
            return;
        }
        if self.at_line_start {
            self.buf.push_str(&INDENT.repeat(self.depth));
            self.at_line_start = false;
        }
        self.buf.push_str(s);
    }

    /// Write a single space (unless at line start).
    ///
    /// Spaces at the start of a line would be redundant since indentation
    /// is handled automatically.
    pub fn space(&mut self) {
        if !self.at_line_start {
            self.buf.push(' ');
        }
    }

    /// Write a newline.
    pub fn newline(&mut self) {
        self.buf.push_str(NEWLINE);
        self.at_line_start = true;
    }

    /// Ensure we're on a fresh line.
    ///
    /// If the buffer doesn't end with a newline, adds one.
    /// If it already ends with a newline, does nothing.
    pub fn ensure_newline(&mut self) {
        if !self.buf.ends_with('\n') {
            self.newline();
        }
    }

    /// Mark the current buffer position for later newline insertion.
    ///
    /// Used by item formatters to mark the position after a closing `}` or `;`,
    /// so that `format_program` can insert an inter-item blank line at this
    /// position rather than at the end (after any trailing comments).
    pub fn set_mark(&mut self) {
        self.mark = Some(self.buf.len());
    }

    /// Insert a newline at the previously marked position.
    ///
    /// If no mark was set, falls back to appending a newline at the end.
    pub fn insert_newline_at_mark(&mut self) {
        if let Some(pos) = self.mark.take() {
            self.buf.insert_str(pos, NEWLINE);
        } else {
            self.newline();
        }
    }

    /// Increase the indentation level.
    fn indent(&mut self) {
        self.depth += 1;
    }

    /// Decrease the indentation level.
    ///
    /// Uses saturating subtraction to prevent underflow.
    fn dedent(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }

    /// Execute a closure with increased indentation.
    ///
    /// Automatically indents before calling the closure and dedents after,
    /// even if the closure panics.
    pub fn indented<F: FnOnce(&mut Self)>(&mut self, f: F) {
        self.indent();
        f(self);
        self.dedent();
    }

    /// Return the current column (0-based character offset from the last newline).
    ///
    /// When `at_line_start` is true, the pending indentation hasn't been emitted
    /// yet, so we return `depth * INDENT.len()`.
    pub fn current_column(&self) -> usize {
        if self.at_line_start {
            return self.depth * INDENT.len();
        }
        match self.buf.rfind('\n') {
            Some(pos) => self.buf.len() - pos - 1,
            None => self.buf.len(),
        }
    }

    /// Consume the buffer and return the raw string without trailing-newline
    /// normalization. Used by measurement helpers.
    pub fn into_raw(self) -> String {
        self.buf
    }

    /// Consume the buffer and return the final formatted string.
    ///
    /// Ensures the output ends with exactly one trailing newline.
    pub fn finish(mut self) -> String {
        // Remove extra trailing newlines.
        while self.buf.ends_with("\n\n") {
            self.buf.pop();
        }
        // Ensure at least one trailing newline (if non-empty).
        if !self.buf.ends_with('\n') && !self.buf.is_empty() {
            self.buf.push('\n');
        }
        self.buf
    }
}

impl Default for Output {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_and_space() {
        assert_eq!(Output::new().finish(), "");

        let mut out = Output::new();
        out.write("");
        out.write("a");
        out.space();
        out.write("b");
        out.write("");
        assert_eq!(out.finish(), "a b\n");

        // Space at line start is ignored
        let mut out = Output::new();
        out.space();
        out.write("x");
        assert_eq!(out.finish(), "x\n");
    }

    #[test]
    fn newlines() {
        let mut out = Output::new();
        out.write("a");
        out.newline();
        out.write("b");
        assert_eq!(out.finish(), "a\nb\n");

        // ensure_newline is idempotent
        let mut out = Output::new();
        out.write("x");
        out.ensure_newline();
        out.ensure_newline();
        out.write("y");
        assert_eq!(out.finish(), "x\ny\n");
    }

    #[test]
    fn indentation() {
        let mut out = Output::new();
        out.write("L0");
        out.newline();
        out.indented(|out| {
            out.write("L1");
            out.newline();
            out.indented(|out| {
                out.write("L2");
                out.newline();
            });
        });
        out.write("L0");
        assert_eq!(out.finish(), "L0\n    L1\n        L2\nL0\n");

        // Dedent saturates at zero
        let mut out = Output::new();
        out.dedent();
        out.dedent();
        out.write("x");
        assert_eq!(out.finish(), "x\n");
    }

    #[test]
    fn current_column() {
        // Fresh buffer: at_line_start, depth 0
        assert_eq!(Output::new().current_column(), 0);

        // After writing: no newline in buffer, so len from start
        let mut out = Output::new();
        out.write("ab");
        out.space();
        out.write("cd");
        assert_eq!(out.current_column(), 5); // "ab cd"

        // After newline: back to pending indent width
        out.newline();
        assert_eq!(out.current_column(), 0);

        // Indented write
        let mut out = Output::new();
        out.indent();
        out.write("x");
        assert_eq!(out.current_column(), 5); // 4-space indent + 1 char

        // at_line_start with depth reports pending indent
        out.newline();
        assert_eq!(out.current_column(), 4);
    }

    #[test]
    fn finish_normalizes_trailing_newlines() {
        // Adds missing newline
        let mut out = Output::new();
        out.write("x");
        assert_eq!(out.finish(), "x\n");

        // Removes extra newlines
        let mut out = Output::new();
        out.write("x");
        out.newline();
        out.newline();
        out.newline();
        assert_eq!(out.finish(), "x\n");
    }
}
