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

use std::path::PathBuf;

use indexmap::IndexMap;

pub const PROGRAM_DELIMITER: &str = "// --- Next Program --- //";
pub const MODULE_DELIMITER: &str = "// --- Next Module:";

/// Compiles a complete program from a single source string that may contain
/// embedded modules marked by a delimiter.
///
/// The source string is expected to contain sections separated by the `MODULE_DELIMITER`,
/// each representing either the main source or a named module. The compiler parses each
/// section and compiles the full program, including any modules.
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

    if !source.contains(MODULE_DELIMITER) {
        // Fast path: no modules
        let filename = FileName::Custom("compiler-test".into());
        let bytecode = compiler.compile(source, filename.clone(), &Vec::new())?;
        return Ok((bytecode, compiler.program_name.unwrap()));
    }

    let mut main_source = String::new();
    let mut modules: Vec<(String, PathBuf)> = Vec::new();

    let mut current_module_path: Option<PathBuf> = None;
    let mut current_module_source = String::new();

    for line in source.lines() {
        if let Some(rest) = line.strip_prefix(MODULE_DELIMITER) {
            // Save previous block
            if let Some(path) = current_module_path.take() {
                modules.push((current_module_source.clone(), path));
                current_module_source.clear();
            } else {
                main_source = current_module_source.clone();
                current_module_source.clear();
            }

            // Start new module
            let trimmed_path = rest.trim().trim_end_matches(" --- //");
            current_module_path = Some(PathBuf::from(trimmed_path));
        } else {
            current_module_source.push_str(line);
            current_module_source.push('\n');
        }
    }

    // Push the last module or main
    if let Some(path) = current_module_path {
        modules.push((current_module_source.clone(), path));
    } else {
        main_source = current_module_source;
    }

    // Prepare module references for compiler
    let module_refs: Vec<(&str, FileName)> =
        modules.iter().map(|(src, path)| (src.as_str(), FileName::Custom(path.to_string_lossy().into()))).collect();

    let filename = FileName::Custom("compiler-test".into());
    let bytecode = compiler.compile(&main_source, filename, &module_refs)?;

    Ok((bytecode, compiler.program_name.unwrap()))
}
