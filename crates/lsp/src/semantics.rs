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

use leo_ast::Location;
use leo_span::Symbol;
use std::{path::PathBuf, sync::Arc};

/// Stable file-relative byte range for semantic indexing.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileRange {
    /// File path owning this range.
    pub path: Arc<PathBuf>,
    /// Inclusive start byte offset relative to `path`.
    pub start: u32,
    /// Exclusive end byte offset relative to `path`.
    pub end: u32,
}

impl FileRange {
    /// Create a non-empty file-relative range.
    pub fn new(path: Arc<PathBuf>, start: u32, end: u32) -> Option<Self> {
        (start < end).then_some(Self { path, start, end })
    }
}

/// Semantic token source used to build the latest snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticSource {
    /// Only syntax-based token classification succeeded.
    SyntaxOnly,
    /// Compiler frontend analysis refined the syntax fallback.
    CompilerEnhanced,
}

/// Internal token kind independent from the LSP wire format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticKind {
    Namespace,
    Type,
    Interface,
    Function,
    Parameter,
    Variable,
    Property,
    Keyword,
    Comment,
    String,
    Number,
    Operator,
}

/// Declaration-vs-reference role tracked for later navigation features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OccurrenceRole {
    Declaration,
    Reference,
}

/// Stable semantic identity shared across occurrences of the same symbol.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolIdentity {
    /// A lexical binding keyed by its declaration span.
    Local { declaration: FileRange },
    /// A top-level declaration tracked by compiler `Location`.
    GlobalItem { location: Location, declaration: Option<FileRange> },
    /// A member declaration or access owned by the current surrounding item.
    Member { owner: Option<Location>, name: Symbol, declaration: Option<FileRange> },
    /// A program or imported namespace reference.
    Program { name: Symbol, declaration: Option<FileRange> },
    /// Syntax-only occurrence without enough semantic data to identify the symbol.
    Unknown,
}

/// One symbol occurrence in a source file.
///
/// These occurrences feed both semantic highlighting and the reusable semantic
/// index that later navigation features build on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolOccurrence {
    /// File-relative source range for the occurrence.
    pub range: FileRange,
    /// Stable identity shared with related declarations or references.
    pub identity: SymbolIdentity,
    /// Whether this occurrence is a declaration or reference.
    pub role: OccurrenceRole,
    /// Internal token classification before LSP encoding.
    pub token_kind: SemanticKind,
    /// Whether the occurrence should also carry the readonly modifier.
    pub readonly: bool,
}

/// One highlighting-only token in a source file.
///
/// Lexical tokens such as comments and operators should improve editor
/// coverage without becoming part of the symbol index used by navigation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticTokenOccurrence {
    /// File-relative source range for the token.
    pub range: FileRange,
    /// Internal token classification before LSP encoding.
    pub token_kind: SemanticKind,
    /// Whether this token is a declaration or reference.
    pub role: OccurrenceRole,
    /// Whether the token should also carry the readonly modifier.
    pub readonly: bool,
}

impl SemanticTokenOccurrence {
    /// Convert a symbol occurrence into its highlighting representation.
    pub fn from_symbol(occurrence: &SymbolOccurrence) -> Self {
        Self {
            range: occurrence.range.clone(),
            token_kind: occurrence.token_kind,
            role: occurrence.role,
            readonly: occurrence.readonly,
        }
    }
}

/// Plain-Rust semantic index cached on the main thread for later features.
#[derive(Debug, Clone, Default)]
pub struct SemanticIndex {
    /// All semantic occurrences known for the document generation.
    #[allow(dead_code)]
    pub occurrences: Vec<SymbolOccurrence>,
}

/// Cached semantic-token payload and reusable semantic index for one document generation.
#[derive(Debug, Clone)]
pub struct SemanticSnapshot {
    /// LSP wire-format token data ready to return to the client.
    pub encoded_tokens: Arc<[u32]>,
    /// Rich occurrence data kept for future semantic navigation features.
    #[allow(dead_code)]
    pub index: Arc<SemanticIndex>,
    /// Whether the snapshot is syntax-only or compiler-enhanced.
    #[allow(dead_code)]
    pub source: SemanticSource,
}

