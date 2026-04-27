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
    document_store::DocumentStore,
    features::semantic_tokens::{capability as semantic_tokens_capability, empty_response_value, response_value},
    panic_boundary::catch_unwind,
    project_model::{ProjectModel, uri_to_file_path},
    scheduler::{DocumentAnalysis, Scheduler, WorkerEvent},
    semantics::SemanticSnapshot,
};
use anyhow::{Context, Result};
use lsp_server::{Connection, ErrorCode, Message, Notification, Request, RequestId, Response, ResponseError};
use lsp_types::{
    CancelParams,
    DidChangeTextDocumentParams,
    DidCloseTextDocumentParams,
    DidOpenTextDocumentParams,
    InitializeParams,
    InitializeResult,
    NumberOrString,
    SemanticTokensParams,
    ServerCapabilities,
    ServerInfo,
    TextDocumentContentChangeEvent,
    TextDocumentSyncCapability,
    TextDocumentSyncKind,
    TextDocumentSyncOptions,
    Uri,
};
use serde_json::Value;
use std::{collections::HashMap, path::PathBuf, process::ExitCode};

const INTERNAL_ERROR: i32 = -32603;
const METHOD_NOT_FOUND: i32 = -32601;

const INITIALIZED: &str = "initialized";
const EXIT: &str = "exit";
const SHUTDOWN: &str = "shutdown";
const DID_OPEN: &str = "textDocument/didOpen";
const DID_CHANGE: &str = "textDocument/didChange";
const DID_CLOSE: &str = "textDocument/didClose";
const CANCEL_REQUEST: &str = "$/cancelRequest";
const SEMANTIC_TOKENS_FULL: &str = "textDocument/semanticTokens/full";

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
    /// Async request state for generation-scoped semantic-token results.
    ///
    /// Additional async document features can reuse the same bookkeeping
    /// pattern instead of growing bespoke pending/cached/failure maps.
    semantic_tokens: DocumentRequestState<SemanticSnapshot>,
    hooks: TestHooks,
}

/// Shared state for one async LSP capability that resolves per-document generations.
#[derive(Debug)]
struct DocumentRequestState<T> {
    cached: HashMap<Uri, CachedDocumentResult<T>>,
    failed: HashMap<Uri, FailedDocumentResult>,
    pending_by_uri: HashMap<Uri, Vec<RequestId>>,
    pending_owner: HashMap<RequestId, Uri>,
}

/// Successful async result paired with the document generation it belongs to.
#[derive(Debug, Clone)]
struct CachedDocumentResult<T> {
    generation: u64,
    value: T,
}

/// Failed async result paired with the document generation it belongs to.
#[derive(Debug, Clone, Copy)]
struct FailedDocumentResult {
    generation: u64,
}

/// Immediate state of an async document request for the current generation.
enum DocumentRequestStatus<'a, T> {
    Cached(&'a T),
    Failed,
    Pending,
}

impl<T> Default for DocumentRequestState<T> {
    fn default() -> Self {
        Self {
            cached: HashMap::new(),
            failed: HashMap::new(),
            pending_by_uri: HashMap::new(),
            pending_owner: HashMap::new(),
        }
    }
}

pub(crate) fn run(connection: Connection) -> Result<ExitCode> {
    run_with_hooks(connection, TestHooks::from_env())
}

fn run_with_hooks(connection: Connection, hooks: TestHooks) -> Result<ExitCode> {
    let (request_id, params) = connection.initialize_start()?;
    let initialize_params: InitializeParams =
        serde_json::from_value(params).context("failed to deserialize initialize params")?;
    let workspace_roots = collect_workspace_roots(&initialize_params);

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
        semantic_tokens: DocumentRequestState::default(),
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
        }
    }

    state.scheduler.shutdown();
    Ok(state.exit_code.unwrap_or(ExitCode::from(1)))
}

