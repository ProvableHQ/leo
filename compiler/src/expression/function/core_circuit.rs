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

use std::cell::Cell;

use crate::{program::ConstrainedProgram, value::ConstrainedValue, CoreCircuit, GroupType};

use crate::errors::ExpressionError;
use leo_asg::{Expression, Function, Span};
use snarkvm_models::{curves::PrimeField, gadgets::r1cs::ConstraintSystem};

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    /// Call a default core circuit function with arguments
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_core_circuit_call_expression<CS: ConstraintSystem<F>, C: CoreCircuit<'a, F, G>>(
        &mut self,
        cs: &mut CS,
        core_circuit: &C,
        function: &'a Function<'a>,
        target: Option<&'a Expression<'a>>,
        arguments: &[Cell<&'a Expression<'a>>],
        span: &Span,
    ) -> Result<ConstrainedValue<'a, F, G>, ExpressionError> {
        let target_value = if let Some(target) = target {
            Some(self.enforce_expression(cs, target)?)
        } else {
            None
        };

        // Get the value of each core function argument
        let arguments = arguments
            .iter()
            .map(|argument| self.enforce_expression(cs, argument.get()))
            .collect::<Result<Vec<_>, _>>()?;

        // Call the core function
        let return_value = core_circuit.call_function(cs, function, span, target_value, arguments)?;

        Ok(return_value)
    }
}
