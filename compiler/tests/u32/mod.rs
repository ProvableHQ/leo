use leo_compiler::{compiler::Compiler, ConstrainedValue};

use snarkos_curves::bls12_377::Fr;

use snarkos_models::gadgets::r1cs::{ConstraintSynthesizer, TestConstraintSystem};
use snarkos_models::gadgets::utilities::uint32::UInt32;
use std::env::current_dir;

const DIRECTORY_NAME: &str = "tests/u32/";

fn compile_program(directory_name: &str, file_name: &str) -> Compiler<Fr> {
    let path = current_dir().unwrap();

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
    let program = Compiler::<Fr>::init(file_name.to_string(), main_file_path);

    program
}

#[test]
fn test_zero() {
    let mut cs = TestConstraintSystem::<Fr>::new();
    let program = compile_program(DIRECTORY_NAME, "zero.leo");
    let output = program.evaluate_program(&mut cs);
    assert!(cs.is_satisfied());

    let output = output.unwrap();
    assert_eq!(
        ConstrainedValue::<Fr>::Return(vec![ConstrainedValue::<Fr>::Integer(UInt32::constant(0))]),
        output
    );
    println!("{}", output);
}

#[test]
fn test_one() {
    let mut cs = TestConstraintSystem::<Fr>::new();
    let program = compile_program(DIRECTORY_NAME, "one.leo");
    let output = program.evaluate_program(&mut cs);
    assert!(cs.is_satisfied());

    let output = output.unwrap();
    assert_eq!(
        ConstrainedValue::<Fr>::Return(vec![ConstrainedValue::<Fr>::Integer(UInt32::constant(1))]),
        output
    );
    println!("{}", output);
}

#[test]
fn test_1_plus_1() {
    let mut cs = TestConstraintSystem::<Fr>::new();
    let program = compile_program(DIRECTORY_NAME, "1+1.leo");
    let output = program.evaluate_program(&mut cs);
    assert!(cs.is_satisfied());

    let output = output.unwrap();
    assert_eq!(
        ConstrainedValue::<Fr>::Return(vec![ConstrainedValue::<Fr>::Integer(UInt32::constant(2))]),
        output
    );
    println!("{}", output);
}

#[test]
fn test_1_minus_1() {
    let mut cs = TestConstraintSystem::<Fr>::new();
    let program = compile_program(DIRECTORY_NAME, "1-1.leo");
    let output = program.evaluate_program(&mut cs);
    assert!(cs.is_satisfied());

    let output = output.unwrap();
    assert_eq!(
        ConstrainedValue::<Fr>::Return(vec![ConstrainedValue::<Fr>::Integer(UInt32::constant(0))]),
        output
    );
    println!("{}", output);
}

#[test]
fn test_1_minus_2_should_fail() {
    // TODO (howardwu): Catch panic from subtraction overflow

    let mut cs = TestConstraintSystem::<Fr>::new();
    let program = compile_program(DIRECTORY_NAME, "1-2.leo");
    let output = program.evaluate_program(&mut cs);
    assert!(output.is_err());
}
