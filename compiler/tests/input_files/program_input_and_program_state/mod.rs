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

use crate::assert_satisfied;
use crate::parse_input_and_state;
use crate::parse_program_with_input_and_state;

#[test]
fn test_basic() {
    let input_string = include_str!("input/basic.in");
    let state_string = include_str!("input/basic.state");

    parse_input_and_state(input_string, state_string).unwrap();
}

#[test]
fn test_full() {
    let input_string = include_str!("input/token_withdraw.in");
    let state_string = include_str!("input/token_withdraw.state");

    parse_input_and_state(input_string, state_string).unwrap();
}

#[test]
fn test_access() {
    let program_string = include_str!("access.leo");
    let input_string = include_str!("input/token_withdraw.in");
    let state_string = include_str!("input/token_withdraw.state");

    let program = parse_program_with_input_and_state(program_string, input_string, state_string).unwrap();

    assert_satisfied(program);
}
