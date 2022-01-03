// Copyright (C) 2019-2022 Aleo Systems Inc.
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
fn test_not_u32() {
    let program_string = include_str!("not_u32.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_true_or_u32() {
    let program_string = include_str!("true_or_u32.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_true_and_u32() {
    let program_string = include_str!("true_and_u32.leo");
    load_asg(program_string).err().unwrap();
}
