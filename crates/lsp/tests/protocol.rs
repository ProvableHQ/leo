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

//! End-to-end protocol coverage for the `leo-lsp` binary.
//!
//! These tests drive the server through JSON-RPC requests and notifications so
//! lifecycle, malformed-message handling, semantic tokens, definitions, and
//! references are validated through the same transport surface used by editors.

mod common;

use self::common::TestServer;
use lsp_types::Uri;
use serde_json::{Value, json};
use std::{fs, path::Path, time::Duration};
use tempfile::{TempDir, tempdir};

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

/// Write a minimal package around `source` and return its open/canonical URIs.
fn write_test_package(source: &str) -> (TempDir, Uri, Uri) {
    write_test_package_with_manifest(
        source,
        r#"{ "program": "demo.aleo", "version": "0.1.0", "description": "", "license": "MIT", "leo": "4.0.0" }"#,
    )
}

/// Write a test package with a custom manifest and return its open/canonical URIs.
fn write_test_package_with_manifest(source: &str, manifest: &str) -> (TempDir, Uri, Uri) {
    let tempdir = tempdir().expect("tempdir");
    let package_root = tempdir.path().join("example");
    let source_dir = package_root.join("src");
    fs::create_dir_all(&source_dir).expect("create source dir");
    fs::write(package_root.join("program.json"), manifest).expect("write manifest");
    let main_path = source_dir.join("main.leo");
    fs::write(&main_path, source).expect("write source");
    let document_uri = file_uri(&main_path);
    let canonical_uri = file_uri(&main_path.canonicalize().expect("canonical main path"));
    (tempdir, document_uri, canonical_uri)
}

/// Open a Leo document in the test server.
fn open_document(server: &mut TestServer, document_uri: &Uri, source: &str) {
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
}

