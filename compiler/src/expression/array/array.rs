//! Enforces an array expression in a compiled Leo program.

use crate::{
    errors::ExpressionError,
    program::{new_scope, ConstrainedProgram},
    value::ConstrainedValue,
    GroupType,
};
use leo_typed::{Expression, Span, SpreadOrExpression, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    /// Enforce array expressions
    pub fn enforce_array<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        mut expected_type: Option<Type>,
        array: Vec<Box<SpreadOrExpression>>,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        // Check explicit array type dimension if given
        let expected_dimensions = vec![];

        if expected_type.is_some() {
            match expected_type.unwrap() {
                Type::Array(ref type_, ref dimensions) => {
                    expected_type = Some(type_.inner_dimension(dimensions).clone());
                }
                ref _type => {
                    return Err(ExpressionError::unexpected_array(
                        _type.to_string(),
                        format!("{:?}", array),
                        span,
                    ));
                }
            }
        }

        let mut result = vec![];
        for element in array.into_iter() {
            match *element {
                SpreadOrExpression::Spread(spread) => match spread {
                    Expression::Identifier(identifier) => {
                        let array_name = new_scope(function_scope.clone(), identifier.to_string());
                        match self.get(&array_name) {
                            Some(value) => match value {
                                ConstrainedValue::Array(array) => result.extend(array.clone()),
                                value => {
                                    return Err(ExpressionError::invalid_spread(value.to_string(), span));
                                }
                            },
                            None => return Err(ExpressionError::undefined_array(identifier.name, span)),
                        }
                    }
                    value => return Err(ExpressionError::invalid_spread(value.to_string(), span)),
                },
                SpreadOrExpression::Expression(expression) => {
                    result.push(self.enforce_expression(
                        cs,
                        file_scope.clone(),
                        function_scope.clone(),
                        expected_type.clone(),
                        expression,
                    )?);
                }
            }
        }

        // Check expected_dimensions if given
        if !expected_dimensions.is_empty() {
            if expected_dimensions[expected_dimensions.len() - 1] != result.len() {
                return Err(ExpressionError::invalid_length(
                    expected_dimensions[expected_dimensions.len() - 1],
                    result.len(),
                    span,
                ));
            }
        }

        Ok(ConstrainedValue::Array(result))
    }
}
