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
    expect_compiler_error,
    generate_main_input,
    integers::{IntegerTester},
    expect_asg_error,
    parse_program,
};
use leo_ast::InputValue;
use leo_input::types::{IntegerType, U128Type, UnsignedIntegerType};

test_uint!(
    TestU128,
    u128,
    IntegerType::Unsigned(UnsignedIntegerType::U128Type(U128Type {})),
    UInt128
);

#[test]
fn test_u128_min() {
    TestU128::test_min();
}

#[test]
fn test_u128_min_fail() {
    TestU128::test_min_fail();
}

#[test]
fn test_u128_max() {
    TestU128::test_max();
}

#[test]
fn test_u128_max_fail() {
    TestU128::test_max_fail();
}

#[test]
fn test_u128_add() {
    TestU128::test_add();
}

#[test]
fn test_u128_sub() {
    TestU128::test_sub();
}

#[test]
fn test_u128_mul() {
    TestU128::test_mul();
}

#[test]
fn test_u128_div() {
    TestU128::test_div();
}

#[test]
fn test_u128_pow() {
    TestU128::test_pow();
}

#[test]
fn test_u128_eq() {
    TestU128::test_eq();
}

#[test]
fn test_u128_ne() {
    TestU128::test_ne();
}

#[test]
fn test_u128_ge() {
    TestU128::test_ge();
}

#[test]
fn test_u128_gt() {
    TestU128::test_gt();
}

#[test]
fn test_u128_le() {
    TestU128::test_le();
}

#[test]
fn test_u128_lt() {
    TestU128::test_lt();
}

#[test]
fn test_u128_console_assert() {
    TestU128::test_console_assert();
}

#[test]
fn test_u128_ternary() {
    TestU128::test_ternary();
}
