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

//! Enforces a conditional expression in a compiled Leo program.

use crate::{program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_asg::Expression;
use leo_errors::{CompilerError, LeoError, Span};


use snarkvm_fields::PrimeField;
use snarkvm_gadgets::traits::select::CondSelectGadget;
use snarkvm_r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    /// Enforce ternary conditional expression
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_conditional_expression<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        conditional: &'a Expression<'a>,
        first: &'a Expression<'a>,
        second: &'a Expression<'a>,
        span: &Span,
    ) -> Result<ConstrainedValue<'a, F, G>, LeoError> {
        let conditional_value = match self.enforce_expression(cs, conditional)? {
            ConstrainedValue::Boolean(resolved) => resolved,
            value => return Err(LeoError::from(CompilerError::conditional_boolean(value.to_string(), span))),
        };

        let first_value = self.enforce_expression(cs, first)?;

        let second_value = self.enforce_expression(cs, second)?;

        let unique_namespace = cs.ns(|| {
            format!(
                "select {} or {} {}:{}",
                first_value, second_value, span.line_start, span.col_start
            )
        });

        ConstrainedValue::conditionally_select(unique_namespace, &conditional_value, &first_value, &second_value)
            .map_err(|e| LeoError::from(CompilerError::cannot_enforce("conditional select".to_string(), e, span)))
    }
}
