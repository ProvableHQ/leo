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

const PANIC_NOTICE: &str =
    "INTERNAL PANIC: this indicates a bug in the Leo compiler or language server implementation.";

fn initialize(server: &mut TestServer) {
    let response = server.request(
        1,
        "initialize",
        json!({
            "processId": null,
            "rootUri": null,
            "capabilities": {},
        }),
    );

    assert_eq!(response["result"]["serverInfo"]["name"], "leo-lsp");
    // Mirror the real client handshake so notification handling is exercised
    // under the same lifecycle the binary sees in editors.
    server.notify("initialized", json!({}));
}

fn package_file_uri() -> (tempfile::TempDir, Uri) {
    let tempdir = tempdir().expect("tempdir");
    let package_root = tempdir.path().join("example");
    let source_dir = package_root.join("src");
    fs::create_dir_all(&source_dir).expect("create source dir");
    // These tests go through package-root resolution, so create the smallest
    // valid Leo package layout instead of an unmanaged loose file.
    fs::write(package_root.join("program.json"), "{}").expect("write manifest");
    let uri = file_uri(&source_dir.join("main.leo"));
    (tempdir, uri)
}

fn file_uri(path: &Path) -> Uri {
    #[cfg(target_os = "windows")]
    let path = format!("/{}", path.display()).replace('\\', "/");

    #[cfg(not(target_os = "windows"))]
    let path = path.display().to_string();

    format!("file://{path}").parse().expect("file uri")
}

#[test]
fn main_thread_notification_panic_does_not_kill_server() {
    let (_tempdir, file_uri) = package_file_uri();
    let mut server = TestServer::spawn(&[("LEO_LSP_TEST_PANIC_NOTIFICATION", "textDocument/didChange")]);
    initialize(&mut server);

    server.notify(
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": file_uri,
                "languageId": "leo",
                "version": 1,
                "text": "program test.aleo {}",
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
                    "text": "program test.aleo { fn main() {} }",
                }
            ]
        }),
    );

    let shutdown = server.request(2, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);

    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
    assert!(stderr.contains(PANIC_NOTICE), "stderr:\n{stderr}");
    assert!(stderr.contains("Please report it at"), "stderr:\n{stderr}");
}

#[test]
fn worker_panic_does_not_kill_server_and_stdout_stays_clean() {
    let (_tempdir, file_uri) = package_file_uri();
    let mut server = TestServer::spawn(&[("LEO_LSP_TEST_PANIC_WORKER", "1"), ("RUST_LOG", "debug")]);
    initialize(&mut server);

    server.notify(
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": file_uri,
                "languageId": "leo",
                "version": 1,
                "text": "program test.aleo { fn main() {} }",
            }
        }),
    );

    assert!(server.wait_for_stderr_contains(PANIC_NOTICE, Duration::from_secs(1)), "stderr:\n{}", server.finish().1);

    let shutdown = server.request(2, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);

    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
    assert!(stderr.contains(PANIC_NOTICE), "stderr:\n{stderr}");
    assert!(stderr.contains("Please report it at"), "stderr:\n{stderr}");
}
