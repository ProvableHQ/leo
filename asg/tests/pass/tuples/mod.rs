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

#[test]
fn test_tuple_basic() {
    let program_string = include_str!("basic.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_tuple_access() {
    let program_string = include_str!("access.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_tuple_typed() {
    let program_string = include_str!("typed.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_multiple() {
    let program_string = include_str!("multiple.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_multiple_typed() {
    let program_string = include_str!("multiple_typed.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_function() {
    let program_string = include_str!("function.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_function_typed() {
    let program_string = include_str!("function_typed.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_function_multiple() {
    let program_string = include_str!("function_multiple.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_nested() {
    let program_string = include_str!("nested.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_nested_access() {
    let program_string = include_str!("nested_access.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_nested_typed() {
    let program_string = include_str!("nested_typed.leo");
    load_asg(program_string).unwrap();
}
