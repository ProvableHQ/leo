use crate::{
    compile_program, get_error, get_output, integer::IntegerTester, EdwardsConstrainedValue,
    EdwardsTestCompiler,
};
use leo_compiler::{
    errors::{CompilerError, FunctionError, IntegerError},
    types::Integer,
    ConstrainedValue, InputValue,
};

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::r1cs::TestConstraintSystem;
use snarkos_models::gadgets::utilities::{alloc::AllocGadget, uint::UInt32};

const DIRECTORY_NAME: &str = "tests/integer/u32/";

fn output_expected_allocated(program: EdwardsTestCompiler, expected: UInt32) {
    let output = get_output(program);

    match output {
        EdwardsConstrainedValue::Return(vec) => match vec.as_slice() {
            [ConstrainedValue::Integer(Integer::U32(actual))] => assert_eq!(*actual, expected),
            _ => panic!("program output unknown return value"),
        },
        _ => panic!("program output unknown return value"),
    }
}

pub(crate) fn output_zero(program: EdwardsTestCompiler) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Integer(Integer::U32(
            UInt32::constant(0u32)
        ))])
        .to_string(),
        output.to_string()
    )
}

pub(crate) fn output_one(program: EdwardsTestCompiler) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Integer(Integer::U32(
            UInt32::constant(1u32)
        ))])
        .to_string(),
        output.to_string()
    )
}

fn fail_integer(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::IntegerError(
            IntegerError::InvalidInteger(_string),
        )) => {}
        error => panic!("Expected invalid boolean error, got {}", error),
    }
}

fn fail_synthesis(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::IntegerError(
            IntegerError::SynthesisError(_string),
        )) => {}
        error => panic!("Expected synthesis error, got {}", error),
    }
}

#[test]
fn test_u32() {
    test_uint!(TestU32, u32, UInt32, DIRECTORY_NAME);

    TestU32::test_min(std::u32::MIN);
    TestU32::test_max(std::u32::MAX);

    TestU32::test_input();

    TestU32::test_add();
    // TestU32::test_sub(); //Todo: Catch subtraction overflow error in gadget
    TestU32::test_mul();
    TestU32::test_div();
    TestU32::test_pow();
}

#[test]
fn test_zero() {
    let program = compile_program(DIRECTORY_NAME, "zero.leo").unwrap();
    output_zero(program);
}

#[test]
fn test_one() {
    let program = compile_program(DIRECTORY_NAME, "one.leo").unwrap();
    output_one(program);
}
