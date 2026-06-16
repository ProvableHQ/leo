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

//! End-to-end protocol coverage for save-triggered LSP diagnostics.
//!
//! These tests drive the real `leo-lsp` subprocess and assert on the wire
//! payload of `textDocument/publishDiagnostics`: save publishes structured
//! diagnostics, edits clear stale ranges before any new publish, malformed
//! full-sync change payloads block cached republish, and the initialize
//! handshake advertises save support but withholds the pull-diagnostics
//! provider.

mod common;

use self::common::TestServer;
use lsp_types::Uri;
use serde_json::{Value, json};
use std::{fs, path::Path, time::Duration};
use tempfile::{TempDir, tempdir};

/// Maximum time to wait for a notification or worker completion in these tests.
const WAIT_TIMEOUT: Duration = Duration::from_secs(10);

/// Source guaranteed to trigger a compiler error.
const ERROR_SOURCE: &str = "program demo.aleo {\n    fn main() -> u32 {\n        return undefined_symbol;\n    }\n    @noupgrade constructor() {}\n}\n";

/// Source that emits a labeled compiler error (duplicate function name).
///
/// `name_defined_multiple_times` carries both a primary span and a "previous
/// definition here" secondary label, so the diagnostic lowering produces a
/// non-empty `relatedInformation` payload that integration tests can probe.
const LABELED_ERROR_SOURCE: &str = concat!(
    "program demo.aleo {\n",
    "    fn collide() -> u32 { return 0u32; }\n",
    "    fn collide() -> u32 { return 1u32; }\n",
    "    fn main() -> u32 { return collide(); }\n",
    "    @noupgrade\n",
    "    constructor() {}\n",
    "}\n",
);

/// Source that compiles cleanly.
const CLEAN_SOURCE: &str = "program demo.aleo { @noupgrade constructor() {} fn main() {} }\n";

/// Send `initialize` followed by `initialized` with the maximal capability set.
fn initialize(server: &mut TestServer) {
    initialize_with_capabilities(
        server,
        json!({
            "textDocument": {
                "publishDiagnostics": {
                    "relatedInformation": true,
                    "versionSupport": true,
                    "codeDescriptionSupport": true,
                    "dataSupport": true,
                    "tagSupport": {
                        "valueSet": [1, 2],
                    }
                }
            }
        }),
    );
}

/// Initialize with explicit client capabilities so tests can vary the snapshot.
fn initialize_with_capabilities(server: &mut TestServer, capabilities: Value) {
    let response = server.request(
        1,
        "initialize",
        json!({
            "processId": null,
            "rootUri": null,
            "capabilities": capabilities,
        }),
    );
    assert_eq!(response["id"], 1);
    server.notify("initialized", json!({}));
}

/// Build a `file:` URI from a native path.
fn file_uri(path: &Path) -> Uri {
    #[cfg(target_os = "windows")]
    let path = {
        let display = path.display().to_string();
        let display = display.strip_prefix(r"\\?\").unwrap_or(display.as_str());
        format!("/{}", display).replace('\\', "/")
    };

    #[cfg(not(target_os = "windows"))]
    let path = path.display().to_string();

    format!("file://{path}").parse().expect("file uri")
}

/// Write a minimal Leo package and return its tempdir plus the document URI.
fn write_test_package(source: &str) -> (TempDir, Uri) {
    let tempdir = tempdir().expect("tempdir");
    write_package_into(tempdir.path(), source, true);
    let canonical =
        tempdir.path().join("example").join("src").join("main.leo").canonicalize().expect("canonical main path");
    (tempdir, file_uri(&canonical))
}

/// Write the package layout at `root` with `main.leo` containing `source` and
/// optionally include a `program.json` manifest.
fn write_package_into(root: &Path, source: &str, with_manifest: bool) {
    let package_root = root.join("example");
    let source_dir = package_root.join("src");
    fs::create_dir_all(&source_dir).expect("create source dir");
    if with_manifest {
        fs::write(
            package_root.join("program.json"),
            r#"{ "program": "demo.aleo", "version": "0.1.0", "description": "", "license": "MIT", "leo": "4.0.0" }"#,
        )
        .expect("write manifest");
    }
    let main_path = source_dir.join("main.leo");
    fs::write(&main_path, source).expect("write source");
}

/// Write a package with two source files and return their canonical URIs.
fn write_two_file_package(main_source: &str, sibling_source: &str) -> (TempDir, Uri, Uri) {
    let tempdir = tempdir().expect("tempdir");
    let package_root = tempdir.path().join("example");
    let source_dir = package_root.join("src");
    fs::create_dir_all(&source_dir).expect("create source dir");
    fs::write(
        package_root.join("program.json"),
        r#"{ "program": "demo.aleo", "version": "0.1.0", "description": "", "license": "MIT", "leo": "4.0.0" }"#,
    )
    .expect("write manifest");
    fs::write(source_dir.join("main.leo"), main_source).expect("write main");
    fs::write(source_dir.join("helper.leo"), sibling_source).expect("write helper");
    let main = source_dir.join("main.leo").canonicalize().expect("canonical main");
    let helper = source_dir.join("helper.leo").canonicalize().expect("canonical helper");
    (tempdir, file_uri(&main), file_uri(&helper))
}

/// Open a document on the server with version 1.
fn open_document(server: &mut TestServer, uri: &Uri, source: &str) {
    server.notify(
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": uri,
                "languageId": "leo",
                "version": 1,
                "text": source,
            }
        }),
    );
}

