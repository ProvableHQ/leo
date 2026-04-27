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

use crate::{
    compiler_bridge::{PackageAnalysisCache, analyze_snapshot},
    document_store::DocumentSnapshot,
    panic_boundary::{PanicReport, catch_unwind},
    semantics::SemanticSnapshot,
};
use crossbeam_channel::{Receiver, Sender, TryRecvError, unbounded};
use lsp_types::Uri;
use std::{
    collections::HashMap,
    sync::atomic::AtomicU64,
    thread::{self, JoinHandle},
};

type PendingJobs = HashMap<String, PendingJob>;

/// Commands sent from the main thread to the background worker.
#[derive(Debug)]
pub enum WorkerCommand {
    Analyze(DocumentSnapshot),
    Shutdown,
}

/// Events sent from the background worker back to the main thread.
#[derive(Debug)]
pub enum WorkerEvent {
    Analyzed(DocumentAnalysis),
    Cancelled { uri: Uri, generation: u64 },
    Panicked { uri: Uri, generation: u64, report: PanicReport },
}

/// Worker-produced semantic state for one document generation.
#[derive(Debug, Clone)]
pub struct DocumentAnalysis {
    pub uri: Uri,
    pub generation: u64,
    pub semantic_snapshot: SemanticSnapshot,
}

/// A coalesced worker job paired with its arrival order.
///
/// The worker keeps at most one pending snapshot per URI, and `sequence`
/// preserves global recency so the most recently updated document runs first.
#[derive(Debug)]
struct PendingJob {
    sequence: u64,
    snapshot: DocumentSnapshot,
}

/// Background worker owner and communication channels.
#[derive(Debug)]
pub struct Scheduler {
    command_tx: Sender<WorkerCommand>,
    event_rx: Receiver<WorkerEvent>,
    worker: Option<JoinHandle<()>>,
}

impl Scheduler {
    /// Spawn the dedicated analysis worker thread.
    pub fn new(panic_on_worker_job: bool) -> Self {
        let (command_tx, command_rx) = unbounded();
        let (event_tx, event_rx) = unbounded();

        let worker = thread::Builder::new()
            .name("leo-lsp-worker".to_owned())
            .spawn(move || worker_loop(command_rx, event_tx, panic_on_worker_job))
            .expect("failed to spawn leo-lsp worker");

        Self { command_tx, event_rx, worker: Some(worker) }
    }

    /// Enqueue a document snapshot for background analysis.
    pub fn enqueue(&self, snapshot: DocumentSnapshot) {
        let _ = self.command_tx.send(WorkerCommand::Analyze(snapshot));
    }

    /// Return the receiver used to observe worker events.
    pub fn events(&self) -> &Receiver<WorkerEvent> {
        &self.event_rx
    }

