// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::load_asg;

// Registers

#[test]
fn test_registers() {
    let program_string = include_str!("registers.leo");
    load_asg(program_string).unwrap();
}

// Expressions

#[test]
fn test_inline() {
    let program_string = include_str!("inline.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_initializer() {
    let program_string = include_str!("initializer.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_initializer_input() {
    let program_string = include_str!("initializer_input.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_input_nested_3x2() {
    let program_string = include_str!("input_nested_3x2.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_input_tuple_3x2() {
    let program_string = include_str!("input_tuple_3x2.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_multi_initializer() {
    let program_string = include_str!("multi_initializer.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_nested_3x2_value() {
    let program_string = include_str!("nested_3x2_value.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_tuple_3x2_value() {
    let program_string = include_str!("tuple_3x2_value.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_spread() {
    let program_string = include_str!("spread.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_slice() {
    let program_string = include_str!("slice.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_index_u8() {
    let program_string = include_str!("index_u8.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_slice_i8() {
    let program_string = include_str!("slice_i8.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_slice_lower() {
    let program_string = include_str!("slice_lower.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_implicit() {
    let program_string = include_str!("implicit.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_type_nested_value_nested_3x2() {
    let program_string = include_str!("type_nested_value_nested_3x2.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_type_nested_value_nested_4x3x2() {
    let program_string = include_str!("type_nested_value_nested_4x3x2.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_type_nested_value_tuple_3x2() {
    let program_string = include_str!("type_nested_value_tuple_3x2.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_type_nested_value_tuple_4x3x2() {
    let program_string = include_str!("type_nested_value_tuple_4x3x2.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_type_tuple_value_nested_3x2() {
    let program_string = include_str!("type_tuple_value_nested_3x2.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_type_tuple_value_nested_4x3x2() {
    let program_string = include_str!("type_tuple_value_nested_4x3x2.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_type_tuple_value_tuple_3x2() {
    let program_string = include_str!("type_tuple_value_tuple_3x2.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_type_tuple_value_tuple_4x3x2() {
    let program_string = include_str!("type_tuple_value_tuple_4x3x2.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_input_type_nested_value_nested_3x2() {
    let program_string = include_str!("type_input_3x2.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_input_type_nested_value_nested_4x3x2() {
    let program_string = include_str!("type_input_4x3x2.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_input_type_nested_value_tuple_3x2() {
    let program_string = include_str!("type_input_3x2.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_input_type_nested_value_tuple_4x3x2() {
    let program_string = include_str!("type_input_4x3x2.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_input_type_tuple_value_nested_3x2() {
    let program_string = include_str!("type_input_3x2.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_input_type_tuple_value_nested_4x3x2() {
    let program_string = include_str!("type_input_4x3x2.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_input_type_tuple_value_tuple_3x2() {
    let program_string = include_str!("type_input_3x2.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_input_type_tuple_value_tuple_4x3x2() {
    let program_string = include_str!("type_input_4x3x2.leo");
    load_asg(program_string).unwrap();
}
