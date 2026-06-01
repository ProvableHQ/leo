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

//! `leo run` — compile a project and execute one function.
//!
//! Thin wrapper: project load + compile + `project::run_function`. The shared
//! execution helpers live in [`leo_cli_core::project`]; this file just shapes the
//! `{ success, output, finalize, diagnostics }` JSON the playground expects.

use crate::wire::{EnvOptions, error_json};
use leo_cli_core::project;

use leo_ast::NetworkName;
use leo_compiler::run::EvaluationStatus;
use leo_span::create_session_if_not_set_then;
use serde_json::json;

/// Compile a project and run one function.
///
/// `inputs_json` is a JSON array of Leo-typed input strings, e.g. `["1u32", "2u32"]`.
/// `env_json` is the JSON shape of [`EnvOptions`] (network, endpoint, private
/// key, …); mirror's the CLI's flags.
///
/// Returns JSON: `{ success, output, finalize, diagnostics }`.
pub fn run_impl(files_json: &str, root: &str, function_name: &str, inputs_json: &str, env_json: &str) -> String {
    let env = match EnvOptions::from_json(env_json) {
        Ok(e) => e,
        Err(e) => return error_json(&e, &["output"]),
    };
    let inputs: Vec<String> = match serde_json::from_str(inputs_json) {
        Ok(v) => v,
        Err(e) => return error_json(&format!("invalid inputs JSON: {e}"), &["output"]),
    };
    create_session_if_not_set_then(|_| {
        let proj = match project::Project::from_files_json(files_json, root) {
            Ok(p) => p,
            Err(e) => return error_json(&e, &["output"]),
        };
        let network = env.resolved_network();
        if network != NetworkName::TestnetV0 {
            return error_json(&format!("leo-wasm `run` only supports `network: \"testnet\"` (got {network})"), &[
                "output",
            ]);
        }
        let compiled = match project::compile(&proj, /* is_test */ false, network) {
            Ok(c) => c,
            Err(diag) => return error_json(&diag, &["output"]),
        };

        let programs = project::stage_programs(&compiled);
        let outcome = match project::run_function(programs, &compiled.primary.name, function_name, inputs) {
            Ok(o) => o,
            Err(e) => return error_json(&e, &["output"]),
        };
        if let EvaluationStatus::Failed(e) = &outcome.status {
            return error_json(e, &["output"]);
        }
        shape_run_outcome(&outcome)
    })
}

/// Shape a successful [`EvaluationOutcome`] into the playground's run-result JSON.
fn shape_run_outcome(outcome: &leo_compiler::run::EvaluationOutcome) -> String {
    // `()` (unit) collapses to empty so the playground falls through to its
    // `(no output)` placeholder.
    let raw = outcome.output().to_string();
    let output = if raw == "()" { String::new() } else { raw };

    let finalize = outcome.finalize.as_ref().map(|f| {
        let mapping_obj: serde_json::Map<String, serde_json::Value> = f
            .mappings
            .iter()
            .map(|(name, pairs)| {
                let entries: Vec<serde_json::Value> =
                    pairs.iter().map(|(k, v)| serde_json::Value::String(format!("{k} => {v}"))).collect();
                (name.clone(), serde_json::Value::Array(entries))
            })
            .collect();
        serde_json::Value::Object(mapping_obj)
    });

    json!({
        "success": true,
        "output": output,
        "finalize": finalize,
        "diagnostics": "",
    })
    .to_string()
}
