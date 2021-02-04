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
fn test_input_pass() {
    let program_string = include_str!("assert_eq_input.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_registers() {
    let program_string = include_str!("output_register.leo");
    load_asg(program_string).unwrap();
}

// Boolean not !

#[test]
fn test_not_true() {
    let program_string = include_str!("not_true.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_not_false() {
    let program_string = include_str!("not_false.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_not_mutable() {
    let program_string = include_str!("not_mutable.leo");
    load_asg(program_string).unwrap();
}

// Boolean or ||

#[test]
fn test_true_or_true() {
    let program_string = include_str!("true_or_true.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_true_or_false() {
    let program_string = include_str!("true_or_false.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_false_or_false() {
    let program_string = include_str!("false_or_false.leo");
    load_asg(program_string).unwrap();
}

// Boolean and &&

#[test]
fn test_true_and_true() {
    let program_string = include_str!("true_and_true.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_true_and_false() {
    let program_string = include_str!("true_and_false.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_false_and_false() {
    let program_string = include_str!("false_and_false.leo");
    load_asg(program_string).unwrap();
}

// All

#[test]
fn test_all() {
    let program_string = include_str!("all.leo");
    load_asg(program_string).unwrap();
}
