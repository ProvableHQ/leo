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
//! - **Manifest plumbing** — `network_from_manifest` pulls the network out
//!   of a project's `program.json`.
//! - **JSON shape** — `error_json` / `import_summaries` produce the
//!   `{ success, output, abi, diagnostics }`-style strings the
//!   `wasm_bindings` shim returns verbatim.
//! - **File-map plumbing** — `clone_file_source` rebuilds an
//!   `InMemoryFileSource` from the original JSON blob (wasm-only).

use crate::project;

use leo_ast::NetworkName;
use serde_json::json;

#[cfg(target_arch = "wasm32")]
use indexmap::IndexMap;

// ---------------------------------------------------------------------------
// Manifest plumbing
// ---------------------------------------------------------------------------

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
    match v.get("network").and_then(|n| n.as_str()) {
        Some("mainnet") => NetworkName::MainnetV0,
        Some("canary") => NetworkName::CanaryV0,
        _ => NetworkName::TestnetV0,
    }
}

// ---------------------------------------------------------------------------
// JSON shape
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// File-map plumbing
// ---------------------------------------------------------------------------

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
