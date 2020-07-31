use crate::{
    assert_satisfied,
    expect_synthesis_error,
    generate_main_inputs,
    integers::{expect_computation_error, expect_parsing_error, IntegerTester},
    parse_program,
};
use leo_inputs::types::{I16Type, IntegerType};
use leo_types::InputValue;

test_int!(Testi16, i16, IntegerType::I16Type(I16Type {}), Int16);

#[test]
fn test_i16_min() {
    Testi16::test_min();
}

#[test]
fn test_i16_min_fail() {
    Testi16::test_min_fail();
}

#[test]
fn test_i16_max() {
    Testi16::test_max();
}

#[test]
fn test_i16_max_fail() {
    Testi16::test_max_fail();
}

#[test]
fn test_i16_add() {
    Testi16::test_add();
}

#[test]
fn test_i16_sub() {
    Testi16::test_sub();
}

#[test]
fn test_i16_mul() {
    Testi16::test_mul();
}

#[test]
fn test_i16_div() {
    Testi16::test_div();
}

#[test]
fn test_i16_pow() {
    Testi16::test_pow();
}

#[test]
fn test_i16_eq() {
    Testi16::test_eq();
}

#[test]
fn test_i16_ge() {
    Testi16::test_ge();
}

#[test]
fn test_i16_gt() {
    Testi16::test_gt();
}

#[test]
fn test_i16_le() {
    Testi16::test_le();
}

#[test]
fn test_i16_lt() {
    Testi16::test_lt();
}

#[test]
fn test_i16_assert_eq() {
    Testi16::test_assert_eq();
}

#[test]
fn test_i16_ternary() {
    Testi16::test_ternary();
}
