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

//! In-browser execution of Aleo bytecode via `Process::evaluate` (no proof).
//!
//! This wraps the wasm-friendly subset of snarkVM (`snarkvm-circuit`,
//! `snarkvm-synthesizer-{process,program}`, `snarkvm-ledger-{block,store}`)
//! so the rest of `leo-wasm` can stay synchronous and JSON-shaped.

use aleo_std::StorageMode;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use serde_json::json;
use snarkvm_circuit::AleoTestnetV0;
use snarkvm_console::{
    account::PrivateKey,
    network::TestnetV0,
    program::{Identifier, Value as SvmValue},
};
use snarkvm_ledger_block::{Execution, Transition};
use snarkvm_ledger_store::{FinalizeStore, helpers::memory::FinalizeMemory};
use snarkvm_synthesizer_process::Process;
use snarkvm_synthesizer_program::{FinalizeGlobalState, Program as SvmProgram};
use std::str::FromStr;

/// Well-known test key used for every playground execution.
const TEST_PRIVATE_KEY: &str = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH";

/// Evaluate an Aleo function with the given inputs and return JSON.
///
/// `extra_bytecodes` are dependency programs (e.g. the main program when running
/// a test). They are loaded into the process before `bytecode`. snarkVM panics
/// (overflow, halt) are caught so they surface as JSON rather than wasm traps.
pub fn run_function(bytecode: &str, function_name: &str, inputs: &[String], extra_bytecodes: &[String]) -> String {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        run_function_inner(bytecode, function_name, inputs, extra_bytecodes)
    }))
    .unwrap_or_else(|e| {
        let msg = e
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_else(|| "runtime error".to_string());
        json!({"success": false, "output": "", "diagnostics": msg}).to_string()
    })
}

fn run_function_inner(bytecode: &str, function_name: &str, inputs: &[String], extra_bytecodes: &[String]) -> String {
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
    // Keep parsed programs so their mappings can be initialized in `run_finalize`.
    let mut dep_programs: Vec<SvmProgram<TestnetV0>> = Vec::new();
    for dep in extra_bytecodes.iter().filter(|s| !s.is_empty()) {
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

    // Capture transitions before `evaluate` consumes the authorization.
    // `authorize_unchecked` runs in Authorize mode and populates full transitions
    // (including finalize Future outputs), so no proof generation is needed.
    let transitions = auth.transitions();

    let response = match process.evaluate::<AleoTestnetV0>(auth) {
        Ok(r) => r,
        Err(e) => fail!("evaluation error: {e}"),
    };

    let output = response.outputs().iter().map(|v| v.to_string()).collect::<Vec<_>>().join("\n");

    let has_finalize = program.get_function(&fn_id).is_ok_and(|f| f.finalize_logic().is_some());
    if !has_finalize {
        return json!({"success": true, "output": output, "finalize": null, "diagnostics": ""}).to_string();
    }

    match run_finalize(&process, &program, &dep_programs, transitions.into_values()) {
        Ok(finalize) => json!({"success": true, "output": output, "finalize": finalize, "diagnostics": ""}).to_string(),
        Err(e) => json!({"success": false, "output": "", "diagnostics": e}).to_string(),
    }
}

/// Run the finalize block against a fresh in-memory store and return the
/// resulting mapping state, or an error string on failure.
///
/// `dep_programs` mappings are initialized too so cross-program finalize calls
/// (e.g. view functions, cross-program awaits) can access them.
fn run_finalize(
    process: &Process<TestnetV0>,
    program: &SvmProgram<TestnetV0>,
    dep_programs: &[SvmProgram<TestnetV0>],
    transitions: impl Iterator<Item = Transition<TestnetV0>>,
) -> Result<serde_json::Value, String> {
    let execution = Execution::from(transitions, Default::default(), None).map_err(|e| format!("execution: {e}"))?;

    let store = FinalizeStore::<TestnetV0, FinalizeMemory<TestnetV0>>::open(StorageMode::Test(None))
        .map_err(|e| format!("store: {e}"))?;

    // Initialize mappings for every loaded program (normally done at deployment).
    for p in dep_programs.iter().chain(std::iter::once(program)) {
        for mapping_name in p.mappings().keys() {
            store
                .initialize_mapping(*p.id(), *mapping_name)
                .map_err(|e| format!("init mapping '{}::{}': {e}", p.id(), mapping_name))?;
        }
    }

    let state = FinalizeGlobalState::new_genesis::<TestnetV0>().map_err(|e| format!("state: {e}"))?;
    process.lock().finalize_execution(state, &store, &execution, None).map_err(|e| format!("finalize: {e}"))?;

    // Read back every primary-program mapping's key-value pairs.
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
