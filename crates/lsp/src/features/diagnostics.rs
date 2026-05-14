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

//! Internal diagnostics model and LSP wire-format conversion.
//!
//! Worker lowering happens in [`compiler_bridge::lower_compiler_diagnostics`]
//! while the Leo session source map is still alive; the routing thread reads
//! the resulting [`DiagnosticSet`] off `CachedPackageAnalysis` and converts
//! it to `lsp_types::Diagnostic` payloads at publish time.

use crate::document_store::PackageAnalysisKey;
use leo_errors::{DiagnosticView, LeoError, LeoWarning};
use leo_span::{Span, source_map::FileName, with_session_globals};
use line_index::{LineIndex, TextSize, WideEncoding, WideLineCol};
use lsp_types::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, Position, Range};
use std::{path::PathBuf, sync::Arc};

/// `Diagnostic.source` value shared by every Leo-emitted entry.
const DIAGNOSTIC_SOURCE: &str = "leo";

/// UTF-16 LSP range carried by internal diagnostic entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct DiagnosticRange {
    pub(crate) start_line: u32,
    pub(crate) start_character: u32,
    pub(crate) end_line: u32,
    pub(crate) end_character: u32,
}

impl DiagnosticRange {
    fn to_lsp_range(self) -> Range {
        Range::new(
            Position::new(self.start_line, self.start_character),
            Position::new(self.end_line, self.end_character),
        )
    }

    /// Zero-width range anchored at the start of the document, used by
    /// spanless package- and dependency-level errors.
    fn document_start() -> Self {
        Self { start_line: 0, start_character: 0, end_line: 0, end_character: 0 }
    }
}

/// Internal severity decoupled from the LSP wire enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum DiagnosticSeverityInternal {
    Error,
    Warning,
}

/// One compiler diagnostic plus its LSP-ready range, severity, and labels.
#[derive(Debug, Clone)]
pub(crate) struct DiagnosticEntry {
    pub(crate) path: Arc<PathBuf>,
    pub(crate) range: DiagnosticRange,
    pub(crate) severity: DiagnosticSeverityInternal,
    /// Message with help/note appendices already folded in.
    pub(crate) message: Arc<str>,
    pub(crate) related: Arc<[DiagnosticRelatedEntry]>,
    /// Synthetic entries lack a usable source span and are pinned to the
    /// saved trigger document by the publish path.
    pub(crate) synthetic: bool,
}

/// One secondary label keyed by file path and UTF-16 range.
#[derive(Debug, Clone)]
pub(crate) struct DiagnosticRelatedEntry {
    pub(crate) path: Arc<PathBuf>,
    pub(crate) range: DiagnosticRange,
    pub(crate) message: Arc<str>,
}

/// Immutable package-keyed diagnostic set shared with the routing thread.
#[derive(Debug, Clone)]
pub(crate) struct DiagnosticSet {
    pub(crate) key: PackageAnalysisKey,
    pub(crate) entries: Arc<[DiagnosticEntry]>,
}

#[cfg(test)]
impl DiagnosticSet {
    /// Empty set placeholder used by feature unit tests that build a
    /// `CachedPackageAnalysis` directly.
    pub(crate) fn empty(key: PackageAnalysisKey) -> Self {
        Self { key, entries: Arc::from(Vec::<DiagnosticEntry>::new()) }
    }
}

/// Snapshot of `textDocument.publishDiagnostics` capabilities captured at
/// initialize.
#[derive(Debug, Clone, Default)]
pub(crate) struct DiagnosticClientCapabilitySnapshot {
    pub(crate) related_information: bool,
}

impl DiagnosticClientCapabilitySnapshot {
    pub(crate) fn from_initialize(params: &lsp_types::InitializeParams) -> Self {
        let related_information = params
            .capabilities
            .text_document
            .as_ref()
            .and_then(|text_document| text_document.publish_diagnostics.as_ref())
            .and_then(|publish| publish.related_information)
            .unwrap_or(false);
        Self { related_information }
    }
}

