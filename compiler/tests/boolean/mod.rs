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
    let program_bytes = include_bytes!("assert_eq_input.leo");
    let input_bytes = include_bytes!("input/true_true.in");

    let program = parse_program_with_input(program_bytes, input_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_input_fail() {
    let program_bytes = include_bytes!("assert_eq_input.leo");
    let input_bytes = include_bytes!("input/true_false.in");

    let program = parse_program_with_input(program_bytes, input_bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_registers() {
    let program_bytes = include_bytes!("output_register.leo");
    let true_input_bytes = include_bytes!("input/registers_true.in");
    let false_input_bytes = include_bytes!("input/registers_false.in");

    // test true input register => true output register
    let program = parse_program_with_input(program_bytes, true_input_bytes).unwrap();

    output_true(program);

    // test false input register => false output register
    let program = parse_program_with_input(program_bytes, false_input_bytes).unwrap();

    output_false(program);
}

// Boolean not !

#[test]
fn test_not_true() {
    let bytes = include_bytes!("not_true.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_not_false() {
    let bytes = include_bytes!("not_false.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_not_mutable() {
    let bytes = include_bytes!("not_mutable.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_not_u32() {
    let bytes = include_bytes!("not_u32.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

// Boolean or ||

#[test]
fn test_true_or_true() {
    let bytes = include_bytes!("true_or_true.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_true_or_false() {
    let bytes = include_bytes!("true_or_false.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_false_or_false() {
    let bytes = include_bytes!("false_or_false.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_true_or_u32() {
    let bytes = include_bytes!("true_or_u32.leo");
    let error = parse_program(bytes).err().unwrap();

    expect_type_inference_error(error);
}

// Boolean and &&

#[test]
fn test_true_and_true() {
    let bytes = include_bytes!("true_and_true.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_true_and_false() {
    let bytes = include_bytes!("true_and_false.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_false_and_false() {
    let bytes = include_bytes!("false_and_false.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_true_and_u32() {
    let bytes = include_bytes!("true_and_u32.leo");
    let error = parse_program(bytes).err().unwrap();

    expect_type_inference_error(error);
}

// All

#[test]
fn test_all() {
    let bytes = include_bytes!("all.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}
