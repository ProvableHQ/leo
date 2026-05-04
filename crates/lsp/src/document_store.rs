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

#![allow(clippy::mutable_key_type)]

use crate::project_model::ProjectContext;
use line_index::LineIndex;
use lsp_types::Uri;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

/// Package-sized freshness bucket for semantic analysis.
///
/// Managed documents share a bucket by canonical package root so changing one
/// open sibling invalidates package analysis for every open file in that
/// package. Unmanaged buffers keep URI-local buckets because they have no
/// package graph to share.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnalysisBucket {
    /// A Leo package rooted at a canonical `program.json` directory.
    ManagedPackage { package_root: Arc<PathBuf> },
    /// A scratch or non-file document with no resolved package context.
    UnmanagedDocument { uri: Uri },
}

/// Freshness key for package-level semantic analysis.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackageAnalysisKey {
    /// Package or unmanaged-document bucket being analyzed.
    pub bucket: AnalysisBucket,
    /// Monotonic generation for all open-buffer inputs in the bucket.
    pub bucket_generation: u64,
}

/// Freshness key for one document's encoded semantic-token view.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocumentViewKey {
    /// Open document URI for the encoded view.
    pub uri: Uri,
    /// Per-document generation used for position and lexical-token freshness.
    pub document_generation: u64,
    /// Package analysis this document view merges against.
    pub package: PackageAnalysisKey,
}

/// Open-buffer overlay supplied to compiler package analysis.
#[derive(Debug, Clone)]
pub struct OpenFileOverlay {
    /// Native path used by compiler file-source lookups.
    pub path: Arc<PathBuf>,
    /// Current committed editor text.
    pub text: Arc<str>,
    /// Line index for range conversion against this exact text.
    pub line_index: Arc<LineIndex>,
}

/// Committed state for an open Leo document tracked by the server.
///
/// The `generation` is monotonically increasing per document URI, including
/// reopen-after-close flows, so a stale worker result can never be mistaken for
/// the latest committed state. The shared `cancel_token` always stores the most
/// recent committed generation for that URI.
#[derive(Debug)]
pub struct OpenDocument {
    pub language_id: Arc<str>,
    pub text: Arc<str>,
    pub line_index: Arc<LineIndex>,
    pub version: i32,
    pub generation: u64,
    pub file_path: Option<Arc<PathBuf>>,
    pub project: Option<Arc<ProjectContext>>,
    /// Package-sized invalidation bucket this open document contributes to.
    pub analysis_bucket: AnalysisBucket,
    pub cancel_token: Arc<AtomicU64>,
}

/// Owned document state passed to the background worker.
///
/// Worker jobs must treat the snapshot as stale once `cancel_token` stops
/// matching `generation`, even if the URI still refers to an open document.
#[derive(Debug, Clone)]
pub struct DocumentSnapshot {
    pub uri: Uri,
    pub text: Arc<str>,
    #[allow(dead_code)]
    pub line_index: Arc<LineIndex>,
    #[allow(dead_code)]
    pub version: i32,
    pub generation: u64,
    #[allow(dead_code)]
    pub file_path: Option<Arc<PathBuf>>,
    #[allow(dead_code)]
    pub project: Option<Arc<ProjectContext>>,
    /// Package-analysis key for this snapshot's package-sized inputs.
    pub package_key: PackageAnalysisKey,
    /// Document-view key for the trigger document.
    pub view_key: DocumentViewKey,
    /// Same-package open buffers visible to compiler analysis.
    pub open_overlays: Arc<[OpenFileOverlay]>,
    pub cancel_token: Arc<AtomicU64>,
}

/// Document-sized worker input for rebuilding one semantic-token view.
#[derive(Debug, Clone)]
pub struct DocumentViewSnapshot {
    pub key: DocumentViewKey,
    pub uri: Uri,
    pub text: Arc<str>,
    pub line_index: Arc<LineIndex>,
    pub file_path: Option<Arc<PathBuf>>,
    pub project: Option<Arc<ProjectContext>>,
    pub cancel_token: Arc<AtomicU64>,
}

