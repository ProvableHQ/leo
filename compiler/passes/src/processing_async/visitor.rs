// Copyright (C) 2019-2025 Provable Inc.
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

use crate::CompilerState;

use leo_ast::{Function, NodeID};
use leo_span::Symbol;

pub struct ProcessingAsyncVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// The maximum number of inputs allowed for a function. This is the same limit we will enforce
    /// on the number of variables captured by an `async` block.
    pub max_inputs: usize,
    /// The name of the current program being processed
    pub current_program: Symbol,
    /// The name of the current function being processed
    pub current_function: Symbol,
    /// A map of reconstructed functions in the current program scope.
    pub new_async_functions: Vec<(Symbol, Function)>,
    /// Indicates whether this pass actually processed any async blocks.
    pub modified: bool,
}

impl ProcessingAsyncVisitor<'_> {
    pub fn in_scope<T>(&mut self, id: NodeID, func: impl FnOnce(&mut Self) -> T) -> T {
        self.state.symbol_table.enter_scope(Some(id));
        let result = func(self);
        self.state.symbol_table.enter_parent();
        result
    }
}
