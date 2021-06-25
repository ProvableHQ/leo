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

use crate::{errors::StatementError, program::Program};
use leo_asg::{AssignAccess, AssignOperation, AssignStatement, ExpressionNode, Type};
use snarkvm_ir::{Instruction, QueryData, Value};

mod array_index;
mod array_range_index;
mod member;
mod tuple;

struct ResolverContext<'a, 'b> {
    target_array_length: u32,
    input_type: Type<'a>,
    input_register: u32,
    remaining_accesses: &'b [&'b AssignAccess<'a>],
    operation: AssignOperation,
    target_value: Value,
}

impl<'a> Program<'a> {
    fn resolve_target_access<'b>(&mut self, mut context: ResolverContext<'a, 'b>) -> Result<Value, StatementError> {
        if context.remaining_accesses.is_empty() {
            let resulting_value = self.enforce_assign_operation(
                &context.operation,
                Value::Ref(context.input_register),
                context.target_value,
            )?;

            return Ok(resulting_value);
        }
        let access = context.remaining_accesses[context.remaining_accesses.len() - 1];
        context.remaining_accesses = &context.remaining_accesses[..context.remaining_accesses.len() - 1];
        match access {
            AssignAccess::ArrayRange(start, stop) => {
                self.resolve_target_access_array_range(context, start.get(), stop.get())
            }
            AssignAccess::ArrayIndex(index) => self.resolve_target_access_array_index(context, index.get()),
            AssignAccess::Tuple(index) => self.resolve_target_access_tuple(context, *index),
            AssignAccess::Member(identifier) => self.resolve_target_access_member(context, identifier),
        }
    }

    pub fn resolve_assign(
        &mut self,
        assignee: &AssignStatement<'a>,
        target_value: Value,
    ) -> Result<(), StatementError> {
        let variable = assignee.target_variable.get();
        let type_ = variable.borrow().type_.clone();
        let target_array_length = match &assignee.value.get().get_type().expect("missing assignment value type") {
            Type::Array(_, x) => *x,
            _ => 0,
        };

        let target = self.resolve_var(variable);
        let accesses: Vec<_> = assignee.target_accesses.iter().rev().collect();
        let resulting_value = self.resolve_target_access(ResolverContext {
            input_type: type_,
            input_register: target,
            remaining_accesses: &accesses[..],
            target_array_length,
            operation: assignee.operation,
            target_value,
        })?;

        self.emit(Instruction::Store(QueryData {
            destination: target,
            values: vec![resulting_value],
        }));

        Ok(())
    }
}
