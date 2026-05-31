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

//! Shared plumbing for every `commands::*` entry point.
//!
//! Splits into three roughly independent concerns:
//!
//! * **Compile helpers** — `compile_session` / `compile_with` wrap
//!   `Compiler::new` + `Compiler::compile` so each `commands::*` doesn't have
//!   to reconstruct the boilerplate.
//! * **Manifest plumbing** — `parse_program_json` and `network_from_*` pull
//!   the program name / network out of a `program.json` blob.
//! * **JSON shape** — `error_json`, `import_summaries`, `diagnostics_from`
//!   produce the `{ success, output, abi, diagnostics }`-style strings the
//!   `wasm_bindings` shim returns verbatim.

use crate::project;

use indexmap::IndexMap;
use leo_ast::{NetworkName, NodeBuilder, Stub};
use leo_compiler::{Compiled, Compiler};
use leo_errors::{BufferEmitter, Handler};
use leo_span::{Symbol, source_map::FileName};
use serde_json::json;
use std::{path::PathBuf, rc::Rc};

// ---------------------------------------------------------------------------
// Compile helpers
// ---------------------------------------------------------------------------

/// Compile a single Leo source string with a fresh handler + node builder.
pub fn compile_session(
    source: &str,
    expected_name: Option<String>,
    network: NetworkName,
    is_test: bool,
    import_stubs: IndexMap<Symbol, Stub>,
) -> Result<Compiled, String> {
    let (handler, buf) = Handler::new_with_buf();
    let node_builder = Rc::new(NodeBuilder::default());
    compile_with(handler, &buf, node_builder, source, expected_name, network, is_test, import_stubs)
}

/// Compile a Leo source string using the supplied handler + node builder.
///
/// Sharing the [`NodeBuilder`] across multiple parses keeps `NodeID`s coherent
/// so stubs parsed up front (e.g. the main program when compiling tests) line
/// up with the symbol-table scopes built by the passes.
#[allow(clippy::too_many_arguments)]
pub fn compile_with(
    handler: Handler,
    buf: &BufferEmitter,
    node_builder: Rc<NodeBuilder>,
    source: &str,
    expected_name: Option<String>,
    network: NetworkName,
    is_test: bool,
    import_stubs: IndexMap<Symbol, Stub>,
) -> Result<Compiled, String> {
    let mut compiler = Compiler::new(
        expected_name,
        is_test,
        handler,
        node_builder,
        PathBuf::new(), // unused: write_ast is a no-op on wasm32 and unset on native (snapshots opt-in)
        None,
        import_stubs,
        network,
    );
    compiler.compile(source, FileName::Custom("main.leo".to_string()), &Vec::new()).map_err(|_| diagnostics_from(buf))
}

// ---------------------------------------------------------------------------
// Manifest plumbing
// ---------------------------------------------------------------------------

/// Extract `(program_name, network)` from a `program.json` blob. Falls back to
/// `("main.aleo", TestnetV0)` when the JSON is missing or malformed.
pub fn parse_program_json(program_json: &str) -> (Option<String>, NetworkName) {
    let v: serde_json::Value = serde_json::from_str(program_json).unwrap_or_default();
    let name = v.get("program").and_then(|p| p.as_str()).unwrap_or("main.aleo").to_string();
    (Some(name), network_from_value(&v))
}

/// Extract `network` from the project's `program.json`. Defaults to
/// `TestnetV0` when the manifest is missing or doesn't specify a network.
pub fn network_from_manifest(project: &project::Project) -> NetworkName {
    use leo_span::file_source::FileSource;
    let Ok(raw) = project.file_source.read_file(&project.manifest_path()) else {
        return NetworkName::TestnetV0;
    };
    let Ok(v) = serde_json::from_str::<serde_json::Value>(&raw) else {
        return NetworkName::TestnetV0;
    };
    network_from_value(&v)
}

/// Pull the network field out of a parsed `program.json` value, defaulting to
/// `TestnetV0`.
fn network_from_value(v: &serde_json::Value) -> NetworkName {
    match v.get("network").and_then(|n| n.as_str()) {
        Some("mainnet") => NetworkName::MainnetV0,
        Some("canary") => NetworkName::CanaryV0,
        _ => NetworkName::TestnetV0,
    }
}

// ---------------------------------------------------------------------------
// JSON shape
// ---------------------------------------------------------------------------

/// Render buffered diagnostics as a newline-joined string.
pub fn diagnostics_from(buf: &BufferEmitter) -> String {
    buf.extract_errs().into_inner().iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n")
}

/// Build a JSON error response with `success: false`, `diagnostics: <msg>`, and
/// empty placeholders for the named result fields.
pub fn error_json(msg: &str, empty_fields: &[&str]) -> String {
    let mut obj = serde_json::Map::new();
    obj.insert("success".into(), serde_json::Value::Bool(false));
    for field in empty_fields {
        obj.insert((*field).into(), serde_json::Value::String(String::new()));
    }
    obj.insert("diagnostics".into(), serde_json::Value::String(msg.to_string()));
    serde_json::Value::Object(obj).to_string()
}

/// `Compiler::compile` returns one `CompiledProgram` per emitted import; project
/// API callers want a compact view they can iterate in JS.
pub fn import_summaries(imports: &[leo_compiler::CompiledProgram]) -> Vec<serde_json::Value> {
    imports
        .iter()
        .map(|p| {
            json!({
                "name": p.name,
                "bytecode": p.bytecode,
                "abi": serde_json::to_string_pretty(&p.abi).unwrap_or_default(),
            })
        })
        .collect()
}

/// Reconstruct a fresh `InMemoryFileSource` from `files_json`. `InMemoryFileSource`
/// doesn't expose `Clone`; the cheapest way to share the same file map between
/// the main project and the test project is to deserialize twice.
#[cfg(target_arch = "wasm32")]
pub fn clone_file_source(files_json: &str) -> leo_span::file_source::InMemoryFileSource {
    use leo_span::file_source::InMemoryFileSource;
    let mut out = InMemoryFileSource::new();
    if let Ok(map) = serde_json::from_str::<IndexMap<String, String>>(files_json) {
        for (path, contents) in map {
            out.set(std::path::PathBuf::from(path), contents);
        }
    }
    out
}
