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
    integers::{expect_computation_error, expect_parsing_error, IntegerTester},
    parse_program,
};
use leo_input::types::{I16Type, IntegerType, SignedIntegerType};
use leo_typed::InputValue;

test_int!(
    TestI16,
    i16,
    IntegerType::Signed(SignedIntegerType::I16Type(I16Type {})),
    Int16
);

#[test]
fn test_i16_min() {
    TestI16::test_min();
}

#[test]
fn test_i16_min_fail() {
    TestI16::test_min_fail();
}

#[test]
fn test_i16_max() {
    TestI16::test_max();
}

#[test]
fn test_i16_max_fail() {
    TestI16::test_max_fail();
}

#[test]
fn test_i16_neg() {
    TestI16::test_negate();
}

#[test]
fn test_i16_neg_max_fail() {
    TestI16::test_negate_min_fail();
}

#[test]
fn test_i16_neg_zero() {
    TestI16::test_negate_zero();
}

#[test]
fn test_i16_add() {
    TestI16::test_add();
}

#[test]
fn test_i16_sub() {
    TestI16::test_sub();
}

#[test]
fn test_i16_mul() {
    TestI16::test_mul();
}

#[test]
fn test_i16_div() {
    TestI16::test_div();
}

#[test]
fn test_i16_pow() {
    TestI16::test_pow();
}

#[test]
fn test_i16_eq() {
    TestI16::test_eq();
}

#[test]
fn test_i16_ne() {
    TestI16::test_ne();
}

#[test]
fn test_i16_ge() {
    TestI16::test_ge();
}

#[test]
fn test_i16_gt() {
    TestI16::test_gt();
}

#[test]
fn test_i16_le() {
    TestI16::test_le();
}

#[test]
fn test_i16_lt() {
    TestI16::test_lt();
}

#[test]
fn test_i16_console_assert() {
    TestI16::test_console_assert();
}

#[test]
fn test_i16_ternary() {
    TestI16::test_ternary();
}
