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
use leo_inputs::types::{IntegerType, i8Type};
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
            [ConstrainedValue::Integer(Integer::i8(actual))] => assert_eq!(*actual, expected),
            _ => panic!("program output unknown return value"),
        },
        _ => panic!("program output unknown return value"),
    }
}

test_int!(Testi8, i8, IntegerType::i8Type(i8Type {}), UInt8);

#[test]
fn test_i8_min() {
    Testi8::test_min(std::i8::MIN);
}

#[test]
fn test_i8_max() {
    Testi8::test_max(std::i8::MAX);
}

#[test]
fn test_i8_input() {
    Testi8::test_input();
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
