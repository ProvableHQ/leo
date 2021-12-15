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

use std::fmt;

use serde_yaml::Value;

use crate::runner::sanitize_output;
use crate::{test::TestExpectationMode, Runner};

pub struct TestFailure {
    pub path: String,
    pub errors: Vec<TestError>,
}

#[derive(Debug)]
pub enum TestError {
    UnexpectedOutput {
        index: usize,
        expected: Value,
        output: Value,
    },
    PassedAndShouldntHave {
        index: usize,
    },
    FailedAndShouldntHave {
        index: usize,
        error: String,
    },
    UnexpectedError {
        index: usize,
        expected: String,
        output: String,
    },
    MismatchedTestExpectationLength,
    MissingTestConfig,
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestError::UnexpectedOutput {
                index,
                expected,
                output,
            } => {
                write!(
                    f,
                    "test #{} expected\n{}\ngot\n{}",
                    index + 1,
                    serde_yaml::to_string(&expected).expect("serialization failed"),
                    serde_yaml::to_string(&output).expect("serialization failed")
                )
            }
            TestError::PassedAndShouldntHave { index } => write!(f, "test #{} passed and shouldn't have", index + 1),
            TestError::FailedAndShouldntHave { index, error } => {
                write!(f, "test #{} failed and shouldn't have:\n{}", index + 1, error)
            }
            TestError::UnexpectedError {
                expected,
                output,
                index,
            } => {
                write!(f, "test #{} expected error\n{}\ngot\n{}", index + 1, expected, output)
            }
            TestError::MismatchedTestExpectationLength => write!(f, "invalid number of test expectations"),
            TestError::MissingTestConfig => write!(f, "missing test config"),
        }
    }
}

pub fn emit_errors<T: Runner>(
    runner: &T,
    output: Result<&Value, &str>,
    mode: &TestExpectationMode,
    expected_output: Option<Value>,
    test_index: usize,
) -> Option<TestError> {
    match (output, mode) {
        (Ok(output), TestExpectationMode::Pass) => {
            // passed and should have
            if let Some(expected_output) = expected_output.as_ref() {
                if !runner.compare_output(expected_output, output) {
                    // invalid output
                    return Some(TestError::UnexpectedOutput {
                        index: test_index,
                        expected: expected_output.clone(),
                        output: output.clone(),
                    });
                }
            }
            None
        }
        (Ok(_tokens), TestExpectationMode::Fail) => Some(TestError::PassedAndShouldntHave { index: test_index }),
        (Err(err), TestExpectationMode::Pass) => Some(TestError::FailedAndShouldntHave {
            error: err.to_string(),
            index: test_index,
        }),
        (Err(err), TestExpectationMode::Fail) => {
            let expected_output: Option<String> =
                expected_output.map(|x| serde_yaml::from_value(x).expect("test expectation deserialize failed"));
            if let Some(expected_output) = expected_output.as_deref() {
                let sanitized = sanitize_output(err);
                if sanitized != expected_output {
                    // invalid output
                    return Some(TestError::UnexpectedError {
                        expected: expected_output.to_string(),
                        output: sanitized,
                        index: test_index,
                    });
                }
            }
            None
        }
    }
}
