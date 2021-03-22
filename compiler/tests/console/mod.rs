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

use crate::{
    assert_satisfied, expect_asg_error, expect_compiler_error, generate_main_input, parse_program,
    parse_program_with_input,
};
use leo_ast::InputValue;

#[test]
fn test_log() {
    let program_string = include_str!("log.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_log_fail() {
    let program_string = include_str!("log_fail.leo");

    assert!(parse_program(program_string).is_err());
}

#[test]
fn test_log_parameter() {
    let program_string = include_str!("log_parameter.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_log_parameter_many() {
    let program_string = include_str!("log_parameter_many.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_log_parameter_fail_empty() {
    let program_string = include_str!("log_parameter_fail_empty.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_log_parameter_fail_none() {
    let program_string = include_str!("log_parameter_fail_empty.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_log_parameter_fail_unknown() {
    let program_string = include_str!("log_parameter_fail_unknown.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_log_input() {
    let program_string = include_str!("log_input.leo");
    let mut program = parse_program(program_string).unwrap();

    let main_input = generate_main_input(vec![("a", Some(InputValue::Boolean(true)))]);

    program.set_main_input(main_input);

    assert_satisfied(program);
}

#[test]
fn test_log_conditional() {
    let program_string = include_str!("log_conditional.leo");
    let input_equal_string = include_str!("input/input_equal.in");

    let program = parse_program_with_input(program_string, input_equal_string).unwrap();

    assert_satisfied(program);

    let input_unequal_string = include_str!("input/input_unequal.in");

    let program = parse_program_with_input(program_string, input_unequal_string).unwrap();

    assert_satisfied(program);
}

// Debug

#[test]
fn test_debug() {
    let program_string = include_str!("debug.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

// Error

#[test]
fn test_error() {
    let program_string = include_str!("error.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

// Assertion

#[test]
fn test_assert() {
    let program_string = include_str!("assert.leo");
    let mut program = parse_program(program_string).unwrap();

    let main_input = generate_main_input(vec![("a", Some(InputValue::Boolean(true)))]);

    program.set_main_input(main_input);

    assert_satisfied(program);

    let mut program = parse_program(program_string).unwrap();

    let main_input = generate_main_input(vec![("a", Some(InputValue::Boolean(false)))]);

    program.set_main_input(main_input);

    expect_compiler_error(program);
}

#[test]
fn test_conditional_assert() {
    let program_string = include_str!("conditional_assert.leo");
    let mut program = parse_program(program_string).unwrap();

    let main_input = generate_main_input(vec![("a", Some(InputValue::Boolean(true)))]);
    program.set_main_input(main_input);

    assert_satisfied(program);

    let mut program = parse_program(program_string).unwrap();

    let main_input = generate_main_input(vec![("a", Some(InputValue::Boolean(false)))]);

    program.set_main_input(main_input);

    assert_satisfied(program);
}
