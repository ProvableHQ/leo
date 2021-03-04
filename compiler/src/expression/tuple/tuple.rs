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

//! Enforces an tuple expression in a compiled Leo program.

use std::cell::Cell;

use crate::errors::ExpressionError;
use crate::program::ConstrainedProgram;
use crate::value::ConstrainedValue;
use crate::GroupType;
use leo_asg::Expression;

use snarkvm_models::curves::PrimeField;
use snarkvm_models::gadgets::r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    /// Enforce tuple expressions
    pub fn enforce_tuple<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        tuple: &[Cell<&'a Expression<'a>>],
    ) -> Result<ConstrainedValue<'a, F, G>, ExpressionError> {
        let mut result = Vec::with_capacity(tuple.len());
        for expression in tuple.iter() {
            result.push(self.enforce_expression(cs, expression.get())?);
        }

        Ok(ConstrainedValue::Tuple(result))
    }
}
