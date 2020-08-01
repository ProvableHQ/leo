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
use leo_inputs::types::{I32Type, IntegerType};
use leo_typed::InputValue;

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::{r1cs::TestConstraintSystem, utilities::alloc::AllocGadget};

fn output_expected_allocated(program: EdwardsTestCompiler, expected: Int32) {
    let output = get_output(program);

    match output {
        EdwardsConstrainedValue::Return(vec) => match vec.as_slice() {
            [ConstrainedValue::Integer(Integer::I32(actual))] => assert_eq!(*actual, expected),
            _ => panic!("program output unknown return value"),
        },
        _ => panic!("program output unknown return value"),
    }
}

test_int!(TestI32, i32, IntegerType::I32Type(I32Type {}), Int32);

#[test]
fn test_i32_min() {
    TestI32::test_min(std::i32::MIN);
}

#[test]
fn test_i32_max() {
    TestI32::test_max(std::i32::MAX);
}

#[test]
fn test_i32_input() {
    TestI32::test_input();
}

#[test]
fn test_i32_add() {
    TestI32::test_add();
}

#[test]
fn test_i32_sub() {
    TestI32::test_sub();
}

#[test]
fn test_i32_mul() {
    TestI32::test_mul();
}

#[test]
fn test_i32_div() {
    TestI32::test_div();
}

#[test]
fn test_i32_pow() {
    TestI32::test_pow();
}

#[test]
fn test_i32_eq() {
    TestI32::test_eq();
}

#[test]
fn test_i32_ge() {
    TestI32::test_ge();
}

#[test]
fn test_i32_gt() {
    TestI32::test_gt();
}

#[test]
fn test_i32_le() {
    TestI32::test_le();
}

#[test]
fn test_i32_lt() {
    TestI32::test_lt();
}

#[test]
fn test_i32_assert_eq() {
    TestI32::test_assert_eq();
}

#[test]
fn test_i32_ternary() {
    TestI32::test_ternary();
}
