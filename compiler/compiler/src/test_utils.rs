// Copyright (C) 2019-2025 Provable Inc.
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

use crate::Compiler;

use leo_ast::{NetworkName, Stub};
use leo_errors::{Handler, LeoError};
use leo_span::{Symbol, source_map::FileName};

use indexmap::IndexMap;

/// Compiles a complete inlined test file containing the main program and modules separated by markers.
pub fn whole_compile(
    source: &str,
    handler: &Handler,
    import_stubs: IndexMap<Symbol, Stub>,
) -> Result<(String, String), LeoError> {
    let mut compiler = Compiler::new(
        None,
        /* is_test */ false,
        handler.clone(),
        "/fakedirectory-wont-use".into(),
        None,
        import_stubs,
        NetworkName::TestnetV0,
    );

    if !source.contains("// --- Next Module:") {
        // Fast path: no modules, just compile the main source
        let filename = FileName::Custom("compiler-test".into());
        let bytecode = compiler.compile(source, filename.clone(), &Vec::new())?;
        return Ok((bytecode, compiler.program_name.unwrap()));
    }

    // Parse the main source and modules
    let lines = source.lines().peekable();
    let mut main_source = String::new();
    let mut modules = Vec::new();

    let mut current_module_path: Option<String> = None;
    let mut current_module_source = String::new();

    for line in lines {
        if let Some(rest) = line.strip_prefix("// --- Next Module: ") {
            // Save previous module or main source
            if let Some(path) = current_module_path.take() {
                modules.push((current_module_source.clone(), FileName::Custom(path)));
                current_module_source.clear();
            } else {
                main_source = current_module_source.clone();
                current_module_source.clear();
            }

            // Start new module
            let path = rest.trim().trim_end_matches(" --- //").to_string();
            current_module_path = Some(path);
        } else {
            current_module_source.push_str(line);
            current_module_source.push('\n');
        }
    }

    // Push the last module, if any
    if let Some(path) = current_module_path {
        modules.push((current_module_source.clone(), FileName::Custom(path)));
    } else {
        main_source = current_module_source;
    }

    // === Sort modules by path depth (deeper first) ===
    modules.sort_by(|(_, a), (_, b)| {
        let a_depth = match a {
            FileName::Custom(s) => std::path::Path::new(s).components().count(),
            _ => 0, // Shouldn't happen
        };
        let b_depth = match b {
            FileName::Custom(s) => std::path::Path::new(s).components().count(),
            _ => 0,
        };
        b_depth.cmp(&a_depth)
    });

    // Prepare module refs: Vec<(&str, FileName)>
    let module_refs: Vec<(&str, FileName)> = modules.iter().map(|(src, fname)| (src.as_str(), fname.clone())).collect();

    let filename = FileName::Custom("compiler-test".into());
    let bytecode = compiler.compile(&main_source, filename, &module_refs)?;

    Ok((bytecode, compiler.program_name.unwrap()))
}
