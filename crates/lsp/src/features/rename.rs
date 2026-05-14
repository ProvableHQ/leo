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

//! Rename and prepare-rename resolution for Leo LSP.
//!
//! This module deliberately stops at compact byte ranges and per-file rename
//! eligibility. It does not know about JSON-RPC request IDs, worker scheduling,
//! disk reads, or `WorkspaceEdit` materialization — those concerns live in the
//! response pool worker. The validator delegates to the canonical Leo lexer so
//! adding a keyword in the parser automatically extends rename rejection.

use crate::{
    document_store::DocumentViewKey,
    semantics::{CachedPackageAnalysis, CompactRange, FileId, SourceFingerprint},
};
use leo_parser_rowan::{SyntaxKind, lex};
use lsp_types::Position;
use std::path::Path;

/// Cursor query captured before any async package-analysis wait.
#[derive(Debug, Clone)]
pub struct RenameQuery {
    /// UTF-8 byte offset resolved from the original LSP position.
    pub offset: u32,
    /// Original LSP position retained for diagnostics.
    #[allow(dead_code)]
    pub position: Position,
    /// Freshness key for the document view active when the request arrived.
    pub view_key: DocumentViewKey,
    /// Validated new identifier text the client requested.
    pub new_name: String,
}

/// Cursor query captured for prepare-rename, mirroring `RenameQuery` minus the new name.
#[derive(Debug, Clone)]
pub struct PrepareRenameQuery {
    /// UTF-8 byte offset resolved from the original LSP position.
    pub offset: u32,
    /// Original LSP position retained for diagnostics.
    #[allow(dead_code)]
    pub position: Position,
    /// Freshness key for the document view active when the request arrived.
    pub view_key: DocumentViewKey,
}

/// Compact rename edit before any LSP-range or URI conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenameTarget {
    /// Interned analyzed-file ID this edit lives in.
    pub file: FileId,
    /// Compact byte range to be replaced with the validated new name.
    pub range: CompactRange,
}

/// Structured rename failure surfaced to the routing thread.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenameError {
    /// The new name is not a single Leo identifier token.
    InvalidIdentifier(String),
    /// The cursor occurrence is not eligible for rename (`null` reply).
    NotRenameable,
    /// A disk-backed source file changed between analysis and materialization.
    /// Constructed by the response-pool worker after `read_verified_disk_text`.
    #[allow(dead_code)]
    SourceChanged { path: String },
    /// At least one occurrence resolves to a file outside the current package.
    LeavesPackage { path: String },
    /// At least one occurrence resolves to a volatile-fingerprint file.
    VolatileSource { path: String },
}

/// Validate that `name` lexes as exactly one Leo identifier token.
///
/// The validator delegates to the canonical Leo lexer so keyword and
/// identifier shape rules track the parser without a parallel keyword table.
pub fn validate_new_name(name: &str) -> Result<(), RenameError> {
    let (tokens, errors) = lex(name);
    if !errors.is_empty() {
        return Err(RenameError::InvalidIdentifier(
            "contains characters that are not part of any Leo token".to_owned(),
        ));
    }
    let payload: Vec<&_> =
        tokens.iter().filter(|token| !token.kind.is_trivia() && token.kind != SyntaxKind::EOF).collect();
    match payload.as_slice() {
        [token] if token.kind == SyntaxKind::IDENT => Ok(()),
        [token] if token.kind.is_keyword() => Err(RenameError::InvalidIdentifier(format!("'{name}' is a Leo keyword"))),
        [] => Err(RenameError::InvalidIdentifier("name is empty".to_owned())),
        [_] => Err(RenameError::InvalidIdentifier("name is not a Leo identifier".to_owned())),
        _ => Err(RenameError::InvalidIdentifier("name is more than one Leo token".to_owned())),
    }
}

