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

use std::path::PathBuf;

use leo_ast::NetworkName;
use leo_errors::{BufferEmitter, Handler};
use leo_span::{create_session_if_not_set_then, source_map::FileName};

use indexmap::IndexMap;
use serial_test::serial;

use crate::Linter;

pub const MODULE_DELIMITER: &str = "// --- Next Module:";

/// Splits a monolithic source file into a main script and submodules.
/// Modules are separated using the `MODULE_DELIMITER` followed by the module path.
///
/// Returns:
/// - `Option<(&str, FileName)>`: The main module's source (if present)
/// - `Vec<(&str, FileName)>`: A list of module source strings and their corresponding paths
#[expect(clippy::type_complexity)]
fn split_modules(source: &str) -> (Option<(&str, FileName)>, Vec<(&str, FileName)>) {
    let mut main_source = None;
    let mut modules = Vec::new();

    let mut current_module_path: Option<PathBuf> = None;
    let mut current_start = 0;

    for (i, line) in source.lines().enumerate() {
        let line_start = source.lines().take(i).map(|s| s.len() + 1).sum(); // byte offset

        if let Some(rest) = line.strip_prefix(MODULE_DELIMITER) {
            // End the previous block
            let block = &source[current_start..line_start];

            if let Some(path) = current_module_path.take() {
                modules.push((block, path));
            } else {
                main_source = Some(block);
            }

            // Start new module
            let trimmed_path = rest.trim().trim_end_matches(" --- //");
            current_module_path = Some(PathBuf::from(trimmed_path));
            current_start = line_start + line.len() + 1;
        }
    }

    // Handle final block
    let last_block = &source[current_start..];
    if let Some(path) = current_module_path {
        modules.push((last_block, path));
    } else {
        main_source = Some(last_block);
    }

    // Prepare module references for compiler
    let module_refs: Vec<(&str, FileName)> =
        modules.iter().map(|(src, path)| (*src, FileName::Custom(path.to_string_lossy().into()))).collect();

    let filename = FileName::Custom("linter-test".into());
    (main_source.filter(|s| !s.trim().is_empty()).map(|m| (m, filename)), module_refs)
}

fn run_test(test: &str, handler: &Handler) -> Result<(), ()> {
    let (main, modules) = split_modules(test);

    let mut linter = Linter::new(None, handler.clone(), false, IndexMap::new(), NetworkName::TestnetV0);
    handler.extend_if_error(linter.lint(main, modules.as_slice()))?;

    if handler.err_count() != 0 {
        return Err(());
    }

    Ok(())
}

fn runner(source: &str) -> String {
    let buf = BufferEmitter::new();
    let handler = Handler::new(buf.clone());

    create_session_if_not_set_then(|_| match run_test(source, &handler) {
        Ok(_) => format!("{}", buf.extract_warnings()),
        Err(_) => format!("{}{}", buf.extract_errs(), buf.extract_warnings()),
    })
}

#[test]
#[serial]
fn test_linter() {
    leo_test_framework::run_tests("linter", runner);
}
