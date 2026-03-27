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

use crate::{Compiled, Compiler};

use leo_ast::{Library, NetworkName, NodeBuilder, Program, Stub};
use leo_errors::{Handler, LeoError};
use leo_span::{Symbol, source_map::FileName};

use std::{path::PathBuf, rc::Rc};

use indexmap::IndexMap;

pub const PROGRAM_DELIMITER: &str = "// --- Next Program --- //";
pub const MODULE_DELIMITER: &str = "// --- Next Module:";

/// Fully compiles a Leo source string into bytecode.
///
/// This performs the entire compilation pipeline:
/// - splits embedded modules,
/// - initializes a compiler with the given `handler`, `node_builder`, and `import_stubs`,
/// - compiles the main program and its modules,
/// - returns:
///     * `(main_bytecode, imported_bytecodes)` and
///     * the compiled program's name.
///
/// Used when compiling the final (top-level) program in a test.
#[allow(clippy::type_complexity)]
pub fn whole_compile(
    source: &str,
    handler: &Handler,
    node_builder: &Rc<NodeBuilder>,
    import_stubs: IndexMap<Symbol, Stub>,
) -> Result<(Compiled, String), LeoError> {
    let (main_source, modules) = split_modules(source);

    let mut compiler = Compiler::new(
        None,
        /* is_test */ false,
        handler.clone(),
        node_builder.clone(),
        "/fakedirectory-wont-use".into(),
        None,
        import_stubs,
        NetworkName::TestnetV0,
    );

    // Prepare module references
    let module_refs: Vec<(&str, FileName)> =
        modules.iter().map(|(src, path)| (src.as_str(), FileName::Custom(path.to_string_lossy().into()))).collect();

    let filename = FileName::Custom("compiler-test".into());
    let bytecodes = compiler.compile(&main_source, filename, &module_refs)?;

    Ok((bytecodes, compiler.program_name.unwrap()))
}

/// Parses a Leo source string into an AST `Program` without generating bytecode.
///
/// This runs only the front-end portion of the pipeline:
/// - splits embedded modules,
/// - initializes a compiler with the given `handler`, `node_builder`, and `import_stubs`,
/// - parses the main program and its modules into an AST,
/// - returns the parsed `Program` and the program's name.
///
/// Used for intermediate programs that are imported by the final one.
pub fn parse_program(
    source: &str,
    handler: &Handler,
    node_builder: &Rc<NodeBuilder>,
    import_stubs: IndexMap<Symbol, Stub>,
) -> Result<(Program, String), LeoError> {
    let (main_source, modules) = split_modules(source);

    let mut compiler = Compiler::new(
        None,
        /* is_test */ false,
        handler.clone(),
        node_builder.clone(),
        "/fakedirectory-wont-use".into(),
        None,
        import_stubs,
        NetworkName::TestnetV0,
    );

    // Prepare module references
    let module_refs: Vec<(&str, FileName)> =
        modules.iter().map(|(src, path)| (src.as_str(), FileName::Custom(path.to_string_lossy().into()))).collect();

    let filename = FileName::Custom("compiler-test".into());
    let program = compiler.parse_and_return_program(&main_source, filename, &module_refs)?;

    Ok((program, compiler.program_name.unwrap()))
}

/// Parses a Leo library from the given source for use in compiler tests.
///
/// The source may contain `// --- Next Module: name --- //` delimiters to embed
/// submodule source files, using the same protocol as `parse_program`.
pub fn parse_library(
    library_name: &str,
    source: &str,
    handler: &Handler,
    node_builder: &Rc<NodeBuilder>,
) -> Result<Library, LeoError> {
    let (main_source, modules) = split_modules(source);

    let mut compiler = Compiler::new(
        None,
        /* is_test */ false,
        handler.clone(),
        node_builder.clone(),
        "/fakedirectory-wont-use".into(),
        None,
        IndexMap::new(),
        NetworkName::TestnetV0,
    );

    let filename = FileName::Custom("compiler-test".into());
    let module_refs: Vec<(&str, FileName)> =
        modules.iter().map(|(src, path)| (src.as_str(), FileName::Custom(path.to_string_lossy().into()))).collect();

    compiler.parse_and_return_library(library_name, &main_source, filename, &module_refs)
}

/// Splits a single source string into a main source and a list of module
/// `(source, path)` pairs using the MODULE_DELIMITER protocol.
///
/// Shared by both `whole_compile` and `parse`.
fn split_modules(source: &str) -> (String, Vec<(String, PathBuf)>) {
    // Fast path — no modules at all
    if !source.contains(MODULE_DELIMITER) {
        return (source.to_string(), Vec::new());
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

    (main_source, modules)
}

/// Extracts the program name from a Leo source string.
///
/// This parses the source using the test parser pipeline and returns the
/// program identifier declared in the program scope.
pub fn extract_program_name(source: &str, handler: &Handler) -> Result<String, LeoError> {
    let (_program, program_name) =
        parse_program(source, handler, &Rc::new(NodeBuilder::default()), indexmap::IndexMap::new())?;
    Ok(program_name)
}

/// Extracts a test library header of the form `// --- library: NAME --- //`.
///
/// If present, returns the library name together with the remaining source
/// after the header. Otherwise returns `None`.
pub fn extract_library_header(source: &str) -> Option<(String, &str)> {
    let mut offset = 0;

    for line in source.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("// --- library:") && trimmed.ends_with("--- //") {
            let name = trimmed.trim_start_matches("// --- library:").trim_end_matches("--- //").trim().to_string();

            let rest = &source[offset + line.len()..].trim_start_matches('\n');
            return Some((name, rest));
        }

        offset += line.len() + 1; // advance past line + newline
    }

    None
}
