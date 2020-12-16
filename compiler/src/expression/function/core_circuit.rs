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
use crate::{program::ConstrainedProgram, value::ConstrainedValue, CoreCircuit, GroupType};

use crate::errors::ExpressionError;
use leo_asg::{Expression, Function, Span};
use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};
use std::sync::Arc;

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    /// Call a default core circuit function with arguments
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_core_circuit_call_expression<CS: ConstraintSystem<F>, C: CoreCircuit<F, G>>(
        &mut self,
        cs: &mut CS,
        core_circuit: &C,
        function: &Arc<Function>,
        target: Option<&Arc<Expression>>,
        arguments: &[Arc<Expression>],
        span: &Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        let function = function
            .body
            .borrow()
            .upgrade()
            .expect("stale function in call expression");

        let target_value = if let Some(target) = target {
            Some(self.enforce_expression(cs, target)?)
        } else {
            None
        };

        // Get the value of each core function argument
        let arguments = arguments
            .iter()
            .map(|argument| self.enforce_expression(cs, argument))
            .collect::<Result<Vec<_>, _>>()?;

        // Call the core function
        let return_value = core_circuit.call_function(cs, function, span, target_value, arguments)?;

        Ok(return_value)
    }
}
