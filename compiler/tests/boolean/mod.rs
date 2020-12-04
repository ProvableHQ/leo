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
    expect_type_inference_error,
    get_output,
    parse_program,
    parse_program_with_input,
    EdwardsTestCompiler,
};

pub fn output_true(program: EdwardsTestCompiler) {
    let expected = include_bytes!("output/registers_true.out");
    let actual = get_output(program);

    assert_eq!(expected, actual.bytes().as_slice());
}

pub fn output_false(program: EdwardsTestCompiler) {
    let expected = include_bytes!("output/registers_false.out");
    let actual = get_output(program);

    assert_eq!(expected, actual.bytes().as_slice());
}

#[test]
fn test_input_pass() {
    let program_string = include_str!("assert_eq_input.leo");
    let input_string = include_str!("input/true_true.in");

    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_input_fail() {
    let program_string = include_str!("assert_eq_input.leo");
    let input_string = include_str!("input/true_false.in");

    let program = parse_program_with_input(program_string, input_string).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_registers() {
    let program_string = include_str!("output_register.leo");
    let true_input_string = include_str!("input/registers_true.in");
    let false_input_string = include_str!("input/registers_false.in");

    // test true input register => true output register
    let program = parse_program_with_input(program_string, true_input_string).unwrap();

    output_true(program);

    // test false input register => false output register
    let program = parse_program_with_input(program_string, false_input_string).unwrap();

    output_false(program);
}

// Boolean not !

#[test]
fn test_not_true() {
    let program_string = include_str!("not_true.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_not_false() {
    let program_string = include_str!("not_false.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_not_mutable() {
    let program_string = include_str!("not_mutable.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_not_u32() {
    let program_string = include_str!("not_u32.leo");
    let program = parse_program(program_string).unwrap();

    expect_compiler_error(program);
}

// Boolean or ||

#[test]
fn test_true_or_true() {
    let program_string = include_str!("true_or_true.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_true_or_false() {
    let program_string = include_str!("true_or_false.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_false_or_false() {
    let program_string = include_str!("false_or_false.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_true_or_u32() {
    let program_string = include_str!("true_or_u32.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_type_inference_error(error);
}

// Boolean and &&

#[test]
fn test_true_and_true() {
    let program_string = include_str!("true_and_true.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_true_and_false() {
    let program_string = include_str!("true_and_false.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_false_and_false() {
    let program_string = include_str!("false_and_false.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_true_and_u32() {
    let program_string = include_str!("true_and_u32.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_type_inference_error(error);
}

// All

#[test]
fn test_all() {
    let program_string = include_str!("all.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}
