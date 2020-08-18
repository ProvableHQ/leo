// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{
    assert_satisfied,
    expect_compiler_error,
    get_output,
    parse_program,
    parse_program_with_input,
    EdwardsTestCompiler,
};
use leo_compiler::errors::{CompilerError, ExpressionError, FunctionError, StatementError};

fn expect_undefined_identifier(program: EdwardsTestCompiler) {
    match expect_compiler_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::Error(_),
        ))) => {}
        error => panic!("Expected function undefined, got {}", error),
    }
}

#[test]
fn test_empty() {
    let bytes = include_bytes!("empty.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_iteration() {
    let bytes = include_bytes!("iteration.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_iteration_repeated() {
    let bytes = include_bytes!("iteration_repeated.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_newlines() {
    let input_bytes = include_bytes!("input/newlines.in");
    let program_bytes = include_bytes!("newlines.leo");
    let program = parse_program_with_input(program_bytes, input_bytes).unwrap();

    let expected_bytes = include_bytes!("output/newlines.out");
    let expected = std::str::from_utf8(expected_bytes).unwrap();
    let actual_bytes = get_output(program);
    let actual = std::str::from_utf8(actual_bytes.bytes().as_slice()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn test_multiple_returns() {
    let bytes = include_bytes!("multiple.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_multiple_returns_main() {
    let program_bytes = include_bytes!("multiple_main.leo");
    let input_bytes = include_bytes!("input/registers.in");

    let program = parse_program_with_input(program_bytes, input_bytes).unwrap();

    let expected_bytes = include_bytes!("output/registers.out");
    let expected = std::str::from_utf8(expected_bytes).unwrap();
    let actual_bytes = get_output(program);
    let actual = std::str::from_utf8(actual_bytes.bytes().as_slice()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn test_repeated_function_call() {
    let bytes = include_bytes!("repeated.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_return() {
    let bytes = include_bytes!("return.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_scope_fail() {
    let bytes = include_bytes!("scope_fail.leo");
    let program = parse_program(bytes).unwrap();

    match expect_compiler_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::FunctionError(value),
        ))) => match *value {
            FunctionError::StatementError(StatementError::ExpressionError(ExpressionError::Error(_))) => {}
            error => panic!("Expected function undefined, got {}", error),
        },
        error => panic!("Expected function undefined, got {}", error),
    }
}

#[test]
fn test_undefined() {
    let bytes = include_bytes!("undefined.leo");
    let program = parse_program(bytes).unwrap();

    expect_undefined_identifier(program);
}

#[test]
fn test_value_unchanged() {
    let bytes = include_bytes!("value_unchanged.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}
