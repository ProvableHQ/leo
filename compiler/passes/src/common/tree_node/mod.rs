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

use indexmap::IndexSet;

use leo_span::Symbol;
use std::{fmt::Debug, hash::Hash};

///  A binary search tree to store all paths through nested conditional blocks.
pub type ConditionalTreeNode = TreeNode<Symbol>;

/// A node in a graph.
pub trait Node: Copy + 'static + Eq + PartialEq + Debug + Hash {}

impl Node for Symbol {}

/// A node in a tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode<N: Node> {
    /// The current depth.
    pub depth: usize,
    /// The current node.
    pub elements: IndexSet<N>, // TODO: Can optimize with bitmap if performance is bad.
    /// A counter.
    pub counter: usize,
}

impl<N: Node> TreeNode<N> {
    /// Initializes a new `TreeNode` from a vector of starting elements.
    pub fn new(elements: IndexSet<N>) -> Self {
        Self { depth: 0, elements, counter: 0 }
    }

    /// Adds a child to the current node.
    pub fn create_child(&mut self) -> TreeNode<N> {
        Self { depth: self.depth + 1, elements: self.elements.clone(), counter: self.counter }
    }

    /// Removes an element from the current node.
    pub fn remove_element(&mut self, element: &N) {
        if !self.elements.remove(element) {
            self.counter += 1;
        }
    }
}
