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

/// Send the initialize request and return the raw response for protocol assertions.
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

/// Build a test `file:` URI from a native path.
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

/// Return the LSP position for the selected occurrence of a source substring.
fn position_json(source: &str, needle: &str, occurrence: usize) -> Value {
    let offset = source
        .match_indices(needle)
        .nth(occurrence)
        .map(|(offset, _)| offset)
        .unwrap_or_else(|| panic!("missing occurrence {occurrence} of {needle:?}"));
    let line = source[..offset].bytes().filter(|byte| *byte == b'\n').count() as u32;
    let line_start = source[..offset].rfind('\n').map_or(0, |index| index + 1);
    json!({ "line": line, "character": (offset - line_start) as u32 })
}

/// Return the LSP range for the selected occurrence of a source substring.
fn range_json(source: &str, needle: &str, occurrence: usize) -> Value {
    let start = source
        .match_indices(needle)
        .nth(occurrence)
        .map(|(offset, _)| offset)
        .unwrap_or_else(|| panic!("missing occurrence {occurrence} of {needle:?}"));
    let end = start + needle.len();
    let position = |offset| {
        let line = source[..offset].bytes().filter(|byte| *byte == b'\n').count() as u32;
        let line_start = source[..offset].rfind('\n').map_or(0, |index| index + 1);
        json!({ "line": line, "character": (offset - line_start) as u32 })
    };
    json!({ "start": position(start), "end": position(end) })
}

/// Verifies initialize, shutdown, and exit follow the LSP lifecycle.
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
    assert_eq!(initialize["result"]["capabilities"]["definitionProvider"], json!(true));
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

/// Verifies go-to-definition returns a local variable declaration target.
#[test]
fn definition_returns_local_variable_declaration() {
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
        "    fn main() -> u32 {\n",
        "        let total: u32 = 1u32;\n",
        "        return total;\n",
        "    }\n",
        "}\n",
    );
    let main_path = source_dir.join("main.leo");
    fs::write(&main_path, source).expect("write source");
    let document_uri = file_uri(&main_path);
    let canonical_file_uri = file_uri(&main_path.canonicalize().expect("canonical main path"));

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    server.notify("initialized", json!({}));

    server.notify(
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": document_uri,
                "languageId": "leo",
                "version": 1,
                "text": source,
            }
        }),
    );

    let response = server.request(
        2,
        "textDocument/definition",
        json!({
            "textDocument": {
                "uri": document_uri,
            },
            "position": {
                "line": 3,
                "character": 16,
            }
        }),
    );

    assert_eq!(response["id"], 2);
    assert_eq!(response["result"][0]["uri"], json!(canonical_file_uri.to_string()));
    assert_eq!(
        response["result"][0]["range"],
        json!({
            "start": { "line": 2, "character": 12 },
            "end": { "line": 2, "character": 17 },
        })
    );

    let shutdown = server.request(3, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);

    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
}

/// Verifies go-to-definition resolves function, type, and member targets.
#[test]
fn definition_resolves_function_type_and_member_targets() {
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
        "struct Point { x: u32, }\n\n",
        "fn helper(point: Point) -> u32 {\n",
        "    return point.x;\n",
        "}\n\n",
        "program demo.aleo {\n",
        "    fn main() -> u32 {\n",
        "        let local: Point = Point { x: 1u32 };\n",
        "        return helper(local);\n",
        "    }\n",
        "}\n",
    );
    let main_path = source_dir.join("main.leo");
    fs::write(&main_path, source).expect("write source");
    let document_uri = file_uri(&main_path);
    let canonical_uri = file_uri(&main_path.canonicalize().expect("canonical main path"));

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    server.notify("initialized", json!({}));
    server.notify(
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": document_uri,
                "languageId": "leo",
                "version": 1,
                "text": source,
            }
        }),
    );

    let cases = [
        (2, "helper", 1, range_json(source, "helper", 0)),
        (3, "Point", 2, range_json(source, "Point", 0)),
        (4, "x", 1, range_json(source, "x", 0)),
    ];

    for (id, needle, occurrence, expected_range) in cases {
        let response = server.request(
            id,
            "textDocument/definition",
            json!({
                "textDocument": {
                    "uri": document_uri,
                },
                "position": position_json(source, needle, occurrence),
            }),
        );
        assert_eq!(
            response["result"][0]["uri"],
            json!(canonical_uri.to_string()),
            "bad uri for {needle}: {response}; stderr:\n{}",
            server.stderr_contents()
        );
        assert_eq!(response["result"][0]["range"], expected_range, "bad target for {needle}");
    }

    let shutdown = server.request(5, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);

    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
}