/// Borrowed view of a single buffered compiler error or warning.
pub(crate) enum CompilerDiagnostic<'a> {
    Error(&'a LeoError),
    Warning(&'a LeoWarning),
}

/// Resolved source-file context for a Leo span.
struct ResolvedSpan {
    path: Arc<PathBuf>,
    range: DiagnosticRange,
}

/// Lower buffered compiler diagnostics into LSP-ready entries.
///
/// Must run while the Leo session is still active because span resolution
/// reads the source map through `with_session_globals`. Diagnostics whose
/// primary span cannot resolve to a real file produce a synthetic entry on
/// `trigger_path` (if any); the publish path pins those to the saved
/// document.
pub(crate) fn lower_diagnostic_entries<'a, I>(
    trigger_path: Option<&Arc<PathBuf>>,
    diagnostics: I,
) -> Vec<DiagnosticEntry>
where
    I: IntoIterator<Item = CompilerDiagnostic<'a>>,
{
    let mut entries = Vec::new();
    for diagnostic in diagnostics {
        let (view, severity) = match diagnostic {
            CompilerDiagnostic::Error(error) => {
                // `LastErrorCode` is a sentinel for an already-emitted error.
                if error.is_last_error_code() {
                    continue;
                }
                match error.diagnostic_view() {
                    Some(view) => (view, DiagnosticSeverityInternal::Error),
                    None => {
                        if let Some(path) = trigger_path {
                            entries.push(make_synthetic_entry(
                                Arc::clone(path),
                                error.to_string(),
                                DiagnosticSeverityInternal::Error,
                            ));
                        }
                        continue;
                    }
                }
            }
            CompilerDiagnostic::Warning(warning) => match warning.diagnostic_view() {
                Some(view) => (view, DiagnosticSeverityInternal::Warning),
                None => {
                    if let Some(path) = trigger_path {
                        entries.push(make_synthetic_entry(
                            Arc::clone(path),
                            warning.to_string(),
                            DiagnosticSeverityInternal::Warning,
                        ));
                    }
                    continue;
                }
            },
        };
        if let Some(entry) = lower_view(view, severity, trigger_path) {
            entries.push(entry);
        }
    }
    entries
}

/// Sort and dedupe entries into the canonical order used at publish time.
///
/// Ordering: path → range → severity (errors first) → message. Duplicates
/// compare on the same tuple.
pub(crate) fn finalize_entries(mut entries: Vec<DiagnosticEntry>) -> Vec<DiagnosticEntry> {
    entries.sort_by(|left, right| {
        left.path
            .as_path()
            .cmp(right.path.as_path())
            .then_with(|| left.range.start_line.cmp(&right.range.start_line))
            .then_with(|| left.range.start_character.cmp(&right.range.start_character))
            .then_with(|| left.range.end_line.cmp(&right.range.end_line))
            .then_with(|| left.range.end_character.cmp(&right.range.end_character))
            .then_with(|| severity_rank(left.severity).cmp(&severity_rank(right.severity)))
            .then_with(|| left.message.as_ref().cmp(right.message.as_ref()))
    });
    entries.dedup_by(|left, right| {
        left.path == right.path
            && left.range == right.range
            && left.severity == right.severity
            && left.message.as_ref() == right.message.as_ref()
    });
    entries
}

/// Lower a structured view into an entry, falling back to a synthetic entry
/// when the span cannot resolve and a trigger path is available.
fn lower_view(
    view: DiagnosticView<'_>,
    severity: DiagnosticSeverityInternal,
    trigger_path: Option<&Arc<PathBuf>>,
) -> Option<DiagnosticEntry> {
    let related = view
        .labels
        .iter()
        .filter_map(|label| {
            let span = resolve_span(label.span)?;
            Some(DiagnosticRelatedEntry {
                path: span.path,
                range: span.range,
                message: Arc::from(label.message.as_str()),
            })
        })
        .collect::<Vec<_>>();
    let message = Arc::from(build_message(&view).as_str());

    match view.span.and_then(resolve_span) {
        Some(resolved) => Some(DiagnosticEntry {
            path: resolved.path,
            range: resolved.range,
            severity,
            message,
            related: Arc::from(related),
            synthetic: false,
        }),
        None => trigger_path.map(|trigger| DiagnosticEntry {
            path: Arc::clone(trigger),
            range: DiagnosticRange::document_start(),
            severity,
            message,
            related: Arc::from(related),
            synthetic: true,
        }),
    }
}