/// Resolve every package-internal occurrence sharing the cursor's symbol key.
///
/// Returns `Err(NotRenameable)` for cursors that should produce a `null` LSP
/// reply (no occurrence, no key, or a `Program` identity). Returns the other
/// error variants when a per-file invariant rejects the whole rename.
pub fn resolve_targets(
    query: &RenameQuery,
    package: &CachedPackageAnalysis,
    source_path: &Path,
) -> Result<Vec<RenameTarget>, RenameError> {
    let _ = query;
    let occurrence = package.index.occurrence_at(source_path, query.offset).ok_or(RenameError::NotRenameable)?;
    let key_id = occurrence.occurrence.key_id().ok_or(RenameError::NotRenameable)?;
    if package.index.is_program_key(key_id) {
        return Err(RenameError::NotRenameable);
    }

    let mut targets = Vec::new();
    let mut current_file: Option<FileId> = None;
    for occurrence in package.index.occurrences_for(key_id) {
        let file_id = occurrence.range.file;
        if Some(file_id) != current_file {
            let analyzed = package
                .analyzed_files
                .get(file_id)
                .ok_or_else(|| RenameError::LeavesPackage { path: format!("<unknown file id {file_id}>") })?;
            if analyzed.is_sentinel || !analyzed.in_package_scope {
                return Err(RenameError::LeavesPackage { path: analyzed.path.display().to_string() });
            }
            if matches!(analyzed.fingerprint, SourceFingerprint::Volatile) {
                return Err(RenameError::VolatileSource { path: analyzed.path.display().to_string() });
            }
            current_file = Some(file_id);
        }
        targets.push(RenameTarget { file: file_id, range: occurrence.range });
    }
    Ok(targets)
}

/// Return the compact byte range of the renameable cursor occurrence.
pub fn prepare_rename_target(
    query: &PrepareRenameQuery,
    package: &CachedPackageAnalysis,
    source_path: &Path,
) -> Option<CompactRange> {
    let occurrence = package.index.occurrence_at(source_path, query.offset)?;
    let key_id = occurrence.occurrence.key_id()?;
    if package.index.is_program_key(key_id) {
        return None;
    }
    Some(occurrence.occurrence.range)
}

#[cfg(test)]
mod tests {
    use super::{
        PrepareRenameQuery,
        RenameError,
        RenameQuery,
        prepare_rename_target,
        resolve_targets,
        validate_new_name,
    };
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
    use leo_span::{Symbol, create_session_if_not_set_then};
    use lsp_types::{Position, Uri};
    use std::{path::PathBuf, sync::Arc};

    /// Build an unmanaged-document view key for synthetic rename-resolution tests.
    fn document_view_key(uri: &Uri) -> DocumentViewKey {
        let bucket = AnalysisBucket::UnmanagedDocument { uri: uri.clone() };
        DocumentViewKey {
            uri: uri.clone(),
            document_generation: 1,
            package: PackageAnalysisKey { bucket, bucket_generation: 1 },
        }
    }

    /// Build a rename query at the given cursor offset with the given new name.
    fn rename_query(offset: u32, view_key: DocumentViewKey, new_name: &str) -> RenameQuery {
        RenameQuery { offset, position: Position::new(0, 0), view_key, new_name: new_name.to_owned() }
    }

    /// Build a prepare-rename query at the given cursor offset.
    fn prepare_query(offset: u32, view_key: DocumentViewKey) -> PrepareRenameQuery {
        PrepareRenameQuery { offset, position: Position::new(0, 0), view_key }
    }

    /// Build a keyed local occurrence for rename-resolution unit tests.
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

