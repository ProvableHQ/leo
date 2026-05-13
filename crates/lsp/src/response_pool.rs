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

//! Small response-conversion pool for navigation requests with disk targets.
//!
//! Package analysis stores compact byte ranges, not retained file text. This
//! pool keeps disk re-read, fingerprint verification, and UTF-16 range
//! conversion off the routing thread while sending completions back through the
//! single JSON-RPC writer.

use crate::{
    document_store::{DocumentStore, PackageAnalysisKey},
    features::{
        lsp_range::{byte_range_to_lsp_range, compact_range_to_location_with_line_index, read_verified_disk_text},
        references::{ReferenceQuery, resolve_targets, response_value as references_response_value},
        rename::{RenameError, RenameQuery, resolve_targets as resolve_rename_targets},
    },
    project_model::path_to_file_uri,
    semantics::{CachedPackageAnalysis, FileId, SourceFingerprint},
};
use crossbeam_channel::{Receiver, Sender, unbounded};
use line_index::LineIndex;
use lsp_server::RequestId;
use lsp_types::{
    DocumentChanges,
    Location,
    OneOf,
    OptionalVersionedTextDocumentIdentifier,
    TextDocumentEdit,
    TextEdit,
    Uri,
    WorkspaceEdit,
};
use serde_json::Value;
use std::{
    collections::{BTreeMap, HashMap},
    panic::{AssertUnwindSafe, catch_unwind},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
};

/// Fixed worker thread count for the response materialization pool.
const RESPONSE_WORKERS: usize = 2;

/// JSON-RPC code for `RequestFailed` (LSP 3.17 spec).
pub const REQUEST_FAILED: i32 = -32803;

/// Bounded worker pool for response materialization.
#[derive(Debug)]
pub struct ResponsePool {
    job_tx: Sender<ResponseJob>,
    completion_rx: Receiver<ResponseCompletion>,
    workers: Vec<JoinHandle<()>>,
}

/// Work item executed off the routing thread.
#[derive(Debug)]
pub enum ResponseJob {
    /// Convert one find-all-references query into its final LSP response value.
    References {
        /// Original JSON-RPC request ID.
        id: RequestId,
        /// Cursor query captured by the routing thread.
        query: Box<ReferenceQuery>,
        /// Package analysis containing compact reference ranges.
        package: Arc<CachedPackageAnalysis>,
        /// Open-document snapshot captured when the job was dispatched.
        open_snapshot: Arc<OpenSnapshot>,
        /// Cancellation flag shared with the routing thread.
        cancel: Arc<AtomicBool>,
    },
    /// Convert one rename query into a `WorkspaceEdit` JSON-RPC response.
    Rename {
        /// Original JSON-RPC request ID.
        id: RequestId,
        /// Cursor and validated new-name query captured by the routing thread.
        query: Box<RenameQuery>,
        /// Package analysis containing compact rename targets.
        package: Arc<CachedPackageAnalysis>,
        /// Open-document snapshot captured when the job was dispatched.
        open_snapshot: Arc<OpenSnapshot>,
        /// Cancellation flag shared with the routing thread.
        cancel: Arc<AtomicBool>,
    },
    /// Ask a worker to stop after completing any current job.
    Shutdown,
}

/// Snapshot of open-document line indexes, paths, and versions captured at
/// job dispatch time.
///
/// Versions are keyed by the canonical `Arc<PathBuf>` from
/// `OpenDocument::file_path`, not by the client-supplied URI. The client
/// URI may differ from the canonical path on platforms where
/// `path.canonicalize()` rewrites the prefix (macOS `/Users` →
/// `/private/Users`, Windows verbatim paths). Path-keyed lookup keeps
/// `OpenBuffer` files matching their `OptionalVersionedTextDocumentIdentifier`
/// version stamp regardless of where the analysis happens to recover them.
#[derive(Debug, Default)]
pub struct OpenSnapshot {
    indexes: HashMap<Arc<PathBuf>, Arc<LineIndex>>,
    uri_paths: Vec<(Uri, Arc<PathBuf>)>,
    versions: HashMap<Arc<PathBuf>, i32>,
}

