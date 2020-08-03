pub mod address;
pub mod array;
pub mod boolean;
pub mod circuits;
pub mod field;
pub mod function;
pub mod group;
pub mod import;
pub mod input_files;
pub mod integers;
pub mod macros;
pub mod mutability;
pub mod statements;
pub mod syntax;

use leo_compiler::{
    compiler::Compiler,
    errors::CompilerError,
    group::targets::edwards_bls12::EdwardsGroupType,
    ConstrainedValue,
    OutputBytes,
};
use leo_types::{InputValue, MainInputs};

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::r1cs::TestConstraintSystem;

use std::path::PathBuf;

pub const TEST_OUTPUTS_DIRECTORY: &str = "/outputs/";
pub const TEST_OUTPUTS_FILE_NAME: &str = "/outputs/test.out";
const EMPTY_FILE: &str = "";

pub type EdwardsTestCompiler = Compiler<Fq, EdwardsGroupType>;
pub type EdwardsConstrainedValue = ConstrainedValue<Fq, EdwardsGroupType>;

fn new_compiler() -> EdwardsTestCompiler {
    let program_name = "test".to_string();
    let path = PathBuf::from("/test/src/main.leo");
    let outputs_dir = PathBuf::from(TEST_OUTPUTS_DIRECTORY);

    EdwardsTestCompiler::new(program_name, path, outputs_dir)
}

pub(crate) fn parse_program(bytes: &[u8]) -> Result<EdwardsTestCompiler, CompilerError> {
    let mut compiler = new_compiler();
    let program_string = String::from_utf8_lossy(bytes);

    compiler.parse_program_from_string(&program_string)?;

    Ok(compiler)
}

pub(crate) fn parse_inputs(bytes: &[u8]) -> Result<EdwardsTestCompiler, CompilerError> {
    let mut compiler = new_compiler();
    let inputs_string = String::from_utf8_lossy(bytes);

    compiler.parse_inputs(&inputs_string, EMPTY_FILE)?;

    Ok(compiler)
}

pub(crate) fn parse_state(bytes: &[u8]) -> Result<EdwardsTestCompiler, CompilerError> {
    let mut compiler = new_compiler();
    let state_string = String::from_utf8_lossy(bytes);

    compiler.parse_inputs(EMPTY_FILE, &state_string)?;

    Ok(compiler)
}

pub(crate) fn parse_inputs_and_state(
    inputs_bytes: &[u8],
    state_bytes: &[u8],
) -> Result<EdwardsTestCompiler, CompilerError> {
    let mut compiler = new_compiler();
    let inputs_string = String::from_utf8_lossy(inputs_bytes);
    let state_string = String::from_utf8_lossy(state_bytes);

    compiler.parse_inputs(&inputs_string, &state_string)?;

    Ok(compiler)
}

pub fn parse_program_with_inputs(
    program_bytes: &[u8],
    input_bytes: &[u8],
) -> Result<EdwardsTestCompiler, CompilerError> {
    let mut compiler = new_compiler();

    let program_string = String::from_utf8_lossy(program_bytes);
    let inputs_string = String::from_utf8_lossy(input_bytes);

    compiler.parse_inputs(&inputs_string, EMPTY_FILE)?;
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

    compiler.parse_inputs(EMPTY_FILE, &state_string)?;
    compiler.parse_program_from_string(&program_string)?;

    Ok(compiler)
}

pub fn parse_program_with_inputs_and_state(
    program_bytes: &[u8],
    inputs_bytes: &[u8],
    state_bytes: &[u8],
) -> Result<EdwardsTestCompiler, CompilerError> {
    let mut compiler = new_compiler();

    let program_string = String::from_utf8_lossy(program_bytes);
    let inputs_string = String::from_utf8_lossy(inputs_bytes);
    let state_string = String::from_utf8_lossy(state_bytes);

    compiler.parse_inputs(&inputs_string, &state_string)?;
    compiler.parse_program_from_string(&program_string)?;

    Ok(compiler)
}

pub(crate) fn get_outputs(program: EdwardsTestCompiler) -> OutputBytes {
    // synthesize the circuit on the test constraint system
    let mut cs = TestConstraintSystem::<Fq>::new();
    let output = program.generate_constraints_helper(&mut cs).unwrap();

    // assert the constraint system is satisfied
    assert!(cs.is_satisfied());

    output
}

pub(crate) fn assert_satisfied(program: EdwardsTestCompiler) {
    let empty_output_bytes = include_bytes!("compiler_outputs/empty.out");
    let res = get_outputs(program);

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

pub(crate) fn generate_main_inputs(inputs: Vec<(&str, Option<InputValue>)>) -> MainInputs {
    let mut main_inputs = MainInputs::new();

    for (name, value) in inputs {
        main_inputs.insert(name.to_string(), value);
    }

    main_inputs
}