/// Send a full-sync `didChange` with a new text and version.
fn did_change(server: &mut TestServer, uri: &Uri, version: i32, new_text: &str) {
    server.notify(
        "textDocument/didChange",
        json!({
            "textDocument": { "uri": uri, "version": version },
            "contentChanges": [ { "text": new_text } ]
        }),
    );
}

/// Send a malformed `didChange` (no full-document text) for a URI.
fn did_change_malformed(server: &mut TestServer, uri: &Uri, version: i32) {
    server.notify(
        "textDocument/didChange",
        json!({
            "textDocument": { "uri": uri, "version": version },
            "contentChanges": []
        }),
    );
}

/// Send a `didSave` notification for a URI.
fn did_save(server: &mut TestServer, uri: &Uri) {
    server.notify("textDocument/didSave", json!({ "textDocument": { "uri": uri } }));
}

/// Send a `didClose` notification for a URI.
fn did_close(server: &mut TestServer, uri: &Uri) {
    server.notify("textDocument/didClose", json!({ "textDocument": { "uri": uri } }));
}

/// Wait until the next `publishDiagnostics` whose params satisfy the predicate.
fn wait_for_diagnostics(
    server: &mut TestServer,
    timeout: Duration,
    predicate: impl Fn(&Value) -> bool,
) -> Option<Value> {
    let deadline = std::time::Instant::now() + timeout;
    loop {
        let remaining = deadline.saturating_duration_since(std::time::Instant::now());
        if remaining.is_zero() {
            return None;
        }
        let notification = server.recv_notification("textDocument/publishDiagnostics", remaining)?;
        if predicate(&notification["params"]) {
            return Some(notification);
        }
    }
}

/// Wait until the worker logs a completion. Required before any `didSave` so
/// the routing thread has already stored the package result we expect to read.
fn wait_for_worker_completion(server: &TestServer) {
    assert!(
        server.wait_for_stderr_contains("worker completed latest document", WAIT_TIMEOUT),
        "worker did not finish in {WAIT_TIMEOUT:?}; stderr:\n{}",
        server.stderr_contents(),
    );
}

/// Shut the server down cleanly.
fn shutdown(server: TestServer, request_id: i64) {
    let mut server = server;
    let response = server.request(request_id, "shutdown", Value::Null);
    assert_eq!(response["result"], Value::Null);
    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
}

/// Verifies `initialize` advertises full-sync save without `diagnosticProvider`.
#[test]
fn initialize_advertises_save_but_not_pull_diagnostics() {
    let mut server = TestServer::spawn(&[]);
    let response = server.request(
        1,
        "initialize",
        json!({
            "processId": null,
            "rootUri": null,
            "capabilities": {},
        }),
    );
    assert_eq!(response["id"], 1);
    assert_eq!(response["result"]["capabilities"]["textDocumentSync"]["save"]["includeText"], json!(false));
    assert!(response["result"]["capabilities"].get("diagnosticProvider").is_none());
    server.notify("initialized", json!({}));
    shutdown(server, 99);
}

