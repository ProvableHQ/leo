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
use std::fmt::Debug;
use std::hash::Hash;

/// A node in a graph.
pub trait Node: Copy + 'static + Eq + PartialEq + Debug + Hash {}

impl Node for Symbol {}

/// A directed graph.
#[derive(Debug)]
pub struct DiGraph<N: Node> {
    /// The set of nodes in the graph.
    nodes: IndexSet<N>,
    // TODO: Better name.
    /// The directed edges in the graph.
    edges: IndexMap<N, IndexSet<N>>,
}

impl<N: Node> DiGraph<N> {
    /// Initializes a new `CallGraph` from a vector of source nodes.
    pub fn new(nodes: IndexSet<N>) -> Self {
        Self {
            nodes,
            edges: IndexMap::new(),
        }
    }

    /// Adds an edge to the call graph.
    pub fn add_edge(&mut self, from: N, to: N) {
        // Add `from` and `to` to the set of nodes if they are not already in the set.
        self.nodes.insert(from);
        self.nodes.insert(to);

        // Add the edge to the adjacency list.
        let entry = self.edges.entry(from).or_default();
        entry.insert(to);
    }

    /// Returns `true` if the graph contains the given node.
    pub fn contains_node(&self, node: N) -> bool {
        self.nodes.contains(&node)
    }

    /// Detects if there is a cycle in the graph.
    pub fn contains_cycle(&self) -> bool {
        // The set of nodes that do not need to be visited again.
        let mut finished: IndexSet<N> = IndexSet::with_capacity(self.nodes.len());
        // The set of nodes that are on the path to the current node in the search.
        let mut discovered: IndexSet<N> = IndexSet::with_capacity(self.nodes.len());

        // Perform a depth-first search of the graph, starting from `node`, for each node in the graph.
        for node in self.nodes.iter() {
            // If the node has not been explored, explore it.
            if !discovered.contains(node)
                && !finished.contains(node)
                && self.contains_cycle_from(*node, &mut discovered, &mut finished)
            {
                // A cycle was found.
                return true;
            }
        }
        // No cycle was found.
        false
    }

    // Detects if there is a cycle in the graph starting from the given node, via a recursive depth-first search.
    fn contains_cycle_from(&self, node: N, discovered: &mut IndexSet<N>, finished: &mut IndexSet<N>) -> bool {
        // Add the node to the set of discovered nodes.
        discovered.insert(node);

        // Check each outgoing edge of the node.
        if let Some(children) = self.edges.get(&node) {
            for child in children.iter() {
                // If the node already been discovered, there is a cycle.
                if discovered.contains(child) {
                    return true;
                }
                // If the node has not been explored, explore it.
                if !finished.contains(child) && self.contains_cycle_from(*child, discovered, finished) {
                    return true;
                }
            }
        }

        // Remove the node from the set of discovered nodes.
        discovered.remove(&node);
        // Add the node to the set of finished nodes.
        finished.insert(node);

        false
    }
}
