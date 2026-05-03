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

use crate::{compute_exit_code, format_error_code, format_warning_code};
use leo_span::{
    SESSION_GLOBALS,
    Span,
    source_map::{LeoSourceCache, is_color},
};

pub use ariadne::Color;
use ariadne::Report;
use std::{
    fmt,
    hash::{Hash, Hasher},
};

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

/// Helper span for Ariadne that includes the source file start index.
struct AriadneSpan {
    file_start_index: u32,
    span: Span,
}

impl ariadne::Span for AriadneSpan {
    type SourceId = u32;

    fn source(&self) -> &Self::SourceId {
        &self.file_start_index
    }

    fn start(&self) -> usize {
        (self.span.lo - self.file_start_index) as usize
    }

    fn end(&self) -> usize {
        (self.span.hi - self.file_start_index) as usize
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
///
/// Stores all error components as plain owned fields.
/// The ariadne `Report` is built on the fly in `Display::fmt`.
#[derive(Clone, Debug)]
pub struct Formatted {
    message: String,
    help: Option<String>,
    code: i32,
    code_identifier: i8,
    type_: String,
    error: bool,
    span: Span,
    labels: Vec<Label>,
}

impl Formatted {
    /// Creates a formatted error from a span and labels.
    #[allow(clippy::too_many_arguments)]
    pub fn new_from_span<S>(
        message: S,
        help: Option<String>,
        code: i32,
        code_identifier: i8,
        type_: String,
        error: bool,
        span: Span,
        labels: Vec<Label>,
    ) -> Self
    where
        S: ToString,
    {
        Self { message: message.to_string(), help, code, code_identifier, type_, error, span, labels }
    }

    /// Gets the exit code.
    pub fn exit_code(&self) -> i32 {
        compute_exit_code(self.code_identifier, self.code)
    }

    /// Gets a unique error identifier.
    pub fn error_code(&self) -> String {
        format_error_code(&self.type_, self.code_identifier, self.code)
    }

    /// Gets a unique warning identifier.
    pub fn warning_code(&self) -> String {
        format_warning_code(&self.type_, self.code_identifier, self.code)
    }

    /// Resolve a Leo `Span` to an `AriadneSpan` using the source map.
    fn resolve_span(span: Span, source_map: &leo_span::source_map::SourceMap) -> AriadneSpan {
        let file_start_index = source_map.find_source_file(span.lo).unwrap().absolute_start;
        AriadneSpan { file_start_index, span }
    }

    /// Build an ariadne Report from the stored fields.
    fn build_report(&self) -> Report<'_, AriadneSpan> {
        use leo_span::with_session_globals;

        let primary_color = if self.error { Color::Red } else { Color::Yellow };

        with_session_globals(|s| {
            let primary_span = Self::resolve_span(self.span, &s.source_map);

            // Always include a label for the primary span so ariadne renders the source snippet.
            let primary_label = std::iter::once(
                ariadne::Label::new(Self::resolve_span(self.span, &s.source_map)).with_color(primary_color),
            );
            let extra_labels: Vec<_> = self
                .labels
                .iter()
                .map(|l| {
                    ariadne::Label::new(Self::resolve_span(l.span, &s.source_map))
                        .with_message(&l.msg)
                        .with_color(l.color)
                })
                .collect();

            let mut report = Report::build(
                if self.error { ariadne::ReportKind::Error } else { ariadne::ReportKind::Warning },
                primary_span,
            )
            .with_config(ariadne::Config::default().with_color(is_color()))
            .with_message(&self.message)
            .with_code(if self.error { self.error_code() } else { self.warning_code() })
            .with_labels(primary_label.chain(extra_labels));

            if let Some(help) = &self.help {
                report = report.with_help(help);
            }

            report.finish()
        })
    }
}

impl fmt::Display for Formatted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if SESSION_GLOBALS.is_set() {
            let report = self.build_report();
            let mut cache = LeoSourceCache::new();
            let mut buf = Vec::new();
            report.write(&mut cache, &mut buf).map_err(|_| fmt::Error)?;
            let output = String::from_utf8(buf).map_err(|_| fmt::Error)?;
            write!(f, "{output}")
        } else {
            // Fallback when session globals are unavailable (e.g. tests).
            let (kind, code) = if self.error { ("Error", self.error_code()) } else { ("Warning", self.warning_code()) };
            write!(f, "{kind} [{code}]: {}", self.message)?;
            if let Some(help) = &self.help {
                write!(f, "\n    = help: {help}")?;
            }
            Ok(())
        }
    }
}

impl std::error::Error for Formatted {
    fn description(&self) -> &str {
        &self.message
    }
}

impl Hash for Formatted {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.message.hash(state);
        self.code.hash(state);
        self.span.hash(state);
    }
}

impl PartialEq for Formatted {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message && self.code == other.code && self.span == other.span
    }
}

impl Eq for Formatted {}
