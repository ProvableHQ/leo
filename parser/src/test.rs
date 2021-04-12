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

use std::{
    fmt,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use leo_ast::{Expression, ExpressionStatement, Program, Span, Statement, ValueExpression};
use serde_yaml::Value;
use tokenizer::Token;

use crate::{tokenizer, DeprecatedError, ParserContext, SyntaxError, TokenError};

struct TestFailure {
    path: String,
    errors: Vec<TestError>,
}

#[derive(Debug)]
enum TestError {
    UnexpectedOutput {
        index: usize,
        expected: String,
        output: String,
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
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestError::UnexpectedOutput {
                index,
                expected,
                output,
            } => {
                write!(f, "test #{} expected\n{}\ngot\n{}", index + 1, expected, output)
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
        }
    }
}

pub fn find_tests<T: AsRef<Path>>(path: T, out: &mut Vec<(String, String)>) {
    for entry in fs::read_dir(path).expect("fail to read tests").into_iter() {
        let entry = entry.expect("fail to read tests").path();
        if entry.is_dir() {
            find_tests(entry.as_path(), out);
            continue;
        } else if entry.extension().map(|x| x.to_str()).flatten().unwrap_or_default() != "leo" {
            continue;
        }
        let content = fs::read_to_string(entry.as_path()).expect("failed to read test");
        out.push((entry.as_path().to_str().unwrap_or_default().to_string(), content));
    }
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone)]
enum TestNamespace {
    Parse,
    ParseStatement,
    ParseExpression,
    Token,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone)]
