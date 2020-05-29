pub mod array;
pub mod boolean;
pub mod circuit;
pub mod field_element;
pub mod function;
pub mod group;
pub mod import;
pub mod integer;
pub mod mutability;
pub mod statement;

use leo_compiler::{compiler::Compiler, errors::CompilerError, ConstrainedValue};

use leo_compiler::group::edwards_bls12::EdwardsGroupType;
use snarkos_curves::edwards_bls12::{EdwardsParameters, Fq};
use snarkos_models::curves::ModelParameters;
use snarkos_models::gadgets::r1cs::TestConstraintSystem;
use std::env::current_dir;

pub(crate) fn get_output(
    program: Compiler<<EdwardsParameters as ModelParameters>::BaseField, Fq, EdwardsGroupType>,
) -> ConstrainedValue<<EdwardsParameters as ModelParameters>::BaseField, Fq, EdwardsGroupType> {
    let mut cs = TestConstraintSystem::<Fq>::new();
    let output = program.compile_constraints(&mut cs).unwrap();
    assert!(cs.is_satisfied());
    output
}

pub(crate) fn get_error(
    program: Compiler<<EdwardsParameters as ModelParameters>::BaseField, Fq, EdwardsGroupType>,
) -> CompilerError {
    let mut cs = TestConstraintSystem::<Fq>::new();
    program.compile_constraints(&mut cs).unwrap_err()
}

pub(crate) fn compile_program(
    directory_name: &str,
    file_name: &str,
) -> Result<
    Compiler<<EdwardsParameters as ModelParameters>::BaseField, Fq, EdwardsGroupType>,
    CompilerError,
> {
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
    Compiler::<Fq>::init(file_name.to_string(), main_file_path)
}
