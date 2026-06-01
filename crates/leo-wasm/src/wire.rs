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
//! - **Env options** — [`EnvOptions`] parses the per-call JSON blob the JS
//!   side passes (network, endpoint, private key, etc.), mirroring the
//!   native CLI's `--network`/`--endpoint`/`--private-key` flags.
//! - **JSON shape** — `error_json` / `import_summaries` produce the
//!   `{ success, output, abi, diagnostics }`-style strings the
//!   `wasm_bindings` shim returns verbatim.

use leo_ast::NetworkName;
use serde::Deserialize;
use serde_json::json;

// ---------------------------------------------------------------------------
// Env options
// ---------------------------------------------------------------------------

/// Per-call environment overrides — mirrors the native CLI's [`EnvOptions`]
/// (`crates/leo/src/cli/commands/common/options.rs`). Passed as a JSON blob
/// at the wasm boundary so the JS side can populate the same fields the CLI
/// reads from flags or `.env`.
///
/// Every field is optional; an empty `""` / `"{}"` blob yields all defaults.
#[derive(Default, Debug, Clone, Deserialize)]
#[serde(default)]
pub struct EnvOptions {
    pub network: Option<NetworkName>,
    pub endpoint: Option<String>,
    pub private_key: Option<String>,
    pub devnet: bool,
    pub consensus_heights: Option<Vec<u32>>,
    #[serde(default = "default_network_retries")]
    pub network_retries: u32,
}

fn default_network_retries() -> u32 {
    2
}

impl EnvOptions {
    /// Parse from the JSON blob the JS caller passes. An empty / whitespace
    /// blob yields `Self::default()` so callers can pass `""` when they have
    /// no overrides.
    pub fn from_json(env_json: &str) -> Result<Self, String> {
        if env_json.trim().is_empty() {
            return Ok(Self::default());
        }
        serde_json::from_str(env_json).map_err(|e| format!("invalid env JSON: {e}"))
    }

    /// Resolved network, defaulting to `TestnetV0` (matches the CLI default).
    pub fn network(&self) -> NetworkName {
        self.network.unwrap_or(NetworkName::TestnetV0)
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
