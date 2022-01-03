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

//! Enforces an array expression in a compiled Leo program.

use std::cell::Cell;

use crate::program::Program;
use leo_asg::{Expression, ExpressionNode};
use leo_errors::Result;
use snarkvm_ir::{ArrayInitRepeatData, Instruction, Value, VarData};

impl<'a> Program<'a> {
    /// Enforce array expressions
    pub fn enforce_array(&mut self, array: &[(Cell<&'a Expression<'a>>, bool)]) -> Result<Value> {
        let any_spread = array.iter().any(|x| x.1);

        let mut result = vec![];
        for (element, is_spread) in array.iter() {
            let element_value = self.enforce_expression(element.get())?;
            // when arrayinit instruction is flattening -- we don't want it to flatten genuinely included non-spreaded arrays
            if any_spread
                && !is_spread
                && matches!(
                    element.get().get_type().expect("no type for array init element"),
                    leo_asg::Type::Array(_, _)
                )
            {
                result.push(Value::Array(vec![element_value]));
            } else {
                result.push(element_value);
            }
        }

        if any_spread {
            let intermediate = self.alloc();
            self.emit(Instruction::ArrayInit(VarData {
                destination: intermediate,
                values: result,
            }));
            Ok(Value::Ref(intermediate))
        } else {
            Ok(Value::Array(result))
        }
    }

    ///
    /// Returns an array value from an array initializer expression.
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_array_initializer(
        &mut self,
        element_expression: &'a Expression<'a>,
        actual_size: u32,
    ) -> Result<Value> {
        let value = self.enforce_expression(element_expression)?;

        let out = self.alloc();
        self.emit(Instruction::ArrayInitRepeat(ArrayInitRepeatData {
            destination: out,
            length: actual_size,
            value,
        }));

        Ok(Value::Ref(out))
    }
}
