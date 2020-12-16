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
    TestI128
);

#[test]
fn test_i128_min() {
    TestI128::test_min();
}

#[test]
fn test_i128_max() {
    TestI128::test_max();
}

#[test]
fn test_i128_neg() {
    TestI128::test_negate();
}

#[test]
fn test_i128_neg_zero() {
    TestI128::test_negate_zero();
}

#[test]
fn test_i128_add() {
    TestI128::test_add();
}

#[test]
fn test_i128_sub() {
    TestI128::test_sub();
}

#[test]
fn test_i128_mul() {
    TestI128::test_mul();
}

#[test]
#[ignore] // takes several minutes
fn test_i128_div() {
    TestI128::test_div();
}

#[test]
fn test_i128_pow() {
    TestI128::test_pow();
}

#[test]
fn test_i128_eq() {
    TestI128::test_eq();
}

#[test]
fn test_i128_ne() {
    TestI128::test_ne();
}

#[test]
fn test_i128_ge() {
    TestI128::test_ge();
}

#[test]
fn test_i128_gt() {
    TestI128::test_gt();
}

#[test]
fn test_i128_le() {
    TestI128::test_le();
}

#[test]
fn test_i128_lt() {
    TestI128::test_lt();
}

#[test]
fn test_i128_assert_eq() {
    TestI128::test_console_assert();
}

#[test]
fn test_i128_ternary() {
    TestI128::test_ternary();
}
