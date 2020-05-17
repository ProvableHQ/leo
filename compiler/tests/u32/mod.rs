use leo_compiler::{compiler::Compiler, errors::CompilerError, types::Integer, ConstrainedValue};

use snarkos_curves::{
    bls12_377::{Bls12_377, Fr},
    edwards_bls12::EdwardsProjective
};
use snarkos_models::gadgets::r1cs::{ConstraintSynthesizer, TestConstraintSystem};
use snarkos_models::gadgets::utilities::uint32::UInt32;

use std::env::current_dir;

const DIRECTORY_NAME: &str = "tests/u32/";

fn compile_program(directory_name: &str, file_name: &str) -> Result<Compiler<Fr, EdwardsProjective>, CompilerError> {
    let path = current_dir().map_err(|error| CompilerError::DirectoryError(error))?;

    // Sanitize the package path to the test directory
    let mut package_path = path.clone();
    if package_path.is_file() {
        package_path.pop();
    }

    // Construct the path to the test file in the test directory
    let mut main_file_path = package_path.clone();
    main_file_path.push(directory_name);
    main_file_path.push(file_name);

    println!("Compiling file - {:?}", main_file_path);

    // Compile from the main file path
    Compiler::<Fr, EdwardsProjective>::init(file_name.to_string(), main_file_path)
}

#[test]
fn test_zero() {
    let mut cs = TestConstraintSystem::<Fr>::new();
    let program = compile_program(DIRECTORY_NAME, "zero.leo").unwrap();
    let output = program.compile_constraints(&mut cs).unwrap();
    println!("{}", output);

    assert!(cs.is_satisfied());
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::Integer(Integer::U32(UInt32::constant(0)))]),
        output
    );
}

#[test]
fn test_one() {
    let mut cs = TestConstraintSystem::<Fr>::new();
    let program = compile_program(DIRECTORY_NAME, "one.leo").unwrap();
    let output = program.compile_constraints(&mut cs).unwrap();
    println!("{}", output);

    assert!(cs.is_satisfied());
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::Integer(Integer::U32(UInt32::constant(1)))]),
        output
    );
}

#[test]
fn test_1_plus_1() {
    let mut cs = TestConstraintSystem::<Fr>::new();
    let program = compile_program(DIRECTORY_NAME, "1+1.leo").unwrap();
    let output = program.compile_constraints(&mut cs).unwrap();
    println!("{}", output);

    assert!(cs.is_satisfied());
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::Integer(Integer::U32(UInt32::constant(2)))]),
        output
    );
}

#[test]
fn test_1_minus_1() {
    let mut cs = TestConstraintSystem::<Fr>::new();
    let program = compile_program(DIRECTORY_NAME, "1-1.leo").unwrap();
    let output = program.compile_constraints(&mut cs).unwrap();
    println!("{}", output);

    assert!(cs.is_satisfied());
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::Integer(Integer::U32(UInt32::constant(0)))]),
        output
    );
}

#[test]
fn test_1_minus_2_should_fail() {
    // TODO (howardwu): Catch panic from subtraction overflow

    let mut cs = TestConstraintSystem::<Fr>::new();
    let program = compile_program(DIRECTORY_NAME, "1-2.leo").unwrap();
    let output = program.compile_constraints(&mut cs);
    assert!(output.is_err());
}
