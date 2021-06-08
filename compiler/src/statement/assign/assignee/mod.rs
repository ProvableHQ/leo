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

use crate::{errors::StatementError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_asg::{AssignAccess, AssignOperation, AssignStatement, Span};

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::boolean::Boolean;
use snarkvm_r1cs::ConstraintSystem;

mod array_index;
mod array_range_index;
mod member;
mod tuple;

struct ResolverContext<'a, 'b, F: PrimeField, G: GroupType<F>> {
    input: Vec<&'b mut ConstrainedValue<'a, F, G>>,
    span: Span,
    target_value: ConstrainedValue<'a, F, G>,
    remaining_accesses: Vec<&'b AssignAccess<'a>>,
    indicator: &'b Boolean,
    operation: AssignOperation,
}

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    fn enforce_assign_context<'b, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        context: &ResolverContext<'a, 'b, F, G>,
        target: &mut ConstrainedValue<'a, F, G>,
    ) -> Result<(), StatementError> {
        Self::enforce_assign_operation(
            cs,
            context.indicator,
            format!("select_assign {}:{}", &context.span.line_start, &context.span.col_start),
            &context.operation,
            target,
            context.target_value.clone(),
            &context.span,
        )
    }

    fn resolve_target_access<'b, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        mut context: ResolverContext<'a, 'b, F, G>,
    ) -> Result<(), StatementError> {
        if context.remaining_accesses.is_empty() {
            if context.input.len() != 1 {
                panic!("invalid non-array-context multi-value assignment");
            }
            let input = context.input.remove(0);
            self.enforce_assign_context(cs, &context, input)?;
            return Ok(());
        }
        match context.remaining_accesses.pop().unwrap() {
            AssignAccess::ArrayRange(start, stop) => {
                self.resolve_target_access_array_range(cs, context, start.get(), stop.get())
            }
            AssignAccess::ArrayIndex(index) => self.resolve_target_access_array_index(cs, context, index.get()),
            AssignAccess::Tuple(index) => self.resolve_target_access_tuple(cs, context, *index),
            AssignAccess::Member(identifier) => self.resolve_target_access_member(cs, context, identifier),
        }
    }

    pub fn resolve_assign<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        assignee: &AssignStatement<'a>,
        target_value: ConstrainedValue<'a, F, G>,
        indicator: &Boolean,
    ) -> Result<(), StatementError> {
        let span = assignee.span.clone().unwrap_or_default();
        let variable = assignee.target_variable.get().borrow();

        let mut target = self.get(variable.id).unwrap().clone();
        self.resolve_target_access(cs, ResolverContext {
            input: vec![&mut target],
            span,
            target_value,
            remaining_accesses: assignee.target_accesses.iter().rev().collect(),
            indicator,
            operation: assignee.operation,
        })?;
        *self.get_mut(variable.id).unwrap() = target;
        Ok(())
    }

    pub(crate) fn check_range_index(
        start_index: usize,
        stop_index: usize,
        len: usize,
        span: &Span,
    ) -> Result<(), StatementError> {
        if stop_index < start_index {
            Err(StatementError::array_assign_range_order(
                start_index,
                stop_index,
                len,
                span,
            ))
        } else if start_index > len {
            Err(StatementError::array_assign_index_bounds(start_index, len, span))
        } else if stop_index > len {
            Err(StatementError::array_assign_index_bounds(stop_index, len, span))
        } else {
            Ok(())
        }
    }
}
