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

use leo_span::{Span, source_map::LineContents, with_session_globals};

use backtrace::Backtrace;
use color_backtrace::{BacktracePrinter, Verbosity};
use colored::Colorize;
use itertools::Itertools;
use std::fmt;

/// Represents available colors for error labels.
#[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Color {
    #[default]
    Red,
    Yellow,
    Blue,
    Green,
    Cyan,
    Magenta,
}

impl Color {
    /// Color `text` with `self` ane make it bold.
    pub fn color_and_bold(&self, text: &str, use_colors: bool) -> String {
        if use_colors {
            match self {
                Color::Red => text.bold().red().to_string(),
                Color::Yellow => text.bold().yellow().to_string(),
                Color::Blue => text.bold().blue().to_string(),
                Color::Green => text.bold().green().to_string(),
                Color::Cyan => text.bold().cyan().to_string(),
                Color::Magenta => text.bold().magenta().to_string(),
            }
        } else {
            text.to_string()
        }
    }
}

/// Represents error labels.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Label {
    msg: String,
    span: Span,
    color: Color,
}

impl Label {
    pub fn new(msg: impl fmt::Display, span: Span) -> Self {
        Self { msg: msg.to_string(), span, color: Color::default() }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
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
    /// Additional spans with labels and optional colors for multi-span errors.
    pub labels: Vec<Label>,
    /// The backtrace to track where the Leo error originated.
    pub backtrace: Box<Backtraced>,
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
            labels: Vec::new(),
            backtrace: Box::new(Backtraced::new_from_backtrace(
                message.to_string(),
                help,
                code,
                code_identifier,
                type_,
                error,
                backtrace,
            )),
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

    /// Returns a new instance of `Formatted` which has labels.
    pub fn with_labels(mut self, labels: Vec<Label>) -> Self {
        self.labels = labels;
        self
    }
}

/// Compute the start and end columns of the highlight for each line
fn compute_line_spans(lc: &LineContents) -> Vec<(usize, usize)> {
    let lines: Vec<&str> = lc.contents.lines().collect();
    let mut byte_index = 0;
    let mut line_spans = Vec::new();

    for line in &lines {
        let line_start = byte_index;
        let line_end = byte_index + line.len();
        let start = lc.start.saturating_sub(line_start);
        let end = lc.end.saturating_sub(line_start);
        line_spans.push((start.min(line.len()), end.min(line.len())));
        byte_index = line_end + 1; // +1 for '\n'
    }

    line_spans
}

/// Print a gap or ellipsis between blocks of code
fn print_gap(
    f: &mut impl std::fmt::Write,
    prev_last_line: Option<usize>,
    first_line_of_block: usize,
) -> std::fmt::Result {
    if let Some(prev_last) = prev_last_line {
        let gap = first_line_of_block.saturating_sub(prev_last + 1);
        if gap == 1 {
            // Single skipped line
            writeln!(f, "{:width$} |", prev_last + 1, width = INDENT.len())?;
        } else if gap > 1 {
            // Multiple skipped lines
            writeln!(f, "{:width$}...", "", width = INDENT.len() - 1)?;
        }
    }
    Ok(())
}

/// Print a single line of code with connector and optional highlight
#[allow(clippy::too_many_arguments)]
fn print_code_line(
    f: &mut impl std::fmt::Write,
    line_num: usize,
    line_text: &str,
    connector: &str,
    start: usize,
    end: usize,
    multiline: bool,
    first_line: Option<usize>,
    last_line: Option<usize>,
    label: &Label,
) -> std::fmt::Result {
    let use_colors = std::env::var("NOCOLOR").unwrap_or_default().trim().to_owned().is_empty();

    // Print line number, connector, and code
    write!(f, "{:width$} | {} ", line_num, label.color.color_and_bold(connector, use_colors), width = INDENT.len())?;
    writeln!(f, "{line_text}")?;

    // Single-line highlight with caret
    if !multiline && end > start {
        writeln!(
            f,
            "{INDENT} |   {:start$}{} {}",
            "",
            label.color.color_and_bold(&"^".repeat(end - start), use_colors),
            label.color.color_and_bold(&label.msg, use_colors),
            start = start
        )?;
    }
    // Multi-line highlight: only print underline on last line
    else if multiline {
        if let (Some(first), Some(last)) = (first_line, last_line) {
            if line_num - first_line.unwrap() == last - first {
                let underline_len = (end - start).max(1);
                writeln!(
                    f,
                    "{INDENT} | {:start$}{} {}",
                    label.color.color_and_bold("|", use_colors), // vertical pointer
                    label.color.color_and_bold(&"_".repeat(underline_len), use_colors), // underline
                    label.color.color_and_bold(&label.msg, use_colors), // message
                    start = start
                )?;
            }
        }
    }

    Ok(())
}

/// Print the final underline for a multi-line highlight (Rust-style with `-`)
fn print_multiline_underline(
    f: &mut impl std::fmt::Write,
    start_col: usize,
    end_col: usize,
    label: &Label,
) -> std::fmt::Result {
    let use_colors = std::env::var("NOCOLOR").unwrap_or_default().trim().to_owned().is_empty();
    let underline_len = (end_col - start_col).max(1);
    let underline = format!("{}-", "_".repeat(underline_len));

    writeln!(
        f,
        "{INDENT} | {:start$}{} {}",
        label.color.color_and_bold("|", use_colors), // vertical pointer
        label.color.color_and_bold(&underline, use_colors), // colored underline + dash
        label.color.color_and_bold(&label.msg, use_colors), // message
        start = start_col
    )
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

        if let Some(source_file) = with_session_globals(|s| s.source_map.find_source_file(self.span.lo)) {
            let line_contents = source_file.line_contents(self.span);

            writeln!(
                f,
                "{indent     }--> {path}:{line_start}:{start}",
                indent = INDENT,
                path = &source_file.name,
                // Report lines starting from line 1.
                line_start = line_contents.line + 1,
                // And columns - comments in some old code claims to report columns indexing from 0,
                // but that doesn't appear to have been true.
                start = line_contents.start + 1,
            )?;

            if self.labels.is_empty() {
                // If there are no labels, just print the line contents which will point to the right location.
                write!(f, "{line_contents}")?;
            } else {
                // If there are labels, we handle the printing manually. Something like:
                //     |
                //  50 | /         x
                //  51 | |             :
                //  52 | |                 u32,
                //     | |___________________- `x` first declared here
                //   ...
                //  55 |           x: u32
                //     |           ^^^^^^ struct field already declared

                // Sort the labels by their source line number.
                let labels = self
                    .labels
                    .iter()
                    .filter_map(|label| {
                        with_session_globals(|s| s.source_map.find_source_file(label.span.lo)).map(|source_file| {
                            let lc = source_file.line_contents(label.span);
                            (label.clone(), lc.line)
                        })
                    })
                    .sorted_by_key(|(_, line)| *line)
                    .map(|(label, _)| label)
                    .collect_vec();

                // Track the last printed line number to handle gaps between blocks
                let mut prev_last_line: Option<usize> = None;

                for label in labels {
                    // Find the source file corresponding to this label's span
                    let Some(source_file) = with_session_globals(|s| s.source_map.find_source_file(label.span.lo))
                    else {
                        continue;
                    };

                    // Get the line contents and offsets for this span
                    let lc = source_file.line_contents(label.span);

                    // Compute start and end columns of highlights for each line
                    let line_spans = compute_line_spans(&lc);

                    let first_line_of_block = lc.line + 1; // 1-based line number of the first line

                    // Print a gap or ellipsis if there are skipped lines since previous label
                    print_gap(f, prev_last_line, first_line_of_block)?;

                    // Print a leading vertical margin only for the first label
                    if prev_last_line.is_none() {
                        writeln!(f, "{INDENT} |")?;
                    }

                    // Determine if this label spans multiple lines
                    let multiline = line_spans.iter().any(|&(s, e)| e > s) && lc.contents.lines().count() > 1;

                    // Identify first and last lines that have a highlight
                    let first_line = line_spans.iter().position(|&(s, e)| e > s);
                    let last_line = line_spans.iter().rposition(|&(s, e)| e > s);

                    // Iterate over each line in the span
                    for (i, (line_text, &(start, end))) in lc.contents.lines().zip(&line_spans).enumerate() {
                        let line_num = lc.line + i + 1;

                        // Choose connector symbol for multi-line highlights:
                        // "/" for first line, "|" for continuation lines
                        let connector = if multiline {
                            match (first_line, last_line) {
                                (Some(first), Some(_last)) => {
                                    if i == first {
                                        "/"
                                    } else {
                                        "|"
                                    }
                                }
                                _ => " ",
                            }
                        } else {
                            " "
                        };

                        // Print the code line with connector and optional single-line caret
                        print_code_line(
                            f, line_num, line_text, connector, start, end, multiline, first_line, last_line, &label,
                        )?;
                    }

                    // If this was a multi-line highlight, print the final underline + message
                    if multiline {
                        if let (Some(_), Some(last)) = (first_line, last_line) {
                            // Start column: first highlighted character on the last line
                            let start_col = line_spans[last].0;

                            // End column: last highlighted character on the last line
                            let end_col = line_spans[last].1;

                            print_multiline_underline(f, start_col, end_col, &label)?;
                        }
                    }

                    // Update the previous last line to track gaps for the next label
                    prev_last_line = Some(lc.line + lc.contents.lines().count());
                }
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
