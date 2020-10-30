// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

//! Enforces a statement in a compiled Leo program.

use crate::{errors::StatementError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_core_ast::{Statement, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

pub type StatementResult<T> = Result<T, StatementError>;
pub type IndicatorAndConstrainedValue<T, U> = (Option<Boolean>, ConstrainedValue<T, U>);

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    /// Enforce a program statement.
    /// Returns a Vector of (indicator, value) tuples.
    /// Each evaluated statement may execute of one or more statements that may return early.
    /// To indicate which of these return values to take we conditionally select the value according
    /// to the `indicator` bit that evaluates to true.
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        indicator: Option<Boolean>,
        statement: Statement,
        return_type: Option<Type>,
        declared_circuit_reference: &str,
    ) -> StatementResult<Vec<IndicatorAndConstrainedValue<F, G>>> {
        let mut results = vec![];

        match statement {
            Statement::Return(expression, span) => {
                let return_value = (
                    indicator,
                    self.enforce_return_statement(cs, file_scope, function_scope, expression, return_type, &span)?,
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
                    &span,
                )?;
            }
            Statement::Assign(variable, expression, span) => {
                self.enforce_assign_statement(
                    cs,
                    file_scope,
                    function_scope,
                    declared_circuit_reference,
                    indicator,
                    variable,
                    expression,
                    &span,
                )?;
            }
            Statement::Conditional(statement, span) => {
                let mut result = self.enforce_conditional_statement(
                    cs,
                    file_scope,
                    function_scope,
                    indicator,
                    statement,
                    return_type,
                    &span,
                )?;

                results.append(&mut result);
            }
            Statement::Iteration(index, start_stop, statements, span) => {
                let mut result = self.enforce_iteration_statement(
                    cs,
                    file_scope,
                    function_scope,
                    indicator,
                    index,
                    start_stop.0,
                    start_stop.1,
                    statements,
                    return_type,
                    &span,
                )?;

                results.append(&mut result);
            }
            Statement::Console(console) => {
                self.evaluate_console_function_call(cs, file_scope, function_scope, indicator, console)?;
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
