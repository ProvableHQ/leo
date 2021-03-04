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

//! Enforces array access in a compiled Leo program.

use crate::errors::ExpressionError;
use crate::program::ConstrainedProgram;
use crate::value::ConstrainedValue;
use crate::GroupType;
use leo_asg::Expression;
use leo_asg::Span;

use snarkvm_models::curves::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_array_access<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        array: &'a Expression<'a>,
        index: &'a Expression<'a>,
        span: &Span,
    ) -> Result<ConstrainedValue<'a, F, G>, ExpressionError> {
        let array = match self.enforce_expression(cs, array)? {
            ConstrainedValue::Array(array) => array,
            value => return Err(ExpressionError::undefined_array(value.to_string(), span.to_owned())),
        };

        let index_resolved = self.enforce_index(cs, index, span)?;
        Ok(array[index_resolved].to_owned())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn enforce_array_range_access<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        array: &'a Expression<'a>,
        left: Option<&'a Expression<'a>>,
        right: Option<&'a Expression<'a>>,
        span: &Span,
    ) -> Result<ConstrainedValue<'a, F, G>, ExpressionError> {
        let array = match self.enforce_expression(cs, array)? {
            ConstrainedValue::Array(array) => array,
            value => return Err(ExpressionError::undefined_array(value.to_string(), span.to_owned())),
        };

        let from_resolved = match left {
            Some(from_index) => self.enforce_index(cs, from_index, span)?,
            None => 0usize, // Array slice starts at index 0
        };
        let to_resolved = match right {
            Some(to_index) => self.enforce_index(cs, to_index, span)?,
            None => array.len(), // Array slice ends at array length
        };
        Ok(ConstrainedValue::Array(array[from_resolved..to_resolved].to_owned()))
    }
}
