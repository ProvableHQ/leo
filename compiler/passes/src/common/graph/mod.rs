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

use leo_span::Symbol;

use indexmap::{IndexMap, IndexSet};
use std::{fmt::Debug, hash::Hash};

/// A struct dependency graph.
pub type StructGraph = DiGraph<Symbol>;

/// A call graph.
pub type CallGraph = DiGraph<Symbol>;

/// An import dependency graph.
pub type ImportGraph = DiGraph<Symbol>;

/// A node in a graph.
pub trait Node: Copy + 'static + Eq + PartialEq + Debug + Hash {}

impl Node for Symbol {}

/// Errors in directed graph operations.
#[derive(Debug)]
pub enum DiGraphError<N: Node> {
    /// An error that is emitted when a cycle is detected in the directed graph. Contains the path of the cycle.
    CycleDetected(Vec<N>),
}

/// A directed graph.
#[derive(Debug)]
pub struct DiGraph<N: Node> {
    /// The set of nodes in the graph.
    nodes: IndexSet<N>,
    /// The directed edges in the graph.
    /// Each entry in the map is a node in the graph, and the set of nodes that it points to.
    edges: IndexMap<N, IndexSet<N>>,
}

impl<N: Node> DiGraph<N> {
    /// Initializes a new `DiGraph` from a vector of source nodes.
    pub fn new(nodes: IndexSet<N>) -> Self {
        Self { nodes, edges: IndexMap::new() }
    }

    /// Adds an edge to the graph.
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

    /// Returns the post-order ordering of the graph.
    /// Detects if there is a cycle in the graph.
    pub fn post_order(&self) -> Result<IndexSet<N>, DiGraphError<N>> {
        // The set of nodes that do not need to be visited again.
        let mut finished: IndexSet<N> = IndexSet::with_capacity(self.nodes.len());

        // Perform a depth-first search of the graph, starting from `node`, for each node in the graph.
        for node in self.nodes.iter() {
            // If the node has not been explored, explore it.
            if !finished.contains(node) {
                // The set of nodes that are on the path to the current node in the search.
                let mut discovered: IndexSet<N> = IndexSet::new();
                // Check if there is a cycle in the graph starting from `node`.
                if let Some(node) = self.contains_cycle_from(*node, &mut discovered, &mut finished) {
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
                    // A cycle was detected. Return the path of the cycle.
                    return Err(DiGraphError::CycleDetected(path));
                }
            }
        }
        // No cycle was found. Return the set of nodes in topological order.
        Ok(finished)
    }

    // Detects if there is a cycle in the graph starting from the given node, via a recursive depth-first search.
    // If there is no cycle, returns `None`.
    // If there is a cycle, returns the node that was most recently discovered.
    // Nodes are added to to `finished` in post-order order.
    fn contains_cycle_from(&self, node: N, discovered: &mut IndexSet<N>, finished: &mut IndexSet<N>) -> Option<N> {
        // Add the node to the set of discovered nodes.
        discovered.insert(node);

        // Check each outgoing edge of the node.
        if let Some(children) = self.edges.get(&node) {
            for child in children.iter() {
                // If the node already been discovered, there is a cycle.
                if discovered.contains(child) {
                    // Insert the child node into the set of discovered nodes; this is used to reconstruct the cycle.
                    // Note that this case is always hit when there is a cycle.
                    return Some(*child);
                }
                // If the node has not been explored, explore it.
                if !finished.contains(child) {
                    if let Some(child) = self.contains_cycle_from(*child, discovered, finished) {
                        return Some(child);
                    }
                }
            }
        }

        // Remove the node from the set of discovered nodes.
        discovered.pop();
        // Add the node to the set of finished nodes.
        finished.insert(node);

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Node for u32 {}

    fn check_post_order<N: Node>(graph: &DiGraph<N>, expected: &[N]) {
        let result = graph.post_order();
        assert!(result.is_ok());

        let order: Vec<N> = result.unwrap().into_iter().collect();
        assert_eq!(order, expected);
    }

    #[test]
    fn test_post_order() {
        let mut graph = DiGraph::<u32>::new(IndexSet::new());

        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);

        check_post_order(&graph, &[5, 4, 2, 3, 1]);

        let mut graph = DiGraph::<u32>::new(IndexSet::new());

        // F -> B
        graph.add_edge(6, 2);
        // B -> A
        graph.add_edge(2, 1);
        // B -> D
        graph.add_edge(2, 4);
        // D -> C
        graph.add_edge(4, 3);
        // D -> E
        graph.add_edge(4, 5);
        // F -> G
        graph.add_edge(6, 7);
        // G -> I
        graph.add_edge(7, 9);
        // I -> H
        graph.add_edge(9, 8);

        // A, C, E, D, B, H, I, G, F.
        check_post_order(&graph, &[1, 3, 5, 4, 2, 8, 9, 7, 6]);
    }

    #[test]
    fn test_cycle() {
        let mut graph = DiGraph::<u32>::new(IndexSet::new());

        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(2, 4);
        graph.add_edge(4, 1);

        let result = graph.post_order();
        assert!(result.is_err());

        let DiGraphError::CycleDetected(cycle) = result.unwrap_err();
        let expected = Vec::from([1u32, 2, 4, 1]);
        assert_eq!(cycle, expected);
    }

    #[test]
    fn test_unconnected_graph() {
        let graph = DiGraph::<u32>::new(IndexSet::from([1, 2, 3, 4, 5]));

        check_post_order(&graph, &[1, 2, 3, 4, 5]);
    }
}
