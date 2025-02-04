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

use crate::SymbolTable;

use leo_ast::NodeID;

/// Dead code elimination.
///
/// Currently this pass only eliminates unused variables.
/// Note that this pass expects SSA form, so there is no shadowing
/// of variable names, and there are no reassignments.
pub struct DeadCodeEliminator<'a> {
    /// A `SymbolTable` filled in by the `VariableTracker` below.
    pub(crate) symbol_table: &'a mut SymbolTable,
    /// Has this pass actually made any changes to the AST?
    pub(crate) changed: bool,
}

impl<'a> DeadCodeEliminator<'a> {
    /// Initializes a new `DeadCodeEliminator`.
    pub fn new(symbol_table: &'a mut SymbolTable) -> Self {
        Self { symbol_table, changed: false }
    }

    /// Enter, in the symbol table, the scope indicated by this `id`.
    pub(crate) fn in_scope<T>(&mut self, id: NodeID, func: impl FnOnce(&mut Self) -> T) -> T {
        self.symbol_table.enter_scope(Some(id));
        let result = func(self);
        self.symbol_table.enter_parent();
        result
    }
}

pub struct VariableTracker<'a> {
    /// A `SymbolTable` for tracking which variables are actually used.
    pub(crate) symbol_table: &'a mut SymbolTable,
}

impl VariableTracker<'_> {
    /// Enter, in the symbol table, the scope indicated by this `id`.
    pub(crate) fn in_scope<T>(&mut self, id: NodeID, func: impl FnOnce(&mut Self) -> T) -> T {
        self.symbol_table.enter_scope(Some(id));
        let result = func(self);
        self.symbol_table.enter_parent();
        result
    }
}
