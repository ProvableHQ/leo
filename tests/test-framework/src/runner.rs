// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use serde_yaml::Value;
use std::{
    any::Any,
    panic::{self, RefUnwindSafe, UnwindSafe},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
};

use crate::{error::*, fetch::find_tests, output::TestExpectation, test::*};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParseType {
    Line,
    ContinuousLines,
    Whole,
}

pub struct Test {
    pub name: String,
    pub content: String,
    pub path: PathBuf,
    pub config: TestConfig,
}

pub trait Namespace: UnwindSafe + RefUnwindSafe {
    fn parse_type(&self) -> ParseType;

    fn run_test(&self, test: Test) -> Result<Value, String>;
}

pub trait Runner {
    fn resolve_namespace(&self, name: &str) -> Option<Box<dyn Namespace>>;
}

fn is_env_var_set(var: &str) -> bool {
    std::env::var(var).unwrap_or_else(|_| "".to_string()).trim().is_empty()
}

fn set_hook() -> Arc<Mutex<Option<String>>> {
    let panic_buf = Arc::new(Mutex::new(None));
    let thread_id = thread::current().id();
    panic::set_hook({
        let panic_buf = panic_buf.clone();
        Box::new(move |e| {
            if thread::current().id() == thread_id {
                if !is_env_var_set("RUST_BACKTRACE") {
                    *panic_buf.lock().unwrap() = Some(format!("{:?}", backtrace::Backtrace::new()));
                } else {
                    *panic_buf.lock().unwrap() = Some(e.to_string());
                }
            } else {
                println!("{e}")
            }
        })
    });
    panic_buf
}

fn take_hook(
    output: Result<Result<Value, String>, Box<dyn Any + Send>>,
    panic_buf: Arc<Mutex<Option<String>>>,
) -> Result<Result<Value, String>, String> {
    let _ = panic::take_hook();
    output.map_err(|_| panic_buf.lock().unwrap().take().expect("failed to get panic message"))
}

pub struct TestCases {
    tests: Vec<(PathBuf, String)>,
    path_prefix: PathBuf,
    expectation_category: String,
    fail_categories: Vec<TestFailure>,
}

impl TestCases {
    fn new(expectation_category: &str, additional_check: impl Fn(&TestConfig) -> bool) -> (Self, Vec<TestConfig>) {
        let mut path_prefix = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path_prefix.push("../../tests");

        let mut new = Self {
            tests: Vec::new(),
            path_prefix,
            expectation_category: expectation_category.to_string(),
            fail_categories: Vec::new(),
        };
        let tests = new.load_tests(additional_check);
        (new, tests)
    }

    fn load_tests(&mut self, additional_check: impl Fn(&TestConfig) -> bool) -> Vec<TestConfig> {
        let mut configs = Vec::new();

        let mut test_path = self.path_prefix.clone();
        test_path.push("tests");
        test_path.push(&self.expectation_category);
        if let Ok(p) = std::env::var("TEST_FILTER") {
            test_path.push(p);
        }

        self.tests = find_tests(&test_path)
            .filter(|(path, content)| match extract_test_config(content) {
                None => {
                    self.fail_categories.push(TestFailure {
                        path: path.to_str().unwrap_or("").to_string(),
                        errors: vec![TestError::MissingTestConfig],
                    });
                    true
                }
                Some(cfg) => {
                    let res = additional_check(&cfg);
                    configs.push(cfg);
                    res
                }
            })
            .collect();

        configs
    }

    pub(crate) fn process_tests<P, O>(&mut self, configs: Vec<TestConfig>, mut process: P) -> Vec<O>
    where
        P: FnMut(&mut Self, (&Path, &str, &str, TestConfig)) -> O,
    {
        std::env::remove_var("LEO_BACKTRACE"); // always remove backtrace so it doesn't clog output files
        std::env::set_var("LEO_TESTFRAMEWORK", "true");

        let mut output = Vec::new();
        for ((path, content), config) in self.tests.clone().iter().zip(configs.into_iter()) {
            let test_name = path.file_stem().expect("no file name for test").to_str().unwrap().to_string();

            let end_of_header = content.find("*/").expect("failed to find header block in test");
            let content = &content[end_of_header + 2..];

            output.push(process(self, (path, content, &test_name, config)));
        }
        output
    }