/// Verifies loose program files still resolve struct field member targets.
#[test]
fn definition_resolves_unmanaged_struct_field_targets() {
    let tempdir = tempdir().expect("tempdir");
    let source = concat!(
        "program test.aleo {\n",
        "    struct Point {\n",
        "        x: u64,\n",
        "        y: u64,\n",
        "    }\n\n",
        "    struct TransferInfo {\n",
        "        sender_amount: u64,\n",
        "        receiver_amount: u64,\n",
        "        transfer_fee: u64,\n",
        "    }\n\n",
        "    fn main(public a: u64, public b: u64) -> u64 {\n",
        "        let p: Point = Point { x: 1u64, y: 2u64 };\n",
        "        let info: TransferInfo = TransferInfo {\n",
        "            sender_amount: a,\n",
        "            receiver_amount: b,\n",
        "            transfer_fee: 10u64,\n",
        "        };\n",
        "        return p.x + info.sender_amount;\n",
        "    }\n",
        "}\n",
    );
    let file_path = tempdir.path().join("wrap_struct_expr.leo");
    fs::write(&file_path, source).expect("write source");
    let document_uri = file_uri(&file_path);
    let canonical_uri = file_uri(&file_path.canonicalize().expect("canonical source"));

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    server.notify("initialized", json!({}));
    server.notify(
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": document_uri,
                "languageId": "leo",
                "version": 1,
                "text": source,
            }
        }),
    );

    let response = server.request(
        2,
        "textDocument/definition",
        json!({
            "textDocument": {
                "uri": document_uri,
            },
            "position": position_json(source, "sender_amount", 2),
        }),
    );

    assert_eq!(
        response["result"][0]["uri"],
        json!(canonical_uri.to_string()),
        "bad uri for sender_amount: {response}; stderr:\n{}",
        server.stderr_contents()
    );
    assert_eq!(response["result"][0]["range"], range_json(source, "sender_amount", 0));

    let shutdown = server.request(3, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);

    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
}

/// Verifies go-to-definition can target saved local dependency sources.
#[test]
fn definition_resolves_saved_local_source_dependency_target() {
    let tempdir = tempdir().expect("tempdir");

    let helper_root = tempdir.path().join("helper");
    let helper_src = helper_root.join("src");
    fs::create_dir_all(&helper_src).expect("create helper source dir");
    fs::write(
        helper_root.join("program.json"),
        r#"{ "program": "helper.aleo", "version": "0.1.0", "description": "", "license": "MIT", "leo": "4.0.0" }"#,
    )
    .expect("write helper manifest");
    let helper_source = concat!(
        "program helper.aleo {\n",
        "    fn double(x: u32) -> u32 {\n",
        "        return x + x;\n",
        "    }\n",
        "}\n",
    );
    let helper_path = helper_src.join("main.leo");
    fs::write(&helper_path, helper_source).expect("write helper source");
    let helper_root = helper_root.canonicalize().expect("canonical helper root");
    let helper_uri = file_uri(&helper_path.canonicalize().expect("canonical helper source"));

    let package_root = tempdir.path().join("example");
    let source_dir = package_root.join("src");
    fs::create_dir_all(&source_dir).expect("create source dir");
    fs::write(
        package_root.join("program.json"),
        json!({
            "program": "demo.aleo",
            "version": "0.1.0",
            "description": "",
            "license": "MIT",
            "leo": "4.0.0",
            "dependencies": [
                {
                    "name": "helper.aleo",
                    "location": "local",
                    "path": helper_root,
                }
            ],
        })
        .to_string(),
    )
    .expect("write manifest");

    let source = concat!(
        "import helper.aleo;\n\n",
        "program demo.aleo {\n",
        "    fn main(x: u32) -> u32 {\n",
        "        return helper.aleo::double(x);\n",
        "    }\n",
        "}\n",
    );
    let main_path = source_dir.join("main.leo");
    fs::write(&main_path, source).expect("write source");
    let document_uri = file_uri(&main_path);

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    server.notify("initialized", json!({}));
    server.notify(
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": document_uri,
                "languageId": "leo",
                "version": 1,
                "text": source,
            }
        }),
    );

    let response = server.request(
        2,
        "textDocument/definition",
        json!({
            "textDocument": {
                "uri": document_uri,
            },
            "position": position_json(source, "double", 0),
        }),
    );

    assert_eq!(
        response["result"][0]["uri"],
        json!(helper_uri.to_string()),
        "bad uri for dependency target: {response}; stderr:\n{}",
        server.stderr_contents()
    );
    assert_eq!(response["result"][0]["range"], range_json(helper_source, "double", 0));

    let shutdown = server.request(3, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);

    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
}

