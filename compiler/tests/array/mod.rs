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
    integers::{expect_computation_error, expect_parsing_error},
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
    let program_bytes = include_bytes!("registers.leo");
    let ones_input_bytes = include_bytes!("input/registers_ones.in");
    let zeros_input_bytes = include_bytes!("input/registers_zeros.in");

    // test ones input register => ones output register
    let program = parse_program_with_input(program_bytes, ones_input_bytes).unwrap();

    output_ones(program);

    // test zeros input register => zeros output register
    let program = parse_program_with_input(program_bytes, zeros_input_bytes).unwrap();

    output_zeros(program);
}

// Expressions

#[test]
fn test_type_fail() {
    let program_bytes = include_bytes!("type_fail.leo");
    let syntax_error = parse_program(program_bytes).is_err();

    assert!(syntax_error);
}

#[test]
fn test_inline() {
    let program_bytes = include_bytes!("inline.leo");
    let input_bytes = include_bytes!("input/three_ones.in");
    let program = parse_program_with_input(program_bytes, input_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_inline_fail() {
    let program_bytes = include_bytes!("inline.leo");
    let program = parse_program(program_bytes).unwrap();

    let _err = expect_compiler_error(program);
}

#[test]
fn test_initializer() {
    let program_bytes = include_bytes!("initializer.leo");
    let input_bytes = include_bytes!("input/three_ones.in");
    let program = parse_program_with_input(program_bytes, input_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_initializer_fail() {
    let program_bytes = include_bytes!("initializer_fail.leo");
    let input_bytes = include_bytes!("input/three_ones.in");
    let syntax_error = parse_program_with_input(program_bytes, input_bytes).is_err();

    assert!(syntax_error);
}

#[test]
fn test_initializer_input() {
    let program_bytes = include_bytes!("initializer_input.leo");
    let input_bytes = include_bytes!("input/six_zeros.in");
    let program = parse_program_with_input(program_bytes, input_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_initializer_input_fail() {
    let program_bytes = include_bytes!("initializer_input.leo");
    let input_bytes = include_bytes!("input/initializer_fail.in");
    let syntax_error = parse_program_with_input(program_bytes, input_bytes).is_err();

    assert!(syntax_error);
}

#[test]
fn test_spread() {
    let program_bytes = include_bytes!("spread.leo");
    let input_bytes = include_bytes!("input/three_ones.in");
    let program = parse_program_with_input(program_bytes, input_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_slice() {
    let program_bytes = include_bytes!("slice.leo");
    let input_bytes = include_bytes!("input/three_ones.in");
    let program = parse_program_with_input(program_bytes, input_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_multi() {
    let program_bytes = include_bytes!("multi.leo");
    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_multi_fail() {
    let program_bytes = include_bytes!("multi_fail_initializer.leo");
    let program = parse_program(program_bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_multi_inline_fail() {
    let program_bytes = include_bytes!("multi_fail_inline.leo");
    let program = parse_program(program_bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_multi_initializer() {
    let program_bytes = include_bytes!("multi_initializer.leo");
    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_multi_initializer_fail() {
    let program_bytes = include_bytes!("multi_initializer_fail.leo");
    let program = parse_program(program_bytes).unwrap();

    expect_compiler_error(program);
}