impl ServerState {
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
            _ => {
                tracing::debug!(method, "request is not implemented");
                send_error_response(connection, request_id, METHOD_NOT_FOUND, "method not found")
            }
        }
    }

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
                self.handle_did_open(params);
                Ok(false)
            }
            DID_CHANGE => {
                let params: DidChangeTextDocumentParams =
                    serde_json::from_value(notification.params).context("failed to deserialize didChange")?;
                self.handle_did_change(params);
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

    fn handle_did_open(&mut self, params: DidOpenTextDocumentParams) {
        let document = params.text_document;
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
        self.semantic_tokens.invalidate(&snapshot.uri);
        self.scheduler.enqueue(snapshot);
    }

    fn handle_did_change(&mut self, params: DidChangeTextDocumentParams) {
        let DidChangeTextDocumentParams { text_document, content_changes } = params;

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
        self.semantic_tokens.invalidate(&snapshot.uri);
        self.scheduler.enqueue(snapshot);
    }

    fn handle_did_close(&mut self, connection: &Connection, params: DidCloseTextDocumentParams) {
        self.hooks.maybe_panic_notification(DID_CLOSE);
        let uri = params.text_document.uri;
        self.documents.close(&uri);
        if let Err(error) = send_ok_responses(connection, self.semantic_tokens.clear(&uri), empty_response_value()) {
            tracing::error!(uri = uri.as_str(), error = %error, "failed to flush semantic token close responses");
        }
    }

    fn handle_cancel_request(&mut self, connection: &Connection, params: CancelParams) -> Result<()> {
        let request_id = request_id_from_cancel(params.id);
        if self.semantic_tokens.remove_pending_request(&request_id) {
            send_error_response(
                connection,
                request_id,
                ErrorCode::RequestCanceled as i32,
                "semantic token request cancelled",
            )
        } else {
            Ok(())
        }
    }

    fn handle_worker_event(&mut self, connection: &Connection, event: WorkerEvent) {
        match event {
            WorkerEvent::Analyzed(DocumentAnalysis { uri, generation, semantic_snapshot }) => {
                // Worker jobs finish asynchronously, so only the latest
                // committed generation is allowed to surface as current.
                if self.documents.generation(&uri) == Some(generation) {
                    let pending = self.semantic_tokens.store_success(uri.clone(), generation, semantic_snapshot);
                    if let Some(snapshot) = self.semantic_tokens.cached(&uri, generation)
                        && let Err(error) =
                            send_ok_responses(connection, pending, response_value(snapshot.encoded_tokens.as_ref()))
                    {
                        tracing::error!(uri = uri.as_str(), error = %error, "failed to send semantic token response");
                    }
                    tracing::debug!(
                        uri = uri.as_str(),
                        generation,
                        workspace_roots = self.workspace_roots.len(),
                        "worker completed latest document"
                    );
                } else {
                    tracing::debug!(uri = uri.as_str(), generation, "dropping stale worker completion");
                }
            }
            WorkerEvent::Cancelled { uri, generation } => {
                tracing::debug!(uri = uri.as_str(), generation, "worker cancelled stale document");
            }
            WorkerEvent::Panicked { uri, generation, report } => {
                report.log();
                if self.documents.generation(&uri) == Some(generation) {
                    let pending = self.semantic_tokens.store_failure(uri.clone(), generation);
                    if let Err(error) = send_error_responses(
                        connection,
                        pending,
                        INTERNAL_ERROR,
                        "semantic token analysis panicked; see server logs for details",
                    ) {
                        tracing::error!(uri = uri.as_str(), error = %error, "failed to send semantic analysis panic");
                    }
                }
            }
        }
    }

    fn handle_semantic_tokens_full(
        &mut self,
        connection: &Connection,
        request_id: RequestId,
        params: SemanticTokensParams,
    ) -> Result<()> {
        let uri = params.text_document.uri;
        let Some(generation) = self.documents.generation(&uri) else {
            return send_ok_response(connection, request_id, empty_response_value());
        };

        match self.semantic_tokens.status_or_queue(uri, generation, request_id.clone()) {
            DocumentRequestStatus::Cached(snapshot) => {
                send_ok_response(connection, request_id, response_value(snapshot.encoded_tokens.as_ref()))
            }
            DocumentRequestStatus::Failed => send_error_response(
                connection,
                request_id,
                INTERNAL_ERROR,
                "semantic token analysis panicked; see server logs for details",
            ),
            DocumentRequestStatus::Pending => Ok(()),
        }
    }
}

impl<T> DocumentRequestState<T> {
    /// Drop cached success/failure state for one URI after the document changes.
    fn invalidate(&mut self, uri: &Uri) {
        self.cached.remove(uri);
        self.failed.remove(uri);
    }

    /// Drop all state for one URI and return any waiters that still need a response.
    fn clear(&mut self, uri: &Uri) -> Vec<RequestId> {
        self.invalidate(uri);
        self.take_pending(uri)
    }

    /// Return the current-generation state for a request, or queue it as a waiter.
    fn status_or_queue(&mut self, uri: Uri, generation: u64, request_id: RequestId) -> DocumentRequestStatus<'_, T> {
        if let Some(cached) = self.cached.get(&uri)
            && cached.generation == generation
        {
            return DocumentRequestStatus::Cached(&cached.value);
        }

