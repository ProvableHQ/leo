pub mod address;
pub mod array;
pub mod boolean;
// pub mod circuits;
pub mod definition;
pub mod field;
pub mod function;
pub mod group;
pub mod import;
pub mod input_files;
// pub mod integers;
pub mod macros;
// pub mod mutability;
pub mod statements;
pub mod syntax;
pub mod tuples;

use leo_compiler::{
    compiler::Compiler,
    errors::CompilerError,
    group::targets::edwards_bls12::EdwardsGroupType,
    ConstrainedValue,
    OutputBytes,
};
use leo_input::types::{IntegerType, U32Type, UnsignedIntegerType};
use leo_typed::{InputValue, MainInput};

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::r1cs::TestConstraintSystem;

use std::path::PathBuf;

pub const TEST_OUTPUT_DIRECTORY: &str = "/output/";
const EMPTY_FILE: &str = "";

pub type EdwardsTestCompiler = Compiler<Fq, EdwardsGroupType>;
pub type EdwardsConstrainedValue = ConstrainedValue<Fq, EdwardsGroupType>;

fn new_compiler() -> EdwardsTestCompiler {
    let program_name = "test".to_string();
    let path = PathBuf::from("/test/src/main.leo");
    let output_dir = PathBuf::from(TEST_OUTPUT_DIRECTORY);

    EdwardsTestCompiler::new(program_name, path, output_dir)
}

pub(crate) fn parse_program(bytes: &[u8]) -> Result<EdwardsTestCompiler, CompilerError> {
    let mut compiler = new_compiler();
    let program_string = String::from_utf8_lossy(bytes);

    compiler.parse_program_from_string(&program_string)?;

    Ok(compiler)
}

pub(crate) fn parse_input(bytes: &[u8]) -> Result<EdwardsTestCompiler, CompilerError> {
    let mut compiler = new_compiler();
    let input_string = String::from_utf8_lossy(bytes);

    compiler.parse_input(&input_string, EMPTY_FILE)?;

    Ok(compiler)
}

pub(crate) fn parse_state(bytes: &[u8]) -> Result<EdwardsTestCompiler, CompilerError> {
    let mut compiler = new_compiler();
    let state_string = String::from_utf8_lossy(bytes);

    compiler.parse_input(EMPTY_FILE, &state_string)?;

    Ok(compiler)
}

pub(crate) fn parse_input_and_state(
    input_bytes: &[u8],
    state_bytes: &[u8],
) -> Result<EdwardsTestCompiler, CompilerError> {
    let mut compiler = new_compiler();
    let input_string = String::from_utf8_lossy(input_bytes);
    let state_string = String::from_utf8_lossy(state_bytes);

    compiler.parse_input(&input_string, &state_string)?;

    Ok(compiler)
}

pub fn parse_program_with_input(
    program_bytes: &[u8],
    input_bytes: &[u8],
) -> Result<EdwardsTestCompiler, CompilerError> {
    let mut compiler = new_compiler();

    let program_string = String::from_utf8_lossy(program_bytes);
    let input_string = String::from_utf8_lossy(input_bytes);

    compiler.parse_input(&input_string, EMPTY_FILE)?;
    compiler.parse_program_from_string(&program_string)?;

    Ok(compiler)
}

pub fn parse_program_with_state(
    program_bytes: &[u8],
    state_bytes: &[u8],
) -> Result<EdwardsTestCompiler, CompilerError> {
    let mut compiler = new_compiler();

    let program_string = String::from_utf8_lossy(program_bytes);
    let state_string = String::from_utf8_lossy(state_bytes);

    compiler.parse_input(EMPTY_FILE, &state_string)?;
    compiler.parse_program_from_string(&program_string)?;

    Ok(compiler)
}

pub fn parse_program_with_input_and_state(
    program_bytes: &[u8],
    input_bytes: &[u8],
    state_bytes: &[u8],
) -> Result<EdwardsTestCompiler, CompilerError> {
    let mut compiler = new_compiler();

    let program_string = String::from_utf8_lossy(program_bytes);
    let input_string = String::from_utf8_lossy(input_bytes);
    let state_string = String::from_utf8_lossy(state_bytes);

    compiler.parse_input(&input_string, &state_string)?;
    compiler.parse_program_from_string(&program_string)?;

    Ok(compiler)
}

pub(crate) fn get_output(program: EdwardsTestCompiler) -> OutputBytes {
    // synthesize the circuit on the test constraint system
    let mut cs = TestConstraintSystem::<Fq>::new();
    let output = program.generate_constraints_helper(&mut cs).unwrap();

    // assert the constraint system is satisfied
    assert!(cs.is_satisfied());

    output
}

pub(crate) fn assert_satisfied(program: EdwardsTestCompiler) {
    let empty_output_bytes = include_bytes!("compiler_output/empty.out");
    let res = get_output(program);

    // assert that the output is empty
    assert_eq!(empty_output_bytes, res.bytes().as_slice());
}

pub(crate) fn expect_compiler_error(program: EdwardsTestCompiler) -> CompilerError {
    let mut cs = TestConstraintSystem::<Fq>::new();
    program.generate_constraints_helper(&mut cs).unwrap_err()
}

pub(crate) fn expect_synthesis_error(program: EdwardsTestCompiler) {
    let mut cs = TestConstraintSystem::<Fq>::new();
    let _output = program.generate_constraints_helper(&mut cs).unwrap();

    assert!(!cs.is_satisfied());
}

pub(crate) fn generate_main_input(input: Vec<(&str, Option<InputValue>)>) -> MainInput {
    let mut main_input = MainInput::new();

    for (name, value) in input {
        main_input.insert(name.to_string(), value);
    }

    main_input
}

pub(crate) fn generate_test_input_u32(number: u32) -> Option<InputValue> {
    Some(InputValue::Integer(
        IntegerType::Unsigned(UnsignedIntegerType::U32Type(U32Type {})),
        number.to_string(),
    ))
}