/// Fully prepared document mutation ready to be committed atomically.
///
/// Preparing a mutation never changes visible server state. Only the
/// `commit_*` methods publish the new generation and update cancellation state.
#[derive(Debug)]
pub struct PreparedDocument {
    uri: Uri,
    document: OpenDocument,
}

/// Store of currently open Leo documents.
///
/// The store also remembers the latest generation seen for each URI after a
/// close so a later reopen cannot reuse an older generation number.
#[derive(Debug, Default)]
pub struct DocumentStore {
    documents: HashMap<Uri, OpenDocument>,
    latest_generation: HashMap<Uri, u64>,
    bucket_generations: HashMap<AnalysisBucket, u64>,
}

impl DocumentStore {
    /// Prepare a newly opened document without mutating committed state.
    ///
    /// Reopens continue the URI's generation sequence instead of resetting to
    /// `1`, which preserves the cancellation invariant across close/reopen
    /// cycles and duplicate opens.
    pub fn prepare_open(
        &self,
        uri: Uri,
        language_id: String,
        version: i32,
        text: String,
        file_path: Option<Arc<PathBuf>>,
        project: Option<Arc<ProjectContext>>,
    ) -> PreparedDocument {
        let generation = next_generation(self.latest_generation.get(&uri).copied().unwrap_or_default());
        let language_id: Arc<str> = Arc::from(language_id);
        let text: Arc<str> = Arc::from(text);
        let line_index = Arc::new(LineIndex::new(text.as_ref()));
        let cancel_token = Arc::new(AtomicU64::new(generation));
        let analysis_bucket = analysis_bucket_for(&uri, project.as_ref());

        let document = OpenDocument {
            language_id,
            text,
            line_index,
            version,
            generation,
            file_path,
            project,
            analysis_bucket,
            cancel_token,
        };

        PreparedDocument { uri, document }
    }

    /// Commit a prepared open-document mutation and return the worker snapshot.
    ///
    /// If the URI was already open, the previous document's in-flight work is
    /// invalidated before the new snapshot becomes current.
    pub fn commit_open(&mut self, prepared: PreparedDocument) -> DocumentSnapshot {
        let (uri, document) = prepared.into_parts();
        if let Some(previous) = self.documents.insert(uri.clone(), document) {
            previous.cancel_token.store(next_generation(previous.generation), Ordering::SeqCst);
            self.bump_bucket(&previous.analysis_bucket);
        }

        let bucket = self.documents.get(&uri).expect("document just inserted").analysis_bucket.clone();
        self.bump_bucket(&bucket);
        let snapshot = self.snapshot_for_package_analysis(&uri).expect("document just inserted");
        self.latest_generation.insert(uri, snapshot.generation);
        snapshot
    }

    /// Prepare a full-document replacement for an already open document.
    ///
    /// The prepared change reuses the existing cancellation token so commit can
    /// advance the same per-URI generation stream. Callers may also refresh the
    /// file and project context alongside the new text so package discovery can
    /// react to manifests appearing or changing mid-session.
    pub fn prepare_full_change(
        &self,
        uri: &Uri,
        version: i32,
        text: String,
        file_path: Option<Arc<PathBuf>>,
        project: Option<Arc<ProjectContext>>,
    ) -> Option<PreparedDocument> {
        let current = self.documents.get(uri)?;
        let text: Arc<str> = Arc::from(text);
        let line_index = Arc::new(LineIndex::new(text.as_ref()));
        let generation = next_generation(current.generation);
        let analysis_bucket = analysis_bucket_for(uri, project.as_ref());

        let document = OpenDocument {
            language_id: current.language_id.clone(),
            text,
            line_index,
            version,
            generation,
            file_path,
            project,
            analysis_bucket,
            cancel_token: Arc::clone(&current.cancel_token),
        };

        Some(PreparedDocument { uri: uri.clone(), document })
    }

