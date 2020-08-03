//! Enforces a function input parameter in a compiled Leo program.

use crate::{errors::FunctionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};

use leo_types::{Expression, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_function_input<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        caller_scope: String,
        function_name: String,
        expected_types: Vec<Type>,
        input: Expression,
    ) -> Result<ConstrainedValue<F, G>, FunctionError> {
        // Evaluate the function input value as pass by value from the caller or
        // evaluate as an expression in the current function scope
        match input {
            Expression::Identifier(identifier) => {
                Ok(self.evaluate_identifier(caller_scope, function_name, &expected_types, identifier)?)
            }
            expression => Ok(self.enforce_expression(cs, scope, function_name, &expected_types, expression)?),
        }
    }
}
