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

use super::*;

use leo_ast::{NetworkName, TEST_PRIVATE_KEY};
use leo_compiler::run;
use leo_package::{Package, ProgramData};
use leo_span::{Symbol, sym};

use snarkvm::prelude::TestnetV0;

use colored::Colorize as _;
use std::fs;

/// Test a leo program.
#[derive(Parser, Debug)]
pub struct LeoTest {
    #[clap(
        name = "TEST_NAME",
        help = "If specified, run only tests whose qualified name matches against this string.",
        default_value = ""
    )]
    pub(crate) test_name: String,

    #[clap(long, help = "Run all tests with full proof generation.", default_value = "false")]
    pub(crate) prove: bool,

    #[clap(flatten)]
    pub(crate) compiler_options: BuildOptions,
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
}

impl Command for LeoTest {
    type Input = <LeoBuild as Command>::Output;
    type Output = TestOutput;

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        let mut options = self.compiler_options.clone();
        options.build_tests = true;
        (LeoBuild { env_override: self.env_override.clone(), options }).execute(context)
    }

    fn apply(self, _: Context, input: Self::Input) -> Result<Self::Output> {
        handle_test(self, input)
    }
}

struct TestFunction {
    program: String,
    function: String,
    should_fail: bool,
    private_key: Option<String>,
}

/// Discover `@test`-annotated entry point functions from the compiled package.
///
/// Walks the Leo source files, parses them, and extracts functions with the
/// `@test` annotation that are entry points (transitions).
fn discover_test_functions(package: &Package, match_str: &str, network: NetworkName) -> Result<Vec<TestFunction>> {
    use indexmap::IndexMap;
    use leo_ast::NodeBuilder;
    use leo_compiler::Compiler;
    use leo_errors::Handler;
    use std::rc::Rc;

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

fn handle_test(command: LeoTest, package: Package) -> Result<TestOutput> {
    if package.compilation_units.last().map(|p| p.kind.is_library()).unwrap_or(false) {
        return Err(CliError::custom("`leo test` is not supported for library packages.").into());
    }

    // Get the private key.
    let _private_key = PrivateKey::<TestnetV0>::from_str(TEST_PRIVATE_KEY)?;

    let network = command.env_override.network.unwrap_or(NetworkName::TestnetV0);
    let test_functions = discover_test_functions(&package, &command.test_name, network)?;

    let program_name_symbol = Symbol::intern(&package.manifest.program);
    let build_directory = package.build_directory();

    let credits = Symbol::intern("credits.aleo");

    // Get bytecode and name for all programs, either directly or from the filesystem if they were compiled.
    let programs: Vec<run::Program> = package
        .compilation_units
        .iter()
        .filter_map(|unit| {
            // Skip credits.aleo so we don't try to deploy it again.
            if unit.name == credits {
                return None;
            }
            // Libraries have no bytecode — their consts are inlined into the main program.
            if unit.kind.is_library() {
                return None;
            }
            let bytecode = match &unit.data {
                ProgramData::Bytecode(c) => c.clone(),
                ProgramData::SourcePath { .. } => {
                    // This was not a network dependency, so get its bytecode from the filesystem.
                    let aleo_path = if unit.name == program_name_symbol {
                        build_directory.join("main.aleo")
                    } else {
                        package.imports_directory().join(format!("{}", unit.name))
                    };
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
            }]
        })
        .collect();

    let outcomes = run::run_with_ledger(
        &run::Config { seed: 0, start_height: None, programs, skip_proving: !command.prove },
        &cases,
    )?
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

    // Report results.
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
