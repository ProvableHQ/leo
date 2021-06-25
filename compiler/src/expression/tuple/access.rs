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

//! Enforces array access in a compiled Leo program.

use crate::{errors::ExpressionError, program::Program};
use leo_asg::Expression;
use snarkvm_ir::{Instruction, Integer, QueryData, Value};

impl<'a> Program<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_tuple_access(&mut self, tuple: &'a Expression<'a>, index: usize) -> Result<Value, ExpressionError> {
        // Get the tuple values.
        let inner = self.enforce_expression(tuple)?;

        let out = self.alloc();

        self.emit(Instruction::TupleIndexGet(QueryData {
            destination: out,
            values: vec![inner, Value::Integer(Integer::U32(index as u32))],
        }));

        Ok(Value::Ref(out))
    }
}
