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
use leo_inputs::types::{I64Type, IntegerType};
use leo_types::InputValue;

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::{r1cs::TestConstraintSystem, utilities::alloc::AllocGadget};

fn output_expected_allocated(program: EdwardsTestCompiler, expected: Int64) {
    let output = get_output(program);

    match output {
        EdwardsConstrainedValue::Return(vec) => match vec.as_slice() {
            [ConstrainedValue::Integer(Integer::I64(actual))] => assert_eq!(*actual, expected),
            _ => panic!("program output unknown return value"),
        },
        _ => panic!("program output unknown return value"),
    }
}

test_int!(TestI64, i64, IntegerType::I64Type(I64Type {}), Int64);

#[test]
fn test_i64_min() {
    TestI64::test_min(std::i64::MIN);
}

#[test]
fn test_i64_max() {
    TestI64::test_max(std::i64::MAX);
}

#[test]
fn test_i64_input() {
    TestI64::test_input();
}

#[test]
fn test_i64_add() {
    TestI64::test_add();
}

#[test]
fn test_i64_sub() {
    TestI64::test_sub();
}

#[test]
fn test_i64_mul() {
    TestI64::test_mul();
}

#[test]
// #[ignore] // this test takes ~1 min
fn test_i64_div() {
    TestI64::test_div();
}

#[test]
fn test_i64_pow() {
    TestI64::test_pow();
}

#[test]
fn test_i64_eq() {
    TestI64::test_eq();
}

#[test]
fn test_i64_ge() {
    TestI64::test_ge();
}

#[test]
fn test_i64_gt() {
    TestI64::test_gt();
}

#[test]
fn test_i64_le() {
    TestI64::test_le();
}

#[test]
fn test_i64_lt() {
    TestI64::test_lt();
}

#[test]
fn test_i64_assert_eq() {
    TestI64::test_assert_eq();
}

#[test]
fn test_i64_ternary() {
    TestI64::test_ternary();
}
