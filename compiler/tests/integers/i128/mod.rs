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
use leo_inputs::types::{I128Type, IntegerType};
use leo_types::InputValue;

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::{r1cs::TestConstraintSystem, utilities::alloc::AllocGadget};

fn output_expected_allocated(program: EdwardsTestCompiler, expected: Int128) {
    let output = get_output(program);

    match output {
        EdwardsConstrainedValue::Return(vec) => match vec.as_slice() {
            [ConstrainedValue::Integer(Integer::I128(actual))] => assert_eq!(*actual, expected),
            _ => panic!("program output unknown return value"),
        },
        _ => panic!("program output unknown return value"),
    }
}

test_int!(TestI128, i128, IntegerType::I128Type(I128Type {}), Int128);

#[test]
fn test_i128_min() {
    TestI128::test_min(std::i128::MIN);
}

#[test]
fn test_i128_max() {
    TestI128::test_max(std::i128::MAX);
}

#[test]
fn test_i128_input() {
    TestI128::test_input();
}

#[test]
fn test_i128_add() {
    TestI128::test_add();
}

#[test]
fn test_i128_sub() {
    TestI128::test_sub();
}

#[test]
fn test_i128_mul() {
    TestI128::test_mul();
}

#[test]
#[ignore] // this test takes ~5 mins
fn test_i128_div() {
    TestI128::test_div();
}

#[test]
fn test_i128_pow() {
    TestI128::test_pow();
}

#[test]
fn test_i128_eq() {
    TestI128::test_eq();
}

#[test]
fn test_i128_ge() {
    TestI128::test_ge();
}

#[test]
fn test_i128_gt() {
    TestI128::test_gt();
}

#[test]
fn test_i128_le() {
    TestI128::test_le();
}

#[test]
fn test_i128_lt() {
    TestI128::test_lt();
}

#[test]
fn test_i128_assert_eq() {
    TestI128::test_assert_eq();
}

#[test]
fn test_i128_ternary() {
    TestI128::test_ternary();
}
