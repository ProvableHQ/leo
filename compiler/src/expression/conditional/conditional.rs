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

//! Enforces a conditional expression in a compiled Leo program.

use crate::program::Program;
use leo_asg::Expression;
use leo_errors::Result;
use snarkvm_ir::{Instruction, QueryData, Value};

impl<'a> Program<'a> {
    /// Enforce ternary conditional expression
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_conditional_expression(
        &mut self,
        conditional: &'a Expression<'a>,
        first: &'a Expression<'a>,
        second: &'a Expression<'a>,
    ) -> Result<Value> {
        let conditional_value = self.enforce_expression(conditional)?;

        let first_value = self.enforce_expression(first)?;

        let second_value = self.enforce_expression(second)?;

        let out = self.alloc();
        self.emit(Instruction::Pick(QueryData {
            destination: out,
            values: vec![conditional_value, first_value, second_value],
        }));

        Ok(Value::Ref(out))
    }
}