    /// Commit a prepared full-document change and return the worker snapshot.
    ///
    /// Committing updates both the visible document state and the shared cancel
    /// token so older snapshots become stale immediately.
    pub fn commit_change(&mut self, prepared: PreparedDocument) -> DocumentSnapshot {
        let (uri, document) = prepared.into_parts();
        document.cancel_token.store(document.generation, Ordering::SeqCst);
        if let Some(previous) = self.documents.insert(uri.clone(), document)
            && previous.analysis_bucket != self.documents.get(&uri).expect("document just inserted").analysis_bucket
        {
            self.bump_bucket(&previous.analysis_bucket);
        }
        let bucket = self.documents.get(&uri).expect("document just inserted").analysis_bucket.clone();
        self.bump_bucket(&bucket);
        let snapshot = self.snapshot_for_package_analysis(&uri).expect("document just inserted");
        self.latest_generation.insert(uri, snapshot.generation);
        snapshot
    }

    /// Close an open document and invalidate any in-flight work for it.
    ///
    /// Generation bookkeeping is retained after close so a future reopen cannot
    /// reuse the closed document's generation number.
    pub fn close(&mut self, uri: &Uri) -> bool {
        let Some(document) = self.documents.remove(uri) else {
            return false;
        };

        document.cancel_token.store(next_generation(document.generation), Ordering::SeqCst);
        self.bump_bucket(&document.analysis_bucket);
        true
    }

    /// Return the latest committed generation for the given open document URI.
    ///
    /// Closed documents return `None` even though their last generation remains
    /// tracked internally for future reopen handling.
    pub fn generation(&self, uri: &Uri) -> Option<u64> {
        self.documents.get(uri).map(|document| document.generation)
    }

    /// Return the current package-analysis key for an open document.
    pub fn package_key(&self, uri: &Uri) -> Option<PackageAnalysisKey> {
        let document = self.documents.get(uri)?;
        Some(self.package_key_for_bucket(&document.analysis_bucket))
    }

    /// Return the current document-view key for an open document.
    pub fn document_view_key(&self, uri: &Uri) -> Option<DocumentViewKey> {
        let document = self.documents.get(uri)?;
        Some(DocumentViewKey {
            uri: uri.clone(),
            document_generation: document.generation,
            package: self.package_key_for_bucket(&document.analysis_bucket),
        })
    }

    /// Return the committed document for main-thread request handling.
    pub fn open_document(&self, uri: &Uri) -> Option<&OpenDocument> {
        self.documents.get(uri)
    }

    /// Build the latest package-analysis snapshot for an open document.
    pub fn snapshot_for_package_analysis(&self, uri: &Uri) -> Option<DocumentSnapshot> {
        let document = self.documents.get(uri)?;
        Some(document.snapshot(
            uri.clone(),
            self.package_key_for_bucket(&document.analysis_bucket),
            self.open_overlays(&document.analysis_bucket),
        ))
    }

    /// Build a document-sized snapshot for a semantic-token view rebuild.
    pub fn snapshot_for_document_view(&self, uri: &Uri) -> Option<DocumentViewSnapshot> {
        let document = self.documents.get(uri)?;
        Some(DocumentViewSnapshot {
            key: self.document_view_key(uri)?,
            uri: uri.clone(),
            text: Arc::clone(&document.text),
            line_index: Arc::clone(&document.line_index),
            file_path: document.file_path.clone(),
            project: document.project.clone(),
            cancel_token: Arc::clone(&document.cancel_token),
        })
    }

    /// Return the line index for an open path captured by current document state.
    #[allow(dead_code)]
    pub fn open_line_index_for_path(&self, path: &Path) -> Option<Arc<LineIndex>> {
        self.documents.values().find_map(|document| {
            document
                .file_path
                .as_deref()
                .is_some_and(|candidate| candidate.as_path() == path)
                .then(|| Arc::clone(&document.line_index))
        })
    }

    /// Return currently open buckets for worker-side cache eviction.
    pub fn open_buckets(&self) -> HashSet<AnalysisBucket> {
        self.documents.values().map(|document| document.analysis_bucket.clone()).collect()
    }

    /// Return the committed document for tests.
    #[cfg(test)]
    pub fn get(&self, uri: &Uri) -> Option<&OpenDocument> {
        self.documents.get(uri)
    }
}

