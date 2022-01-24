// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::program::Program;
use leo_asg::{Expression, Type};
use leo_errors::Result;
use snarkvm_ir::{Instruction, QueryData, Value};

use super::ResolverContext;

impl<'a> Program<'a> {
    pub(super) fn resolve_target_access_array_index<'b>(
        &mut self,
        mut context: ResolverContext<'a, 'b>,
        index: &'a Expression<'a>,
    ) -> Result<Value> {
        let index_value = self.enforce_expression(index)?;
        let input_var = context.input_register;

        let inner_type = match &context.input_type {
            Type::Array(inner_type, _) => &**inner_type,
            Type::ArrayWithoutSize(inner_type) => &**inner_type,
            _ => panic!("illegal type in array index assignment"),
        };

        let out = self.alloc();
        self.emit(Instruction::ArrayIndexGet(QueryData {
            destination: out,
            values: vec![Value::Ref(input_var), index_value.clone()],
        }));
        context.input_register = out;
        context.input_type = inner_type.clone();
        let inner = self.resolve_target_access(context)?;
        self.emit(Instruction::ArrayIndexStore(QueryData {
            destination: input_var,
            values: vec![index_value, inner],
        }));
        Ok(Value::Ref(input_var))
    }
}
