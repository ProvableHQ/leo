//! Generates R1CS constraints for a compiled Leo program.

use crate::{errors::CompilerError, new_scope, ConstrainedProgram, ConstrainedValue, GroupType, ImportParser};
use leo_types::{Inputs, Program};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::{ConstraintSystem, TestConstraintSystem},
};

pub fn generate_constraints<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    program: Program,
    inputs: Inputs,
    imported_programs: &ImportParser,
) -> Result<ConstrainedValue<F, G>, CompilerError> {
    let mut resolved_program = ConstrainedProgram::new();
    let program_name = program.get_name();
    let main_function_name = new_scope(program_name.clone(), "main".into());

    resolved_program.store_definitions(program, imported_programs)?;

    let main = resolved_program
        .get(&main_function_name)
        .ok_or_else(|| CompilerError::NoMain)?;

    match main.clone() {
        ConstrainedValue::Function(_circuit_identifier, function) => {
            let result = resolved_program.enforce_main_function(cs, program_name, function, inputs)?;
            Ok(result)
        }
        _ => Err(CompilerError::NoMainFunction),
    }
}

pub fn generate_test_constraints<F: Field + PrimeField, G: GroupType<F>>(
    cs: &mut TestConstraintSystem<F>,
    program: Program,
    inputs: Inputs,
    imported_programs: &ImportParser,
) -> Result<(), CompilerError> {
    let mut resolved_program = ConstrainedProgram::<F, G>::new();
    let program_name = program.get_name();

    let tests = program.tests.clone();

    resolved_program.store_definitions(program, imported_programs)?;

    log::info!("Running {} tests", tests.len());

    for (test_name, test_function) in tests.into_iter() {
        let full_test_name = format!("{}::{}", program_name.clone(), test_name.to_string());

        let result = resolved_program.enforce_main_function(
            cs,
            program_name.clone(),
            test_function.0,
            inputs.clone(), // pass program inputs into every test
        );

        if result.is_ok() {
            log::info!(
                "test {} passed. Constraint system satisfied: {}",
                full_test_name,
                cs.is_satisfied()
            );
        } else {
            log::error!("test {} errored: {}", full_test_name, result.unwrap_err());
        }
    }

    Ok(())
}
