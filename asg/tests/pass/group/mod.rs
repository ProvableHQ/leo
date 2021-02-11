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

use leo_asg::new_context;

use crate::load_asg;

#[test]
fn test_one() {
    let program_string = include_str!("one.leo");
    load_asg(&new_context(), program_string).unwrap();
}

#[test]
fn test_implicit() {
    let program_string = r#"
    function main() {
        let element: group = 0;
    }
    "#;
    load_asg(&new_context(), program_string).unwrap();
}

#[test]
fn test_zero() {
    let program_string = include_str!("zero.leo");
    load_asg(&new_context(), program_string).unwrap();
}

#[test]
fn test_point() {
    let program_string = include_str!("point.leo");
    load_asg(&new_context(), program_string).unwrap();
}

#[test]
fn test_x_sign_high() {
    let program_string = include_str!("x_sign_high.leo");
    load_asg(&new_context(), program_string).unwrap();
}

#[test]
fn test_x_sign_low() {
    let program_string = include_str!("x_sign_low.leo");
    load_asg(&new_context(), program_string).unwrap();
}

#[test]
fn test_x_sign_inferred() {
    let program_string = include_str!("x_sign_inferred.leo");
    load_asg(&new_context(), program_string).unwrap();
}

#[test]
fn test_y_sign_high() {
    let program_string = include_str!("y_sign_high.leo");
    load_asg(&new_context(), program_string).unwrap();
}

#[test]
fn test_y_sign_low() {
    let program_string = include_str!("y_sign_low.leo");
    load_asg(&new_context(), program_string).unwrap();
}

#[test]
fn test_y_sign_inferred() {
    let program_string = include_str!("y_sign_inferred.leo");
    load_asg(&new_context(), program_string).unwrap();
}

#[test]
fn test_point_input() {
    let program_string = include_str!("point_input.leo");
    load_asg(&new_context(), program_string).unwrap();
}

#[test]
fn test_input() {
    let program_string = include_str!("input.leo");
    load_asg(&new_context(), program_string).unwrap();
}

#[test]
fn test_negate() {
    let program_string = include_str!("negate.leo");
    load_asg(&new_context(), program_string).unwrap();
}

#[test]
fn test_add() {
    let program_string = include_str!("add.leo");
    load_asg(&new_context(), program_string).unwrap();
}

#[test]
fn test_add_explicit() {
    let program_string = r#"
    function main() {
        let c: group = 0group + 1group;
    }
    "#;
    load_asg(program_string).unwrap();
}

#[test]
fn test_sub() {
    let program_string = include_str!("sub.leo");
    load_asg(&new_context(), program_string).unwrap();
}

#[test]
fn test_console_assert_pass() {
    let program_string = include_str!("assert_eq.leo");
    load_asg(&new_context(), program_string).unwrap();
}

#[test]
fn test_eq() {
    let program_string = include_str!("eq.leo");
    load_asg(&new_context(), program_string).unwrap();
}

#[test]
fn test_ternary() {
    let program_string = include_str!("ternary.leo");
    load_asg(&new_context(), program_string).unwrap();
}