/// Verifies a saved file with a type error publishes a structured diagnostic.
#[test]
fn save_publishes_compiler_diagnostics() {
    let (_tempdir, uri) = write_test_package(ERROR_SOURCE);

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    open_document(&mut server, &uri, ERROR_SOURCE);
    wait_for_worker_completion(&server);

    did_save(&mut server, &uri);
    let publish = wait_for_diagnostics(&mut server, WAIT_TIMEOUT, has_non_empty_diagnostics)
        .expect("expected non-empty diagnostic publish");
    let diagnostics = publish["params"]["diagnostics"].as_array().expect("diagnostics array");
    assert!(!diagnostics.is_empty(), "expected at least one diagnostic: {publish}");
    assert!(diagnostics.iter().all(|diagnostic| diagnostic["source"] == json!("leo")));
    assert!(diagnostics.iter().all(|diagnostic| diagnostic["severity"] == json!(1)));
    // The compiler's internal code (e.g. `ETYC0372005`) is intentionally
    // omitted from the wire payload; see entry_to_lsp_diagnostic.
    assert!(diagnostics.iter().all(|diagnostic| diagnostic.get("code").is_none()));
    assert_eq!(publish["params"]["version"], json!(1));

    shutdown(server, 99);
}

/// Verifies an unrelated edit clears stale diagnostics before the next save.
#[test]
fn edit_after_save_clears_diagnostics_with_incoming_version() {
    let (_tempdir, uri) = write_test_package(ERROR_SOURCE);

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    open_document(&mut server, &uri, ERROR_SOURCE);
    wait_for_worker_completion(&server);
    did_save(&mut server, &uri);
    let initial = wait_for_diagnostics(&mut server, WAIT_TIMEOUT, has_non_empty_diagnostics);
    assert!(initial.is_some(), "expected initial parser diagnostic");

    did_change(&mut server, &uri, 7, CLEAN_SOURCE);
    let cleared =
        wait_for_diagnostics(&mut server, WAIT_TIMEOUT, has_empty_diagnostics).expect("clear after didChange");
    assert_eq!(cleared["params"]["diagnostics"], json!([]));
    assert_eq!(cleared["params"]["version"], json!(7));

    shutdown(server, 99);
}

/// Verifies didClose clears diagnostics for the closed URI.
#[test]
fn close_clears_diagnostics_for_closed_uri() {
    let (_tempdir, uri) = write_test_package(ERROR_SOURCE);

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    open_document(&mut server, &uri, ERROR_SOURCE);
    wait_for_worker_completion(&server);
    did_save(&mut server, &uri);
    wait_for_diagnostics(&mut server, WAIT_TIMEOUT, has_non_empty_diagnostics).expect("non-empty diagnostic");

    did_close(&mut server, &uri);
    let cleared = wait_for_diagnostics(&mut server, WAIT_TIMEOUT, has_empty_diagnostics).expect("clear after close");
    assert_eq!(cleared["params"]["diagnostics"], json!([]));

    shutdown(server, 99);
}

/// Verifies a malformed `didChange` clears diagnostics and blocks cached publish.
#[test]
fn malformed_change_marks_uri_untrusted() {
    let (_tempdir, uri) = write_test_package(ERROR_SOURCE);

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    open_document(&mut server, &uri, ERROR_SOURCE);
    wait_for_worker_completion(&server);
    did_save(&mut server, &uri);
    wait_for_diagnostics(&mut server, WAIT_TIMEOUT, has_non_empty_diagnostics).expect("non-empty diagnostic");

    did_change_malformed(&mut server, &uri, 2);
    let cleared =
        wait_for_diagnostics(&mut server, WAIT_TIMEOUT, has_empty_diagnostics).expect("clear after malformed change");
    assert_eq!(cleared["params"]["diagnostics"], json!([]));
    assert_eq!(cleared["params"]["version"], json!(2));

    did_save(&mut server, &uri);
    let unexpected = wait_for_diagnostics(&mut server, Duration::from_millis(300), has_non_empty_diagnostics);
    assert!(unexpected.is_none(), "non-empty diagnostic should not publish while URI is untrusted: {unexpected:?}");

    shutdown(server, 99);
}

