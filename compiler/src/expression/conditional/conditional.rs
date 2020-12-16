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

//! Enforces a conditional expression in a compiled Leo program.

use crate::{errors::ExpressionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_asg::{Expression, Span};
use std::sync::Arc;

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::select::CondSelectGadget},
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    /// Enforce ternary conditional expression
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_conditional_expression<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        conditional: &Arc<Expression>,
        first: &Arc<Expression>,
        second: &Arc<Expression>,
        span: &Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        let conditional_value = match self.enforce_expression(cs, conditional)? {
            ConstrainedValue::Boolean(resolved) => resolved,
            value => return Err(ExpressionError::conditional_boolean(value.to_string(), span.to_owned())),
        };

        let first_value = self.enforce_operand(cs, first)?;

        let second_value = self.enforce_operand(cs, second)?;

        let unique_namespace = cs.ns(|| {
            format!(
                "select {} or {} {}:{}",
                first_value, second_value, span.line, span.start
            )
        });

        ConstrainedValue::conditionally_select(unique_namespace, &conditional_value, &first_value, &second_value)
            .map_err(|e| ExpressionError::cannot_enforce("conditional select".to_string(), e, span.to_owned()))
    }
}
