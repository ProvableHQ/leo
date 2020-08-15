//! Enforces an array index expression in a compiled Leo program.

use crate::{errors::ExpressionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_typed::{Expression, IntegerType, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn enforce_index<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        index: Expression,
        span: Span,
    ) -> Result<usize, ExpressionError> {
        let expected_type = Some(Type::IntegerType(IntegerType::U32));
        match self.enforce_operand(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            expected_type,
            index,
            span.clone(),
        )? {
            ConstrainedValue::Integer(number) => Ok(number.to_usize(span.clone())?),
            value => Err(ExpressionError::invalid_index(value.to_string(), span)),
        }
    }
}
