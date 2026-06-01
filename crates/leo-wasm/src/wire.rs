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

//! JSON-shaping helpers shared across every `commands::*` entry point.
//!
//! The `EnvOptions` / `BuildOptions` structs are re-exported here from
//! [`leo_cli_core::options`] so callers can stay in `crate::wire::*` while the
//! actual definitions stay co-located with the native CLI's `clap` parser.

use serde_json::json;

// Re-export the shared option structs that came from the CLI. Both targets
// see the same struct shape — the CLI parses them from `clap` flags, the
// wasm bindings parse them from a JSON blob via `EnvOptions::from_json`.
pub use leo_cli_core::options::{BuildOptions, DEFAULT_ENDPOINT, EnvOptions};

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
