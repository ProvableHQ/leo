// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::{Backtraced, INDENT};

use leo_span::{source_map::SpanLocation, symbol::with_session_globals, Span};

use backtrace::Backtrace;
use color_backtrace::{BacktracePrinter, Verbosity};
use colored::Colorize;
use std::fmt;

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
pub struct Formatted {
    /// The formatted error span information.
    pub span: Span,
    /// The backtrace to track where the Leo error originated.
    pub backtrace: Backtraced,
}

impl Formatted {
    /// Creates a backtraced error from a span and a backtrace.
    #[allow(clippy::too_many_arguments)]
    pub fn new_from_span<S>(
        message: S,
        help: Option<String>,
        code: i32,
        code_identifier: i8,
        type_: String,
        error: bool,
        span: Span,
        backtrace: Backtrace,
    ) -> Self
    where
        S: ToString,
    {
        Self {
            span,
            backtrace: Backtraced::new_from_backtrace(
                message.to_string(),
                help,
                code,
                code_identifier,
                type_,
                error,
                backtrace,
            ),
        }
    }

    /// Calls the backtraces error exit code.
    pub fn exit_code(&self) -> i32 {
        self.backtrace.exit_code()
    }

    /// Returns an error identifier.
    pub fn error_code(&self) -> String {
        self.backtrace.error_code()
    }

    /// Returns an warning identifier.
    pub fn warning_code(&self) -> String {
        self.backtrace.warning_code()
    }
}

impl fmt::Display for Formatted {
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

        let (loc, contents) = with_session_globals(|s| {
            (
                s.source_map.span_to_location(self.span).unwrap_or_else(SpanLocation::dummy),
                s.source_map.line_contents_of_span(self.span).unwrap_or_else(|| "<contents unavailable>".to_owned()),
            )
        });

        let underlined = underline(loc.col_start, loc.col_stop);

        let (kind, code) =
            if self.backtrace.error { ("Error", self.error_code()) } else { ("Warning", self.warning_code()) };

        let message = format!("{kind} [{code}]: {message}", message = self.backtrace.message,);

        // To avoid the color enabling characters for comparison with test expectations.
        if std::env::var("LEO_TESTFRAMEWORK").unwrap_or_default().trim().to_owned().is_empty() {
            if self.backtrace.error {
                write!(f, "{}", message.bold().red())?;
            } else {
                write!(f, "{}", message.bold().yellow())?;
            }
        } else {
            write!(f, "{message}")?;
        };

        write!(
            f,
            "\n{indent     }--> {path}:{line_start}:{start}\n\
            {indent     } |\n",
            indent = INDENT,
            path = &loc.source_file.name,
            line_start = loc.line_start,
            start = loc.col_start,
        )?;

        for (line_no, line) in contents.lines().enumerate() {
            writeln!(
                f,
                "{line_no:width$} | {text}",
                width = INDENT.len(),
                line_no = loc.line_start + line_no,
                text = line,
            )?;
        }

        write!(f, "{INDENT     } |{underlined}",)?;

        if let Some(help) = &self.backtrace.help {
            write!(
                f,
                "\n{INDENT     } |\n\
            {INDENT     } = {help}",
            )?;
        }

        let leo_backtrace = std::env::var("LEO_BACKTRACE").unwrap_or_default().trim().to_owned();
        match leo_backtrace.as_ref() {
            "1" => {
                let mut printer = BacktracePrinter::default();
                printer = printer.verbosity(Verbosity::Medium);
                printer = printer.lib_verbosity(Verbosity::Medium);
                let trace = printer.format_trace_to_string(&self.backtrace.backtrace).map_err(|_| fmt::Error)?;
                write!(f, "\n{trace}")?;
            }
            "full" => {
                let mut printer = BacktracePrinter::default();
                printer = printer.verbosity(Verbosity::Full);
                printer = printer.lib_verbosity(Verbosity::Full);
                let trace = printer.format_trace_to_string(&self.backtrace.backtrace).map_err(|_| fmt::Error)?;
                write!(f, "\n{trace}")?;
            }
            _ => {}
        }

        Ok(())
    }
}

impl std::error::Error for Formatted {
    fn description(&self) -> &str {
        &self.backtrace.message
    }
}
