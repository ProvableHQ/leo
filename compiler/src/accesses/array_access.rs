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

//! Enforces array access in a compiled Leo program.

use crate::program::Program;
use leo_asg::{Expression, ExpressionNode, Type};
use leo_errors::Result;
use snarkvm_ir::{Instruction, Integer, QueryData, Value};

impl<'a> Program<'a> {
    pub fn enforce_array_access(&mut self, array: &'a Expression<'a>, index: &'a Expression<'a>) -> Result<Value> {
        let array_value = self.enforce_expression(array)?;
        let index_value = self.enforce_expression(index)?;

        let out = self.alloc();
        self.emit(Instruction::ArrayIndexGet(QueryData {
            destination: out,
            values: vec![array_value, index_value],
        }));
        Ok(Value::Ref(out))
    }

    pub fn enforce_array_range_access(
        &mut self,
        array: &'a Expression<'a>,
        left: Option<&'a Expression<'a>>,
        right: Option<&'a Expression<'a>>,
        length: u32,
    ) -> Result<Value> {
        let full_length = match array.get_type() {
            Some(Type::Array(_, length)) => length,
            // TODO: change this to either resolving an array reference or
            // restrict the usage of ranges with open bounds on arrays with
            // unknown size.
            Some(Type::ArrayWithoutSize(_)) => 0,
            _ => panic!("invalid array type for index"),
        };

        let array_value = self.enforce_expression(array)?;
        let left_value = left
            .map(|left| self.enforce_expression(left))
            .transpose()?
            .unwrap_or(Value::Integer(Integer::U32(0)));
        let right_value = right
            .map(|left| self.enforce_expression(left))
            .transpose()?
            .unwrap_or(Value::Integer(Integer::U32(full_length)));

        let out = self.alloc();
        self.emit(Instruction::ArraySliceGet(QueryData {
            destination: out,
            values: vec![
                array_value,
                left_value,
                right_value,
                Value::Integer(Integer::U32(length)),
            ],
        }));
        Ok(Value::Ref(out))
    }
}