/// Pool-to-routing notification sent after a job completes.
#[derive(Debug)]
pub enum ResponseCompletion {
    /// Completed find-all-references response materialization.
    References {
        /// Original JSON-RPC request ID.
        id: RequestId,
        /// Package key the response was computed against.
        key: PackageAnalysisKey,
        /// Cancellation token captured at dispatch, used to reject reused IDs.
        cancel: Arc<AtomicBool>,
        /// Final response payload or error.
        result: ResponseResult,
    },
    /// Completed rename response materialization.
    Rename {
        /// Original JSON-RPC request ID.
        id: RequestId,
        /// Package key the response was computed against.
        key: PackageAnalysisKey,
        /// Cancellation token captured at dispatch, used to reject reused IDs.
        cancel: Arc<AtomicBool>,
        /// Final rename response or error.
        result: RenameResult,
    },
}

/// Prepared response payload returned to the routing thread.
#[derive(Debug)]
pub enum ResponseResult {
    /// Successful JSON-RPC result payload.
    Ok(Value),
    /// Internal error message to return for a failed conversion.
    InternalError(String),
}

/// Rename-scoped result returned to the routing thread.
#[derive(Debug)]
pub enum RenameResult {
    /// Successful JSON-RPC result payload (`WorkspaceEdit` or `Value::Null`).
    Ok(Value),
    /// Internal error message returned for pool-worker panics.
    InternalError(String),
    /// LSP `RequestFailed` error with a stable diagnostic message.
    RequestFailed { code: i32, message: String },
}

impl ResponsePool {
    /// Start the fixed-size response conversion pool.
    pub fn new() -> Self {
        let (job_tx, job_rx) = unbounded();
        let (completion_tx, completion_rx) = unbounded();
        let workers =
            (0..RESPONSE_WORKERS).map(|index| spawn_worker(index, job_rx.clone(), completion_tx.clone())).collect();
        Self { job_tx, completion_rx, workers }
    }

    /// Return the completion channel observed by the routing loop.
    pub fn completions(&self) -> &Receiver<ResponseCompletion> {
        &self.completion_rx
    }

    /// Submit one response conversion job.
    pub fn submit(&self, job: ResponseJob) {
        if let Err(error) = self.job_tx.send(job) {
            tracing::debug!(error = %error, "response pool is shut down");
        }
    }

    /// Stop worker threads and wait for them to exit.
    pub fn shutdown(&mut self) {
        for _ in 0..self.workers.len() {
            let _ = self.job_tx.send(ResponseJob::Shutdown);
        }
        while let Some(worker) = self.workers.pop() {
            if let Err(error) = worker.join() {
                tracing::error!(?error, "response worker panicked during shutdown");
            }
        }
    }
}

impl Drop for ResponsePool {
    /// Shut down response workers when the pool owner is dropped.
    fn drop(&mut self) {
        self.shutdown();
    }
}

impl Default for ResponsePool {
    /// Start a default fixed-size response conversion pool.
    fn default() -> Self {
        Self::new()
    }
}

impl OpenSnapshot {
    /// Snapshot open file paths, line indexes, and document versions.
    pub fn snapshot(documents: &DocumentStore) -> Arc<Self> {
        let mut indexes = HashMap::new();
        let mut uri_paths = Vec::new();
        let mut versions = HashMap::new();
        for (uri, document) in documents.iter_open() {
            let Some(path) = document.file_path.as_ref() else {
                continue;
            };
            indexes.entry(Arc::clone(path)).or_insert_with(|| Arc::clone(&document.line_index));
            uri_paths.push((uri.clone(), Arc::clone(path)));
            versions.entry(Arc::clone(path)).or_insert(document.version);
        }
        Arc::new(Self { indexes, uri_paths, versions })
    }

