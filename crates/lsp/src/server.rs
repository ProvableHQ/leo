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
    panic_boundary::catch_unwind,
    project_model::{ProjectModel, uri_to_file_path},
    scheduler::{Scheduler, WorkerEvent},
};
use anyhow::{Context, Result};
use lsp_server::{Connection, Message, Notification, Request, RequestId, Response, ResponseError};
use lsp_types::{
    DidChangeTextDocumentParams,
    DidCloseTextDocumentParams,
    DidOpenTextDocumentParams,
    InitializeParams,
    InitializeResult,
    ServerCapabilities,
    ServerInfo,
    TextDocumentContentChangeEvent,
    TextDocumentSyncCapability,
    TextDocumentSyncKind,
    TextDocumentSyncOptions,
};
use serde_json::Value;
use std::{path::PathBuf, process::ExitCode};

const INTERNAL_ERROR: i32 = -32603;
const METHOD_NOT_FOUND: i32 = -32601;

const INITIALIZED: &str = "initialized";
const EXIT: &str = "exit";
const SHUTDOWN: &str = "shutdown";
const DID_OPEN: &str = "textDocument/didOpen";
const DID_CHANGE: &str = "textDocument/didChange";
const DID_CLOSE: &str = "textDocument/didClose";

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
    hooks: TestHooks,
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
                    state.handle_worker_event(event);
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
            Message::Notification(notification) => self.handle_notification(notification),
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
        _params: Value,
    ) -> Result<()> {
        self.hooks.maybe_panic_request(method);

        match method {
            SHUTDOWN => {
                self.shutdown_requested = true;
                send_ok_response(connection, request_id, Value::Null)
            }
            _ => {
                tracing::debug!(method, "request is not implemented");
                send_error_response(connection, request_id, METHOD_NOT_FOUND, "method not found")
            }
        }
    }

    fn handle_notification(&mut self, notification: Notification) -> Result<bool> {
        let method = notification.method.clone();

        // Notifications do not have a request ID to fail back through, so this
        // outer boundary is the narrowest place we can contain a panic, log it
        // as a bug, and keep the process alive for the next message.
        match catch_unwind("notification_dispatch", None, None, || self.dispatch_notification(notification)) {
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

    fn dispatch_notification(&mut self, notification: Notification) -> Result<bool> {
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
                self.handle_did_close(params);
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
        let package_root = self.project_model.resolve_package_root(&document.uri);
        let prepared = self.documents.prepare_open(
            document.uri,
            document.language_id,
            document.version,
            document.text,
            package_root,
        );

        self.hooks.maybe_panic_notification(DID_OPEN);

        let snapshot = self.documents.commit_open(prepared);
        self.scheduler.enqueue(snapshot);
    }

    fn handle_did_change(&mut self, params: DidChangeTextDocumentParams) {
        let DidChangeTextDocumentParams { text_document, content_changes } = params;

        let Some(text) = extract_full_sync_text(content_changes) else {
            return;
        };

        let Some(prepared) = self.documents.prepare_full_change(&text_document.uri, text_document.version, text) else {
            tracing::debug!(uri = text_document.uri.as_str(), "ignoring didChange for unopened document");
            return;
        };

        self.hooks.maybe_panic_notification(DID_CHANGE);

        let snapshot = self.documents.commit_change(prepared);
        self.scheduler.enqueue(snapshot);
    }

    fn handle_did_close(&mut self, params: DidCloseTextDocumentParams) {
        self.hooks.maybe_panic_notification(DID_CLOSE);
        self.documents.close(&params.text_document.uri);
    }

    fn handle_worker_event(&mut self, event: WorkerEvent) {
        match event {
            WorkerEvent::Completed { uri, generation } => {
                // Worker jobs finish asynchronously, so only the latest
                // committed generation is allowed to surface as current.
                if self.documents.generation(&uri) == Some(generation) {
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
            WorkerEvent::Panicked(report) => {
                report.log();
            }
        }
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
    use lsp_server::{Connection, Message, Notification, Request, Response};
    use serde_json::{Value, json};
    use std::{process::ExitCode, thread, time::Duration};

    fn spawn_server(hooks: TestHooks) -> (Connection, thread::JoinHandle<anyhow::Result<ExitCode>>) {
        // Use an in-memory transport so these tests exercise the real server
        // event loop without paying the cost of subprocess management.
        let (server, client) = Connection::memory();
        let handle = thread::spawn(move || run_with_hooks(server, hooks));
        (client, handle)
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
}