    fn load_expectations(&self, path: &Path) -> (PathBuf, Option<TestExpectation>) {
        let test_dir = [env!("CARGO_MANIFEST_DIR"), "../../tests"].iter().collect::<PathBuf>();
        let relative_path = path
            .strip_prefix(&test_dir)
            .expect("path error for test")
            .strip_prefix("tests")
            .expect("path error for test");

        let expectation_path = test_dir
            .join("expectations")
            .join(relative_path.parent().expect("no parent for test"))
            .join(relative_path.file_name().expect("no file name for test"))
            .with_extension("out");

        if expectation_path.exists() {
            if !is_env_var_set("CLEAR_LEO_TEST_EXPECTATIONS") {
                (expectation_path, None)
            } else {
                let raw = std::fs::read_to_string(&expectation_path).expect("failed to read expectations file");
                (expectation_path, Some(serde_yaml::from_str(&raw).expect("invalid yaml in expectations file")))
            }
        } else {
            (expectation_path, None)
        }
    }
}

pub fn run_tests<T: Runner>(runner: &T, expectation_category: &str) {
    let (mut cases, configs) = TestCases::new(expectation_category, |_| true);

    let mut pass_categories = 0;
    let mut pass_tests = 0;
    let mut fail_tests = 0;

    let mut outputs = vec![];
    cases.process_tests(configs, |cases, (path, content, test_name, config)| {
        let namespace = match runner.resolve_namespace(&config.namespace) {
            Some(ns) => ns,
            None => return,
        };

        let (expectation_path, expectations) = cases.load_expectations(path);

        let tests = match namespace.parse_type() {
            ParseType::Line => crate::fetch::split_tests_one_line(content).into_iter().map(|x| x.to_string()).collect(),
            ParseType::ContinuousLines => crate::fetch::split_tests_two_line(content),
            ParseType::Whole => vec![content.to_string()],
        };

        let mut errors = vec![];
        if let Some(expectations) = expectations.as_ref() {
            if tests.len() != expectations.outputs.len() {
                errors.push(TestError::MismatchedTestExpectationLength);
            }
        }

        let mut new_outputs = vec![];
        let mut expected_output = expectations.as_ref().map(|x| x.outputs.iter());
        for (i, test) in tests.into_iter().enumerate() {
            let expected_output = expected_output.as_mut().and_then(|x| x.next()).cloned();
            println!("running test {test_name} @ '{}'", path.to_str().unwrap());
            let panic_buf = set_hook();
            let leo_output = panic::catch_unwind(|| {
                namespace.run_test(Test {
                    name: test_name.to_string(),
                    content: test.clone(),
                    path: path.into(),
                    config: config.clone(),
                })
            });
            let output = take_hook(leo_output, panic_buf);
            if let Some(error) = emit_errors(&test, &output, &config.expectation, expected_output, i) {
                fail_tests += 1;
                errors.push(error);
            } else {
                pass_tests += 1;
                new_outputs.push(
                    output
                        .unwrap()
                        .as_ref()
                        .map(|x| serde_yaml::to_value(x).expect("serialization failed"))
                        .unwrap_or_else(|e| Value::String(e.clone())),
                );
            }
        }

        if errors.is_empty() {
            if expectations.is_none() {
                outputs.push((expectation_path, TestExpectation {
                    namespace: config.namespace,
                    expectation: config.expectation,
                    outputs: new_outputs,
                }));
            }
            pass_categories += 1;
        } else {
            cases.fail_categories.push(TestFailure { path: path.to_str().unwrap().to_string(), errors })
        }
    });

    if !cases.fail_categories.is_empty() {
        for (i, fail) in cases.fail_categories.iter().enumerate() {
            println!("\n\n-----------------TEST #{} FAILED (and shouldn't have)-----------------", i + 1);
            println!("File: {}", fail.path);
            for error in &fail.errors {
                println!("{error}");
            }
        }
        panic!(
            "failed {}/{} tests in {}/{} categories",
            pass_tests,
            fail_tests + pass_tests,
            cases.fail_categories.len(),
            cases.fail_categories.len() + pass_categories
        );
    } else {
        for (path, new_expectation) in outputs {
            std::fs::create_dir_all(path.parent().unwrap()).expect("failed to make test expectation parent directory");
            std::fs::write(
                &path,
                serde_yaml::to_string(&new_expectation).expect("failed to serialize expectation yaml"),
            )
            .expect("failed to write expectation file");
        }
        println!(
            "passed {}/{} tests in {}/{} categories",
            pass_tests,
            fail_tests + pass_tests,
            pass_categories,
            pass_categories
        );
    }

    std::env::remove_var("LEO_TESTFRAMEWORK");
}

/// returns (name, content) for all benchmark samples
pub fn get_benches() -> Vec<(String, String)> {
    let (mut cases, configs) = TestCases::new("compiler", |config| {
        (&config.namespace == "Bench" && config.expectation == TestExpectationMode::Pass)
            || ((&config.namespace == "Compile" || &config.namespace == "Execute")
                && !matches!(config.expectation, TestExpectationMode::Fail | TestExpectationMode::Skip))
    });

    cases.process_tests(configs, |_, (_, content, test_name, _)| (test_name.to_string(), content.to_string()))
}