    /// Return the open line index for a path, if that file is currently open.
    fn get(&self, path: &Path) -> Option<&Arc<LineIndex>> {
        self.indexes.iter().find_map(|(candidate, line_index)| (candidate.as_path() == path).then_some(line_index))
    }

    /// Return the native path for an open document URI in this snapshot.
    fn path_for_uri(&self, uri: &Uri) -> Option<&Arc<PathBuf>> {
        self.uri_paths.iter().find_map(|(candidate, path)| (candidate == uri).then_some(path))
    }

    /// Return the open-buffer version for a canonical path captured at
    /// dispatch time.
    fn version_for_path(&self, path: &Path) -> Option<i32> {
        self.versions.iter().find_map(|(candidate, version)| (candidate.as_path() == path).then_some(*version))
    }
}

/// Spawn one response worker thread.
fn spawn_worker(
    index: usize,
    job_rx: Receiver<ResponseJob>,
    completion_tx: Sender<ResponseCompletion>,
) -> JoinHandle<()> {
    thread::Builder::new()
        .name(format!("leo-lsp-response-{index}"))
        .spawn(move || {
            while let Ok(job) = job_rx.recv() {
                match job {
                    ResponseJob::References { id, query, package, open_snapshot, cancel } => {
                        let key = package.key.clone();
                        let completion_cancel = Arc::clone(&cancel);
                        let result = catch_unwind(AssertUnwindSafe(|| {
                            references_response(*query, package, open_snapshot, cancel)
                        }));
                        let result = match result {
                            Ok(Some(value)) => ResponseResult::Ok(value),
                            Ok(None) => continue,
                            Err(_) => ResponseResult::InternalError(
                                "references response conversion panicked; see server logs for details".to_owned(),
                            ),
                        };
                        let _ = completion_tx.send(ResponseCompletion::References {
                            id,
                            key,
                            cancel: completion_cancel,
                            result,
                        });
                    }
                    ResponseJob::Rename { id, query, package, open_snapshot, cancel } => {
                        let key = package.key.clone();
                        let completion_cancel = Arc::clone(&cancel);
                        let result =
                            catch_unwind(AssertUnwindSafe(|| rename_response(*query, package, open_snapshot, cancel)));
                        let result = match result {
                            Ok(Some(result)) => result,
                            Ok(None) => continue,
                            Err(_) => RenameResult::InternalError(
                                "rename response conversion panicked; see server logs for details".to_owned(),
                            ),
                        };
                        let _ = completion_tx.send(ResponseCompletion::Rename {
                            id,
                            key,
                            cancel: completion_cancel,
                            result,
                        });
                    }
                    ResponseJob::Shutdown => break,
                }
            }
        })
        .expect("spawn leo-lsp response worker")
}

/// Resolve and materialize one references response off the routing thread.
fn references_response(
    query: ReferenceQuery,
    package: Arc<CachedPackageAnalysis>,
    open_snapshot: Arc<OpenSnapshot>,
    cancel: Arc<AtomicBool>,
) -> Option<Value> {
    let Some(source_path) = open_snapshot.path_for_uri(&query.view_key.uri) else {
        return Some(references_response_value(false, Vec::new()));
    };
    let targets = resolve_targets(&query, package.as_ref(), source_path.as_ref());
    if !targets.navigable {
        return Some(references_response_value(false, Vec::new()));
    }

    let mut locations = Vec::new();
    let mut start = 0_usize;
    while start < targets.ranges.len() {
        if cancel.load(Ordering::SeqCst) {
            return None;
        }
        let file = targets.ranges[start].file;
        let mut end = start + 1;
        while end < targets.ranges.len() && targets.ranges[end].file == file {
            end += 1;
        }
        append_file_locations(
            file,
            &targets.ranges[start..end],
            package.as_ref(),
            open_snapshot.as_ref(),
            &mut locations,
        );
        start = end;
    }

    Some(references_response_value(true, locations))
}

