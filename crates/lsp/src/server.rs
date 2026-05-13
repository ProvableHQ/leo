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

//! Main `leo-lsp` protocol loop and routing-thread state.
//!
//! This module owns the initialized LSP lifecycle, open-document routing,
//! request cancellation, worker completion handling, and response-pool
//! handoffs. Feature modules answer semantic questions; this layer preserves
//! freshness, ordering, and single-writer JSON-RPC response semantics.

use crate::{
    document_store::{AnalysisBucket, DocumentStore, DocumentViewKey, PackageAnalysisKey},
    features::{
        goto_definition::{
            DefinitionQuery,
            resolve as resolve_definition,
            response_value as definition_response_value,
        },
        lsp_range::{byte_range_to_lsp_range, position_to_offset},
        references::ReferenceQuery,
        rename::{PrepareRenameQuery, RenameQuery, prepare_rename_target, validate_new_name},
        semantic_tokens::{capability as semantic_tokens_capability, empty_response_value, response_value},
    },
    panic_boundary::catch_unwind,
    pending::{PendingFeature, PendingRequest, cancel_drained},
    project_model::{ProjectModel, uri_to_file_path},
    response_pool::{
        OpenSnapshot,
        REQUEST_FAILED,
        RenameResult,
        ResponseCompletion,
        ResponseJob,
        ResponsePool,
        ResponseResult,
    },
    scheduler::{PackageAnalysis, Scheduler, WorkerEvent},
    semantics::{CachedDocumentView, CachedPackageAnalysis},
};
use anyhow::{Context, Result};
use lsp_server::{Connection, ErrorCode, Message, Notification, Request, RequestId, Response, ResponseError};
use lsp_types::{
    CancelParams,
    DidChangeTextDocumentParams,
    DidCloseTextDocumentParams,
    DidOpenTextDocumentParams,
    GotoDefinitionParams,
    InitializeParams,
    InitializeResult,
    NumberOrString,
    OneOf,
    PrepareRenameResponse,
    Range,
    ReferenceParams,
    RenameOptions,
    RenameParams,
    SemanticTokensParams,
    ServerCapabilities,
    ServerInfo,
    TextDocumentContentChangeEvent,
    TextDocumentPositionParams,
    TextDocumentSyncCapability,
    TextDocumentSyncKind,
    TextDocumentSyncOptions,
    Uri,
    WorkDoneProgressOptions,
};
use serde_json::Value;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    path::PathBuf,
    process::ExitCode,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

/// JSON-RPC code for internal server errors.
const INTERNAL_ERROR: i32 = -32603;
/// JSON-RPC code for unsupported LSP methods.
const METHOD_NOT_FOUND: i32 = -32601;

/// LSP initialized notification method.
const INITIALIZED: &str = "initialized";
/// LSP process-exit notification method.
const EXIT: &str = "exit";
/// LSP shutdown request method.
const SHUTDOWN: &str = "shutdown";
/// LSP open-document notification method.
const DID_OPEN: &str = "textDocument/didOpen";
/// LSP full-document change notification method.
const DID_CHANGE: &str = "textDocument/didChange";
/// LSP close-document notification method.
const DID_CLOSE: &str = "textDocument/didClose";
/// LSP request-cancellation notification method.
const CANCEL_REQUEST: &str = "$/cancelRequest";
/// LSP semantic-token request method.
const SEMANTIC_TOKENS_FULL: &str = "textDocument/semanticTokens/full";
/// LSP go-to-definition request method.
const TEXT_DOCUMENT_DEFINITION: &str = "textDocument/definition";
/// LSP find-all-references request method.
const TEXT_DOCUMENT_REFERENCES: &str = "textDocument/references";
/// LSP rename request method.
const TEXT_DOCUMENT_RENAME: &str = "textDocument/rename";
/// LSP prepare-rename request method.
const TEXT_DOCUMENT_PREPARE_RENAME: &str = "textDocument/prepareRename";
/// Maximum package analyses retained on the routing thread.
const MAX_PACKAGE_CACHE_ENTRIES: usize = 8;
/// Maximum pending go-to-definition requests across all packages.
const MAX_PENDING_DEFINITIONS: usize = 128;
/// Maximum pending go-to-definition requests waiting on one package key.
const MAX_PENDING_DEFINITIONS_PER_KEY: usize = 16;
/// Maximum pending references requests across all packages.
const MAX_PENDING_REFERENCES: usize = 128;
/// Maximum pending references requests waiting on one package key.
const MAX_PENDING_REFERENCES_PER_KEY: usize = 16;
/// Maximum pending rename requests across all packages.
const MAX_PENDING_RENAMES: usize = 128;
/// Maximum pending rename requests waiting on one package key.
const MAX_PENDING_RENAMES_PER_KEY: usize = 16;
/// Maximum pending prepare-rename requests across all packages.
const MAX_PENDING_PREPARE_RENAMES: usize = 128;
/// Maximum pending prepare-rename requests waiting on one package key.
const MAX_PENDING_PREPARE_RENAMES_PER_KEY: usize = 16;

/// In-memory state for one running `leo-lsp` server instance.
///
/// This struct owns the protocol lifecycle flags, the currently open-document
/// view of the workspace, the lightweight package-root cache, and the worker
/// thread used for background analysis. The event loop mutates this state in
/// response to incoming LSP messages and worker completions.
#[derive(Debug)]
struct ServerState {
    shutdown_requested: bool,
    exit_code: Option<ExitCode>,
    workspace_roots: Vec<PathBuf>,
    documents: DocumentStore,
    project_model: ProjectModel,
    scheduler: Scheduler,
    response_pool: ResponsePool,
    analysis: AnalysisCaches,
    semantic_token_requests: SemanticTokenRequestState,
    definition_requests: DefinitionRequestState,
    reference_requests: ReferencesRequestState,
    rename_requests: RenameRequestState,
    prepare_rename_requests: PrepareRenameRequestState,
    client_definition_link_support: bool,
    hooks: TestHooks,
}

/// Shared semantic analysis and encoded document views.
#[derive(Debug, Default)]
struct AnalysisCaches {
    /// Package analyses keyed by exact package/bucket generation.
    packages: HashMap<PackageAnalysisKey, Arc<CachedPackageAnalysis>>,
    /// FIFO package cache order, capped so stale packages cannot accumulate.
    package_order: VecDeque<PackageAnalysisKey>,
    /// Latest encoded view per URI. The embedded key proves freshness.
    document_views: HashMap<Uri, CachedDocumentView>,
    /// Package keys whose latest analysis panicked; repeated requests fail fast.
    failed_packages: HashSet<PackageAnalysisKey>,
    /// Document-view keys whose latest token-view build panicked.
    failed_views: HashSet<DocumentViewKey>,
    /// Package analyses scheduled or running on the worker.
    in_flight_packages: HashSet<PackageAnalysisKey>,
    /// Document views scheduled or running on the worker.
    in_flight_views: HashSet<DocumentViewKey>,
}

/// Pending semantic-token requests keyed by exact document-view freshness.
#[derive(Debug, Default)]
struct SemanticTokenRequestState {
    /// Waiters grouped by the document-view key that will answer them.
    pending_by_key: HashMap<DocumentViewKey, Vec<RequestId>>,
    /// Reverse lookup used to handle LSP `$/cancelRequest` in O(1) by ID.
    pending_owner: HashMap<RequestId, DocumentViewKey>,
}

/// Pending go-to-definition requests keyed by package analysis.
#[derive(Debug, Default)]
struct DefinitionRequestState {
    /// Waiters grouped by package analysis, each preserving its own cursor query.
    pending_by_package: HashMap<PackageAnalysisKey, Vec<PendingDefinitionRequest>>,
    /// Reverse lookup used to remove a cancelled request from its package queue.
    pending_owner: HashMap<RequestId, PackageAnalysisKey>,
}

/// One pending definition request with its own cursor query preserved.
#[derive(Debug, Clone)]
struct PendingDefinitionRequest {
    /// Original LSP request ID to answer once package analysis is available.
    id: RequestId,
    /// Cursor and freshness state captured when the request arrived.
    query: DefinitionQuery,
}

/// Pending references requests keyed by package analysis.
#[derive(Debug, Default)]
struct ReferencesRequestState {
    /// Waiters and in-flight conversions grouped by package analysis.
    pending_by_package: HashMap<PackageAnalysisKey, Vec<PendingReferencesRequest>>,
    /// Reverse lookup used to cancel or complete a request by ID.
    pending_owner: HashMap<RequestId, PackageAnalysisKey>,
}

/// One pending references request with cancellation state owned by routing.
#[derive(Debug, Clone)]
struct PendingReferencesRequest {
    /// Original LSP request ID to answer when conversion completes.
    id: RequestId,
    /// Cursor and freshness state captured when the request arrived.
    query: ReferenceQuery,
    /// Cancellation flag observed by the response pool.
    cancel: Arc<AtomicBool>,
    /// Whether this request has already been handed to the response pool.
    dispatched: bool,
}

/// Pending rename requests keyed by package analysis.
#[derive(Debug, Default)]
struct RenameRequestState {
    /// Waiters and in-flight conversions grouped by package analysis.
    pending_by_package: HashMap<PackageAnalysisKey, Vec<PendingRenameRequest>>,
    /// Reverse lookup used to cancel or complete a request by ID.
    pending_owner: HashMap<RequestId, PackageAnalysisKey>,
}

/// One pending rename request with cancellation state owned by routing.
#[derive(Debug, Clone)]
struct PendingRenameRequest {
    /// Original LSP request ID to answer when conversion completes.
    id: RequestId,
    /// Cursor, view freshness, and validated new-name state.
    query: RenameQuery,
    /// Cancellation flag observed by the response pool.
    cancel: Arc<AtomicBool>,
    /// Whether this request has already been handed to the response pool.
    dispatched: bool,
}

/// Pending prepare-rename requests keyed by package analysis.
#[derive(Debug, Default)]
struct PrepareRenameRequestState {
    /// Waiters grouped by package analysis. Prepare-rename never enters the pool.
    pending_by_package: HashMap<PackageAnalysisKey, Vec<PendingPrepareRenameRequest>>,
    /// Reverse lookup used to cancel a request by ID.
    pending_owner: HashMap<RequestId, PackageAnalysisKey>,
}

/// One pending prepare-rename request, answered synchronously on cache hit.
#[derive(Debug, Clone)]
struct PendingPrepareRenameRequest {
    /// Original LSP request ID to answer when analysis arrives.
    id: RequestId,
    /// Cursor and view freshness captured when the request arrived.
    query: PrepareRenameQuery,
}

/// Run the production LSP server with hooks loaded from the process environment.
pub(crate) fn run(connection: Connection) -> Result<ExitCode> {
    run_with_hooks(connection, TestHooks::from_env())
}

