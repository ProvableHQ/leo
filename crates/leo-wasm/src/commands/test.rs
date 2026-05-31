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

//! `leo test` — compile and run every `@test` function.
//!
//! Wasm-only: uses [`crate::evaluate::run_function`] for execution.
//!
//! Two flavours:
//! - [`run_tests_impl`] — main source + test source as separate strings. The
//!   simpler shape used by the playground today.
//! - [`test_project_impl`] — main package + test package, both addressable in
//!   the same virtual filesystem. Mirrors `leo test` on a real project tree.

use crate::{
    evaluate,
    project,
    wire::{clone_file_source, compile_with, diagnostics_from, network_from_manifest, parse_program_json},
};

use indexmap::IndexMap;
use leo_ast::{NetworkName, NodeBuilder, Stub};
use leo_errors::Handler;
use leo_span::{Symbol, create_session_if_not_set_then, source_map::FileName, with_session_globals};
use serde_json::json;
use std::rc::Rc;

/// Compile main + test source together and run every `@test` fn.
///
/// Returns JSON: `{ success, results: [ { name, passed, error } ], diagnostics }`.
pub fn run_tests_impl(main_source: &str, test_source: &str, program_json: &str) -> String {
    let (_, network) = parse_program_json(program_json);

    create_session_if_not_set_then(|_| {
        let test_fns = find_test_functions(test_source, network);
        if test_fns.is_empty() {
            return json!({"success": true, "results": [], "diagnostics": ""}).to_string();
        }

        // One shared Handler + NodeBuilder so the main stub (Stub::FromLeo) and
        // the test source share NodeIDs — matching how `leo build` constructs
        // stubs via `parse_leo_source_directory`. Otherwise PathResolution can't
        // resolve cross-program references like `counter.aleo::count`.
        let (handler, buf) = Handler::new_with_buf();
        let node_builder = Rc::new(NodeBuilder::default());

        let main_stub = match parse_main_as_stub(main_source, &handler, &node_builder, network) {
            Ok(s) => s,
            Err(()) => {
                return json!({"success": false, "results": [], "diagnostics": diagnostics_from(&buf)}).to_string();
            }
        };

        let mut stubs = IndexMap::with_capacity(1);
        stubs.insert(main_stub.0, main_stub.1);

        let compiled =
            match compile_with(handler, &buf, Rc::clone(&node_builder), test_source, None, network, true, stubs) {
                Ok(c) => c,
                Err(diag) => {
                    return json!({"success": false, "results": [], "diagnostics": diag}).to_string();
                }
            };

        // Codegen emits bytecode for the primary (test) and for every imported
        // FromLeo stub (the main program) in one pass — no second compile needed.
        let test_bytecode = compiled.primary.bytecode;
        let main_bytecode = compiled.imports.first().map(|p| p.bytecode.clone()).unwrap_or_default();

        let results = collect_test_results(&test_fns, &test_bytecode, std::slice::from_ref(&main_bytecode));
        let all_passed = results.iter().all(|r| r["passed"].as_bool().unwrap_or(false));
        json!({"success": all_passed, "results": results, "diagnostics": ""}).to_string()
    })
}

/// Compile a project together with a test package and run every `@test` fn.
///
/// `test_root` points at the test package's root (its own `program.json`).
pub fn test_project_impl(files_json: &str, root: &str, test_root: &str) -> String {
    create_session_if_not_set_then(|_| {
        let proj = match project::Project::from_files_json(files_json, root) {
            Ok(p) => p,
            Err(e) => return json!({"success": false, "results": [], "diagnostics": e}).to_string(),
        };
        let network = network_from_manifest(&proj);

        // The same FileSource serves the test package — both root paths live in
        // the same virtual FS the caller supplied.
        let test_proj =
            project::Project { root: std::path::PathBuf::from(test_root), file_source: clone_file_source(files_json) };

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

        let test_bytecode = compiled.primary.bytecode;
        let dep_bytecodes: Vec<String> = compiled.imports.iter().map(|p| p.bytecode.clone()).collect();

        let results = collect_test_results(&test_fns, &test_bytecode, &dep_bytecodes);
        let all_passed = results.iter().all(|r| r["passed"].as_bool().unwrap_or(false));
        json!({"success": all_passed, "results": results, "diagnostics": ""}).to_string()
    })
}

// ---------------------------------------------------------------------------
// Test-specific helpers
// ---------------------------------------------------------------------------

/// Parse `main_source` once so it can be injected as a `Stub::FromLeo` when
/// the test program is compiled with the *same* node builder.
fn parse_main_as_stub(
    main_source: &str,
    handler: &Handler,
    node_builder: &Rc<NodeBuilder>,
    network: NetworkName,
) -> Result<(Symbol, Stub), ()> {
    let source_file =
        with_session_globals(|s| s.source_map.new_source(main_source, FileName::Custom("main.leo".to_string())));
    let program =
        leo_parser::parse_program(handler.clone(), node_builder, &source_file, &[], network).map_err(|_| ())?;
    if handler.had_errors() {
        return Err(());
    }
    let key = program.program_scopes.first().map(|(k, _)| *k).ok_or(())?;
    Ok((key, Stub::from(program)))
}

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

/// Execute each `@test` function against the compiled test bytecode and turn
/// the result into the playground's `[{name, passed, error}, …]` JSON shape.
fn collect_test_results(
    test_fns: &[(String, String, bool)],
    test_bytecode: &str,
    dep_bytecodes: &[String],
) -> Vec<serde_json::Value> {
    test_fns
        .iter()
        .map(|(prog, fn_name, should_fail)| {
            let qualified = format!("{prog}/{fn_name}");
            let exec = evaluate::run_function(test_bytecode, fn_name, &[], dep_bytecodes);
            let v: serde_json::Value = serde_json::from_str(&exec).unwrap_or_default();
            let success = v["success"].as_bool().unwrap_or(false);

            let passed = if *should_fail { !success } else { success };
            let error = if !passed {
                if *should_fail {
                    "Test was expected to fail but succeeded.".to_string()
                } else {
                    v["diagnostics"].as_str().unwrap_or("evaluation failed").to_string()
                }
            } else {
                String::new()
            };

            json!({ "name": qualified, "passed": passed, "error": error })
        })
        .collect()
}
