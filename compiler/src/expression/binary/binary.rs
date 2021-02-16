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

//! Enforces a binary expression in a compiled Leo program.

use crate::{errors::ExpressionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_asg::Expression;
use std::sync::Arc;

use snarkvm_models::{curves::PrimeField, gadgets::r1cs::ConstraintSystem};

type ConstrainedValuePair<T, U> = (ConstrainedValue<T, U>, ConstrainedValue<T, U>);

impl<F: PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_binary_expression<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: &Arc<Expression>,
        right: &Arc<Expression>,
    ) -> Result<ConstrainedValuePair<F, G>, ExpressionError> {
        let resolved_left = self.enforce_expression(cs, left)?;
        let resolved_right = self.enforce_expression(cs, right)?;

        Ok((resolved_left, resolved_right))
    }
}
