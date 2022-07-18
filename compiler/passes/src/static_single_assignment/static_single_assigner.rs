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

use std::marker::PhantomData;

pub struct StaticSingleAssigner<'a> {
    /// The `RenameTable` for the current basic block in the AST
    pub(crate) rename_table: RenameTable,
    /// A strictly increasing counter, used to ensure that new variable names are unique.
    pub(crate) counter: usize,
    /// A flag to determine whether or not the traversal is on the left-hand side of a definition or an assignment.
    pub(crate) is_lhs: bool,
    /// Phi functions produced by static single assignment.
    pub(crate) phi_functions: Vec<Statement>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> StaticSingleAssigner<'a> {
    // Note: This implementation of `Director` does not use `handler`.
    // It may later become necessary once we stabilize errors.
    pub(crate) fn new(_handler: &'a Handler) -> Self {
        Self {
            rename_table: RenameTable::new(None),
            counter: 0,
            is_lhs: false,
            phi_functions: Vec::new(),
            phantom: Default::default(),
        }
    }

    /// Returns the value of `self.counter`. Increments the counter by 1, ensuring that all invocations of this function return a unique value.
    pub(crate) fn get_unique_id(&mut self) -> usize {
        self.counter += 1;
        self.counter - 1
    }

    /// Clears the `self.phi_functions`, returning the ones that were previously produced.
    pub(crate) fn clear_phi_functions(&mut self) -> Vec<Statement> {
        core::mem::take(&mut self.phi_functions)
    }

    /// Pushes a new scope for a child basic block.
    pub(crate) fn push(&mut self) {
        let parent_table = core::mem::take(&mut self.rename_table);
        self.rename_table = RenameTable {
            parent: Some(Box::from(parent_table)),
            mapping: Default::default(),
        };
    }

    /// If the RenameTable has a parent, then `self.rename_table` is set to the parent, otherwise it is set to a default `RenameTable`.
    pub(crate) fn pop(&mut self) -> RenameTable {
        let parent = self.rename_table.parent.clone().unwrap();
        core::mem::replace(&mut self.rename_table, *parent)
    }
}
