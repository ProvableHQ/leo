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

//! Methods to enforce constraints on statements in a compiled Leo program.

use crate::{program::Program, StatementResult};
use leo_asg::ConditionalStatement;
use snarkvm_ir::{Instruction, QueryData, Value};

impl<'a> Program<'a> {
    /// Enforces a conditional statement with one or more branches.
    /// Due to R1CS constraints, we must evaluate every branch to properly construct the circuit.
    /// At program execution, we will pass an `indicator` bit down to all child statements within each branch.
    /// The `indicator` bit will select that branch while keeping the constraint system satisfied.
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_conditional_statement(&mut self, statement: &ConditionalStatement<'a>) -> StatementResult<()> {
        // Inherit an indicator from a previous statement.
        let condition = self.enforce_expression(statement.condition.get())?;
        let not_condition_var = self.alloc();

        self.mask(condition.clone(), |program| {
            program.enforce_statement(statement.result.get())
        })?;
        if let Some(next) = statement.next.get() {
            self.emit(Instruction::Not(QueryData {
                destination: not_condition_var,
                values: vec![condition],
            }));
            self.mask(Value::Ref(not_condition_var), |program| program.enforce_statement(next))?;
        }
        Ok(())
    }
}