impl OpenDocument {
    /// Capture the current open document plus package context for worker analysis.
    fn snapshot(
        &self,
        uri: Uri,
        package_key: PackageAnalysisKey,
        open_overlays: Arc<[OpenFileOverlay]>,
    ) -> DocumentSnapshot {
        let view_key =
            DocumentViewKey { uri: uri.clone(), document_generation: self.generation, package: package_key.clone() };

        DocumentSnapshot {
            uri,
            text: Arc::clone(&self.text),
            line_index: Arc::clone(&self.line_index),
            version: self.version,
            generation: self.generation,
            file_path: self.file_path.clone(),
            project: self.project.clone(),
            package_key,
            view_key,
            open_overlays,
            cancel_token: Arc::clone(&self.cancel_token),
        }
    }
}

impl PreparedDocument {
    /// Split a prepared mutation into its target URI and committed document state.
    fn into_parts(self) -> (Uri, OpenDocument) {
        let Self { uri, document } = self;
        (uri, document)
    }
}

impl DocumentStore {
    /// Build the package-analysis freshness key for a bucket.
    fn package_key_for_bucket(&self, bucket: &AnalysisBucket) -> PackageAnalysisKey {
        PackageAnalysisKey {
            bucket: bucket.clone(),
            bucket_generation: self.bucket_generations.get(bucket).copied().unwrap_or_default(),
        }
    }

    /// Advance a bucket generation after any open-buffer input changes.
    fn bump_bucket(&mut self, bucket: &AnalysisBucket) {
        let generation = self.bucket_generations.get(bucket).copied().unwrap_or_default();
        self.bucket_generations.insert(bucket.clone(), next_generation(generation));
    }

    /// Collect open same-bucket buffers that compiler package analysis may read.
    fn open_overlays(&self, bucket: &AnalysisBucket) -> Arc<[OpenFileOverlay]> {
        let overlays = self
            .documents
            .values()
            .filter_map(|document| {
                if &document.analysis_bucket != bucket {
                    return None;
                }

                let path = document.file_path.as_ref()?;
                if let AnalysisBucket::ManagedPackage { .. } = bucket
                    && let Some(project) = document.project.as_ref()
                    && !path.starts_with(project.source_directory.as_ref())
                {
                    // Managed package analysis compiles from the package source
                    // root. Open files outside `src` share the package bucket
                    // for invalidation, but they must not be offered to the
                    // compiler as module overlays.
                    return None;
                }

                Some(OpenFileOverlay {
                    path: Arc::clone(path),
                    text: Arc::clone(&document.text),
                    line_index: Arc::clone(&document.line_index),
                })
            })
            .collect::<Vec<_>>();
        Arc::from(overlays)
    }
}

/// Choose the invalidation bucket for a document based on project discovery.
fn analysis_bucket_for(uri: &Uri, project: Option<&Arc<ProjectContext>>) -> AnalysisBucket {
    match project {
        Some(project) => AnalysisBucket::ManagedPackage { package_root: Arc::clone(&project.package_root) },
        None => AnalysisBucket::UnmanagedDocument { uri: uri.clone() },
    }
}

/// Return the next monotonic generation, panicking rather than wrapping.
fn next_generation(generation: u64) -> u64 {
    // Generation reuse would break stale-work detection, so overflow is treated
    // as a hard invariant violation rather than wrapping silently.
    generation.checked_add(1).expect("leo-lsp document generation overflow")
}

#[cfg(test)]
mod tests {
    use super::DocumentStore;
    use line_index::{TextSize, WideEncoding};
    use lsp_types::Uri;
    use std::sync::{Arc, atomic::Ordering};

    /// Return the canonical URI used by document-store unit tests.
    fn test_uri() -> Uri {
        "file:///tmp/main.leo".parse().expect("valid file uri")
    }

    /// Verifies full-sync edits replace text and advance document generations.
    #[test]
    fn full_sync_replaces_text_and_increments_generation() {
        let mut store = DocumentStore::default();
        let uri = test_uri();

        let opened = store.prepare_open(uri.clone(), "leo".to_owned(), 1, "hello".to_owned(), None, None);
        let first = store.commit_open(opened);

        let changed =
            store.prepare_full_change(&uri, 2, "goodbye".to_owned(), None, None).expect("document should be open");
        let second = store.commit_change(changed);

        assert_eq!(first.generation, 1);
        assert_eq!(second.generation, 2);
        assert_eq!(store.get(&uri).expect("open document").text.as_ref(), "goodbye");
        assert!(!Arc::ptr_eq(&first.line_index, &second.line_index));
    }

