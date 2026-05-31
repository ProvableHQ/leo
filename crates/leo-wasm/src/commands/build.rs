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

//! `leo build` — compile a Leo project to Aleo bytecode + ABI.
//!
//! Takes a project laid out as a `{ "<path>": "<contents>" }` virtual file
//! map and a `root` pointing at the directory that contains the main
//! package's `program.json`. The shape mirrors what `leo build` reads from
//! a real directory; single-source callers stage a 2-entry map
//! (`program.json` + `src/main.leo`).

use crate::{
    project,
    wire::{error_json, import_summaries, network_from_manifest},
};

use leo_span::create_session_if_not_set_then;
use serde_json::json;

/// Compile a Leo project.
///
/// Returns JSON:
/// `{ success, output, abi, imports: [{name, bytecode, abi}], diagnostics }`.
pub fn build_impl(files_json: &str, root: &str) -> String {
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
