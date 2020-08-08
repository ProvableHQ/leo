use crate::{
    assert_satisfied,
    expect_synthesis_error,
    generate_main_input,
    integers::{expect_computation_error, expect_parsing_error, IntegerTester},
    parse_program,
};
use leo_input::types::{I8Type, IntegerType, SignedIntegerType};
use leo_typed::InputValue;

test_int!(
    TestI8,
    i8,
    IntegerType::Signed(SignedIntegerType::I8Type(I8Type {})),
    Int8
);

#[test]
fn test_i8_min() {
    TestI8::test_min();
}

#[test]
fn test_i8_min_fail() {
    TestI8::test_min_fail();
}

#[test]
fn test_i8_max() {
    TestI8::test_max();
}

#[test]
fn test_i8_max_fail() {
    TestI8::test_max_fail();
}

#[test]
fn test_i8_neg() {
    TestI8::test_negate();
}

#[test]
fn test_i8_neg_max_fail() {
    TestI8::test_negate_min_fail();
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
fn test_i8_assert_eq() {
    TestI8::test_assert_eq();
}

#[test]
fn test_i8_ternary() {
    TestI8::test_ternary();
}