    /// Build a package analysis from synthetic occurrences with a custom in-scope predicate.
    fn build_package(
        occurrences: Vec<SymbolOccurrence>,
        in_scope: impl Fn(&std::path::Path) -> bool + Copy,
    ) -> CachedPackageAnalysis {
        let uri: Uri = "untitled:main.leo".parse().expect("uri");
        let key = PackageAnalysisKey { bucket: AnalysisBucket::UnmanagedDocument { uri }, bucket_generation: 1 };
        let (index, analyzed_files) = SemanticIndex::build(
            &occurrences,
            |_| SourceFingerprint::Disk { modified_nanos: Some(0), len: 0, content_hash: 0 },
            |_| None,
            |path| in_scope(path),
        );
        CachedPackageAnalysis {
            key: key.clone(),
            index: Arc::new(index),
            analyzed_files: Arc::new(analyzed_files),
            source: SemanticSource::CompilerEnhanced,
            diagnostics: Arc::new(crate::features::diagnostics::DiagnosticSet::empty(key)),
        }
    }

    /// Verifies validate_new_name accepts simple identifiers.
    #[test]
    fn validate_new_name_accepts_simple_identifiers() {
        for name in ["x", "total", "Foo123", "_inner"] {
            let result = validate_new_name(name);
            assert!(result.is_ok(), "{name} should be accepted: {result:?}");
        }
    }

    /// Verifies validate_new_name rejects empty, digit-prefixed, and punctuation names.
    #[test]
    fn validate_new_name_rejects_invalid_shapes() {
        assert!(matches!(validate_new_name(""), Err(RenameError::InvalidIdentifier(_))));
        assert!(matches!(validate_new_name("1abc"), Err(RenameError::InvalidIdentifier(_))));
        assert!(matches!(validate_new_name("foo bar"), Err(RenameError::InvalidIdentifier(_))));
        assert!(matches!(validate_new_name("foo-bar"), Err(RenameError::InvalidIdentifier(_))));
        assert!(matches!(validate_new_name("foo.bar"), Err(RenameError::InvalidIdentifier(_))));
    }

    /// Verifies validate_new_name rejects every Leo keyword by name.
    #[test]
    fn validate_new_name_rejects_keywords() {
        for keyword in ["let", "fn", "program", "if", "else", "return", "for", "true", "false"] {
            let result = validate_new_name(keyword);
            assert!(matches!(result, Err(RenameError::InvalidIdentifier(_))), "{keyword} should be rejected");
        }
    }

    /// Verifies resolve_targets returns NotRenameable when the cursor misses every occurrence.
    #[test]
    fn resolve_targets_rejects_unknown_cursor() {
        create_session_if_not_set_then(|_| {
            let path = Arc::new(PathBuf::from("/pkg/src/main.leo"));
            let declaration = FileRange::new(Arc::clone(&path), 1, 6).expect("declaration");
            let package =
                build_package(vec![local_occurrence(&path, 1, 6, &declaration, OccurrenceRole::Declaration)], |_| true);
            let view_key = document_view_key(&"untitled:main.leo".parse().unwrap());
            let result = resolve_targets(&rename_query(50, view_key, "renamed"), &package, path.as_ref());
            assert_eq!(result, Err(RenameError::NotRenameable));
        });
    }

    /// Verifies resolve_targets returns every package-internal occurrence for a Local key.
    #[test]
    fn resolve_targets_returns_local_occurrences() {
        create_session_if_not_set_then(|_| {
            let path = Arc::new(PathBuf::from("/pkg/src/main.leo"));
            let declaration = FileRange::new(Arc::clone(&path), 1, 6).expect("declaration");
            let package = build_package(
                vec![
                    local_occurrence(&path, 1, 6, &declaration, OccurrenceRole::Declaration),
                    local_occurrence(&path, 10, 15, &declaration, OccurrenceRole::Reference),
                    local_occurrence(&path, 20, 25, &declaration, OccurrenceRole::Reference),
                ],
                |_| true,
            );
            let view_key = document_view_key(&"untitled:main.leo".parse().unwrap());
            let targets = resolve_targets(&rename_query(12, view_key, "renamed"), &package, path.as_ref())
                .expect("eligible local");
            let starts = targets.iter().map(|target| target.range.start).collect::<Vec<_>>();
            assert_eq!(starts, vec![1, 10, 20]);
        });
    }

