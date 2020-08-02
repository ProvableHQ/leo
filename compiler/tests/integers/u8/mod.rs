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
use leo_inputs::types::{IntegerType, U8Type};
use leo_types::InputValue;

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::{
    r1cs::TestConstraintSystem,
    utilities::{alloc::AllocGadget, uint::UInt8},
};

fn output_expected_allocated(program: EdwardsTestCompiler, expected: UInt8) {
    let output = get_output(program);

    match output {
        EdwardsConstrainedValue::Return(vec) => match vec.as_slice() {
            [ConstrainedValue::Integer(Integer::U8(actual))] => assert_eq!(*actual, expected),
            _ => panic!("program output unknown return value"),
        },
        _ => panic!("program output unknown return value"),
    }
}

test_uint!(TestU8, u8, IntegerType::U8Type(U8Type {}), UInt8);

#[test]
fn test_u8_min() {
    TestU8::test_min(std::u8::MIN);
}

#[test]
fn test_u8_max() {
    TestU8::test_max(std::u8::MAX);
}

#[test]
fn test_u8_input() {
    TestU8::test_input();
}

#[test]
fn test_u8_add() {
    TestU8::test_add();
}

#[test]
fn test_u8_sub() {
    TestU8::test_sub();
}

#[test]
fn test_u8_mul() {
    TestU8::test_mul();
}

#[test]
fn test_u8_div() {
    TestU8::test_div();
}

#[test]
fn test_u8_pow() {
    TestU8::test_pow();
}

#[test]
fn test_u8_eq() {
    TestU8::test_eq();
}

#[test]
fn test_u8_ge() {
    TestU8::test_ge();
}

#[test]
fn test_u8_gt() {
    TestU8::test_gt();
}

#[test]
fn test_u8_le() {
    TestU8::test_le();
}

#[test]
fn test_u8_lt() {
    TestU8::test_lt();
}

#[test]
fn test_u8_assert_eq() {
    TestU8::test_assert_eq();
}

#[test]
fn test_u8_ternary() {
    TestU8::test_ternary();
}
