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

use std::{fmt, sync::Arc};

use backtrace::Backtrace;
use derivative::Derivative;

pub const INDENT: &str = "    ";

/// Formatted compiler error type
///     undefined value `x`
///     --> file.leo: 2:8
///      |
///    2 | let a = x;
///      |         ^
///      |
///      = help: Initialize a variable `x` first.
#[derive(Derivative)]
#[derivative(Clone, Debug, Default, Hash, PartialEq)]
pub struct FormattedError {
    pub line_start: usize,
    pub line_stop: usize,
    pub col_start: usize,
    pub col_stop: usize,
    pub path: Arc<String>,
    pub content: String,
    pub message: String,
    pub help: Option<String>,
    pub exit_code: u32,
    pub code_identifier: String,
    pub error_type: String,
    #[derivative(PartialEq = "ignore")]
    #[derivative(Hash = "ignore")]
    pub backtrace: Backtrace,
}

impl FormattedError {
    pub fn new_from_span(
        message: String,
        help: Option<String>,
        exit_code: u32,
        code_identifier: String,
        error_type: String,
        span: &Span,
    ) -> Self {
        Self {
            line_start: span.line_start,
            line_stop: span.line_stop,
            col_start: span.col_start,
            col_stop: span.col_stop,
            path: span.path.clone(),
            content: span.content.to_string(),
            message,
            help,
            exit_code,
            code_identifier,
            error_type,
            backtrace: Backtrace::new(),
        }
    }
}

impl fmt::Display for FormattedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let underline = |mut start: usize, mut end: usize| -> String {
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
        };

        let underlined = underline(self.col_start, self.col_start);

        let error_message = format!(
            "{indent     }[E{error_type}{code_identifier}{exit_code}]: {message}\
             {indent     }--> {path}:{line_start}:{start}\n\
	  {indent     } ",
            indent = INDENT,
            error_type = self.error_type,
            code_identifier = self.code_identifier,
            exit_code = self.exit_code,
            message = self.message,
            path = &*self.path,
            line_start = self.line_start,
            start = self.col_start,
        );

        write!(f, "{}", error_message)?;

        for (line_no, line) in self.content.lines().enumerate() {
            writeln!(
                f,
                "|\n{line_no:width$} | {text}",
                width = INDENT.len(),
                line_no = self.line_start + line_no,
                text = line,
            )?;
        }

        write!(
            f,
            "{indent     } |{underlined}\n\
             {indent     } |\n",
            indent = INDENT,
            underlined = underlined,
        )?;

        if let Some(help) = &self.help {
            write!(f, "{indent     } = {help}", indent = INDENT, help = help)?;
        }

        write!(f, "{:?}", self.backtrace,)
    }
}

impl std::error::Error for FormattedError {
    fn description(&self) -> &str {
        &self.message
    }
}
