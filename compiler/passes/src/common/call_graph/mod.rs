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

use leo_span::Symbol;

use indexmap::{IndexMap, IndexSet};

/// A directed graph describing the caller-callee relationships of the program.
/// A node corresponds to a function.
/// A directed edge of the form `a --> b` corresponds to an invocation of function `b` in the body of `a`.
pub struct CallGraph {
    /// The set of nodes in the call graph.
    nodes: IndexSet<Symbol>,
    // TODO: Better name.
    /// The directed edges in the call graph.
    edges: IndexMap<Symbol, IndexSet<Symbol>>,
}

impl CallGraph {
    /// Initializes a new `CallGraph` from a vector of source nodes.
    pub fn new(nodes: IndexSet<Symbol>) -> Self {
        Self {
            nodes,
            edges: IndexMap::new(),
        }
    }

    /// Adds an edge to the call graph.
    pub fn add_edge(&mut self, from: Symbol, to: Symbol) {
        // Add `from` and `to` to the set of nodes if they are not already in the set.
        self.nodes.insert(from);
        self.nodes.insert(to);

        // Add the edge to the adjacency list.
        let entry = self.edges.entry(from).or_default();
        entry.insert(to);
    }

    /// Returns `true` if the call graph contains the given node.
    pub fn contains_node(&self, node: Symbol) -> bool {
        self.nodes.contains(&node)
    }

    /// Detects if there is a cycle in the call graph.
    pub fn contains_cycle(&self) -> bool {
        let mut discovered: IndexSet<Symbol> = IndexSet::with_capacity(self.nodes.len());
        let mut finished: IndexSet<Symbol> = IndexSet::with_capacity(self.nodes.len());

        for node in self.nodes.iter() {
            if !discovered.contains(node)
                && !finished.contains(node)
                && self.contains_cycle_from(*node, &mut discovered, &mut finished)
            {
                return true;
            }
        }
        false
    }

    fn contains_cycle_from(
        &self,
        node: Symbol,
        discovered: &mut IndexSet<Symbol>,
        finished: &mut IndexSet<Symbol>,
    ) -> bool {
        discovered.insert(node);

        if let Some(children) = self.edges.get(&node) {
            for child in children.iter() {
                if discovered.contains(child) {
                    return true;
                }
                if !finished.contains(child) && self.contains_cycle_from(*child, discovered, finished) {
                    return true;
                }
            }
        }

        discovered.remove(&node);
        finished.insert(node);

        false
    }
}
