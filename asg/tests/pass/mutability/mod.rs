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

#[test]
fn test_let_mutate() {
    let program_string = include_str!("let_mutate.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_let_mutate_nested() {
    let program_string = include_str!("let_mutate_nested.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_array_mutate() {
    let program_string = include_str!("array_mutate.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_array_tuple_mutate() {
    let program_string = include_str!("array_tuple_mutate.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_array_splice_mutate() {
    let program_string = include_str!("array_splice_mutate.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_circuit_mutate() {
    let program_string = include_str!("circuit_mutate.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_circuit_variable_mutate() {
    let program_string = include_str!("circuit_variable_mutate.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_function_input_mutate() {
    let program_string = include_str!("function_input_mutate.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_swap() {
    let program_string = include_str!("swap.leo");
    load_asg(program_string).unwrap();
}
