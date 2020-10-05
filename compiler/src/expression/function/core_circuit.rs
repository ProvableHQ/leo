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
use crate::{program::ConstrainedProgram, value::ConstrainedValue, GroupType};

use crate::errors::{ExpressionError, FunctionError};
use leo_core::call_core_circuit;
use leo_typed::{Expression, Span, Type};
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    /// Call a default core circuit function with arguments
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_core_circuit_call_expression<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        expected_type: Option<Type>,
        core_circuit: String,
        arguments: Vec<Expression>,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        // Get the value of each core function argument
        let mut argument_values = vec![];
        for argument in arguments.into_iter() {
            let argument_value =
                self.enforce_expression(cs, file_scope.clone(), function_scope.clone(), None, argument)?;
            let core_function_argument = argument_value.to_value();

            argument_values.push(core_function_argument);
        }

        // Call the core function in `leo-core`
        let res = call_core_circuit(cs, core_circuit, argument_values, span.clone())?;

        // Convert the core function returns into constrained values
        let returns = res
            .into_iter()
            .map(ConstrainedValue::from)
            .collect::<Vec<_>>();

        let return_value = if returns.len() == 1 {
            // The function has a single return
            returns[0].clone()
        } else {
            // The function has multiple returns
            ConstrainedValue::Tuple(returns)
        };

        // Check that function returns expected type
        if let Some(expected) = expected_type {
            let actual = return_value.to_type(span.clone())?;
            if expected.ne(&actual) {
                return Err(ExpressionError::FunctionError(Box::new(
                    FunctionError::return_argument_type(expected.to_string(), actual.to_string(), span),
                )));
            }
        }

        Ok(return_value)
    }
}
