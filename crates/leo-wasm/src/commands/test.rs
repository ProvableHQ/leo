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
//! `test_root` points at the test package's own `program.json` directory.
//! The test package's manifest must list the main project as a dependency,
//! the same way `leo test` requires it on disk.
//!
//! Execution goes through [`leo_compiler::run::run_without_ledger`], the same
//! in-memory path the native test framework takes for non-async cases. The
//! native CLI's `leo test` additionally goes through `run_with_ledger` to
//! resolve async/finalize semantics; that path requires the full snarkVM
//! ledger and is unavailable in wasm.

use crate::{
    project,
    wire::{clone_file_source, network_from_manifest},
};

use indexmap::IndexMap;
use leo_ast::{NetworkName, NodeBuilder};
use leo_compiler::run::{Case, Config as RunConfig, EvaluationStatus, Program as RunProgram, run_without_ledger};
use leo_errors::Handler;
use leo_span::{create_session_if_not_set_then, source_map::FileName, with_session_globals};
use serde_json::json;
use std::rc::Rc;

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
            return json!({"success": false, "results": [], "diagnostics": e}).to_string();
        }

        // The same FileSource serves the test package — both root paths live in
        // the same virtual FS the caller supplied.
        let test_proj =
            project::Project { root: std::path::PathBuf::from(test_root), file_source: clone_file_source(files_json) };

        // The test package is the unit being compiled here; read its manifest's
        // `network` field so e.g. a `mainnet` test package isn't silently run
        // under the main project's `testnet` semantics.
        let network = network_from_manifest(&test_proj);
        if network != NetworkName::TestnetV0 {
            // `run_without_ledger` is hardcoded to `TestnetV0`. Reject other
            // manifests up front so compile-vs-run don't silently disagree.
            return json!({
                "success": false,
                "results": [],
                "diagnostics": format!(
                    "leo-wasm `test` only supports `network: \"testnet\"` (manifest says {network})"
                ),
            })
            .to_string();
        }

        // Discover @test functions from the test entry source before passes mutate the AST.
        let test_entry = match test_proj.entry_file() {
            Ok(e) => e,
            Err(e) => return json!({"success": false, "results": [], "diagnostics": e}).to_string(),
        };
        let test_source = match leo_span::file_source::FileSource::read_file(&test_proj.file_source, &test_entry) {
            Ok(s) => s,
            Err(e) => {
                return json!({
                    "success": false,
                    "results": [],
                    "diagnostics": format!("read {}: {e}", test_entry.display())
                })
                .to_string();
            }
        };
        let test_fns = find_test_functions(&test_source, network);
        if test_fns.is_empty() {
            return json!({"success": true, "results": [], "diagnostics": ""}).to_string();
        }

        // Compile the test package — `collect_import_stubs` resolves the main
        // project (and any of its transitive deps) declared in the test
        // package's manifest.
        let compiled = match project::compile_test(&test_proj, network, IndexMap::new()) {
            Ok(c) => c,
            Err(diag) => return json!({"success": false, "results": [], "diagnostics": diag}).to_string(),
        };

        // Stage every emitted program (deps + the test program itself) into
        // `Config::programs`. `run_without_ledger` loads them all into one
        // Process so cross-program calls inside tests resolve.
        let mut programs: Vec<RunProgram> = compiled
            .imports
            .iter()
            .map(|p| RunProgram { bytecode: p.bytecode.clone(), name: p.name.clone() })
            .collect();
        programs.push(RunProgram { bytecode: compiled.primary.bytecode, name: compiled.primary.name.clone() });

        let results = collect_test_results(&test_fns, &programs);
        let all_passed = results.iter().all(|r| r["passed"].as_bool().unwrap_or(false));
        json!({"success": all_passed, "results": results, "diagnostics": ""}).to_string()
    })
}

// ---------------------------------------------------------------------------
// Test-specific helpers
// ---------------------------------------------------------------------------

/// Parse the test Leo source (without running passes) and list every
/// `(program_name, function_name, should_fail)` for `@test`-annotated functions.
fn find_test_functions(test_source: &str, network: NetworkName) -> Vec<(String, String, bool)> {
    use leo_span::sym;
    let (handler, _) = Handler::new_with_buf();
    let node_builder = Rc::new(NodeBuilder::default());
    let source_file =
        with_session_globals(|s| s.source_map.new_source(test_source, FileName::Custom("test.leo".to_string())));
    let Ok(program) = leo_parser::parse_program(handler, &node_builder, &source_file, &[], network) else {
        return Vec::new();
    };

    let mut out = Vec::new();
    for (prog_sym, scope) in &program.program_scopes {
        let prog_name = prog_sym.to_string();
        for (fn_sym, function) in &scope.functions {
            if !function.annotations.iter().any(|a| a.identifier.name == sym::test) {
                continue;
            }
            let should_fail = function.annotations.iter().any(|a| a.identifier.name == sym::should_fail);
            out.push((prog_name.clone(), fn_sym.to_string(), should_fail));
        }
    }
    out
}

/// Execute each `@test` function via `run_without_ledger` and shape the result
/// into the `[{name, passed, error}, …]` JSON the playground expects.
fn collect_test_results(test_fns: &[(String, String, bool)], programs: &[RunProgram]) -> Vec<serde_json::Value> {
    test_fns
        .iter()
        .map(|(prog, fn_name, should_fail)| {
            let qualified = format!("{prog}/{fn_name}");
            let case = Case {
                // `prog` already includes the `.aleo` suffix — it's the
                // symbol form of the `program <name>.aleo` declaration.
                program_name: prog.clone(),
                function: fn_name.clone(),
                private_key: None,
                input: Vec::new(),
                seed_mapping: Vec::new(),
            };

            let outcome = match run_without_ledger(
                &RunConfig {
                    seed: 0,
                    start_height: None,
                    programs: programs.to_vec(),
                    skip_proving: true,
                },
                &[case],
            ) {
                Ok(mut o) => o.pop().expect("one case in, one outcome out"),
                Err(e) => {
                    let error = format!("{e}");
                    return json!({ "name": qualified, "passed": *should_fail, "error": if *should_fail { String::new() } else { error.clone() } });
                }
            };

            let success = matches!(outcome.status, EvaluationStatus::Success);
            let passed = if *should_fail { !success } else { success };
            let error = if !passed {
                if *should_fail {
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
        })
        .collect()
}
