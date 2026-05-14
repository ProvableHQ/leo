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

//! Find-all-references resolution for Leo LSP.
//!
//! This module deliberately stops at compact byte ranges. It does not know
//! about JSON-RPC request IDs, worker scheduling, disk reads, or connection
//! writes; the server and response pool own those runtime concerns.

use crate::{
    document_store::DocumentViewKey,
    semantics::{CachedPackageAnalysis, CompactRange, OccurrenceRole},
};
use lsp_types::{Location, Position};
use serde_json::Value;
use std::path::Path;

/// Cursor query captured before any async package-analysis wait.
#[derive(Debug, Clone)]
pub struct ReferenceQuery {
    /// UTF-8 byte offset resolved from the original LSP position.
    pub offset: u32,
    /// Original LSP position retained for diagnostics and future link-style responses.
    #[allow(dead_code)]
    pub position: Position,
    /// Freshness key for the document view active when the request arrived.
    pub view_key: DocumentViewKey,
    /// Whether declaration occurrences should be included in the result.
    pub include_declaration: bool,
}

/// Feature-level compact result before LSP range conversion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceTargets {
    /// Whether the cursor was on a navigation-grade semantic occurrence.
    pub navigable: bool,
    /// Compact ranges for retained references, already in deterministic order.
    pub ranges: Vec<CompactRange>,
}

/// Resolve reference ranges for a cursor query against a fresh package analysis.
pub fn resolve_targets(
    query: &ReferenceQuery,
    package: &CachedPackageAnalysis,
    source_path: &Path,
) -> ReferenceTargets {
    let Some(occurrence) = package.index.occurrence_at(source_path, query.offset) else {
        return ReferenceTargets { navigable: false, ranges: Vec::new() };
    };
    let Some(key_id) = occurrence.occurrence.key_id() else {
        return ReferenceTargets { navigable: false, ranges: Vec::new() };
    };

    let mut ranges = package
        .index
        .occurrences_for(key_id)
        .filter(|occurrence| query.include_declaration || occurrence.role() != OccurrenceRole::Declaration)
        .map(|occurrence| occurrence.range)
        .collect::<Vec<_>>();
    if query.include_declaration {
        ranges.extend_from_slice(package.index.definitions_for(key_id));
        ranges.sort_unstable();
        ranges.dedup();
    }
    ReferenceTargets { navigable: true, ranges }
}

/// Serialize references into the standard LSP response payload.
pub fn response_value(navigable: bool, locations: Vec<Location>) -> Value {
    if navigable { serde_json::to_value(locations).expect("references should serialize") } else { Value::Null }
}

#[cfg(test)]
mod tests {
    use super::{ReferenceQuery, resolve_targets, response_value};
    use crate::{
        document_store::{AnalysisBucket, DocumentViewKey, PackageAnalysisKey},
        semantics::{
            CachedPackageAnalysis,
            FileRange,
            OccurrenceRole,
            SemanticIndex,
            SemanticKind,
            SemanticSource,
            SourceFingerprint,
            SymbolIdentity,
            SymbolOccurrence,
        },
    };
    use lsp_types::{Position, Uri};
    use serde_json::json;
    use std::{path::PathBuf, sync::Arc};

    /// Build a package analysis around the supplied synthetic occurrences.
    fn package(occurrences: Vec<SymbolOccurrence>) -> (CachedPackageAnalysis, DocumentViewKey, Arc<PathBuf>) {
        let uri: Uri = "untitled:main.leo".parse().expect("uri");
        let path = Arc::new(PathBuf::from("main.leo"));
        let key =
            PackageAnalysisKey { bucket: AnalysisBucket::UnmanagedDocument { uri: uri.clone() }, bucket_generation: 1 };
        let view_key = DocumentViewKey { uri, document_generation: 1, package: key.clone() };
        let (index, analyzed_files) =
            SemanticIndex::build(&occurrences, |_| SourceFingerprint::Volatile, |_| None, |_| true);
        (
            CachedPackageAnalysis {
                key: key.clone(),
                index: Arc::new(index),
                analyzed_files: Arc::new(analyzed_files),
                source: SemanticSource::CompilerEnhanced,
                diagnostics: Arc::new(crate::features::diagnostics::DiagnosticSet::empty(key)),
            },
            view_key,
            path,
        )
    }

    /// Build a keyed local occurrence for reference-resolution unit tests.
    fn local_occurrence(
        path: &Arc<PathBuf>,
        start: u32,
        end: u32,
        declaration: &FileRange,
        role: OccurrenceRole,
    ) -> SymbolOccurrence {
        SymbolOccurrence {
            range: FileRange::new(Arc::clone(path), start, end).expect("range"),
            identity: SymbolIdentity::Local { declaration: declaration.clone() },
            role,
            token_kind: SemanticKind::Variable,
            readonly: false,
        }
    }

    /// Build a references query with the requested cursor and declaration mode.
    fn query(view_key: DocumentViewKey, offset: u32, include_declaration: bool) -> ReferenceQuery {
        ReferenceQuery { offset, position: Position::new(0, 0), view_key, include_declaration }
    }

    /// Verifies reference resolution preserves source order and declaration filtering.
    #[test]
    fn resolve_targets_preserves_order_and_include_declaration() {
        let path = Arc::new(PathBuf::from("main.leo"));
        let declaration = FileRange::new(Arc::clone(&path), 1, 6).expect("declaration");
        let occurrences = vec![
            local_occurrence(&path, 20, 25, &declaration, OccurrenceRole::Reference),
            local_occurrence(&path, 1, 6, &declaration, OccurrenceRole::Declaration),
            local_occurrence(&path, 10, 15, &declaration, OccurrenceRole::Reference),
        ];
        let (package, view_key, source_path) = package(occurrences);

        let with_declaration = resolve_targets(&query(view_key.clone(), 12, true), &package, source_path.as_ref());
        assert!(with_declaration.navigable);
        assert_eq!(with_declaration.ranges.iter().map(|range| range.start).collect::<Vec<_>>(), vec![1, 10, 20]);

        let references_only = resolve_targets(&query(view_key, 12, false), &package, source_path.as_ref());
        assert!(references_only.navigable);
        assert_eq!(references_only.ranges.iter().map(|range| range.start).collect::<Vec<_>>(), vec![10, 20]);
    }

    /// Verifies navigable declaration-only symbols return an empty array, not `null`.
    #[test]
    fn resolve_targets_distinguishes_null_from_empty_array() {
        let path = Arc::new(PathBuf::from("main.leo"));
        let declaration = FileRange::new(Arc::clone(&path), 1, 6).expect("declaration");
        let (package, view_key, source_path) =
            package(vec![local_occurrence(&path, 1, 6, &declaration, OccurrenceRole::Declaration)]);

        let empty = resolve_targets(&query(view_key, 2, false), &package, source_path.as_ref());
        assert!(empty.navigable);
        assert!(empty.ranges.is_empty());
        assert_eq!(response_value(empty.navigable, Vec::new()), json!([]));

        let unknown = resolve_targets(&query(empty_query_key(), 50, true), &package, source_path.as_ref());
        assert!(!unknown.navigable);
        assert_eq!(response_value(unknown.navigable, Vec::new()), serde_json::Value::Null);
    }

    /// Build a package key for a query that intentionally misses the test package.
    fn empty_query_key() -> DocumentViewKey {
        let uri: Uri = "untitled:main.leo".parse().expect("uri");
        DocumentViewKey {
            uri: uri.clone(),
            document_generation: 1,
            package: PackageAnalysisKey { bucket: AnalysisBucket::UnmanagedDocument { uri }, bucket_generation: 1 },
        }
    }
}
