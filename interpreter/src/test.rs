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

fn runner_leo_test(test: &str) -> String {
    if !test.contains("// --- Next Module:") {
        create_session_if_not_set_then(|_| {
            let tempdir = tempfile::tempdir().expect("tempdir");
            let mut filename = PathBuf::from(tempdir.path());
            filename.push("main.leo");
            fs::write(&filename, test).expect("write failed");

            let private_key: PrivateKey<TestnetV0> =
                PrivateKey::from_str(TEST_PRIVATE_KEY).expect("should be able to parse private key");
            let address = Address::try_from(&private_key).expect("should be able to create address");
            let empty: [&PathBuf; 0] = [];
            let mut interpreter = Interpreter::new([filename].iter(), empty, address, 0, NetworkName::TestnetV0)
                .expect("creating interpreter");
            match interpreter.action(InterpreterAction::LeoInterpretOver("test.aleo/main()".into())) {
                Err(e) => format!("{e}\n"),
                Ok(None) => "no value received\n".to_string(),
                Ok(Some(v)) => format!("{v}\n"),
            }
        })
    } else {
        create_session_if_not_set_then(|_| {
            let tempdir = tempfile::tempdir().expect("tempdir");

            // === Step 1: Parse modules and main source ===
            let lines = test.lines().peekable();
            let mut main_source = String::new();
            let mut modules = Vec::new();

            let mut current_module_path: Option<String> = None;
            let mut current_module_source = String::new();

            for line in lines {
                if let Some(rest) = line.strip_prefix("// --- Next Module: ") {
                    if let Some(path) = current_module_path.take() {
                        modules.push((current_module_source.clone(), path));
                        current_module_source.clear();
                    } else {
                        main_source = current_module_source.clone();
                        current_module_source.clear();
                    }

                    let mut path = rest.trim().trim_end_matches(" --- //").to_string();
                    path.push_str(".leo");
                    current_module_path = Some(path);
                } else {
                    current_module_source.push_str(line);
                    current_module_source.push('\n');
                }
            }

            // Handle last segment
            if let Some(path) = current_module_path {
                modules.push((current_module_source.clone(), path));
            } else {
                main_source = current_module_source;
            }

            // === Step 2: Sort modules by path depth ===
            modules.sort_by(|(_, a), (_, b)| {
                let a_depth = std::path::Path::new(a).components().count();
                let b_depth = std::path::Path::new(b).components().count();
                b_depth.cmp(&a_depth)
            });

            // === Step 3: Write all files to shared tempdir ===
            let mut filenames = Vec::new();

            // Write main source to main.leo
            let mut main_path = PathBuf::from(tempdir.path());
            main_path.push("main.leo");
            std::fs::write(&main_path, main_source).expect("write main failed");
            filenames.push(main_path.clone());

            // Write module sources
            for (source, path) in modules {
                let mut full_path = PathBuf::from(tempdir.path());
                full_path.push(&path);

                // Ensure parent directories exist
                if let Some(parent) = full_path.parent() {
                    std::fs::create_dir_all(parent).expect("create_dir_all failed");
                }

                std::fs::write(&full_path, source).expect("write module failed");
                filenames.push(full_path);
            }

            // === Step 4: Run interpreter ===
            let private_key: PrivateKey<TestnetV0> =
                PrivateKey::from_str(TEST_PRIVATE_KEY).expect("should be able to parse private key");
            let address = Address::try_from(&private_key).expect("should be able to create address");

            let empty: [&PathBuf; 0] = [];

            let mut interpreter = Interpreter::new(filenames.iter(), empty, address, 0, NetworkName::TestnetV0)
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
