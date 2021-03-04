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

use crate::{assert_satisfied, parse_program_with_state, parse_state};

#[test]
fn test_basic() {
    let state_string = include_str!("input/basic.state");

    parse_state(state_string).unwrap();
}

#[test]
fn test_token_withdraw() {
    let state_string = include_str!("input/token_withdraw.state");

    parse_state(state_string).unwrap();
}

#[test]
fn test_access_state() {
    let program_string = include_str!("access_state.leo");
    let state_string = include_str!("input/token_withdraw.state");

    let program = parse_program_with_state(program_string, state_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_access_all() {
    let program_string = include_str!("access_all.leo");
    let state_string = include_str!("input/token_withdraw.state");

    let program = parse_program_with_state(program_string, state_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_visibility_fail() {
    let state_string = include_str!("input/visibility_fail.state");

    let is_err = parse_state(state_string).is_err();

    assert!(is_err);
}

#[test]
fn test_section_undefined() {
    let state_string = include_str!("input/section_undefined.state");

    let is_err = parse_state(state_string).is_err();

    assert!(is_err);
}

#[test]
fn test_section_invalid() {
    let state_string = include_str!("input/section_invalid.state");

    let is_err = parse_state(state_string).is_err();

    assert!(is_err);
}
