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

use crate::{Interpreter, InterpreterAction};

use leo_ast::NetworkName;
use leo_span::create_session_if_not_set_then;

use snarkvm::prelude::{Address, PrivateKey, TestnetV0};

use serial_test::serial;
use std::{fs, path::PathBuf, str::FromStr as _};

pub static TEST_PRIVATE_KEY: &str = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH";

/// A special token used to separate modules in test input source code.
const MODULE_SEPARATOR: &str = "// --- Next Module:";

/// Runs a Leo test case provided as a string, with optional inlined module definitions.
///
/// # Behavior
/// - If the source contains no `MODULE_SEPARATOR`, it is treated as a standalone Leo file and executed directly.
/// - If the source contains inlined modules separated by `MODULE_SEPARATOR`, it will:
///   - Split the input into a main source and its modules,
///   - Write each source to a temporary file structure,
///   - Compile and interpret them as a single Leo program.
///
/// # Arguments
/// - `test`: The input Leo program as a string. Can include inlined modules using `MODULE_SEPARATOR`.
///
/// # Returns
/// - A string containing either the result of interpretation, an error message, or "no value received".
///
/// # Panics
/// - Panics on file I/O failure or if the Leo interpreter setup fails.
/// - Panics if the hardcoded private key is invalid.
fn runner_leo_test(test: &str) -> String {
    if !test.contains(MODULE_SEPARATOR) {
        // === Simple case: single source file ===
        create_session_if_not_set_then(|_| {
            let tempdir = tempfile::tempdir().expect("tempdir");

            // Write source to temporary main.leo file
            let filename = tempdir.path().join("main.leo");
            fs::write(&filename, test).expect("write failed");

            // Set up interpreter using testnet private key
            let private_key: PrivateKey<TestnetV0> =
                PrivateKey::from_str(TEST_PRIVATE_KEY).expect("should parse private key");
            let address = Address::try_from(&private_key).expect("should create address");

            let empty: [&PathBuf; 0] = [];
            let mut interpreter = Interpreter::new(&[(filename, vec![])], empty, address, 0, NetworkName::TestnetV0)
                .expect("creating interpreter");

            match interpreter.action(InterpreterAction::LeoInterpretOver("test.aleo/main()".into())) {
                Err(e) => format!("{e}\n"),
                Ok(None) => "no value received\n".to_string(),
                Ok(Some(v)) => format!("{v}\n"),
            }
        })
    } else {
        // === Multi-module case ===
        create_session_if_not_set_then(|_| {
            let tempdir = tempfile::tempdir().expect("tempdir");

            // === Step 1: Parse test source into main and modules ===
            let lines = test.lines().peekable();
            let mut main_source = String::new();
            let mut modules = Vec::new();

            let mut current_module_path: Option<String> = None;
            let mut current_module_source = String::new();

            for line in lines {
                if let Some(rest) = line.strip_prefix(MODULE_SEPARATOR) {
                    // Save previous module or main
                    if let Some(path) = current_module_path.take() {
                        modules.push((current_module_source.clone(), path));
                        current_module_source.clear();
                    } else {
                        main_source = current_module_source.clone();
                        current_module_source.clear();
                    }

                    // Prepare the new module path
                    let path = rest.trim().trim_end_matches(" --- //").to_string();
                    current_module_path = Some(path);
                } else {
                    current_module_source.push_str(line);
                    current_module_source.push('\n');
                }
            }

            // Save last module or main source
            if let Some(path) = current_module_path {
                modules.push((current_module_source.clone(), path));
            } else {
                main_source = current_module_source;
            }

            // === Step 2: Write all source files into the temp directory ===
            let mut module_paths = Vec::new();

            // Write main source to main.leo
            let main_path = tempdir.path().join("main.leo");
            std::fs::write(&main_path, main_source).expect("write main failed");

            // Write module files to appropriate relative paths
            for (source, path) in modules {
                let full_path = tempdir.path().join(&path);

                // Ensure parent directories exist
                if let Some(parent) = full_path.parent() {
                    std::fs::create_dir_all(parent).expect("create_dir_all failed");
                }

                std::fs::write(&full_path, source).expect("write module failed");
                module_paths.push(full_path);
            }

            // === Step 3: Run interpreter on main() ===
            let private_key: PrivateKey<TestnetV0> =
                PrivateKey::from_str(TEST_PRIVATE_KEY).expect("should parse private key");
            let address = Address::try_from(&private_key).expect("should create address");

            let empty: [&PathBuf; 0] = [];

            let mut interpreter =
                Interpreter::new(&[(main_path, module_paths)], empty, address, 0, NetworkName::TestnetV0)
                    .expect("creating interpreter");

            match interpreter.action(InterpreterAction::LeoInterpretOver("test.aleo/main()".into())) {
                Err(e) => format!("{e}\n"),
                Ok(None) => "no value received\n".to_string(),
                Ok(Some(v)) => format!("{v}\n"),
            }
        })
    }
}

#[test]
#[serial]
fn test_interpreter() {
    leo_test_framework::run_tests("interpreter-leo", runner_leo_test);
}
