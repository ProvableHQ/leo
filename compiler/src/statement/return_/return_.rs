//! Enforces a return statement in a compiled Leo program.

use crate::{errors::StatementError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_typed::{Expression, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

fn check_return_type(expected: Option<Type>, actual: Type, span: Span) -> Result<(), StatementError> {
    match expected {
        Some(expected) => {
            if expected.ne(&actual) {
                if expected.is_self() && actual.is_circuit() {
                    return Ok(());
                } else {
                    return Err(StatementError::arguments_type(&expected, &actual, span));
                }
            }
            Ok(())
        }
        None => Ok(()),
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_return_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        expression: Expression,
        return_type: Option<Type>,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, StatementError> {
        // Make sure we return the correct number of values

        let result = self.enforce_operand(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            return_type.clone(),
            expression,
            span.clone(),
        )?;

        check_return_type(return_type, result.to_type(span.clone())?, span)?;

        Ok(result)
    }
}
