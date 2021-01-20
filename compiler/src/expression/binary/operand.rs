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

//! Enforces one operand in a binary expression in a compiled Leo program.

use crate::{errors::ExpressionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_asg::{Expression};
use std::sync::Arc;

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    /// Enforce an operand of a binary expression.
    /// We don't care about mutability because we are not changing any variables.
    /// We try to resolve unresolved types here if the type is given explicitly.
    pub fn enforce_operand<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        expression: &Arc<Expression>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        let mut branch = self.enforce_expression(cs, file_scope, function_scope, expression)?;

        branch.get_inner_mut();

        Ok(branch)
    }
}
