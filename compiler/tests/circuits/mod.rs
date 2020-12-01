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

use crate::{assert_satisfied, expect_compiler_error, expect_type_inference_error, parse_program};

// Expressions

#[test]
fn test_inline() {
    let bytes = include_bytes!("inline.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_inline_fail() {
    let bytes = include_bytes!("inline_fail.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_inline_undefined() {
    let bytes = include_bytes!("inline_undefined.leo");
    let error = parse_program(bytes).err().unwrap();

    expect_type_inference_error(error);
}

// Members

#[test]
fn test_member_variable() {
    let bytes = include_bytes!("member_variable.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_member_variable_fail() {
    let bytes = include_bytes!("member_variable_fail.leo");
    let error = parse_program(bytes).err().unwrap();

    expect_type_inference_error(error);
}

#[test]
fn test_member_variable_and_function() {
    let bytes = include_bytes!("member_variable_and_function.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_member_function() {
    let bytes = include_bytes!("member_function.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_member_function_fail() {
    let bytes = include_bytes!("member_function_fail.leo");
    let error = parse_program(bytes).err().unwrap();

    expect_type_inference_error(error);
}

#[test]
fn test_member_function_invalid() {
    let bytes = include_bytes!("member_function_invalid.leo");
    let error = parse_program(bytes).err().unwrap();

    expect_type_inference_error(error);
}

#[test]
fn test_member_function_nested() {
    let bytes = include_bytes!("member_function_nested.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_member_static_function() {
    let bytes = include_bytes!("member_static_function.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_member_static_function_nested() {
    let bytes = include_bytes!("member_static_function_nested.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_member_static_function_invalid() {
    let bytes = include_bytes!("member_static_function_invalid.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program)
}

#[test]
fn test_member_static_function_undefined() {
    let bytes = include_bytes!("member_static_function_undefined.leo");
    let error = parse_program(bytes).err().unwrap();

    expect_type_inference_error(error)
}

// Mutability

#[test]
fn test_mutate_function_fail() {
    let bytes = include_bytes!("mut_function_fail.leo");
    let error = parse_program(bytes).err().unwrap();

    expect_type_inference_error(error);
}

#[test]
fn test_mutate_self_variable() {
    let bytes = include_bytes!("mut_self_variable.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_mutate_self_variable_fail() {
    let bytes = include_bytes!("mut_self_variable_fail.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_mutate_self_function_fail() {
    let bytes = include_bytes!("mut_self_function_fail.leo");
    let error = parse_program(bytes).err().unwrap();

    expect_type_inference_error(error);
}

#[test]
fn test_mutate_self_static_function_fail() {
    let bytes = include_bytes!("mut_self_static_function_fail.leo");
    let error = parse_program(bytes).err().unwrap();

    expect_type_inference_error(error);
}

#[test]
fn test_mutate_static_function_fail() {
    let bytes = include_bytes!("mut_static_function_fail.leo");
    let error = parse_program(bytes).err().unwrap();

    expect_type_inference_error(error);
}

#[test]
fn test_mutate_variable() {
    let bytes = include_bytes!("mut_variable.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_mutate_variable_fail() {
    let bytes = include_bytes!("mut_variable_fail.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

// Self

#[test]
fn test_self_fail() {
    let bytes = include_bytes!("self_fail.leo");
    let error = parse_program(bytes).err().unwrap();

    expect_type_inference_error(error);
}

#[test]
fn test_self_member_pass() {
    let bytes = include_bytes!("self_member.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_self_member_invalid() {
    let bytes = include_bytes!("self_member_invalid.leo");
    let error = parse_program(bytes).err().unwrap();

    expect_type_inference_error(error);
}

#[test]
fn test_self_member_undefined() {
    let bytes = include_bytes!("self_member_undefined.leo");
    let error = parse_program(bytes).err().unwrap();

    expect_type_inference_error(error);
}

// All

#[test]
fn test_pedersen_mock() {
    let bytes = include_bytes!("pedersen_mock.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_define_circuit_inside_circuit_function() {
    let bytes = include_bytes!("define_circuit_inside_circuit_function.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}
