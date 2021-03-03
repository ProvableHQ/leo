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

use std::fmt;

pub const INDENT: &str = "    ";

/// Formatted compiler error type
///     --> file.leo 2:8
///      |
///    2 | let a = x;
///      |         ^
///      |
///      = undefined value `x`
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FormattedError {
    /// File path where error occurred
    pub path: Option<String>,
    /// Line start number
    pub line_start: usize,
    /// Line end number
    pub line_stop: usize,
    /// Starting column
    pub start: usize,
    /// Ending column
    pub end: usize,
    /// Text of errored lines
    pub text: Option<Vec<String>>,
    /// Error explanation
    pub message: String,
}

impl FormattedError {
    pub fn new_from_span(message: String, span: &Span) -> Self {
        Self {
            path: None,
            line_start: span.line_start,
            line_stop: span.line_stop,
            start: span.col_start,
            end: span.col_stop,
            text: None,
            message,
        }
    }
}

impl LeoError for FormattedError {
    fn set_path(&mut self, path: &str, content: &[String]) {
        self.path = Some(path.to_string());
        if self.line_stop - 1 > content.len() {
            self.text = Some(vec!["corrupt file".to_string()]);
            return;
        }
        assert!(self.line_stop >= self.line_start);
        // if self.line_stop == self.line_start {
        //     self.text = Some(vec![content[self.line_start - 1][self.start - 1..self.end - 1].to_string()]);
        // } else {
        self.text = Some(
            content[self.line_start - 1..self.line_stop]
                .iter()
                .map(|x| x.to_string())
                .collect(),
        );
        // }
    }

    fn get_path(&self) -> Option<&str> {
        self.path.as_deref()
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

impl fmt::Display for FormattedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let path = self.path.as_ref().map(|path| format!("{}:", path)).unwrap_or_default();
        let underline = underline(self.start - 1, self.end - 1);

        write!(
            f,
            "{indent     }--> {path}{line_start}:{start}\n\
             {indent     } |\n",
            indent = INDENT,
            path = path,
            line_start = self.line_start,
            start = self.start,
        )?;

        if let Some(lines) = &self.text {
            for (line_no, line) in lines.iter().enumerate() {
                writeln!(
                    f,
                    "{line_no:width$} | {text}",
                    width = INDENT.len(),
                    line_no = self.line_start + line_no,
                    text = line,
                )?;
            }
        }

        write!(
            f,
            "{indent     } | {underline}\n\
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
        path: Some("file.leo".to_string()),
        line_start: 2,
        line_stop: 2,
        start: 8,
        end: 9,
        text: Some(vec!["let a = x;".to_string()]),
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
