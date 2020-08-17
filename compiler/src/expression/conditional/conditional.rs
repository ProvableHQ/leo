//! Enforces a conditional expression in a compiled Leo program.

use crate::{errors::ExpressionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_typed::{Expression, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::select::CondSelectGadget},
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    /// Enforce ternary conditional expression
    pub fn enforce_conditional_expression<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        expected_type: Option<Type>,
        conditional: Expression,
        first: Expression,
        second: Expression,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        let conditional_value = match self.enforce_expression(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            Some(Type::Boolean),
            conditional,
        )? {
            ConstrainedValue::Boolean(resolved) => resolved,
            value => return Err(ExpressionError::conditional_boolean(value.to_string(), span)),
        };

        let first_value = self.enforce_operand(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            expected_type.clone(),
            first,
            span.clone(),
        )?;

        let second_value = self.enforce_operand(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            expected_type,
            second,
            span.clone(),
        )?;

        let unique_namespace = cs.ns(|| {
            format!(
                "select {} or {} {}:{}",
                first_value, second_value, span.line, span.start
            )
        });

        ConstrainedValue::conditionally_select(unique_namespace, &conditional_value, &first_value, &second_value)
            .map_err(|e| ExpressionError::cannot_enforce(format!("conditional select"), e, span))
    }
}
