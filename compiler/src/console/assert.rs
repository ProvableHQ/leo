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

//! Enforces an assert equals statement in a compiled Leo program.

use crate::{
    errors::ConsoleError,
    get_indicator_value,
    program::ConstrainedProgram,
    value::ConstrainedValue,
    GroupType,
};
use leo_ast::{Expression, Span, Type};

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn evaluate_console_assert<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        indicator: &Boolean,
        expression: Expression,
        span: &Span,
    ) -> Result<(), ConsoleError> {
        let expected_type = Some(Type::Boolean);
        let expression_string = expression.to_string();

        // Evaluate assert expression
        let assert_expression = self.enforce_expression(cs, file_scope, function_scope, expected_type, expression)?;

        // If the indicator bit is false, do not evaluate the assertion
        // This is okay since we are not enforcing any constraints
        if !get_indicator_value(indicator) {
            return Ok(()); // Continue execution.
        }

        // Unwrap assertion value and handle errors
        let result_option = match assert_expression {
            ConstrainedValue::Boolean(boolean) => boolean.get_value(),
            _ => {
                return Err(ConsoleError::assertion_must_be_boolean(
                    expression_string,
                    span.to_owned(),
                ));
            }
        };
        let result_bool = result_option.ok_or_else(|| ConsoleError::assertion_depends_on_input(span.to_owned()))?;

        if !result_bool {
            return Err(ConsoleError::assertion_failed(expression_string, span.to_owned()));
        }

        Ok(())
    }
}
