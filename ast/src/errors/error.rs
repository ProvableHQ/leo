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

use crate::{LeoError, Span};

use std::{fmt, sync::Arc};

pub const INDENT: &str = "    ";

/// Formatted compiler error type
///     --> file.leo: 2:8
///      |
///    2 | let a = x;
///      |         ^
///      |
///      = undefined value `x`
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FormattedError {
    pub line_start: usize,
    pub line_stop: usize,
    pub col_start: usize,
    pub col_stop: usize,
    pub path: Arc<String>,
    pub content: String,
    pub message: String,
}

impl FormattedError {
    pub fn new_from_span(message: String, span: &Span) -> Self {
        Self {
            line_start: span.line_start,
            line_stop: span.line_stop,
            col_start: span.col_start,
            col_stop: span.col_stop,
            path: span.path.clone(),
            content: span.content.to_string(),
            message,
        }
    }
}

impl LeoError for FormattedError {}

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

impl fmt::Display for FormattedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let underline = underline(self.col_start - 1, self.col_stop - 1);

        write!(
            f,
            "{indent     }--> {path}: {line_start}:{start}\n\
             {indent     } |\n",
            indent = INDENT,
            path = &*self.path,
            line_start = self.line_start,
            start = self.col_start,
        )?;

        for (line_no, line) in self.content.lines().enumerate() {
            writeln!(
                f,
                "{line_no:width$} | {text}",
                width = INDENT.len(),
                line_no = self.line_start + line_no,
                text = line,
            )?;
        }

        write!(
            f,
            "{indent     } |  {underline}\n\
             {indent     } |\n\
             {indent     } = {message}",
            indent = INDENT,
            underline = underline,
            message = self.message,
        )
    }
}

impl std::error::Error for FormattedError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[test]
fn test_error() {
    let err = FormattedError {
        path: std::sync::Arc::new("file.leo".to_string()),
        line_start: 2,
        line_stop: 2,
        col_start: 8,
        col_stop: 9,
        content: "let a = x;".into(),
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
