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

#[test]
fn test_negate() {
    let program_string = include_str!("negate.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_add() {
    let program_string = include_str!("add.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_sub() {
    let program_string = include_str!("sub.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_div() {
    let program_string = include_str!("div.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_mul() {
    let program_string = include_str!("mul.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_eq() {
    let program_string = include_str!("eq.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_console_assert_pass() {
    let program_string = include_str!("console_assert.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_ternary() {
    let program_string = include_str!("ternary.leo");
    load_asg(program_string).unwrap();
}
