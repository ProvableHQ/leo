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
use eyre::eyre;
use leo_asg::{Expression, Type};
use leo_errors::Result;
use leo_errors::SnarkVMError;
use snarkvm_ir::{Instruction, Integer, QueryData, Value};
use std::convert::TryFrom;

use super::ResolverContext;

impl<'a> Program<'a> {
    pub(super) fn resolve_target_access_array_range<'b>(
        &mut self,
        mut context: ResolverContext<'a, 'b>,
        start: Option<&'a Expression<'a>>,
        stop: Option<&'a Expression<'a>>,
    ) -> Result<Value> {
        let (inner_type, length) = match &context.input_type {
            Type::Array(inner_type, length) => (&**inner_type, *length),
            _ => panic!("illegal type in array range index assignment"),
        };

        let start_value = start
            .map(|e| self.enforce_expression(e))
            .transpose()?
            .unwrap_or(Value::Integer(Integer::U32(0)));
        let stop_value = stop
            .map(|e| self.enforce_expression(e))
            .transpose()?
            .unwrap_or(Value::Integer(Integer::U32(length)));
        let slice_length: u32 = match (&start_value, &stop_value) {
            (Value::Integer(start), Value::Integer(stop)) => {
                u32::try_from(*stop).map_err(|e| SnarkVMError::from(eyre!(e)))?
                    - u32::try_from(*start).map_err(|e| SnarkVMError::from(eyre!(e)))?
            }
            _ => panic!("slice expected Integer"),
        };
        let input_var = context.input_register;

        let out = self.alloc();
        self.emit(Instruction::ArraySliceGet(QueryData {
            destination: out,
            values: vec![
                Value::Ref(input_var),
                start_value.clone(),
                stop_value.clone(),
                Value::Integer(Integer::U32(slice_length)),
            ],
        }));
        context.input_register = out;
        context.input_type = Type::Array(Box::new(inner_type.clone()), slice_length);
        let inner = self.resolve_target_access(context)?;
        self.emit(Instruction::ArraySliceStore(QueryData {
            destination: input_var,
            values: vec![start_value, stop_value, inner],
        }));
        Ok(Value::Ref(input_var))
    }
}
