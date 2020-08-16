//! Enforces an assert equals statement in a compiled Leo program.

use crate::{errors::ConsoleError, evaluate_eq, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
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
        expression: Expression,
        span: Span,
    ) -> Result<(), ConsoleError> {
        let expected_type = Some(Type::Boolean);
        let expression_string = expression.to_string();

        // Evaluate assert expression
        let assert_expression = self.enforce_expression(cs, file_scope, function_scope, expected_type, expression)?;

        // Expect assert expression to evaluate to true
        let expect_true = ConstrainedValue::Boolean(Boolean::Constant(true));
        let result = evaluate_eq(cs, expect_true, assert_expression, span.clone())?;

        // Unwrap assertion value and handle errors
        let result_option = match result {
            ConstrainedValue::Boolean(boolean) => boolean.get_value(),
            _ => unreachable!("evaluate_eq must return boolean"),
        };
        let result_bool = result_option.ok_or(ConsoleError::assertion_depends_on_input(span.clone()))?;

        if !result_bool {
            Err(ConsoleError::assertion_failed(expression_string, span))
        } else {
            Ok(())
        }
    }
}
