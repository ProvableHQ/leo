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

use crate::assert_satisfied;
use crate::expect_asg_error;
use crate::get_output;
use crate::parse_program;
use crate::parse_program_with_input;

#[test]
fn test_conditional_return() {
    let input_string = include_str!("input/conditional_return.in");
    let program_string = include_str!("conditional_return.leo");
    let program = parse_program_with_input(program_string, input_string).unwrap();

    let expected_string = include_str!("output/conditional_return.out");
    let actual_bytes = get_output(program);
    let actual_string = std::str::from_utf8(actual_bytes.bytes().as_slice()).unwrap();

    assert_eq!(expected_string, actual_string);
}

#[test]
fn test_empty() {
    let program_string = include_str!("empty.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_iteration() {
    let program_string = include_str!("iteration.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_iteration_repeated() {
    let program_string = include_str!("iteration_repeated.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_newlines() {
    let input_string = include_str!("input/newlines.in");
    let program_string = include_str!("newlines.leo");
    let program = parse_program_with_input(program_string, input_string).unwrap();

    let expected_string = include_str!("output/newlines.out");
    let actual_bytes = get_output(program);
    let actual_string = std::str::from_utf8(actual_bytes.bytes().as_slice()).unwrap();

    assert_eq!(expected_string, actual_string);
}

#[test]
fn test_multiple_returns() {
    let program_string = include_str!("multiple_returns.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_multiple_returns_fail() {
    let program_string = include_str!("multiple_returns_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_multiple_returns_fail_conditional() {
    let program_string = include_str!("multiple_returns_fail_conditional.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_multiple_returns_main() {
    let program_string = include_str!("multiple_returns_main.leo");
    let input_string = include_str!("input/registers.in");

    let program = parse_program_with_input(program_string, input_string).unwrap();

    let expected_string = include_str!("output/registers.out");
    let actual_bytes = get_output(program);
    let actual_string = std::str::from_utf8(actual_bytes.bytes().as_slice()).unwrap();

    assert_eq!(expected_string, actual_string);
}

#[test]
fn test_repeated_function_call() {
    let program_string = include_str!("repeated.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_return() {
    let program_string = include_str!("return.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_scope_fail() {
    let program_string = include_str!("scope_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_undefined() {
    let program_string = include_str!("undefined.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_value_unchanged() {
    let program_string = include_str!("value_unchanged.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_array_input() {
    let program_string = include_str!("array_input.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error)
}

// Test return multidimensional arrays

#[test]
fn test_return_array_nested_fail() {
    let program_string = include_str!("return_array_nested_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_return_array_nested_pass() {
    let program_string = include_str!("return_array_nested_pass.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_return_array_tuple_fail() {
    let program_string = include_str!("return_array_tuple_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_return_array_tuple_pass() {
    let program_string = include_str!("return_array_tuple_pass.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

// Test return tuples

#[test]
fn test_return_tuple() {
    let program_string = include_str!("return_tuple.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_return_tuple_conditional() {
    let program_string = include_str!("return_tuple_conditional.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_array_params_direct_call() {
    let program_string = include_str!("array_params_direct_call.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}
