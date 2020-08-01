use crate::{
    assert_satisfied,
    expect_synthesis_error,
    generate_main_inputs,
    integers::{expect_parsing_error, IntegerTester},
    parse_program,
};
use leo_input::types::{IntegerType, U64Type};
use leo_types::InputValue;

test_uint!(TestU64, u64, IntegerType::U64Type(U64Type {}), UInt64);

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
fn test_u64_assert_eq() {
    TestU64::test_assert_eq();
}

#[test]
fn test_u64_ternary() {
    TestU64::test_ternary();
}
