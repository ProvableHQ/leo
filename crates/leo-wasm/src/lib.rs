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

//! WASM bindings for the Leo compiler.
//!
//! Build with:
//!   wasm-pack build crates/leo-wasm --target web --out-dir ../../leo-playground/wasm

use aleo_std::StorageMode;
use indexmap::IndexSet;
use leo_ast::{Ast, NetworkName, NodeBuilder, Stub};
use leo_errors::{BufferEmitter, Handler};
use leo_passes::*;
use leo_span::{Symbol, create_session_if_not_set_then, source_map::FileName, sym, with_session_globals};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use serde_json::json;
use snarkvm_circuit::AleoTestnetV0;
use snarkvm_console::{
    account::PrivateKey,
    network::TestnetV0,
    program::{Identifier, Value as SvmValue},
};
use snarkvm_ledger_block::Execution;
use snarkvm_ledger_store::{FinalizeStore, helpers::memory::FinalizeMemory};
use snarkvm_synthesizer_process::Process;
use snarkvm_synthesizer_program::{FinalizeGlobalState, Program as SvmProgram};
use std::{rc::Rc, str::FromStr};
use wasm_bindgen::prelude::*;

// Private key used for all playground executions (well-known test key).
const TEST_PRIVATE_KEY: &str = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH";

// Install a panic hook so Rust panics surface as JS errors.
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Compiles Leo source code to Aleo bytecode.
///
/// Returns JSON: `{ success, output, abi, diagnostics }`.
#[wasm_bindgen]
pub fn compile(source: &str, program_json: &str) -> String {
    let (program_name, network) = parse_program_json(program_json);

    create_session_if_not_set_then(|_| compile_leo(source, program_name.as_deref(), network, false, &[]))
}

/// Formats Leo source code.
#[wasm_bindgen]
pub fn format(source: &str) -> String {
    leo_fmt::format_source(source)
}

/// Compiles and runs a Leo function with the provided inputs.
///
/// - `inputs_json`: JSON array of strings, e.g. `["1u32", "2u32"]`.
/// - `program_json`: the program.json object as a JSON string.
///
/// Returns JSON: `{ success, output, diagnostics }`.
#[wasm_bindgen]
pub fn run(source: &str, function_name: &str, inputs_json: &str, program_json: &str) -> String {
    let (program_name, network) = parse_program_json(program_json);

    let bytecode = create_session_if_not_set_then(|_| {
        let result = compile_leo(source, program_name.as_deref(), network, false, &[]);
        let v: serde_json::Value = serde_json::from_str(&result).unwrap_or_default();
        if !v["success"].as_bool().unwrap_or(false) {
            return Err(v["diagnostics"].as_str().unwrap_or("compile failed").to_string());
        }
        Ok(v["output"].as_str().unwrap_or("").to_string())
    });

    let bytecode = match bytecode {
        Ok(b) => b,
        Err(diag) => {
            return json!({"success": false, "output": "", "diagnostics": diag}).to_string();
        }
    };

    let inputs: Vec<String> = serde_json::from_str(inputs_json).unwrap_or_default();
    evaluate_aleo(&bytecode, function_name, &inputs, &[])
}

