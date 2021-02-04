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

use super::IntegerTester;

test_uint!(TestU8);

#[test]
fn test_u8_min() {
    TestU8::test_min();
}

#[test]
fn test_u8_max() {
    TestU8::test_max();
}

#[test]
fn test_u8_add() {
    TestU8::test_add();
}

#[test]
fn test_u8_sub() {
    TestU8::test_sub();
}

#[test]
fn test_u8_mul() {
    TestU8::test_mul();
}

#[test]
fn test_u8_div() {
    TestU8::test_div();
}

#[test]
fn test_u8_pow() {
    TestU8::test_pow();
}

#[test]
fn test_u8_eq() {
    TestU8::test_eq();
}

#[test]
fn test_u8_ne() {
    TestU8::test_ne();
}

#[test]
fn test_u8_ge() {
    TestU8::test_ge();
}

#[test]
fn test_u8_gt() {
    TestU8::test_gt();
}

#[test]
fn test_u8_le() {
    TestU8::test_le();
}

#[test]
fn test_u8_lt() {
    TestU8::test_lt();
}

#[test]
fn test_u8_console_assert() {
    TestU8::test_console_assert();
}

#[test]
fn test_u8_ternary() {
    TestU8::test_ternary();
}
