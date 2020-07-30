use crate::{
    assert_satisfied,
    expect_synthesis_error,
    generate_main_inputs,
    integers::{expect_fail, IntegerTester},
    parse_program,
};
use leo_inputs::types::{I64Type, IntegerType};
use leo_types::InputValue;

test_int!(Testi64, i64, IntegerType::I64Type(I64Type {}), Int64);

#[test]
fn test_i64_min() {
    Testi64::test_min();
}

#[test]
fn test_i64_min_fail() {
    Testi64::test_min_fail();
}

#[test]
fn test_i64_max() {
    Testi64::test_max();
}

#[test]
fn test_i64_max_fail() {
    Testi64::test_max_fail();
}

#[test]
fn test_i64_add() {
    Testi64::test_add();
}

#[test]
fn test_i64_sub() {
    Testi64::test_sub();
}

#[test]
fn test_i64_mul() {
    Testi64::test_mul();
}

#[test] // takes 90 seconds
fn test_i64_div() {
    Testi64::test_div();
}

#[test]
fn test_i64_pow() {
    Testi64::test_pow();
}

#[test]
fn test_i64_eq() {
    Testi64::test_eq();
}

#[test]
fn test_i64_ge() {
    Testi64::test_ge();
}

#[test]
fn test_i64_gt() {
    Testi64::test_gt();
}

#[test]
fn test_i64_le() {
    Testi64::test_le();
}

#[test]
fn test_i64_lt() {
    Testi64::test_lt();
}

#[test]
fn test_i64_assert_eq() {
    Testi64::test_assert_eq();
}

#[test]
fn test_i64_ternary() {
    Testi64::test_ternary();
}
