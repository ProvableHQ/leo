//! Enforces an assert equals statement in a compiled Leo program.

use crate::{errors::ConsoleError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_typed::{Expression, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn evaluate_console_assert<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        indicator: Option<Boolean>,
        expression: Expression,
        span: Span,
    ) -> Result<(), ConsoleError> {
        let expected_type = Some(Type::Boolean);
        let expression_string = expression.to_string();

        // Evaluate assert expression
        let assert_expression = self.enforce_expression(cs, file_scope, function_scope, expected_type, expression)?;

        // If the indicator bit is false, do not evaluate the assertion
        // This is okay since we are not enforcing any constraints
        let false_boolean = Boolean::Constant(false);

        if let Some(indicator_bool) = indicator {
            if indicator_bool.eq(&false_boolean) {
                return Ok(()); // continue execution
            }
        }

        // Unwrap assertion value and handle errors
        let result_option = match assert_expression {
            ConstrainedValue::Boolean(boolean) => boolean.get_value(),
            _ => return Err(ConsoleError::assertion_must_be_boolean(expression_string, span.clone())),
        };
        let result_bool = result_option.ok_or(ConsoleError::assertion_depends_on_input(span.clone()))?;

        if !result_bool {
            return Err(ConsoleError::assertion_failed(expression_string, span));
        }

        Ok(())
    }
}