/// Compiles main + test source and runs all `@test` functions.
///
/// Returns JSON: `{ success, results: [ { name, passed, error } ] }`.
#[wasm_bindgen]
pub fn run_tests(main_source: &str, test_source: &str, program_json: &str) -> String {
    let (_, network) = parse_program_json(program_json);

    create_session_if_not_set_then(|_| {
        // Step 1: Compile main program.
        let (_, main_network) = parse_program_json(program_json);
        let main_result = compile_leo(main_source, None, main_network, false, &[]);
        let main_v: serde_json::Value = serde_json::from_str(&main_result).unwrap_or_default();
        if !main_v["success"].as_bool().unwrap_or(false) {
            return json!({
                "success": false,
                "results": [],
                "diagnostics": main_v["diagnostics"].as_str().unwrap_or("main compile failed"),
            })
            .to_string();
        }
        let main_bytecode = main_v["output"].as_str().unwrap_or("").to_string();

        // Step 2: Disassemble main bytecode → Leo stub.
        let main_stub = match make_stub_from_bytecode(&main_bytecode, network) {
            Ok(s) => s,
            Err(e) => {
                return json!({"success": false, "results": [], "diagnostics": e}).to_string();
            }
        };

        // Step 3: Parse test source to find @test functions (before compilation modifies it).
        let test_fns = find_test_functions(test_source, network);

        if test_fns.is_empty() {
            return json!({"success": true, "results": [], "diagnostics": ""}).to_string();
        }

        // Step 4: Compile test program with the main stub injected.
        let result = compile_leo(test_source, None, network, true, &[main_stub]);
        let test_bytecode = {
            let v: serde_json::Value = serde_json::from_str(&result).unwrap_or_default();
            if !v["success"].as_bool().unwrap_or(false) {
                return json!({
                    "success": false,
                    "results": [],
                    "diagnostics": v["diagnostics"].as_str().unwrap_or("test compile failed"),
                })
                .to_string();
            }
            v["output"].as_str().unwrap_or("").to_string()
        };

        // Step 5: Execute each @test function.
        let results: Vec<serde_json::Value> = test_fns
            .iter()
            .map(|(prog, fn_name, should_fail)| {
                let qualified = format!("{prog}/{fn_name}");
                let exec = evaluate_aleo(&test_bytecode, fn_name, &[], std::slice::from_ref(&main_bytecode));
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
            .collect();

        let all_passed = results.iter().all(|r| r["passed"].as_bool().unwrap_or(false));
        json!({"success": all_passed, "results": results, "diagnostics": ""}).to_string()
    })
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Runs the full Leo compile pipeline.
/// `stubs` are injected into the program's stub map before passes run.
fn compile_leo(
    source: &str,
    expected_name: Option<&str>,
    network: NetworkName,
    is_test: bool,
    stubs: &[(Symbol, Stub)],
) -> String {
    let (handler, buf) = Handler::new_with_buf();
    let node_builder = Rc::new(NodeBuilder::default());

    let source_file =
        with_session_globals(|s| s.source_map.new_source(source, FileName::Custom("main.leo".to_string())));

    let mut program = match leo_parser::parse_program(handler.clone(), &node_builder, &source_file, &[], network) {
        Ok(p) if !handler.had_errors() => p,
        _ => return error_result(&buf),
    };

    // Validate program name against expected (from program.json).
    if let (Some(expected), Some((sym, _))) = (expected_name, program.program_scopes.first())
        && sym.to_string() != expected
    {
        return json!({
            "success": false,
            "output": "",
            "abi": "",
            "diagnostics": format!("program name `{}` does not match `{}` in program.json", sym, expected),
        })
        .to_string();
    }

    // Inject import stubs with the current program set as parent, matching the
    // parent-relationship setup that Compiler::add_import_stubs normally does.
    // Without this, GlobalVarsCollection cannot populate the imports map and
    // PathResolution fails to resolve external calls like `hello.aleo::sum`.
    let current_program_sym = program.program_scopes.first().map(|(k, _)| *k);
    for (key, stub) in stubs {
        let mut stub = stub.clone();
        if let Some(sym) = current_program_sym {
            stub.add_parent(sym);
        }
        program.stubs.insert(*key, stub);
    }

    let tc_input = TypeCheckingInput::new(network);

    let mut state = CompilerState {
        ast: Ast::Program(program),
        handler: handler.clone(),
        node_builder: Rc::clone(&node_builder),
        network,
        is_test,
        ..Default::default()
    };

    if frontend_passes(&mut state, tc_input.clone()).is_err() || handler.had_errors() {
        return error_result(&buf);
    }

    let abi = match intermediate_passes(&mut state, tc_input) {
        Ok(abi) => abi,
        Err(_) => return error_result(&buf),
    };

    if handler.had_errors() {
        return error_result(&buf);
    }

    let bytecodes = match CodeGenerating::do_pass((), &mut state) {
        Ok(b) => b,
        Err(_) => return error_result(&buf),
    };

    json!({
        "success": true,
        "output": bytecodes.primary_bytecode,
        "abi": serde_json::to_string_pretty(&abi).unwrap_or_default(),
        "diagnostics": "",
    })
    .to_string()
}

/// Evaluates an Aleo function using `Process::evaluate` (no proof).
/// If the function has a `finalize` block, also runs it against an in-memory store.
/// `extra_bytecodes` are additional programs to load before running `bytecode`.
fn evaluate_aleo(bytecode: &str, function_name: &str, inputs: &[String], extra_bytecodes: &[String]) -> String {
    // Wrap in catch_unwind to convert snarkVM panics (e.g. integer overflow via N::halt)
    // into structured error responses instead of WASM unreachable traps.
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        evaluate_aleo_inner(bytecode, function_name, inputs, extra_bytecodes)
    }))
    .unwrap_or_else(|e| {
        let msg = if let Some(s) = e.downcast_ref::<String>() {
            s.clone()
        } else if let Some(s) = e.downcast_ref::<&str>() {
            s.to_string()
        } else {
            "runtime error".to_string()
        };
        json!({"success": false, "output": "", "diagnostics": msg}).to_string()
    })
}

