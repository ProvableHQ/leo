//! Enforces array access in a compiled Leo program.

use crate::{errors::ExpressionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_typed::{Expression, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_tuple_access<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        expected_type: Option<Type>,
        tuple: Box<Expression>,
        index: usize,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        let tuple = match self.enforce_operand(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            expected_type,
            *tuple,
            span.clone(),
        )? {
            ConstrainedValue::Tuple(tuple) => tuple,
            value => return Err(ExpressionError::undefined_array(value.to_string(), span.clone())),
        };

        if index > tuple.len() - 1 {
            return Err(ExpressionError::index_out_of_bounds(index, span));
        }

        Ok(tuple[index].to_owned())
    }
}
