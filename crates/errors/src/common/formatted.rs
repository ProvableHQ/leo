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
    SESSION_GLOBALS, Span,
    source_map::{LeoSourceCache, is_color},
};

pub use ariadne::Color;
use ariadne::{IndexType, Report};
use std::fmt;

/// Represents error labels.
#[derive(Debug)]
pub struct Label {
    msg: String,
    span: Span,
    color: Color,
}

impl Label {
    pub fn new(span: Span) -> Self {
        Self { msg: String::new(), span, color: Color::default() }
    }

    pub fn with_message(mut self, msg: impl fmt::Display) -> Self {
        self.msg = msg.to_string();
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Borrow the secondary label's message without copying.
    ///
    /// Surfaced for `Formatted::diagnostic_view`, which lowers labels into
    /// LSP-facing related-information entries without re-parsing rendered text.
    pub fn message(&self) -> &str {
        &self.msg
    }

    /// Return the secondary label's source span.
    ///
    /// Used by structured diagnostic consumers (notably `leo-lsp`) to convert
    /// labels into UTF-16 ranges while the session source map is still alive.
    pub fn span(&self) -> Span {
        self.span
    }
}

/// Helper span for Ariadne that includes the source file start index.
#[derive(Clone)]
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
#[derive(Debug)]
pub struct Formatted {
    inner: Box<FormattedInner>,
}

#[derive(Debug)]
struct FormattedInner {
    message: String,
    help: Option<String>,
    note: Option<String>,
    code: i32,
    type_: String,
    error: bool,
    span: Span,
    labels: Vec<Label>,
    primary_span_underline: bool,
}

impl Formatted {
    /// Creates a formatted error from a span and labels.
    pub fn new_from_span<S>(
        message: S,
        help: Option<String>,
        code: i32,
        type_: String,
        error: bool,
        span: Span,
        labels: Vec<Label>,
    ) -> Self
    where
        S: ToString,
    {
        Self {
            inner: Box::new(FormattedInner {
                message: message.to_string(),
                help,
                note: None,
                code,
                type_,
                error,
                span,
                labels,
                primary_span_underline: false,
            }),
        }
    }

    /// Create a new error.
    pub fn error(code_prefix: &str, code: i32, message: impl ToString, span: Span) -> Self {
        Self::new_from_span(message, None, code, code_prefix.to_string(), true, span, vec![])
    }

    /// Create a new warning.
    pub fn warning(code_prefix: &str, code: i32, message: impl ToString, span: Span) -> Self {
        Self::new_from_span(message, None, code, code_prefix.to_string(), false, span, vec![])
    }

    pub fn with_help(mut self, help: impl fmt::Display) -> Self {
        self.inner.help = Some(help.to_string());
        self
    }

    pub fn with_note(mut self, note: impl fmt::Display) -> Self {
        self.inner.note = Some(note.to_string());
        self
    }

    pub fn with_label(mut self, label: Label) -> Self {
        self.inner.labels.push(label);
        self
    }

    /// Render the primary single-line span with a plain underline even when the
    /// diagnostic has no primary label message.
    pub fn with_primary_span_underline(mut self) -> Self {
        self.inner.primary_span_underline = true;
        self
    }

    pub fn with_labels(mut self, labels: impl IntoIterator<Item = Label>) -> Self {
        self.inner.labels.extend(labels);
        self
    }

    /// Gets the exit code.
    pub fn exit_code(&self) -> i32 {
        compute_exit_code(37, self.inner.code)
    }

    /// Gets a unique error identifier.
    pub fn error_code(&self) -> String {
        format_error_code(&self.inner.type_, 37, self.inner.code)
    }

    /// Gets a unique warning identifier.
    pub fn warning_code(&self) -> String {
        format_warning_code(&self.inner.type_, 37, self.inner.code)
    }