        if let Some(failed) = self.failed.get(&uri)
            && failed.generation == generation
        {
            return DocumentRequestStatus::Failed;
        }

        // Hold the request open until analysis for this generation finishes,
        // then fan the single result out to every waiter on the same URI.
        self.pending_by_uri.entry(uri.clone()).or_default().push(request_id.clone());
        self.pending_owner.insert(request_id, uri);
        DocumentRequestStatus::Pending
    }

    /// Cache a successful result and return every pending waiter for that URI.
    fn store_success(&mut self, uri: Uri, generation: u64, value: T) -> Vec<RequestId> {
        self.failed.remove(&uri);
        self.cached.insert(uri.clone(), CachedDocumentResult { generation, value });
        self.take_pending(&uri)
    }

    /// Cache a generation-scoped failure and return every pending waiter for that URI.
    fn store_failure(&mut self, uri: Uri, generation: u64) -> Vec<RequestId> {
        self.cached.remove(&uri);
        self.failed.insert(uri.clone(), FailedDocumentResult { generation });
        self.take_pending(&uri)
    }

    /// Return the cached success payload for one URI/generation pair.
    fn cached(&self, uri: &Uri, generation: u64) -> Option<&T> {
        self.cached.get(uri).filter(|entry| entry.generation == generation).map(|entry| &entry.value)
    }

    /// Remove one queued waiter by request ID.
    fn remove_pending_request(&mut self, request_id: &RequestId) -> bool {
        let Some(owner) = self.pending_owner.remove(request_id) else {
            return false;
        };

        if let Some(queue) = self.pending_by_uri.get_mut(&owner) {
            queue.retain(|pending| pending != request_id);
            if queue.is_empty() {
                self.pending_by_uri.remove(&owner);
            }
        }

        true
    }

    /// Drain every queued waiter for one URI.
    fn take_pending(&mut self, uri: &Uri) -> Vec<RequestId> {
        let Some(requests) = self.pending_by_uri.remove(uri) else {
            return Vec::new();
        };

        for request_id in &requests {
            self.pending_owner.remove(request_id);
        }

        requests
    }
}

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
        ..Default::default()
    }
}

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

fn request_id_from_cancel(id: NumberOrString) -> RequestId {
    match id {
        NumberOrString::Number(number) => number.into(),
        NumberOrString::String(string) => string.into(),
    }
}

fn send_ok_response(connection: &Connection, id: RequestId, result: Value) -> Result<()> {
    let response = Response { id, result: Some(result), error: None };
    connection.sender.send(Message::Response(response))?;
    Ok(())
}

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
    panic_on_request_method: Option<String>,
    panic_on_notification_method: Option<String>,
    panic_on_worker_job: bool,
}

impl TestHooks {
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

    fn maybe_panic_request(&self, method: &str) {
        if self.panic_on_request_method.as_deref() == Some(method) {
            panic!("injected request panic for {method}");
        }
    }

