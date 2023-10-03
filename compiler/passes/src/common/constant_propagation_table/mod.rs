// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use indexmap::IndexMap;
use std::cell::RefCell;

use leo_ast::Expression;
use leo_errors::Result;
use leo_span::Symbol;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ConstantPropagationTable {
    /// The parent scope if it exists.
    /// For example, the parent scope of a then-block is the scope containing the associated ConditionalStatement.
    pub(crate) parent: Option<Box<ConstantPropagationTable>>,
    /// The known constants in the current scope
    /// This field is populated as necessary.
    pub(crate) constants: IndexMap<Symbol, Expression>,
    /// The index of the current scope.
    pub(crate) scope_index: usize,
    /// The sub-scopes of this scope.
    pub(crate) scopes: Vec<RefCell<ConstantPropagationTable>>,
}

impl ConstantPropagationTable {
    /// Returns the current scope index.
    /// Increments the scope index.
    pub fn scope_index(&mut self) -> usize {
        let index = self.scope_index;
        self.scope_index += 1;
        index
    }

    /// Inserts a constant into the constant propagation table.
    pub fn insert_constant(&mut self, symbol: Symbol, expr: Expression) -> Result<()> {
        self.constants.insert(symbol, expr);
        Ok(())
    }

    /// Creates a new scope for the block and stores it in the constant propagation table.
    pub fn insert_block(&mut self) -> usize {
        self.scopes.push(RefCell::new(Default::default()));
        self.scope_index()
    }

    /// Inserts a function into the symbol table.
    pub fn insert_fn_scope(&mut self) -> Result<()> {
        self.scope_index();
        self.scopes.push(Default::default());
        Ok(())
    }

    /// Attempts to lookup a constant in the constant propagation table.
    pub fn lookup_constant(&self, symbol: Symbol) -> Option<&Expression> {
        if let Some(constant) = self.constants.get(&symbol) {
            Some(constant)
        } else if let Some(parent) = self.parent.as_ref() {
            parent.lookup_constant(symbol)
        } else {
            None
        }
    }

    /// Returns true if the constant exists in the local scope
    pub fn constant_in_local_scope(&self, symbol: Symbol) -> bool {
        self.constants.contains_key(&symbol)
    }

    /// Returns true if the constant exists in any parent scope
    pub fn constant_in_parent_scope(&self, symbol: Symbol) -> bool {
        if let Some(parent) = self.parent.as_ref() {
            if parent.constants.contains_key(&symbol) { true } else { parent.constant_in_parent_scope(symbol) }
        } else {
            false
        }
    }

    /// Returns the scope associated with `index`, if it exists in the constant propagation table
    pub fn lookup_scope_by_index(&self, index: usize) -> Option<&RefCell<Self>> {
        self.scopes.get(index)
    }
}
