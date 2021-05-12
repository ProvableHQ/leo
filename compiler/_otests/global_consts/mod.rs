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

use crate::{assert_satisfied, expect_compiler_error, parse_program};


#[test]
fn test_global_consts() {
    let program_string = include_str!("global_consts.leo");

    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_modify_global_const() {
    let program_string = include_str!("modify_global_const.leo");

    let program = parse_program(program_string).unwrap();

    assert!(parse_program(program_string).is_err());
}