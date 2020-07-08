//! Enforces a return statement in a compiled Leo program.

use crate::{
    errors::{StatementError, ValueError},
    program::ConstrainedProgram,
    value::ConstrainedValue,
    GroupType,
};
use leo_types::{Expression, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

fn check_return_types(expected: &Vec<Type>, actual: &Vec<Type>, span: Span) -> Result<(), StatementError> {
    expected
        .iter()
        .zip(actual.iter())
        .map(|(type_1, type_2)| {
            if type_1.ne(type_2) {
                // catch return Self type
                if type_1.is_self() && type_2.is_circuit() {
                    Ok(())
                } else {
                    Err(StatementError::arguments_type(type_1, type_2, span.clone()))
                }
            } else {
                Ok(())
            }
        })
        .collect::<Result<Vec<()>, StatementError>>()?;

    Ok(())
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_return_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        expressions: Vec<Expression>,
        return_types: Vec<Type>,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, StatementError> {
        // Make sure we return the correct number of values
        if return_types.len() != expressions.len() {
            return Err(StatementError::invalid_number_of_returns(
                return_types.len(),
                expressions.len(),
                span,
            ));
        }

        let mut returns = vec![];
        for (expression, ty) in expressions.into_iter().zip(return_types.clone().into_iter()) {
            let expected_types = vec![ty.clone()];
            let result = self.enforce_operand(
                cs,
                file_scope.clone(),
                function_scope.clone(),
                &expected_types,
                expression,
                span.clone(),
            )?;

            returns.push(result);
        }

        let actual_types = returns
            .iter()
            .map(|value| value.to_type(span.clone()))
            .collect::<Result<Vec<Type>, ValueError>>()?;

        check_return_types(&return_types, &actual_types, span)?;

        Ok(ConstrainedValue::Return(returns))
    }
}
