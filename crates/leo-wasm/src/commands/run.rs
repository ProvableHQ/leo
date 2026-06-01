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
//! Reuses [`leo_compiler::run::run_without_ledger`], the same in-memory
//! execution path the native test framework uses for non-async cases.

use crate::{
    project,
    wire::{error_json, network_from_manifest},
};

use leo_ast::NetworkName;
use leo_compiler::run::{Case, Config as RunConfig, EvaluationStatus, Program as RunProgram, run_without_ledger};
use leo_span::create_session_if_not_set_then;
use serde_json::json;

/// Compile a project and run one function.
///
/// `inputs_json` is a JSON array of Leo-typed input strings, e.g. `["1u32", "2u32"]`.
///
/// Returns JSON: `{ success, output, finalize, diagnostics }`.
pub fn run_impl(files_json: &str, root: &str, function_name: &str, inputs_json: &str) -> String {
    let inputs: Vec<String> = match serde_json::from_str(inputs_json) {
        Ok(v) => v,
        Err(e) => return error_json(&format!("invalid inputs JSON: {e}"), &["output"]),
    };
    create_session_if_not_set_then(|_| {
        let proj = match project::Project::from_files_json(files_json, root) {
            Ok(p) => p,
            Err(e) => return error_json(&e, &["output"]),
        };
        let network = network_from_manifest(&proj);
        if network != NetworkName::TestnetV0 {
            // `run_without_ledger` is hardcoded to `TestnetV0`. Reject other
            // manifests up front so compile-vs-run don't silently disagree.
            return error_json(
                &format!("leo-wasm `run` only supports `network: \"testnet\"` (manifest says {network})"),
                &["output"],
            );
        }
        let compiled = match project::compile(&proj, false, network) {
            Ok(c) => c,
            Err(diag) => return error_json(&diag, &["output"]),
        };

        // Stage every emitted program (deps first, then the primary) so cross-program
        // calls resolve. `run_without_ledger` loads them all into the same Process.
        let mut programs: Vec<RunProgram> = compiled
            .imports
            .iter()
            .map(|p| RunProgram { bytecode: p.bytecode.clone(), name: p.name.clone() })
            .collect();
        programs.push(RunProgram { bytecode: compiled.primary.bytecode, name: compiled.primary.name.clone() });

        let case = Case {
            // `compiled.primary.name` is the full program ID (e.g. `hello.aleo`).
            program_name: compiled.primary.name.clone(),
            function: function_name.to_string(),
            private_key: None,
            input: inputs,
            seed_mapping: Vec::new(),
        };

        let outcomes =
            match run_without_ledger(&RunConfig { seed: 0, start_height: None, programs, skip_proving: true }, &[case])
            {
                Ok(o) => o,
                Err(e) => return error_json(&format!("{e}"), &["output"]),
            };
        let outcome = outcomes.into_iter().next().expect("one case in, one outcome out");

        if let EvaluationStatus::Failed(e) = &outcome.status {
            return error_json(e, &["output"]);
        }

        // Surface the output as a string; `()` (unit) collapses to empty so
        // the playground falls through to its `(no output)` placeholder.
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
    })
}
