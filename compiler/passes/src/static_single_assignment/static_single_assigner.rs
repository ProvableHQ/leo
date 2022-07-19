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

use crate::RenameTable;

use leo_ast::Statement;
use leo_errors::emitter::Handler;

pub struct StaticSingleAssigner<'a> {
    /// The `RenameTable` for the current basic block in the AST
    pub(crate) rename_table: RenameTable,
    /// An error handler used for any errors found during unrolling.
    pub(crate) _handler: &'a Handler,
    /// A strictly increasing counter, used to ensure that new variable names are unique.
    pub(crate) counter: usize,
    /// A flag to determine whether or not the traversal is on the left-hand side of a definition or an assignment.
    pub(crate) is_lhs: bool,
    /// Phi functions produced by static single assignment.
    pub(crate) phi_functions: Vec<Statement>,
}

impl<'a> StaticSingleAssigner<'a> {
    pub(crate) fn new(handler: &'a Handler) -> Self {
        Self {
            rename_table: RenameTable::new(None),
            _handler: handler,
            counter: 0,
            is_lhs: false,
            phi_functions: Vec::new(),
        }
    }

    /// Returns a unique value on each invocation.
    pub(crate) fn unique_id(&mut self) -> usize {
        self.counter += 1;
        self.counter - 1
    }

    /// Clears the `self.phi_functions`, returning the ones that were previously produced.
    pub(crate) fn clear_phi_functions(&mut self) -> Vec<Statement> {
        core::mem::take(&mut self.phi_functions)
    }

    /// Pushes a new scope, setting the current scope as the new scope's parent.
    pub(crate) fn push(&mut self) {
        let parent_table = core::mem::take(&mut self.rename_table);
        self.rename_table = RenameTable::new(Some(Box::from(parent_table)));
    }

    /// If the RenameTable has a parent, then `self.rename_table` is set to the parent, otherwise it is set to a default `RenameTable`.
    pub(crate) fn pop(&mut self) -> RenameTable {
        let parent = self.rename_table.parent.clone().unwrap_or_default();
        core::mem::replace(&mut self.rename_table, *parent)
    }
}
