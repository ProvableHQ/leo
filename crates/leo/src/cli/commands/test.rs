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
        (LeoBuild { env_override: self.env_override.clone(), options, rename: None }).execute(context)
    }

    fn apply(self, _: Context, input: Self::Input) -> Result<Self::Output> {
        handle_test(self, input)
    }

    fn execute(self, context: Context) -> Result<Self::Output> {
        // Check for workspace mode before falling through to the default
        // prelude+apply flow, because test needs to build and test each
        // member independently.
        match context.resolve_targets()? {
            Some((_, targets)) if targets.len() > 1 => {
                let mut aggregate = TestOutput::default();
                for target in &targets {
                    let member_name = target.file_name().and_then(|n| n.to_str()).unwrap_or("?");
                    println!("\n--- workspace member '{member_name}' ---");

                    let member_ctx = context.with_path(target.clone());

                    // Build with tests.
                    let mut opts = self.compiler_options.clone();
                    opts.build_tests = true;
                    let package = (LeoBuild { env_override: self.env_override.clone(), options: opts, rename: None })
                        .execute(member_ctx)?;

                    // Run tests for this member.
                    let member_test = LeoTest {
                        test_name: self.test_name.clone(),
                        prove: self.prove,
                        compiler_options: self.compiler_options.clone(),
                        env_override: self.env_override.clone(),
                    };
                    let result = handle_test(member_test, package)?;
                    aggregate.passed += result.passed;
                    aggregate.failed += result.failed;
                    aggregate.tests.extend(result.tests);
                }
                Ok(aggregate)
            }
            _ => {
                // Single target or no workspace - use the normal flow.
                let input = self.prelude(context.clone())?;
                let span = self.log_span();
                let span = span.enter();
                let out = self.apply(context, input);
                drop(span);
                out
            }
        }
    }
}

struct TestFunction {
    /// Source file the test lives in, e.g. `test_larger.leo`; used as the nextest-style "binary".
    file: String,
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

        let file = source.file_name().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default();

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

        // A test is a single standalone file; its `tests/` siblings are independent programs,
        // so parse only this file rather than scanning the directory for modules.
        let ast = if unit.kind.is_test() {
            compiler.parse_program_from_file(source)
        } else {
            compiler.parse_program_from_directory(source, directory.join("src"))
        };
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
                    file: file.clone(),
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
        return Err(crate::errors::custom("`leo test` is not supported for library packages.").into());
    }

    // Get the private key.
    let _private_key = PrivateKey::<TestnetV0>::from_str(TEST_PRIVATE_KEY)?;

    let network = command.env_override.network.unwrap_or(NetworkName::TestnetV0);
    let test_functions = discover_test_functions(&package, &command.test_name, network)?;

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
            // Libraries have no bytecode - their consts are inlined into the main program.
            if unit.kind.is_library() {
                return None;
            }
            let bytecode = match &unit.data {
                ProgramData::Bytecode(c) => c.clone(),
                ProgramData::SourcePath { .. } => {
                    // This was not a network dependency, so get its bytecode from its build directory.
                    let aleo_path = package.unit_bytecode_path(&unit.name.to_string());
                    fs::read_to_string(&aleo_path)
                        .unwrap_or_else(|e| panic!("Failed to read Aleo file at {}: {}", aleo_path.display(), e))
                }
            };
            Some(run::Program { bytecode, name: unit.name.to_string() })
        })
        .collect();

    // Per-test metadata, indexed the same as `cases` / the outcomes streamed back below.
    let should_fails: Vec<bool> = test_functions.iter().map(|tf| tf.should_fail).collect();
    let display_names: Vec<String> = test_functions.iter().map(|tf| format!("{}::{}", tf.file, tf.function)).collect();
    let qualified_names: Vec<String> =
        test_functions.iter().map(|tf| format!("{}.aleo/{}", tf.program, tf.function)).collect();

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

    let total = cases.len();
    if total == 0 {
        println!("No tests run.");
        return Ok(TestOutput::default());
    }

    // Report results in a nextest-style layout, streaming each test as it finishes.
    let plural = if total == 1 { "" } else { "s" };
    println!();
    println!("{} {total} test{plural}", gutter("Running").green().bold());

    let mut tests = Vec::with_capacity(total);
    let mut passed = 0usize;
    let mut failures: Vec<(usize, String)> = Vec::new();

    // Print each test's result as it finishes. We don't show a transient `RUNNING` line: log output
    // (e.g. under `-d`) can print between a test's start and finish, which would leave a cursor-based
    // in-place rewrite overwriting the wrong line.
    run::run_with_ledger(
        &run::Config { seed: 0, start_height: None, programs, skip_proving: !command.prove },
        &cases,
        |index, outcomes| {
            let outcome = &outcomes[0];
            let should_fail = should_fails[index];
            let display = &display_names[index];

            let message = match (&outcome.status, should_fail) {
                (run::ExecutionStatus::Accepted, false) => None,
                (run::ExecutionStatus::Accepted, true) => Some("test succeeded when failure was expected".to_string()),
                (_, true) => None,
                (_, false) => Some(format!("{} -- {}", outcome.status, outcome.outcome.output)),
            };

            match message {
                Some(err) => {
                    println!("{} {display}", gutter("FAIL").red().bold());
                    failures.push((index, err.clone()));
                    tests.push(TestResult { name: qualified_names[index].clone(), passed: false, error: Some(err) });
                }
                None => {
                    passed += 1;
                    println!("{} {display}", gutter("PASS").green().bold());
                    tests.push(TestResult { name: qualified_names[index].clone(), passed: true, error: None });
                }
            }
        },
    )?;

    let failed = total - passed;
    println!("{}", "─".repeat(24).dimmed());
    let summary = gutter("Summary");
    let summary = if failed == 0 { summary.green().bold() } else { summary.red().bold() };
    println!("{summary} {total} test{plural} run: {passed} passed, {failed} failed");
    for (index, err) in &failures {
        println!("{} {}\n{:>14}{}", gutter("FAIL").red().bold(), display_names[*index], "", err.dimmed());
    }

    Ok(TestOutput { passed, failed, tests })
}

/// Right-align a status verb into nextest's 12-column gutter. Padding is applied before coloring so
/// ANSI escapes don't throw off the alignment.
fn gutter(word: &str) -> String {
    format!("{word:>12}")
}