    /// Return the diagnostic's primary message without ariadne rendering.
    ///
    /// Used by tooling consumers (notably `leo-lsp`) that need the bare message
    /// text rather than the formatted report. The returned slice borrows from
    /// the same allocation as the rest of the diagnostic and therefore stays
    /// valid for the lifetime of the `Formatted` value.
    pub fn message(&self) -> &str {
        &self.inner.message
    }

    /// Return the diagnostic's optional help text, if any.
    ///
    /// `leo-lsp` appends this to the LSP diagnostic message so editor clients
    /// see the same hint that the CLI report would render.
    pub fn help(&self) -> Option<&str> {
        self.inner.help.as_deref()
    }

    /// Return the diagnostic's optional follow-up note text, if any.
    ///
    /// `leo-lsp` appends this to the LSP diagnostic message for parity with
    /// CLI-rendered reports.
    pub fn note(&self) -> Option<&str> {
        self.inner.note.as_deref()
    }

    /// Return whether this diagnostic was raised as an error rather than a
    /// warning.
    ///
    /// LSP severity mapping depends on this flag instead of inspecting the
    /// rendered `Error`/`Warning` prefix in the formatted message.
    pub fn is_error(&self) -> bool {
        self.inner.error
    }

    /// Return the diagnostic's primary span.
    ///
    /// Callers must resolve this span against `leo_span` session globals to
    /// recover the originating source file before the surrounding session is
    /// torn down.
    pub fn span(&self) -> Span {
        self.inner.span
    }

    /// Iterate the diagnostic's secondary labels in declaration order.
    ///
    /// Labels carry their own span and human-readable message, which `leo-lsp`
    /// surfaces as `Diagnostic.relatedInformation` when the client supports it.
    pub fn labels(&self) -> impl Iterator<Item = &Label> {
        self.inner.labels.iter()
    }

    /// Borrow this diagnostic as a plain structured view.
    ///
    /// The returned [`DiagnosticView`] exposes the same fields that are used
    /// when rendering the ariadne report, without round-tripping through a
    /// formatted string. Consumers like `leo-lsp` use the view to build LSP
    /// `Diagnostic` payloads without parsing rendered output.
    pub fn diagnostic_view(&self) -> DiagnosticView<'_> {
        let code = if self.inner.error { self.error_code() } else { self.warning_code() };
        let labels = self
            .inner
            .labels
            .iter()
            .map(|label| DiagnosticLabelView { message: label.message().to_owned(), span: label.span() })
            .collect();
        DiagnosticView {
            message: &self.inner.message,
            help: self.inner.help.as_deref(),
            note: self.inner.note.as_deref(),
            code,
            is_error: self.inner.error,
            span: Some(self.inner.span),
            labels,
        }
    }

    /// Resolve a Leo `Span` to an `AriadneSpan` using the source map.
    fn resolve_span(span: Span, source_map: &leo_span::source_map::SourceMap) -> AriadneSpan {
        let file_start_index = source_map.find_source_file(span.lo).unwrap().absolute_start;
        AriadneSpan { file_start_index, span }
    }

    /// Build an ariadne Report from the stored fields.
    fn build_report(&self) -> Report<'_, AriadneSpan> {
        use leo_span::with_session_globals;

        let primary_color = if self.inner.error { Color::Red } else { Color::Yellow };

