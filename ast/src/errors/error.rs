// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::Span;

use std::fmt;
use std::path::Path;

pub const INDENT: &str = "    ";

/// Formatted compiler error type
///     --> file.leo 2:8
///      |
///    2 | let a = x;
///      |         ^
///      |
///      = undefined value `x`
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Error {
    /// File path where error occurred
    pub path: Option<String>,
    /// Line number
    pub line: usize,
    /// Starting column
    pub start: usize,
    /// Ending column
    pub end: usize,
    /// Text of errored line
    pub text: String,
    /// Error explanation
    pub message: String,
}

impl Error {
    pub fn new_from_span(message: String, span: Span) -> Self {
        Self {
            path: None,
            line: span.line,
            start: span.start,
            end: span.end,
            text: span.text,
            message,
        }
    }

    pub fn new_from_span_with_path(message: String, span: Span, path: &Path) -> Self {
        Self {
            path: Some(format!("{:?}", path)),
            line: span.line,
            start: span.start,
            end: span.end,
            text: span.text,
            message,
        }
    }

    pub fn set_path(&mut self, path: &Path) {
        self.path = Some(format!("{:?}", path));
    }

    pub fn format(&self) -> String {
        let path = self.path.as_ref().map(|path| format!("{}:", path)).unwrap_or_default();
        let underline = underline(self.start, self.end);

        format!(
            "{indent     }--> {path} {line}:{start}\n\
             {indent     } |\n\
             {line:width$} | {text}\n\
             {indent     } | {underline}\n\
             {indent     } |\n\
             {indent     } = {message}",
            indent = INDENT,
            width = INDENT.len(),
            path = path,
            line = self.line,
            start = self.start,
            text = self.text,
            underline = underline,
            message = self.message,
        )
    }
}

fn underline(mut start: usize, mut end: usize) -> String {
    if start > end {
        std::mem::swap(&mut start, &mut end)
    }

    let mut underline = String::new();

    for _ in 0..start {
        underline.push(' ');
        end -= 1;
    }

    for _ in 0..end {
        underline.push('^');
    }

    underline
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

#[test]
fn test_error() {
    let err = Error {
        path: Some("file.leo".to_string()),
        line: 2,
        start: 8,
        end: 9,
        text: "let a = x;".to_string(),
        message: "undefined value `x`".to_string(),
    };

    assert_eq!(
        err.to_string(),
        vec![
            "    --> file.leo: 2:8",
            "     |",
            "   2 | let a = x;",
            "     |         ^",
            "     |",
            "     = undefined value `x`",
        ]
        .join("\n")
    );
}
