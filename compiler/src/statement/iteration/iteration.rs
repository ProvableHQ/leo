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

//! Enforces an iteration statement in a compiled Leo program.

use crate::{program::Program, StatementResult};
use leo_asg::IterationStatement;

impl<'a> Program<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_iteration_statement(&mut self, statement: &IterationStatement<'a>) -> StatementResult<()> {
        let span = statement.span.clone().unwrap_or_default();
        let from = self.enforce_index(statement.start.get(), &span)?;
        let to = self.enforce_index(statement.stop.get(), &span)?;

        self.repeat(statement.variable, statement.inclusive, from, to, |program| {
            program.enforce_statement(statement.body.get())
        })?;

        Ok(())
    }
}
