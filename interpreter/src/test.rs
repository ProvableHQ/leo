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

//! These tests compare interpreter runs against previous interpreter runs.

use crate::{Element, Interpreter, InterpreterAction};

use leo_ast::{NetworkName, interpreter_value::Value};
use leo_span::{Span, Symbol, create_session_if_not_set_then};

use snarkvm::prelude::{PrivateKey, TestnetV0};

use serial_test::serial;
use std::{fs, path::PathBuf, str::FromStr as _};

pub static TEST_PRIVATE_KEY: &str = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH";

/// A special token used to separate modules in test input source code.
const MODULE_SEPARATOR: &str = "// --- Next Module:";

fn run_and_format_output(interpreter: &mut Interpreter) -> String {
    let action_result = interpreter.action(InterpreterAction::LeoInterpretOver("test.aleo/main()".into()));

    let futures =
        interpreter.cursor.futures.iter().map(|f| format!(" async call to {f}")).collect::<Vec<_>>().join("\n");

    let futures_section = if futures.is_empty() { "# Futures".to_string() } else { format!("# Futures\n{futures}") };

    let output_section = match action_result {
        Err(e) => format!("# Output\n{e}"),
        Ok(None) => "# Output\nno value received".to_string(),
        Ok(Some(v)) => format!("# Output\n{v}"),
    };

    format!("{futures_section}\n\n{output_section}\n")
}

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

            let empty: [&PathBuf; 0] = [];
            let mut interpreter = Interpreter::new(
                &[(filename, vec![])],
                empty,
                private_key.to_string(),
                0,
                chrono::Utc::now().timestamp(),
                NetworkName::TestnetV0,
            )
            .expect("creating interpreter");

            run_and_format_output(&mut interpreter)
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

            let empty: [&PathBuf; 0] = [];

            let mut interpreter = Interpreter::new(
                &[(main_path, module_paths)],
                empty,
                private_key.to_string(),
                0,
                chrono::Utc::now().timestamp(),
                NetworkName::TestnetV0,
            )
            .expect("creating interpreter");

            run_and_format_output(&mut interpreter)
        })
    }
}

#[test]
#[serial]
fn test_interpreter() {
    leo_test_framework::run_tests("interpreter-leo", runner_leo_test);
}

#[test]
fn cast_lossy_advances_instruction_index() {
    create_session_if_not_set_then(|_| {
        let tempdir = tempfile::tempdir().expect("tempdir");

        // Minimal Aleo program with a single cast.lossy instruction in a function.
        let aleo_source = r"program test_castlossy.aleo;

function cast_lossy_test:
    input r0 as field.private;
    cast.lossy r0 into r1 as u8;
    output r1 as u8.private;
";

        let filename = tempdir.path().join("test_castlossy.aleo");
        fs::write(&filename, aleo_source).expect("write failed");

        let private_key: PrivateKey<TestnetV0> = PrivateKey::from_str(TEST_PRIVATE_KEY).expect("should parse private key");

        let leo_files: [(PathBuf, Vec<PathBuf>); 0] = [];
        let aleo_files = [filename.clone()];

        let mut interpreter = Interpreter::new(
            &leo_files,
            &aleo_files,
            private_key.to_string(),
            0,
            chrono::Utc::now().timestamp(),
            NetworkName::TestnetV0,
        )
        .expect("creating interpreter");

        let program = Symbol::intern("test_castlossy");
        let function = Symbol::intern("cast_lossy_test");

        // Single field argument value.
        let arg = Value::from_str("1field").expect("parse field literal");

        interpreter
            .cursor
            .do_call(program, &[function], std::iter::once(arg), false, Span::default())
            .expect("do_call failed");

        // Ensure we have an AleoExecution frame on top of the stack.
        assert!(matches!(interpreter.cursor.frames.last().map(|f| &f.element), Some(Element::AleoExecution { .. })));

        // Step through Aleo execution a few times at most; with a single cast.lossy
        // instruction, the frame must be popped quickly if the instruction index advances.
        let mut steps = 0usize;
        while matches!(interpreter.cursor.frames.last().map(|f| &f.element), Some(Element::AleoExecution { .. }))
            && steps < 10
        {
            interpreter.cursor.step_aleo().expect("step_aleo failed");
            steps += 1;
        }

        assert!(steps <= 2, "CastLossy should complete in at most a couple of steps, got {steps}");
        assert!(
            !matches!(interpreter.cursor.frames.last().map(|f| &f.element), Some(Element::AleoExecution { .. })),
            "AleoExecution frame should be popped after executing CastLossy-only function",
        );
    })
}
