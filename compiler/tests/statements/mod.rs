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

use crate::{assert_satisfied, generate_main_input, parse_program};
use leo_ast::InputValue;

pub mod conditional;

// Ternary if {bool}? {expression} : {expression};

#[test]
fn test_ternary_basic() {
    let bytes = include_bytes!("ternary_basic.leo");
    let mut program = parse_program(bytes).unwrap();

    let main_input = generate_main_input(vec![
        ("a", Some(InputValue::Boolean(true))),
        ("b", Some(InputValue::Boolean(true))),
    ]);

    program.set_main_input(main_input);

    assert_satisfied(program);

    let mut program = parse_program(bytes).unwrap();

    let main_input = generate_main_input(vec![
        ("a", Some(InputValue::Boolean(false))),
        ("b", Some(InputValue::Boolean(false))),
    ]);

    program.set_main_input(main_input);

    assert_satisfied(program);
}

// Iteration for i {start}..{stop} { statements }

#[test]
fn test_iteration_basic() {
    let bytes = include_bytes!("iteration_basic.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

// #[test]
// fn test_num_returns_fail() {
//     let bytes = include_bytes!("num_returns_fail.leo");
//     let error = parse_program(bytes).err().unwrap();
//
//     expect_type_inference_error(error);
// }
