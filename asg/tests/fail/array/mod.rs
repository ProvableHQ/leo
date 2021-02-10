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

use crate::load_asg;

// Expressions

#[test]
fn test_initializer_fail() {
    let program_string = include_str!("initializer_fail.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_input_nested_3x2_fail() {
    let program_string = include_str!("input_nested_3x2_fail.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_input_tuple_3x2_fail() {
    let program_string = include_str!("input_tuple_3x2_fail.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_multi_fail_initializer() {
    let program_string = include_str!("multi_fail_initializer.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_multi_inline_fail() {
    let program_string = include_str!("multi_fail_inline.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_multi_initializer_fail() {
    let program_string = include_str!("multi_initializer_fail.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_nested_3x2_value_fail() {
    let program_string = include_str!("nested_3x2_value_fail.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_tuple_3x2_value_fail() {
    let program_string = include_str!("tuple_3x2_value_fail.leo");
    load_asg(program_string).err().unwrap();
}

// Array type tests

#[test]
fn test_type_fail() {
    let program_string = include_str!("type_fail.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_type_nested_value_nested_3x2_fail() {
    let program_string = include_str!("type_nested_value_nested_3x2_fail.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_type_nested_value_nested_4x3x2_fail() {
    let program_string = include_str!("type_nested_value_nested_4x3x2_fail.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_type_nested_value_tuple_3x2_fail() {
    let program_string = include_str!("type_nested_value_tuple_3x2_fail.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_type_nested_value_tuple_4x3x2_fail() {
    let program_string = include_str!("type_nested_value_tuple_4x3x2_fail.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_type_tuple_value_nested_3x2_fail() {
    let program_string = include_str!("type_tuple_value_nested_3x2_fail.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_type_tuple_value_nested_3x2_swap_fail() {
    let program_string = include_str!("type_tuple_value_nested_3x2_swap_fail.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_type_tuple_value_nested_4x3x2_fail() {
    let program_string = include_str!("type_tuple_value_nested_4x3x2_fail.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_type_tuple_value_tuple_3x2_fail() {
    let program_string = include_str!("type_tuple_value_tuple_3x2_fail.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_type_tuple_value_tuple_3x2_swap_fail() {
    let program_string = include_str!("type_tuple_value_tuple_3x2_swap_fail.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_type_tuple_value_tuple_4x3x2_fail() {
    let program_string = include_str!("type_tuple_value_tuple_4x3x2_fail.leo");
    load_asg(program_string).err().unwrap();
}
