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

//! Enforces an assert equals statement in a compiled Leo program.

use crate::{errors::ConsoleError, program::Program};
use leo_asg::Expression;
use snarkvm_ir::{Instruction, PredicateData};

impl<'a> Program<'a> {
    pub fn evaluate_console_assert(&mut self, expression: &'a Expression<'a>) -> Result<(), ConsoleError> {
        // Evaluate assert expression
        let assert_expression = self.enforce_expression(expression)?;

        self.emit(Instruction::Assert(PredicateData {
            values: vec![assert_expression],
        }));

        Ok(())
    }
}
