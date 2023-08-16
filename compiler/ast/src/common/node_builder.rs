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

use crate::NodeID;

use std::cell::RefCell;

/// A counter that produces sequentially increasing `NodeID`s.
#[derive(Debug, Clone)]
pub struct NodeBuilder {
    /// The inner counter.
    /// `RefCell` is used here to avoid `&mut` all over the compiler.
    inner: RefCell<NodeBuilderInner>,
}

impl NodeBuilder {
    /// Returns a new `NodeCounter` with the given `NodeID` as the starting value.
    pub fn new(next: NodeID) -> Self {
        Self { inner: RefCell::new(NodeBuilderInner::new(next)) }
    }

    /// Returns the next `NodeID` and increments the internal state.
    pub fn next_id(&self) -> NodeID {
        self.inner.borrow_mut().next_id()
    }
}

impl Default for NodeBuilder {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Contains the actual data for `Handler`.
/// Modeled this way to afford an API using interior mutability.
#[derive(Debug, Clone)]
pub struct NodeBuilderInner {
    /// The next `NodeID`.
    next: NodeID,
}

impl NodeBuilderInner {
    /// Returns a new `NodeCounter` with the given `NodeID` as the starting value.
    pub fn new(next: NodeID) -> Self {
        Self { next }
    }

    /// Returns the next `NodeID` and increments the internal state.
    pub fn next_id(&mut self) -> NodeID {
        let next = self.next;
        self.next += 1;
        next
    }
}
