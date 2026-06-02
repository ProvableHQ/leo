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

//! Native-only `leo test` core. The `LeoTest` clap struct in `crates/leo`
//! resolves env defaults and forwards to [`handle_test`].

#![cfg(not(target_arch = "wasm32"))]

use crate::errors;

use leo_ast::{NetworkName, NodeBuilder, TEST_PRIVATE_KEY};
use leo_compiler::{Compiler, run};
use leo_errors::{Handler, Result};
use leo_package::{Package, ProgramData};
use leo_span::{Symbol, sym};

use snarkvm::prelude::{PrivateKey, TestnetV0};

use colored::Colorize as _;
use indexmap::IndexMap;
use serde::Serialize;
use std::{fs, rc::Rc, str::FromStr as _};

/// Output for `leo test`.
#[derive(Serialize, Default)]
pub struct TestOutput {
    pub passed: usize,
    pub failed: usize,
    pub tests: Vec<TestResult>,
}

/// A single test result.
#[derive(Serialize)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

struct TestFunction {
    program: String,
    function: String,
    should_fail: bool,
    private_key: Option<String>,
}

/// Discover `@test`-annotated entry-point functions in the compiled package.
fn discover_test_functions(package: &Package, match_str: &str, network: NetworkName) -> Result<Vec<TestFunction>> {
    let private_key_symbol = Symbol::intern("private_key");
    let mut test_functions = Vec::new();

    for unit in &package.compilation_units {
        let ProgramData::SourcePath { directory, source } = &unit.data else {
            continue;
        };

        let source_dir =
            if unit.kind.is_test() { source.parent().unwrap().to_path_buf() } else { directory.join("src") };

        let handler = Handler::default();
        let node_builder = Rc::new(NodeBuilder::default());

        let mut compiler = Compiler::new(
            None,
            unit.kind.is_test(),
            handler,
            node_builder,
            "/unused".into(),
            None,
            IndexMap::new(),
            network,
        );

        let ast = compiler.parse_program_from_directory(source, &source_dir);
        let ast = match ast {
            Ok(ast) => ast,
            Err(_) => continue,
        };

        for scope in ast.program_scopes.values() {
            let program_name = scope.program_id.name.to_string();

            for (_, function) in &scope.functions {
                let has_test = function.annotations.iter().any(|a| a.identifier.name == sym::test);
                if !has_test {
                    continue;
                }

                if !function.variant.is_entry() {
                    continue;
                }

                let qualified = format!("{program_name}/{}", function.identifier);
                if !match_str.is_empty() && !qualified.contains(match_str) {
                    continue;
                }

                let should_fail = function.annotations.iter().any(|a| a.identifier.name == sym::should_fail);

                let private_key = function
                    .annotations
                    .iter()
                    .find(|a| a.identifier.name == sym::test)
                    .and_then(|a| a.map.get(&private_key_symbol).cloned());

                test_functions.push(TestFunction {
                    program: program_name.clone(),
                    function: function.identifier.to_string(),
                    should_fail,
                    private_key,
                });
            }
        }
    }

    Ok(test_functions)
}

/// Drive a `leo test`. `package` is the already-built package with tests.
pub fn handle_test(package: Package, test_name: &str, network: NetworkName, prove: bool) -> Result<TestOutput> {
    if package.compilation_units.last().map(|p| p.kind.is_library()).unwrap_or(false) {
        return Err(errors::custom("`leo test` is not supported for library packages.").into());
    }

    // The fixed-key parse is here to fail fast if `TEST_PRIVATE_KEY` ever
    // breaks against snarkVM; the actual key threading happens inside
    // `run::run_with_ledger`.
    let _private_key = PrivateKey::<TestnetV0>::from_str(TEST_PRIVATE_KEY)?;

    let test_functions = discover_test_functions(&package, test_name, network)?;

    let credits = Symbol::intern("credits.aleo");

    let programs: Vec<run::Program> = package
        .compilation_units
        .iter()
        .filter_map(|unit| {
            if unit.name == credits {
                return None;
            }
            if unit.kind.is_library() {
                return None;
            }
            let bytecode = match &unit.data {
                ProgramData::Bytecode(c) => c.clone(),
                ProgramData::SourcePath { .. } => {
                    let aleo_path = package.unit_bytecode_path(&unit.name.to_string());
                    fs::read_to_string(&aleo_path)
                        .unwrap_or_else(|e| panic!("Failed to read Aleo file at {}: {}", aleo_path.display(), e))
                }
            };
            Some(run::Program { bytecode, name: unit.name.to_string() })
        })
        .collect();

    let should_fails: Vec<bool> = test_functions.iter().map(|tf| tf.should_fail).collect();
    let cases: Vec<Vec<run::Case>> = test_functions
        .into_iter()
        .map(|tf| {
            vec![run::Case {
                program_name: format!("{}.aleo", tf.program),
                function: tf.function,
                private_key: tf.private_key,
                input: Vec::new(),
                seed_mapping: Vec::new(),
            }]
        })
        .collect();

    let outcomes =
        run::run_with_ledger(&run::Config { seed: 0, start_height: None, programs, skip_proving: !prove }, &cases)?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

    let results: Vec<_> = outcomes
        .into_iter()
        .zip(should_fails)
        .map(|(outcome, should_fail)| {
            let run::ExecutionOutcome { outcome: inner, status, .. } = outcome;

            let message = match (&status, should_fail) {
                (run::ExecutionStatus::Accepted, false) => None,
                (run::ExecutionStatus::Accepted, true) => Some("Test succeeded when failure was expected.".to_string()),
                (_, true) => None,
                (_, false) => Some(format!("{} -- {}", status, inner.output)),
            };

            (inner.program_name, inner.function, message)
        })
        .collect::<Vec<_>>();

    let total = results.len();
    let total_passed = results.iter().filter(|(_, _, x)| x.is_none()).count();

    let mut tests = Vec::new();

    if total == 0 {
        println!("No tests run.");
    } else {
        println!("{total_passed} / {total} tests passed.");
        let failed = "FAILED".bold().red();
        let passed = "PASSED".bold().green();

        for (program, function, case_result) in &results {
            let str_id = format!("{program}/{function}");
            if let Some(err_str) = case_result {
                println!("{failed}: {str_id:<30} | {err_str}");
                tests.push(TestResult { name: str_id, passed: false, error: Some(err_str.clone()) });
            } else {
                println!("{passed}: {str_id}");
                tests.push(TestResult { name: str_id, passed: true, error: None });
            }
        }
    }

    Ok(TestOutput { passed: total_passed, failed: total - total_passed, tests })
}
