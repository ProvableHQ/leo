// Copyright (C) 2019-2021 Aleo Systems Inc.
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
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use crate::{error::*, fetch::find_tests, output::TestExpectation, test::*};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseType {
    Line,
    ContinuousLines,
    Whole,
}

pub struct Test {
    pub name: String,
    pub content: String,
    pub path: PathBuf,
    pub config: BTreeMap<String, Value>,
}

pub trait Namespace {
    fn parse_type(&self) -> ParseType;

    fn run_test(&self, test: Test) -> Result<Value, String>;
}

pub trait Runner {
    fn resolve_namespace(&self, name: &str) -> Option<Box<dyn Namespace>>;
}

pub fn run_tests<T: Runner>(runner: &T, expectation_category: &str) {
    std::env::set_var("LEO_TESTFRAMEWORK", "true");
    let mut pass_categories = 0;
    let mut pass_tests = 0;
    let mut fail_tests = 0;
    let mut fail_categories = Vec::new();

    let mut tests = Vec::new();
    let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_dir.push("../tests/");

    let mut expectation_dir = test_dir.clone();
    expectation_dir.push("expectations");

    find_tests(&test_dir, &mut tests);

    let filter = std::env::var("TEST_FILTER").unwrap_or_default();
    let filter = filter.trim();

    let mut outputs = vec![];

    for (path, content) in tests.into_iter() {
        if !filter.is_empty() && !path.contains(filter) {
            continue;
        }
        let config = extract_test_config(&content);
        if config.is_none() {
            //panic!("missing configuration for {}", path);
            // fail_categories.push(TestFailure {
            //     path,
            //     errors: vec![TestError::MissingTestConfig],
            // });
            continue;
        }
        let config = config.unwrap();
        let namespace = runner.resolve_namespace(&config.namespace);
        if namespace.is_none() {
            continue;
        }
        let namespace = namespace.unwrap();

        let path = Path::new(&path);
        let relative_path = path.strip_prefix(&test_dir).expect("path error for test");
        let mut expectation_path = expectation_dir.clone();
        expectation_path.push(expectation_category);
        expectation_path.push(relative_path.parent().expect("no parent dir for test"));
        let mut expectation_name = relative_path
            .file_name()
            .expect("no file name for test")
            .to_str()
            .unwrap()
            .to_string();
        expectation_name += ".out";
        expectation_path.push(&expectation_name);

        let test_name = relative_path
            .file_stem()
            .expect("no file name for test")
            .to_str()
            .unwrap()
            .to_string();

        let expectations: Option<TestExpectation> = if expectation_path.exists() {
            if !std::env::var("CLEAR_LEO_TEST_EXPECTATIONS")
                .unwrap_or_default()
                .trim()
                .is_empty()
            {
                None
            } else {
                let raw = std::fs::read_to_string(&expectation_path).expect("failed to read expectations file");
                Some(serde_yaml::from_str(&raw).expect("invalid yaml in expectations file"))
            }
        } else {
            None
        };

        let end_of_header = content.find("*/").expect("failed to find header block in test");
        let content = &content[end_of_header + 2..];

        let tests = match namespace.parse_type() {
            ParseType::Line => crate::fetch::split_tests_oneline(content)
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
            ParseType::ContinuousLines => crate::fetch::split_tests_twoline(content),
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
            let expected_output = expected_output.as_mut().map(|x| x.next()).flatten().cloned();
            println!("running test {} @ '{}'", test_name, path.to_str().unwrap());
            let output = namespace.run_test(Test {
                name: test_name.clone(),
                content: test.clone(),
                path: path.into(),
                config: config.extra.clone(),
            });
            if let Some(error) = emit_errors(
                output.as_ref().map_err(|x| &**x),
                &config.expectation,
                expected_output,
                i,
            ) {
                fail_tests += 1;
                errors.push(error);
            } else {
                pass_tests += 1;
                new_outputs.push(
                    output
                        .as_ref()
                        .map(|x| serde_yaml::to_value(x).expect("serialization failed"))
                        .unwrap_or_else(|e| Value::String(e.clone())),
                );
            }
        }

        if errors.is_empty() {
            if expectations.is_none() {
                outputs.push((
                    expectation_path,
                    TestExpectation {
                        namespace: config.namespace,
                        expectation: config.expectation,
                        outputs: new_outputs,
                    },
                ));
            }
            pass_categories += 1;
        } else {
            fail_categories.push(TestFailure {
                path: path.to_str().unwrap().to_string(),
                errors,
            })
        }
    }
    if !fail_categories.is_empty() {
        for (i, fail) in fail_categories.iter().enumerate() {
            println!(
                "\n\n-----------------TEST #{} FAILED (and shouldn't have)-----------------",
                i + 1
            );
            println!("File: {}", fail.path);
            for error in &fail.errors {
                println!("{}", error);
            }
        }
        panic!(
            "failed {}/{} tests in {}/{} categories",
            pass_tests,
            fail_tests + pass_tests,
            fail_categories.len(),
            fail_categories.len() + pass_categories
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
