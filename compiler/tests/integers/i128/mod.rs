use crate::{
    assert_satisfied,
    expect_synthesis_error,
    generate_main_input,
    integers::{expect_computation_error, expect_parsing_error, IntegerTester},
    parse_program,
};
use leo_input::types::{I128Type, IntegerType, SignedIntegerType};
use leo_typed::InputValue;

test_int!(
    TestI128,
    i128,
    IntegerType::Signed(SignedIntegerType::I128Type(I128Type {})),
    Int128
);

#[test]
fn test_i128_min() {
    TestI128::test_min();
}

#[test]
fn test_i128_min_fail() {
    TestI128::test_min_fail();
}

#[test]
fn test_i128_max() {
    TestI128::test_max();
}

#[test]
fn test_i128_max_fail() {
    TestI128::test_max_fail();
}

#[test]
fn test_i128_neg() {
    TestI128::test_negate();
}

#[test]
fn test_i128_neg_max_fail() {
    TestI128::test_negate_min_fail();
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
    TestI128::test_assert_eq();
}

#[test]
fn test_i128_ternary() {
    TestI128::test_ternary();
}