/// Run the initialized server loop with optional test fault-injection hooks.
fn run_with_hooks(connection: Connection, hooks: TestHooks) -> Result<ExitCode> {
    let (request_id, params) = connection.initialize_start()?;
    let initialize_params: InitializeParams =
        serde_json::from_value(params).context("failed to deserialize initialize params")?;
    let workspace_roots = collect_workspace_roots(&initialize_params);
    let client_definition_link_support = client_supports_definition_links(&initialize_params);

    // Finish the initialize handshake before any main-loop state exists so the
    // steady-state server only has to reason about post-initialize traffic.
    let initialize_result = InitializeResult {
        capabilities: server_capabilities(),
        server_info: Some(ServerInfo {
            name: "leo-lsp".to_owned(),
            version: Some(env!("CARGO_PKG_VERSION").to_owned()),
        }),
    };

    connection.initialize_finish(request_id, serde_json::to_value(initialize_result)?)?;

    let mut state = ServerState {
        shutdown_requested: false,
        exit_code: None,
        workspace_roots,
        documents: DocumentStore::default(),
        project_model: ProjectModel::default(),
        scheduler: Scheduler::new(hooks.panic_on_worker_job),
        response_pool: ResponsePool::new(),
        analysis: AnalysisCaches::default(),
        semantic_token_requests: SemanticTokenRequestState::default(),
        definition_requests: DefinitionRequestState::default(),
        reference_requests: ReferencesRequestState::default(),
        rename_requests: RenameRequestState::default(),
        prepare_rename_requests: PrepareRenameRequestState::default(),
        client_definition_link_support,
        hooks,
    };

    loop {
        // Route client messages and worker completions from the same loop so
        // main-thread protocol handling stays responsive while background work
        // progresses independently.
        crossbeam_channel::select! {
            recv(connection.receiver) -> message => {
                match message {
                    Ok(message) => {
                        if state.handle_message(&connection, message)? {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
            recv(state.scheduler.events()) -> event => {
                if let Ok(event) = event {
                    state.handle_worker_event(&connection, event);
                }
            }
            recv(state.response_pool.completions()) -> completion => {
                if let Ok(completion) = completion {
                    state.handle_response_completion(&connection, completion);
                }
            }
        }
    }

    state.scheduler.shutdown();
    state.response_pool.shutdown();
    Ok(state.exit_code.unwrap_or(ExitCode::from(1)))
}

impl ServerState {
    /// Route one inbound LSP message to request, notification, or response handling.
    fn handle_message(&mut self, connection: &Connection, message: Message) -> Result<bool> {
        match message {
            Message::Request(request) => {
                self.handle_request(connection, request)?;
                Ok(false)
            }
            Message::Notification(notification) => self.handle_notification(connection, notification),
            Message::Response(response) => {
                tracing::debug!(id = ?response.id, "ignoring unexpected client response");
                Ok(false)
            }
        }
    }

    /// Handle an LSP request inside a panic boundary that can still answer the client.
    fn handle_request(&mut self, connection: &Connection, request: Request) -> Result<()> {
        let Request { id: request_id, method, params } = request;

        // Request dispatch is a top-level task boundary: a panic here still
        // indicates a bug, but containing it lets the server reply with an
        // internal error instead of taking down the whole editor session.
        match catch_unwind("request_dispatch", None, None, || {
            self.dispatch_request(connection, request_id.clone(), method.as_str(), params)
        }) {
            Ok(result) => result,
            Err(report) => {
                report.log();
                send_error_response(connection, request_id, INTERNAL_ERROR, format!("request `{method}` panicked"))
            }
        }
    }

    /// Dispatch a deserialized request by method name.
    fn dispatch_request(
        &mut self,
        connection: &Connection,
        request_id: RequestId,
        method: &str,
        params: Value,
    ) -> Result<()> {
        self.hooks.maybe_panic_request(method);

        match method {
            SHUTDOWN => {
                self.shutdown_requested = true;
                send_ok_response(connection, request_id, Value::Null)
            }
            SEMANTIC_TOKENS_FULL => {
                let params: SemanticTokensParams =
                    serde_json::from_value(params).context("failed to deserialize semanticTokens/full")?;
                self.handle_semantic_tokens_full(connection, request_id, params)
            }
            TEXT_DOCUMENT_DEFINITION => {
                let params: GotoDefinitionParams =
                    serde_json::from_value(params).context("failed to deserialize textDocument/definition")?;
                self.handle_goto_definition(connection, request_id, params)
            }
            TEXT_DOCUMENT_REFERENCES => {
                let params: ReferenceParams =
                    serde_json::from_value(params).context("failed to deserialize textDocument/references")?;
                self.handle_references(connection, request_id, params)
            }
            TEXT_DOCUMENT_RENAME => {
                let params: RenameParams =
                    serde_json::from_value(params).context("failed to deserialize textDocument/rename")?;
                self.handle_rename(connection, request_id, params)
            }
            TEXT_DOCUMENT_PREPARE_RENAME => {
                let params: TextDocumentPositionParams =
                    serde_json::from_value(params).context("failed to deserialize textDocument/prepareRename")?;
                self.handle_prepare_rename(connection, request_id, params)
            }
            _ => {
                tracing::debug!(method, "request is not implemented");
                send_error_response(connection, request_id, METHOD_NOT_FOUND, "method not found")
            }
        }
    }

    /// Handle an LSP notification inside a logging-only panic boundary.
    fn handle_notification(&mut self, connection: &Connection, notification: Notification) -> Result<bool> {
        let method = notification.method.clone();

        // Notifications do not have a request ID to fail back through, so this
        // outer boundary is the narrowest place we can contain a panic, log it
        // as a bug, and keep the process alive for the next message.
        match catch_unwind("notification_dispatch", None, None, || self.dispatch_notification(connection, notification))
        {
            Ok(Ok(result)) => Ok(result),
            Ok(Err(error)) => {
                // Client notifications do not have request IDs, so the least
                // disruptive behavior is to log the failure and keep serving.
                tracing::error!(method = %method, error = %error, "failed to handle client notification");
                Ok(false)
            }
            Err(report) => {
                report.log();
                Ok(false)
            }
        }
    }

    /// Dispatch a deserialized notification by method name.
    fn dispatch_notification(&mut self, connection: &Connection, notification: Notification) -> Result<bool> {
        match notification.method.as_str() {
            INITIALIZED => Ok(false),
            EXIT => {
                self.exit_code = Some(if self.shutdown_requested { ExitCode::SUCCESS } else { ExitCode::from(1) });
                Ok(true)
            }
            DID_OPEN => {
                let params: DidOpenTextDocumentParams =
                    serde_json::from_value(notification.params).context("failed to deserialize didOpen")?;
                self.handle_did_open(connection, params);
                Ok(false)
            }
            DID_CHANGE => {
                let params: DidChangeTextDocumentParams =
                    serde_json::from_value(notification.params).context("failed to deserialize didChange")?;
                self.handle_did_change(connection, params);
                Ok(false)
            }
            DID_CLOSE => {
                let params: DidCloseTextDocumentParams =
                    serde_json::from_value(notification.params).context("failed to deserialize didClose")?;
                self.handle_did_close(connection, params);
                Ok(false)
            }
            CANCEL_REQUEST => {
                let params: CancelParams =
                    serde_json::from_value(notification.params).context("failed to deserialize $/cancelRequest")?;
                self.handle_cancel_request(connection, params)?;
                Ok(false)
            }
            _ => {
                tracing::debug!(method = %notification.method, "ignoring unknown notification");
                Ok(false)
            }
        }
    }

    /// Commit an opened document, invalidate its package bucket, and enqueue analysis.
    fn handle_did_open(&mut self, connection: &Connection, params: DidOpenTextDocumentParams) {
        let document = params.text_document;
        let previous_bucket = self.documents.package_key(&document.uri).map(|key| key.bucket);
        // Resolve the package root before commit so the worker snapshot and the
        // main-thread document state observe the same project context.
        let (file_path, project) = self.project_model.resolve_document_context(&document.uri);
        let prepared = self.documents.prepare_open(
            document.uri,
            document.language_id,
            document.version,
            document.text,
            file_path,
            project,
        );

        self.hooks.maybe_panic_notification(DID_OPEN);

        let snapshot = self.documents.commit_open(prepared);
        self.invalidate_bucket_for_new_snapshot(
            connection,
            previous_bucket.as_ref(),
            &snapshot.package_key.bucket,
            "package analysis was superseded",
        );
        self.scheduler.set_open_buckets(self.documents.open_buckets());
        self.analysis.in_flight_packages.insert(snapshot.package_key.clone());
        self.scheduler.enqueue_package(snapshot);
    }

    /// Commit a full-document change and refresh package ownership before analysis.
    fn handle_did_change(&mut self, connection: &Connection, params: DidChangeTextDocumentParams) {
        let DidChangeTextDocumentParams { text_document, content_changes } = params;
        let previous_bucket = self.documents.package_key(&text_document.uri).map(|key| key.bucket);

        let Some(text) = extract_full_sync_text(content_changes) else {
            return;
        };
        // Re-resolve package ownership on every committed edit so semantic
        // analysis notices manifests appearing, disappearing, or moving while
        // the document stays open.
        let (file_path, project) = self.project_model.resolve_document_context(&text_document.uri);

        let Some(prepared) =
            self.documents.prepare_full_change(&text_document.uri, text_document.version, text, file_path, project)
        else {
            tracing::debug!(uri = text_document.uri.as_str(), "ignoring didChange for unopened document");
            return;
        };

        self.hooks.maybe_panic_notification(DID_CHANGE);

        let snapshot = self.documents.commit_change(prepared);
        self.invalidate_bucket_for_new_snapshot(
            connection,
            previous_bucket.as_ref(),
            &snapshot.package_key.bucket,
            "package analysis was superseded",
        );
        self.scheduler.set_open_buckets(self.documents.open_buckets());
        self.analysis.in_flight_packages.insert(snapshot.package_key.clone());
        self.scheduler.enqueue_package(snapshot);
    }

    /// Close a document and flush or cancel any waiters tied to its bucket.
    fn handle_did_close(&mut self, connection: &Connection, params: DidCloseTextDocumentParams) {
        self.hooks.maybe_panic_notification(DID_CLOSE);
        let uri = params.text_document.uri;
        let previous_bucket = self.documents.package_key(&uri).map(|key| key.bucket);
        self.documents.close(&uri);
        self.scheduler.set_open_buckets(self.documents.open_buckets());
        if let Err(error) =
            send_ok_responses(connection, self.semantic_token_requests.clear_uri(&uri), empty_response_value())
        {
            tracing::error!(uri = uri.as_str(), error = %error, "failed to flush semantic token close responses");
        }
        if let Err(error) = send_definition_nulls(connection, self.definition_requests.clear_uri(&uri)) {
            tracing::error!(uri = uri.as_str(), error = %error, "failed to flush definition close responses");
        }
        if let Err(error) = send_reference_nulls(connection, self.reference_requests.clear_uri(&uri)) {
            tracing::error!(uri = uri.as_str(), error = %error, "failed to flush references close responses");
        }
        // Rename and prepare-rename reply with `RequestCanceled` on close
        // because the user closing the document is backing out of the
        // action, not the server stating "not renameable".
        log_drain(
            cancel_drained(
                connection,
                self.rename_requests.drain_uri(&uri),
                ErrorCode::RequestCanceled as i32,
                "rename request cancelled by close",
            ),
            "rename close waiters",
        );
        log_drain(
            cancel_drained(
                connection,
                self.prepare_rename_requests.drain_uri(&uri),
                ErrorCode::RequestCanceled as i32,
                "prepare-rename request cancelled by close",
            ),
            "prepare-rename close waiters",
        );
        if let Some(bucket) = previous_bucket {
            self.analysis.invalidate_bucket(&bucket);
            self.cancel_pending_bucket_requests(connection, &bucket, "package analysis was superseded");
        } else {
            self.analysis.invalidate_uri(&uri);
        }
    }

    /// Remove a pending semantic-token, definition, or references request by LSP request ID.
    fn handle_cancel_request(&mut self, connection: &Connection, params: CancelParams) -> Result<()> {
        let request_id = request_id_from_cancel(params.id);
        if self.semantic_token_requests.remove_pending_request(&request_id) {
            send_error_response(
                connection,
                request_id,
                ErrorCode::RequestCanceled as i32,
                "semantic token request cancelled",
            )
        } else if self.definition_requests.remove_pending_request(&request_id).is_some() {
            send_error_response(
                connection,
                request_id,
                ErrorCode::RequestCanceled as i32,
                "definition request cancelled",
            )
        } else if let Some(pending) = self.reference_requests.remove_pending_request(&request_id) {
            pending.cancel.store(true, Ordering::SeqCst);
            send_error_response(
                connection,
                request_id,
                ErrorCode::RequestCanceled as i32,
                "references request cancelled",
            )
        } else if let Some(pending) = self.rename_requests.remove_pending_request(&request_id) {
            pending.cancel.store(true, Ordering::SeqCst);
            send_error_response(connection, request_id, ErrorCode::RequestCanceled as i32, "rename request cancelled")
        } else if self.prepare_rename_requests.remove_pending_request(&request_id).is_some() {
            send_error_response(
                connection,
                request_id,
                ErrorCode::RequestCanceled as i32,
                "prepare-rename request cancelled",
            )
        } else {
            Ok(())
        }
    }

    /// Apply one worker event to caches and answer any pending client requests.
    fn handle_worker_event(&mut self, connection: &Connection, event: WorkerEvent) {
        match event {
            WorkerEvent::PackageAnalyzed(PackageAnalysis { uri, generation, key, result }) => {
                self.analysis.in_flight_packages.remove(&key);
                if self.documents.generation(&uri) == Some(generation)
                    && self.documents.package_key(&uri) == Some(key.clone())
                {
                    for evicted in self.analysis.store_package(Arc::clone(&result.package)) {
                        self.cancel_pending_package_requests(
                            connection,
                            &evicted,
                            "package analysis was evicted from cache",
                        );
                    }
                    self.store_document_view(connection, result.document_view);
                    self.answer_pending_definitions(connection, &key);
                    self.answer_pending_references(&key);
                    self.answer_pending_prepare_renames(connection, &key);
                    self.answer_pending_renames(&key);
                    self.enqueue_pending_document_views_for_package(&key);
                    tracing::debug!(
                        uri = uri.as_str(),
                        generation,
                        workspace_roots = self.workspace_roots.len(),
                        "worker completed latest document"
                    );
                } else {
                    self.cancel_pending_package_requests(connection, &key, "package analysis was superseded");
                    tracing::debug!(uri = uri.as_str(), generation, "dropping stale package worker completion");
                }
            }
            WorkerEvent::DocumentViewBuilt(view) => {
                self.analysis.in_flight_views.remove(&view.key);
                if self.documents.document_view_key(&view.key.uri) == Some(view.key.clone()) {
                    self.store_document_view(connection, view);
                } else {
                    self.cancel_pending_document_view_requests(
                        connection,
                        &view.key,
                        "semantic token document view was superseded",
                    );
                }
            }
            WorkerEvent::PackageCancelled { key, uri, generation } => {
                self.analysis.in_flight_packages.remove(&key);
                self.cancel_pending_package_requests(connection, &key, "package analysis was cancelled");
                tracing::debug!(uri = uri.as_str(), generation, "worker cancelled stale package analysis");
            }
            WorkerEvent::DocumentViewCancelled { key } => {
                self.analysis.in_flight_views.remove(&key);
                self.cancel_pending_document_view_requests(
                    connection,
                    &key,
                    "semantic token document view was cancelled",
                );
                tracing::debug!(
                    uri = key.uri.as_str(),
                    generation = key.document_generation,
                    "worker cancelled stale document view"
                );
            }
            WorkerEvent::PackagePanicked { key, uri, generation, report } => {
                report.log();
                self.analysis.in_flight_packages.remove(&key);
                if self.documents.generation(&uri) == Some(generation)
                    && self.documents.package_key(&uri) == Some(key.clone())
                {
                    self.analysis.store_failed_package(key.clone());
                    self.fail_pending_package_requests(
                        connection,
                        &key,
                        "analysis panicked; see server logs for details",
                    );
                } else {
                    self.cancel_pending_package_requests(connection, &key, "package analysis was superseded");
                }
            }
            WorkerEvent::DocumentViewPanicked { key, report } => {
                report.log();
                self.analysis.in_flight_views.remove(&key);
                if self.documents.document_view_key(&key.uri) == Some(key.clone()) {
                    self.analysis.failed_views.insert(key.clone());
                    if let Err(error) = send_error_responses(
                        connection,
                        self.semantic_token_requests.take_key(&key),
                        INTERNAL_ERROR,
                        "semantic token document view panicked; see server logs for details",
                    ) {
                        tracing::error!(uri = key.uri.as_str(), error = %error, "failed to send document-view panic");
                    }
                } else {
                    self.cancel_pending_document_view_requests(
                        connection,
                        &key,
                        "semantic token document view was superseded",
                    );
                }
            }
        }
    }

    /// Forward a response-pool completion if the request is still live.
    fn handle_response_completion(&mut self, connection: &Connection, completion: ResponseCompletion) {
        match completion {
            ResponseCompletion::References { id, key, cancel, result } => {
                let Some(pending) = self.reference_requests.get(&id) else {
                    return;
                };
                // JSON-RPC IDs can be reused after cancellation. The package
                // key proves freshness for the analysis; the Arc identity
                // proves this completion belongs to the same dispatched waiter,
                // not an older request that happened to use the same ID.
                if pending.query.view_key.package != key || !Arc::ptr_eq(&pending.cancel, &cancel) {
                    return;
                }
                let Some(_pending) = self.reference_requests.remove_pending_request(&id) else {
                    return;
                };
                let send_result = match result {
                    ResponseResult::Ok(value) => send_ok_response(connection, id, value),
                    ResponseResult::InternalError(message) => {
                        send_error_response(connection, id, INTERNAL_ERROR, message)
                    }
                };
                if let Err(error) = send_result {
                    tracing::error!(error = %error, "failed to send references response");
                }
            }
            ResponseCompletion::Rename { id, key, cancel, result } => {
                let Some(pending) = self.rename_requests.get(&id) else {
                    return;
                };
                if pending.query.view_key.package != key || !Arc::ptr_eq(&pending.cancel, &cancel) {
                    return;
                }
                let Some(_pending) = self.rename_requests.remove_pending_request(&id) else {
                    return;
                };
                let send_result = match result {
                    RenameResult::Ok(value) => send_ok_response(connection, id, value),
                    RenameResult::InternalError(message) => {
                        send_error_response(connection, id, INTERNAL_ERROR, message)
                    }
                    RenameResult::RequestFailed { code, message } => send_error_response(connection, id, code, message),
                };
                if let Err(error) = send_result {
                    tracing::error!(error = %error, "failed to send rename response");
                }
            }
        }
    }

    /// Answer or queue one full semantic-token request.
    fn handle_semantic_tokens_full(
        &mut self,
        connection: &Connection,
        request_id: RequestId,
        params: SemanticTokensParams,
    ) -> Result<()> {
        let uri = params.text_document.uri;
        let Some(view_key) = self.documents.document_view_key(&uri) else {
            return send_ok_response(connection, request_id, empty_response_value());
        };

        if let Some(view) = self.analysis.document_view(&view_key) {
            return send_ok_response(connection, request_id, response_value(view.encoded_tokens.as_ref()));
        }

        if self.analysis.failed_views.contains(&view_key) || self.analysis.failed_packages.contains(&view_key.package) {
            return send_error_response(
                connection,
                request_id,
                INTERNAL_ERROR,
                "semantic token analysis panicked; see server logs for details",
            );
        }

        self.semantic_token_requests.queue(view_key.clone(), request_id);
        self.ensure_analysis_for_view(&view_key);
        Ok(())
    }

    /// Answer or queue one go-to-definition request.
    fn handle_goto_definition(
        &mut self,
        connection: &Connection,
        request_id: RequestId,
        params: GotoDefinitionParams,
    ) -> Result<()> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let Some(document) = self.documents.open_document(&uri) else {
            return send_ok_response(connection, request_id, Value::Null);
        };
        let Some(file_path) = document.file_path.clone() else {
            return send_ok_response(connection, request_id, Value::Null);
        };
        let Some(offset) = position_to_offset(document.line_index.as_ref(), position) else {
            return send_ok_response(connection, request_id, Value::Null);
        };
        let Some(view_key) = self.documents.document_view_key(&uri) else {
            return send_ok_response(connection, request_id, Value::Null);
        };

        let query = DefinitionQuery {
            uri: uri.clone(),
            file_path,
            position,
            offset,
            line_index: Arc::clone(&document.line_index),
            view_key: view_key.clone(),
            link_support: self.client_definition_link_support,
        };

        if let Some(package) = self.analysis.packages.get(&view_key.package) {
            return send_ok_response(
                connection,
                request_id,
                definition_response_value(resolve_definition(&query, package)),
            );
        }

        if self.analysis.failed_packages.contains(&view_key.package) {
            return send_error_response(
                connection,
                request_id,
                INTERNAL_ERROR,
                "definition analysis panicked; see server logs for details",
            );
        }

        if self.definition_requests.queue(query, request_id.clone()) {
            // Definition resolution needs the package index but not the encoded
            // token view, so pending requests wait at package granularity.
            self.ensure_package_analysis(&view_key.package, &uri);
            Ok(())
        } else {
            send_error_response(
                connection,
                request_id,
                ErrorCode::RequestCanceled as i32,
                "too many pending definition requests",
            )
        }
    }

    /// Answer or queue one find-all-references request.
    fn handle_references(
        &mut self,
        connection: &Connection,
        request_id: RequestId,
        params: ReferenceParams,
    ) -> Result<()> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let Some(document) = self.documents.open_document(&uri) else {
            return send_ok_response(connection, request_id, Value::Null);
        };
        if document.file_path.is_none() {
            return send_ok_response(connection, request_id, Value::Null);
        }
        let Some(offset) = position_to_offset(document.line_index.as_ref(), position) else {
            return send_ok_response(connection, request_id, Value::Null);
        };
        let Some(view_key) = self.documents.document_view_key(&uri) else {
            return send_ok_response(connection, request_id, Value::Null);
        };

        if self.analysis.failed_packages.contains(&view_key.package) {
            return send_error_response(
                connection,
                request_id,
                INTERNAL_ERROR,
                "references analysis panicked; see server logs for details",
            );
        }

        let package = self.analysis.packages.get(&view_key.package).cloned();
        let query = ReferenceQuery {
            offset,
            position,
            view_key: view_key.clone(),
            include_declaration: params.context.include_declaration,
        };
        let cancel = Arc::new(AtomicBool::new(false));
        let dispatched = package.is_some();
        if !self.reference_requests.queue(query, request_id.clone(), Arc::clone(&cancel), dispatched) {
            return send_error_response(
                connection,
                request_id,
                ErrorCode::RequestCanceled as i32,
                "too many pending references requests",
            );
        }

        if let Some(package) = package {
            self.dispatch_reference_job(request_id, view_key.package, package, cancel);
        } else {
            self.ensure_package_analysis(&view_key.package, &uri);
        }
        Ok(())
    }

    /// Answer or queue one prepare-rename request.
    fn handle_prepare_rename(
        &mut self,
        connection: &Connection,
        request_id: RequestId,
        params: TextDocumentPositionParams,
    ) -> Result<()> {
        let uri = params.text_document.uri;
        let position = params.position;
        let Some(document) = self.documents.open_document(&uri) else {
            return send_ok_response(connection, request_id, Value::Null);
        };
        let Some(file_path) = document.file_path.clone() else {
            return send_ok_response(connection, request_id, Value::Null);
        };
        let Some(offset) = position_to_offset(document.line_index.as_ref(), position) else {
            return send_ok_response(connection, request_id, Value::Null);
        };
        let line_index = Arc::clone(&document.line_index);
        let Some(view_key) = self.documents.document_view_key(&uri) else {
            return send_ok_response(connection, request_id, Value::Null);
        };

        if self.analysis.failed_packages.contains(&view_key.package) {
            return send_error_response(
                connection,
                request_id,
                INTERNAL_ERROR,
                "prepare-rename analysis panicked; see server logs for details",
            );
        }

        let query = PrepareRenameQuery { offset, position, view_key: view_key.clone() };

        if let Some(package) = self.analysis.packages.get(&view_key.package).cloned() {
            let value =
                prepare_rename_response_value(&query, package.as_ref(), file_path.as_ref(), line_index.as_ref());
            return send_ok_response(connection, request_id, value);
        }

        if !self.prepare_rename_requests.queue(query, request_id.clone()) {
            return send_error_response(
                connection,
                request_id,
                ErrorCode::RequestCanceled as i32,
                "too many pending prepare-rename requests",
            );
        }
        self.ensure_package_analysis(&view_key.package, &uri);
        Ok(())
    }

    /// Answer or queue one rename request.
    fn handle_rename(&mut self, connection: &Connection, request_id: RequestId, params: RenameParams) -> Result<()> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let new_name = params.new_name;

        // Validate the new name synchronously before allocating any pending-state
        // slot or pool resources. Statically unrenameable inputs reject inline.
        if let Err(error) = validate_new_name(&new_name) {
            let crate::features::rename::RenameError::InvalidIdentifier(message) = error else {
                // validate_new_name only ever returns InvalidIdentifier.
                unreachable!("validate_new_name returned non-identifier error");
            };
            return send_error_response(connection, request_id, REQUEST_FAILED, message);
        }

        let Some(document) = self.documents.open_document(&uri) else {
            return send_ok_response(connection, request_id, Value::Null);
        };
        if document.file_path.is_none() {
            return send_ok_response(connection, request_id, Value::Null);
        }
        let Some(offset) = position_to_offset(document.line_index.as_ref(), position) else {
            return send_ok_response(connection, request_id, Value::Null);
        };
        let Some(view_key) = self.documents.document_view_key(&uri) else {
            return send_ok_response(connection, request_id, Value::Null);
        };

        if self.analysis.failed_packages.contains(&view_key.package) {
            return send_error_response(
                connection,
                request_id,
                INTERNAL_ERROR,
                "rename analysis panicked; see server logs for details",
            );
        }

        let package = self.analysis.packages.get(&view_key.package).cloned();
        let query = RenameQuery { offset, position, view_key: view_key.clone(), new_name };
        let cancel = Arc::new(AtomicBool::new(false));
        let dispatched = package.is_some();
        if !self.rename_requests.queue(query, request_id.clone(), Arc::clone(&cancel), dispatched) {
            return send_error_response(
                connection,
                request_id,
                ErrorCode::RequestCanceled as i32,
                "too many pending rename requests",
            );
        }

        if let Some(package) = package {
            self.dispatch_rename_job(request_id, view_key.package, package, cancel);
        } else {
            self.ensure_package_analysis(&view_key.package, &uri);
        }
        Ok(())
    }

    /// Ensure the package and document-view analysis needed for a token request is queued.
    fn ensure_analysis_for_view(&mut self, view_key: &DocumentViewKey) {
        if let Some(package) = self.analysis.packages.get(&view_key.package).cloned() {
            self.ensure_document_view(view_key, package);
        } else {
            self.ensure_package_analysis(&view_key.package, &view_key.uri);
        }
    }

    /// Queue package analysis unless the exact package key is cached or in flight.
    fn ensure_package_analysis(&mut self, package_key: &PackageAnalysisKey, uri: &Uri) {
        if self.analysis.in_flight_packages.contains(package_key) || self.analysis.packages.contains_key(package_key) {
            return;
        }
        let Some(snapshot) = self.documents.snapshot_for_package_analysis(uri) else {
            return;
        };
        self.analysis.in_flight_packages.insert(snapshot.package_key.clone());
        self.scheduler.enqueue_package(snapshot);
    }

    /// Queue a document-view rebuild against a cached package analysis.
    fn ensure_document_view(&mut self, view_key: &DocumentViewKey, package: Arc<CachedPackageAnalysis>) {
        if self.analysis.in_flight_views.contains(view_key) || self.analysis.document_view(view_key).is_some() {
            return;
        }
        let Some(snapshot) = self.documents.snapshot_for_document_view(&view_key.uri) else {
            return;
        };
        self.analysis.in_flight_views.insert(snapshot.key.clone());
        self.scheduler.enqueue_document_view(snapshot, package);
    }

    /// Cache an encoded document view and answer matching semantic-token waiters.
    fn store_document_view(&mut self, connection: &Connection, view: CachedDocumentView) {
        let key = view.key.clone();
        let uri = key.uri.clone();
        // Cache by URI for cheap steady-state lookup; `document_view` checks the
        // embedded key so a stale view never answers a newer generation.
        self.analysis.document_views.insert(uri.clone(), view.clone());
        let pending = self.semantic_token_requests.take_key(&key);
        if let Err(error) = send_ok_responses(connection, pending, response_value(view.encoded_tokens.as_ref())) {
            tracing::error!(uri = uri.as_str(), error = %error, "failed to send semantic token response");
        }
    }

    /// Resolve all queued definition requests waiting on one package analysis.
    fn answer_pending_definitions(&mut self, connection: &Connection, key: &PackageAnalysisKey) {
        let Some(package) = self.analysis.packages.get(key).cloned() else {
            return;
        };
        for pending in self.definition_requests.take_package(key) {
            // Each definition request keeps its original cursor offset and line
            // index, so many requests can share one package analysis without
            // collapsing to the same target.
            let value = definition_response_value(resolve_definition(&pending.query, package.as_ref()));
            if let Err(error) = send_ok_response(connection, pending.id, value) {
                tracing::error!(error = %error, "failed to send definition response");
            }
        }
    }

    /// Dispatch queued references requests unblocked by a cached package.
    fn answer_pending_references(&mut self, key: &PackageAnalysisKey) {
        let Some(package) = self.analysis.packages.get(key).cloned() else {
            return;
        };
        for pending in self.reference_requests.mark_undispatched(key) {
            self.dispatch_reference_job(pending.id, key.clone(), Arc::clone(&package), Arc::clone(&pending.cancel));
        }
    }

    /// Hand one references request to the response pool.
    fn dispatch_reference_job(
        &self,
        request_id: RequestId,
        _key: PackageAnalysisKey,
        package: Arc<CachedPackageAnalysis>,
        cancel: Arc<AtomicBool>,
    ) {
        let Some(pending) = self.reference_requests.get(&request_id) else {
            return;
        };
        self.response_pool.submit(ResponseJob::References {
            id: request_id,
            query: Box::new(pending.query.clone()),
            package,
            open_snapshot: OpenSnapshot::snapshot(&self.documents),
            cancel,
        });
    }

    /// Dispatch queued rename requests unblocked by a cached package.
    fn answer_pending_renames(&mut self, key: &PackageAnalysisKey) {
        let Some(package) = self.analysis.packages.get(key).cloned() else {
            return;
        };
        for pending in self.rename_requests.mark_undispatched(key) {
            self.dispatch_rename_job(pending.id, key.clone(), Arc::clone(&package), Arc::clone(&pending.cancel));
        }
    }

    /// Resolve queued prepare-rename requests synchronously against a cached package.
    fn answer_pending_prepare_renames(&mut self, connection: &Connection, key: &PackageAnalysisKey) {
        let Some(package) = self.analysis.packages.get(key).cloned() else {
            return;
        };
        for pending in self.prepare_rename_requests.take_package(key) {
            let Some(document) = self.documents.open_document(&pending.query.view_key.uri) else {
                if let Err(error) = send_ok_response(connection, pending.id, Value::Null) {
                    tracing::error!(error = %error, "failed to send prepare-rename response");
                }
                continue;
            };
            let Some(file_path) = document.file_path.clone() else {
                if let Err(error) = send_ok_response(connection, pending.id, Value::Null) {
                    tracing::error!(error = %error, "failed to send prepare-rename response");
                }
                continue;
            };
            let line_index = Arc::clone(&document.line_index);
            let value = prepare_rename_response_value(
                &pending.query,
                package.as_ref(),
                file_path.as_ref(),
                line_index.as_ref(),
            );
            if let Err(error) = send_ok_response(connection, pending.id, value) {
                tracing::error!(error = %error, "failed to send prepare-rename response");
            }
        }
    }

    /// Hand one rename request to the response pool.
    fn dispatch_rename_job(
        &self,
        request_id: RequestId,
        _key: PackageAnalysisKey,
        package: Arc<CachedPackageAnalysis>,
        cancel: Arc<AtomicBool>,
    ) {
        let Some(pending) = self.rename_requests.get(&request_id) else {
            return;
        };
        self.response_pool.submit(ResponseJob::Rename {
            id: request_id,
            query: Box::new(pending.query.clone()),
            package,
            open_snapshot: OpenSnapshot::snapshot(&self.documents),
            cancel,
        });
    }

    /// Start document-view jobs unblocked by a newly cached package analysis.
    fn enqueue_pending_document_views_for_package(&mut self, key: &PackageAnalysisKey) {
        let Some(package) = self.analysis.packages.get(key).cloned() else {
            return;
        };
        let keys = self.semantic_token_requests.keys_for_package(key);
        for view_key in keys {
            self.ensure_document_view(&view_key, Arc::clone(&package));
        }
    }

    /// Invalidate stale analysis state when a document enters a new package snapshot.
    fn invalidate_bucket_for_new_snapshot(
        &mut self,
        connection: &Connection,
        previous_bucket: Option<&AnalysisBucket>,
        current_bucket: &AnalysisBucket,
        message: &'static str,
    ) {
        // Package analysis spans all open buffers in the bucket. Any committed
        // edit or bucket move invalidates package-level state, document views,
        // in-flight markers, and pending waiters that depended on old inputs.
        if let Some(previous_bucket) = previous_bucket
            && previous_bucket != current_bucket
        {
            self.analysis.invalidate_bucket(previous_bucket);
            self.cancel_pending_bucket_requests(connection, previous_bucket, message);
        }
        self.analysis.invalidate_bucket(current_bucket);
        self.cancel_pending_bucket_requests(connection, current_bucket, message);
    }

    /// Cancel every pending waiter tied to one analysis bucket.
    ///
    /// `cancel_drained` flips per-feature cancel flags before sending the
    /// reply so an in-flight pool job whose bucket just bumped observes the
    /// cancellation and drops its completion on arrival.
    fn cancel_pending_bucket_requests(
        &mut self,
        connection: &Connection,
        bucket: &AnalysisBucket,
        message: &'static str,
    ) {
        let code = ErrorCode::RequestCanceled as i32;
        log_drain(
            cancel_drained(
                connection,
                self.semantic_token_requests.drain_bucket(bucket),
                code,
                format!("semantic token {message}"),
            ),
            "semantic token bucket waiters",
        );
        log_drain(
            cancel_drained(
                connection,
                self.definition_requests.drain_bucket(bucket),
                code,
                format!("definition {message}"),
            ),
            "definition bucket waiters",
        );
        log_drain(
            cancel_drained(
                connection,
                self.reference_requests.drain_bucket(bucket),
                code,
                format!("references {message}"),
            ),
            "references bucket waiters",
        );
        log_drain(
            cancel_drained(connection, self.rename_requests.drain_bucket(bucket), code, format!("rename {message}")),
            "rename bucket waiters",
        );
        log_drain(
            cancel_drained(
                connection,
                self.prepare_rename_requests.drain_bucket(bucket),
                code,
                format!("prepare-rename {message}"),
            ),
            "prepare-rename bucket waiters",
        );
    }

    /// Fail every pending waiter tied to one package analysis key with `INTERNAL_ERROR`.
    ///
    /// Used by `PackagePanicked` for the still-current package key. Mirrors
    /// the structure of `cancel_pending_package_requests` but maps to
    /// `INTERNAL_ERROR` and a panic-shaped message per feature so the spec's
    /// "shared drain helper... becomes load-bearing" goal is met for the
    /// panic path too.
    fn fail_pending_package_requests(
        &mut self,
        connection: &Connection,
        key: &PackageAnalysisKey,
        message: &'static str,
    ) {
        log_drain(
            cancel_drained(
                connection,
                self.semantic_token_requests.drain_package(key),
                INTERNAL_ERROR,
                format!("semantic token {message}"),
            ),
            "semantic token panic waiters",
        );
        log_drain(
            cancel_drained(
                connection,
                self.definition_requests.drain_package(key),
                INTERNAL_ERROR,
                format!("definition {message}"),
            ),
            "definition panic waiters",
        );
        log_drain(
            cancel_drained(
                connection,
                self.reference_requests.drain_package(key),
                INTERNAL_ERROR,
                format!("references {message}"),
            ),
            "references panic waiters",
        );
        log_drain(
            cancel_drained(
                connection,
                self.rename_requests.drain_package(key),
                INTERNAL_ERROR,
                format!("rename {message}"),
            ),
            "rename panic waiters",
        );
        log_drain(
            cancel_drained(
                connection,
                self.prepare_rename_requests.drain_package(key),
                INTERNAL_ERROR,
                format!("prepare-rename {message}"),
            ),
            "prepare-rename panic waiters",
        );
    }

    /// Cancel every pending waiter tied to one package analysis key.
    fn cancel_pending_package_requests(
        &mut self,
        connection: &Connection,
        key: &PackageAnalysisKey,
        message: &'static str,
    ) {
        let code = ErrorCode::RequestCanceled as i32;
        log_drain(
            cancel_drained(
                connection,
                self.semantic_token_requests.drain_package(key),
                code,
                format!("semantic token {message}"),
            ),
            "semantic token package waiters",
        );
        log_drain(
            cancel_drained(
                connection,
                self.definition_requests.drain_package(key),
                code,
                format!("definition {message}"),
            ),
            "definition package waiters",
        );
        log_drain(
            cancel_drained(
                connection,
                self.reference_requests.drain_package(key),
                code,
                format!("references {message}"),
            ),
            "references package waiters",
        );
        log_drain(
            cancel_drained(connection, self.rename_requests.drain_package(key), code, format!("rename {message}")),
            "rename package waiters",
        );
        log_drain(
            cancel_drained(
                connection,
                self.prepare_rename_requests.drain_package(key),
                code,
                format!("prepare-rename {message}"),
            ),
            "prepare-rename package waiters",
        );
    }

    /// Cancel semantic-token waiters tied to one document-view key.
    fn cancel_pending_document_view_requests(
        &mut self,
        connection: &Connection,
        key: &DocumentViewKey,
        message: &'static str,
    ) {
        if let Err(error) = send_error_responses(
            connection,
            self.semantic_token_requests.take_key(key),
            ErrorCode::RequestCanceled as i32,
            message,
        ) {
            tracing::error!(uri = key.uri.as_str(), error = %error, "failed to cancel semantic token view waiters");
        }
    }
}

impl AnalysisCaches {
    /// Remove URI-local document-view state after close or reclassification.
    fn invalidate_uri(&mut self, uri: &Uri) {
        self.document_views.remove(uri);
        self.failed_views.retain(|key| &key.uri != uri);
        self.in_flight_views.retain(|key| &key.uri != uri);
    }

    /// Remove all package and document-view state for one analysis bucket.
    fn invalidate_bucket(&mut self, bucket: &AnalysisBucket) {
        self.packages.retain(|key, _| &key.bucket != bucket);
        self.package_order.retain(|key| &key.bucket != bucket);
        self.failed_packages.retain(|key| &key.bucket != bucket);
        self.in_flight_packages.retain(|key| &key.bucket != bucket);
        self.document_views.retain(|_, view| &view.key.package.bucket != bucket);
        self.failed_views.retain(|key| &key.package.bucket != bucket);
        self.in_flight_views.retain(|key| &key.package.bucket != bucket);
    }

    /// Return a cached document view only when the embedded key still matches.
    fn document_view(&self, key: &DocumentViewKey) -> Option<&CachedDocumentView> {
        self.document_views.get(&key.uri).filter(|view| &view.key == key)
    }

    /// Store a package analysis and enforce the routing-thread LRU cap.
    fn store_package(&mut self, package: Arc<CachedPackageAnalysis>) -> Vec<PackageAnalysisKey> {
        self.failed_packages.remove(&package.key);
        if !self.packages.contains_key(&package.key) {
            self.package_order.push_back(package.key.clone());
        }
        self.packages.insert(package.key.clone(), package);
        let mut evicted = Vec::new();
        // Bound package analyses separately from worker stub caches. This keeps
        // the routing thread from retaining package-sized indexes for closed or
        // long-idle generations.
        while self.package_order.len() > MAX_PACKAGE_CACHE_ENTRIES {
            if let Some(oldest) = self.package_order.pop_front() {
                self.packages.remove(&oldest);
                self.failed_packages.remove(&oldest);
                evicted.push(oldest);
            }
        }
        evicted
    }

    /// Remember that current package analysis failed so repeated requests fail fast.
    fn store_failed_package(&mut self, key: PackageAnalysisKey) {
        self.failed_packages.retain(|failed| failed.bucket != key.bucket);
        self.failed_packages.insert(key);
    }
}

impl SemanticTokenRequestState {
    /// Queue a semantic-token waiter for an exact document-view key.
    fn queue(&mut self, key: DocumentViewKey, request_id: RequestId) {
        self.pending_by_key.entry(key.clone()).or_default().push(request_id.clone());
        self.pending_owner.insert(request_id, key);
    }

    /// Remove one pending semantic-token request by request ID.
    fn remove_pending_request(&mut self, request_id: &RequestId) -> bool {
        let Some(key) = self.pending_owner.remove(request_id) else {
            return false;
        };
        if let Some(queue) = self.pending_by_key.get_mut(&key) {
            queue.retain(|pending| pending != request_id);
            if queue.is_empty() {
                self.pending_by_key.remove(&key);
            }
        }
        true
    }

    /// Drain every semantic-token waiter for a closed URI.
    fn clear_uri(&mut self, uri: &Uri) -> Vec<RequestId> {
        let keys = self.pending_by_key.keys().filter(|key| &key.uri == uri).cloned().collect::<Vec<_>>();
        keys.into_iter().flat_map(|key| self.take_key(&key)).collect()
    }

    /// Drain every semantic-token waiter for an exact document-view key.
    fn take_key(&mut self, key: &DocumentViewKey) -> Vec<RequestId> {
        let Some(requests) = self.pending_by_key.remove(key) else {
            return Vec::new();
        };
        for request_id in &requests {
            self.pending_owner.remove(request_id);
        }
        requests
    }

    /// Drain every semantic-token waiter blocked on one package key.
    fn take_package(&mut self, package: &PackageAnalysisKey) -> Vec<RequestId> {
        let keys = self.keys_for_package(package);
        keys.into_iter().flat_map(|key| self.take_key(&key)).collect()
    }

    /// Drain every semantic-token waiter blocked on one analysis bucket.
    fn take_bucket(&mut self, bucket: &AnalysisBucket) -> Vec<RequestId> {
        let keys = self.pending_by_key.keys().filter(|key| &key.package.bucket == bucket).cloned().collect::<Vec<_>>();
        keys.into_iter().flat_map(|key| self.take_key(&key)).collect()
    }

    /// Return document-view keys waiting on one package key.
    fn keys_for_package(&self, package: &PackageAnalysisKey) -> Vec<DocumentViewKey> {
        self.pending_by_key.keys().filter(|key| &key.package == package).cloned().collect()
    }
}

impl DefinitionRequestState {
    /// Queue a definition request, enforcing global and per-package caps.
    fn queue(&mut self, query: DefinitionQuery, request_id: RequestId) -> bool {
        // Definition requests are cheap individually but can otherwise pile up
        // behind one slow package analysis. Cap both global and per-package
        // waiters so a client burst cannot turn into unbounded retained queries.
        if self.pending_owner.len() >= MAX_PENDING_DEFINITIONS {
            return false;
        }
        let package = query.view_key.package.clone();
        let queue = self.pending_by_package.entry(package.clone()).or_default();
        if queue.len() >= MAX_PENDING_DEFINITIONS_PER_KEY {
            return false;
        }
        queue.push(PendingDefinitionRequest { id: request_id.clone(), query });
        self.pending_owner.insert(request_id, package);
        true
    }

    /// Remove one pending definition request by request ID.
    fn remove_pending_request(&mut self, request_id: &RequestId) -> Option<PendingDefinitionRequest> {
        let package = self.pending_owner.remove(request_id)?;
        let queue = self.pending_by_package.get_mut(&package)?;
        let index = queue.iter().position(|pending| &pending.id == request_id)?;
        let pending = queue.remove(index);
        if queue.is_empty() {
            self.pending_by_package.remove(&package);
        }
        Some(pending)
    }

    /// Drain definition requests whose source document has closed.
    fn clear_uri(&mut self, uri: &Uri) -> Vec<PendingDefinitionRequest> {
        let packages = self.pending_by_package.keys().cloned().collect::<Vec<_>>();
        let mut cleared = Vec::new();
        for package in packages {
            let Some(queue) = self.pending_by_package.get_mut(&package) else {
                continue;
            };
            let mut index = 0;
            while index < queue.len() {
                if &queue[index].query.uri == uri {
                    let pending = queue.remove(index);
                    self.pending_owner.remove(&pending.id);
                    cleared.push(pending);
                } else {
                    index += 1;
                }
            }
            if queue.is_empty() {
                self.pending_by_package.remove(&package);
            }
        }
        cleared
    }

    /// Drain definition requests waiting on one package key.
    fn take_package(&mut self, package: &PackageAnalysisKey) -> Vec<PendingDefinitionRequest> {
        let Some(requests) = self.pending_by_package.remove(package) else {
            return Vec::new();
        };
        for request in &requests {
            self.pending_owner.remove(&request.id);
        }
        requests
    }

    /// Drain definition requests waiting on any package key in a bucket.
    fn take_bucket(&mut self, bucket: &AnalysisBucket) -> Vec<PendingDefinitionRequest> {
        let packages =
            self.pending_by_package.keys().filter(|package| &package.bucket == bucket).cloned().collect::<Vec<_>>();
        packages.into_iter().flat_map(|package| self.take_package(&package)).collect()
    }
}

impl ReferencesRequestState {
    /// Queue a references request, enforcing global and per-package caps.
    fn queue(
        &mut self,
        query: ReferenceQuery,
        request_id: RequestId,
        cancel: Arc<AtomicBool>,
        dispatched: bool,
    ) -> bool {
        if self.pending_owner.len() >= MAX_PENDING_REFERENCES {
            return false;
        }
        let package = query.view_key.package.clone();
        let queue = self.pending_by_package.entry(package.clone()).or_default();
        if queue.len() >= MAX_PENDING_REFERENCES_PER_KEY {
            return false;
        }
        queue.push(PendingReferencesRequest { id: request_id.clone(), query, cancel, dispatched });
        self.pending_owner.insert(request_id, package);
        true
    }

    /// Return one pending references request without mutating ownership.
    fn get(&self, request_id: &RequestId) -> Option<&PendingReferencesRequest> {
        let package = self.pending_owner.get(request_id)?;
        self.pending_by_package.get(package)?.iter().find(|pending| &pending.id == request_id)
    }

    /// Remove one pending references request by request ID.
    fn remove_pending_request(&mut self, request_id: &RequestId) -> Option<PendingReferencesRequest> {
        let package = self.pending_owner.remove(request_id)?;
        let queue = self.pending_by_package.get_mut(&package)?;
        let index = queue.iter().position(|pending| &pending.id == request_id)?;
        let pending = queue.remove(index);
        if queue.is_empty() {
            self.pending_by_package.remove(&package);
        }
        Some(pending)
    }

    /// Mark undispatched requests for one package as handed to the response pool.
    fn mark_undispatched(&mut self, package: &PackageAnalysisKey) -> Vec<PendingReferencesRequest> {
        let Some(queue) = self.pending_by_package.get_mut(package) else {
            return Vec::new();
        };
        let mut pending = Vec::new();
        for request in queue.iter_mut() {
            if !request.dispatched {
                request.dispatched = true;
                pending.push(request.clone());
            }
        }
        pending
    }

    /// Drain references requests whose source document has closed.
    fn clear_uri(&mut self, uri: &Uri) -> Vec<PendingReferencesRequest> {
        let packages = self.pending_by_package.keys().cloned().collect::<Vec<_>>();
        let mut cleared = Vec::new();
        for package in packages {
            let Some(queue) = self.pending_by_package.get_mut(&package) else {
                continue;
            };
            let mut index = 0;
            while index < queue.len() {
                if &queue[index].query.view_key.uri == uri {
                    let pending = queue.remove(index);
                    self.pending_owner.remove(&pending.id);
                    cleared.push(pending);
                } else {
                    index += 1;
                }
            }
            if queue.is_empty() {
                self.pending_by_package.remove(&package);
            }
        }
        cleared
    }

    /// Drain references requests waiting on one package key.
    fn take_package(&mut self, package: &PackageAnalysisKey) -> Vec<PendingReferencesRequest> {
        let Some(requests) = self.pending_by_package.remove(package) else {
            return Vec::new();
        };
        for request in &requests {
            self.pending_owner.remove(&request.id);
        }
        requests
    }

    /// Drain references requests waiting on any package key in a bucket.
    fn take_bucket(&mut self, bucket: &AnalysisBucket) -> Vec<PendingReferencesRequest> {
        let packages =
            self.pending_by_package.keys().filter(|package| &package.bucket == bucket).cloned().collect::<Vec<_>>();
        packages.into_iter().flat_map(|package| self.take_package(&package)).collect()
    }
}

impl RenameRequestState {
    /// Queue a rename request, enforcing global and per-package caps.
    fn queue(&mut self, query: RenameQuery, request_id: RequestId, cancel: Arc<AtomicBool>, dispatched: bool) -> bool {
        if self.pending_owner.len() >= MAX_PENDING_RENAMES {
            return false;
        }
        let package = query.view_key.package.clone();
        let queue = self.pending_by_package.entry(package.clone()).or_default();
        if queue.len() >= MAX_PENDING_RENAMES_PER_KEY {
            return false;
        }
        queue.push(PendingRenameRequest { id: request_id.clone(), query, cancel, dispatched });
        self.pending_owner.insert(request_id, package);
        true
    }

    /// Return one pending rename request without mutating ownership.
    fn get(&self, request_id: &RequestId) -> Option<&PendingRenameRequest> {
        let package = self.pending_owner.get(request_id)?;
        self.pending_by_package.get(package)?.iter().find(|pending| &pending.id == request_id)
    }

    /// Remove one pending rename request by request ID.
    fn remove_pending_request(&mut self, request_id: &RequestId) -> Option<PendingRenameRequest> {
        let package = self.pending_owner.remove(request_id)?;
        let queue = self.pending_by_package.get_mut(&package)?;
        let index = queue.iter().position(|pending| &pending.id == request_id)?;
        let pending = queue.remove(index);
        if queue.is_empty() {
            self.pending_by_package.remove(&package);
        }
        Some(pending)
    }

    /// Mark undispatched rename requests for one package as handed to the response pool.
    fn mark_undispatched(&mut self, package: &PackageAnalysisKey) -> Vec<PendingRenameRequest> {
        let Some(queue) = self.pending_by_package.get_mut(package) else {
            return Vec::new();
        };
        let mut pending = Vec::new();
        for request in queue.iter_mut() {
            if !request.dispatched {
                request.dispatched = true;
                pending.push(request.clone());
            }
        }
        pending
    }

    /// Drain rename requests whose source document has closed.
    fn clear_uri(&mut self, uri: &Uri) -> Vec<PendingRenameRequest> {
        let packages = self.pending_by_package.keys().cloned().collect::<Vec<_>>();
        let mut cleared = Vec::new();
        for package in packages {
            let Some(queue) = self.pending_by_package.get_mut(&package) else {
                continue;
            };
            let mut index = 0;
            while index < queue.len() {
                if &queue[index].query.view_key.uri == uri {
                    let pending = queue.remove(index);
                    self.pending_owner.remove(&pending.id);
                    cleared.push(pending);
                } else {
                    index += 1;
                }
            }
            if queue.is_empty() {
                self.pending_by_package.remove(&package);
            }
        }
        cleared
    }

    /// Drain rename requests waiting on one package key.
    fn take_package(&mut self, package: &PackageAnalysisKey) -> Vec<PendingRenameRequest> {
        let Some(requests) = self.pending_by_package.remove(package) else {
            return Vec::new();
        };
        for request in &requests {
            self.pending_owner.remove(&request.id);
        }
        requests
    }

    /// Drain rename requests waiting on any package key in a bucket.
    fn take_bucket(&mut self, bucket: &AnalysisBucket) -> Vec<PendingRenameRequest> {
        let packages =
            self.pending_by_package.keys().filter(|package| &package.bucket == bucket).cloned().collect::<Vec<_>>();
        packages.into_iter().flat_map(|package| self.take_package(&package)).collect()
    }
}

impl PrepareRenameRequestState {
    /// Queue a prepare-rename request, enforcing global and per-package caps.
    fn queue(&mut self, query: PrepareRenameQuery, request_id: RequestId) -> bool {
        if self.pending_owner.len() >= MAX_PENDING_PREPARE_RENAMES {
            return false;
        }
        let package = query.view_key.package.clone();
        let queue = self.pending_by_package.entry(package.clone()).or_default();
        if queue.len() >= MAX_PENDING_PREPARE_RENAMES_PER_KEY {
            return false;
        }
        queue.push(PendingPrepareRenameRequest { id: request_id.clone(), query });
        self.pending_owner.insert(request_id, package);
        true
    }

    /// Remove one pending prepare-rename request by request ID.
    fn remove_pending_request(&mut self, request_id: &RequestId) -> Option<PendingPrepareRenameRequest> {
        let package = self.pending_owner.remove(request_id)?;
        let queue = self.pending_by_package.get_mut(&package)?;
        let index = queue.iter().position(|pending| &pending.id == request_id)?;
        let pending = queue.remove(index);
        if queue.is_empty() {
            self.pending_by_package.remove(&package);
        }
        Some(pending)
    }

    /// Drain prepare-rename requests whose source document has closed.
    fn clear_uri(&mut self, uri: &Uri) -> Vec<PendingPrepareRenameRequest> {
        let packages = self.pending_by_package.keys().cloned().collect::<Vec<_>>();
        let mut cleared = Vec::new();
        for package in packages {
            let Some(queue) = self.pending_by_package.get_mut(&package) else {
                continue;
            };
            let mut index = 0;
            while index < queue.len() {
                if &queue[index].query.view_key.uri == uri {
                    let pending = queue.remove(index);
                    self.pending_owner.remove(&pending.id);
                    cleared.push(pending);
                } else {
                    index += 1;
                }
            }
            if queue.is_empty() {
                self.pending_by_package.remove(&package);
            }
        }
        cleared
    }

    /// Drain prepare-rename requests waiting on one package key.
    fn take_package(&mut self, package: &PackageAnalysisKey) -> Vec<PendingPrepareRenameRequest> {
        let Some(requests) = self.pending_by_package.remove(package) else {
            return Vec::new();
        };
        for request in &requests {
            self.pending_owner.remove(&request.id);
        }
        requests
    }

    /// Drain prepare-rename requests waiting on any package key in a bucket.
    fn take_bucket(&mut self, bucket: &AnalysisBucket) -> Vec<PendingPrepareRenameRequest> {
        let packages =
            self.pending_by_package.keys().filter(|package| &package.bucket == bucket).cloned().collect::<Vec<_>>();
        packages.into_iter().flat_map(|package| self.take_package(&package)).collect()
    }
}

impl PendingRequest for PendingDefinitionRequest {
    /// Return the JSON-RPC request ID this definition waiter answers.
    fn id(&self) -> &RequestId {
        &self.id
    }
}

impl PendingRequest for PendingReferencesRequest {
    /// Return the JSON-RPC request ID this references waiter answers.
    fn id(&self) -> &RequestId {
        &self.id
    }

    /// Expose the per-request cancel flag so a routing-thread cancel path can
    /// flip it before the in-flight pool job emits a stale response.
    fn cancel_flag(&self) -> Option<&Arc<AtomicBool>> {
        Some(&self.cancel)
    }
}

impl PendingRequest for PendingRenameRequest {
    /// Return the JSON-RPC request ID this rename waiter answers.
    fn id(&self) -> &RequestId {
        &self.id
    }

    /// Expose the per-request cancel flag so a routing-thread cancel path can
    /// flip it before the in-flight pool job emits a stale `WorkspaceEdit`.
    fn cancel_flag(&self) -> Option<&Arc<AtomicBool>> {
        Some(&self.cancel)
    }
}

impl PendingRequest for PendingPrepareRenameRequest {
    /// Return the JSON-RPC request ID this prepare-rename waiter answers.
    fn id(&self) -> &RequestId {
        &self.id
    }
}

impl PendingFeature for SemanticTokenRequestState {
    /// Pending semantic-token waiters carry no per-request payload beyond the ID.
    type Request = RequestId;

    /// Drain semantic-token waiters whose source document has closed.
    fn drain_uri(&mut self, uri: &Uri) -> Vec<RequestId> {
        self.clear_uri(uri)
    }

    /// Drain semantic-token waiters blocked on one package key.
    fn drain_package(&mut self, key: &PackageAnalysisKey) -> Vec<RequestId> {
        self.take_package(key)
    }

    /// Drain semantic-token waiters blocked on one analysis bucket.
    fn drain_bucket(&mut self, bucket: &AnalysisBucket) -> Vec<RequestId> {
        self.take_bucket(bucket)
    }
}

impl PendingFeature for DefinitionRequestState {
    /// Pending definition waiters retain their original cursor query.
    type Request = PendingDefinitionRequest;

    /// Drain definition waiters whose source document has closed.
    fn drain_uri(&mut self, uri: &Uri) -> Vec<PendingDefinitionRequest> {
        self.clear_uri(uri)
    }

    /// Drain definition waiters blocked on one package key.
    fn drain_package(&mut self, key: &PackageAnalysisKey) -> Vec<PendingDefinitionRequest> {
        self.take_package(key)
    }

    /// Drain definition waiters blocked on one analysis bucket.
    fn drain_bucket(&mut self, bucket: &AnalysisBucket) -> Vec<PendingDefinitionRequest> {
        self.take_bucket(bucket)
    }
}

impl PendingFeature for ReferencesRequestState {
    /// Pending references waiters carry a cancel flag observed by the response pool.
    type Request = PendingReferencesRequest;

    /// Drain references waiters whose source document has closed.
    fn drain_uri(&mut self, uri: &Uri) -> Vec<PendingReferencesRequest> {
        self.clear_uri(uri)
    }

    /// Drain references waiters blocked on one package key.
    fn drain_package(&mut self, key: &PackageAnalysisKey) -> Vec<PendingReferencesRequest> {
        self.take_package(key)
    }

    /// Drain references waiters blocked on one analysis bucket.
    fn drain_bucket(&mut self, bucket: &AnalysisBucket) -> Vec<PendingReferencesRequest> {
        self.take_bucket(bucket)
    }
}

impl PendingFeature for RenameRequestState {
    /// Pending rename waiters carry a cancel flag observed by the response pool.
    type Request = PendingRenameRequest;

    /// Drain rename waiters whose source document has closed.
    fn drain_uri(&mut self, uri: &Uri) -> Vec<PendingRenameRequest> {
        self.clear_uri(uri)
    }

    /// Drain rename waiters blocked on one package key.
    fn drain_package(&mut self, key: &PackageAnalysisKey) -> Vec<PendingRenameRequest> {
        self.take_package(key)
    }

    /// Drain rename waiters blocked on one analysis bucket.
    fn drain_bucket(&mut self, bucket: &AnalysisBucket) -> Vec<PendingRenameRequest> {
        self.take_bucket(bucket)
    }
}

impl PendingFeature for PrepareRenameRequestState {
    /// Pending prepare-rename waiters retain only cursor query state.
    type Request = PendingPrepareRenameRequest;

    /// Drain prepare-rename waiters whose source document has closed.
    fn drain_uri(&mut self, uri: &Uri) -> Vec<PendingPrepareRenameRequest> {
        self.clear_uri(uri)
    }

    /// Drain prepare-rename waiters blocked on one package key.
    fn drain_package(&mut self, key: &PackageAnalysisKey) -> Vec<PendingPrepareRenameRequest> {
        self.take_package(key)
    }

    /// Drain prepare-rename waiters blocked on one analysis bucket.
    fn drain_bucket(&mut self, bucket: &AnalysisBucket) -> Vec<PendingPrepareRenameRequest> {
        self.take_bucket(bucket)
    }
}

/// Convert a prepare-rename target to the LSP wire payload.
fn prepare_rename_response_value(
    query: &PrepareRenameQuery,
    package: &CachedPackageAnalysis,
    source_path: &std::path::Path,
    line_index: &line_index::LineIndex,
) -> Value {
    let Some(range) = prepare_rename_target(query, package, source_path) else {
        return Value::Null;
    };
    let Some(lsp_range) = byte_range_to_lsp_range(line_index, range.start, range.end) else {
        return Value::Null;
    };
    let response: Range = lsp_range;
    serde_json::to_value(PrepareRenameResponse::Range(response)).expect("PrepareRenameResponse should serialize")
}

/// Collect initialize-time workspace roots using LSP's preferred fallback order.
#[allow(deprecated)]
fn collect_workspace_roots(params: &InitializeParams) -> Vec<PathBuf> {
    let mut roots = Vec::new();

    // Follow the LSP preference order: workspace folders first, then the
    // legacy root URI when folders are not available.
    if let Some(workspace_folders) = &params.workspace_folders {
        roots.extend(workspace_folders.iter().filter_map(|folder| uri_to_file_path(&folder.uri)));
    }

    if roots.is_empty()
        && let Some(path) = params.root_uri.as_ref().and_then(uri_to_file_path)
    {
        roots.push(path);
    }

    roots
}

/// Advertise the LSP capabilities implemented by this server.
fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(TextDocumentSyncOptions {
            open_close: Some(true),
            change: Some(TextDocumentSyncKind::FULL),
            will_save: None,
            will_save_wait_until: None,
            save: None,
        })),
        semantic_tokens_provider: Some(semantic_tokens_capability()),
        definition_provider: Some(OneOf::Left(true)),
        references_provider: Some(OneOf::Left(true)),
        rename_provider: Some(OneOf::Right(RenameOptions {
            prepare_provider: Some(true),
            work_done_progress_options: WorkDoneProgressOptions::default(),
        })),
        ..Default::default()
    }
}