enum TestExpectationMode {
    Pass,
    Fail,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct TestConfig {
    namespace: TestNamespace,
    expectation: TestExpectationMode,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct TestExpectation {
    namespace: TestNamespace,
    expectation: TestExpectationMode,
    outputs: Vec<Value>,
}

fn extract_test_config(source: &str) -> Option<TestConfig> {
    let first_comment_start = source.find("/*")?;
    let end_first_comment = source[first_comment_start + 2..].find("*/")?;
    let comment_inner = &source[first_comment_start + 2..first_comment_start + 2 + end_first_comment];
    Some(serde_yaml::from_str(comment_inner).expect("invalid test configuration"))
}

fn split_tests_oneline(source: &str) -> Vec<&str> {
    source.lines().map(|x| x.trim()).filter(|x| !x.is_empty()).collect()
}

fn split_tests_twoline(source: &str) -> Vec<String> {
    let mut out = vec![];
    let mut lines = vec![];
    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() {
            if !lines.is_empty() {
                out.push(lines.join("\n"));
            }
            lines.clear();
            continue;
        }
        lines.push(line);
    }
    let last_test = lines.join("\n");
    if !last_test.trim().is_empty() {
        out.push(last_test.trim().to_string());
    }
    out
}

fn run_individual_token_test(path: &str, source: &str) -> Result<String, String> {
    let output = tokenizer::tokenize(path, source.into());
    output
        .map(|tokens| {
            tokens
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
        })
        .map_err(|x| strip_path_syntax_error(x.into()))
}

fn not_fully_consumed(tokens: &mut ParserContext) -> Result<(), String> {
    if !tokens.has_next() {
        return Ok(());
    }
    let mut out = "did not consume all input: ".to_string();
    while tokens.has_next() {
        out.push_str(&tokens.expect_any().unwrap().to_string());
        out.push('\n');
    }
    Err(out)
}

fn run_individual_expression_test(path: &str, source: &str) -> Result<Expression, String> {
    let tokenizer = tokenizer::tokenize(path, source.into()).map_err(|x| strip_path_syntax_error(x.into()))?;
    if tokenizer
        .iter()
        .all(|x| matches!(x.token, Token::CommentLine(_) | Token::CommentBlock(_)))
    {
        return Ok(Expression::Value(ValueExpression::Implicit("".into(), Span::default())));
    }
    let mut tokens = ParserContext::new(tokenizer);

    let parsed = tokens.parse_expression().map_err(strip_path_syntax_error)?;
    not_fully_consumed(&mut tokens)?;

    Ok(parsed)
}

fn run_individual_statement_test(path: &str, source: &str) -> Result<Statement, String> {
    let tokenizer = tokenizer::tokenize(path, source.into()).map_err(|x| strip_path_syntax_error(x.into()))?;
    if tokenizer
        .iter()
        .all(|x| matches!(x.token, Token::CommentLine(_) | Token::CommentBlock(_)))
    {
        return Ok(Statement::Expression(ExpressionStatement {
            expression: Expression::Value(ValueExpression::Implicit("".into(), Span::default())),
            span: Span::default(),
        }));
    }
    let mut tokens = ParserContext::new(tokenizer);

    let parsed = tokens.parse_statement().map_err(strip_path_syntax_error)?;
    not_fully_consumed(&mut tokens)?;

    Ok(parsed)
}

fn strip_path_syntax_error(mut err: SyntaxError) -> String {
    let inner = match &mut err {
        SyntaxError::DeprecatedError(DeprecatedError::Error(x)) => x,
        SyntaxError::Error(x) => x,
        SyntaxError::TokenError(TokenError::Error(x)) => x,
    };
    inner.path = Arc::new("test".to_string());
    err.to_string()
}

fn run_individual_parse_test(path: &str, source: &str) -> Result<Program, String> {
    let tokenizer = tokenizer::tokenize(path, source.into()).map_err(|x| strip_path_syntax_error(x.into()))?;
    let mut tokens = ParserContext::new(tokenizer);

    let parsed = tokens.parse_program().map_err(strip_path_syntax_error)?;
    not_fully_consumed(&mut tokens)?;

    Ok(parsed)
}

fn emit_errors<T: PartialEq + ToString + serde::de::DeserializeOwned>(
    output: Result<&T, &str>,
    mode: &TestExpectationMode,
    expected_output: Option<Value>,
    test_index: usize,
) -> Option<TestError> {
    match (output, mode) {
        (Ok(output), TestExpectationMode::Pass) => {
            let expected_output: Option<T> =
                expected_output.map(|x| serde_yaml::from_value(x).expect("test expectation deserialize failed"));
            // passed and should have
            if let Some(expected_output) = expected_output.as_ref() {
                if output != expected_output {
                    // invalid output
                    return Some(TestError::UnexpectedOutput {
                        index: test_index,
                        expected: expected_output.to_string(),
                        output: output.to_string(),
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
                // if expected_output.is_some() && err != expected_output.as_deref().unwrap() {
                if err != expected_output {
                    // invalid output
                    return Some(TestError::UnexpectedError {
                        expected: expected_output.to_string(),
                        output: err.to_string(),
                        index: test_index,
                    });
                }
            }
            None
        }
    }
}

fn run_test(
    config: &TestConfig,
    path: &str,
    source: &str,
    expectations: Option<&TestExpectation>,
    errors: &mut Vec<TestError>,
) -> Vec<Value> {
    let end_of_header = source.find("*/").expect("failed to find header block in test");
    let source = &source[end_of_header + 2..];
    let mut outputs = vec![];
    match &config.namespace {
        TestNamespace::Token => {
            let tests = split_tests_oneline(source);
            if let Some(expectations) = expectations.as_ref() {
                if tests.len() != expectations.outputs.len() {
                    errors.push(TestError::MismatchedTestExpectationLength);
                }
            }
            let mut expected_output = expectations.as_ref().map(|x| x.outputs.iter());
            for (i, test) in tests.into_iter().enumerate() {
                let expected_output = expected_output
                    .as_mut()
                    .map(|x| x.next())
                    .flatten()
                    .map(|x| x.as_str())
                    .flatten();
                let output = run_individual_token_test(path, test);
                if let Some(error) = emit_errors(
                    output.as_ref().map_err(|x| &**x),
                    &config.expectation,
                    expected_output.map(|x| Value::String(x.to_string())),
                    i,
                ) {
                    errors.push(error);
                } else {
                    outputs.push(serde_yaml::to_value(output.unwrap_or_else(|e| e)).expect("serialization failed"));
                }
            }
        }
        TestNamespace::Parse => {
            if let Some(expectations) = expectations.as_ref() {
                if expectations.outputs.len() != 1 {
                    errors.push(TestError::MismatchedTestExpectationLength);
                }
            }
            let expected_output = expectations
                .map(|x| x.outputs.get(0))
                .flatten()
                .map(|x| serde_yaml::from_value(x.clone()).expect("invalid test expectation form"));
            let output = run_individual_parse_test(path, source);
            if let Some(error) = emit_errors(
                output.as_ref().map_err(|x| &**x),
                &config.expectation,
                expected_output,
                0,
            ) {
                errors.push(error);
            } else {
                outputs.push(
                    output
                        .map(|x| serde_yaml::to_value(x).expect("serialization failed"))
                        .unwrap_or_else(|e| serde_yaml::to_value(e).expect("serialization failed")),
                );
            }
        }
        TestNamespace::ParseStatement => {
            let tests = split_tests_twoline(source);
            if let Some(expectations) = expectations.as_ref() {
                if tests.len() != expectations.outputs.len() {
                    errors.push(TestError::MismatchedTestExpectationLength);
                }
            }
            let mut expected_output = expectations.as_ref().map(|x| x.outputs.iter());
            for (i, test) in tests.into_iter().enumerate() {
                let expected_output = expected_output
                    .as_mut()
                    .map(|x| x.next())
                    .flatten()
                    .map(|x| serde_yaml::from_value(x.clone()).expect("invalid test expectation form"));

                let output = run_individual_statement_test(path, &test);
                if let Some(error) = emit_errors(
                    output.as_ref().map_err(|x| &**x),
                    &config.expectation,
                    expected_output,
                    i,
                ) {
                    errors.push(error);
                } else {
                    outputs.push(
                        output
                            .map(|x| serde_yaml::to_value(x).expect("serialization failed"))
                            .unwrap_or_else(|e| serde_yaml::to_value(e).expect("serialization failed")),
                    );
                }
            }
        }
        TestNamespace::ParseExpression => {
            let tests = split_tests_oneline(source);
            if let Some(expectations) = expectations.as_ref() {
                if tests.len() != expectations.outputs.len() {
                    errors.push(TestError::MismatchedTestExpectationLength);
                }
            }
            let mut expected_output = expectations.as_ref().map(|x| x.outputs.iter());
            for (i, test) in tests.into_iter().enumerate() {
                let expected_output = expected_output
                    .as_mut()
                    .map(|x| x.next())
                    .flatten()
                    .map(|x| serde_yaml::from_value(x.clone()).expect("invalid test expectation form"));

                let output = run_individual_expression_test(path, test);
                if let Some(error) = emit_errors(
                    output.as_ref().map_err(|x| &**x),
                    &config.expectation,
                    expected_output,
                    i,
                ) {
                    errors.push(error);
                } else {
                    outputs.push(
                        output
                            .map(|x| serde_yaml::to_value(x).expect("serialization failed"))
                            .unwrap_or_else(|e| serde_yaml::to_value(e).expect("serialization failed")),
                    );
                }
            }
        }
    }
    outputs
}

#[test]
pub fn parser_tests() {
    let mut pass = 0;
    let mut fail = Vec::new();
    let mut tests = Vec::new();
    let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_dir.push("../tests/parser/");
    find_tests(&test_dir, &mut tests);
    let mut outputs = vec![];
    for (path, content) in tests.into_iter() {
        let config = extract_test_config(&content);
        if config.is_none() {
            panic!("missing configuration for {}", path);
        }
        let config = config.unwrap();
        let mut expectation_path = path.clone();
        expectation_path += ".out";
        let expectations: Option<TestExpectation> = if std::path::Path::new(&expectation_path).exists() {
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
        let mut errors = vec![];
        let raw_path = Path::new(&path);
        let new_outputs = run_test(
            &config,
            raw_path.file_name().unwrap_or_default().to_str().unwrap_or_default(),
            &content,
            expectations.as_ref(),
            &mut errors,
        );
        if errors.is_empty() {
            if expectations.is_none() {
                outputs.push((expectation_path, TestExpectation {
                    namespace: config.namespace,
                    expectation: config.expectation,
                    outputs: new_outputs,
                }));
            }
            pass += 1;
        } else {
            fail.push(TestFailure {
                path: path.clone(),
                errors,
            })
        }
    }
    if !fail.is_empty() {
        for (i, fail) in fail.iter().enumerate() {
            println!(
                "\n\n-----------------TEST #{} FAILED (and shouldn't have)-----------------",
                i + 1
            );
            println!("File: {}", fail.path);
            for error in &fail.errors {
                println!("{}", error);
            }
        }
        panic!("failed {}/{} tests", fail.len(), fail.len() + pass);
    } else {
        for (path, new_expectation) in outputs {
            std::fs::write(
                &path,
                serde_yaml::to_string(&new_expectation).expect("failed to serialize expectation yaml"),
            )
            .expect("failed to write expectation file");
        }
        println!("passed {}/{} tests", pass, pass);
    }
}

#[test]
pub fn parser_pass_tests() {
    let mut pass = 0;
    let mut fail = Vec::new();
    let mut tests = Vec::new();
    let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_dir.push("../tests/old/pass/");
    find_tests(&test_dir, &mut tests);
    for (path, content) in tests.into_iter() {
        match crate::parse(&path, &content) {
            Ok(_) => {
                pass += 1;
            }
            Err(e) => {
                fail.push(TestFailure {
                    path,
                    errors: vec![TestError::FailedAndShouldntHave {
                        index: 0,
                        error: e.to_string(),
                    }],
                });
            }
        }
    }
    if !fail.is_empty() {
        for (i, fail) in fail.iter().enumerate() {
            println!(
                "\n\n-----------------TEST #{} FAILED (and shouldn't have)-----------------",
                i + 1
            );
            println!("File: {}", fail.path);
            for error in &fail.errors {
                println!("{}", error);
            }
        }
        panic!("failed {}/{} tests", fail.len(), fail.len() + pass);
    } else {
        println!("passed {}/{} tests", pass, pass);
    }
}

#[test]
pub fn parser_fail_tests() {
    let mut pass = 0;
    let mut fail = Vec::new();
    let mut tests = Vec::new();
    let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_dir.push("../tests/old/fail/");
    find_tests(&test_dir, &mut tests);
    for (path, content) in tests.into_iter() {
        match crate::parse(&path, &content) {
            Ok(_) => {
                fail.push(path);
            }
            Err(_e) => {
                pass += 1;
            }
        }
    }
    if !fail.is_empty() {
        for (i, fail) in fail.iter().enumerate() {
            println!(
                "\n\n-----------------TEST #{} PASSED (and shouldn't have)-----------------",
                i + 1
            );
            println!("File: {}", fail);
        }
        panic!("failed {}/{} tests", fail.len(), fail.len() + pass);
    } else {
        println!("passed {}/{} tests", pass, pass);
    }
}
