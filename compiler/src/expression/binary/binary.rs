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

//! Enforces a binary expression in a compiled Leo program.

use crate::{errors::ExpressionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_ast::{Expression, Span, Type};

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

type ConstrainedValuePair<T, U> = (ConstrainedValue<T, U>, ConstrainedValue<T, U>);

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_binary_expression<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        expected_type: Option<Type>,
        left: Expression,
        right: Expression,
        span: &Span,
    ) -> Result<ConstrainedValuePair<F, G>, ExpressionError> {
        let mut resolved_left =
            self.enforce_operand(cs, file_scope, function_scope, expected_type.clone(), left, span)?;
        let mut resolved_right =
            self.enforce_operand(cs, file_scope, function_scope, expected_type.clone(), right, span)?;

        resolved_left.resolve_types(&mut resolved_right, expected_type, span)?;

        Ok((resolved_left, resolved_right))
    }
}
