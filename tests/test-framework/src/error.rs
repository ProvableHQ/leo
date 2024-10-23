// Copyright (C) 2019-2024 Aleo Systems Inc.
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

use toml::Value;

use crate::test::TestExpectationMode;

pub struct TestFailure {
    pub path: String,
    pub errors: Vec<TestError>,
}

#[derive(Debug)]
pub enum TestError {
    Panicked { test: String, index: usize, error: String },
    UnexpectedOutput { test: String, index: usize, expected: Value, output: Value },
    PassedAndShouldntHave { test: String, index: usize },
    FailedAndShouldntHave { test: String, index: usize, error: String },
    UnexpectedError { test: String, index: usize, expected: String, output: String },
    MismatchedTestExpectationLength,
    MissingTestConfig,
}

/// Make a user-readable string representing this Value.
///
/// TOML only supports tables as the top level type,
/// so we handle arrays on our own.
fn toml_to_string(x: &Value) -> String {
    if let Some(array) = x.as_array() {
        let mut s = String::new();
        for value in array.iter() {
            s.push_str(&toml::to_string(value).expect("serialization failed"));
            s.push('\n');
        }
        s
    } else {
        toml::to_string(x).expect("serialization failed")
    }
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let format_test =
            |test: &str| -> String { if test.len() > 50 { String::new() } else { format!("\n\n{test}\n\n") } };
        match self {
            TestError::Panicked { test, index, error } => {
                write!(f, "test #{}: {}encountered a rust panic:\n{}", index + 1, format_test(test), error)
            }
            TestError::UnexpectedOutput { test, index, expected, output } => {
                write!(
                    f,
                    "test #{}: {}expected\n{}\ngot\n{}",
                    index + 1,
                    format_test(test),
                    toml_to_string(expected),
                    toml_to_string(output),
                )
            }
            TestError::PassedAndShouldntHave { test, index } => {
                write!(f, "test #{}: {}passed and shouldn't have", index + 1, format_test(test))
            }
            TestError::FailedAndShouldntHave { test, index, error } => {
                write!(f, "test #{}: {}failed and shouldn't have:\n{}", index + 1, format_test(test), error)
            }
            TestError::UnexpectedError { test, expected, output, index } => {
                write!(f, "test #{}: {}expected error\n{}\ngot\n{}", index + 1, format_test(test), expected, output)
            }
            TestError::MismatchedTestExpectationLength => write!(f, "invalid number of test expectations"),
            TestError::MissingTestConfig => write!(f, "missing test config"),
        }
    }
}

pub fn emit_errors(
    test: &str,
    output: &Result<Result<Value, String>, String>,
    mode: &TestExpectationMode,
    expected_output: Option<Value>,
    test_index: usize,
) -> Option<TestError> {
    match (output, mode) {
        (Err(e), _) => Some(TestError::Panicked { test: test.to_string(), index: test_index, error: e.to_string() }),
        (Ok(Ok(output)), TestExpectationMode::Pass) => {
            // passed and should have
            if let Some(expected_output) = expected_output.as_ref() {
                if output != expected_output {
                    // invalid output
                    return Some(TestError::UnexpectedOutput {
                        test: test.to_string(),
                        index: test_index,
                        expected: expected_output.clone(),
                        output: output.clone(),
                    });
                }
            }
            None
        }
        (Ok(Ok(_tokens)), TestExpectationMode::Fail) => {
            Some(TestError::PassedAndShouldntHave { test: test.to_string(), index: test_index })
        }
        (Ok(Err(err)), TestExpectationMode::Pass) => {
            Some(TestError::FailedAndShouldntHave { test: test.to_string(), error: err.to_string(), index: test_index })
        }
        (Ok(Err(err)), TestExpectationMode::Fail) => {
            let expected_output: Option<String> =
                expected_output.map(|x| if let Value::String(s) = x { s } else { panic!("output should be string") });
            if let Some(expected_output) = expected_output {
                if err != &expected_output {
                    // invalid output
                    return Some(TestError::UnexpectedError {
                        test: test.to_string(),
                        expected: expected_output,
                        output: err.to_string(),
                        index: test_index,
                    });
                }
            }
            None
        }
        (Ok(_), TestExpectationMode::Skip) => None,
    }
}
