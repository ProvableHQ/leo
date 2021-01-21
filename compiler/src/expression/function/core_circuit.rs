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

use crate::errors::ExpressionError;
use leo_asg::{Expression, Span};
use leo_core::call_core_circuit;
use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};
use std::sync::Arc;

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    /// Call a default core circuit function with arguments
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_core_circuit_call_expression<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        core_circuit: String,
        arguments: &Vec<Arc<Expression>>,
        span: &Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        // Get the value of each core function argument
        let mut argument_values = Vec::with_capacity(arguments.len());
        for argument in arguments.into_iter() {
            let argument_value = self.enforce_expression(cs, file_scope, function_scope, argument)?;
            let core_function_argument = argument_value.to_value();

            argument_values.push(core_function_argument);
        }

        // Call the core function in `leo-core`
        let res = call_core_circuit(cs, core_circuit, argument_values, span.clone())?;

        // Convert the core function returns into constrained values
        let returns = res.into_iter().map(ConstrainedValue::from).collect::<Vec<_>>();

        let return_value = if returns.len() == 1 {
            // The function has a single return
            returns.into_iter().next().unwrap()
        } else {
            // The function has multiple returns
            ConstrainedValue::Tuple(returns)
        };

        Ok(return_value)
    }
}