/// Return whether the client can consume rich `LocationLink` definition results.
fn client_supports_definition_links(params: &InitializeParams) -> bool {
    params
        .capabilities
        .text_document
        .as_ref()
        .and_then(|text_document| text_document.definition.as_ref())
        .and_then(|definition| definition.link_support)
        .unwrap_or(false)
}

/// Extract the committed text from a full-sync `didChange` payload.
fn extract_full_sync_text(changes: Vec<TextDocumentContentChangeEvent>) -> Option<String> {
    if changes.is_empty() {
        tracing::warn!("didChange arrived without content changes");
        return None;
    }

    if changes.iter().any(|change| change.range.is_some() || change.range_length.is_some()) {
        tracing::warn!("rejecting incremental didChange for full-sync server");
        return None;
    }

    // Full-sync notifications may still batch multiple whole-document payloads;
    // the last one is the committed text per the LSP ordering guarantee.
    changes.into_iter().next_back().map(|change| change.text)
}

/// Convert LSP's string-or-number cancellation ID into `lsp_server::RequestId`.
fn request_id_from_cancel(id: NumberOrString) -> RequestId {
    match id {
        NumberOrString::Number(number) => number.into(),
        NumberOrString::String(string) => string.into(),
    }
}

/// Send one successful JSON-RPC response.
fn send_ok_response(connection: &Connection, id: RequestId, result: Value) -> Result<()> {
    let response = Response { id, result: Some(result), error: None };
    connection.sender.send(Message::Response(response))?;
    Ok(())
}

