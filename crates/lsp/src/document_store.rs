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

use crate::project_model::ProjectContext;
use line_index::LineIndex;
use lsp_types::Uri;
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

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

        let document =
            OpenDocument { language_id, text, line_index, version, generation, file_path, project, cancel_token };

        PreparedDocument { uri, document }
    }

    /// Commit a prepared open-document mutation and return the worker snapshot.
    ///
    /// If the URI was already open, the previous document's in-flight work is
    /// invalidated before the new snapshot becomes current.
    pub fn commit_open(&mut self, prepared: PreparedDocument) -> DocumentSnapshot {
        let (uri, document, snapshot) = prepared.into_parts();
        if let Some(previous) = self.documents.insert(uri.clone(), document) {
            previous.cancel_token.store(next_generation(previous.generation), Ordering::SeqCst);
        }

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

        let document = OpenDocument {
            language_id: current.language_id.clone(),
            text,
            line_index,
            version,
            generation,
            file_path,
            project,
            cancel_token: Arc::clone(&current.cancel_token),
        };

        Some(PreparedDocument { uri: uri.clone(), document })
    }

    /// Commit a prepared full-document change and return the worker snapshot.
    ///
    /// Committing updates both the visible document state and the shared cancel
    /// token so older snapshots become stale immediately.
    pub fn commit_change(&mut self, prepared: PreparedDocument) -> DocumentSnapshot {
        let (uri, document, snapshot) = prepared.into_parts();
        document.cancel_token.store(document.generation, Ordering::SeqCst);
        self.documents.insert(uri.clone(), document);
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
        true
    }

    /// Return the latest committed generation for the given open document URI.
    ///
    /// Closed documents return `None` even though their last generation remains
    /// tracked internally for future reopen handling.
    pub fn generation(&self, uri: &Uri) -> Option<u64> {
        self.documents.get(uri).map(|document| document.generation)
    }

    /// Return the committed document for tests.
    #[cfg(test)]
    pub fn get(&self, uri: &Uri) -> Option<&OpenDocument> {
        self.documents.get(uri)
    }
}

impl OpenDocument {
    fn snapshot(&self, uri: Uri) -> DocumentSnapshot {
        DocumentSnapshot {
            uri,
            text: Arc::clone(&self.text),
            line_index: Arc::clone(&self.line_index),
            version: self.version,
            generation: self.generation,
            file_path: self.file_path.clone(),
            project: self.project.clone(),
            cancel_token: Arc::clone(&self.cancel_token),
        }
    }
}

impl PreparedDocument {
    fn into_parts(self) -> (Uri, OpenDocument, DocumentSnapshot) {
        let Self { uri, document } = self;
        // Snapshot before moving the document into the store so the worker sees
        // the exact committed state, including the shared cancel token.
        let snapshot = document.snapshot(uri.clone());
        (uri, document, snapshot)
    }
}

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

    fn test_uri() -> Uri {
        "file:///tmp/main.leo".parse().expect("valid file uri")
    }

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
