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
    expect_asg_error,
    expect_compiler_error,
    get_output,
    parse_program,
    parse_program_with_input,
    EdwardsTestCompiler,
};

pub fn output_ones(program: EdwardsTestCompiler) {
    let expected = include_bytes!("output/registers_ones.out");
    let actual = get_output(program);

    assert!(expected.eq(actual.bytes().as_slice()));
}

pub fn output_zeros(program: EdwardsTestCompiler) {
    let expected = include_bytes!("output/registers_zeros.out");
    let actual = get_output(program);

    assert!(expected.eq(actual.bytes().as_slice()));
}

// Registers

#[test]
fn test_registers() {
    let program_string = include_str!("registers.leo");
    let ones_input_string = include_str!("input/registers_ones.in");
    let zeros_input_string = include_str!("input/registers_zeros.in");

    // test ones input register => ones output register
    let program = parse_program_with_input(program_string, ones_input_string).unwrap();

    output_ones(program);

    // test zeros input register => zeros output register
    let program = parse_program_with_input(program_string, zeros_input_string).unwrap();

    output_zeros(program);
}

// Expressions

#[test]
fn test_inline() {
    let program_string = include_str!("inline.leo");
    let input_string = include_str!("input/three_ones.in");
    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_inline_fail() {
    let program_string = include_str!("inline.leo");
    let program = parse_program(program_string).unwrap();

    let _err = expect_compiler_error(program);
}

#[test]
fn test_initializer() {
    let program_string = include_str!("initializer.leo");
    let input_string = include_str!("input/three_ones.in");
    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_initializer_fail() {
    let program_string = include_str!("initializer_fail.leo");
    let input_string = include_str!("input/three_ones.in");
    let syntax_error = parse_program_with_input(program_string, input_string).is_err();

    assert!(syntax_error);
}

#[test]
fn test_initializer_input() {
    let program_string = include_str!("initializer_input.leo");
    let input_string = include_str!("input/six_zeros.in");
    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_initializer_input_fail() {
    let program_string = include_str!("initializer_input.leo");
    let input_string = include_str!("input/initializer_fail.in");
    let syntax_error = parse_program_with_input(program_string, input_string).is_err();

    assert!(syntax_error);
}

#[test]
fn test_input_nested_3x2() {
    let program_string = include_str!("input_nested_3x2.leo");
    let input_string = include_str!("input/input_nested_3x2.in");
    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_input_nested_3x2_fail() {
    let program_string = include_str!("input_nested_3x2_fail.leo");
    let input_string = include_str!("input/input_nested_3x2_fail.in");
    let syntax_error = parse_program_with_input(program_string, input_string).is_err();

    assert!(syntax_error);
}

#[test]
fn test_input_tuple_3x2() {
    let program_string = include_str!("input_tuple_3x2.leo");
    let input_string = include_str!("input/input_tuple_3x2.in");
    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_input_tuple_3x2_fail() {
    let program_string = include_str!("input_tuple_3x2_fail.leo");
    let input_string = include_str!("input/input_tuple_3x2_fail.in");
    let syntax_error = parse_program_with_input(program_string, input_string).is_err();

    assert!(syntax_error);
}

#[test]
fn test_multi_fail_initializer() {
    let program_string = include_str!("multi_fail_initializer.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_multi_inline_fail() {
    let program_string = include_str!("multi_fail_inline.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_multi_initializer() {
    let program_string = include_str!("multi_initializer.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_multi_initializer_fail() {
    let program_string = include_str!("multi_initializer_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_nested_3x2_value() {
    let program_string = include_str!("nested_3x2_value.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_nested_3x2_value_fail() {
    let program_string = include_str!("nested_3x2_value_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_tuple_3x2_value() {
    let program_string = include_str!("tuple_3x2_value.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_tuple_3x2_value_fail() {
    let program_string = include_str!("tuple_3x2_value_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_spread() {
    let program_string = include_str!("spread.leo");
    let input_string = include_str!("input/three_ones.in");
    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_slice() {
    let program_string = include_str!("slice.leo");
    let input_string = include_str!("input/three_ones.in");
    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_slice_lower() {
    let program_string = include_str!("slice_lower.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

// Array type tests

#[test]
fn test_type_fail() {
    let program_string = include_str!("type_fail.leo");
    let syntax_error = parse_program(program_string).is_err();

    assert!(syntax_error);
}

#[test]
fn test_type_nested_value_nested_3x2() {
    let program_string = include_str!("type_nested_value_nested_3x2.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_type_nested_value_nested_3x2_fail() {
    let program_string = include_str!("type_nested_value_nested_3x2_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_type_nested_value_nested_4x3x2() {
    let program_string = include_str!("type_nested_value_nested_4x3x2.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_type_nested_value_nested_4x3x2_fail() {
    let program_string = include_str!("type_nested_value_nested_4x3x2_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_type_nested_value_tuple_3x2() {
    let program_string = include_str!("type_nested_value_tuple_3x2.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_type_nested_value_tuple_3x2_fail() {
    let program_string = include_str!("type_nested_value_tuple_3x2_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_type_nested_value_tuple_4x3x2() {
    let program_string = include_str!("type_nested_value_tuple_4x3x2.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_type_nested_value_tuple_4x3x2_fail() {
    let program_string = include_str!("type_nested_value_tuple_4x3x2_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_type_tuple_value_nested_3x2() {
    let program_string = include_str!("type_tuple_value_nested_3x2.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_type_tuple_value_nested_3x2_fail() {
    let program_string = include_str!("type_tuple_value_nested_3x2_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_type_tuple_value_nested_4x3x2() {
    let program_string = include_str!("type_tuple_value_nested_4x3x2.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_type_tuple_value_nested_4x3x2_fail() {
    let program_string = include_str!("type_tuple_value_nested_4x3x2_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_type_tuple_value_tuple_3x2() {
    let program_string = include_str!("type_tuple_value_tuple_3x2.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_type_tuple_value_tuple_3x2_fail() {
    let program_string = include_str!("type_tuple_value_tuple_3x2_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_type_tuple_value_tuple_4x3x2() {
    let program_string = include_str!("type_tuple_value_tuple_4x3x2.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_type_tuple_value_tuple_4x3x2_fail() {
    let program_string = include_str!("type_tuple_value_tuple_4x3x2_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

// Tests for nested multi-dimensional arrays as input to the program

#[test]
fn test_input_type_nested_value_nested_3x2() {
    let program_string = include_str!("type_input_3x2.leo");
    let input_string = include_str!("input/type_nested_value_nested_3x2.in");
    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_input_type_nested_value_nested_3x2_fail() {
    let program_string = include_str!("type_input_3x2.leo");
    let input_string = include_str!("input/type_nested_value_nested_3x2_fail.in");
    let syntax_error = parse_program_with_input(program_string, input_string).is_err();

    assert!(syntax_error);
}

#[test]
fn test_input_type_nested_value_nested_4x3x2() {
    let program_string = include_str!("type_input_4x3x2.leo");
    let input_string = include_str!("input/type_nested_value_nested_4x3x2.in");
    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_input_type_nested_value_nested_4x3x2_fail() {
    let program_string = include_str!("type_input_4x3x2.leo");
    let input_string = include_str!("input/type_nested_value_nested_4x3x2_fail.in");
    let syntax_error = parse_program_with_input(program_string, input_string).is_err();

    assert!(syntax_error);
}

#[test]
fn test_input_type_nested_value_tuple_3x2() {
    let program_string = include_str!("type_input_3x2.leo");
    let input_string = include_str!("input/type_nested_value_tuple_3x2.in");
    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_input_type_nested_value_tuple_3x2_fail() {
    let program_string = include_str!("type_input_3x2.leo");
    let input_string = include_str!("input/type_nested_value_tuple_3x2_fail.in");
    let syntax_error = parse_program_with_input(program_string, input_string).is_err();

    assert!(syntax_error);
}

#[test]
fn test_input_type_nested_value_tuple_4x3x2() {
    let program_string = include_str!("type_input_4x3x2.leo");
    let input_string = include_str!("input/type_nested_value_tuple_4x3x2.in");
    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program)
}

#[test]
fn test_input_type_nested_value_tuple_4x3x2_fail() {
    let program_string = include_str!("type_input_4x3x2.leo");
    let input_string = include_str!("input/type_nested_value_tuple_4x3x2_fail.in");
    let syntax_error = parse_program_with_input(program_string, input_string).is_err();

    assert!(syntax_error);
}

// Tests for multi-dimensional arrays using tuple syntax as input to the program

#[test]
fn test_input_type_tuple_value_nested_3x2() {
    let program_string = include_str!("type_input_3x2.leo");
    let input_string = include_str!("input/type_tuple_value_nested_3x2.in");
    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_input_type_tuple_value_nested_3x2_fail() {
    let program_string = include_str!("type_input_3x2.leo");
    let input_string = include_str!("input/type_tuple_value_nested_3x2_fail.in");
    let syntax_error = parse_program_with_input(program_string, input_string).is_err();

    assert!(syntax_error);
}

#[test]
fn test_input_type_tuple_value_nested_4x3x2() {
    let program_string = include_str!("type_input_4x3x2.leo");
    let input_string = include_str!("input/type_tuple_value_nested_4x3x2.in");
    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_input_type_tuple_value_nested_4x3x2_fail() {
    let program_string = include_str!("type_input_4x3x2.leo");
    let input_string = include_str!("input/type_tuple_value_nested_4x3x2_fail.in");
    let syntax_error = parse_program_with_input(program_string, input_string).is_err();

    assert!(syntax_error);
}

#[test]
fn test_input_type_tuple_value_tuple_3x2() {
    let program_string = include_str!("type_input_3x2.leo");
    let input_string = include_str!("input/type_tuple_value_tuple_3x2.in");
    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_input_type_tuple_value_tuple_3x2_fail() {
    let program_string = include_str!("type_input_3x2.leo");
    let input_string = include_str!("input/type_tuple_value_tuple_3x2_fail.in");
    let syntax_error = parse_program_with_input(program_string, input_string).is_err();

    assert!(syntax_error);
}

#[test]
fn test_input_type_tuple_value_tuple_4x3x2() {
    let program_string = include_str!("type_input_4x3x2.leo");
    let input_string = include_str!("input/type_tuple_value_tuple_4x3x2.in");
    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_input_type_tuple_value_tuple_4x3x2_fail() {
    let program_string = include_str!("type_input_4x3x2.leo");
    let input_string = include_str!("input/type_tuple_value_tuple_4x3x2_fail.in");
    let syntax_error = parse_program_with_input(program_string, input_string).is_err();

    assert!(syntax_error);
}

#[test]
fn test_variable_slice_fail() {
    let program_string = include_str!("variable_slice_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}