/// Send one JSON-RPC error response.
fn send_error_response(connection: &Connection, id: RequestId, code: i32, message: impl Into<String>) -> Result<()> {
    let response =
        Response { id, result: None, error: Some(ResponseError { code, message: message.into(), data: None }) };
    connection.sender.send(Message::Response(response))?;
    Ok(())
}

/// Send the same successful payload to every queued waiter on a URI.
fn send_ok_responses(connection: &Connection, request_ids: Vec<RequestId>, result: Value) -> Result<()> {
    for request_id in request_ids {
        send_ok_response(connection, request_id, result.clone())?;
    }

    Ok(())
}

/// Send successful `null` definition responses for requests orphaned by close.
fn send_definition_nulls(connection: &Connection, requests: Vec<PendingDefinitionRequest>) -> Result<()> {
    for request in requests {
        // A close means the source document no longer exists from the client's
        // point of view, so a successful "no definition" result is less noisy
        // than `RequestCanceled`. Superseded open documents still use explicit
        // cancellation so clients can distinguish stale work from no target.
        send_ok_response(connection, request.id, Value::Null)?;
    }
    Ok(())
}

/// Send successful `null` references responses for requests orphaned by close.
fn send_reference_nulls(connection: &Connection, requests: Vec<PendingReferencesRequest>) -> Result<()> {
    for request in requests {
        request.cancel.store(true, Ordering::SeqCst);
        send_ok_response(connection, request.id, Value::Null)?;
    }
    Ok(())
}

