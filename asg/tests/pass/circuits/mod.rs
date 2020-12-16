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

use crate::load_asg;

// Expressions

#[test]
fn test_inline() {
    let program_string = include_str!("inline.leo");
    load_asg(program_string).unwrap();
}

// Members

#[test]
fn test_member_variable() {
    let program_string = include_str!("member_variable.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_member_variable_and_function() {
    let program_string = include_str!("member_variable_and_function.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_member_function() {
    let program_string = include_str!("member_function.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_member_function_nested() {
    let program_string = include_str!("member_function_nested.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_member_static_function() {
    let program_string = include_str!("member_static_function.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_member_static_function_nested() {
    let program_string = include_str!("member_static_function_nested.leo");
    load_asg(program_string).unwrap();
}

// Mutability

#[test]
fn test_mutate_self_variable() {
    let program_string = include_str!("mut_self_variable.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_mutate_self_variable_conditional() {
    let program_string = include_str!("mut_self_variable_conditional.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_mutate_variable() {
    let program_string = include_str!("mut_variable.leo");
    load_asg(program_string).unwrap();
}

// Self

#[test]
fn test_self_member_pass() {
    let program_string = include_str!("self_member.leo");
    load_asg(program_string).unwrap();
}

// All

#[test]
fn test_pedersen_mock() {
    let program_string = include_str!("pedersen_mock.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_define_circuit_inside_circuit_function() {
    let program_string = include_str!("define_circuit_inside_circuit_function.leo");
    load_asg(program_string).unwrap();
}