fn evaluate_aleo_inner(bytecode: &str, function_name: &str, inputs: &[String], extra_bytecodes: &[String]) -> String {
    macro_rules! fail {
        ($fmt:literal $(, $($arg:tt)*)?) => {
            return json!({"success": false, "output": "", "diagnostics": format!($fmt $(, $($arg)*)?)}).to_string()
        };
    }

    let process = match Process::<TestnetV0>::load_web() {
        Ok(p) => p,
        Err(e) => fail!("process init error: {e}"),
    };

    // Load dependency programs first (e.g. main program when running a test).
    // Keep parsed programs so their mappings can be initialized in run_finalize.
    let mut dep_programs: Vec<SvmProgram<TestnetV0>> = Vec::new();
    for dep in extra_bytecodes {
        if dep.is_empty() {
            continue;
        }
        match SvmProgram::<TestnetV0>::from_str(dep) {
            Ok(p) => {
                if let Err(e) = process.lock().add_program(&p) {
                    fail!("add dep error: {e}");
                }
                dep_programs.push(p);
            }
            Err(e) => fail!("dep parse error: {e}"),
        }
    }

    let program = match SvmProgram::<TestnetV0>::from_str(bytecode) {
        Ok(p) => p,
        Err(e) => fail!("bytecode parse error: {e}"),
    };
    if let Err(e) = process.lock().add_program(&program) {
        fail!("add program error: {e}");
    }

    let private_key = match PrivateKey::<TestnetV0>::from_str(TEST_PRIVATE_KEY) {
        Ok(pk) => pk,
        Err(e) => fail!("key error: {e}"),
    };

    let parsed_inputs: Result<Vec<SvmValue<TestnetV0>>, _> = inputs.iter().map(|s| SvmValue::from_str(s)).collect();
    let parsed_inputs = match parsed_inputs {
        Ok(v) => v,
        Err(e) => fail!("input parse error: {e}"),
    };

    let fn_id = match Identifier::<TestnetV0>::from_str(function_name) {
        Ok(id) => id,
        Err(e) => fail!("function name error: {e}"),
    };

    let rng = &mut ChaCha20Rng::seed_from_u64(0);
    let auth = match process.authorize_unchecked::<AleoTestnetV0, _>(
        &private_key,
        *program.id(),
        fn_id,
        parsed_inputs.into_iter(),
        rng,
    ) {
        Ok(a) => a,
        Err(e) => fail!("authorization error: {e}"),
    };

    // Clone transitions from the authorization before it is consumed by evaluate.
    // authorize_unchecked runs in Authorize mode and already populates full transitions
    // (including finalize Future outputs), so no proof generation is needed.
    let transitions = auth.transitions();

    let response = match process.evaluate::<AleoTestnetV0>(auth) {
        Ok(r) => r,
        Err(e) => fail!("evaluation error: {e}"),
    };

    let output = response.outputs().iter().map(|v| v.to_string()).collect::<Vec<_>>().join("\n");

    // Check if this function has a finalize block; if so, execute it.
    let has_finalize = program.get_function(&fn_id).is_ok_and(|f| f.finalize_logic().is_some());
    if !has_finalize {
        return json!({"success": true, "output": output, "finalize": null, "diagnostics": ""}).to_string();
    }

    match run_finalize(&process, &program, &dep_programs, transitions.into_values()) {
        Ok(finalize) => json!({"success": true, "output": output, "finalize": finalize, "diagnostics": ""}).to_string(),
        Err(e) => json!({"success": false, "output": "", "diagnostics": e}).to_string(),
    }
}