/// Request references for a substring occurrence.
fn request_references(
    server: &mut TestServer,
    id: i64,
    document_uri: &Uri,
    source: &str,
    needle: &str,
    occurrence: usize,
    include_declaration: bool,
) -> Value {
    server.request(
        id,
        "textDocument/references",
        json!({
            "textDocument": {
                "uri": document_uri,
            },
            "position": position_json(source, needle, occurrence),
            "context": {
                "includeDeclaration": include_declaration,
            }
        }),
    )
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
    assert_eq!(initialize["result"]["capabilities"]["referencesProvider"], json!(true));
    assert_eq!(initialize["result"]["capabilities"]["renameProvider"]["prepareProvider"], json!(true));
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

/// Verifies find-all-references returns local variable uses with LSP null/array semantics.
#[test]
fn references_returns_local_variable_occurrences() {
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
        "        let next: u32 = total + 1u32;\n",
        "        return total + next;\n",
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

    let include_declaration = server.request(
        2,
        "textDocument/references",
        json!({
            "textDocument": {
                "uri": document_uri,
            },
            "position": position_json(source, "total", 1),
            "context": {
                "includeDeclaration": true,
            }
        }),
    );
    assert_eq!(include_declaration["id"], 2);
    let results = include_declaration["result"].as_array().expect("references array");
    assert_eq!(results.len(), 3, "bad references response: {include_declaration}");
    assert!(results.iter().all(|location| location["uri"] == json!(canonical_file_uri.to_string())));
    assert_eq!(results[0]["range"], range_json(source, "total", 0));
    assert_eq!(results[1]["range"], range_json(source, "total", 1));
    assert_eq!(results[2]["range"], range_json(source, "total", 2));

    let references_only = server.request(
        3,
        "textDocument/references",
        json!({
            "textDocument": {
                "uri": document_uri,
            },
            "position": position_json(source, "total", 1),
            "context": {
                "includeDeclaration": false,
            }
        }),
    );
    let results = references_only["result"].as_array().expect("references-only array");
    assert_eq!(results.len(), 2, "bad references-only response: {references_only}");
    assert_eq!(results[0]["range"], range_json(source, "total", 1));
    assert_eq!(results[1]["range"], range_json(source, "total", 2));

    let shutdown = server.request(4, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);

    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
}

/// Verifies references cover compiler-backed parameters, functions, types, and members.
#[test]
fn references_cover_core_compiler_symbol_families() {
    let source = concat!(
        "const LIMIT: u32 = 3u32;\n\n",
        "struct Point {\n",
        "    x_coordinate: u32,\n",
        "}\n\n",
        "fn combine(left_value: u32, right_value: u32) -> u32 {\n",
        "    let sum_value: u32 = left_value + LIMIT;\n",
        "    return sum_value + right_value;\n",
        "}\n\n",
        "fn unused(unused_value: u32) -> u32 {\n",
        "    return 0u32;\n",
        "}\n\n",
        "fn consume(point_value: Point) -> u32 {\n",
        "    let first_value: u32 = point_value.x_coordinate;\n",
        "    return first_value + point_value.x_coordinate;\n",
        "}\n\n",
        "program demo.aleo {\n",
        "    fn main() -> u32 {\n",
        "        let first_value: u32 = combine(1u32, LIMIT);\n",
        "        return first_value + combine(2u32, LIMIT);\n",
        "    }\n",
        "}\n",
    );
    let (_tempdir, document_uri, canonical_uri) = write_test_package(source);

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    server.notify("initialized", json!({}));
    open_document(&mut server, &document_uri, source);

    let function = request_references(&mut server, 2, &document_uri, source, "combine", 1, true);
    assert!(function["result"].is_array(), "bad function response: {function}");
    let function_results = function["result"].as_array().expect("function references");
    assert_eq!(function_results.len(), 3, "bad function references: {function}");
    assert_eq!(function_results[0]["range"], range_json(source, "combine", 0));
    assert_eq!(function_results[1]["range"], range_json(source, "combine", 1));
    assert_eq!(function_results[2]["range"], range_json(source, "combine", 2));

    let parameter = request_references(&mut server, 3, &document_uri, source, "left_value", 1, true);
    let parameter_results = parameter["result"].as_array().expect("parameter references");
    assert_eq!(parameter_results.len(), 2, "bad parameter references: {parameter}");
    assert_eq!(parameter_results[0]["range"], range_json(source, "left_value", 0));
    assert_eq!(parameter_results[1]["range"], range_json(source, "left_value", 1));

    let declaration_only = request_references(&mut server, 4, &document_uri, source, "unused_value", 0, false);
    assert_eq!(declaration_only["result"], json!([]), "declaration-only token should be navigable-empty");

    let type_refs = request_references(&mut server, 5, &document_uri, source, "Point", 1, true);
    let type_results = type_refs["result"].as_array().expect("type references");
    assert_eq!(type_results.len(), 2, "bad type references: {type_refs}");
    assert_eq!(type_results[0]["range"], range_json(source, "Point", 0));
    assert_eq!(type_results[1]["range"], range_json(source, "Point", 1));

    let member = request_references(&mut server, 6, &document_uri, source, "x_coordinate", 1, true);
    let member_results = member["result"].as_array().expect("member references");
    assert_eq!(member_results.len(), 3, "bad member references: {member}");
    assert_eq!(member_results[0]["range"], range_json(source, "x_coordinate", 0));
    assert_eq!(member_results[1]["range"], range_json(source, "x_coordinate", 1));
    assert_eq!(member_results[2]["range"], range_json(source, "x_coordinate", 2));
    assert!(member_results.iter().all(|location| location["uri"] == json!(canonical_uri.to_string())));

    let unknown = request_references(&mut server, 7, &document_uri, source, "u32", 0, true);
    assert_eq!(unknown["result"], Value::Null, "keyword/type primitive should not be a guessed reference target");

    let shutdown = server.request(8, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);
    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
}

/// Verifies syntax fallback keys same-file program types and functions when dependency analysis cannot run.
#[test]
fn references_cover_syntax_fallback_program_type_and_function_occurrences() {
    let source = concat!(
        "import credits.aleo;\n\n",
        "program demo.aleo {\n",
        "    struct TransferAuth {\n",
        "        amount: u64,\n",
        "    }\n",
        "\n",
        "    fn main(first: u64, second: u64) -> u64 {\n",
        "        let one: u64 = auth_digest(first);\n",
        "        return one + auth_digest(second);\n",
        "    }\n",
        "}\n\n",
        "// TransferAuth in comments is not a semantic reference.\n",
        "fn auth_digest(amount: u64) -> u64 {\n",
        "    let auth: TransferAuth = TransferAuth { amount: amount };\n",
        "    return auth.amount;\n",
        "}\n",
    );
    let (_tempdir, document_uri, canonical_uri) = write_test_package_with_manifest(
        source,
        r#"{
  "program": "demo.aleo",
  "version": "0.1.0",
  "description": "",
  "license": "MIT",
  "leo": "4.0.0",
  "dependencies": [{ "name": "credits.aleo", "location": "network", "path": null, "edition": null }]
}"#,
    );

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    server.notify("initialized", json!({}));
    open_document(&mut server, &document_uri, source);

    let response = request_references(&mut server, 2, &document_uri, source, "TransferAuth", 0, true);
    let results = response["result"].as_array().expect("syntax fallback type references");
    assert_eq!(results.len(), 3, "bad syntax fallback references: {response}");
    assert!(results.iter().all(|location| location["uri"] == json!(canonical_uri.to_string())));
    assert_eq!(results[0]["range"], range_json(source, "TransferAuth", 0));
    assert_eq!(results[1]["range"], range_json(source, "TransferAuth", 2));
    assert_eq!(results[2]["range"], range_json(source, "TransferAuth", 3));

    let response = request_references(&mut server, 3, &document_uri, source, "auth_digest", 2, true);
    let results = response["result"].as_array().expect("syntax fallback function references");
    assert_eq!(results.len(), 3, "bad syntax fallback function references: {response}");
    assert_eq!(results[0]["range"], range_json(source, "auth_digest", 0));
    assert_eq!(results[1]["range"], range_json(source, "auth_digest", 1));
    assert_eq!(results[2]["range"], range_json(source, "auth_digest", 2));

    let shutdown = server.request(4, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);
    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
}

