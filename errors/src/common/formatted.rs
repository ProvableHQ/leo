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

use crate::{BacktracedError, Span, INDENT};

use std::fmt;

use backtrace::Backtrace;
use color_backtrace::{BacktracePrinter, Verbosity};
use colored::Colorize;

/// Formatted compiler error type
///     undefined value `x`
///     --> file.leo: 2:8
///      |
///    2 | let a = x;
///      |         ^
///      |
///      = help: Initialize a variable `x` first.
/// Makes use of the same fields as a BacktracedError.
#[derive(Clone, Debug, Default, Hash, PartialEq)]
pub struct FormattedError {
    /// The formatted error span information.
    pub span: Span,
    /// The backtrace to track where the Leo error originated.
    pub backtrace: BacktracedError,
}

impl FormattedError {
    /// Creates a backtraced error from a span and a backtrace.
    pub fn new_from_span<S>(
        message: S,
        help: Option<String>,
        exit_code: i32,
        code_identifier: i8,
        error_type: String,
        span: &Span,
        backtrace: Backtrace,
    ) -> Self
    where
        S: ToString,
    {
        Self {
            span: span.clone(),
            backtrace: BacktracedError::new_from_backtrace(
                message.to_string(),
                help,
                exit_code,
                code_identifier,
                error_type,
                backtrace,
            ),
        }
    }

    /// Calls the backtraces error code.
    pub fn exit_code(&self) -> i32 {
        self.backtrace.exit_code()
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

        let underlined = underline(self.span.col_start, self.span.col_stop);

        let error_message = format!(
            "Error: [E{error_type}{code_identifier:0>3}{exit_code:0>4}]: {message}",
            error_type = self.backtrace.error_type,
            code_identifier = self.backtrace.code_identifier,
            exit_code = self.backtrace.exit_code,
            message = self.backtrace.message,
        );

        // To avoid the color enabling characters for comparison with test expectations.
        if std::env::var("LEO_TESTFRAMEWORK")
            .unwrap_or_default()
            .trim()
            .to_owned()
            .is_empty()
        {
            write!(f, "{}", error_message.bold().red())?;
        } else {
            write!(f, "{}", error_message)?;
        };

        write!(
            f,
            "\n{indent     }--> {path}:{line_start}:{start}\n\
            {indent     } ",
            indent = INDENT,
            path = &*self.span.path,
            line_start = self.span.line_start,
            start = self.span.col_start,
        )?;

        for (line_no, line) in self.span.content.lines().enumerate() {
            writeln!(
                f,
                "|\n{line_no:width$} | {text}",
                width = INDENT.len(),
                line_no = self.span.line_start + line_no,
                text = line,
            )?;
        }

        write!(
            f,
            "{indent     } |{underlined}",
            indent = INDENT,
            underlined = underlined,
        )?;

        if let Some(help) = &self.backtrace.help {
            write!(
                f,
                "\n{indent     } |\n\
            {indent     } = {help}",
                indent = INDENT,
                help = help
            )?;
        }

        let leo_backtrace = std::env::var("LEO_BACKTRACE").unwrap_or_default().trim().to_owned();
        match leo_backtrace.as_ref() {
            "1" => {
                let mut printer = BacktracePrinter::default();
                printer = printer.verbosity(Verbosity::Medium);
                printer = printer.lib_verbosity(Verbosity::Medium);
                let trace = printer
                    .format_trace_to_string(&self.backtrace.backtrace)
                    .map_err(|_| fmt::Error)?;
                write!(f, "{}", trace)?;
            }
            "full" => {
                let mut printer = BacktracePrinter::default();
                printer = printer.verbosity(Verbosity::Full);
                printer = printer.lib_verbosity(Verbosity::Full);
                let trace = printer
                    .format_trace_to_string(&self.backtrace.backtrace)
                    .map_err(|_| fmt::Error)?;
                write!(f, "{}", trace)?;
            }
            _ => {}
        }

        Ok(())
    }
}

impl std::error::Error for FormattedError {
    fn description(&self) -> &str {
        &self.backtrace.message
    }
}