/// Append all verified locations for one analyzed file.
fn append_file_locations(
    file_id: FileId,
    ranges: &[crate::semantics::CompactRange],
    package: &CachedPackageAnalysis,
    open_snapshot: &OpenSnapshot,
    locations: &mut Vec<Location>,
) {
    let Some(file) = package.analyzed_files.get(file_id) else {
        return;
    };
    if file.is_sentinel {
        return;
    }

    match &file.fingerprint {
        SourceFingerprint::OpenBuffer => {
            let line_index = file.open_line_index.as_ref().or_else(|| open_snapshot.get(file.path.as_ref()));
            let Some(line_index) = line_index else {
                return;
            };
            append_locations_with_line_index(ranges, package, line_index.as_ref(), locations);
        }
        SourceFingerprint::Disk { .. } => {
            let Some(text) = read_verified_disk_text(file.path.as_ref(), &file.fingerprint) else {
                return;
            };
            let line_index = LineIndex::new(text.as_str());
            append_locations_with_line_index(ranges, package, &line_index, locations);
        }
        SourceFingerprint::Volatile => {}
    }
}

/// Convert compact ranges through a known line index and append LSP locations.
fn append_locations_with_line_index(
    ranges: &[crate::semantics::CompactRange],
    package: &CachedPackageAnalysis,
    line_index: &LineIndex,
    locations: &mut Vec<Location>,
) {
    for range in ranges {
        if let Some((uri, range)) =
            compact_range_to_location_with_line_index(*range, package.analyzed_files.as_ref(), line_index)
        {
            locations.push(Location { uri, range });
        }
    }
}

