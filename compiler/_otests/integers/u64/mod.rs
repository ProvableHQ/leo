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

use crate::{
    assert_satisfied,
    expect_asg_error,
    expect_compiler_error,
    generate_main_input,
    integers::IntegerTester,
    parse_program,
};
use leo_ast::InputValue;
use leo_input::types::{IntegerType, U64Type, UnsignedIntegerType};

test_uint!(
    TestU64,
    u64,
    IntegerType::Unsigned(UnsignedIntegerType::U64Type(U64Type {})),
    UInt64
);

#[test]
fn test_u64_min() {
    TestU64::test_min();
}

#[test]
fn test_u64_min_fail() {
    TestU64::test_min_fail();
}

#[test]
fn test_u64_max() {
    TestU64::test_max();
}

#[test]
fn test_u64_max_fail() {
    TestU64::test_max_fail();
}

#[test]
fn test_u64_add() {
    TestU64::test_add();
}

#[test]
fn test_u64_sub() {
    TestU64::test_sub();
}

#[test]
fn test_u64_mul() {
    TestU64::test_mul();
}

#[test]
fn test_u64_div() {
    TestU64::test_div();
}

#[test]
fn test_u64_pow() {
    TestU64::test_pow();
}

#[test]
fn test_u64_eq() {
    TestU64::test_eq();
}

#[test]
fn test_u64_ne() {
    TestU64::test_ne();
}

#[test]
fn test_u64_ge() {
    TestU64::test_ge();
}

#[test]
fn test_u64_gt() {
    TestU64::test_gt();
}

#[test]
fn test_u64_le() {
    TestU64::test_le();
}

#[test]
fn test_u64_lt() {
    TestU64::test_lt();
}

#[test]
fn test_u64_console_assert() {
    TestU64::test_console_assert();
}

#[test]
fn test_u64_ternary() {
    TestU64::test_ternary();
}

#[test]
fn test_no_space_between_literal() {
    let program_string = include_str!("no_space_between_literal.leo");
    let program = parse_program(program_string);

    assert!(program.is_err());
}
