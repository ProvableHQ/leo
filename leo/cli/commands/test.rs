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

use super::*;

use leo_ast::{NetworkName, TEST_PRIVATE_KEY};
use leo_compiler::run;
use leo_package::{Package, ProgramData};
use leo_span::Symbol;

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

fn handle_test(command: LeoTest, package: Package) -> Result<TestOutput> {
    // Get the private key.
    let private_key = PrivateKey::<TestnetV0>::from_str(TEST_PRIVATE_KEY)?;

    let leo_paths = collect_leo_paths(&package);
    let aleo_paths = collect_aleo_paths(&package);

    let (native_test_functions, interpreter_result) = leo_interpreter::find_and_run_tests(
        &leo_paths,
        &aleo_paths,
        private_key.to_string(),
        0u32,
        chrono::Utc::now().timestamp(),
        &command.test_name,
        NetworkName::TestnetV0,
    )?;

    // Now for native tests.
    let program_name = package.manifest.program.strip_suffix(".aleo").unwrap();
    let program_name_symbol = Symbol::intern(program_name);
    let build_directory = package.build_directory();

    let credits = Symbol::intern("credits");

    // Get bytecode and name for all programs, either directly or from the filesystem if they were compiled.
    let programs: Vec<run::Program> = package
        .programs
        .iter()
        .filter_map(|program| {
            // Skip credits.aleo so we don't try to deploy it again.
            if program.name == credits {
                return None;
            }
            let bytecode = match &program.data {
                ProgramData::Bytecode(c) => c.clone(),
                ProgramData::SourcePath { .. } => {
                    // This was not a network dependency, so get its bytecode from the filesystem.
                    let aleo_path = if program.name == program_name_symbol {
                        build_directory.join("main.aleo")
                    } else {
                        package.imports_directory().join(format!("{}.aleo", program.name))
                    };
                    fs::read_to_string(&aleo_path)
                        .unwrap_or_else(|e| panic!("Failed to read Aleo file at {}: {}", aleo_path.display(), e))
                }
            };
            Some(run::Program { bytecode, name: program.name.to_string() })
        })
        .collect();

    let should_fails: Vec<bool> = native_test_functions.iter().map(|test_function| test_function.should_fail).collect();
    let cases: Vec<Vec<run::Case>> = native_test_functions
        .into_iter()
        .map(|test_function| {
            // Note. We wrap each individual test in its own vector, so that they are run in insolation.
            vec![run::Case {
                program_name: format!("{}.aleo", test_function.program),
                function: test_function.function,
                private_key: test_function.private_key,
                input: Vec::new(),
            }]
        })
        .collect();

    let outcomes = run::run_with_ledger(&run::Config { seed: 0, start_height: None, programs }, &cases)?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let native_results: Vec<_> = outcomes
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

    // All tests are run. Report results.
    let total = interpreter_result.iter().count() + native_results.len();
    let total_passed = interpreter_result.iter().filter(|(_, test_result)| matches!(test_result, Ok(()))).count()
        + native_results.iter().filter(|(_, _, x)| x.is_none()).count();

    // Build structured output and print results.
    let mut tests = Vec::new();

    if total == 0 {
        println!("No tests run.");
    } else {
        println!("{total_passed} / {total} tests passed.");
        let failed = "FAILED".bold().red();
        let passed = "PASSED".bold().green();

        for (id, id_result) in interpreter_result.iter() {
            // Wasteful to make this, but fill will work.
            let str_id = format!("{id}");
            if let Err(err) = id_result {
                println!("{failed}: {str_id:<30} | {err}");
                tests.push(TestResult { name: str_id, passed: false, error: Some(err.to_string()) });
            } else {
                println!("{passed}: {str_id}");
                tests.push(TestResult { name: str_id, passed: true, error: None });
            }
        }

        for (program, function, case_result) in &native_results {
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