/// Resolve and materialize one rename response off the routing thread.
///
/// Returns `Some(RenameResult)` when the worker has produced a payload (`Ok`,
/// `RequestFailed`, or `InternalError`). Returns `None` when cancellation
/// flipped before materialization completed; the routing thread drops the
/// in-flight job in that case.
fn rename_response(
    query: RenameQuery,
    package: Arc<CachedPackageAnalysis>,
    open_snapshot: Arc<OpenSnapshot>,
    cancel: Arc<AtomicBool>,
) -> Option<RenameResult> {
    let Some(source_path) = open_snapshot.path_for_uri(&query.view_key.uri) else {
        return Some(RenameResult::Ok(Value::Null));
    };
    let targets = match resolve_rename_targets(&query, package.as_ref(), source_path.as_ref()) {
        Ok(targets) => targets,
        Err(RenameError::NotRenameable) => return Some(RenameResult::Ok(Value::Null)),
        Err(error) => return Some(rename_error_to_result(error)),
    };
    if targets.is_empty() {
        return Some(RenameResult::Ok(Value::Null));
    }

    // Group occurrences by FileId to batch one disk read + one LineIndex per file.
    let mut by_file: BTreeMap<FileId, Vec<crate::semantics::CompactRange>> = BTreeMap::new();
    for target in &targets {
        by_file.entry(target.file).or_default().push(target.range);
    }

    let mut document_edits: BTreeMap<Uri, (OptionalVersionedTextDocumentIdentifier, Vec<TextEdit>)> = BTreeMap::new();
    for (file_id, ranges) in by_file {
        if cancel.load(Ordering::SeqCst) {
            return None;
        }
        let Some(file) = package.analyzed_files.get(file_id) else {
            return Some(RenameResult::RequestFailed {
                code: REQUEST_FAILED,
                message: "rename target references unknown analyzed file".to_owned(),
            });
        };
        let Some(uri) = path_to_file_uri(file.path.as_ref()) else {
            return Some(RenameResult::RequestFailed {
                code: REQUEST_FAILED,
                message: format!("rename target '{}' cannot be expressed as an LSP URI", file.path.display()),
            });
        };

        let (line_index, owned_index, version) = match &file.fingerprint {
            SourceFingerprint::OpenBuffer => {
                let line_index =
                    file.open_line_index.as_ref().cloned().or_else(|| open_snapshot.get(file.path.as_ref()).cloned());
                let Some(line_index) = line_index else {
                    return Some(RenameResult::RequestFailed {
                        code: REQUEST_FAILED,
                        message: format!("rename target '{}' has no open-buffer line index", file.path.display()),
                    });
                };
                let version = open_snapshot.version_for_path(file.path.as_ref());
                (LineIndexRef::Shared(line_index), None, version)
            }
            SourceFingerprint::Disk { .. } => {
                let Some(text) = read_verified_disk_text(file.path.as_ref(), &file.fingerprint) else {
                    return Some(RenameResult::RequestFailed {
                        code: REQUEST_FAILED,
                        message: format!(
                            "source for '{}' changed since analysis; please retry rename",
                            file.path.display()
                        ),
                    });
                };
                let owned = LineIndex::new(text.as_str());
                (LineIndexRef::Owned, Some(owned), None)
            }
            SourceFingerprint::Volatile => {
                return Some(RenameResult::RequestFailed {
                    code: REQUEST_FAILED,
                    message: format!("rename target '{}' has volatile source", file.path.display()),
                });
            }
        };

        let line_index_ref: &LineIndex = match &line_index {
            LineIndexRef::Shared(arc) => arc.as_ref(),
            LineIndexRef::Owned => owned_index.as_ref().expect("owned line index"),
        };

        let mut edits: Vec<TextEdit> = Vec::with_capacity(ranges.len());
        let mut last_end: Option<u32> = None;
        for range in &ranges {
            // Per-URI ranges inherit `(start, end)` order from `occurrences_for`;
            // the worker pins that contract instead of resorting.
            debug_assert!(
                last_end.map(|prev| prev <= range.start).unwrap_or(true),
                "rename ranges must be in non-decreasing order"
            );
            last_end = Some(range.end);
            let Some(lsp_range) = byte_range_to_lsp_range(line_index_ref, range.start, range.end) else {
                return Some(RenameResult::RequestFailed {
                    code: REQUEST_FAILED,
                    message: format!("rename target range fell outside source for '{}'", file.path.display()),
                });
            };
            edits.push(TextEdit { range: lsp_range, new_text: query.new_name.clone() });
        }

        document_edits.insert(uri.clone(), (OptionalVersionedTextDocumentIdentifier { uri, version }, edits));
    }

    let text_document_edits = document_edits
        .into_iter()
        .map(|(_uri, (text_document, edits))| TextDocumentEdit {
            text_document,
            edits: edits.into_iter().map(OneOf::Left).collect(),
        })
        .collect::<Vec<_>>();

    let workspace_edit = WorkspaceEdit {
        changes: None,
        document_changes: Some(DocumentChanges::Edits(text_document_edits)),
        change_annotations: None,
    };
    let value = serde_json::to_value(workspace_edit).expect("WorkspaceEdit should serialize");
    Some(RenameResult::Ok(value))
}

/// Convert a per-file rename rejection into the corresponding `RequestFailed` result.
fn rename_error_to_result(error: RenameError) -> RenameResult {
    match error {
        RenameError::InvalidIdentifier(message) => RenameResult::RequestFailed { code: REQUEST_FAILED, message },
        RenameError::NotRenameable => RenameResult::Ok(Value::Null),
        RenameError::SourceChanged { path } => RenameResult::RequestFailed {
            code: REQUEST_FAILED,
            message: format!("source for '{path}' changed since analysis; please retry rename"),
        },
        RenameError::LeavesPackage { path } => RenameResult::RequestFailed {
            code: REQUEST_FAILED,
            message: format!("rename target '{path}' leaves the current package"),
        },
        RenameError::VolatileSource { path } => RenameResult::RequestFailed {
            code: REQUEST_FAILED,
            message: format!("rename target '{path}' has volatile source"),
        },
    }
}

/// Per-file line index slot that keeps both Arc-shared and worker-owned indexes addressable.
enum LineIndexRef {
    /// Open-buffer line index borrowed from `AnalyzedFile` or the snapshot.
    Shared(Arc<LineIndex>),
    /// Disk-backed line index built once per file by the worker.
    Owned,
}
