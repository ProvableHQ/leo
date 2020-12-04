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

pub mod packages;

use crate::{assert_satisfied, expect_symbol_table_error, parse_program};

#[test]
fn test_core_circuit_invalid() {
    let program_bytes = include_str!("core_package_invalid.leo");
    let program = parse_program(program_bytes).err().unwrap();

    expect_symbol_table_error(program);
}

#[test]
fn test_core_circuit_star_fail() {
    let program_bytes = include_str!("core_circuit_star_fail.leo");
    let error = parse_program(program_bytes).err().unwrap();

    expect_symbol_table_error(error);
}

#[test]
fn test_core_package_invalid() {
    let program_bytes = include_str!("core_package_invalid.leo");
    let error = parse_program(program_bytes).err().unwrap();

    expect_symbol_table_error(error);
}

#[test]
fn test_core_unstable_package_invalid() {
    let program_bytes = include_str!("core_unstable_package_invalid.leo");
    let error = parse_program(program_bytes).err().unwrap();

    expect_symbol_table_error(error);
}

#[test]
fn test_unstable_blake2s_sanity() {
    let program_bytes = include_str!("unstable_blake2s.leo");
    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}
