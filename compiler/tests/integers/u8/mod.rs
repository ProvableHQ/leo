use crate::{
    assert_satisfied,
    expect_synthesis_error,
    generate_main_input,
    integers::{expect_parsing_error, IntegerTester},
    parse_program,
};
use leo_input::types::{IntegerType, U8Type};
use leo_types::InputValue;

test_uint!(TestU8, u8, IntegerType::U8Type(U8Type {}), UInt8);

#[test]
fn test_u8_min() {
    TestU8::test_min();
}

#[test]
fn test_u8_min_fail() {
    TestU8::test_min_fail();
}

#[test]
fn test_u8_max() {
    TestU8::test_max();
}

#[test]
fn test_u8_max_fail() {
    TestU8::test_max_fail();
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
fn test_u8_assert_eq() {
    TestU8::test_assert_eq();
}

#[test]
fn test_u8_ternary() {
    TestU8::test_ternary();
}
