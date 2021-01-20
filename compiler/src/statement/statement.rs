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
use leo_asg::{Statement};
use std::sync::Arc;

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

pub type StatementResult<T> = Result<T, StatementError>;
pub type IndicatorAndConstrainedValue<T, U> = (Boolean, ConstrainedValue<T, U>);

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    ///
    /// Enforce a program statement.
    /// Returns a Vector of (indicator, value) tuples.
    /// Each evaluated statement may execute of one or more statements that may return early.
    /// To indicate which of these return values to take we conditionally select the value according
    /// to the `indicator` bit that evaluates to true.
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        indicator: &Boolean,
        statement: &Arc<Statement>,
        mut_self: bool,
    ) -> StatementResult<Vec<IndicatorAndConstrainedValue<F, G>>> {
        let mut results = vec![];

        match &**statement {
            Statement::Return(statement) => {
                let return_value = (
                    *indicator,
                    self.enforce_return_statement(cs, file_scope, function_scope, statement)?,
                );

                results.push(return_value);
            }
            Statement::Definition(statement) => {
                self.enforce_definition_statement(cs, file_scope, function_scope, statement)?;
            }
            Statement::Assign(statement) => {
                self.enforce_assign_statement(
                    cs,
                    file_scope,
                    function_scope,
                    indicator,
                    mut_self,
                    statement,
                )?;
            }
            Statement::Conditional(statement) => {
                let result = self.enforce_conditional_statement(
                    cs,
                    file_scope,
                    function_scope,
                    indicator,
                    mut_self,
                    statement,
                )?;

                results.extend(result);
            }
            Statement::Iteration(statement) => {
                let result = self.enforce_iteration_statement(
                    cs,
                    file_scope,
                    function_scope,
                    indicator,
                    mut_self,
                    statement,
                )?;

                results.extend(result);
            }
            Statement::Console(statement) => {
                self.evaluate_console_function_call(cs, file_scope, function_scope, indicator, statement)?;
            }
            Statement::Expression(statement) => {
                let value = self.enforce_expression(cs, file_scope, function_scope, &statement.expression)?;
                // handle empty return value cases
                match &value {
                    ConstrainedValue::Tuple(values) => {
                        if !values.is_empty() {
                            results.push((*indicator, value));
                        }
                    }
                    _ => return Err(StatementError::unassigned(statement.span.map(|x| x.text.clone()).unwrap_or_default(), statement.span.clone().unwrap_or_default())),
                }
            }
            Statement::Block(statement) => {
                let span = statement.span.clone().unwrap_or_default();
                let result = self.evaluate_block(
                    &mut cs.ns(|| format!("block {}:{}", &span.line, &span.start)),
                    file_scope,
                    function_scope,
                    indicator,
                    statement,
                    mut_self,
                )?;

                results.extend(result);
            }
        };

        Ok(results)
    }
}

/// Unwraps the indicator boolean gadget value or `false` if `None`.
/// This method is used by logging methods only.
/// We can directly get the boolean value of the indicator since we are not enforcing any
/// constraints.
pub fn get_indicator_value(indicator: &Boolean) -> bool {
    indicator.get_value().unwrap_or(false)
}
