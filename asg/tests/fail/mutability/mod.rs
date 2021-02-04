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
fn test_let() {
    let program_string = include_str!("let.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_const_fail() {
    let program_string = include_str!("const.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_const_mut_fail() {
    let program_string = include_str!("const_mut.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_array() {
    let program_string = include_str!("array.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_circuit() {
    let program_string = include_str!("circuit.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_circuit_function_mut() {
    let program_string = include_str!("circuit_function_mut.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_circuit_static_function_mut() {
    let program_string = include_str!("circuit_static_function_mut.leo");
    load_asg(program_string).err().unwrap();
}

#[test]
fn test_function_input() {
    let program_string = include_str!("function_input.leo");
    load_asg(program_string).err().unwrap();
}
