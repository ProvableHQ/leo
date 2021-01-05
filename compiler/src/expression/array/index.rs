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

//! Enforces an array index expression in a compiled Leo program.

use crate::{errors::ExpressionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_ast::{Expression, IntegerType, Span, Type};

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn enforce_index<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        index: Expression,
        span: &Span,
    ) -> Result<usize, ExpressionError> {
        let expected_type = Some(Type::IntegerType(IntegerType::U32));
        match self.enforce_operand(cs, file_scope, function_scope, expected_type, index, &span)? {
            ConstrainedValue::Integer(number) => Ok(number.to_usize(span)?),
            value => Err(ExpressionError::invalid_index(value.to_string(), span)),
        }
    }
}
