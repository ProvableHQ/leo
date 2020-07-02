//! Module containing methods to enforce constraints in an Leo program

pub(crate) mod boolean;
pub(crate) use boolean::*;

pub mod function;
pub use function::*;

pub mod expression;
pub use expression::*;

pub(crate) mod field;
pub(crate) use field::*;

pub(crate) mod group;
pub(crate) use group::*;

pub mod program;
pub use program::*;

pub mod value;
pub use value::*;

pub mod statement;
pub use statement::*;

use crate::{errors::CompilerError, GroupType};
use leo_types::{InputValue, Program};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::{ConstraintSystem, TestConstraintSystem},
};

pub fn generate_constraints<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    program: Program,
    parameters: Vec<Option<InputValue>>,
) -> Result<ConstrainedValue<F, G>, CompilerError> {
    let mut resolved_program = ConstrainedProgram::new();
    let program_name = program.get_name();
    let main_function_name = new_scope(program_name.clone(), "main".into());

    resolved_program.resolve_definitions(program)?;

    let main = resolved_program
        .get(&main_function_name)
        .ok_or_else(|| CompilerError::NoMain)?;

    match main.clone() {
        ConstrainedValue::Function(_circuit_identifier, function) => {
            let result = resolved_program.enforce_main_function(cs, program_name, function, parameters)?;
            Ok(result)
        }
        _ => Err(CompilerError::NoMainFunction),
    }
}

pub fn generate_test_constraints<F: Field + PrimeField, G: GroupType<F>>(
    cs: &mut TestConstraintSystem<F>,
    program: Program,
) -> Result<(), CompilerError> {
    let mut resolved_program = ConstrainedProgram::<F, G>::new();
    let program_name = program.get_name();

    let tests = program.tests.clone();

    resolved_program.resolve_definitions(program)?;

    log::info!("Running {} tests", tests.len());

    for (test_name, test_function) in tests.into_iter() {
        let full_test_name = format!("{}::{}", program_name.clone(), test_name.to_string());

        let result = resolved_program.enforce_main_function(
            cs,
            program_name.clone(),
            test_function.0,
            vec![], // test functions should not take any inputs
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
