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
    assert_eq!(initialize["result"]["capabilities"]["semanticTokensProvider"]["full"], json!(true));
    assert_eq!(
        initialize["result"]["capabilities"]["semanticTokensProvider"]["legend"]["tokenTypes"],
        json!([
            "namespace",
            "type",
            "interface",
            "function",
            "parameter",
            "variable",
            "property",
            "keyword",
            "comment",
            "string",
            "number",
            "operator"
        ])
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

#[test]
fn semantic_tokens_full_returns_tokens_and_reuses_cached_snapshot() {
    let tempdir = tempdir().expect("tempdir");
    let package_root = tempdir.path().join("example");
    let source_dir = package_root.join("src");
    fs::create_dir_all(&source_dir).expect("create source dir");
    fs::write(
        package_root.join("program.json"),
        r#"{ "program": "demo.aleo", "version": "0.1.0", "description": "", "license": "MIT", "leo": "4.0.0" }"#,
    )
    .expect("write manifest");

    let source = concat!(
        "program demo.aleo {\n",
        "    struct Point { x: u32, }\n\n",
        "    fn main(point: Point) -> u32 {\n",
        "        let local: Point = Point { x: 1u32 };\n",
        "        return point.x + local.x;\n",
        "    }\n",
        "}\n",
    );
    let main_path = source_dir.join("main.leo");
    fs::write(&main_path, source).expect("write source");
    let file_uri = file_uri(&main_path);

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
                "text": source,
            }
        }),
    );

    let first = server.request(
        2,
        "textDocument/semanticTokens/full",
        json!({
            "textDocument": {
                "uri": file_uri,
            }
        }),
    );
    let first_data = first["result"]["data"].as_array().expect("semantic token data");
    assert!(!first_data.is_empty(), "expected semantic tokens, got {first}");
    assert_eq!(server.stderr_contents().matches("worker completed latest document").count(), 1);

    let second = server.request(
        3,
        "textDocument/semanticTokens/full",
        json!({
            "textDocument": {
                "uri": file_uri,
            }
        }),
    );
    assert_eq!(first["result"], second["result"]);
    assert_eq!(server.stderr_contents().matches("worker completed latest document").count(), 1);

    server.notify(
        "textDocument/didClose",
        json!({
            "textDocument": {
                "uri": file_uri,
            }
        }),
    );

    let closed = server.request(
        4,
        "textDocument/semanticTokens/full",
        json!({
            "textDocument": {
                "uri": file_uri,
            }
        }),
    );
    assert_eq!(closed["result"]["data"], json!([]));

    let shutdown = server.request(5, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);

    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
}

#[test]
fn semantic_tokens_full_returns_internal_error_after_worker_panic() {
    let mut server = TestServer::spawn(&[("LEO_LSP_TEST_PANIC_WORKER", "1")]);
    initialize(&mut server);
    server.notify("initialized", json!({}));

    let uri = "untitled:main.leo";
    server.notify(
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": uri,
                "languageId": "leo",
                "version": 1,
                "text": "program test.aleo {}\n",
            }
        }),
    );

    let response = server.request(
        2,
        "textDocument/semanticTokens/full",
        json!({
            "textDocument": {
                "uri": uri,
            }
        }),
    );
    assert_eq!(response["error"]["code"], json!(-32603));
    assert!(response["error"]["message"].as_str().expect("panic error message").contains("panicked"));

    let shutdown = server.request(3, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);

    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
    assert!(stderr.contains("INTERNAL PANIC"), "stderr:\n{stderr}");
}
