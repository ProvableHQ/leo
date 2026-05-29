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

#![forbid(unsafe_code)]
#![cfg_attr(not(any(test, target_arch = "wasm32")), allow(dead_code))]

//! WASM bindings for the Leo compiler.
//!
//! The crate keeps the **implementation** target-neutral so native unit tests
//! exercise the same parsing / compilation / project-walk paths the WASM shim
//! re-exports. The thin `#[wasm_bindgen]` wrappers live in [`wasm_bindings`]
//! and are compiled only when targeting `wasm32`.
//!
//! Build with:
//!   `wasm-pack build crates/leo-wasm --target web --out-dir ../../leo-playground/wasm`

use indexmap::IndexMap;
use leo_ast::{NetworkName, NodeBuilder, Stub};
use leo_compiler::{Compiled, Compiler};
use leo_errors::{BufferEmitter, Handler};
#[cfg(target_arch = "wasm32")]
use leo_span::with_session_globals;
use leo_span::{Symbol, create_session_if_not_set_then, source_map::FileName};
use serde_json::json;
use std::{path::PathBuf, rc::Rc};

// Project walker (manifest parsing + transitive dep resolution against a
// virtual FS). Target-neutral.
mod project;

// snarkVM execution glue (`Process::load_web`, `FinalizeMemory`, …) — only
// builds under the wasm-compatible snarkVM subset crates.
#[cfg(target_arch = "wasm32")]
mod evaluate;

// ---------------------------------------------------------------------------
// Target-neutral impls
// ---------------------------------------------------------------------------
//
// Each `*_impl` returns the same JSON string the wasm-bindgen shim returns.
// Native callers (tests, embedders) can call these directly without going
// through `wasm-bindgen`.

/// Compile a single Leo source file to Aleo bytecode + ABI.
///
/// Returns JSON: `{ success, output, abi, diagnostics }`.
pub fn compile_impl(source: &str, program_json: &str) -> String {
    let (expected_name, network) = parse_program_json(program_json);
    create_session_if_not_set_then(|_| match compile_session(source, expected_name, network, false, IndexMap::new()) {
        Ok(c) => json!({
            "success": true,
            "output": c.primary.bytecode,
            "abi": serde_json::to_string_pretty(&c.primary.abi).unwrap_or_default(),
            "diagnostics": "",
        })
        .to_string(),
        Err(diag) => json!({"success": false, "output": "", "abi": "", "diagnostics": diag}).to_string(),
    })
}

/// Format Leo source.
pub fn format_impl(source: &str) -> String {
    leo_fmt::format_source(source)
}

/// Compile a multi-file project laid out as a virtual filesystem.
///
/// `files_json` is `{ "<path>": "<contents>" }`; `root` is the directory that
/// contains the main package's `program.json`.
///
/// Returns JSON:
/// `{ success, output, abi, imports: [{name, bytecode, abi}], diagnostics }`.
pub fn compile_project_impl(files_json: &str, root: &str) -> String {
    create_session_if_not_set_then(|_| {
        let proj = match project::Project::from_files_json(files_json, root) {
            Ok(p) => p,
            Err(e) => return error_json(&e, &["output", "abi", "imports"]),
        };
        let network = network_from_manifest(&proj);
        match project::compile(&proj, /* is_test */ false, network) {
            Ok(c) => json!({
                "success": true,
                "output": c.primary.bytecode,
                "abi": serde_json::to_string_pretty(&c.primary.abi).unwrap_or_default(),
                "imports": import_summaries(&c.imports),
                "diagnostics": "",
            })
            .to_string(),
            Err(diag) => error_json(&diag, &["output", "abi", "imports"]),
        }
    })
}

// ---------------------------------------------------------------------------
// WASM-only impls (depend on snarkVM execution via the `evaluate` module)
// ---------------------------------------------------------------------------

/// Compile and run one function.
///
/// Returns JSON: `{ success, output, finalize, diagnostics }`.
#[cfg(target_arch = "wasm32")]
pub fn run_impl(source: &str, function_name: &str, inputs_json: &str, program_json: &str) -> String {
    let (expected_name, network) = parse_program_json(program_json);
    let inputs: Vec<String> = serde_json::from_str(inputs_json).unwrap_or_default();

    create_session_if_not_set_then(|_| match compile_session(source, expected_name, network, false, IndexMap::new()) {
        Ok(c) => evaluate::run_function(&c.primary.bytecode, function_name, &inputs, &[]),
        Err(diag) => json!({"success": false, "output": "", "diagnostics": diag}).to_string(),
    })
}

/// Compile main + test source together and run every `@test` fn.
///
/// Returns JSON: `{ success, results: [ { name, passed, error } ], diagnostics }`.
#[cfg(target_arch = "wasm32")]
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