/// Verifies references on an unopened document return `null`.
#[test]
fn references_on_unopened_document_returns_null() {
    let source = "program demo.aleo {}\n";
    let (_tempdir, document_uri, _) = write_test_package(source);
    let mut server = TestServer::spawn(&[]);
    initialize(&mut server);
    server.notify("initialized", json!({}));

    let response = request_references(&mut server, 2, &document_uri, source, "demo", 0, true);
    assert_eq!(response["result"], Value::Null);

    let shutdown = server.request(3, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);
    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
}

/// Verifies imported program namespaces return both import-site and dependency-source references.
#[test]
fn references_return_source_dependency_program_namespace_occurrences() {
    let tempdir = tempdir().expect("tempdir");
    let helper_root = tempdir.path().join("helper");
    fs::create_dir_all(helper_root.join("src")).expect("create helper source dir");
    fs::write(
        helper_root.join("program.json"),
        r#"{ "program": "helper.aleo", "version": "0.1.0", "description": "", "license": "MIT", "leo": "4.0.0" }"#,
    )
    .expect("write helper manifest");
    let helper_source = "program helper.aleo {\n    fn double(x: u32) -> u32 { return x + x; }\n}\n";
    let helper_main = helper_root.join("src").join("main.leo");
    fs::write(&helper_main, helper_source).expect("write helper source");
    let helper_root = helper_root.canonicalize().expect("canonical helper root");
    let helper_uri = file_uri(&helper_main.canonicalize().expect("canonical helper source"));

    let root = tempdir.path().join("root");
    fs::create_dir_all(root.join("src")).expect("create root source dir");
    fs::write(
        root.join("program.json"),
        format!(
            r#"{{
  "program": "demo.aleo",
  "version": "0.1.0",
  "description": "",
  "license": "MIT",
  "leo": "4.0.0",
  "dependencies": [{{ "name": "helper.aleo", "location": "local", "path": {} }}]
}}"#,
            serde_json::to_string(&helper_root).expect("helper root json")
        ),
    )
    .expect("write root manifest");
    let source = concat!(
        "import helper.aleo;\n\n",
        "program demo.aleo {\n",
        "    fn main() -> u32 {\n",
        "        return helper.aleo::double(1u32);\n",
        "    }\n",
        "}\n",
    );
    let main_path = root.join("src").join("main.leo");
    fs::write(&main_path, source).expect("write root source");
    let document_uri = file_uri(&main_path);
    let canonical_uri = file_uri(&main_path.canonicalize().expect("canonical root source"));

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    server.notify("initialized", json!({}));
    open_document(&mut server, &document_uri, source);

    let response = request_references(&mut server, 2, &document_uri, source, "helper", 1, true);
    let results = response["result"].as_array().expect("program namespace references");
    assert_eq!(results.len(), 3, "bad namespace references: {response}");
    assert_eq!(results[0]["uri"], json!(canonical_uri.to_string()));
    assert_eq!(results[0]["range"], range_json(source, "helper", 0));
    assert_eq!(results[1]["uri"], json!(canonical_uri.to_string()));
    assert_eq!(results[1]["range"], range_json(source, "helper", 1));
    assert_eq!(results[2]["uri"], json!(helper_uri.to_string()));
    assert_eq!(results[2]["range"], range_json(helper_source, "helper", 0));

    let shutdown = server.request(3, "shutdown", Value::Null);
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