    /// Verifies prepared-but-uncommitted edits do not mutate visible state.
    #[test]
    fn dropping_prepared_change_preserves_committed_state() {
        let mut store = DocumentStore::default();
        let uri = test_uri();

        let opened = store.prepare_open(uri.clone(), "leo".to_owned(), 1, "hello".to_owned(), None, None);
        store.commit_open(opened);

        let _prepared =
            store.prepare_full_change(&uri, 2, "goodbye".to_owned(), None, None).expect("document should be open");

        let current = store.get(&uri).expect("open document");
        assert_eq!(current.generation, 1);
        assert_eq!(current.text.as_ref(), "hello");
        assert_eq!(current.cancel_token.load(Ordering::SeqCst), 1);
    }

    /// Verifies closing a document invalidates snapshots already sent to workers.
    #[test]
    fn close_invalidates_in_flight_work() {
        let mut store = DocumentStore::default();
        let uri = test_uri();

        let opened = store.prepare_open(uri.clone(), "leo".to_owned(), 1, "hello".to_owned(), None, None);
        let snapshot = store.commit_open(opened);

        assert!(store.close(&uri));
        assert_eq!(snapshot.cancel_token.load(Ordering::SeqCst), 2);
        assert!(store.generation(&uri).is_none());
    }

    /// Verifies reopening a URI cannot reuse its closed generation.
    #[test]
    fn reopen_after_close_advances_generation() {
        let mut store = DocumentStore::default();
        let uri = test_uri();

        let first =
            store.commit_open(store.prepare_open(uri.clone(), "leo".to_owned(), 1, "hello".to_owned(), None, None));
        assert!(store.close(&uri));

        let second =
            store.commit_open(store.prepare_open(uri.clone(), "leo".to_owned(), 2, "goodbye".to_owned(), None, None));

        assert_eq!(first.generation, 1);
        assert_eq!(second.generation, 2);
    }

    /// Verifies duplicate opens cancel work tied to the replaced document.
    #[test]
    fn duplicate_open_invalidates_previous_in_flight_work() {
        let mut store = DocumentStore::default();
        let uri = test_uri();

        let first =
            store.commit_open(store.prepare_open(uri.clone(), "leo".to_owned(), 1, "hello".to_owned(), None, None));
        let second =
            store.commit_open(store.prepare_open(uri.clone(), "leo".to_owned(), 2, "goodbye".to_owned(), None, None));

        assert_eq!(first.generation, 1);
        assert_eq!(second.generation, 2);
        assert_eq!(first.cancel_token.load(Ordering::SeqCst), 2);
        assert_eq!(second.cancel_token.load(Ordering::SeqCst), 2);
    }

    /// Verifies line indices preserve UTF-16 columns for multibyte text.
    #[test]
    fn line_index_tracks_utf16_columns_for_multibyte_text() {
        let mut store = DocumentStore::default();
        let uri = test_uri();

        let opened = store.prepare_open(uri, "leo".to_owned(), 1, "a\né\n𐐷x".to_owned(), None, None);
        let snapshot = store.commit_open(opened);

        let accent_utf8 = snapshot.line_index.line_col(TextSize::from(4));
        assert_eq!(accent_utf8.line, 1);
        assert_eq!(accent_utf8.col, 2);
        assert_eq!(snapshot.line_index.to_wide(WideEncoding::Utf16, accent_utf8).expect("utf16 column").col, 1);

        let astral_utf8 = snapshot.line_index.line_col(TextSize::from(9));
        assert_eq!(astral_utf8.line, 2);
        assert_eq!(astral_utf8.col, 4);
        assert_eq!(snapshot.line_index.to_wide(WideEncoding::Utf16, astral_utf8).expect("utf16 column").col, 2);
    }
}
