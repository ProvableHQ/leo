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

//! Resolves assignees in a compiled Leo program.

use std::convert::TryInto;

use crate::{
    errors::{ExpressionError, StatementError},
    program::ConstrainedProgram,
    value::ConstrainedValue,
    GroupType,
    Integer,
};
use leo_asg::{ConstInt, Expression, Node};

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::utilities::{eq::EvaluateEqGadget, select::CondSelectGadget};
use snarkvm_r1cs::ConstraintSystem;

use super::ResolverContext;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub(super) fn resolve_target_access_array_index<'b, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        mut context: ResolverContext<'a, 'b, F, G>,
        index: &'a Expression<'a>,
    ) -> Result<(), StatementError> {
        if context.input.len() != 1 {
            return Err(StatementError::array_assign_interior_index(&context.span));
        }
        let input = match context.input.remove(0) {
            ConstrainedValue::Array(old) => old,
            _ => return Err(StatementError::array_assign_index(&context.span)),
        };
        let index_resolved = self.enforce_index(cs, index, &context.span)?;
        if let Some(index) = index_resolved.to_usize() {
            if index >= input.len() {
                Err(StatementError::array_assign_index_bounds(
                    index,
                    input.len(),
                    &context.span,
                ))
            } else {
                let target = input.get_mut(index).unwrap();
                if context.remaining_accesses.is_empty() {
                    self.enforce_assign_context(cs, &context, target)
                } else {
                    context.input = vec![target];
                    self.resolve_target_access(cs, context)
                }
            }
        } else {
            let span = index.span().cloned().unwrap_or_default();
            {
                let array_len: u32 = input
                    .len()
                    .try_into()
                    .map_err(|_| ExpressionError::array_length_out_of_bounds(&span))?;
                self.array_bounds_check(cs, &&index_resolved, array_len, &span)?;
            }

            for (i, item) in input.iter_mut().enumerate() {
                let namespace_string = format!(
                    "evaluate dyn array assignment eq {} {}:{}",
                    i, span.line_start, span.col_start
                );
                let eq_namespace = cs.ns(|| namespace_string);

                let index_bounded = i
                    .try_into()
                    .map_err(|_| ExpressionError::array_index_out_of_legal_bounds(&span))?;
                let const_index = ConstInt::U32(index_bounded).cast_to(&index_resolved.get_type());
                let index_comparison = index_resolved
                    .evaluate_equal(eq_namespace, &Integer::new(&const_index))
                    .map_err(|_| ExpressionError::cannot_evaluate("==".to_string(), &span))?;

                let unique_namespace = cs.ns(|| {
                    format!(
                        "select array dyn assignment {} {}:{}",
                        i, span.line_start, span.col_start
                    )
                });
                let value = ConstrainedValue::conditionally_select(
                    unique_namespace,
                    &index_comparison,
                    &context.target_value,
                    &item,
                )
                .map_err(|e| ExpressionError::cannot_enforce("conditional select".to_string(), e, &span))?;
                *item = value;
            }
            Ok(())
        }
    }
}