/// Sort occurrences into stable file-relative source order.
///
/// The sort stays stable so callers can order preferred duplicates ahead of
/// fallback ones before deduplicating equal ranges.
pub(crate) fn sort_occurrences(occurrences: &mut [SymbolOccurrence]) {
    occurrences.sort_by(|left, right| {
        left.range
            .path
            .cmp(&right.range.path)
            .then_with(|| left.range.start.cmp(&right.range.start))
            .then_with(|| left.range.end.cmp(&right.range.end))
    });
}

/// Sort semantic-token occurrences into stable file-relative source order.
pub(crate) fn sort_token_occurrences(tokens: &mut [SemanticTokenOccurrence]) {
    tokens.sort_by(|left, right| {
        left.range
            .path
            .cmp(&right.range.path)
            .then_with(|| left.range.start.cmp(&right.range.start))
            .then_with(|| left.range.end.cmp(&right.range.end))
    });
}

/// Merge syntax-only and compiler-backed occurrences, preferring compiler
/// truth on exact range conflicts.
///
/// The merge consumes both vectors because the caller already owns them and the
/// result must be owned anyway. Borrowing slices here would force the merge to
/// clone every surviving occurrence into the returned buffer.
pub fn merge_occurrences(
    mut syntax_occurrences: Vec<SymbolOccurrence>,
    mut compiler_occurrences: Vec<SymbolOccurrence>,
) -> Vec<SymbolOccurrence> {
    if syntax_occurrences.is_empty() {
        sort_occurrences(&mut compiler_occurrences);
        return compiler_occurrences;
    }

    if compiler_occurrences.is_empty() {
        sort_occurrences(&mut syntax_occurrences);
        return syntax_occurrences;
    }

    compiler_occurrences.reserve(syntax_occurrences.len());
    compiler_occurrences.append(&mut syntax_occurrences);
    sort_occurrences(&mut compiler_occurrences);

    // Stable sorting keeps compiler occurrences ahead of syntax occurrences for
    // exact range ties, so deduplication preserves compiler-backed semantics.
    compiler_occurrences.dedup_by(|left, right| left.range == right.range);
    compiler_occurrences
}

#[cfg(test)]
mod tests {
    use super::{FileRange, OccurrenceRole, SemanticKind, SymbolIdentity, SymbolOccurrence, merge_occurrences};
    use std::{path::PathBuf, sync::Arc};

    fn occurrence(path: &Arc<PathBuf>, start: u32, end: u32, token_kind: SemanticKind) -> SymbolOccurrence {
        SymbolOccurrence {
            range: FileRange::new(Arc::clone(path), start, end).expect("non-empty range"),
            identity: SymbolIdentity::Unknown,
            role: OccurrenceRole::Reference,
            token_kind,
            readonly: false,
        }
    }

    #[test]
    fn merge_occurrences_prefers_compiler_occurrences_on_exact_range_conflicts() {
        let path = Arc::new(PathBuf::from("main.leo"));
        let syntax_occurrences = vec![occurrence(&path, 4, 9, SemanticKind::Variable)];
        let mut compiler_occurrence = occurrence(&path, 4, 9, SemanticKind::Function);
        compiler_occurrence.role = OccurrenceRole::Declaration;
        compiler_occurrence.readonly = true;

        let merged = merge_occurrences(syntax_occurrences, vec![compiler_occurrence]);

        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].token_kind, SemanticKind::Function);
        assert_eq!(merged[0].role, OccurrenceRole::Declaration);
        assert!(merged[0].readonly);
    }

    #[test]
    fn merge_occurrences_returns_source_ordered_output() {
        let a_path = Arc::new(PathBuf::from("a.leo"));
        let b_path = Arc::new(PathBuf::from("b.leo"));
        let merged = merge_occurrences(
            vec![
                occurrence(&b_path, 8, 10, SemanticKind::Variable),
                occurrence(&a_path, 9, 11, SemanticKind::Variable),
            ],
            vec![occurrence(&b_path, 1, 3, SemanticKind::Function), occurrence(&a_path, 2, 4, SemanticKind::Function)],
        );

        let starts = merged
            .iter()
            .map(|occurrence| (occurrence.range.path.as_ref().clone(), occurrence.range.start))
            .collect::<Vec<_>>();
        assert_eq!(starts, vec![
            (PathBuf::from("a.leo"), 2),
            (PathBuf::from("a.leo"), 9),
            (PathBuf::from("b.leo"), 1),
            (PathBuf::from("b.leo"), 8),
        ]);
    }
}