/// Compile a project and run one function.
#[cfg(target_arch = "wasm32")]
pub fn run_project_impl(files_json: &str, root: &str, function_name: &str, inputs_json: &str) -> String {
    let inputs: Vec<String> = serde_json::from_str(inputs_json).unwrap_or_default();
    create_session_if_not_set_then(|_| {
        let proj = match project::Project::from_files_json(files_json, root) {
            Ok(p) => p,
            Err(e) => return error_json(&e, &["output"]),
        };
        let network = network_from_manifest(&proj);
        match project::compile(&proj, false, network) {
            Ok(c) => {
                // Dep bytecodes go into the process before the primary so cross-program
                // calls resolve.
                let deps: Vec<String> = c.imports.iter().map(|p| p.bytecode.clone()).collect();
                evaluate::run_function(&c.primary.bytecode, function_name, &inputs, &deps)
            }
            Err(diag) => error_json(&diag, &["output"]),
        }
    })
}

/// Compile a project together with a test package and run every `@test` fn.
///
/// `test_root` points at the test package's root (its own `program.json`).
#[cfg(target_arch = "wasm32")]
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
// Shared helpers (target-neutral unless noted)
// ---------------------------------------------------------------------------

/// Compile a single Leo source string with a fresh handler + node builder.
fn compile_session(
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
fn compile_with(
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

/// Parse `main_source` once so it can be injected as a `Stub::FromLeo` when
/// the test program is compiled with the *same* node builder.
#[cfg(target_arch = "wasm32")]
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
#[cfg(target_arch = "wasm32")]
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
#[cfg(target_arch = "wasm32")]
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

/// Render buffered diagnostics as a newline-joined string.
fn diagnostics_from(buf: &BufferEmitter) -> String {
    buf.extract_errs().into_inner().iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n")
}

/// Build a JSON error response with `success: false`, `diagnostics: <msg>`, and
/// empty placeholders for the named result fields.
fn error_json(msg: &str, empty_fields: &[&str]) -> String {
    let mut obj = serde_json::Map::new();
    obj.insert("success".into(), serde_json::Value::Bool(false));
    for field in empty_fields {
        obj.insert((*field).into(), serde_json::Value::String(String::new()));
    }
    obj.insert("diagnostics".into(), serde_json::Value::String(msg.to_string()));
    serde_json::Value::Object(obj).to_string()
}

/// Extract `network` from the project's `program.json`. Defaults to
/// `TestnetV0` when the manifest is missing or doesn't specify a network.
fn network_from_manifest(project: &project::Project) -> NetworkName {
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

/// `Compiler::compile` returns one `CompiledProgram` per emitted import; project
/// API callers want a compact view they can iterate in JS.
fn import_summaries(imports: &[leo_compiler::CompiledProgram]) -> Vec<serde_json::Value> {
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
fn clone_file_source(files_json: &str) -> leo_span::file_source::InMemoryFileSource {
    use leo_span::file_source::InMemoryFileSource;
    let mut out = InMemoryFileSource::new();
    if let Ok(map) = serde_json::from_str::<IndexMap<String, String>>(files_json) {
        for (path, contents) in map {
            out.set(std::path::PathBuf::from(path), contents);
        }
    }
    out
}

/// Extract `(program_name, network)` from a `program.json` blob. Falls back to
/// `("main.aleo", TestnetV0)` when the JSON is missing or malformed.
fn parse_program_json(program_json: &str) -> (Option<String>, NetworkName) {
    let v: serde_json::Value = serde_json::from_str(program_json).unwrap_or_default();
    let name = v.get("program").and_then(|p| p.as_str()).unwrap_or("main.aleo").to_string();
    (Some(name), network_from_value(&v))
}

// ---------------------------------------------------------------------------
// WASM bindings
// ---------------------------------------------------------------------------
//
// One-liner `#[wasm_bindgen]` wrappers around the `*_impl` functions above.
// Gated to `wasm32` so a native workspace build doesn't pull `wasm-bindgen`
// (and the snarkVM subset crates) into its dependency graph.

#[cfg(target_arch = "wasm32")]
mod wasm_bindings {
    use wasm_bindgen::prelude::*;

    /// Install the panic hook so Rust panics surface as JS errors.
    #[wasm_bindgen(start)]
    pub fn init() {
        console_error_panic_hook::set_once();
    }

    #[wasm_bindgen]
    pub fn compile(source: &str, program_json: &str) -> String {
        super::compile_impl(source, program_json)
    }

    #[wasm_bindgen]
    pub fn format(source: &str) -> String {
        super::format_impl(source)
    }

    #[wasm_bindgen]
    pub fn run(source: &str, function_name: &str, inputs_json: &str, program_json: &str) -> String {
        super::run_impl(source, function_name, inputs_json, program_json)
    }

    #[wasm_bindgen]
    pub fn run_tests(main_source: &str, test_source: &str, program_json: &str) -> String {
        super::run_tests_impl(main_source, test_source, program_json)
    }

    #[wasm_bindgen]
    pub fn compile_project(files_json: &str, root: &str) -> String {
        super::compile_project_impl(files_json, root)
    }

    #[wasm_bindgen]
    pub fn run_project(files_json: &str, root: &str, function_name: &str, inputs_json: &str) -> String {
        super::run_project_impl(files_json, root, function_name, inputs_json)
    }

    #[wasm_bindgen]
    pub fn test_project(files_json: &str, root: &str, test_root: &str) -> String {
        super::test_project_impl(files_json, root, test_root)
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_bindings::{compile, compile_project, format, init, run, run_project, run_tests, test_project};
