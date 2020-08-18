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

use crate::{assert_satisfied, parse_program_with_state, parse_state};

#[test]
fn test_basic() {
    let bytes = include_bytes!("input/basic.state");

    parse_state(bytes).unwrap();
}

#[test]
fn test_token_withdraw() {
    let bytes = include_bytes!("input/token_withdraw.state");

    parse_state(bytes).unwrap();
}

#[test]
fn test_access_state() {
    let program_bytes = include_bytes!("access_state.leo");
    let state_bytes = include_bytes!("input/token_withdraw.state");

    let program = parse_program_with_state(program_bytes, state_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_access_all() {
    let program_bytes = include_bytes!("access_all.leo");
    let state_bytes = include_bytes!("input/token_withdraw.state");

    let program = parse_program_with_state(program_bytes, state_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_visibility_fail() {
    let state_bytes = include_bytes!("input/visibility_fail.state");

    let is_err = parse_state(state_bytes).is_err();

    assert!(is_err);
}

#[test]
fn test_section_undefined() {
    let state_bytes = include_bytes!("input/section_undefined.state");

    let is_err = parse_state(state_bytes).is_err();

    assert!(is_err);
}

#[test]
fn test_section_invalid() {
    let state_bytes = include_bytes!("input/section_invalid.state");

    let is_err = parse_state(state_bytes).is_err();

    assert!(is_err);
}
