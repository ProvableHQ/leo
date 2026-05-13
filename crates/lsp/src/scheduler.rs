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

use crate::{
    compiler_bridge::{PackageAnalysisCache, PackageWorkerAnalysis, analyze_package_snapshot, build_document_view},
    document_store::{AnalysisBucket, DocumentSnapshot, DocumentViewKey, DocumentViewSnapshot, PackageAnalysisKey},
    panic_boundary::{PanicReport, catch_unwind},
    semantics::{CachedDocumentView, CachedPackageAnalysis},
};
use crossbeam_channel::{Receiver, Sender, bounded, unbounded};
use lsp_types::Uri;
use std::{
    collections::{HashMap, HashSet},
    sync::{
        Arc,
        Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    thread::{self, JoinHandle},
};

/// One wakeup is enough: real work lives in `PendingQueues` and coalesces there.
const WORKER_CHANNEL_BOUND: usize = 1;
/// Hard cap for heavy package jobs waiting behind the worker.
const MAX_PENDING_PACKAGE_JOBS: usize = 16;
/// Hard cap for lightweight document-view rebuilds waiting behind the worker.
const MAX_PENDING_VIEW_JOBS: usize = 64;

/// Pending heavy-analysis jobs keyed by package bucket.
type PendingPackageJobs = HashMap<AnalysisBucket, PendingPackageJob>;
/// Pending document-view jobs keyed by exact view freshness.
type PendingViewJobs = HashMap<DocumentViewKey, PendingViewJob>;

/// Events sent from the background worker back to the main thread.
#[derive(Debug)]
pub enum WorkerEvent {
    /// Fresh package analysis and trigger-document view are ready.
    PackageAnalyzed(PackageAnalysis),
    /// A semantic-token view was rebuilt from an already cached package.
    DocumentViewBuilt(CachedDocumentView),
    /// A package job became stale before or after worker execution.
    PackageCancelled { key: PackageAnalysisKey, uri: Uri, generation: u64 },
    /// A document-view job became stale before or after worker execution.
    DocumentViewCancelled { key: DocumentViewKey },
    /// Package analysis panicked inside the worker panic boundary.
    PackagePanicked { key: PackageAnalysisKey, uri: Uri, generation: u64, report: PanicReport },
    /// Document-view construction panicked inside the worker panic boundary.
    DocumentViewPanicked { key: DocumentViewKey, report: PanicReport },
}

/// Worker-produced semantic state for one document generation.
#[derive(Debug, Clone)]
pub struct PackageAnalysis {
    /// URI whose edit/request triggered this package analysis.
    pub uri: Uri,
    /// Per-document generation of the triggering URI.
    pub generation: u64,
    /// Package freshness key that all waiters must match.
    pub key: PackageAnalysisKey,
    /// Shared package index plus the trigger document's encoded view.
    pub result: PackageWorkerAnalysis,
}

/// A coalesced worker job paired with its arrival order.
///
/// Package jobs are keyed by [`AnalysisBucket`], so the worker keeps at most
/// one pending heavy-analysis snapshot per package bucket. `sequence` preserves
/// global recency so the freshest package edit runs first.
#[derive(Debug)]
struct PendingPackageJob {
    /// Monotonic enqueue order used to pick the freshest job globally.
    sequence: u64,
    /// Latest package snapshot retained for this bucket.
    snapshot: DocumentSnapshot,
}

/// A coalesced document-view job paired with its arrival order.
#[derive(Debug)]
struct PendingViewJob {
    /// Monotonic enqueue order used to pick the freshest view job globally.
    sequence: u64,
    /// Document snapshot used to rebuild encoded semantic tokens.
    snapshot: DocumentViewSnapshot,
    /// Package analysis the document view merges against.
    package: Arc<CachedPackageAnalysis>,
}

/// Heavy analysis jobs waiting for the worker.
///
/// The routing thread writes snapshots into this short-lived queue and sends a
/// tiny wakeup over the channel. Keeping package-sized snapshots out of the
/// channel lets edit storms coalesce in place without blocking LSP request
/// handling behind compiler work.
///
/// The mutex around this queue is intentionally not part of compiler analysis:
/// it is held only while inserting, dropping, or removing queued jobs. The
/// expensive worker path starts after the guard is gone, so request handling is
/// not serialized behind package analysis.
#[derive(Debug, Default)]
struct PendingQueues {
    /// Coalesced package-analysis jobs.
    packages: PendingPackageJobs,
    /// Coalesced document-view jobs.
    views: PendingViewJobs,
    /// Global sequence counter shared by both queues.
    sequence: u64,
    /// Latest open-bucket set supplied by the routing thread for cache eviction.
    open_buckets: Option<HashSet<AnalysisBucket>>,
}

/// Bounded worker wake token.
#[derive(Debug)]
enum WorkerWake {
    /// Ask the worker to inspect `PendingQueues`.
    Wake,
}

/// Concrete job selected from the coalescing queues.
enum WorkerJob {
    /// Rebuild shared package analysis for a snapshot.
    Package(DocumentSnapshot),
    /// Rebuild one encoded document view from a cached package.
    View(DocumentViewSnapshot, Arc<CachedPackageAnalysis>),
}

/// Background worker owner and communication channels.
#[derive(Debug)]
pub struct Scheduler {
    wake_tx: Sender<WorkerWake>,
    event_tx: Sender<WorkerEvent>,
    event_rx: Receiver<WorkerEvent>,
    pending: Arc<Mutex<PendingQueues>>,
    shutdown_requested: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
}

impl Scheduler {
    /// Spawn the dedicated analysis worker thread.
    pub fn new(panic_on_worker_job: bool) -> Self {
        let (wake_tx, wake_rx) = bounded(WORKER_CHANNEL_BOUND);
        let (event_tx, event_rx) = unbounded();
        let pending = Arc::new(Mutex::new(PendingQueues::default()));
        let shutdown_requested = Arc::new(AtomicBool::new(false));

        let worker_pending = Arc::clone(&pending);
        let worker_shutdown = Arc::clone(&shutdown_requested);
        let worker_event_tx = event_tx.clone();

        let worker = thread::Builder::new()
            .name("leo-lsp-worker".to_owned())
            .spawn(move || worker_loop(wake_rx, worker_event_tx, worker_pending, worker_shutdown, panic_on_worker_job))
            .expect("failed to spawn leo-lsp worker");

        Self { wake_tx, event_tx, event_rx, pending, shutdown_requested, worker: Some(worker) }
    }

    /// Enqueue a document snapshot for background analysis.
    pub fn enqueue_package(&self, snapshot: DocumentSnapshot) {
        with_pending_queues(&self.pending, |pending| queue_package(pending, snapshot, self.event_tx.clone()));
        self.wake_worker();
    }

    /// Enqueue one document-view rebuild against an already cached package analysis.
    pub fn enqueue_document_view(&self, snapshot: DocumentViewSnapshot, package: Arc<CachedPackageAnalysis>) {
        with_pending_queues(&self.pending, |pending| queue_view(pending, snapshot, package, self.event_tx.clone()));
        self.wake_worker();
    }

    /// Inform the worker which package buckets still have open documents.
    pub fn set_open_buckets(&self, buckets: HashSet<AnalysisBucket>) {
        with_pending_queues(&self.pending, |pending| pending.open_buckets = Some(buckets));
        self.wake_worker();
    }

    /// Return the receiver used to observe worker events.
    pub fn events(&self) -> &Receiver<WorkerEvent> {
        &self.event_rx
    }

    /// Shut down the worker thread and wait for it to exit.
    pub fn shutdown(&mut self) {
        self.shutdown_requested.store(true, Ordering::SeqCst);
        self.wake_worker();

        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }

    /// Wake the worker without queueing duplicate wake tokens.
    fn wake_worker(&self) {
        // The bounded wake channel carries no payload beyond "check the queue".
        // If it is already full, the worker has either been woken or will wake
        // shortly; another token would not represent another distinct job.
        let _ = self.wake_tx.try_send(WorkerWake::Wake);
    }
}

impl Drop for Scheduler {
    /// Join the worker thread when callers drop the scheduler without explicit shutdown.
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// Run the worker event loop until shutdown or channel disconnect.
fn worker_loop(
    wake_rx: Receiver<WorkerWake>,
    event_tx: Sender<WorkerEvent>,
    pending: Arc<Mutex<PendingQueues>>,
    shutdown_requested: Arc<AtomicBool>,
    panic_on_worker_job: bool,
) {
    let mut package_cache = PackageAnalysisCache::default();

    loop {
        if shutdown_requested.load(Ordering::SeqCst) {
            break;
        }

        if let Some(job) = take_next_job(&pending, &mut package_cache) {
            match job {
                WorkerJob::Package(snapshot) => {
                    run_package_job(snapshot, &event_tx, panic_on_worker_job, &mut package_cache);
                }
                WorkerJob::View(snapshot, package) => run_view_job(snapshot, package, &event_tx, panic_on_worker_job),
            }
            continue;
        }

        match wake_rx.recv() {
            Ok(WorkerWake::Wake) => {}
            Err(_) => break,
        }
    }
}

/// Mutate pending queue state while keeping the mutex critical section explicit.
fn with_pending_queues<R>(pending: &Arc<Mutex<PendingQueues>>, f: impl FnOnce(&mut PendingQueues) -> R) -> R {
    // Keep the critical section explicit and tiny. Callers should mutate queue
    // metadata here, then do compiler or document-view work after the guard is
    // dropped by returning from this helper.
    let mut pending = pending.lock().expect("scheduler pending queue mutex poisoned");
    f(&mut pending)
}

/// Insert or replace the pending heavy-analysis job for a package bucket.
fn queue_package(pending: &mut PendingQueues, snapshot: DocumentSnapshot, event_tx: Sender<WorkerEvent>) {
    pending.sequence += 1;
    if pending.packages.len() >= MAX_PENDING_PACKAGE_JOBS
        && !pending.packages.contains_key(&snapshot.package_key.bucket)
        && let Some(dropped) = drop_oldest_package(&mut pending.packages)
    {
        let _ = event_tx.send(WorkerEvent::PackageCancelled {
            key: dropped.snapshot.package_key,
            uri: dropped.snapshot.uri,
            generation: dropped.snapshot.generation,
        });
    }

    // Replacing by bucket, not generation, keeps edit storms to one heavy
    // overlay snapshot per package while preserving the freshest generation.
    pending
        .packages
        .insert(snapshot.package_key.bucket.clone(), PendingPackageJob { sequence: pending.sequence, snapshot });
}

/// Insert or replace the pending document-view job for an exact view key.
fn queue_view(
    pending: &mut PendingQueues,
    snapshot: DocumentViewSnapshot,
    package: Arc<CachedPackageAnalysis>,
    event_tx: Sender<WorkerEvent>,
) {
    // Document views are small, but still bounded because every pending view
    // retains a package `Arc` and an open-document snapshot.
    pending.sequence += 1;
    if pending.views.len() >= MAX_PENDING_VIEW_JOBS
        && !pending.views.contains_key(&snapshot.key)
        && let Some(dropped) = drop_oldest_view(&mut pending.views)
    {
        let _ = event_tx.send(WorkerEvent::DocumentViewCancelled { key: dropped.snapshot.key });
    }
    pending.views.insert(snapshot.key.clone(), PendingViewJob { sequence: pending.sequence, snapshot, package });
}

/// Remove the freshest runnable job and apply any queued cache-retention update.
fn take_next_job(pending: &Arc<Mutex<PendingQueues>>, package_cache: &mut PackageAnalysisCache) -> Option<WorkerJob> {
    with_pending_queues(pending, |pending| {
        if let Some(open_buckets) = pending.open_buckets.take() {
            package_cache.retain_open_buckets(&open_buckets);
        }

        // Package jobs come first because they unblock definition waiters and
        // any semantic-token document views that need fresh package analysis.
        if let Some(snapshot) = take_latest_package(&mut pending.packages) {
            Some(WorkerJob::Package(snapshot))
        } else {
            take_latest_view(&mut pending.views).map(|(snapshot, package)| WorkerJob::View(snapshot, package))
        }
    })
}

/// Remove the newest package job across all buckets.
fn take_latest_package(pending: &mut PendingPackageJobs) -> Option<DocumentSnapshot> {
    let next_sequence = pending.values().max_by_key(|job| job.sequence)?.sequence;
    pending.extract_if(|_, job| job.sequence == next_sequence).next().map(|(_, job)| job.snapshot)
}

/// Remove the newest document-view job across all view keys.
fn take_latest_view(pending: &mut PendingViewJobs) -> Option<(DocumentViewSnapshot, Arc<CachedPackageAnalysis>)> {
    let next_sequence = pending.values().max_by_key(|job| job.sequence)?.sequence;
    pending.extract_if(|_, job| job.sequence == next_sequence).next().map(|(_, job)| (job.snapshot, job.package))
}

/// Drop the oldest pending package job when the queue exceeds its cap.
fn drop_oldest_package(pending: &mut PendingPackageJobs) -> Option<PendingPackageJob> {
    if let Some(oldest) = pending.iter().min_by_key(|(_, job)| job.sequence).map(|(key, _)| key.clone()) {
        return pending.remove(&oldest);
    }
    None
}

/// Drop the oldest pending view job when the queue exceeds its cap.
fn drop_oldest_view(pending: &mut PendingViewJobs) -> Option<PendingViewJob> {
    if let Some(oldest) = pending.iter().min_by_key(|(_, job)| job.sequence).map(|(key, _)| key.clone()) {
        return pending.remove(&oldest);
    }
    None
}

/// Execute one package-analysis job inside the worker panic/cancellation boundary.
fn run_package_job(
    snapshot: DocumentSnapshot,
    event_tx: &Sender<WorkerEvent>,
    panic_on_worker_job: bool,
    package_cache: &mut PackageAnalysisCache,
) {
    let uri = snapshot.uri.clone();
    let generation = snapshot.generation;
    let key = snapshot.package_key.clone();
    let cancel_token = snapshot.cancel_token.clone();

    if is_cancelled(cancel_token.as_ref(), generation) {
        let _ = event_tx.send(WorkerEvent::PackageCancelled { key, uri, generation });
        return;
    }

    // Worker analysis runs off the main thread, so this is the task boundary
    // where we can contain an internal panic, report it as a bug, and keep
    // the server alive for future requests and newer document generations.
    let result = catch_unwind("worker_analyze", Some(&uri), Some(generation), || {
        if panic_on_worker_job {
            panic!("injected worker panic");
        }

        PackageAnalysis {
            uri: uri.clone(),
            generation,
            key: key.clone(),
            result: analyze_package_snapshot(&snapshot, package_cache),
        }
    });

    match result {
        Ok(analysis) => {
            // Check cancellation again after analysis because a newer commit can
            // arrive while this job is already in flight.
            let event = if is_cancelled(cancel_token.as_ref(), generation) {
                WorkerEvent::PackageCancelled { key, uri, generation }
            } else {
                WorkerEvent::PackageAnalyzed(analysis)
            };

            let _ = event_tx.send(event);
        }
        Err(report) => {
            let _ = event_tx.send(WorkerEvent::PackagePanicked { key, uri, generation, report });
        }
    }
}

/// Execute one document-view rebuild inside the worker panic/cancellation boundary.
fn run_view_job(
    snapshot: DocumentViewSnapshot,
    package: Arc<CachedPackageAnalysis>,
    event_tx: &Sender<WorkerEvent>,
    panic_on_worker_job: bool,
) {
    let key = snapshot.key.clone();
    if is_cancelled(snapshot.cancel_token.as_ref(), key.document_generation) {
        let _ = event_tx.send(WorkerEvent::DocumentViewCancelled { key });
        return;
    }

    let result = catch_unwind("worker_document_view", Some(&snapshot.uri), Some(key.document_generation), || {
        if panic_on_worker_job {
            panic!("injected worker panic");
        }

        build_document_view(&snapshot, package)
    });

    match result {
        Ok(view) => {
            let event = if is_cancelled(snapshot.cancel_token.as_ref(), key.document_generation) {
                WorkerEvent::DocumentViewCancelled { key }
            } else {
                WorkerEvent::DocumentViewBuilt(view)
            };
            let _ = event_tx.send(event);
        }
        Err(report) => {
            let _ = event_tx.send(WorkerEvent::DocumentViewPanicked { key, report });
        }
    }
}

/// Return whether a snapshot generation has been superseded for its URI.
fn is_cancelled(cancel_token: &AtomicU64, generation: u64) -> bool {
    // The token stores the latest committed generation for a URI, so any
    // mismatch means newer document state has superseded this snapshot.
    cancel_token.load(std::sync::atomic::Ordering::SeqCst) != generation
}

#[cfg(test)]
mod tests {
    use super::{PendingQueues, Scheduler, WorkerEvent, queue_package, take_latest_package};
    use crate::document_store::{AnalysisBucket, DocumentSnapshot, DocumentViewKey, PackageAnalysisKey};
    use crossbeam_channel::unbounded;
    use line_index::LineIndex;
    use lsp_types::Uri;
    use std::{
        path::PathBuf,
        sync::{Arc, atomic::AtomicU64},
        time::Duration,
    };

    /// Build a test snapshot whose cancel token initially matches its generation.
    fn snapshot(uri: &str, generation: u64) -> DocumentSnapshot {
        snapshot_with_token(uri, generation, Arc::new(AtomicU64::new(generation)))
    }

    /// Build a test snapshot with an explicit cancellation token.
    fn snapshot_with_token(uri: &str, generation: u64, cancel_token: Arc<AtomicU64>) -> DocumentSnapshot {
        let uri = uri.parse::<Uri>().expect("valid uri");
        let bucket = AnalysisBucket::UnmanagedDocument { uri: uri.clone() };
        let package_key = PackageAnalysisKey { bucket, bucket_generation: generation };
        let view_key =
            DocumentViewKey { uri: uri.clone(), document_generation: generation, package: package_key.clone() };
        DocumentSnapshot {
            uri,
            text: Arc::from("program test.aleo {}"),
            line_index: Arc::new(LineIndex::new("program test.aleo {}")),
            version: generation as i32,
            generation,
            file_path: Some(Arc::new(PathBuf::from("/tmp/main.leo"))),
            project: None,
            package_key,
            view_key,
            open_overlays: Arc::from([]),
            cancel_token,
        }
    }

    /// Return a worker-event sender whose receiver is intentionally unused.
    fn event_sink() -> crossbeam_channel::Sender<WorkerEvent> {
        let (tx, _rx) = unbounded();
        tx
    }

    /// Verifies per-bucket package job coalescing keeps only the latest snapshot.
    #[test]
    fn coalescing_keeps_latest_snapshot_per_uri() {
        let mut pending = PendingQueues::default();
        let event_tx = event_sink();
        let uri = "file:///tmp/main.leo";

        queue_package(&mut pending, snapshot(uri, 1), event_tx.clone());
        queue_package(&mut pending, snapshot(uri, 2), event_tx);

        let next = take_latest_package(&mut pending.packages).expect("pending snapshot");
        assert_eq!(next.generation, 2);
    }

    /// Verifies global freshness ordering runs the most recently updated package first.
    #[test]
    fn latest_updated_document_runs_first() {
        let mut pending = PendingQueues::default();
        let event_tx = event_sink();

        queue_package(&mut pending, snapshot("file:///tmp/a.leo", 1), event_tx.clone());
        queue_package(&mut pending, snapshot("file:///tmp/b.leo", 1), event_tx.clone());
        queue_package(&mut pending, snapshot("file:///tmp/a.leo", 2), event_tx);

        let next = take_latest_package(&mut pending.packages).expect("pending snapshot");
        assert_eq!(next.uri.as_str(), "file:///tmp/a.leo");
        assert_eq!(next.generation, 2);
    }

    /// Verifies a stale package snapshot is cancelled before expensive work begins.
    #[test]
    fn stale_snapshot_is_cancelled_before_work_starts() {
        let mut scheduler = Scheduler::new(false);
        let cancel_token = Arc::new(AtomicU64::new(2));

        scheduler.enqueue_package(snapshot_with_token("file:///tmp/main.leo", 1, cancel_token));

        let event = scheduler.events().recv_timeout(Duration::from_secs(1)).expect("worker event");

        match event {
            WorkerEvent::PackageCancelled { uri, generation, .. } => {
                assert_eq!(uri.as_str(), "file:///tmp/main.leo");
                assert_eq!(generation, 1);
            }
            other => panic!("expected cancelled event, got {other:?}"),
        }

        scheduler.shutdown();
    }
}
