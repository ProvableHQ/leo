use crate::{
    assert_satisfied,
    expect_synthesis_error,
    generate_main_input,
    integers::{expect_computation_error, expect_parsing_error, IntegerTester},
    parse_program,
};
use leo_input::types::{I8Type, IntegerType};
use leo_typed::InputValue;

test_int!(Testi8, i8, IntegerType::I8Type(I8Type {}), Int8);

#[test]
fn test_i8_min() {
    Testi8::test_min();
}

#[test]
fn test_i8_min_fail() {
    Testi8::test_min_fail();
}

#[test]
fn test_i8_max() {
    Testi8::test_max();
}

#[test]
fn test_i8_max_fail() {
    Testi8::test_max_fail();
}

#[test]
fn test_i8_add() {
    Testi8::test_add();
}

#[test]
fn test_i8_sub() {
    Testi8::test_sub();
}

#[test]
fn test_i8_mul() {
    Testi8::test_mul();
}

#[test]
fn test_i8_div() {
    Testi8::test_div();
}

#[test]
fn test_i8_pow() {
    Testi8::test_pow();
}

#[test]
fn test_i8_eq() {
    Testi8::test_eq();
}

#[test]
fn test_i8_ge() {
    Testi8::test_ge();
}

#[test]
fn test_i8_gt() {
    Testi8::test_gt();
}

#[test]
fn test_i8_le() {
    Testi8::test_le();
}

#[test]
fn test_i8_lt() {
    Testi8::test_lt();
}

#[test]
fn test_i8_assert_eq() {
    Testi8::test_assert_eq();
}

#[test]
fn test_i8_ternary() {
    Testi8::test_ternary();
}
