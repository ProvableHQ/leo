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

use leo_ast::{NodeID, Type};

use indexmap::IndexMap;
use std::cell::RefCell;

/// A mapping between node IDs and their types.
#[derive(Debug, Default, Clone)]
pub struct TypeTable {
    /// The inner table.
    /// `RefCell` is used here to avoid `&mut` all over the compiler.
    inner: RefCell<IndexMap<NodeID, Type>>,
}

impl TypeTable {
    /// Gets an entry from the table.
    pub fn get(&self, index: &NodeID) -> Option<Type> {
        self.inner.borrow().get(index).cloned()
    }

    /// Inserts an entry into the table.
    pub fn insert(&self, index: NodeID, value: Type) {
        self.inner.borrow_mut().insert(index, value);
    }
}