/// Log and discard a `cancel_drained` failure without surfacing it to callers.
///
/// The drain helpers fan out across five pending-state owners; one feature's
/// transport failure must not short-circuit the remaining four.
fn log_drain(result: Result<()>, what: &'static str) {
    if let Err(error) = result {
        tracing::error!(error = %error, "failed to cancel {what}");
    }
}

/// Send the same error payload to every queued waiter on a URI.
fn send_error_responses(
    connection: &Connection,
    request_ids: Vec<RequestId>,
    code: i32,
    message: impl Into<String>,
) -> Result<()> {
    let message = message.into();
    for request_id in request_ids {
        send_error_response(connection, request_id, code, message.clone())?;
    }

    Ok(())
}

/// Test-only fault injection toggles shared by in-process and subprocess tests.
///
/// Keeping these hooks in the real server path lets tests exercise the same
/// dispatch code the binary uses, rather than a parallel test-only harness.
#[derive(Debug, Clone, Default)]
struct TestHooks {
    /// Request method that should panic when dispatched.
    panic_on_request_method: Option<String>,
    /// Notification method that should panic when dispatched.
    panic_on_notification_method: Option<String>,
    /// Whether every worker job should panic under test.
    panic_on_worker_job: bool,
}

impl TestHooks {
    /// Build test hooks from environment variables used by subprocess tests.
    fn from_env() -> Self {
        Self {
            panic_on_request_method: std::env::var("LEO_LSP_TEST_PANIC_REQUEST").ok(),
            panic_on_notification_method: std::env::var("LEO_LSP_TEST_PANIC_NOTIFICATION").ok(),
            panic_on_worker_job: std::env::var("LEO_LSP_TEST_PANIC_WORKER")
                .ok()
                .as_deref()
                .is_some_and(|value| value == "1" || value.eq_ignore_ascii_case("true")),
        }
    }