    /// Shut down the worker thread and wait for it to exit.
    pub fn shutdown(&mut self) {
        let _ = self.command_tx.send(WorkerCommand::Shutdown);

        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

impl Drop for Scheduler {
    fn drop(&mut self) {
        self.shutdown();
    }
}

fn worker_loop(command_rx: Receiver<WorkerCommand>, event_tx: Sender<WorkerEvent>, panic_on_worker_job: bool) {
    let mut pending = PendingJobs::new();
    let mut sequence = 0_u64;
    let mut package_cache = PackageAnalysisCache::default();

    loop {
        // Block only when there is nothing queued locally; otherwise keep
        // draining and coalescing messages before choosing the next job.
        if pending.is_empty() {
            match command_rx.recv() {
                Ok(command) => {
                    if absorb_command(command, &mut pending, &mut sequence) {
                        break;
                    }
                }
                Err(_) => break,
            }
        }

        if drain_commands(&command_rx, &mut pending, &mut sequence) {
            break;
        }

        // Prefer the most recent document edit across the whole queue instead
        // of processing URIs round-robin. That keeps the actively edited file
        // feeling responsive when multiple documents are open.
        let Some(snapshot) = take_latest_pending(&mut pending) else {
            continue;
        };

        run_job(snapshot, &event_tx, panic_on_worker_job, &mut package_cache);
    }
}

fn absorb_command(command: WorkerCommand, pending: &mut PendingJobs, sequence: &mut u64) -> bool {
    match command {
        WorkerCommand::Analyze(snapshot) => {
            *sequence += 1;
            // Replacing by URI coalesces bursts of edits down to the latest
            // snapshot while still remembering when that URI was last touched.
            pending.insert(snapshot.uri.as_str().to_owned(), PendingJob { sequence: *sequence, snapshot });
            false
        }
        WorkerCommand::Shutdown => true,
    }
}

fn drain_commands(command_rx: &Receiver<WorkerCommand>, pending: &mut PendingJobs, sequence: &mut u64) -> bool {
    // Collapse any burst of pending commands before work starts so the worker
    // spends time on the freshest snapshots rather than intermediate states.
    loop {
        match command_rx.try_recv() {
            Ok(command) => {
                if absorb_command(command, pending, sequence) {
                    return true;
                }
            }
            Err(TryRecvError::Empty) => return false,
            Err(TryRecvError::Disconnected) => return true,
        }
    }
}

fn take_latest_pending(pending: &mut PendingJobs) -> Option<DocumentSnapshot> {
    // Pick the single newest URI update globally, then remove that one pending
    // entry from the queue.
    let next_sequence = pending.values().max_by_key(|job| job.sequence)?.sequence;
    pending.extract_if(|_, job| job.sequence == next_sequence).next().map(|(_, job)| job.snapshot)
}

fn run_job(
    snapshot: DocumentSnapshot,
    event_tx: &Sender<WorkerEvent>,
    panic_on_worker_job: bool,
    package_cache: &mut PackageAnalysisCache,
) {
    let uri = snapshot.uri.clone();
    let generation = snapshot.generation;
    let cancel_token = snapshot.cancel_token.clone();

    if is_cancelled(cancel_token.as_ref(), generation) {
        let _ = event_tx.send(WorkerEvent::Cancelled { uri, generation });
        return;
    }

    // Worker analysis runs off the main thread, so this is the task boundary
    // where we can contain an internal panic, report it as a bug, and keep
    // the server alive for future requests and newer document generations.
    let result = catch_unwind("worker_analyze", Some(&uri), Some(generation), || {
        if panic_on_worker_job {
            panic!("injected worker panic");
        }

        DocumentAnalysis { uri: uri.clone(), generation, semantic_snapshot: analyze_snapshot(&snapshot, package_cache) }
    });

    match result {
        Ok(analysis) => {
            // Check cancellation again after analysis because a newer commit can
            // arrive while this job is already in flight.
            let event = if is_cancelled(cancel_token.as_ref(), generation) {
                WorkerEvent::Cancelled { uri, generation }
            } else {
                WorkerEvent::Analyzed(analysis)
            };

            let _ = event_tx.send(event);
        }
        Err(report) => {
            let _ = event_tx.send(WorkerEvent::Panicked { uri, generation, report });
        }
    }
}

fn is_cancelled(cancel_token: &AtomicU64, generation: u64) -> bool {
    // The token stores the latest committed generation for a URI, so any
    // mismatch means newer document state has superseded this snapshot.
    cancel_token.load(std::sync::atomic::Ordering::SeqCst) != generation
}

#[cfg(test)]
mod tests {
    use super::{PendingJobs, Scheduler, WorkerCommand, WorkerEvent, absorb_command, take_latest_pending};
    use crate::document_store::DocumentSnapshot;
    use line_index::LineIndex;
    use lsp_types::Uri;
    use std::{
        path::PathBuf,
        sync::{Arc, atomic::AtomicU64},
        time::Duration,
    };

    fn snapshot(uri: &str, generation: u64) -> DocumentSnapshot {
        snapshot_with_token(uri, generation, Arc::new(AtomicU64::new(generation)))
    }

    fn snapshot_with_token(uri: &str, generation: u64, cancel_token: Arc<AtomicU64>) -> DocumentSnapshot {
        DocumentSnapshot {
            uri: uri.parse::<Uri>().expect("valid uri"),
            text: Arc::from("program test.aleo {}"),
            line_index: Arc::new(LineIndex::new("program test.aleo {}")),
            version: generation as i32,
            generation,
            file_path: Some(Arc::new(PathBuf::from("/tmp/main.leo"))),
            project: None,
            cancel_token,
        }
    }

    #[test]
    fn coalescing_keeps_latest_snapshot_per_uri() {
        let mut pending = PendingJobs::new();
        let mut sequence = 0;
        let uri = "file:///tmp/main.leo";

        assert!(!absorb_command(WorkerCommand::Analyze(snapshot(uri, 1)), &mut pending, &mut sequence));
        assert!(!absorb_command(WorkerCommand::Analyze(snapshot(uri, 2)), &mut pending, &mut sequence));

        let next = take_latest_pending(&mut pending).expect("pending snapshot");
        assert_eq!(next.generation, 2);
    }

    #[test]
    fn latest_updated_document_runs_first() {
        let mut pending = PendingJobs::new();
        let mut sequence = 0;

        assert!(
            !absorb_command(WorkerCommand::Analyze(snapshot("file:///tmp/a.leo", 1)), &mut pending, &mut sequence,)
        );
        assert!(
            !absorb_command(WorkerCommand::Analyze(snapshot("file:///tmp/b.leo", 1)), &mut pending, &mut sequence,)
        );
        assert!(
            !absorb_command(WorkerCommand::Analyze(snapshot("file:///tmp/a.leo", 2)), &mut pending, &mut sequence,)
        );

        let next = take_latest_pending(&mut pending).expect("pending snapshot");
        assert_eq!(next.uri.as_str(), "file:///tmp/a.leo");
        assert_eq!(next.generation, 2);
    }

    #[test]
    fn stale_snapshot_is_cancelled_before_work_starts() {
        let mut scheduler = Scheduler::new(false);
        let cancel_token = Arc::new(AtomicU64::new(2));

        scheduler.enqueue(snapshot_with_token("file:///tmp/main.leo", 1, cancel_token));

        let event = scheduler.events().recv_timeout(Duration::from_secs(1)).expect("worker event");

        match event {
            WorkerEvent::Cancelled { uri, generation } => {
                assert_eq!(uri.as_str(), "file:///tmp/main.leo");
                assert_eq!(generation, 1);
            }
            other => panic!("expected cancelled event, got {other:?}"),
        }

        scheduler.shutdown();
    }
}
