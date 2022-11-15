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

/// Errors in graph operations.
#[derive(Debug)]
pub enum GraphError<N: Node> {
    /// An error that is emitted when a cycle is detected in the graph. Contains the path of cycle.
    CycleDetected(Vec<N>),
}

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
    pub fn topological_sort(&self) -> Result<IndexSet<N>, GraphError<N>> {
        // The set of nodes that do not need to be visited again.
        let mut finished: IndexSet<N> = IndexSet::with_capacity(self.nodes.len());

        // Perform a depth-first search of the graph, starting from `node`, for each node in the graph.
        for node in self.nodes.iter() {
            // If the node has not been explored, explore it.
            if !finished.contains(node) {
                // The set of nodes that are on the path to the current node in the search.
                let mut discovered: IndexSet<N> = IndexSet::new();
                // Check if there is a cycle in the graph starting from `node`.
                if self.contains_cycle_from(*node, &mut discovered, &mut finished) {
                    let path = match discovered.pop() {
                        // TODO: Should this error more silently?
                        None => unreachable!("If `contains_cycle_from` returns `true`, `discovered` is not empty."),
                        Some(node) => {
                            let mut path = vec![node];
                            // Backtrack through the discovered nodes to find the cycle.
                            while let Some(next) = discovered.pop() {
                                // Add the node to the path.
                                path.push(next);
                                // If the node is the same as the first node in the path, we have found the cycle.
                                if next == node {
                                    break;
                                }
                            }
                            // Reverse the path to get the cycle in the correct order.
                            path.reverse();
                            path
                        }
                    };
                    // A cycle was detected. Return the path of the cycle.
                    return Err(GraphError::CycleDetected(path));
                }
            }
        }
        // No cycle was found. Return the set of nodes in topological order.
        Ok(finished)
    }

    // Detects if there is a cycle in the graph starting from the given node, via a recursive depth-first search.
    // Nodes are added to to `finished` in topological order.
    fn contains_cycle_from(&self, node: N, discovered: &mut IndexSet<N>, finished: &mut IndexSet<N>) -> bool {
        // Add the node to the set of discovered nodes.
        discovered.insert(node);

        // Check each outgoing edge of the node.
        if let Some(children) = self.edges.get(&node) {
            for child in children.iter() {
                // If the node already been discovered, there is a cycle.
                if discovered.contains(child) {
                    // Insert the child node into the set of discovered nodes; this is used to reconstruct the cycle.
                    // Note that this case is always hit when there is a cycle.
                    discovered.insert(*child);
                    return true;
                }
                // If the node has not been explored, explore it.
                if !finished.contains(child) && self.contains_cycle_from(*child, discovered, finished) {
                    return true;
                }
            }
        }

        // Remove the node from the set of discovered nodes.
        discovered.pop();
        // Add the node to the set of finished nodes.
        finished.insert(node);

        false
    }
}