/// Build a synthetic entry directly from a diagnostic's `Display` text.
///
/// Used for non-`Formatted` errors (`Backtraced`, etc.) that carry no
/// structured view; the rendered string is the best fidelity available.
fn make_synthetic_entry(path: Arc<PathBuf>, message: String, severity: DiagnosticSeverityInternal) -> DiagnosticEntry {
    DiagnosticEntry {
        path,
        range: DiagnosticRange::document_start(),
        severity,
        message: Arc::from(message.as_str()),
        related: Arc::from(Vec::<DiagnosticRelatedEntry>::new()),
        synthetic: true,
    }
}

/// Append help and note text to the primary message. Severity is published
/// independently, so no severity prefix is added.
fn build_message(view: &DiagnosticView<'_>) -> String {
    let mut message = String::from(view.message);
    if let Some(help) = view.help {
        message.push_str("\nhelp: ");
        message.push_str(help);
    }
    if let Some(note) = view.note {
        message.push_str("\nnote: ");
        message.push_str(note);
    }
    message
}

/// Resolve a Leo span to a `(path, UTF-16 range)` pair via session globals.
fn resolve_span(span: Span) -> Option<ResolvedSpan> {
    if span.is_dummy() {
        return None;
    }
    with_session_globals(|session| {
        let start_file = session.source_map.find_source_file(span.lo)?;
        if span.hi > start_file.absolute_end {
            // Span crosses a source-file boundary; not expressible as one LSP range.
            return None;
        }
        let FileName::Real(path) = &start_file.name else {
            return None;
        };
        let line_index = LineIndex::new(&start_file.src);
        let range = byte_range_to_diagnostic_range(
            &line_index,
            start_file.relative_offset(span.lo),
            start_file.relative_offset(span.hi),
        )?;
        Some(ResolvedSpan { path: Arc::new(path.clone()), range })
    })
}

/// Convert UTF-8 byte offsets within one file into a UTF-16 LSP range.
pub(crate) fn byte_range_to_diagnostic_range(line_index: &LineIndex, start: u32, end: u32) -> Option<DiagnosticRange> {
    let (start_line, start_character) = byte_offset_to_utf16(line_index, start)?;
    let (end_line, end_character) = byte_offset_to_utf16(line_index, end)?;
    Some(DiagnosticRange { start_line, start_character, end_line, end_character })
}

fn byte_offset_to_utf16(line_index: &LineIndex, offset: u32) -> Option<(u32, u32)> {
    let utf8 = line_index.try_line_col(TextSize::from(offset))?;
    let WideLineCol { line, col } = line_index.to_wide(WideEncoding::Utf16, utf8)?;
    Some((line, col))
}

fn severity_rank(severity: DiagnosticSeverityInternal) -> u8 {
    match severity {
        DiagnosticSeverityInternal::Error => 0,
        DiagnosticSeverityInternal::Warning => 1,
    }
}

/// Build the LSP wire payload for one entry.
///
/// `Diagnostic.code`, `codeDescription`, and `data` are intentionally
/// omitted from the wire — they have no consumer yet and would just clutter
/// editors. Related-information is gated by the client capability snapshot.
pub(crate) fn entry_to_lsp_diagnostic(
    entry: &DiagnosticEntry,
    capabilities: &DiagnosticClientCapabilitySnapshot,
) -> Diagnostic {
    let mut diagnostic = Diagnostic {
        range: entry.range.to_lsp_range(),
        severity: Some(internal_to_lsp_severity(entry.severity)),
        code: None,
        code_description: None,
        source: Some(DIAGNOSTIC_SOURCE.to_owned()),
        message: entry.message.as_ref().to_owned(),
        related_information: None,
        tags: None,
        data: None,
    };

    if capabilities.related_information && !entry.related.is_empty() {
        let related = entry
            .related
            .iter()
            .filter_map(|label| {
                let uri = crate::project_model::path_to_file_uri(label.path.as_path())?;
                Some(DiagnosticRelatedInformation {
                    location: Location { uri, range: label.range.to_lsp_range() },
                    message: label.message.as_ref().to_owned(),
                })
            })
            .collect::<Vec<_>>();
        if !related.is_empty() {
            diagnostic.related_information = Some(related);
        }
    }

    diagnostic
}

