use crate::{
    compile_program,
    integers::u32::{output_one, output_zero},
};
use leo_types::InputValue;

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::r1cs::TestConstraintSystem;

const DIRECTORY_NAME: &str = "tests/statements/";

// Ternary if {bool}? {expression} : {expression};

#[test]
fn test_ternary_basic() {
    let mut program_input_true = compile_program(DIRECTORY_NAME, "ternary_basic.leo").unwrap();
    let mut program_input_false = program_input_true.clone();

    program_input_true.set_inputs(vec![Some(InputValue::Boolean(true))]);
    output_one(program_input_true);

    program_input_false.set_inputs(vec![Some(InputValue::Boolean(false))]);
    output_zero(program_input_false);
}

// Iteration for i {start}..{stop} { statements }

#[test]
fn test_iteration_basic() {
    let program = compile_program(DIRECTORY_NAME, "iteration_basic.leo").unwrap();
    output_one(program);
}

// Assertion

#[test]
fn test_assertion_basic() {
    let program = compile_program(DIRECTORY_NAME, "assertion_basic.leo").unwrap();

    let mut program_input_true = program.clone();
    let mut cs_satisfied = TestConstraintSystem::<Fq>::new();

    program_input_true.set_inputs(vec![Some(InputValue::Boolean(true))]);
    let _output = program_input_true.compile_constraints(&mut cs_satisfied).unwrap();

    assert!(cs_satisfied.is_satisfied());

    let mut program_input_false = program.clone();
    let mut cs_unsatisfied = TestConstraintSystem::<Fq>::new();

    program_input_false.set_inputs(vec![Some(InputValue::Boolean(false))]);
    let _output = program_input_false.compile_constraints(&mut cs_unsatisfied).unwrap();

    assert!(!cs_unsatisfied.is_satisfied());
}
