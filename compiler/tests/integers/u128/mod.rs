use crate::{
    assert_satisfied,
    expect_synthesis_error,
    generate_main_input,
    integers::{expect_parsing_error, IntegerTester},
    parse_program,
};
use leo_input::types::{IntegerType, U128Type};
use leo_typed::InputValue;

test_uint!(TestU128, u128, IntegerType::U128Type(U128Type {}), UInt128);

#[test]
fn test_u128_min() {
    TestU128::test_min();
}

#[test]
fn test_u128_min_fail() {
    TestU128::test_min_fail();
}

#[test]
fn test_u128_max() {
    TestU128::test_max();
}

#[test]
fn test_u128_max_fail() {
    TestU128::test_max_fail();
}

#[test]
fn test_u128_add() {
    TestU128::test_add();
}

#[test]
fn test_u128_sub() {
    TestU128::test_sub();
}

#[test]
fn test_u128_mul() {
    TestU128::test_mul();
}

#[test]
fn test_u128_div() {
    TestU128::test_div();
}

#[test]
fn test_u128_pow() {
    TestU128::test_pow();
}

#[test]
fn test_u128_eq() {
    TestU128::test_eq();
}

#[test]
fn test_u128_ge() {
    TestU128::test_ge();
}

#[test]
fn test_u128_gt() {
    TestU128::test_gt();
}

#[test]
fn test_u128_le() {
    TestU128::test_le();
}

#[test]
fn test_u128_lt() {
    TestU128::test_lt();
}

#[test]
fn test_u128_assert_eq() {
    TestU128::test_assert_eq();
}

#[test]
fn test_u128_ternary() {
    TestU128::test_ternary();
}
