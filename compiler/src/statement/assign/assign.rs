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

//! Enforces an assign statement in a compiled Leo program.

use crate::program::Program;
use leo_asg::{AssignOperation, AssignStatement};
use leo_errors::Result;
use snarkvm_ir::Value;

impl<'a> Program<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_assign_statement(&mut self, statement: &AssignStatement<'a>) -> Result<()> {
        // Get the name of the variable we are assigning to
        let new_value = self.enforce_expression(statement.value.get())?;

        self.resolve_assign(statement, new_value)?;

        Ok(())
    }

    pub(super) fn enforce_assign_operation(
        &mut self,
        operation: &AssignOperation,
        target: Value,
        new_value: Value,
    ) -> Result<Value> {
        let new_value = match operation {
            AssignOperation::Assign => new_value,
            AssignOperation::Add => self.evaluate_add(target, new_value)?,
            AssignOperation::Sub => self.evaluate_sub(target, new_value)?,
            AssignOperation::Mul => self.evaluate_mul(target, new_value)?,
            AssignOperation::Div => self.evaluate_div(target, new_value)?,
            AssignOperation::Pow => self.evaluate_pow(target, new_value)?,
            _ => unimplemented!("unimplemented assign operator"),
        };

        Ok(new_value)
    }
}
