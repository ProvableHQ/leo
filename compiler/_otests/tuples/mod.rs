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

use crate::{assert_satisfied, parse_program};

#[test]
fn test_tuple_basic() {
    let program_string = include_str!("basic.leo");

    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_tuple_access() {
    let program_string = include_str!("access.leo");

    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_tuple_typed() {
    let program_string = include_str!("typed.leo");

    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_multiple() {
    let program_string = include_str!("multiple.leo");

    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_multiple_typed() {
    let program_string = include_str!("multiple_typed.leo");

    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_function() {
    let program_string = include_str!("function.leo");

    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_function_typed() {
    let program_string = include_str!("function_typed.leo");

    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_function_multiple() {
    let progam_string = include_str!("function_multiple.leo");

    let program = parse_program(progam_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_nested() {
    let program_string = include_str!("nested.leo");

    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_nested_access() {
    let program_string = include_str!("nested_access.leo");

    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_nested_typed() {
    let program_string = include_str!("nested_typed.leo");

    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

// #[test]
// fn test_input() {
//     let input_string = include_str!("inputs/input.in");
//     let program_string = include_str!("")
// }
