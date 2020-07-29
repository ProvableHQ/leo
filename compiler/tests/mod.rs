pub mod address;
pub mod array;
pub mod boolean;
pub mod circuits;
pub mod field;
pub mod function;
pub mod group;
pub mod import;
pub mod inputs;
pub mod integers;
pub mod macros;
pub mod mutability;
pub mod statements;
pub mod syntax;

use leo_compiler::{
    compiler::Compiler,
    errors::{CompilerError, FunctionError, StatementError},
    group::targets::edwards_bls12::EdwardsGroupType,
    ConstrainedValue,
};

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::r1cs::TestConstraintSystem;
use std::path::PathBuf;

pub type EdwardsTestCompiler = Compiler<Fq, EdwardsGroupType>;
pub type EdwardsConstrainedValue = ConstrainedValue<Fq, EdwardsGroupType>;

pub(crate) fn get_output(program: EdwardsTestCompiler) -> EdwardsConstrainedValue {
    let mut cs = TestConstraintSystem::<Fq>::new();
    let output = program.compile_constraints(&mut cs).unwrap();
    assert!(cs.is_satisfied());
    output
}

pub(crate) fn get_error(program: EdwardsTestCompiler) -> CompilerError {
    let mut cs = TestConstraintSystem::<Fq>::new();
    program.compile_constraints(&mut cs).unwrap_err()
}

pub(crate) fn fail_enforce(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::Error(_))) => {}
        error => panic!("Expected evaluate error, got {}", error),
    }
}

fn new_compiler() -> EdwardsTestCompiler {
    let program_name = "test".to_string();
    let path = PathBuf::from("/test/src/main.leo");
    let mut compiler = EdwardsTestCompiler::new(program_name, path);

    compiler
}

pub(crate) fn parse_program(bytes: &[u8]) -> Result<EdwardsTestCompiler, CompilerError> {
    let mut compiler = new_compiler();
    let program_string = String::from_utf8_lossy(bytes);

    compiler.parse_program(&program_string)?;

    Ok(compiler)
}

pub(crate) fn parse_inputs(bytes: &[u8]) -> Result<EdwardsTestCompiler, CompilerError> {
    let mut compiler = new_compiler();
    let inputs_string = String::from_utf8_lossy(bytes);

    compiler.parse_inputs(&inputs_string)?;

    Ok(compiler)
}