/// Send a rename request and return the raw JSON response.
fn request_rename(
    server: &mut TestServer,
    id: i64,
    document_uri: &Uri,
    source: &str,
    needle: &str,
    occurrence: usize,
    new_name: &str,
) -> Value {
    server.request(
        id,
        "textDocument/rename",
        json!({
            "textDocument": { "uri": document_uri },
            "position": position_json(source, needle, occurrence),
            "newName": new_name,
        }),
    )
}

/// Send a prepareRename request and return the raw JSON response.
fn request_prepare_rename(
    server: &mut TestServer,
    id: i64,
    document_uri: &Uri,
    source: &str,
    needle: &str,
    occurrence: usize,
) -> Value {
    server.request(
        id,
        "textDocument/prepareRename",
        json!({
            "textDocument": { "uri": document_uri },
            "position": position_json(source, needle, occurrence),
        }),
    )
}

/// Verifies prepareRename returns the cursor occurrence range for a renameable local.
#[test]
fn prepare_rename_returns_local_range() {
    let source = concat!(
        "program demo.aleo {\n",
        "    fn main() -> u32 {\n",
        "        let total: u32 = 1u32;\n",
        "        let next: u32 = total + 1u32;\n",
        "        return total + next;\n",
        "    }\n",
        "}\n",
    );
    let (_tempdir, document_uri, _canonical) = write_test_package(source);

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    server.notify("initialized", json!({}));
    open_document(&mut server, &document_uri, source);

    let response = request_prepare_rename(&mut server, 2, &document_uri, source, "total", 1);
    // PrepareRenameResponse::Range serializes as the raw range payload.
    assert_eq!(response["result"], range_json(source, "total", 1), "bad prepareRename: {response}");

    let null = request_prepare_rename(&mut server, 3, &document_uri, source, "u32", 0);
    assert_eq!(null["result"], Value::Null, "primitive type should not be renameable: {null}");

    let shutdown = server.request(4, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);
    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
}

/// Verifies rename returns a versioned WorkspaceEdit covering every local occurrence.
#[test]
fn rename_returns_workspace_edit_for_local() {
    let source = concat!(
        "program demo.aleo {\n",
        "    fn main() -> u32 {\n",
        "        let total: u32 = 1u32;\n",
        "        let next: u32 = total + 1u32;\n",
        "        return total + next;\n",
        "    }\n",
        "}\n",
    );
    let (_tempdir, document_uri, canonical_uri) = write_test_package(source);

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    server.notify("initialized", json!({}));
    open_document(&mut server, &document_uri, source);

    let response = request_rename(&mut server, 2, &document_uri, source, "total", 1, "subtotal");
    let edits = response["result"]["documentChanges"].as_array().expect("documentChanges array");
    assert_eq!(edits.len(), 1, "single-file rename: {response}");
    let document_edit = &edits[0];
    assert_eq!(document_edit["textDocument"]["uri"], json!(canonical_uri.to_string()));
    // OptionalVersionedTextDocumentIdentifier carries the open-buffer version
    // when the analyzed fingerprint is `OpenBuffer`, or `null` when the file
    // was read from disk. Both are LSP-correct; the client uses the version
    // to refuse stale-buffer applies when present.
    let version = &document_edit["textDocument"]["version"];
    assert!(version.is_null() || version == &json!(1), "bad version: {version}");
    let text_edits = document_edit["edits"].as_array().expect("text edits");
    assert_eq!(text_edits.len(), 3, "three occurrences: {response}");
    for edit in text_edits {
        assert_eq!(edit["newText"], json!("subtotal"));
    }
    assert_eq!(text_edits[0]["range"], range_json(source, "total", 0));
    assert_eq!(text_edits[1]["range"], range_json(source, "total", 1));
    assert_eq!(text_edits[2]["range"], range_json(source, "total", 2));

    let shutdown = server.request(3, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);
    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
}

