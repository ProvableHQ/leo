//! Methods to enforce constraints on statements in a compiled Leo program.

use crate::{
    errors::StatementError,
    new_scope,
    program::ConstrainedProgram,
    value::ConstrainedValue,
    GroupType,
    Integer,
};
use leo_types::{Expression, Identifier, Span, Statement, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, eq::ConditionalEqGadget, uint::UInt32},
    },
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    fn enforce_for_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        indicator: Option<Boolean>,
        index: Identifier,
        start: Expression,
        stop: Expression,
        statements: Vec<Statement>,
        return_types: Vec<Type>,
        span: Span,
    ) -> Result<Vec<(Option<Boolean>, ConstrainedValue<F, G>)>, StatementError> {
        let mut results = vec![];

        let from = self.enforce_index(cs, file_scope.clone(), function_scope.clone(), start, span.clone())?;
        let to = self.enforce_index(cs, file_scope.clone(), function_scope.clone(), stop, span.clone())?;

        for i in from..to {
            // Store index in current function scope.
            // For loop scope is not implemented.
            let index_name = new_scope(function_scope.clone(), index.to_string());

            self.store(
                index_name,
                ConstrainedValue::Integer(Integer::U32(UInt32::constant(i as u32))),
            );

            // Evaluate statements and possibly return early
            let name_unique = format!("for loop iteration {} {}:{}", i, span.line, span.start);
            let mut result = self.evaluate_branch(
                &mut cs.ns(|| name_unique),
                file_scope.clone(),
                function_scope.clone(),
                indicator,
                statements.clone(),
                return_types.clone(),
            )?;

            results.append(&mut result);
        }

        Ok(results)
    }

    fn enforce_assert_eq_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        indicator: Option<Boolean>,
        left: &ConstrainedValue<F, G>,
        right: &ConstrainedValue<F, G>,
        span: Span,
    ) -> Result<(), StatementError> {
        let condition = indicator.unwrap_or(Boolean::Constant(true));
        let name_unique = format!("assert {} == {} {}:{}", left, right, span.line, span.start);
        let result = left.conditional_enforce_equal(cs.ns(|| name_unique), right, &condition);

        Ok(result.map_err(|_| StatementError::assertion_failed(left.to_string(), right.to_string(), span))?)
    }

    /// Enforce a program statement.
    /// Returns a Vector of (indicator, value) tuples.
    /// Each evaluated statement may execute of one or more statements that may return early.
    /// To indicate which of these return values to take we conditionally select that value with the indicator bit.
    pub(crate) fn enforce_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        indicator: Option<Boolean>,
        statement: Statement,
        return_types: Vec<Type>,
    ) -> Result<Vec<(Option<Boolean>, ConstrainedValue<F, G>)>, StatementError> {
        let mut results = vec![];
        match statement {
            Statement::Return(expressions, span) => {
                let return_value = (
                    indicator,
                    self.enforce_return_statement(cs, file_scope, function_scope, expressions, return_types, span)?,
                );

                results.push(return_value);
            }
            Statement::Definition(declare, variable, expression, span) => {
                self.enforce_definition_statement(cs, file_scope, function_scope, declare, variable, expression, span)?;
            }
            Statement::MultipleDefinition(variables, function, span) => {
                self.enforce_multiple_definition_statement(cs, file_scope, function_scope, variables, function, span)?;
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
                    return_types,
                    span,
                )?;

                results.append(&mut result);
            }
            Statement::For(index, start, stop, statements, span) => {
                let mut result = self.enforce_for_statement(
                    cs,
                    file_scope,
                    function_scope,
                    indicator,
                    index,
                    start,
                    stop,
                    statements,
                    return_types,
                    span,
                )?;

                results.append(&mut result);
            }
            Statement::AssertEq(left, right, span) => {
                let (resolved_left, resolved_right) =
                    self.enforce_binary_expression(cs, file_scope, function_scope, &vec![], left, right, span.clone())?;

                self.enforce_assert_eq_statement(cs, indicator, &resolved_left, &resolved_right, span)?;
            }
            Statement::Expression(expression, span) => {
                let expression_string = expression.to_string();
                let value = self.enforce_expression(cs, file_scope, function_scope, &vec![], expression)?;

                // handle empty return value cases
                match &value {
                    ConstrainedValue::Return(values) => {
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