    fn maybe_panic_notification(&self, method: &str) {
        if self.panic_on_notification_method.as_deref() == Some(method) {
            panic!("injected notification panic for {method}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DID_CHANGE, TestHooks, run_with_hooks};
    use lsp_server::{Connection, ErrorCode, Message, Notification, Request, Response};
    use lsp_types::{
        CancelParams,
        DidChangeTextDocumentParams,
        DidOpenTextDocumentParams,
        NumberOrString,
        SemanticTokensParams,
        TextDocumentContentChangeEvent,
        TextDocumentItem,
        Uri,
        VersionedTextDocumentIdentifier,
    };
    use serde_json::{Value, json};
    use std::{fs, path::Path, process::ExitCode, sync::Arc, thread, time::Duration};
    use tempfile::tempdir;

    fn spawn_server(hooks: TestHooks) -> (Connection, thread::JoinHandle<anyhow::Result<ExitCode>>) {
        // Use an in-memory transport so these tests exercise the real server
        // event loop without paying the cost of subprocess management.
        let (server, client) = Connection::memory();
        let handle = thread::spawn(move || run_with_hooks(server, hooks));
        (client, handle)
    }

    fn file_uri(path: &Path) -> Uri {
        #[cfg(target_os = "windows")]
        let path = format!("/{}", path.display()).replace('\\', "/");

        #[cfg(not(target_os = "windows"))]
        let path = path.display().to_string();

        format!("file://{path}").parse().expect("file uri")
    }

    fn send_request(client: &Connection, id: i32, method: &str, params: Value) {
        client
            .sender
            .send(Message::Request(Request { id: id.into(), method: method.to_owned(), params }))
            .expect("send request");
    }

    fn send_notification(client: &Connection, method: &str, params: Value) {
        client
            .sender
            .send(Message::Notification(Notification { method: method.to_owned(), params }))
            .expect("send notification");
    }

    fn recv_response(client: &Connection) -> Response {
        // Requests in this module are strictly request/response, so the next
        // frame observed from the server must be the matching response.
        match client.receiver.recv_timeout(Duration::from_secs(1)).expect("response from server") {
            Message::Response(response) => response,
            other => panic!("expected response, got {other:?}"),
        }
    }

    fn empty_semantic_snapshot() -> crate::semantics::SemanticSnapshot {
        crate::semantics::SemanticSnapshot {
            encoded_tokens: Arc::<[u32]>::from([]),
            index: Arc::new(crate::semantics::SemanticIndex::default()),
            source: crate::semantics::SemanticSource::SyntaxOnly,
        }
    }

    fn semantic_tokens_params(uri: Uri) -> SemanticTokensParams {
        SemanticTokensParams {
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            text_document: lsp_types::TextDocumentIdentifier { uri },
        }
    }

    fn test_state() -> super::ServerState {
        super::ServerState {
            shutdown_requested: false,
            exit_code: None,
            workspace_roots: Vec::new(),
            documents: crate::document_store::DocumentStore::default(),
            project_model: crate::project_model::ProjectModel::default(),
            scheduler: crate::scheduler::Scheduler::new(false),
            semantic_tokens: super::DocumentRequestState::default(),
            hooks: TestHooks::default(),
        }
    }

    fn open_unmanaged_document(state: &mut super::ServerState, uri: &Uri, version: i32, text: &str) {
        state.documents.commit_open(state.documents.prepare_open(
            uri.clone(),
            "leo".to_owned(),
            version,
            text.to_owned(),
            None,
            None,
        ));
        state.semantic_tokens.invalidate(uri);
    }

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

    #[test]
    fn exit_without_shutdown_returns_failure() {
        let (client, handle) = spawn_server(TestHooks::default());
        initialize(&client);

        send_notification(&client, "exit", json!({}));

        let exit_code = handle.join().expect("server thread should not panic").expect("server result");
        assert_eq!(exit_code, ExitCode::from(1));
    }

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
        assert!(state.semantic_tokens.pending_by_uri.is_empty());
        state.scheduler.shutdown();
    }

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
        state.handle_worker_event(&server, crate::scheduler::WorkerEvent::Panicked {
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
        assert!(state.semantic_tokens.pending_by_uri.is_empty());
        state.scheduler.shutdown();
    }

    #[test]
    fn stale_worker_panic_does_not_fail_newer_pending_request() {
        let (server, client) = Connection::memory();
        let mut state = test_state();
        let uri: Uri = "untitled:main.leo".parse().expect("uri");

        open_unmanaged_document(&mut state, &uri, 1, "program test.aleo {}\n");

        let changed = state
            .documents
            .prepare_full_change(&uri, 2, "program test.aleo { fn main() {} }\n".to_owned(), None, None)
            .expect("prepared change");
        state.documents.commit_change(changed);
        state.semantic_tokens.invalidate(&uri);

        state
            .handle_semantic_tokens_full(&server, 2.into(), semantic_tokens_params(uri.clone()))
            .expect("queue semantic request");

        let panic_report = crate::panic_boundary::catch_unwind("worker_analyze", Some(&uri), Some(1), || {
            panic!("stale boom");
        })
        .expect_err("panic report");
        state.handle_worker_event(&server, crate::scheduler::WorkerEvent::Panicked {
            uri: uri.clone(),
            generation: 1,
            report: panic_report,
        });

        assert!(client.receiver.recv_timeout(Duration::from_millis(50)).is_err(), "stale panic should not respond");

        state.handle_worker_event(
            &server,
            crate::scheduler::WorkerEvent::Analyzed(crate::scheduler::DocumentAnalysis {
                uri: uri.clone(),
                generation: 2,
                semantic_snapshot: empty_semantic_snapshot(),
            }),
        );

        let response = recv_response(&client);
        assert_eq!(response.id, 2.into());
        assert_eq!(response.result, Some(crate::features::semantic_tokens::empty_response_value()));
        state.scheduler.shutdown();
    }

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
            semantic_tokens: super::DocumentRequestState::default(),
            hooks: TestHooks::default(),
        };

        state.handle_did_open(DidOpenTextDocumentParams {
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

        state.handle_did_change(DidChangeTextDocumentParams {
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
