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

//! Methods to enforce constraints on statements in a compiled Leo program.

use crate::{
    errors::StatementError,
    program::ConstrainedProgram,
    value::ConstrainedValue,
    GroupType,
    IndicatorAndConstrainedValue,
    StatementResult,
};
use leo_core_ast::{ConditionalNestedOrEndStatement, ConditionalStatement, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

fn indicator_to_string(indicator: &Boolean) -> String {
    indicator
        .get_value()
        .map(|b| b.to_string())
        .unwrap_or_else(|| "[input]".to_string())
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    /// Enforces a conditional statement with one or more branches.
    /// Due to R1CS constraints, we must evaluate every branch to properly construct the circuit.
    /// At program execution, we will pass an `indicator` bit down to all child statements within each branch.
    /// The `indicator` bit will select that branch while keeping the constraint system satisfied.
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_conditional_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        indicator: Option<Boolean>,
        statement: ConditionalStatement,
        return_type: Option<Type>,
        span: &Span,
    ) -> StatementResult<Vec<IndicatorAndConstrainedValue<F, G>>> {
        let statement_string = statement.to_string();

        // Inherit the indicator from a previous conditional statement or assume that we are the outer parent
        let outer_indicator = indicator.unwrap_or(Boolean::Constant(true));

        // Evaluate the conditional boolean as the inner indicator
        let inner_indicator = match self.enforce_expression(
            cs,
            file_scope,
            function_scope,
            Some(Type::Boolean),
            statement.condition.clone(),
        )? {
            ConstrainedValue::Boolean(resolved) => resolved,
            value => return Err(StatementError::conditional_boolean(value.to_string(), span.to_owned())),
        };

        // If outer_indicator && inner_indicator, then select branch 1
        let outer_indicator_string = indicator_to_string(&outer_indicator);
        let inner_indicator_string = indicator_to_string(&inner_indicator);
        let branch_1_name = format!(
            "branch indicator 1 {} && {}",
            outer_indicator_string, inner_indicator_string
        );
        let branch_1_indicator = Boolean::and(
            &mut cs.ns(|| format!("branch 1 {} {}:{}", statement_string, span.line, span.start)),
            &outer_indicator,
            &inner_indicator,
        )
        .map_err(|_| StatementError::indicator_calculation(branch_1_name, span.to_owned()))?;

        let mut results = vec![];

        // Evaluate branch 1
        let mut branch_1_result = self.evaluate_branch(
            cs,
            file_scope,
            function_scope,
            Some(branch_1_indicator),
            statement.statements,
            return_type.clone(),
        )?;

        results.append(&mut branch_1_result);

        // If outer_indicator && !inner_indicator, then select branch 2
        let inner_indicator = inner_indicator.not();
        let inner_indicator_string = indicator_to_string(&inner_indicator);
        let branch_2_name = format!(
            "branch indicator 2 {} && {}",
            outer_indicator_string, inner_indicator_string
        );
        let branch_2_indicator = Boolean::and(
            &mut cs.ns(|| format!("branch 2 {} {}:{}", statement_string, span.line, span.start)),
            &outer_indicator,
            &inner_indicator,
        )
        .map_err(|_| StatementError::indicator_calculation(branch_2_name, span.to_owned()))?;

        // Evaluate branch 2
        let mut branch_2_result = match statement.next {
            Some(next) => match next {
                ConditionalNestedOrEndStatement::Nested(nested) => self.enforce_conditional_statement(
                    cs,
                    file_scope,
                    function_scope,
                    Some(branch_2_indicator),
                    *nested,
                    return_type,
                    span,
                )?,
                ConditionalNestedOrEndStatement::End(statements) => self.evaluate_branch(
                    cs,
                    file_scope,
                    function_scope,
                    Some(branch_2_indicator),
                    statements,
                    return_type,
                )?,
            },
            None => vec![],
        };

        results.append(&mut branch_2_result);

        // We return the results of both branches and leave it up to the caller to select the appropriate return
        Ok(results)
    }
}
