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
fn test_valid() {
    let program_string = include_str!("valid.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_implicit_valid() {
    let program_string = include_str!("implicit_valid.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_console_assert_pass() {
    let program_string = include_str!("console_assert_pass.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_ternary() {
    let program_string = include_str!("ternary.leo");

    load_asg(program_string).unwrap();
}

#[test]
fn test_equal() {
    let program_string = include_str!("equal.leo");
    load_asg(program_string).unwrap();
}
