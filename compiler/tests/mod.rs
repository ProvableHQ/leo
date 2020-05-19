pub mod mutability;
pub mod u32;

use leo_compiler::{compiler::Compiler, errors::CompilerError};

use snarkos_curves::{bls12_377::Fr, edwards_bls12::EdwardsProjective};

use std::env::current_dir;

pub(crate) fn compile_program(
    directory_name: &str,
    file_name: &str,
) -> Result<Compiler<Fr, EdwardsProjective>, CompilerError> {
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