fn internal_to_lsp_severity(severity: DiagnosticSeverityInternal) -> DiagnosticSeverity {
    match severity {
        DiagnosticSeverityInternal::Error => DiagnosticSeverity::ERROR,
        DiagnosticSeverityInternal::Warning => DiagnosticSeverity::WARNING,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn sort_orders_by_path_range_severity_message() {
        let path_a = Arc::new(PathBuf::from("/tmp/a.leo"));
        let path_b = Arc::new(PathBuf::from("/tmp/b.leo"));
        let entries = finalize_entries(vec![
            entry(Arc::clone(&path_b), 0, 0, 0, 1, DiagnosticSeverityInternal::Error, "msg"),
            entry(Arc::clone(&path_a), 0, 0, 0, 1, DiagnosticSeverityInternal::Warning, "msg"),
            entry(Arc::clone(&path_a), 0, 0, 0, 1, DiagnosticSeverityInternal::Error, "z"),
            entry(Arc::clone(&path_a), 0, 0, 0, 1, DiagnosticSeverityInternal::Error, "msg"),
        ]);

        assert_eq!(entries.len(), 4);
        assert_eq!(entries[0].path, path_a);
        assert_eq!(entries[0].severity, DiagnosticSeverityInternal::Error);
        assert_eq!(entries[0].message.as_ref(), "msg");
        assert_eq!(entries[1].message.as_ref(), "z");
        assert_eq!(entries[2].severity, DiagnosticSeverityInternal::Warning);
        assert_eq!(entries[3].path, path_b);
    }

    #[test]
    fn dedup_removes_exact_duplicates() {
        let path = Arc::new(PathBuf::from("/tmp/a.leo"));
        let entries = finalize_entries(vec![
            entry(Arc::clone(&path), 0, 0, 0, 1, DiagnosticSeverityInternal::Error, "msg"),
            entry(Arc::clone(&path), 0, 0, 0, 1, DiagnosticSeverityInternal::Error, "msg"),
        ]);
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn related_information_requires_advertised_support() {
        let path = Arc::new(PathBuf::from("/tmp/a.leo"));
        let mut entry = entry(Arc::clone(&path), 1, 0, 1, 5, DiagnosticSeverityInternal::Error, "msg");
        entry.related = Arc::from(vec![DiagnosticRelatedEntry {
            path: Arc::clone(&path),
            range: DiagnosticRange { start_line: 2, start_character: 0, end_line: 2, end_character: 4 },
            message: Arc::from("see also"),
        }]);

        let without = entry_to_lsp_diagnostic(&entry, &DiagnosticClientCapabilitySnapshot::default());
        assert!(without.related_information.is_none());

        let with = entry_to_lsp_diagnostic(&entry, &DiagnosticClientCapabilitySnapshot { related_information: true });
        assert!(with.related_information.is_some());
    }

    #[test]
    fn byte_range_uses_utf16_columns_for_multibyte_text() {
        let text = "é a\nb 🦀";
        let line_index = LineIndex::new(text);

        // `é` is two UTF-8 bytes but one UTF-16 code unit; the space after it lands at column 1.
        let space_after_e = byte_range_to_diagnostic_range(&line_index, 2, 3).expect("e acute range");
        assert_eq!((space_after_e.start_line, space_after_e.start_character), (0, 1));
        assert_eq!(space_after_e.end_character, 2);

        // `🦀` is four UTF-8 bytes but two UTF-16 code units (surrogate pair).
        let crab_start = text.find('🦀').expect("crab in text") as u32;
        let crab_end = crab_start + '🦀'.len_utf8() as u32;
        let crab = byte_range_to_diagnostic_range(&line_index, crab_start, crab_end).expect("crab range");
        assert_eq!((crab.start_line, crab.start_character), (1, 2));
        assert_eq!(crab.end_character, 4);
    }

    fn entry(
        path: Arc<PathBuf>,
        start_line: u32,
        start_character: u32,
        end_line: u32,
        end_character: u32,
        severity: DiagnosticSeverityInternal,
        message: &str,
    ) -> DiagnosticEntry {
        DiagnosticEntry {
            path,
            range: DiagnosticRange { start_line, start_character, end_line, end_character },
            severity,
            message: Arc::from(message),
            related: Arc::from(Vec::<DiagnosticRelatedEntry>::new()),
            synthetic: false,
        }
    }
}