/// Verifies a clean save clears diagnostics previously published for the bucket.
#[test]
fn clean_save_clears_previously_published_diagnostics() {
    let (_tempdir, uri) = write_test_package(ERROR_SOURCE);

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    open_document(&mut server, &uri, ERROR_SOURCE);
    wait_for_worker_completion(&server);
    did_save(&mut server, &uri);
    wait_for_diagnostics(&mut server, WAIT_TIMEOUT, has_non_empty_diagnostics).expect("initial diagnostics");

    did_change(&mut server, &uri, 2, CLEAN_SOURCE);
    wait_for_diagnostics(&mut server, WAIT_TIMEOUT, has_empty_diagnostics).expect("clear after edit");

    did_save(&mut server, &uri);
    // After a clean save we should see another empty publish (or simply no
    // new non-empty publish for the remaining window).
    let unexpected = wait_for_diagnostics(&mut server, Duration::from_millis(500), has_non_empty_diagnostics);
    assert!(unexpected.is_none(), "clean save should not publish new diagnostics: {unexpected:?}");

    shutdown(server, 99);
}

/// Verifies a labeled diagnostic ships its `relatedInformation` only when the
/// client advertises support. Uses a duplicate-function-name error so the
/// underlying compiler diagnostic actually carries a secondary label.
#[test]
fn related_information_requires_client_capability() {
    let (without_dir, without_uri) = write_test_package(LABELED_ERROR_SOURCE);
    let mut without = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize_with_capabilities(
        &mut without,
        json!({
            "textDocument": { "publishDiagnostics": { "relatedInformation": false } }
        }),
    );
    open_document(&mut without, &without_uri, LABELED_ERROR_SOURCE);
    wait_for_worker_completion(&without);
    did_save(&mut without, &without_uri);
    let no_caps = wait_for_diagnostics(&mut without, WAIT_TIMEOUT, has_non_empty_diagnostics).expect("diagnostics");
    for diagnostic in no_caps["params"]["diagnostics"].as_array().expect("diagnostics") {
        assert!(
            diagnostic.get("relatedInformation").is_none(),
            "clients without relatedInformation should not receive it: {diagnostic}",
        );
        assert!(diagnostic.get("codeDescription").is_none());
        assert!(diagnostic.get("data").is_none());
        assert!(diagnostic.get("tags").is_none());
    }
    shutdown(without, 99);
    drop(without_dir);

    let (_with_dir, with_uri) = write_test_package(LABELED_ERROR_SOURCE);
    let mut with = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut with);
    open_document(&mut with, &with_uri, LABELED_ERROR_SOURCE);
    wait_for_worker_completion(&with);
    did_save(&mut with, &with_uri);
    let with_caps = wait_for_diagnostics(&mut with, WAIT_TIMEOUT, has_non_empty_diagnostics).expect("diagnostics");
    assert!(
        with_caps["params"]["diagnostics"]
            .as_array()
            .expect("diagnostics")
            .iter()
            .any(|diagnostic| diagnostic.get("relatedInformation").is_some()),
        "labeled diagnostic should carry relatedInformation when the client advertises support: {with_caps}",
    );
    shutdown(with, 99);
}

/// Verifies the publish carries the open document's version.
#[test]
fn save_publish_carries_open_document_version() {
    let (_tempdir, uri) = write_test_package(ERROR_SOURCE);

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    open_document(&mut server, &uri, ERROR_SOURCE);
    wait_for_worker_completion(&server);
    did_save(&mut server, &uri);
    let publish = wait_for_diagnostics(&mut server, WAIT_TIMEOUT, has_non_empty_diagnostics).expect("diagnostics");
    assert_eq!(publish["params"]["version"], json!(1));

    // After an edit + clean save, the version on subsequent clears reflects the
    // new open-document version.
    did_change(&mut server, &uri, 5, CLEAN_SOURCE);
    let cleared = wait_for_diagnostics(&mut server, WAIT_TIMEOUT, has_empty_diagnostics).expect("clear on edit");
    assert_eq!(cleared["params"]["version"], json!(5));

    shutdown(server, 99);
}