/// Verifies an import name resolves to the imported local program source.
#[test]
fn definition_resolves_imported_program_target() {
    let tempdir = tempdir().expect("tempdir");

    let helper_root = tempdir.path().join("helper");
    let helper_src = helper_root.join("src");
    fs::create_dir_all(&helper_src).expect("create helper source dir");
    fs::write(
        helper_root.join("program.json"),
        r#"{ "program": "helper.aleo", "version": "0.1.0", "description": "", "license": "MIT", "leo": "4.0.0" }"#,
    )
    .expect("write helper manifest");
    let helper_source = concat!(
        "program helper.aleo {\n",
        "    fn double(x: u32) -> u32 {\n",
        "        return x + x;\n",
        "    }\n",
        "}\n",
    );
    let helper_path = helper_src.join("main.leo");
    fs::write(&helper_path, helper_source).expect("write helper source");
    let helper_root = helper_root.canonicalize().expect("canonical helper root");
    let helper_uri = file_uri(&helper_path.canonicalize().expect("canonical helper source"));

    let package_root = tempdir.path().join("example");
    let source_dir = package_root.join("src");
    fs::create_dir_all(&source_dir).expect("create source dir");
    fs::write(
        package_root.join("program.json"),
        json!({
            "program": "demo.aleo",
            "version": "0.1.0",
            "description": "",
            "license": "MIT",
            "leo": "4.0.0",
            "dependencies": [
                {
                    "name": "helper.aleo",
                    "location": "local",
                    "path": helper_root,
                }
            ],
        })
        .to_string(),
    )
    .expect("write manifest");

    let source = concat!(
        "import helper.aleo;\n\n",
        "program demo.aleo {\n",
        "    fn main() -> u32 {\n",
        "        return 1u32;\n",
        "    }\n",
        "}\n",
    );
    let main_path = source_dir.join("main.leo");
    fs::write(&main_path, source).expect("write source");
    let document_uri = file_uri(&main_path);

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    server.notify("initialized", json!({}));
    server.notify(
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": document_uri,
                "languageId": "leo",
                "version": 1,
                "text": source,
            }
        }),
    );

    let response = server.request(
        2,
        "textDocument/definition",
        json!({
            "textDocument": {
                "uri": document_uri,
            },
            "position": position_json(source, "helper", 0),
        }),
    );

    assert_eq!(
        response["result"][0]["uri"],
        json!(helper_uri.to_string()),
        "bad uri for import target: {response}; stderr:\n{}",
        server.stderr_contents()
    );
    assert_eq!(response["result"][0]["range"], range_json(helper_source, "helper", 0));

    let shutdown = server.request(3, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);

    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
}

/// Verifies exit before shutdown returns a nonzero process status.
#[test]
fn exit_without_shutdown_returns_nonzero() {
    let mut server = TestServer::spawn(&[]);
    initialize(&mut server);
    server.notify("initialized", json!({}));

    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();

    assert!(!status.success(), "stderr:\n{stderr}");
}

/// Verifies malformed open/change/close payloads do not kill the server.
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

/// Verifies malformed notifications are contained and logged.
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

/// Verifies semantic tokens return data and reuse the cached snapshot.
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
    assert!(
        server.wait_for_stderr_contains("worker completed latest document", Duration::from_secs(1)),
        "stderr:\n{}",
        server.finish().1
    );
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

/// Verifies worker panics surface as internal semantic-token errors.
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