/// Runs the finalize block for an already-evaluated execution against a fresh in-memory store.
/// Returns the mapping state on success, or an error string on failure.
///
/// `dep_programs` are dependency programs (e.g. the imported program in a test) whose
/// mappings also need to be initialized so cross-program finalize calls can access them.
fn run_finalize(
    process: &Process<TestnetV0>,
    program: &SvmProgram<TestnetV0>,
    dep_programs: &[SvmProgram<TestnetV0>],
    transitions: impl Iterator<Item = snarkvm_ledger_block::Transition<TestnetV0>>,
) -> Result<serde_json::Value, String> {
    // Build the execution without a proof — dev_skip_checks skips proof verification.
    let execution = Execution::from(transitions, Default::default(), None).map_err(|e| format!("execution: {e}"))?;

    // Open a fresh in-memory finalize store.
    let store = FinalizeStore::<TestnetV0, FinalizeMemory<TestnetV0>>::open(StorageMode::Test(None))
        .map_err(|e| format!("store: {e}"))?;

    // Initialize mappings for all programs (normally done at deployment time).
    // Dependencies come first so their mappings are ready when the primary program's
    // finalize block calls into them (e.g. via view functions or cross-program awaits).
    for p in dep_programs.iter().chain(std::iter::once(program)) {
        for mapping_name in p.mappings().keys() {
            store
                .initialize_mapping(*p.id(), *mapping_name)
                .map_err(|e| format!("init mapping '{}::{}': {e}", p.id(), mapping_name))?;
        }
    }

    // Genesis state is sufficient for local testing.
    let state = FinalizeGlobalState::new_genesis::<TestnetV0>().map_err(|e| format!("state: {e}"))?;

    process.lock().finalize_execution(state, &store, &execution, None).map_err(|e| format!("finalize: {e}"))?;

    // Read back every mapping's key-value pairs as human-readable strings.
    let mut out = serde_json::Map::new();
    for mapping_name in program.mappings().keys() {
        let pairs = store
            .get_mapping_speculative(*program.id(), *mapping_name)
            .map_err(|e| format!("read '{mapping_name}': {e}"))?;
        let entries: Vec<serde_json::Value> =
            pairs.iter().map(|(k, v)| serde_json::Value::String(format!("{k} => {v}"))).collect();
        out.insert(mapping_name.to_string(), serde_json::Value::Array(entries));
    }
    Ok(serde_json::Value::Object(out))
}

/// Disassembles Aleo bytecode into a Leo `Stub::FromAleo` ready to inject.
fn make_stub_from_bytecode(bytecode: &str, network: NetworkName) -> Result<(Symbol, Stub), String> {
    let aleo_prog = leo_disassembler::disassemble_from_str_for_network("main.aleo", bytecode, network)
        .map_err(|e| format!("disassemble error: {e}"))?;
    let key = aleo_prog.stub_id.as_symbol();
    let stub = Stub::FromAleo { program: aleo_prog, parents: IndexSet::new() };
    Ok((key, stub))
}