    /// Verifies resolve_targets rejects cursors whose key is the Program variant.
    #[test]
    fn resolve_targets_rejects_program_cursor() {
        create_session_if_not_set_then(|_| {
            let path = Arc::new(PathBuf::from("/pkg/src/main.leo"));
            let package_root = Arc::new(PathBuf::from("/pkg"));
            let occurrence = SymbolOccurrence {
                range: FileRange::new(Arc::clone(&path), 1, 9).expect("range"),
                identity: SymbolIdentity::Program {
                    program: Symbol::intern("hello"),
                    package_root: Some(Arc::clone(&package_root)),
                    declaration: None,
                },
                role: OccurrenceRole::Declaration,
                token_kind: SemanticKind::Namespace,
                readonly: true,
            };
            let package = build_package(vec![occurrence], |_| true);
            let view_key = document_view_key(&"untitled:main.leo".parse().unwrap());
            let result = resolve_targets(&rename_query(3, view_key, "world"), &package, path.as_ref());
            assert_eq!(result, Err(RenameError::NotRenameable));
        });
    }

    /// Verifies resolve_targets rejects renames that touch out-of-package files.
    #[test]
    fn resolve_targets_rejects_out_of_package_target() {
        create_session_if_not_set_then(|_| {
            let in_path = Arc::new(PathBuf::from("/pkg/src/main.leo"));
            let stub_path = Arc::new(PathBuf::from("/dep/src/lib.leo"));
            let declaration = FileRange::new(Arc::clone(&in_path), 1, 6).expect("declaration");
            let package = build_package(
                vec![
                    local_occurrence(&in_path, 1, 6, &declaration, OccurrenceRole::Declaration),
                    local_occurrence(&stub_path, 30, 35, &declaration, OccurrenceRole::Reference),
                ],
                |path| path == std::path::Path::new("/pkg/src/main.leo"),
            );
            let view_key = document_view_key(&"untitled:main.leo".parse().unwrap());
            let result = resolve_targets(&rename_query(3, view_key, "renamed"), &package, in_path.as_ref());
            assert!(matches!(result, Err(RenameError::LeavesPackage { .. })), "{result:?}");
        });
    }

    /// Verifies prepare_rename_target returns the cursor occurrence range for renameable keys.
    #[test]
    fn prepare_rename_target_returns_cursor_range() {
        create_session_if_not_set_then(|_| {
            let path = Arc::new(PathBuf::from("/pkg/src/main.leo"));
            let declaration = FileRange::new(Arc::clone(&path), 1, 6).expect("declaration");
            let package = build_package(
                vec![
                    local_occurrence(&path, 1, 6, &declaration, OccurrenceRole::Declaration),
                    local_occurrence(&path, 10, 15, &declaration, OccurrenceRole::Reference),
                ],
                |_| true,
            );
            let view_key = document_view_key(&"untitled:main.leo".parse().unwrap());
            let range = prepare_rename_target(&prepare_query(11, view_key), &package, path.as_ref())
                .expect("renameable cursor");
            assert_eq!((range.start, range.end), (10, 15));
        });
    }

    /// Verifies prepare_rename_target returns None for cursor positions in whitespace.
    #[test]
    fn prepare_rename_target_returns_none_in_whitespace() {
        create_session_if_not_set_then(|_| {
            let path = Arc::new(PathBuf::from("/pkg/src/main.leo"));
            let declaration = FileRange::new(Arc::clone(&path), 1, 6).expect("declaration");
            let package =
                build_package(vec![local_occurrence(&path, 1, 6, &declaration, OccurrenceRole::Declaration)], |_| true);
            let view_key = document_view_key(&"untitled:main.leo".parse().unwrap());
            assert!(prepare_rename_target(&prepare_query(50, view_key), &package, path.as_ref()).is_none());
        });
    }
}
