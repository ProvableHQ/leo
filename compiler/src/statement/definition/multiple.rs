//! Enforces a multiple definition statement in a compiled Leo program.

use crate::{errors::StatementError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_typed::{Expression, Span, Variable};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_multiple_definition_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        variables: Vec<Variable>,
        function: Expression,
        span: Span,
    ) -> Result<(), StatementError> {
        let mut expected_types = vec![];
        for variable in variables.iter() {
            if let Some(ref _type) = variable._type {
                expected_types.push(_type.clone());
            }
        }

        // Expect return values from function
        let return_values = match self.enforce_expression(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            &expected_types,
            function,
        )? {
            ConstrainedValue::Return(values) => values,
            value => unimplemented!("multiple assignment only implemented for functions, got {}", value),
        };

        if variables.len() != return_values.len() {
            return Err(StatementError::invalid_number_of_definitions(
                variables.len(),
                return_values.len(),
                span,
            ));
        }

        for (variable, value) in variables.into_iter().zip(return_values.into_iter()) {
            self.store_definition(function_scope.clone(), variable, value);
        }

        Ok(())
    }
}
