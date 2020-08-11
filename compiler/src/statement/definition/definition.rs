//! Enforces a definition statement in a compiled Leo program.

use crate::{errors::StatementError, program::ConstrainedProgram, ConstrainedValue, GroupType};
use leo_typed::{Declare, Expression, Span, VariableName, Variables};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    fn enforce_single_definition<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        function_scope: String,
        is_constant: bool,
        variable_name: VariableName,
        mut value: ConstrainedValue<F, G>,
        span: Span,
    ) -> Result<(), StatementError> {
        if is_constant && variable_name.mutable {
            return Err(StatementError::immutable_assign(variable_name.to_string(), span));
        } else {
            value.allocate_value(cs, span)?
        }

        self.store_definition(function_scope, variable_name.mutable, variable_name.identifier, value);

        Ok(())
    }

    pub fn enforce_definition_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        declare: Declare,
        variables: Variables,
        expressions: Vec<Expression>,
        span: Span,
    ) -> Result<(), StatementError> {
        let is_constant = match declare {
            Declare::Let => false,
            Declare::Const => true,
        };

        if variables.names.len() == 1 && expressions.len() == 1 {
            // Define a single variable with a single value

            let variable = variables.names[0].clone();
            let expression = self.enforce_expression(
                cs,
                file_scope.clone(),
                function_scope.clone(),
                &variables.types,
                expressions[0].clone(),
            )?;

            self.enforce_single_definition(cs, function_scope, is_constant, variable, expression, span)
        } else if variables.names.len() == 1 && expressions.len() > 1 {
            // Define a tuple (single variable with multiple values)

            let implicit_types = variables.types.is_empty();
            let mut expected_types = vec![];

            for i in 0..expressions.len() {
                let expected_type = if implicit_types {
                    vec![]
                } else {
                    vec![variables.types[i].clone()]
                };

                expected_types.push(expected_type);
            }

            let mut values = vec![];

            for (expression, expected_type) in expressions.into_iter().zip(expected_types.into_iter()) {
                let value = self.enforce_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    &expected_type,
                    expression,
                )?;

                values.push(value);
            }

            let tuple = ConstrainedValue::Tuple(values);
            let variable = variables.names[0].clone();

            self.enforce_single_definition(cs, function_scope, is_constant, variable, tuple, span)
        } else {
            Ok(())
        }
    }
}
