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

use crate::{
    assert_satisfied,
    expect_asg_error,
    expect_compiler_error,
    generate_main_input,
    integers::IntegerTester,
    parse_program,
};
use leo_ast::InputValue;
use leo_input::types::{IntegerType, U16Type, UnsignedIntegerType};

test_uint!(
    TestU16,
    u16,
    IntegerType::Unsigned(UnsignedIntegerType::U16Type(U16Type {})),
    UInt16
);

#[test]
fn test_u16_min() {
    TestU16::test_min();
}

#[test]
fn test_u16_min_fail() {
    TestU16::test_min_fail();
}

#[test]
fn test_u16_max() {
    TestU16::test_max();
}

#[test]
fn test_u16_max_fail() {
    TestU16::test_max_fail();
}

#[test]
fn test_u16_add() {
    TestU16::test_add();
}

#[test]
fn test_u16_sub() {
    TestU16::test_sub();
}

#[test]
fn test_u16_mul() {
    TestU16::test_mul();
}

#[test]
fn test_u16_div() {
    TestU16::test_div();
}

#[test]
fn test_u16_pow() {
    TestU16::test_pow();
}

#[test]
fn test_u16_eq() {
    TestU16::test_eq();
}

#[test]
fn test_u16_ne() {
    TestU16::test_ne();
}

#[test]
fn test_u16_ge() {
    TestU16::test_ge();
}

#[test]
fn test_u16_gt() {
    TestU16::test_gt();
}

#[test]
fn test_u16_le() {
    TestU16::test_le();
}

#[test]
fn test_u16_lt() {
    TestU16::test_lt();
}

#[test]
fn test_u16_console_assert() {
    TestU16::test_console_assert();
}

#[test]
fn test_u16_ternary() {
    TestU16::test_ternary();
}
