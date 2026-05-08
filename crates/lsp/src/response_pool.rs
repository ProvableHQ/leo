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

//! Small response-conversion pool for navigation requests with disk targets.
//!
//! Package analysis stores compact byte ranges, not retained file text. This
//! pool keeps disk re-read, fingerprint verification, and UTF-16 range
//! conversion off the routing thread while sending completions back through the
//! single JSON-RPC writer.

use crate::{
    document_store::{DocumentStore, PackageAnalysisKey},
    features::{
        lsp_range::{compact_range_to_location_with_line_index, read_verified_disk_text},
        references::{ReferenceQuery, resolve_targets, response_value as references_response_value},
    },
    semantics::{CachedPackageAnalysis, FileId, SourceFingerprint},
};
use crossbeam_channel::{Receiver, Sender, unbounded};
use line_index::LineIndex;
use lsp_server::RequestId;
use lsp_types::{Location, Uri};
use serde_json::Value;
use std::{
    collections::HashMap,
    panic::{AssertUnwindSafe, catch_unwind},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
};

const RESPONSE_WORKERS: usize = 2;

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
        /// Open-buffer line indexes captured when the job was dispatched.
        open_line_indexes: Arc<OpenLineIndexes>,
        /// Cancellation flag shared with the routing thread.
        cancel: Arc<AtomicBool>,
    },
    /// Ask a worker to stop after completing any current job.
    Shutdown,
}

/// Snapshot of open-document line indexes captured at job dispatch time.
#[derive(Debug, Default)]
pub struct OpenLineIndexes {
    indexes: HashMap<Arc<PathBuf>, Arc<LineIndex>>,
    uri_paths: Vec<(Uri, Arc<PathBuf>)>,
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
}

/// Prepared response payload returned to the routing thread.
#[derive(Debug)]
pub enum ResponseResult {
    /// Successful JSON-RPC result payload.
    Ok(Value),
    /// Internal error message to return for a failed conversion.
    InternalError(String),
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

impl OpenLineIndexes {
    /// Snapshot open file paths and line indexes with Arc clones only.
    pub fn snapshot(documents: &DocumentStore) -> Arc<Self> {
        let mut indexes = HashMap::new();
        let mut uri_paths = Vec::new();
        for (uri, document) in documents.iter_open() {
            let Some(path) = document.file_path.as_ref() else {
                continue;
            };
            indexes.entry(Arc::clone(path)).or_insert_with(|| Arc::clone(&document.line_index));
            uri_paths.push((uri.clone(), Arc::clone(path)));
        }
        Arc::new(Self { indexes, uri_paths })
    }

    /// Return the open line index for a path, if that file is currently open.
    fn get(&self, path: &Path) -> Option<&Arc<LineIndex>> {
        self.indexes.iter().find_map(|(candidate, line_index)| (candidate.as_path() == path).then_some(line_index))
    }

    /// Return the native path for an open document URI in this snapshot.
    fn path_for_uri(&self, uri: &Uri) -> Option<&Arc<PathBuf>> {
        self.uri_paths.iter().find_map(|(candidate, path)| (candidate == uri).then_some(path))
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
                    ResponseJob::References { id, query, package, open_line_indexes, cancel } => {
                        let key = package.key.clone();
                        let completion_cancel = Arc::clone(&cancel);
                        let result = catch_unwind(AssertUnwindSafe(|| {
                            references_response(*query, package, open_line_indexes, cancel)
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
    open_line_indexes: Arc<OpenLineIndexes>,
    cancel: Arc<AtomicBool>,
) -> Option<Value> {
    let Some(source_path) = open_line_indexes.path_for_uri(&query.view_key.uri) else {
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
            open_line_indexes.as_ref(),
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
    open_line_indexes: &OpenLineIndexes,
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
            let line_index = file.open_line_index.as_ref().or_else(|| open_line_indexes.get(file.path.as_ref()));
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
