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

//! Enforces a statement in a compiled Leo program.

use crate::{errors::StatementError, program::Program};
use leo_asg::Statement;

pub type StatementResult<T> = Result<T, StatementError>;

impl<'a> Program<'a> {
    ///
    /// Enforce a program statement.
    /// Returns a Vector of (indicator, value) tuples.
    /// Each evaluated statement may execute of one or more statements that may return early.
    /// To indicate which of these return values to take we conditionally select the value according
    /// to the `indicator` bit that evaluates to true.
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_statement(&mut self, statement: &'a Statement<'a>) -> StatementResult<()> {
        match statement {
            Statement::Return(statement) => {
                self.enforce_return_statement(statement)?;
            }
            Statement::Definition(statement) => {
                self.enforce_definition_statement(statement)?;
            }
            Statement::Assign(statement) => {
                self.enforce_assign_statement(statement)?;
            }
            Statement::Conditional(statement) => {
                self.enforce_conditional_statement(statement)?;
            }
            Statement::Iteration(statement) => {
                self.enforce_iteration_statement(statement)?;
            }
            Statement::Console(statement) => {
                self.evaluate_console_function_call(statement)?;
            }
            Statement::Expression(statement) => {
                let _value = self.enforce_expression(statement.expression.get())?;
            }
            Statement::Block(statement) => {
                self.evaluate_block(statement)?;
            }
            Statement::Empty(_) => (),
        };

        Ok(())
    }
}
