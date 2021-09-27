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

//! Enforces a lengthof operator in a compiled Leo program.

use crate::program::Program;
use snarkvm_eval::LEN_CORE;
use snarkvm_ir::CallCoreData;
use snarkvm_ir::{Instruction, Value};

use leo_asg::LengthOfExpression;
use leo_errors::Result;

impl<'a> Program<'a> {
    // /// Enforce array expressions
    // pub fn enforce_lengthof<CS: ConstraintSystem<F>>(
    //     &mut self,
    //     cs: &mut CS,
    //     lengthof: &'a LengthOfExpression<'a>,
    //     span: &Span,
    // ) -> Result<ConstrainedValue<'a, F, G>> {
    //     let value = self.enforce_expression(cs, lengthof.inner.get())?;

    //     Ok(match value {
    //         ConstrainedValue::Array(array) => {
    //             ConstrainedValue::Integer(Integer::new(&ConstInt::U32(array.len() as u32)))
    //         }
    //         _ => return Err(leo_errors::CompilerError::lengthof_can_only_be_used_on_arrays(span).into()),
    //     })
    // }

    /// Enforce array expressions
    pub fn enforce_lengthof(&mut self, lengthof: &'a LengthOfExpression<'a>) -> Result<Value> {
        let value = self.enforce_expression(lengthof.inner.get())?;

        let out = self.alloc();

        self.emit(Instruction::CallCore(CallCoreData {
            destination: out,
            identifier: LEN_CORE.to_string(),
            arguments: vec![value],
        }));

        Ok(Value::Ref(out))
    }
}