    /// Panic when the configured request method is dispatched.
    fn maybe_panic_request(&self, method: &str) {
        if self.panic_on_request_method.as_deref() == Some(method) {
            panic!("injected request panic for {method}");
        }
    }

    /// Panic when the configured notification method is dispatched.
    fn maybe_panic_notification(&self, method: &str) {
        if self.panic_on_notification_method.as_deref() == Some(method) {
            panic!("injected notification panic for {method}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DID_CHANGE, TestHooks, run_with_hooks};
    use lsp_server::{Connection, ErrorCode, Message, Notification, Request, RequestId, Response};
    use lsp_types::{
        CancelParams,
        DidChangeTextDocumentParams,
        DidOpenTextDocumentParams,
        NumberOrString,
        Position,
        SemanticTokensParams,
        TextDocumentContentChangeEvent,
        TextDocumentItem,
        Uri,
        VersionedTextDocumentIdentifier,
    };
    use serde_json::{Value, json};
    use std::{
        fs,
        path::Path,
        process::ExitCode,
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        },
        thread,
        time::Duration,
    };
    use tempfile::tempdir;

    /// Spawn the real server loop over an in-memory transport.
    fn spawn_server(hooks: TestHooks) -> (Connection, thread::JoinHandle<anyhow::Result<ExitCode>>) {
        // Use an in-memory transport so these tests exercise the real server
        // event loop without paying the cost of subprocess management.
        let (server, client) = Connection::memory();
        let handle = thread::spawn(move || run_with_hooks(server, hooks));
        (client, handle)
    }

    /// Build a test `file:` URI from a native path.
    fn file_uri(path: &Path) -> Uri {
        #[cfg(target_os = "windows")]
        let path = format!("/{}", path.display()).replace('\\', "/");

        #[cfg(not(target_os = "windows"))]
        let path = path.display().to_string();

        format!("file://{path}").parse().expect("file uri")
    }

    /// Send a JSON-RPC request frame to the in-memory server.
    fn send_request(client: &Connection, id: i32, method: &str, params: Value) {
        client
            .sender
            .send(Message::Request(Request { id: id.into(), method: method.to_owned(), params }))
            .expect("send request");
    }

    /// Send a JSON-RPC notification frame to the in-memory server.
    fn send_notification(client: &Connection, method: &str, params: Value) {
        client
            .sender
            .send(Message::Notification(Notification { method: method.to_owned(), params }))
            .expect("send notification");
    }

    /// Receive the next server response in tests with a short timeout.
    fn recv_response(client: &Connection) -> Response {
        // Requests in this module are strictly request/response, so the next
        // frame observed from the server must be the matching response.
        match client.receiver.recv_timeout(Duration::from_secs(1)).expect("response from server") {
            Message::Response(response) => response,
            other => panic!("expected response, got {other:?}"),
        }
    }

    /// Build semantic-token request params for a document URI.
    fn semantic_tokens_params(uri: Uri) -> SemanticTokensParams {
        SemanticTokensParams {
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            text_document: lsp_types::TextDocumentIdentifier { uri },
        }
    }

    /// Build a references query for direct pending-state tests.
    fn reference_query(key: crate::document_store::DocumentViewKey) -> super::ReferenceQuery {
        super::ReferenceQuery { offset: 0, position: Position::new(0, 0), view_key: key, include_declaration: true }
    }

    /// Build isolated server state for direct state-machine tests.
    fn test_state() -> super::ServerState {
        super::ServerState {
            shutdown_requested: false,
            exit_code: None,
            workspace_roots: Vec::new(),
            documents: crate::document_store::DocumentStore::default(),
            project_model: crate::project_model::ProjectModel::default(),
            scheduler: crate::scheduler::Scheduler::new(false),
            response_pool: super::ResponsePool::new(),
            analysis: super::AnalysisCaches::default(),
            semantic_token_requests: super::SemanticTokenRequestState::default(),
            definition_requests: super::DefinitionRequestState::default(),
            reference_requests: super::ReferencesRequestState::default(),
            rename_requests: super::RenameRequestState::default(),
            prepare_rename_requests: super::PrepareRenameRequestState::default(),
            client_definition_link_support: false,
            hooks: TestHooks::default(),
        }
    }

    /// Insert an unmanaged open document into direct state-machine tests.
    fn open_unmanaged_document(state: &mut super::ServerState, uri: &Uri, version: i32, text: &str) {
        state.documents.commit_open(state.documents.prepare_open(
            uri.clone(),
            "leo".to_owned(),
            version,
            text.to_owned(),
            None,
            None,
        ));
        state.analysis.invalidate_uri(uri);
    }

    /// Verifies bucket invalidation clears every package and view cache surface.
    #[test]
    fn bucket_invalidation_evicts_package_views_and_state() {
        let mut state = test_state();
        let uri: Uri = "untitled:main.leo".parse().expect("uri");
        open_unmanaged_document(&mut state, &uri, 1, "program test.aleo {}\n");
        let key = state.documents.document_view_key(&uri).expect("document view key");

        state.analysis.document_views.insert(uri.clone(), crate::semantics::CachedDocumentView {
            key: key.clone(),
            encoded_tokens: Arc::from([]),
        });
        state.analysis.failed_packages.insert(key.package.clone());
        state.analysis.in_flight_packages.insert(key.package.clone());
        state.analysis.failed_views.insert(key.clone());
        state.analysis.in_flight_views.insert(key.clone());

        state.analysis.invalidate_bucket(&key.package.bucket);

        assert!(state.analysis.document_views.is_empty());
        assert!(state.analysis.failed_packages.is_empty());
        assert!(state.analysis.in_flight_packages.is_empty());
        assert!(state.analysis.failed_views.is_empty());
        assert!(state.analysis.in_flight_views.is_empty());
        state.scheduler.shutdown();
    }

    /// Complete the initialize/initialized handshake for server-loop tests.
    fn initialize(client: &Connection) {
        send_request(
            client,
            1,
            "initialize",
            json!({
                "processId": null,
                "rootUri": null,
                "capabilities": {},
            }),
        );

        let response = recv_response(client);
        assert_eq!(response.id, 1.into());
        send_notification(client, "initialized", json!({}));
    }

    /// Verifies exiting before shutdown reports an unsuccessful server exit.
    #[test]
    fn exit_without_shutdown_returns_failure() {
        let (client, handle) = spawn_server(TestHooks::default());
        initialize(&client);

        send_notification(&client, "exit", json!({}));

        let exit_code = handle.join().expect("server thread should not panic").expect("server result");
        assert_eq!(exit_code, ExitCode::from(1));
    }

    /// Verifies notification panics are contained in the real server loop.
    #[test]
    fn notification_panic_can_be_tested_without_shelling_out() {
        let hooks = TestHooks { panic_on_notification_method: Some(DID_CHANGE.to_owned()), ..Default::default() };
        let (client, handle) = spawn_server(hooks);
        initialize(&client);

        send_notification(
            &client,
            "textDocument/didOpen",
            json!({
                "textDocument": {
                    "uri": "untitled:main.leo",
                    "languageId": "leo",
                    "version": 1,
                    "text": "program test.aleo {}",
                }
            }),
        );

        send_notification(
            &client,
            "textDocument/didChange",
            json!({
                "textDocument": {
                    "uri": "untitled:main.leo",
                    "version": 2,
                },
                "contentChanges": [
                    {
                        "text": "program test.aleo { fn main() {} }",
                    }
                ]
            }),
        );

        send_request(&client, 2, "shutdown", Value::Null);
        let shutdown = recv_response(&client);
        assert_eq!(shutdown.id, 2.into());
        assert_eq!(shutdown.result, Some(Value::Null));

        send_notification(&client, "exit", json!({}));

        let exit_code = handle.join().expect("server thread should not panic").expect("server result");
        assert_eq!(exit_code, ExitCode::SUCCESS);
    }

    /// Verifies cancelling a queued semantic-token request returns request-cancelled.
    #[test]
    fn cancelled_semantic_request_returns_request_cancelled() {
        let (server, client) = Connection::memory();
        let mut state = test_state();
        let uri: Uri = "untitled:main.leo".parse().expect("uri");

        open_unmanaged_document(&mut state, &uri, 1, "program test.aleo {}\n");

        state
            .handle_semantic_tokens_full(&server, 2.into(), semantic_tokens_params(uri.clone()))
            .expect("queue semantic request");
        state.handle_cancel_request(&server, CancelParams { id: NumberOrString::Number(2) }).expect("cancel request");

        let response = recv_response(&client);
        assert_eq!(response.id, 2.into());
        let error = response.error.expect("cancelled response error");
        assert_eq!(error.code, ErrorCode::RequestCanceled as i32);
        assert!(error.message.contains("cancelled"));
        assert!(state.semantic_token_requests.pending_by_key.is_empty());
        state.scheduler.shutdown();
    }

    /// Verifies package cancellation drains pending references and flips pool cancel flags.
    #[test]
    fn package_cancellation_cancels_pending_references() {
        let (server, client) = Connection::memory();
        let mut state = test_state();
        let uri: Uri = "untitled:main.leo".parse().expect("uri");

        open_unmanaged_document(&mut state, &uri, 1, "program test.aleo {}\n");
        let key = state.documents.document_view_key(&uri).expect("document view key");
        let cancel = Arc::new(AtomicBool::new(false));
        assert!(state.reference_requests.queue(reference_query(key.clone()), 2.into(), Arc::clone(&cancel), true));

        state.cancel_pending_package_requests(&server, &key.package, "package analysis was evicted from cache");

        let response = recv_response(&client);
        assert_eq!(response.id, 2.into());
        let error = response.error.expect("cancelled references response");
        assert_eq!(error.code, ErrorCode::RequestCanceled as i32);
        assert!(error.message.contains("evicted"));
        assert!(cancel.load(Ordering::SeqCst));
        assert!(state.reference_requests.pending_by_package.is_empty());
        assert!(state.reference_requests.pending_owner.is_empty());
        state.scheduler.shutdown();
    }

    /// Verifies a late response-pool completion cannot steal a reused request ID.
    #[test]
    fn stale_reference_completion_with_reused_id_is_dropped() {
        let (server, client) = Connection::memory();
        let mut state = test_state();
        let uri: Uri = "untitled:main.leo".parse().expect("uri");

        open_unmanaged_document(&mut state, &uri, 1, "program test.aleo {}\n");
        let key = state.documents.document_view_key(&uri).expect("document view key");
        let request_id: RequestId = 2.into();
        let old_cancel = Arc::new(AtomicBool::new(true));
        let new_cancel = Arc::new(AtomicBool::new(false));
        assert!(state.reference_requests.queue(
            reference_query(key.clone()),
            request_id.clone(),
            Arc::clone(&new_cancel),
            true,
        ));

        state.handle_response_completion(&server, super::ResponseCompletion::References {
            id: request_id.clone(),
            key: key.package,
            cancel: old_cancel,
            result: super::ResponseResult::Ok(Value::Null),
        });

        assert!(client.receiver.recv_timeout(Duration::from_millis(50)).is_err());
        assert!(state.reference_requests.get(&request_id).is_some());
        assert!(!new_cancel.load(Ordering::SeqCst));
        state.scheduler.shutdown();
    }

    /// Verifies worker cancellation fails semantic-token waiters for that package.
    #[test]
    fn cancelled_package_analysis_fails_pending_waiters() {
        let (server, client) = Connection::memory();
        let mut state = test_state();
        let uri: Uri = "untitled:main.leo".parse().expect("uri");

        open_unmanaged_document(&mut state, &uri, 1, "program test.aleo {}\n");
        let key = state.documents.document_view_key(&uri).expect("document view key");
        state.semantic_token_requests.queue(key.clone(), 2.into());

        state.handle_worker_event(&server, crate::scheduler::WorkerEvent::PackageCancelled {
            key: key.package,
            uri: uri.clone(),
            generation: 1,
        });

        let response = recv_response(&client);
        assert_eq!(response.id, 2.into());
        let error = response.error.expect("cancelled response error");
        assert_eq!(error.code, ErrorCode::RequestCanceled as i32);
        assert!(error.message.contains("cancelled"));
        assert!(state.semantic_token_requests.pending_by_key.is_empty());
        state.scheduler.shutdown();
    }

    /// Verifies a current worker panic fails pending and repeated requests.
    #[test]
    fn current_generation_worker_panic_fails_pending_and_future_requests() {
        let (server, client) = Connection::memory();
        let mut state = test_state();
        let uri: Uri = "untitled:main.leo".parse().expect("uri");

        open_unmanaged_document(&mut state, &uri, 1, "program test.aleo {}\n");

        state
            .handle_semantic_tokens_full(&server, 2.into(), semantic_tokens_params(uri.clone()))
            .expect("queue semantic request");

        let panic_report = crate::panic_boundary::catch_unwind("worker_analyze", Some(&uri), Some(1), || {
            panic!("boom");
        })
        .expect_err("panic report");
        let key = state.documents.package_key(&uri).expect("package key");
        state.handle_worker_event(&server, crate::scheduler::WorkerEvent::PackagePanicked {
            key,
            uri: uri.clone(),
            generation: 1,
            report: panic_report,
        });

        let first = recv_response(&client);
        assert_eq!(first.id, 2.into());
        assert_eq!(first.error.expect("panic error").code, super::INTERNAL_ERROR);

        state
            .handle_semantic_tokens_full(&server, 3.into(), semantic_tokens_params(uri.clone()))
            .expect("fail repeated request immediately");

        let second = recv_response(&client);
        assert_eq!(second.id, 3.into());
        assert_eq!(second.error.expect("repeat panic error").code, super::INTERNAL_ERROR);
        assert!(state.semantic_token_requests.pending_by_key.is_empty());
        state.scheduler.shutdown();
    }

    /// Verifies stale worker panics do not poison newer pending requests.
    #[test]
    fn stale_worker_panic_does_not_fail_newer_pending_request() {
        let (server, client) = Connection::memory();
        let mut state = test_state();
        let uri: Uri = "untitled:main.leo".parse().expect("uri");

        open_unmanaged_document(&mut state, &uri, 1, "program test.aleo {}\n");
        let stale_key = state.documents.package_key(&uri).expect("initial package key");

        let changed = state
            .documents
            .prepare_full_change(&uri, 2, "program test.aleo { fn main() {} }\n".to_owned(), None, None)
            .expect("prepared change");
        state.documents.commit_change(changed);
        state.analysis.invalidate_uri(&uri);

        state
            .handle_semantic_tokens_full(&server, 2.into(), semantic_tokens_params(uri.clone()))
            .expect("queue semantic request");

        let panic_report = crate::panic_boundary::catch_unwind("worker_analyze", Some(&uri), Some(1), || {
            panic!("stale boom");
        })
        .expect_err("panic report");
        state.handle_worker_event(&server, crate::scheduler::WorkerEvent::PackagePanicked {
            key: stale_key,
            uri: uri.clone(),
            generation: 1,
            report: panic_report,
        });

        assert!(client.receiver.recv_timeout(Duration::from_millis(50)).is_err(), "stale panic should not respond");
        state.scheduler.shutdown();
    }

    /// Verifies didChange re-runs project discovery before queueing analysis.
    #[test]
    fn did_change_re_resolves_project_context() {
        let tempdir = tempdir().expect("tempdir");
        let package_root = tempdir.path().join("package");
        let source_dir = package_root.join("src");
        fs::create_dir_all(&source_dir).expect("create source dir");
        let main_path = source_dir.join("main.leo");
        fs::write(&main_path, "program demo.aleo {}\n").expect("write source");
        let uri = file_uri(&main_path);

        let mut state = super::ServerState {
            shutdown_requested: false,
            exit_code: None,
            workspace_roots: Vec::new(),
            documents: crate::document_store::DocumentStore::default(),
            project_model: crate::project_model::ProjectModel::default(),
            scheduler: crate::scheduler::Scheduler::new(false),
            response_pool: super::ResponsePool::new(),
            analysis: super::AnalysisCaches::default(),
            semantic_token_requests: super::SemanticTokenRequestState::default(),
            definition_requests: super::DefinitionRequestState::default(),
            reference_requests: super::ReferencesRequestState::default(),
            rename_requests: super::RenameRequestState::default(),
            prepare_rename_requests: super::PrepareRenameRequestState::default(),
            client_definition_link_support: false,
            hooks: TestHooks::default(),
        };
        let (server, _client) = Connection::memory();

        state.handle_did_open(&server, DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: uri.clone(),
                language_id: "leo".to_owned(),
                version: 1,
                text: "program demo.aleo {}\n".to_owned(),
            },
        });
        assert!(state.documents.get(&uri).expect("open document").project.is_none());

        fs::write(
            package_root.join("program.json"),
            r#"{
  "program": "demo.aleo",
  "version": "0.1.0",
  "description": "",
  "license": "MIT",
  "leo": "4.0.0"
}
"#,
        )
        .expect("write manifest");

        state.handle_did_change(&server, DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier { uri: uri.clone(), version: 2 },
            content_changes: vec![TextDocumentContentChangeEvent {
                range: None,
                range_length: None,
                text: "program demo.aleo {}\n".to_owned(),
            }],
        });

        assert!(state.documents.get(&uri).expect("open document").project.is_some());
        state.scheduler.shutdown();
    }
}
