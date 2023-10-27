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

use leo_ast::NodeID;
use leo_span::Symbol;

use indexmap::IndexMap;

/// `RenameTable` tracks the names assigned by static single assignment in a single scope.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RenameTable {
    /// The `RenameTable` of the parent scope.
    pub(crate) parent: Option<Box<RenameTable>>,
    /// The mapping from names in the original AST to new names in the renamed AST.
    names: IndexMap<Symbol, Symbol>,
    /// The mapping from symbols to node IDs.
    /// These are used to ensure that newly introduced symbols reference the appropriate information
    /// that has been previously indexed by node ID. e,g. `TypeTable`.
    ids: IndexMap<Symbol, NodeID>,
}

impl RenameTable {
    /// Create a new `RenameTable` with the given parent.
    pub(crate) fn new(parent: Option<Box<RenameTable>>) -> Self {
        Self { parent, names: IndexMap::new(), ids: IndexMap::new() }
    }

    /// Returns the symbols that were renamed in the current scope.
    pub(crate) fn local_names(&self) -> impl Iterator<Item = &Symbol> {
        self.names.keys()
    }

    /// Updates `self.mapping` with the desired entry.
    /// Creates a new entry if `symbol` is not already in `self.mapping`.
    pub(crate) fn update(&mut self, symbol: Symbol, new_symbol: Symbol, id: NodeID) {
        self.names.insert(symbol, new_symbol);
        self.ids.insert(new_symbol, id);
    }

    /// Looks up the new name for `symbol`, recursively checking the parent if it is not found.
    pub(crate) fn lookup(&self, symbol: Symbol) -> Option<&Symbol> {
        if let Some(var) = self.names.get(&symbol) {
            Some(var)
        } else if let Some(parent) = &self.parent {
            parent.lookup(symbol)
        } else {
            None
        }
    }

    /// Looks up the node ID for `symbol`, recursively checking the parent if it is not found.
    pub(crate) fn lookup_id(&self, symbol: &Symbol) -> Option<&NodeID> {
        if let Some(id) = self.ids.get(symbol) {
            Some(id)
        } else if let Some(parent) = &self.parent {
            parent.lookup_id(symbol)
        } else {
            None
        }
    }
}