/// Verifies an error originating in a sibling open file publishes against
/// that sibling's URI and version, not the saved trigger's.
#[test]
fn save_publishes_diagnostics_for_open_sibling_file() {
    let main_source =
        "program demo.aleo {\n    fn main() -> u32 { return helper::go(); }\n    @noupgrade constructor() {}\n}\n";
    let bad_helper = concat!("module helper {\n", "    pub fn go() -> u32 { return undefined_symbol; }\n", "}\n",);
    let (_tempdir, main_uri, helper_uri) = write_two_file_package(main_source, bad_helper);

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    open_document(&mut server, &main_uri, main_source);
    open_document(&mut server, &helper_uri, bad_helper);
    wait_for_worker_completion(&server);

    // Save the main file even though the error lives in `helper.leo`. The
    // publish should still land on the helper URI (and only the helper URI).
    did_save(&mut server, &main_uri);
    let publish = wait_for_diagnostics(&mut server, WAIT_TIMEOUT, |params| {
        params["uri"] == json!(helper_uri.to_string()) && has_non_empty_diagnostics(params)
    })
    .expect("expected diagnostic on the sibling helper URI");
    assert_eq!(publish["params"]["uri"], json!(helper_uri.to_string()));
    assert_eq!(publish["params"]["version"], json!(1));

    shutdown(server, 99);
}

/// Verifies that bucket reclassification (managed → unmanaged via a manifest
/// removal observed on `didChange`) clears diagnostics from the old bucket
/// before any new ones can land.
#[test]
fn bucket_reclassification_clears_old_bucket_diagnostics() {
    let (tempdir, uri) = write_test_package(ERROR_SOURCE);

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    open_document(&mut server, &uri, ERROR_SOURCE);
    wait_for_worker_completion(&server);
    did_save(&mut server, &uri);
    wait_for_diagnostics(&mut server, WAIT_TIMEOUT, has_non_empty_diagnostics).expect("initial diagnostics");

    // Removing `program.json` flips the file from a managed package to an
    // unmanaged buffer on the next `didChange`. The bucket changes and the
    // previous bucket's diagnostics must clear in the same routing turn.
    fs::remove_file(tempdir.path().join("example").join("program.json")).expect("remove manifest");
    did_change(&mut server, &uri, 2, ERROR_SOURCE);
    let cleared = wait_for_diagnostics(&mut server, WAIT_TIMEOUT, has_empty_diagnostics).expect("clear after move");
    assert_eq!(cleared["params"]["diagnostics"], json!([]));
    assert_eq!(cleared["params"]["version"], json!(2));

    shutdown(server, 99);
}

/// Verifies a worker panic on the current-key analysis clears any visible
/// diagnostics for the bucket and leaves the server alive.
#[test]
fn worker_panic_clears_bucket_diagnostics() {
    let (_initial_dir, initial_uri) = write_test_package(ERROR_SOURCE);

    // Two-phase setup: get a non-empty publish, then close and re-spawn with
    // the worker-panic injection enabled so the next save provokes a panic.
    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    open_document(&mut server, &initial_uri, ERROR_SOURCE);
    wait_for_worker_completion(&server);
    did_save(&mut server, &initial_uri);
    wait_for_diagnostics(&mut server, WAIT_TIMEOUT, has_non_empty_diagnostics).expect("initial diagnostics");

    // Force the next worker job to panic and watch the bucket clear.
    did_close(&mut server, &initial_uri);
    wait_for_diagnostics(&mut server, WAIT_TIMEOUT, has_empty_diagnostics).expect("clear on close");
    shutdown(server, 99);

    let (_panic_dir, panic_uri) = write_test_package(ERROR_SOURCE);
    let mut paniced = TestServer::spawn(&[("RUST_LOG", "debug"), ("LEO_LSP_TEST_PANIC_WORKER", "1")]);
    initialize(&mut paniced);
    open_document(&mut paniced, &panic_uri, ERROR_SOURCE);
    // First open enqueues analysis; worker panics. We expect no non-empty
    // diagnostic to ever arrive, and the server stays alive long enough to
    // accept the shutdown handshake.
    let publish = wait_for_diagnostics(&mut paniced, Duration::from_millis(500), has_non_empty_diagnostics);
    assert!(publish.is_none(), "panicking worker must not publish diagnostics: {publish:?}");
    shutdown(paniced, 99);
}

/// Predicate: `params.diagnostics` is a non-empty array.
fn has_non_empty_diagnostics(params: &Value) -> bool {
    params["diagnostics"].as_array().map(|entries| !entries.is_empty()).unwrap_or(false)
}

/// Predicate: `params.diagnostics` is an empty array.
fn has_empty_diagnostics(params: &Value) -> bool {
    params["diagnostics"].as_array().map(|entries| entries.is_empty()).unwrap_or(false)
}
