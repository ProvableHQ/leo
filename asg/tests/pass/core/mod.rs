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
fn test_unstable_blake2s() {
    let program_string = include_str!("unstable_blake2s.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_blake2s_input() {
    let program_string = include_str!("blake2s_input.leo");
    load_asg(program_string).unwrap();
}

#[test]
fn test_blake2s_random() {
    let program_string = include_str!("blake2s_random.leo");
    load_asg(program_string).unwrap();
}
