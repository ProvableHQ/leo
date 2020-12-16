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

use super::IntegerTester;

test_int!(
    TestI8
);

#[test]
fn test_i8_min() {
    TestI8::test_min();
}

#[test]
fn test_i8_max() {
    TestI8::test_max();
}

#[test]
fn test_i8_neg() {
    TestI8::test_negate();
}

#[test]
fn test_i8_neg_zero() {
    TestI8::test_negate_zero();
}

#[test]
fn test_i8_add() {
    TestI8::test_add();
}

#[test]
fn test_i8_sub() {
    TestI8::test_sub();
}

#[test]
fn test_i8_mul() {
    TestI8::test_mul();
}

#[test]
fn test_i8_div() {
    TestI8::test_div();
}

#[test]
fn test_i8_pow() {
    TestI8::test_pow();
}

#[test]
fn test_i8_eq() {
    TestI8::test_eq();
}

#[test]
fn test_i8_ne() {
    TestI8::test_ne();
}

#[test]
fn test_i8_ge() {
    TestI8::test_ge();
}

#[test]
fn test_i8_gt() {
    TestI8::test_gt();
}

#[test]
fn test_i8_le() {
    TestI8::test_le();
}

#[test]
fn test_i8_lt() {
    TestI8::test_lt();
}

#[test]
fn test_i8_console_assert() {
    TestI8::test_console_assert();
}

#[test]
fn test_i8_ternary() {
    TestI8::test_ternary();
}
