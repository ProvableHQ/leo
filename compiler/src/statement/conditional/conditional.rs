// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::errors::StatementError;
use crate::program::ConstrainedProgram;
use crate::value::ConstrainedValue;
use crate::GroupType;
use crate::IndicatorAndConstrainedValue;
use crate::StatementResult;
use leo_asg::ConditionalStatement;

use snarkvm_gadgets::traits::boolean::Boolean;
use snarkvm_models::curves::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

fn indicator_to_string(indicator: &Boolean) -> String {
    indicator
        .get_value()
        .map(|b| b.to_string())
        .unwrap_or_else(|| "[input]".to_string())
}

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    /// Enforces a conditional statement with one or more branches.
    /// Due to R1CS constraints, we must evaluate every branch to properly construct the circuit.
    /// At program execution, we will pass an `indicator` bit down to all child statements within each branch.
    /// The `indicator` bit will select that branch while keeping the constraint system satisfied.
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_conditional_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        indicator: &Boolean,
        statement: &ConditionalStatement<'a>,
    ) -> StatementResult<Vec<IndicatorAndConstrainedValue<'a, F, G>>> {
        let span = statement.span.clone().unwrap_or_default();
        // Inherit an indicator from a previous statement.
        let outer_indicator = indicator;

        // Evaluate the conditional boolean as the inner indicator
        let inner_indicator = match self.enforce_expression(cs, statement.condition.get())? {
            ConstrainedValue::Boolean(resolved) => resolved,
            value => {
                return Err(StatementError::conditional_boolean(value.to_string(), span));
            }
        };

        // If outer_indicator && inner_indicator, then select branch 1
        let outer_indicator_string = indicator_to_string(outer_indicator);
        let inner_indicator_string = indicator_to_string(&inner_indicator);
        let branch_1_name = format!(
            "branch indicator 1 {} && {}",
            outer_indicator_string, inner_indicator_string
        );
        let branch_1_indicator = Boolean::and(
            &mut cs.ns(|| format!("branch 1 {} {}:{}", span.text, &span.line, &span.start)),
            outer_indicator,
            &inner_indicator,
        )
        .map_err(|_| StatementError::indicator_calculation(branch_1_name, span.clone()))?;

        let mut results = vec![];

        // Evaluate branch 1
        let mut branch_1_result = self.enforce_statement(cs, &branch_1_indicator, statement.result.get())?;

        results.append(&mut branch_1_result);

        // If outer_indicator && !inner_indicator, then select branch 2
        let inner_indicator = inner_indicator.not();
        let inner_indicator_string = indicator_to_string(&inner_indicator);
        let branch_2_name = format!(
            "branch indicator 2 {} && {}",
            outer_indicator_string, inner_indicator_string
        );
        let branch_2_indicator = Boolean::and(
            &mut cs.ns(|| format!("branch 2 {} {}:{}", span.text, &span.line, &span.start)),
            &outer_indicator,
            &inner_indicator,
        )
        .map_err(|_| StatementError::indicator_calculation(branch_2_name, span.clone()))?;

        // Evaluate branch 2
        let mut branch_2_result = match statement.next.get() {
            Some(next) => self.enforce_statement(cs, &branch_2_indicator, next)?,
            None => vec![],
        };

        results.append(&mut branch_2_result);

        // We return the results of both branches and leave it up to the caller to select the appropriate return
        Ok(results)
    }
}
