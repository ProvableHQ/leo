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

//! `leo test` — compile a project together with a test package and run every
//! `@test` function.
//!
//! Thin wrapper: project load + `project::find_test_functions` +
//! `project::compile_test` + `project::run_function`. The shared execution
//! helpers live in [`crate::project`]; this file just shapes the
//! `{ success, results, diagnostics }` JSON the playground expects.

use crate::{
    project,
    wire::{clone_file_source, network_from_manifest},
};

use indexmap::IndexMap;
use leo_ast::NetworkName;
use leo_compiler::run::EvaluationStatus;
use leo_span::create_session_if_not_set_then;
use serde_json::json;

/// Compile a project + its test package and run every `@test` function.
///
/// Returns JSON: `{ success, results: [ { name, passed, error } ], diagnostics }`.
pub fn test_impl(files_json: &str, root: &str, test_root: &str) -> String {
    create_session_if_not_set_then(|_| {
        // Validate `root` resolves to a real project in the supplied file map.
        // The main project itself isn't compiled here — `compile_test` resolves
        // it through the test package's manifest dependency — but we want a
        // clear error before any test work if the file map is wrong.
        if let Err(e) = project::Project::from_files_json(files_json, root) {
            return test_error(&e);
        }

        let test_proj =
            project::Project { root: std::path::PathBuf::from(test_root), file_source: clone_file_source(files_json) };

        // The test package is the unit being compiled here; read its manifest
        // network so e.g. a `mainnet` test isn't silently run under the main
        // project's `testnet` semantics.
        let network = network_from_manifest(&test_proj);
        if network != NetworkName::TestnetV0 {
            return test_error(&format!(
                "leo-wasm `test` only supports `network: \"testnet\"` (manifest says {network})"
            ));
        }

        let test_fns = match project::find_test_functions(&test_proj, network) {
            Ok(t) => t,
            Err(e) => return test_error(&e),
        };
        if test_fns.is_empty() {
            return json!({"success": true, "results": [], "diagnostics": ""}).to_string();
        }

        let compiled = match project::compile_test(&test_proj, network, IndexMap::new()) {
            Ok(c) => c,
            Err(diag) => return test_error(&diag),
        };
        let programs = project::stage_programs(&compiled);

        let results: Vec<serde_json::Value> = test_fns.iter().map(|tf| run_one_test(tf, programs.clone())).collect();
        let all_passed = results.iter().all(|r| r["passed"].as_bool().unwrap_or(false));
        json!({"success": all_passed, "results": results, "diagnostics": ""}).to_string()
    })
}

/// Run one `@test` function and shape its result into the playground's
/// `{ name, passed, error }` schema.
fn run_one_test(tf: &project::TestFn, programs: Vec<leo_compiler::run::Program>) -> serde_json::Value {
    let qualified = format!("{}/{}", tf.program, tf.function);
    // `tf.program` already includes the `.aleo` suffix (it's the symbol form
    // of the `program <name>.aleo` declaration).
    let outcome = match project::run_function(programs, &tf.program, &tf.function, Vec::new()) {
        Ok(o) => o,
        Err(e) => {
            // Engine-level failure: surface as a per-test error unless the test
            // was already expected to fail.
            return json!({
                "name": qualified,
                "passed": tf.should_fail,
                "error": if tf.should_fail { String::new() } else { e },
            });
        }
    };

    let success = matches!(outcome.status, EvaluationStatus::Success);
    let passed = if tf.should_fail { !success } else { success };
    let error = if !passed {
        if tf.should_fail {
            "Test was expected to fail but succeeded.".to_string()
        } else if let EvaluationStatus::Failed(msg) = &outcome.status {
            msg.clone()
        } else {
            "evaluation failed".to_string()
        }
    } else {
        String::new()
    };

    json!({ "name": qualified, "passed": passed, "error": error })
}

/// Error response with the `test` command's shape.
fn test_error(msg: &str) -> String {
    json!({"success": false, "results": [], "diagnostics": msg}).to_string()
}