        with_session_globals(|s| {
            let primary_span = Self::resolve_span(self.inner.span, &s.source_map);

            // Ariadne only renders source lines for a multi-line label when its message is `Some(_)`
            // (see `multi_labels_with_message` in ariadne's write.rs). For single-line spans we skip
            // `with_message` to avoid the dangling `─┬─` / `╰──` decorations under the caret.
            let primary_is_multiline = s.source_map.find_source_file(self.inner.span.lo).is_some_and(|f| {
                let lo = (self.inner.span.lo - f.absolute_start) as usize;
                let hi = (self.inner.span.hi - f.absolute_start) as usize;
                f.src.as_bytes().get(lo..hi).is_some_and(|b| b.contains(&b'\n'))
            });
            let mut primary = ariadne::Label::new(primary_span.clone()).with_color(primary_color);
            if primary_is_multiline {
                primary = primary.with_message("here");
            } else if self.inner.primary_span_underline {
                primary = primary.with_message("");
            }
            let primary_label = std::iter::once(primary);

            let extra_labels: Vec<_> = self
                .inner
                .labels
                .iter()
                .map(|l| {
                    ariadne::Label::new(Self::resolve_span(l.span, &s.source_map))
                        .with_message(&l.msg)
                        .with_color(l.color)
                })
                .collect();

            let mut report = Report::build(
                if self.inner.error { ariadne::ReportKind::Error } else { ariadne::ReportKind::Warning },
                primary_span,
            )
            .with_config(ariadne::Config::default().with_color(is_color()).with_index_type(IndexType::Byte))
            .with_message(&self.inner.message)
            .with_code(if self.inner.error { self.error_code() } else { self.warning_code() })
            .with_labels(primary_label.chain(extra_labels));

            if let Some(help) = &self.inner.help {
                report = report.with_help(help);
            }

            if let Some(note) = &self.inner.note {
                report = report.with_note(note);
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
            let output = if self.inner.primary_span_underline {
                normalize_empty_primary_label_underline(&output)
            } else {
                output
            };
            write!(f, "{output}")
        } else {
            // Fallback when session globals are unavailable (e.g. tests).
            let (kind, code) =
                if self.inner.error { ("Error", self.error_code()) } else { ("Warning", self.warning_code()) };
            write!(f, "{kind} [{code}]: {}", self.inner.message)?;
            if let Some(help) = &self.inner.help {
                write!(f, "\n    = help: {help}")?;
            }
            if let Some(note) = &self.inner.note {
                write!(f, "\n    = note: {note}")?;
            }
            Ok(())
        }
    }
}

fn normalize_empty_primary_label_underline(output: &str) -> String {
    let mut normalized = Vec::new();
    let mut lines = output.lines();

    while let Some(line) = lines.next() {
        // Ariadne requires an empty label message to draw the underline row, but
        // that also emits an empty pointer connector. Keep the underline and
        // drop only the connector line for diagnostics that explicitly opt in.
        if line.contains('┬') && line.contains('─') {
            normalized.push(line.replace('┬', "─").trim_end().to_owned());
            if let Some(next) = lines.next()
                && !next.contains('╰')
            {
                normalized.push(next.to_owned());
            }
        } else {
            normalized.push(line.to_owned());
        }
    }

    let mut output = normalized.join("\n");
    if output.is_empty() || output.ends_with('\n') {
        output
    } else {
        output.push('\n');
        output
    }
}

impl std::error::Error for Formatted {
    fn description(&self) -> &str {
        &self.inner.message
    }
}

/// LSP-agnostic structured view of a compiler diagnostic.
///
/// Exposed so editor tooling — currently `leo-lsp` — can lower errors and
/// warnings into editor-facing diagnostics without parsing rendered ariadne
/// output. The view borrows from the originating [`Formatted`] for cheap
/// strings while owning a small per-label `Vec`, which is the smallest shape
/// that keeps secondary-label messages alive across an `extract_errs` call.
#[derive(Debug, Clone)]
pub struct DiagnosticView<'a> {
    /// Primary human-readable message.
    pub message: &'a str,
    /// Optional help hint shown beneath the diagnostic on the CLI.
    pub help: Option<&'a str>,
    /// Optional follow-up note shown beneath the help line on the CLI.
    pub note: Option<&'a str>,
    /// Fully formatted code identifier (e.g. `EPAR0001` or `WTYC0001`).
    pub code: String,
    /// Whether the diagnostic is an error (`true`) or a warning (`false`).
    pub is_error: bool,
    /// Primary span, when the diagnostic ties to a concrete source location.
    pub span: Option<Span>,
    /// Secondary spans annotated with their own human-readable messages.
    pub labels: Vec<DiagnosticLabelView>,
}

