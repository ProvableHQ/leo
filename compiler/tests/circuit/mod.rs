use crate::{compile_program, get_error, get_output, integer::u32::output_one};

use leo_compiler::{
    compiler::Compiler,
    errors::{CompilerError, ExpressionError, FunctionError, StatementError},
    ConstrainedCircuitMember, ConstrainedValue, Identifier, Integer,
};
use snarkos_curves::{bls12_377::Fr, edwards_bls12::EdwardsProjective};
use snarkos_models::gadgets::utilities::uint32::UInt32;

const DIRECTORY_NAME: &str = "tests/circuit/";

// Circ { x: 1u32 }
fn output_circuit(program: Compiler<Fr, EdwardsProjective>) {
    let output = get_output(program);
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![
            ConstrainedValue::CircuitExpression(
                Identifier::new("Circ".into()),
                vec![ConstrainedCircuitMember(
                    Identifier::new("x".into()),
                    ConstrainedValue::Integer(Integer::U32(UInt32::constant(1u32)))
                )]
            )
        ]),
        output
    );
}

fn fail_expected_member(program: Compiler<Fr, EdwardsProjective>) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(
            StatementError::ExpressionError(ExpressionError::ExpectedCircuitMember(_string)),
        )) => {}
        error => panic!("Expected invalid circuit member error, got {}", error),
    }
}

// Expressions

#[test]
fn test_inline() {
    let program = compile_program(DIRECTORY_NAME, "inline.leo").unwrap();
    output_circuit(program);
}

#[test]
fn test_inline_fail() {
    let program = compile_program(DIRECTORY_NAME, "inline_fail.leo").unwrap();
    fail_expected_member(program)
}

// Members

#[test]
fn test_member_function() {
    let program = compile_program(DIRECTORY_NAME, "member_function.leo").unwrap();
    output_one(program);
}

#[test]
fn test_member_static_function() {
    let program = compile_program(DIRECTORY_NAME, "member_static_function.leo").unwrap();
    output_one(program);
}
