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

mod common;

use self::common::TestServer;
use lsp_types::Uri;
use serde_json::{Value, json};
use std::{fs, path::Path, time::Duration};
use tempfile::tempdir;

fn initialize(server: &mut TestServer) -> Value {
    // Tests assert on the raw initialize response, so this helper stops before
    // sending the follow-up `initialized` notification.
    server.request(
        1,
        "initialize",
        json!({
            "processId": null,
            "rootUri": null,
            "capabilities": {},
        }),
    )
}

fn file_uri(path: &Path) -> Uri {
    #[cfg(target_os = "windows")]
    let path = format!("/{}", path.display()).replace('\\', "/");

    #[cfg(not(target_os = "windows"))]
    let path = path.display().to_string();

    format!("file://{path}").parse().expect("file uri")
}

#[test]
fn initialize_shutdown_exit_round_trip() {
    let mut server = TestServer::spawn(&[]);

    let initialize = initialize(&mut server);
    assert_eq!(initialize["id"], 1);
    assert_eq!(initialize["result"]["serverInfo"]["name"], "leo-lsp");
    assert_eq!(
        initialize["result"]["capabilities"]["textDocumentSync"],
        json!({
            "openClose": true,
            "change": 1,
        })
    );

    server.notify("initialized", json!({}));

    let shutdown = server.request(2, "shutdown", Value::Null);
    assert_eq!(shutdown["id"], 2);
    assert_eq!(shutdown["result"], Value::Null);

    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();

    assert!(status.success(), "stderr:\n{stderr}");
}

#[test]
fn exit_without_shutdown_returns_nonzero() {
    let mut server = TestServer::spawn(&[]);
    initialize(&mut server);
    server.notify("initialized", json!({}));

    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();

    assert!(!status.success(), "stderr:\n{stderr}");
}

#[test]
fn malformed_open_change_close_lifecycle_stays_alive() {
    let tempdir = tempdir().expect("tempdir");
    let package_root = tempdir.path().join("example");
    let source_dir = package_root.join("src");
    fs::create_dir_all(&source_dir).expect("create source dir");
    fs::write(package_root.join("program.json"), "{}").expect("write manifest");

    let file_uri = file_uri(&source_dir.join("main.leo"));
    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    server.notify("initialized", json!({}));

    server.notify(
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": file_uri,
                "languageId": "leo",
                "version": 1,
                "text": "program test.aleo { fn main( {",
            }
        }),
    );

    server.notify(
        "textDocument/didChange",
        json!({
            "textDocument": {
                "uri": file_uri,
                "version": 2,
            },
            "contentChanges": [
                {
                    "text": "program test.aleo { fn main() { let x = ; } }",
                }
            ]
        }),
    );

    assert!(
        server.wait_for_stderr_contains("worker completed latest document", Duration::from_secs(1)),
        "stderr:\n{}",
        server.finish().1
    );

    server.notify(
        "textDocument/didClose",
        json!({
            "textDocument": {
                "uri": file_uri,
            }
        }),
    );

    let shutdown = server.request(2, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);

    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
}

#[test]
fn malformed_notification_payload_stays_alive() {
    let mut server = TestServer::spawn(&[]);
    initialize(&mut server);
    server.notify("initialized", json!({}));

    server.notify("textDocument/didOpen", json!({}));

    let shutdown = server.request(2, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);

    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
    assert!(stderr.contains("failed to handle client notification"), "stderr:\n{stderr}");
}
