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
    expect_synthesis_error,
    generate_main_input,
    integers::{expect_computation_error, expect_parsing_error, IntegerTester},
    parse_program,
};
use leo_input::types::{I32Type, IntegerType, SignedIntegerType};
use leo_typed::InputValue;

test_int!(
    TestI32,
    i32,
    IntegerType::Signed(SignedIntegerType::I32Type(I32Type {})),
    Int32
);

#[test]
fn test_i32_min() {
    TestI32::test_min();
}

#[test]
fn test_i32_min_fail() {
    TestI32::test_min_fail();
}

#[test]
fn test_i32_max() {
    TestI32::test_max();
}

#[test]
fn test_i32_max_fail() {
    TestI32::test_max_fail();
}

#[test]
fn test_i32_neg() {
    TestI32::test_negate();
}

#[test]
fn test_i32_neg_max_fail() {
    TestI32::test_negate_min_fail();
}

#[test]
fn test_i32_neg_zero() {
    TestI32::test_negate_zero();
}

#[test]
fn test_i32_add() {
    TestI32::test_add();
}

#[test]
fn test_i32_sub() {
    TestI32::test_sub();
}

#[test]
fn test_i32_mul() {
    TestI32::test_mul();
}

#[test]
fn test_i32_div() {
    TestI32::test_div();
}

#[test]
fn test_i32_pow() {
    TestI32::test_pow();
}

#[test]
fn test_i32_eq() {
    TestI32::test_eq();
}

#[test]
fn test_i32_ne() {
    TestI32::test_ne();
}

#[test]
fn test_i32_ge() {
    TestI32::test_ge();
}

#[test]
fn test_i32_gt() {
    TestI32::test_gt();
}

#[test]
fn test_i32_le() {
    TestI32::test_le();
}

#[test]
fn test_i32_lt() {
    TestI32::test_lt();
}

#[test]
fn test_i32_assert_eq() {
    TestI32::test_assert_eq();
}

#[test]
fn test_i32_ternary() {
    TestI32::test_ternary();
}
