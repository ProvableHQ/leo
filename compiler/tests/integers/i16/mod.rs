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
use leo_inputs::types::{I16Type, IntegerType};
use leo_typed::InputValue;

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::{r1cs::TestConstraintSystem, utilities::alloc::AllocGadget};

fn output_expected_allocated(program: EdwardsTestCompiler, expected: Int16) {
    let output = get_output(program);

    match output {
        EdwardsConstrainedValue::Return(vec) => match vec.as_slice() {
            [ConstrainedValue::Integer(Integer::I16(actual))] => assert_eq!(*actual, expected),
            _ => panic!("program output unknown return value"),
        },
        _ => panic!("program output unknown return value"),
    }
}

test_int!(TestI16, i16, IntegerType::I16Type(I16Type {}), Int16);

#[test]
fn test_i16_min() {
    TestI16::test_min(std::i16::MIN);
}

#[test]
fn test_i16_max() {
    TestI16::test_max(std::i16::MAX);
}

#[test]
fn test_i16_input() {
    TestI16::test_input();
}

#[test]
fn test_i16_add() {
    TestI16::test_add();
}

#[test]
fn test_i16_sub() {
    TestI16::test_sub();
}

#[test]
fn test_i16_mul() {
    TestI16::test_mul();
}

#[test]
fn test_i16_div() {
    TestI16::test_div();
}

#[test]
fn test_i16_pow() {
    TestI16::test_pow();
}

#[test]
fn test_i16_eq() {
    TestI16::test_eq();
}

#[test]
fn test_i16_ge() {
    TestI16::test_ge();
}

#[test]
fn test_i16_gt() {
    TestI16::test_gt();
}

#[test]
fn test_i16_le() {
    TestI16::test_le();
}

#[test]
fn test_i16_lt() {
    TestI16::test_lt();
}

#[test]
fn test_i16_assert_eq() {
    TestI16::test_assert_eq();
}

#[test]
fn test_i16_ternary() {
    TestI16::test_ternary();
}
