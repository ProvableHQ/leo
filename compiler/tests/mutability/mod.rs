use crate::{parse_program, EdwardsConstrainedValue, EdwardsTestCompiler};
use leo_compiler::{
    errors::{CompilerError, FunctionError, StatementError},
    ConstrainedValue,
};
use leo_types::{InputValue, Integer};

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::{r1cs::TestConstraintSystem, utilities::uint::UInt32};

fn mut_success(program: EdwardsTestCompiler) {
    let mut cs = TestConstraintSystem::<Fq>::new();
    let output = program.compile_constraints(&mut cs).unwrap();

    assert!(cs.is_satisfied());
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Integer(Integer::U32(UInt32::constant(0)))]).to_string(),
        output.to_string()
    );
}

fn mut_fail(program: EdwardsTestCompiler) {
    let mut cs = TestConstraintSystem::<Fq>::new();
    let err = program.compile_constraints(&mut cs).unwrap_err();

    // It would be ideal if assert_eq!(Error1, Error2) were possible but unfortunately it is not due to
    // https://github.com/rust-lang/rust/issues/34158#issuecomment-224910299
    match err {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ImmutableAssign(_string))) => {}
        err => panic!("Expected immutable assign error, got {}", err),
    }
}

#[test]
fn test_let() {
    let bytes = include_bytes!("let.leo");
    let program = parse_program(bytes).unwrap();

    mut_fail(program);
}

#[test]
fn test_let_mut() {
    let bytes = include_bytes!("let_mut.leo");
    let program = parse_program(bytes).unwrap();

    mut_success(program);
}

#[test]
fn test_array() {
    let bytes = include_bytes!("array.leo");
    let program = parse_program(bytes).unwrap();

    mut_fail(program);
}

#[test]
fn test_array_mut() {
    let bytes = include_bytes!("array_mut.leo");
    let program = parse_program(bytes).unwrap();

    mut_success(program);
}

#[test]
fn test_circuit() {
    let bytes = include_bytes!("circuit.leo");
    let program = parse_program(bytes).unwrap();

    mut_fail(program);
}

#[test]
fn test_circuit_mut() {
    let bytes = include_bytes!("circuit_mut.leo");
    let program = parse_program(bytes).unwrap();

    mut_success(program);
}

#[test]
fn test_function_input() {
    let bytes = include_bytes!("function_input.leo");
    let mut program = parse_program(bytes).unwrap();

    program.set_inputs(vec![Some(InputValue::Integer(1))]);
    mut_fail(program);
}

#[test]
fn test_function_input_mut() {
    let bytes = include_bytes!("function_input_mut.leo");
    let mut program = parse_program(bytes).unwrap();

    program.set_inputs(vec![Some(InputValue::Integer(1))]);
    mut_success(program);
}
