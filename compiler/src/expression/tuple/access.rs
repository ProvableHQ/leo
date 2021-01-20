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

//! Enforces array access in a compiled Leo program.

use crate::{errors::ExpressionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_asg::{Expression, Span};
use std::sync::Arc;

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_tuple_access<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        tuple: &Arc<Expression>,
        index: usize,
        span: &Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        // Get the tuple values.
        let tuple = match self.enforce_operand(cs, file_scope, function_scope, tuple)? {
            ConstrainedValue::Tuple(tuple) => tuple,
            value => return Err(ExpressionError::undefined_array(value.to_string(), span.to_owned())),
        };

        // Check for out of bounds access.
        if index > tuple.len() - 1 { // probably safe to be a panic here
            return Err(ExpressionError::index_out_of_bounds(index, span.to_owned()));
        }

        Ok(tuple[index].to_owned())
    }
}
