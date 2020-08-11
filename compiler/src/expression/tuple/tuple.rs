//! Enforces an tuple expression in a compiled Leo program.

use crate::{errors::ExpressionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_typed::{Expression, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    /// Enforce tuple expressions
    pub fn enforce_tuple<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        expected_type: Option<Type>,
        tuple: Vec<Expression>,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        // Check explicit tuple type dimension if given
        let mut expected_types = vec![];

        if expected_type.is_some() {
            match expected_type.unwrap() {
                Type::Tuple(ref types) => {
                    expected_types = types.clone();
                }
                ref type_ => {
                    return Err(ExpressionError::unexpected_tuple(
                        type_.to_string(),
                        format!("{:?}", tuple),
                        span,
                    ));
                }
            }
        }

        let mut result = vec![];
        for (i, expression) in tuple.into_iter().enumerate() {
            let type_ = if expected_types.is_empty() {
                None
            } else {
                Some(expected_types[i].clone())
            };

            result.push(self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), type_, expression)?);
        }

        Ok(ConstrainedValue::Tuple(result))
    }
}