/// Verifies rename rejects keyword new-names with `RequestFailed` (LSP -32803).
#[test]
fn rename_rejects_keyword_new_name() {
    let source = concat!(
        "program demo.aleo {\n",
        "    fn main() -> u32 {\n",
        "        let total: u32 = 1u32;\n",
        "        return total;\n",
        "    }\n",
        "}\n",
    );
    let (_tempdir, document_uri, _canonical) = write_test_package(source);

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    server.notify("initialized", json!({}));
    open_document(&mut server, &document_uri, source);

    let response = request_rename(&mut server, 2, &document_uri, source, "total", 1, "let");
    assert_eq!(response["error"]["code"], json!(-32803), "{response}");
    assert!(response["error"]["message"].as_str().expect("rename keyword error").contains("keyword"), "{response}");

    let invalid = request_rename(&mut server, 3, &document_uri, source, "total", 1, "1abc");
    assert_eq!(invalid["error"]["code"], json!(-32803), "{invalid}");
    let empty = request_rename(&mut server, 4, &document_uri, source, "total", 1, "");
    assert_eq!(empty["error"]["code"], json!(-32803), "{empty}");

    let shutdown = server.request(5, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);
    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
}

/// Verifies rename returns null when the cursor is on a non-renameable position.
#[test]
fn rename_returns_null_on_non_renameable_cursor() {
    let source = concat!(
        "program demo.aleo {\n",
        "    fn main() -> u32 {\n",
        "        let total: u32 = 1u32;\n",
        "        return total;\n",
        "    }\n",
        "}\n",
    );
    let (_tempdir, document_uri, _canonical) = write_test_package(source);

    let mut server = TestServer::spawn(&[("RUST_LOG", "debug")]);
    initialize(&mut server);
    server.notify("initialized", json!({}));
    open_document(&mut server, &document_uri, source);

    // Cursor on the program identifier `demo` - Program identities are out of PR 5 scope.
    let on_program = request_rename(&mut server, 2, &document_uri, source, "demo", 0, "renamed");
    assert_eq!(on_program["result"], Value::Null, "{on_program}");

    // Cursor on the `u32` keyword/type primitive - no occurrence.
    let on_keyword = request_rename(&mut server, 3, &document_uri, source, "u32", 0, "renamed");
    assert_eq!(on_keyword["result"], Value::Null, "{on_keyword}");

    let shutdown = server.request(4, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);
    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
}

/// Verifies rename and prepareRename return null on unopened documents.
#[test]
fn rename_returns_null_for_unopened_document() {
    let mut server = TestServer::spawn(&[]);
    initialize(&mut server);
    server.notify("initialized", json!({}));

    let uri: Uri = "file:///does/not/exist.leo".parse().expect("uri");
    let prepare = server.request(
        2,
        "textDocument/prepareRename",
        json!({
            "textDocument": { "uri": uri },
            "position": { "line": 0, "character": 0 },
        }),
    );
    assert_eq!(prepare["result"], Value::Null, "{prepare}");

    let rename = server.request(
        3,
        "textDocument/rename",
        json!({
            "textDocument": { "uri": uri },
            "position": { "line": 0, "character": 0 },
            "newName": "renamed",
        }),
    );
    assert_eq!(rename["result"], Value::Null, "{rename}");

    let shutdown = server.request(4, "shutdown", Value::Null);
    assert_eq!(shutdown["result"], Value::Null);
    server.notify("exit", json!({}));
    let (status, stderr) = server.finish();
    assert!(status.success(), "stderr:\n{stderr}");
}
