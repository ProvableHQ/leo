use crate::{
    boolean::{output_expected_boolean, output_false, output_true},
    get_error,
    get_output,
    integers::{fail_integer, IntegerTester},
    parse_program,
    EdwardsConstrainedValue,
    EdwardsTestCompiler,
};
use leo_compiler::{ConstrainedValue, Integer};
use leo_gadgets::*;
use leo_inputs::types::{I8Type, IntegerType};
use leo_typed::InputValue;

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::{r1cs::TestConstraintSystem, utilities::alloc::AllocGadget};

fn output_expected_allocated(program: EdwardsTestCompiler, expected: Int8) {
    let output = get_output(program);

    match output {
        EdwardsConstrainedValue::Return(vec) => match vec.as_slice() {
            [ConstrainedValue::Integer(Integer::I8(actual))] => assert_eq!(*actual, expected),
            _ => panic!("program output unknown return value"),
        },
        _ => panic!("program output unknown return value"),
    }
}

test_int!(TestI8, i8, IntegerType::I8Type(I8Type {}), Int8);

#[test]
fn test_i8_min() {
    TestI8::test_min(std::i8::MIN);
}

#[test]
fn test_i8_max() {
    TestI8::test_max(std::i8::MAX);
}

#[test]
fn test_i8_input() {
    TestI8::test_input();
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
