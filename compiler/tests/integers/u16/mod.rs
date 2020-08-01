use crate::{
    assert_satisfied,
    expect_synthesis_error,
    generate_main_inputs,
    integers::{expect_parsing_error, IntegerTester},
    parse_program,
};
use leo_input::types::{IntegerType, U16Type};
use leo_types::InputValue;

test_uint!(TestU16, u16, IntegerType::U16Type(U16Type {}), UInt16);

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
fn test_u16_assert_eq() {
    TestU16::test_assert_eq();
}

#[test]
fn test_u16_ternary() {
    TestU16::test_ternary();
}
