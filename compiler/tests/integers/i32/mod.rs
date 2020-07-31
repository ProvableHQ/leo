use crate::{
    assert_satisfied,
    expect_synthesis_error,
    generate_main_inputs,
    integers::{expect_computation_error, expect_parsing_error, IntegerTester},
    parse_program,
};
use leo_inputs::types::{I32Type, IntegerType};
use leo_types::InputValue;

test_int!(Testi32, i32, IntegerType::I32Type(I32Type {}), Int32);

#[test]
fn test_i32_min() {
    Testi32::test_min();
}

#[test]
fn test_i32_min_fail() {
    Testi32::test_min_fail();
}

#[test]
fn test_i32_max() {
    Testi32::test_max();
}

#[test]
fn test_i32_max_fail() {
    Testi32::test_max_fail();
}

#[test]
fn test_i32_add() {
    Testi32::test_add();
}

#[test]
fn test_i32_sub() {
    Testi32::test_sub();
}

#[test]
fn test_i32_mul() {
    Testi32::test_mul();
}

#[test]
fn test_i32_div() {
    Testi32::test_div();
}

#[test]
fn test_i32_pow() {
    Testi32::test_pow();
}

#[test]
fn test_i32_eq() {
    Testi32::test_eq();
}

#[test]
fn test_i32_ge() {
    Testi32::test_ge();
}

#[test]
fn test_i32_gt() {
    Testi32::test_gt();
}

#[test]
fn test_i32_le() {
    Testi32::test_le();
}

#[test]
fn test_i32_lt() {
    Testi32::test_lt();
}

#[test]
fn test_i32_assert_eq() {
    Testi32::test_assert_eq();
}

#[test]
fn test_i32_ternary() {
    Testi32::test_ternary();
}
