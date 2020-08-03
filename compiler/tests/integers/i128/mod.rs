use crate::{
    assert_satisfied,
    expect_synthesis_error,
    generate_main_input,
    integers::{expect_computation_error, expect_parsing_error, IntegerTester},
    parse_program,
};
use leo_input::types::{I128Type, IntegerType};
use leo_typed::InputValue;

test_int!(Testi128, i128, IntegerType::I128Type(I128Type {}), Int128);

#[test]
fn test_i128_min() {
    Testi128::test_min();
}

#[test]
fn test_i128_min_fail() {
    Testi128::test_min_fail();
}

#[test]
fn test_i128_max() {
    Testi128::test_max();
}

#[test]
fn test_i128_max_fail() {
    Testi128::test_max_fail();
}

#[test]
fn test_i128_add() {
    Testi128::test_add();
}

#[test]
fn test_i128_sub() {
    Testi128::test_sub();
}

#[test]
fn test_i128_mul() {
    Testi128::test_mul();
}

#[test]
#[ignore] // takes several minutes
fn test_i128_div() {
    Testi128::test_div();
}

#[test]
fn test_i128_pow() {
    Testi128::test_pow();
}

#[test]
fn test_i128_eq() {
    Testi128::test_eq();
}

#[test]
fn test_i128_ge() {
    Testi128::test_ge();
}

#[test]
fn test_i128_gt() {
    Testi128::test_gt();
}

#[test]
fn test_i128_le() {
    Testi128::test_le();
}

#[test]
fn test_i128_lt() {
    Testi128::test_lt();
}

#[test]
fn test_i128_assert_eq() {
    Testi128::test_assert_eq();
}

#[test]
fn test_i128_ternary() {
    Testi128::test_ternary();
}
