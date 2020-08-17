//! Generates R1CS constraints for a compiled Leo program.

use crate::{
    errors::CompilerError,
    new_scope,
    ConstrainedProgram,
    ConstrainedValue,
    GroupType,
    ImportParser,
    OutputBytes,
    OutputFile,
};
use leo_typed::{Input, Program};

use leo_input::LeoInputParser;
use leo_package::inputs::InputPairs;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::{ConstraintSystem, TestConstraintSystem},
};
use std::path::PathBuf;

pub fn generate_constraints<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    program: Program,
    input: Input,
    imported_programs: &ImportParser,
) -> Result<OutputBytes, CompilerError> {
    let mut resolved_program = ConstrainedProgram::<F, G>::new();
    let program_name = program.get_name();
    let main_function_name = new_scope(program_name.clone(), "main".into());

    resolved_program.store_definitions(program, imported_programs)?;

    let main = resolved_program
        .get(&main_function_name)
        .ok_or_else(|| CompilerError::NoMain)?;

    match main.clone() {
        ConstrainedValue::Function(_circuit_identifier, function) => {
            let result = resolved_program.enforce_main_function(cs, program_name, function, input)?;
            Ok(result)
        }
        _ => Err(CompilerError::NoMainFunction),
    }
}

pub fn generate_test_constraints<F: Field + PrimeField, G: GroupType<F>>(
    program: Program,
    input: InputPairs,
    imported_programs: &ImportParser,
    output_directory: &PathBuf,
) -> Result<(), CompilerError> {
    let mut resolved_program = ConstrainedProgram::<F, G>::new();
    let program_name = program.get_name();

    let tests = program.tests.clone();

    // Store definitions
    resolved_program.store_definitions(program, imported_programs)?;

    // Get default input
    let default = input.pairs.get(&program_name);

    log::info!("Running {} tests", tests.len());

    for (test_name, test) in tests.into_iter() {
        let cs = &mut TestConstraintSystem::<F>::new();
        let full_test_name = format!("{}::{}", program_name.clone(), test_name.to_string());
        let mut output_file_name = program_name.clone();

        // get input file name from annotation or use test_name
        let input_pair = match test.input_file {
            Some(file_id) => {
                let file_name = file_id.name;

                output_file_name = file_name.clone();

                match input.pairs.get(&file_name) {
                    Some(pair) => pair.to_owned(),
                    None => return Err(CompilerError::InvalidTestContext(file_name)),
                }
            }
            None => default.ok_or(CompilerError::NoTestInput)?,
        };

        // parse input files to abstract syntax trees
        let input_file = &input_pair.input_file;
        let state_file = &input_pair.state_file;

        let input_ast = LeoInputParser::parse_file(input_file)?;
        let state_ast = LeoInputParser::parse_file(state_file)?;

        // parse input files into input struct
        let mut input = Input::new();
        input.parse_input(input_ast)?;
        input.parse_state(state_ast)?;

        // run test function on new program with input
        let result = resolved_program.clone().enforce_main_function(
            cs,
            program_name.clone(),
            test.function,
            input, // pass program input into every test
        );

        if result.is_ok() {
            log::info!(
                "test {} compiled successfully. Constraint system satisfied: {}",
                full_test_name,
                cs.is_satisfied()
            );

            // write result to file
            let output = result?;
            let output_file = OutputFile::new(&output_file_name);

            log::info!("\tWriting output to registers in `{}.out` ...", output_file_name);

            output_file.write(output_directory, output.bytes()).unwrap();
        } else {
            log::error!("test {} errored: {}", full_test_name, result.unwrap_err());
        }
    }

    Ok(())
}
