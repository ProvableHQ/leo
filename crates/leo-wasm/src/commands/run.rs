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
//! Wasm-only: uses [`crate::evaluate::run_function`] (`Process::load_web` +
//! `FinalizeMemory`) which is only available in wasm builds of snarkVM.

use crate::{
    evaluate,
    project,
    wire::{error_json, network_from_manifest},
};

use leo_span::create_session_if_not_set_then;

/// Compile a project and run one function.
///
/// `inputs_json` is a JSON array of Leo-typed input strings, e.g. `["1u32", "2u32"]`.
///
/// Returns JSON: `{ success, output, finalize, diagnostics }`.
pub fn run_impl(files_json: &str, root: &str, function_name: &str, inputs_json: &str) -> String {
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
