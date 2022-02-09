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

//! Enforces a lengthof operator in a compiled Leo program.

use crate::{
    program::ConstrainedProgram,
    value::{ConstrainedValue, Integer},
    GroupType,
};
use leo_asg::{ConstInt, LengthOfExpression};
use leo_errors::{Result, Span};

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    /// Enforce array expressions
    pub fn enforce_lengthof<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        lengthof: &'a LengthOfExpression<'a>,
        span: &Span,
    ) -> Result<ConstrainedValue<'a, F, G>> {
        let value = self.enforce_expression(cs, lengthof.inner.get())?;

        Ok(match value {
            ConstrainedValue::Array(array) => {
                ConstrainedValue::Integer(Integer::new(&ConstInt::U32(array.len() as u32)))
            }
            _ => return Err(leo_errors::CompilerError::lengthof_can_only_be_used_on_arrays(span).into()),
        })
    }
}