/// Parses the test Leo source (without running passes) and returns a list of
/// `(program_name, function_name, should_fail)` for every `@test`-annotated function.
fn find_test_functions(test_source: &str, network: NetworkName) -> Vec<(String, String, bool)> {
    let (handler, _) = Handler::new_with_buf();
    let node_builder = Rc::new(NodeBuilder::default());

    let source_file =
        with_session_globals(|s| s.source_map.new_source(test_source, FileName::Custom("test.leo".to_string())));

    let program = match leo_parser::parse_program(handler, &node_builder, &source_file, &[], network) {
        Ok(p) => p,
        Err(_) => return Vec::new(),
    };

    let mut out = Vec::new();
    for (prog_sym, scope) in &program.program_scopes {
        let prog_name = prog_sym.to_string();
        for (fn_sym, function) in &scope.functions {
            let has_test = function.annotations.iter().any(|a| a.identifier.name == sym::test);
            if !has_test {
                continue;
            }
            let should_fail = function.annotations.iter().any(|a| a.identifier.name == sym::should_fail);
            out.push((prog_name.clone(), fn_sym.to_string(), should_fail));
        }
    }
    out
}

/// Runs the frontend passes: NameValidation through StaticAnalyzing.
fn frontend_passes(state: &mut CompilerState, tc: TypeCheckingInput) -> leo_errors::Result<()> {
    state.handler.last_err()?;
    NameValidation::do_pass((), state)?;
    GlobalVarsCollection::do_pass((), state)?;
    PathResolution::do_pass((), state)?;
    GlobalItemsCollection::do_pass((), state)?;
    CheckInterfaces::do_pass((), state)?;
    TypeChecking::do_pass(tc.clone(), state)?;
    Disambiguate::do_pass((), state)?;
    CeiAnalyzing::do_pass((), state)?;
    ProcessingAsync::do_pass(tc, state)?;
    StaticAnalyzing::do_pass((), state)?;
    Ok(())
}

/// Runs the intermediate passes and returns the primary ABI.
fn intermediate_passes(state: &mut CompilerState, tc: TypeCheckingInput) -> leo_errors::Result<leo_abi::Program> {
    ConstPropUnrollAndMorphing::do_pass(tc.clone(), state)?;

    let abi = match &state.ast {
        Ast::Program(p) => leo_abi::generate(p),
        Ast::Library(_) => unreachable!("expected Program AST"),
    };

    StorageLowering::do_pass(tc.clone(), state)?;
    OptionLowering::do_pass(tc, state)?;

    SsaForming::do_pass(SsaFormingInput { rename_defs: true }, state)?;
    Destructuring::do_pass((), state)?;
    SsaForming::do_pass(SsaFormingInput { rename_defs: false }, state)?;
    WriteTransforming::do_pass((), state)?;
    SsaForming::do_pass(SsaFormingInput { rename_defs: false }, state)?;
    Flattening::do_pass((), state)?;
    FunctionInlining::do_pass((), state)?;
    SsaForming::do_pass(SsaFormingInput { rename_defs: false }, state)?;
    SsaConstPropagation::do_pass((), state)?;
    SsaForming::do_pass(SsaFormingInput { rename_defs: false }, state)?;
    CommonSubexpressionEliminating::do_pass((), state)?;
    DeadCodeEliminating::do_pass((), state)?;

    Ok(abi)
}

/// Collect emitted errors into a JSON failure result.
fn error_result(buf: &BufferEmitter) -> String {
    let errors = buf.extract_errs();
    let diagnostics: String = errors.into_inner().iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n");
    json!({"success": false, "output": "", "abi": "", "diagnostics": diagnostics}).to_string()
}

/// Extract program name and network from `program.json`.
fn parse_program_json(program_json: &str) -> (Option<String>, NetworkName) {
    let v: serde_json::Value = serde_json::from_str(program_json).unwrap_or_default();
    let name = v.get("program").and_then(|p| p.as_str()).unwrap_or("main.aleo").to_string();
    let network = match v.get("network").and_then(|n| n.as_str()) {
        Some("mainnet") => NetworkName::MainnetV0,
        Some("canary") => NetworkName::CanaryV0,
        _ => NetworkName::TestnetV0,
    };
    (Some(name), network)
}