/// One secondary label paired with its source span.
///
/// `leo-lsp` lowers each label into `Diagnostic.relatedInformation` when the
/// client advertises support, so it captures both the span and the message.
#[derive(Debug, Clone)]
pub struct DiagnosticLabelView {
    /// Human-readable description for the label.
    pub message: String,
    /// Span associated with the label, resolved against session source globals.
    pub span: Span,
}

#[cfg(test)]
mod tests {
    use super::{Color, Formatted, Label};
    use leo_span::{Span, create_session_if_not_set_then, source_map::FileName};

    /// Verifies the structured view round-trips primary message, code, help, and note.
    #[test]
    fn diagnostic_view_exposes_primary_fields() {
        create_session_if_not_set_then(|_| {
            let span = Span::default();
            let error = Formatted::error("TST", 1, "boom", span).with_help("try again").with_note("note text");

            let view = error.diagnostic_view();
            assert_eq!(view.message, "boom");
            assert_eq!(view.help, Some("try again"));
            assert_eq!(view.note, Some("note text"));
            assert_eq!(view.code, error.error_code());
            assert!(view.is_error);
            assert_eq!(view.span, Some(span));
            assert!(view.labels.is_empty());
        });
    }

    /// Verifies labels are exposed with their messages and spans intact.
    #[test]
    fn diagnostic_view_exposes_secondary_labels() {
        create_session_if_not_set_then(|_| {
            let primary = Span::new(0, 4);
            let label_span = Span::new(5, 10);
            let error = Formatted::error("TST", 2, "boom", primary)
                .with_label(Label::new(label_span).with_message("see also").with_color(Color::Blue));

            let view = error.diagnostic_view();
            assert_eq!(view.labels.len(), 1);
            assert_eq!(view.labels[0].message, "see also");
            assert_eq!(view.labels[0].span, label_span);
        });
    }

    /// Verifies warnings round-trip through the structured view with severity preserved.
    #[test]
    fn diagnostic_view_marks_warnings() {
        create_session_if_not_set_then(|_| {
            let warning = Formatted::warning("TST", 3, "watch out", Span::default());
            let view = warning.diagnostic_view();
            assert!(!view.is_error);
            assert_eq!(view.code, warning.warning_code());
        });
    }

    /// Verifies only multi-line primary spans render with a visible fallback label.
    #[test]
    fn multi_line_primary_span_uses_default_label() {
        create_session_if_not_set_then(|s| {
            let source = "program test.aleo {\n    @custom\n    constructor() {}\n}\n";
            let file = s.source_map.new_source(source, FileName::Custom("test.leo".into()));
            let lo = file.absolute_start + source.find("@custom").unwrap() as u32;
            let hi = file.absolute_start + source.find(" {}\n").unwrap() as u32 + 3;
            let rendered = Formatted::error("TST", 4, "boom", Span::new(lo, hi)).to_string();

            assert!(rendered.contains("here"));

            let lo = file.absolute_start + source.find("test").unwrap() as u32;
            let hi = lo + "test".len() as u32;
            let rendered = Formatted::error("TST", 5, "boom", Span::new(lo, hi)).to_string();

            assert!(!rendered.contains("here"));
        });
    }

    /// Verifies diagnostics can opt into an unmessaged primary underline without
    /// inheriting ariadne's empty-message pointer connector.
    #[test]
    fn primary_span_underline_omits_empty_pointer_connector() {
        create_session_if_not_set_then(|s| {
            let source = "program test.aleo {\n}\n";
            let file = s.source_map.new_source(source, FileName::Custom("test.leo".into()));
            let lo = file.absolute_start + source.find("test.aleo").unwrap() as u32;
            let hi = lo + "test.aleo".len() as u32;
            let rendered =
                Formatted::error("TST", 6, "boom", Span::new(lo, hi)).with_primary_span_underline().to_string();

            assert!(rendered.chars().filter(|c| *c == '─').count() >= 13);
            assert!(!rendered.contains('┬'));
            assert!(!rendered.contains('╰'));
        });
    }
}
