//! Enforces a statement in a compiled Leo program.

use crate::{errors::StatementError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_typed::{Statement, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    /// Enforce a program statement.
    /// Returns a Vector of (indicator, value) tuples.
    /// Each evaluated statement may execute of one or more statements that may return early.
    /// To indicate which of these return values to take we conditionally select the value according
    /// to the `indicator` bit that evaluates to true.
    pub fn enforce_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        indicator: Option<Boolean>,
        statement: Statement,
        return_type: Option<Type>,
    ) -> Result<Vec<(Option<Boolean>, ConstrainedValue<F, G>)>, StatementError> {
        let mut results = vec![];

        match statement {
            Statement::Return(expression, span) => {
                let return_value = (
                    indicator,
                    self.enforce_return_statement(cs, file_scope, function_scope, expression, return_type, span)?,
                );

                results.push(return_value);
            }
            Statement::Definition(declare, variables, expressions, span) => {
                self.enforce_definition_statement(
                    cs,
                    file_scope,
                    function_scope,
                    declare,
                    variables,
                    expressions,
                    span,
                )?;
            }
            Statement::Assign(variable, expression, span) => {
                self.enforce_assign_statement(cs, file_scope, function_scope, indicator, variable, expression, span)?;
            }
            Statement::Conditional(statement, span) => {
                let mut result = self.enforce_conditional_statement(
                    cs,
                    file_scope,
                    function_scope,
                    indicator,
                    statement,
                    return_type,
                    span,
                )?;

                results.append(&mut result);
            }
            Statement::Iteration(index, start, stop, statements, span) => {
                let mut result = self.enforce_iteration_statement(
                    cs,
                    file_scope,
                    function_scope,
                    indicator,
                    index,
                    start,
                    stop,
                    statements,
                    return_type,
                    span,
                )?;

                results.append(&mut result);
            }
            Statement::Assert(left, right, span) => {
                let (resolved_left, resolved_right) =
                    self.enforce_binary_expression(cs, file_scope, function_scope, None, left, right, span.clone())?;

                self.enforce_assert_eq_statement(cs, indicator, &resolved_left, &resolved_right, span)?;
            }
            Statement::Console(macro_) => {
                self.evaluate_macro(cs, file_scope, function_scope, macro_)?;
            }
            Statement::Expression(expression, span) => {
                let expression_string = expression.to_string();
                let value = self.enforce_expression(cs, file_scope, function_scope, None, expression)?;

                // handle empty return value cases
                match &value {
                    ConstrainedValue::Tuple(values) => {
                        if !values.is_empty() {
                            return Err(StatementError::unassigned(expression_string, span));
                        }
                    }
                    _ => return Err(StatementError::unassigned(expression_string, span)),
                }

                let result = (indicator, value);

                results.push(result);
            }
        };

        Ok(results)
    }
}
