// Copyright (C) 2019-2025 Provable Inc.
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

use leo_span::{Span, with_session_globals};

use backtrace::Backtrace;
use color_backtrace::{BacktracePrinter, Verbosity};
use colored::Colorize;
use std::fmt;

/// Represents available colors for span labels in error messages.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum SpanColor {
    Red,
    Yellow,
    Blue,
    Green,
    Cyan,
    Magenta,
}

impl SpanColor {
    /// apply bold formatting with an optional color (defaults to red if None).
    pub fn format_bold_colored(text: &str, color: Option<&SpanColor>) -> String {
        let default = SpanColor::default();
        let color = color.unwrap_or(&default);
        match color {
            SpanColor::Red => text.bold().red().to_string(),
            SpanColor::Yellow => text.bold().yellow().to_string(),
            SpanColor::Blue => text.bold().blue().to_string(),
            SpanColor::Green => text.bold().green().to_string(),
            SpanColor::Cyan => text.bold().cyan().to_string(),
            SpanColor::Magenta => text.bold().magenta().to_string(),
        }
    }
}

impl Default for SpanColor {
    fn default() -> Self {
        SpanColor::Red
    }
}

/// Formatted compiler error type
///     undefined value `x`
///     --> file.leo: 2:8
///      |
///    2 | let a = x;
///      |         ^
///      |
///      = help: Initialize a variable `x` first.
/// Makes use of the same fields as a BacktracedError.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Formatted {
    /// The formatted error span information.
    pub span: Span,
    /// Optional label for the primary span
    pub span_label: Option<String>,
    /// Optional color for the primary span (defaults to red)
    pub span_color: Option<SpanColor>,
    /// Additional spans with labels and optional colors for multi-span errors.
    pub additional_spans: Vec<(Span, String, Option<SpanColor>)>,
    /// The backtrace to track where the Leo error originated.
    pub backtrace: Backtraced,
}

impl Default for Formatted {
    fn default() -> Self {
        Self {
            span: Span::default(),
            span_label: None,
            span_color: None,
            additional_spans: Vec::new(),
            backtrace: Backtraced::default(),
        }
    }
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
            span_label: None,
            span_color: None,
            additional_spans: Vec::new(),
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

    /// Creates a backtraced error from multiple spans with labels, colors, and a backtrace.
    #[allow(clippy::too_many_arguments)]
    pub fn new_from_spans<S>(
        message: S,
        help: Option<String>,
        code: i32,
        code_identifier: i8,
        type_: String,
        error: bool,
        span: Span,
        span_label: Option<String>,
        span_color: Option<SpanColor>,
        additional_spans: Vec<(Span, String, Option<SpanColor>)>,
        backtrace: Backtrace,
    ) -> Self
    where
        S: ToString,
    {
        Self {
            span,
            span_label,
            span_color,
            additional_spans,
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
        let (kind, code) =
            if self.backtrace.error { ("Error", self.error_code()) } else { ("Warning", self.warning_code()) };

        let message = format!("{kind} [{code}]: {message}", message = self.backtrace.message,);

        // To avoid the color enabling characters for comparison with test expectations.
        if std::env::var("NOCOLOR").unwrap_or_default().trim().to_owned().is_empty() {
            if self.backtrace.error {
                writeln!(f, "{}", message.bold().red())?;
            } else {
                writeln!(f, "{}", message.bold().yellow())?;
            }
        } else {
            writeln!(f, "{message}")?;
        };

        if !self.additional_spans.is_empty() {
            let mut all_spans: Vec<(Span, Option<&String>, Option<&SpanColor>, usize)> = Vec::new();

            // "previous definition" spans
            for (span, label, color) in &self.additional_spans {
                if let Some(source_file) = with_session_globals(|s| s.source_map.find_source_file(span.lo)) {
                    let line_contents = source_file.line_contents(*span);
                    all_spans.push((*span, Some(label), color.as_ref(), line_contents.line));
                }
            }

            // duplicate/redefinition spans
            if let Some(source_file) = with_session_globals(|s| s.source_map.find_source_file(self.span.lo)) {
                let line_contents = source_file.line_contents(self.span);
                all_spans.push((self.span, self.span_label.as_ref(), self.span_color.as_ref(), line_contents.line));
            }

            all_spans.sort_by_key(|(_, _, _, line)| *line);

            // display source file name and line that the duplicate occured on
            if let Some(source_file) = with_session_globals(|s| s.source_map.find_source_file(self.span.lo)) {
                let line_contents = source_file.line_contents(self.span);
                writeln!(
                    f,
                    "{indent     }--> {path}:{line_start}:{start}",
                    indent = INDENT,
                    path = &source_file.name,
                    line_start = line_contents.line + 1,
                    start = line_contents.start + 1,
                )?;
                writeln!(f, "{INDENT     } |")?;
            }

            let mut last_line: Option<usize> = None;
            let use_color = std::env::var("NOCOLOR").unwrap_or_default().trim().to_owned().is_empty();
            for (span, label_opt, color_opt, line_num) in all_spans {
                // add "..." if there's a gap between spans
                if let Some(last) = last_line {
                    if line_num > last + 1 {
                        writeln!(f, "{INDENT     } ...")?;
                    }
                }

                if let Some(source_file) = with_session_globals(|s| s.source_map.find_source_file(span.lo)) {
                    let line_contents = source_file.line_contents(span);
                    let line_start = line_contents.line + 1;

                    // display the line with line number
                    writeln!(f, "{line_start:>4} | {}", line_contents.contents.trim_end())?;

                    // display the underline with label on the next line
                    write!(f, "{INDENT     } | ")?;

                    // spaces done to align with the span start
                    for _ in 0..line_contents.start {
                        write!(f, " ")?;
                    }

                    // carets to point out the error, i.e.:
                    //    9 |         value: u32,
                    //      |         ^^^^^^^^^^
                    let caret_len = (line_contents.end - line_contents.start).max(1);

                    for _ in 0..caret_len {
                        if use_color {
                            write!(f, "{}", SpanColor::format_bold_colored("^", color_opt))?;
                        } else {
                            write!(f, "^")?;
                        }
                    }

                    // add the label if present, i.e.:
                    //  `value` redefined here
                    if let Some(label) = label_opt {
                        if use_color {
                            write!(f, " {}", SpanColor::format_bold_colored(label, color_opt))?;
                        } else {
                            write!(f, " {}", label)?;
                        }
                    }
                    writeln!(f)?;
                }

                last_line = Some(line_num);
            }
        } else {
            if let Some(source_file) = with_session_globals(|s| s.source_map.find_source_file(self.span.lo)) {
                let line_contents = source_file.line_contents(self.span);

                writeln!(
                    f,
                    "{indent     }--> {path}:{line_start}:{start}",
                    indent = INDENT,
                    path = &source_file.name,
                    line_start = line_contents.line + 1,
                    start = line_contents.start + 1,
                )?;

                write!(f, "{line_contents}")?;
            }
        }

        if let Some(help) = &self.backtrace.help {
            writeln!(
                f,
                "{INDENT     } |\n\
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
